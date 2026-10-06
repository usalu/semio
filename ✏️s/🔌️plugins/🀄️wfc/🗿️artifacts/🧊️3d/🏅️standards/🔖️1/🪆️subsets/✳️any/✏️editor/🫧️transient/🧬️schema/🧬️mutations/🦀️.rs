//! 🫧️ WFC 3D app-transient mutation aggregate — one verb, because the solve result is replaced
//! wholesale by whoever last ran the inference.

use super::Wfc3dTransient;

#[path = "🏁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Wfc3dTransient, diff = Wfc3dTransient, schema = "wfc.wfc3d.transient")]
pub enum Wfc3dTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}





impl protocol::MutationDiff<Wfc3dTransient> for Wfc3dTransient {
    fn apply(&self, _base: &Wfc3dTransient) -> protocol::MutationApplyResult<Wfc3dTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
