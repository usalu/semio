//! 🫧️ WFC 3D app-transient mutation aggregate — one verb, because the solve result is replaced
//! wholesale by whoever last ran the inference.

use super::{Wfc3dTransient, Wfc3dAssignment};

#[path = "🏁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = Wfc3dTransient, diff = Wfc3dTransientDiff, schema = "wfc.wfc3d.transient")]
pub enum Wfc3dTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}






/// 🔺️ Field-sparse diff of [`Wfc3dTransient`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Wfc3dTransientDiff {
    pub assignments: Option<Vec<Wfc3dAssignment>>,
    pub contradiction: Option<bool>,
}

impl protocol::DiffAlgebra<Wfc3dTransient> for Wfc3dTransientDiff {
    fn inverse(&self, base: &Wfc3dTransient) -> Self {
        Self {
            assignments: self.assignments.as_ref().map(|_| base.assignments.clone()),
            contradiction: self.contradiction.as_ref().map(|_| base.contradiction.clone()),
        }
    }
    fn between(base: &Wfc3dTransient, other: &Wfc3dTransient) -> Self {
        Self {
            assignments: (base.assignments != other.assignments).then(|| other.assignments.clone()),
            contradiction: (base.contradiction != other.contradiction).then(|| other.contradiction.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.assignments.is_none() && self.contradiction.is_none()
    }
}

impl protocol::MutationDiff<Wfc3dTransient> for Wfc3dTransientDiff {
    fn apply(&self, base: &Wfc3dTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Wfc3dTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.assignments {
            next.assignments.clone_from(value);
        }
        if let Some(value) = &self.contradiction {
            next.contradiction.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.assignments.is_some() {
            self.assignments = later.assignments;
        }
        if later.contradiction.is_some() {
            self.contradiction = later.contradiction;
        }
    }
}
//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
