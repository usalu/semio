//! 🔺️ Forms artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::forms::schema::FormsArtifact;
use crate::artifacts::forms::{FormStep, FormsSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

pub type FormDiff = FormsDiff;

//#region 🔖️Apply
pub fn apply_steps_delta(items: &[FormStep], delta: &FormsStepsDelta) -> Vec<FormStep> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|item| item.id != *id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(step) = next.iter_mut().find(|step| step.id == entry.id) {
            if let Some(title) = &entry.patch.title {
                step.title = title.clone();
            }
            if let Some(description) = &entry.patch.description {
                step.description = description.clone();
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next.into_iter().map(|item| (item.id.clone(), item)).collect();
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

fn absorb_steps_delta(target: &mut Option<FormsStepsDelta>, incoming: Option<FormsStepsDelta>) {
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

impl FormsDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &FormsArtifact) -> FormsArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = self.current_step_index {
            next.current_step_index = value;
        }
        if let Some(value) = &self.try_values_json {
            next.try_values_json = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        next
    }
}

impl MutationDiff<FormsSnapshot> for FormsDiff {
    fn apply(&self, snapshot: &FormsSnapshot) -> FormsSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(version) = &self.version {
            next.version = version.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_steps_delta(&mut self.steps, other.steps);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(version);
        take!(title);
        take!(selected_ids);
        take!(current_step_index);
        take!(try_values_json);
        take!(locale);
        take!(contributions_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &FormsSnapshot) -> FormsDiff {
    FormsDiff {
        artifact: Some(Box::new(FormsArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

pub fn sparse_diff_between(before: &FormsSnapshot, after: &FormsSnapshot) -> FormsDiff {
    if before == after {
        return FormsDiff::default();
    }
    let mut diff = FormsDiff::default();
    if before.schema != after.schema {
        diff.schema = Some(after.schema.clone());
    }
    if before.id != after.id {
        diff.id = Some(after.id.clone());
    }
    if before.version != after.version {
        diff.version = Some(after.version.clone());
    }
    if before.title != after.title {
        diff.title = Some(after.title.clone());
    }
    if before.steps != after.steps {
        if steps_need_whole_replacement(&before.steps, &after.steps) {
            return diff_set_snapshot(after);
        }
        diff.steps = Some(steps_collection_delta(&before.steps, &after.steps));
    }
    diff
}

fn steps_need_whole_replacement(before: &[FormStep], after: &[FormStep]) -> bool {
    for step in after {
        if let Some(prev) = before.iter().find(|p| p.id == step.id) {
            if prev.blocks != step.blocks {
                return true;
            }
        }
    }
    false
}

fn steps_collection_delta(before: &[FormStep], after: &[FormStep]) -> FormsStepsDelta {
    let before_ids: std::collections::BTreeSet<_> = before.iter().map(|s| s.id.as_str()).collect();
    let after_ids: std::collections::BTreeSet<_> = after.iter().map(|s| s.id.as_str()).collect();
    let removed: Vec<String> = before_ids.difference(&after_ids).map(|id| (*id).to_string()).collect();
    let added: Vec<FormStep> = after.iter().filter(|step| !before_ids.contains(step.id.as_str())).cloned().collect();
    let mut patched = Vec::new();
    for step in after {
        if let Some(prev) = before.iter().find(|p| p.id == step.id) {
            if prev.title != step.title || prev.description != step.description {
                patched.push(FormsStepPatchEntry {
                    id: step.id.clone(),
                    patch: FormsStepPatch {
                        title: if prev.title != step.title { Some(step.title.clone()) } else { None },
                        description: if prev.description != step.description { Some(step.description.clone()) } else { None },
                    },
                });
            }
        }
    }
    let order: Vec<String> = after.iter().map(|s| s.id.clone()).collect();
    let prev_order: Vec<String> = before.iter().map(|s| s.id.clone()).collect();
    let reordered = if order != prev_order { Some(order) } else { None };
    FormsStepsDelta { added, removed, patched, reordered }
}

pub fn diff_from_mutation(base: &FormsSnapshot, mutation: &crate::artifacts::forms::mutations::FormMutation) -> FormsDiff {
    let next = crate::artifacts::forms::mutations::apply_form_edit_mutation(base, mutation);
    sparse_diff_between(base, &next)
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::{mutations::FormMutation, FormStep, FORMS_DOCUMENT_SCHEMA};
    use protocol::Mutation;

    #[test]
    fn empty_diff_is_a_no_operation() {
        let base = FormsSnapshot::default();
        let diff = FormsDiff::default();
        assert_eq!(diff.apply(&base), base);
    }

    #[test]
    fn add_step_diff_applies_onto_the_base_snapshot() {
        let base = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, steps: Vec::new() };
        let step = FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() };
        let operation = FormMutation::AddStep { step, index: None };
        let diff: FormsDiff = operation.diff(&base);
        assert_eq!(diff.apply(&base).steps.len(), 1);
    }
}
//#endregion 🧪️Tests
