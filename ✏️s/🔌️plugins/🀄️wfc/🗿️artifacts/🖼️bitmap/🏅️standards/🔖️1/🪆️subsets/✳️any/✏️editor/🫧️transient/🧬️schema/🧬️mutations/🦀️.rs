//! 🫧️ Bitmap app-transient mutation aggregate — one verb, because the transient is a cache with a
//! single writer, not an edit history.

use super::{BitmapTransient, BitmapTransientDiff, BitmapTransientText};

#[path = "👁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = BitmapTransient, diff = BitmapTransientDiff, schema = "wfcbitmaptransient")]
pub enum BitmapTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
