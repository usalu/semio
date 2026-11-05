//! 🫧️ WFC 3D app-transient mutation aggregate — one verb, because the solve result is replaced
//! wholesale by whoever last ran the inference.

use crate::editor::wfc3d::transient::{Wfc3dTransient, Wfc3dAssignment};

#[path = "🏁️set-solve/🦀️.rs"]
mod set_solve;
pub use set_solve::SetSolve;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Wfc3dTransient, diff = Wfc3dTransientDiff, schema = "wfc.wfc3d.transient", reference = Wfc3dTransient, reference_diff = Wfc3dTransientDiff, retained)]
pub enum Wfc3dTransientMutation {
    #[dsl(key = "set-solve")]
    SetSolve(SetSolve),
}






/// 🔺️ Field-sparse diff of [`Wfc3dTransient`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Wfc3dTransientDiff {
    pub assignments: Option<Vec<Wfc3dAssignment>>,
    pub contradiction: Option<bool>,
    pub source_revision: Option<Vec<u8>>,
}

protocol::retained_row_patch!(pub retained_fields for Wfc3dTransientDiff => Wfc3dTransient { assignments: Vec<Wfc3dAssignment>, contradiction: bool, source_revision: Vec<u8> });
protocol::retained_patch_diff!(Wfc3dTransientDiff => Wfc3dTransient);

impl protocol::DiffAlgebra<Wfc3dTransient> for Wfc3dTransientDiff {
    fn is_empty(&self) -> bool {
        self.assignments.is_none() && self.contradiction.is_none() && self.source_revision.is_none()
    }
}

impl protocol::ColdDiffAlgebra<Wfc3dTransient> for Wfc3dTransientDiff {
    fn inverse(&self, base: &Wfc3dTransient) -> Self {
        Self {
            assignments: self.assignments.as_ref().map(|_| base.assignments.clone()),
            contradiction: self.contradiction.as_ref().map(|_| base.contradiction.clone()),
            source_revision: self.source_revision.as_ref().map(|_| base.source_revision.clone()),
        }
    }
}

impl protocol::MutationDiff<Wfc3dTransient> for Wfc3dTransientDiff {}

impl protocol::ColdMutationDiff<Wfc3dTransient> for Wfc3dTransientDiff {
    fn absorb(&mut self, later: Self) {
        if later.source_revision.is_some() { self.source_revision = later.source_revision; }
        if later.assignments.is_some() {
            self.assignments = later.assignments;
        }
        if later.contradiction.is_some() {
            self.contradiction = later.contradiction;
        }
    }

    fn apply(&self, unpublished: &mut Wfc3dTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.assignments {
            next.assignments.clone_from(value);
        }
        if let Some(value) = &self.contradiction {
            next.contradiction.clone_from(value);
        }
        if let Some(value) = &self.source_revision { next.source_revision.clone_from(value); }
        *unpublished = next;
        Ok(())
    }
}
//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
