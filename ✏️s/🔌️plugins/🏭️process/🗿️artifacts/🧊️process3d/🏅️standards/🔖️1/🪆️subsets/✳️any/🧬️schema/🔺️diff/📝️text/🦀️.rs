//! 🔺️ Process3d artifact — sparse field-delta diff codec and apply/absorb.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` applies as a plain
//! handle-swap now (no `Process3dStepsDelta` collection-apply machinery — the whole timeline is one
//! composed `s.stdio.semio.flow` child), matching `stock_solid`'s own handle-swap shape.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::diff::*;

use crate::schema::Process3dArtifact;
use crate::Process3dSnapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Process3dDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &Process3dArtifact) -> protocol::MutationApplyResult<Process3dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(workshop) = &self.workshop {
                next.workshop = workshop.clone();
            }
            if let Some(value) = &self.stock_id {
                next.stock_id = value.clone();
            }
            if let Some(value) = &self.stock_label {
                next.stock_label = value.clone();
            }
            if let Some(value) = &self.stock_pose {
                next.stock_pose = value.clone();
            }
            if let Some(value) = &self.stock_payload {
                next.stock_payload = value.clone();
            }
            if let Some(value) = &self.stock_solid {
                next.stock_solid = value.clone();
            }
            if let Some(value) = &self.steps {
                next.steps = value.clone();
            }
            if let Some(value) = &self.step_payloads {
                next.step_payloads = value.clone();
            }
            if let Some(value) = &self.tool_solids {
                next.tool_solids = value.values.clone();
            }
            if let Some(value) = &self.resolved_up_to {
                next.resolved_up_to = *value;
            }
            next
        })
    }
}

impl MutationDiff<Process3dSnapshot> for Process3dDiff {
    fn apply(&self, snapshot: &Process3dSnapshot) -> protocol::MutationApplyResult<Process3dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(workshop) = &self.workshop {
                next.workshop = workshop.clone();
            }
            if let Some(value) = &self.stock_id {
                next.stock_id = value.clone();
            }
            if let Some(value) = &self.stock_label {
                next.stock_label = value.clone();
            }
            if let Some(value) = &self.stock_pose {
                next.stock_pose = value.clone();
            }
            if let Some(value) = &self.stock_payload {
                next.stock_payload = value.clone();
            }
            if let Some(value) = &self.stock_solid {
                next.stock_solid = value.clone();
            }
            if let Some(value) = &self.steps {
                next.steps = value.clone();
            }
            if let Some(value) = &self.step_payloads {
                next.step_payloads = value.clone();
            }
            if let Some(value) = &self.tool_solids {
                next.tool_solids = value.values.clone();
            }
            if let Some(value) = &self.resolved_up_to {
                next.resolved_up_to = *value;
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(workshop);
        take!(stock_id);
        take!(stock_label);
        take!(stock_pose);
        take!(stock_payload);
        take!(stock_solid);
        take!(steps);
        take!(step_payloads);
        take!(tool_solids);
        take!(resolved_up_to);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { artifact: Some(Box::new(Process3dArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Process3dDiffText = String;
//#endregion 🚚️Carrier
