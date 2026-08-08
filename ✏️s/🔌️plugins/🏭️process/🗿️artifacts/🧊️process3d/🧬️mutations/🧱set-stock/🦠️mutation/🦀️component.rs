//! 🧱 Process3d mutation — `SetStock`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, Stock};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🧱 `SetStock` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStock {
    pub stock: Stock,
}

pub fn set_stock(stock: Stock) -> Process3dMutation {
    Process3dMutation::SetStock { stock }
}

pub fn apply(doc: &mut Process3dSnapshot, stock: &Stock) {
    doc.stock = stock.clone();
}
//#endregion 🔖️Mutation
