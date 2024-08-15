use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderPayload {
    Standard { coffee: usize, milk: usize, temp: usize, sugar: usize },
    Beth(String),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderInfo {
    pub order_name: String,
    pub coffee_info: OrderPayload,
    pub date: Option<SystemTime>,
}

pub const COFFEE_TYPE: [&str; 8] = ["Espresso", "Latte", "Flat White", "Cappuccino", "Mocha", "Americano", "Breve", "Con Panna"];
pub const MILK_TYPE: [&str; 7] = ["Cows", "A2", "Almond", "Soy",  "Oat", "Lactose Free", "None"];
pub const TEMP_TYPE: [&str; 5] = ["Iced", "Meh", "Warm", "Hot", "Extra hot"];
pub const SUGAR_TYPE: [&str; 4] = ["None", "One", "Two", "Three"];