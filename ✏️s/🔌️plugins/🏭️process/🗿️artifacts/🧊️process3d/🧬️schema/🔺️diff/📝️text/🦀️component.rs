//! 🔺️ Process3d artifact — sparse field-delta diff codec and apply/absorb.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::process3d::schema::diff::*;


use crate::artifacts::process3d::schema::Process3dArtifact;
use crate::artifacts::process3d::{ProcessStep, ProcessStepPatch, Process3dSnapshot, Workshop};
use protocol::{CollectionMutation, MutationDiff, Patchable};

//#region 🔖️Apply
/// 🧩 Applies an identified-collection delta to a step list.
pub fn apply_steps_delta(items: &[ProcessStep], delta: &Process3dStepsDelta) -> Vec<ProcessStep> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|item| &item.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(item) = next.iter_mut().find(|item| item.id == entry.id) {
            item.apply_patch(&entry.patch);
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id.clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn absorb_steps_delta(target: &mut Option<Process3dStepsDelta>, incoming: Option<Process3dStepsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

impl Process3dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Process3dArtifact) -> Process3dArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(workshop) = &self.workshop {
            next.workshop = workshop.clone();
        }
        if let Some(stock) = &self.stock {
            next.stock = stock.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
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
        if let Some(cursor) = next.resolved_up_to {
            next.resolved_up_to = Some(cursor.min(next.steps.len()));
        }
        next
    }
}

impl MutationDiff<Process3dSnapshot> for Process3dDiff {
    fn apply(&self, snapshot: &Process3dSnapshot) -> Process3dSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(workshop) = &self.workshop {
            next.workshop = workshop.clone();
        }
        if let Some(stock) = &self.stock {
            next.stock = stock.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        if let Some(value) = &self.resolved_up_to {
            next.resolved_up_to = *value;
        }
        if let Some(cursor) = next.resolved_up_to {
            next.resolved_up_to = Some(cursor.min(next.steps.len()));
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        if other.workshop.is_some() { self.workshop = other.workshop; }
        if other.stock.is_some() { self.stock = other.stock; }
        absorb_steps_delta(&mut self.steps, other.steps);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
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
/// 🧩 Builds a steps delta from a collection mutation against the pre-state list.
pub fn steps_delta_from_collection_mutation(
    base: &[ProcessStep],
    op: &CollectionMutation<String, ProcessStep, ProcessStepPatch>,
) -> Process3dStepsDelta {
    match op {
        CollectionMutation::Add { item, .. } => Process3dStepsDelta { added: vec![item.clone()], ..Default::default() },
        CollectionMutation::Remove { id } => Process3dStepsDelta { removed: vec![id.clone()], ..Default::default() },
        CollectionMutation::Patch { id, patch } => Process3dStepsDelta {
            patched: vec![Process3dStepPatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|s| s.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            Process3dStepsDelta { reordered: Some(ids), ..Default::default() }
        }
    }
}

/// 🏭️ Applies a machines collection mutation onto a workshop clone.
pub fn workshop_after_machines_mutation(
    workshop: &Workshop,
    op: &CollectionMutation<String, crate::artifacts::process3d::WorkshopMachine, crate::artifacts::process3d::WorkshopMachinePatch>,
) -> Workshop {
    let mut next = workshop.clone();
    protocol::apply_collection_mutation(&mut next.machines, op);
    next
}

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
    use crate::artifacts::process3d::{Pose, ProcessMeasure, SolidSpec, Stock};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep {
            id: id.into(),
            label: "Cut".into(),
            enabled: true,
            origin: None,
            measure: ProcessMeasure::Cut {
                tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 },
                pose: Pose::default(),
            },
        }
    }

    #[test]
    fn a_whole_artifact_diff_wins_over_every_field_diff() {
        let base = Process3dSnapshot {
            steps: vec![cut_step("a")],
            ..Default::default()
        };
        let replacement = Process3dSnapshot {
            stock: Stock { id: "beam".into(), label: "Beam".into(), solid: SolidSpec::Box { width: 1.0, depth: 0.1, height: 0.2 }, pose: Pose::default() },
            ..Default::default()
        };
        let mut diff = Process3dDiff {
            steps: Some(Process3dStepsDelta { removed: vec!["a".into()], ..Default::default() }),
            ..Default::default()
        };
        diff.absorb(diff_set_snapshot(&replacement));
        assert_eq!(diff.apply(&base), replacement);
    }

    #[test]
    fn steps_delta_add_remove_applies() {
        let base = Process3dSnapshot { steps: vec![cut_step("a")], ..Default::default() };
        let mut diff = Process3dDiff {
            steps: Some(Process3dStepsDelta { removed: vec!["a".into()], ..Default::default() }),
            ..Default::default()
        };
        diff.absorb(Process3dDiff {
            steps: Some(Process3dStepsDelta { added: vec![cut_step("b")], ..Default::default() }),
            ..Default::default()
        });
        let next = diff.apply(&base);
        assert_eq!(next.steps.len(), 1);
        assert_eq!(next.steps[0].id, "b");
    }
}
//#endregion 🧪️Tests
