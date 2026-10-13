//! 🫧️ Generation3d viewer transient — the closed semantic mutation aggregate.

use super::{Generation3dViewTransient, Generation3dViewTransientPatch, Generation3dPreviewEvalChange};

#[path = "👁️set-preview/🦀️.rs"]
mod set_preview_eval;

pub use set_preview_eval::SetPreviewEval;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation3dViewTransient, diff = Generation3dViewTransientPatch, schema = "generation3dview.transient")]
pub enum Generation3dViewTransientMutation {
    #[dsl(key = "set-preview-eval")]
    SetPreviewEval(SetPreviewEval),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
