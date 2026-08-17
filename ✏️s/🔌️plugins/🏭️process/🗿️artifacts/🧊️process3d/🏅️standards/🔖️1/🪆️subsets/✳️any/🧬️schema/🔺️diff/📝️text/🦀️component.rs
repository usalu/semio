//! 🔺️ Process3d artifact — sparse field-delta diff codec and apply/absorb.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` applies as a plain
//! handle-swap now (no `Process3dStepsDelta` collection-apply machinery — the whole timeline is one
//! composed `s.stdio.semio.flow` child), matching `stock_solid`'s own handle-swap shape.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::process3d::schema::diff::*;


use crate::artifacts::process3d::schema::Process3dArtifact;
use crate::artifacts::process3d::Process3dSnapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Process3dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
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
            if let Some(value) = &self.stock_solid {
                next.stock_solid = value.clone();
            }
            if let Some(value) = &self.steps {
                next.steps = value.clone();
            }
            if let Some(value) = &self.tool_solids {
                next.tool_solids = value.values.clone();
            }
            if let Some(value) = &self.resolved_up_to {
                next.resolved_up_to = *value;
            }
            if let Some(value) = &self.selected_id {
                next.selected_id = value.clone();
            }
            if let Some(value) = &self.selected_face_id {
                next.selected_face_id = *value;
            }
            if let Some(value) = &self.active_utility_id {
                next.active_utility_id = value.clone();
            }
            if let Some(value) = &self.selection_method {
                next.selection_method = value.clone();
            }
            if let Some(value) = &self.engagement_input {
                next.engagement_input = value.clone();
            }
            if let Some(value) = self.camera_position_x { next.camera_position_x = value; }
            if let Some(value) = self.camera_position_y { next.camera_position_y = value; }
            if let Some(value) = self.camera_position_z { next.camera_position_z = value; }
            if let Some(value) = self.camera_target_x { next.camera_target_x = value; }
            if let Some(value) = self.camera_target_y { next.camera_target_y = value; }
            if let Some(value) = self.camera_target_z { next.camera_target_z = value; }
            if let Some(value) = self.camera_fov { next.camera_fov = value; }
            if let Some(value) = self.sun_enabled { next.sun_enabled = value; }
            if let Some(value) = self.sun_azimuth { next.sun_azimuth = value; }
            if let Some(value) = self.sun_elevation { next.sun_elevation = value; }
            if let Some(value) = self.sun_intensity { next.sun_intensity = value; }
            if let Some(value) = &self.sun_color { next.sun_color = value.clone(); }
            if let Some(value) = &self.locale { next.locale = value.clone(); }
            if let Some(value) = &self.contributions_json { next.contributions_json = value.clone(); }
            if let Some(value) = &self.hovered_id { next.hovered_id = value.clone(); }
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
            if let Some(value) = &self.stock_solid {
                next.stock_solid = value.clone();
            }
            if let Some(value) = &self.steps {
                next.steps = value.clone();
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
        take!(stock_solid);
        take!(steps);
        take!(tool_solids);
        take!(resolved_up_to);
        take!(selected_id);
        take!(selected_face_id);
        take!(active_utility_id);
        take!(selection_method);
        take!(engagement_input);
        take!(camera_position_x);
        take!(camera_position_y);
        take!(camera_position_z);
        take!(camera_target_x);
        take!(camera_target_y);
        take!(camera_target_z);
        take!(camera_fov);
        take!(sun_enabled);
        take!(sun_azimuth);
        take!(sun_elevation);
        take!(sun_intensity);
        take!(sun_color);
        take!(locale);
        take!(contributions_json);
        take!(hovered_id);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff {
        artifact: Some(Box::new(Process3dArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_whole_artifact_diff_wins_over_every_field_diff() {
        let base = crate::artifacts::process3d::empty_process3d_snapshot();
        let replacement = Process3dSnapshot { stock_label: "Beam".into(), ..crate::artifacts::process3d::empty_process3d_snapshot() };
        let mut diff = Process3dDiff {
            stock_label: Some("Ignored".into()),
            ..Default::default()
        };
        diff.absorb(diff_set_snapshot(&replacement));
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), replacement);
    }

    #[test]
    fn stock_solid_handle_swap_applies() {
        let base = crate::artifacts::process3d::empty_process3d_snapshot();
        let new_content = crate::artifacts::process3d::brep_snapshot_for_working_solid(&crate::artifacts::process3d::WorkingSolid::Sphere { radius: 0.5 });
        let new_handle = crate::artifacts::process3d::brep_child_handle("stock", &new_content);
        let diff = Process3dDiff { stock_solid: Some(new_handle.clone()), ..Default::default() };
        let next = diff.apply(&base).expect("valid mutation diff");
        assert_eq!(next.stock_solid, new_handle);
    }
}
//#endregion 🧪️Tests
