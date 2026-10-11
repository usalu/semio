//! 🫧️ Generation3d viewer transient — the closed semantic mutation aggregate.

use super::{Generation3dViewTransient, Generation3dViewTransientPatch, Generation3dPreviewEvalChange};

#[path = "👁️set-preview/🦀️.rs"]
mod set_preview_eval;

pub use set_preview_eval::SetPreviewEval;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation3dViewTransient, diff = Generation3dViewTransientPatch, schema = "generation3dview.transient")]
pub enum Generation3dViewTransientMutation {
    #[dsl(key = "set-preview-eval")]
    SetPreviewEval(SetPreviewEval),
}





/// 🧹️ The mutation's own retirement ladder — the window-transient owner's
/// `OwnedValueRetirementFactory` drains a published evaluation's bytes under a grant instead of
/// dropping them (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
impl semio_framework_value::retirement::RetireOwned for Generation3dViewTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self::SetPreviewEval(SetPreviewEval { eval_text }) = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(eval_text)])
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
