//! 🫧️ Bitmap app-transient mutation aggregate — one verb, because the transient is a cache with a
//! single writer, not an edit history.

use crate::editor::bitmap::transient::{BitmapTransient, BitmapTransientDiff, BitmapTransientText};

#[path = "👁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = BitmapTransient, diff = BitmapTransientDiff, schema = "wfcbitmaptransient", reference = BitmapTransient, reference_diff = BitmapTransientDiff)]
pub enum BitmapTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
