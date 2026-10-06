//! 🫧️ WFC 2D app-transient mutation aggregate — one verb, because the solve result is replaced
//! wholesale by whoever last ran the inference.

use super::Wfc2dTransient;

#[path = "🏁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Wfc2dTransient, diff = Wfc2dTransient, schema = "wfc.wfc2d.transient")]
pub enum Wfc2dTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}





impl protocol::MutationDiff<Wfc2dTransient> for Wfc2dTransient {
    fn apply(&self, _base: &Wfc2dTransient) -> protocol::MutationApplyResult<Wfc2dTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
