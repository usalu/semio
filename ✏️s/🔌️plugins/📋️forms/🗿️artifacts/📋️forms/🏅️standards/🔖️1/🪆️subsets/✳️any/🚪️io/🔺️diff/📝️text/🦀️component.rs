//! 🔺️ Forms artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::forms::schema::diff::{FormsDiff, FormsStepPatch, FormsStepPatchEntry, FormsStepsDelta};
use crate::artifacts::forms::schema::FormsArtifact;
use crate::artifacts::forms::{forms_children_from_steps, forms_steps, FormStep, FormsSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub type FormDiff = FormsDiff;

//#region 🔖️Apply
/// 🧬️ Pure `Vec<FormStep>` transform — UNCHANGED by the composition migration (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM): every mutation triad still builds a
/// `FormsStepsDelta` exactly as before and applies it here; only the CALLER now sources `items`
/// from the working-scene accessor (`forms_steps`/`forms_artifact_steps`) instead of a snapshot
/// field, and wraps the result into composed children via [`forms_diff_from_delta`] below.
pub async fn apply_steps_delta(items: &[FormStep], delta: &FormsStepsDelta) -> Vec<FormStep> {
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
            if let Some(blocks) = &entry.patch.blocks {
                step.blocks = blocks.clone();
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

/// 🏗️ Builds a [`FormsDiff`] carrying regenerated `structure`/`results` handles from a
/// `FormsStepsDelta` applied against `base`'s working-scene steps — the standard way every
/// mutation triad's `diff_*` function produces its result (replaces the old
/// `FormsDiff{steps: Some(delta), ..}` literal).
pub async fn forms_diff_from_delta(delta: FormsStepsDelta, base: &FormsSnapshot) -> FormsDiff {
    let next_steps = apply_steps_delta(&forms_steps(base), &delta);
    let (structure, results) = forms_children_from_steps(&next_steps);
    FormsDiff { structure: Some(structure), results: Some(results), ..Default::default() }
}

impl FormsDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &FormsArtifact) -> protocol::MutationApplyResult<FormsArtifact> {
        Ok({
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
            if let Some(structure) = &self.structure {
                next.structure = structure.clone();
            }
            if let Some(results) = &self.results {
                next.results = results.clone();
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
        })
    }
}

impl MutationDiff<FormsSnapshot> for FormsDiff {
    async fn apply(&self, snapshot: &FormsSnapshot) -> protocol::MutationApplyResult<FormsSnapshot> {
        Ok({
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
            if let Some(structure) = &self.structure {
                next.structure = structure.clone();
            }
            if let Some(results) = &self.results {
                next.results = results.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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
        take!(structure);
        take!(results);
        take!(selected_ids);
        take!(current_step_index);
        take!(try_values_json);
        take!(locale);
        take!(contributions_json);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🔎️ Sparse diff between two full snapshots, expressed via the SAME granular
/// `FormsStepsDelta` every mutation triad builds — never a whole-document replace (that vocabulary
/// is banned; see `FormsDiff`'s own doc comment for the composition-era shape).
pub async fn sparse_diff_between(before: &FormsSnapshot, after: &FormsSnapshot) -> FormsDiff {
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
    let before_steps = forms_steps(before);
    let after_steps = forms_steps(after);
    if before_steps != after_steps {
        let (structure, results) = forms_children_from_steps(&after_steps);
        diff.structure = Some(structure);
        diff.results = Some(results);
    }
    diff
}

/// 🔎️ The granular, id-keyed shape of a `before`→`after` steps change — used by callers that want
/// the sparse delta itself (e.g. a future real `ArtifactView::with_children` seam, or diagnostics),
/// kept alongside [`sparse_diff_between`] though the latter no longer stores it on `FormsDiff`
/// (composed children are whole-slot-replace at the wire level; see this file's `apply`/`absorb`).
pub async fn steps_collection_delta(before: &[FormStep], after: &[FormStep]) -> FormsStepsDelta {
    let before_ids: std::collections::BTreeSet<_> = before.iter().map(|s| s.id.as_str()).collect();
    let after_ids: std::collections::BTreeSet<_> = after.iter().map(|s| s.id.as_str()).collect();
    let removed: Vec<String> = before_ids.difference(&after_ids).map(|id| (*id).to_string()).collect();
    let added: Vec<FormStep> = after.iter().filter(|step| !before_ids.contains(step.id.as_str())).cloned().collect();
    let mut patched = Vec::new();
    for step in after {
        if let Some(prev) = before.iter().find(|p| p.id == step.id) {
            if prev.title != step.title || prev.description != step.description || prev.blocks != step.blocks {
                patched.push(FormsStepPatchEntry {
                    id: step.id.clone(),
                    patch: FormsStepPatch {
                        title: if prev.title != step.title { Some(step.title.clone()) } else { None },
                        description: if prev.description != step.description { Some(step.description.clone()) } else { None },
                        blocks: if prev.blocks != step.blocks { Some(step.blocks.clone()) } else { None },
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

//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::mutations::create_step;
    use crate::artifacts::forms::{mutations::FormMutation, FormStep, FORMS_DOCUMENT_SCHEMA};
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn empty_diff_is_a_no_operation() {
        let base = FormsSnapshot::default();
        let diff = FormsDiff::default();
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_diff_applies_onto_the_base_snapshot() {
        let base = crate::artifacts::forms::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, Vec::new());
        let step = FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() };
        let operation = FormMutation::CreateStep(create_step::mutation::CreateStep { step, index: None });
        let diff: FormsDiff = operation.diff(&base).into_parts().0;
        assert_eq!(crate::artifacts::forms::forms_steps(&diff.apply(&base).expect("valid mutation diff")).len(), 1);
    }
}
//#endregion 🧪️Tests
