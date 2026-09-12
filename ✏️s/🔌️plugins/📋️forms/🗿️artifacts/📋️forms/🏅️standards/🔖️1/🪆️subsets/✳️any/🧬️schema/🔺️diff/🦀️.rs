//! 🧬️ Forms diff schema — sparse field delta over the artifact.

use crate::{forms_children_from_steps, forms_steps, FormQuestion, FormStep, FormsResultsChild, FormsStructureChild, FormsSnapshot};
use crate::schema::FormsArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the forms artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
///
/// Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the old
/// `steps: Option<FormsStepsDelta>` field (an id-keyed sparse collection delta) and the dead
/// whole-snapshot-replace `artifact: Option<Box<FormsArtifact>>` slot (the banned `SetSnapshot`
/// vocabulary — grepped: never constructed by any app command, only by this file's own now-removed
/// `diff_set_snapshot`/`sparse_diff_between` dead code) are both removed. `structure`/`results`
/// (`Option<ArtifactChild<S>>`, single-Option "always-present slot" shape) replace them: every
/// mutation triad still builds its change as a `FormsStepsDelta` internally (that type is UNCHANGED,
/// see `🔖️DeltaHelpers` below) and applies it against the WORKING-SCENE steps
/// (`crate::forms_steps`, not a snapshot field) to get the resulting `Vec<FormStep>`,
/// then regenerates both composed children from that result — the granular, cascade-aware mutation
/// semantics are unchanged, only the diff's own wire representation of "what changed" becomes a
/// pair of regenerated content-addressed handles, exactly like every other composed plugin in this
/// ticket (see `crate::🔖️Composition`'s own doc comment).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsDiff {
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub structure: Option<FormsStructureChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub results: Option<FormsResultsChild>,
}

impl dsl::FromValue for FormsDiff {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut schema = None;
        let mut id = None;
        let mut version = None;
        let mut title = None;
        let mut structure = None;
        let mut results = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "id" if id.is_none() => id = Some(dsl::FromValue::from_value(value)?),
                "version" if version.is_none() => version = Some(dsl::FromValue::from_value(value)?),
                "title" if title.is_none() => title = Some(dsl::FromValue::from_value(value)?),
                "structure" if structure.is_none() => structure = Some(dsl::FromValue::from_value(value)?),
                "results" if results.is_none() => results = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(format!("unknown or duplicate Forms field {key}"))),
            }
        }
        let result = Self { schema, id, version, title, structure, results };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}

impl FormsDiff {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema.as_deref().is_some_and(|value| value != "forms.form") { return Err("invalid Forms document marker".into()); }
        if let Some(child) = &self.structure { validate_semio_child_identity(&child.child_id, &child.target, "value")?; }
        if let Some(child) = &self.results { validate_semio_child_identity(&child.child_id, &child.target, "table")?; }
        Ok(())
    }
}

//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `steps`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsStepsDelta {
    pub added: Vec<FormStep>,
    pub removed: Vec<String>,
    pub patched: Vec<FormsStepPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched step entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FormsStepPatchEntry {
    pub id: String,
    pub patch: FormsStepPatch,
}

/// 🩹 Partial step replacement. `blocks`, when set, is the step's FULL new `blocks` list — a
/// bounded, single-step-scoped whole-value swap (mirrors how a sibling facet's `EquationDiff`
/// replaces a whole bounded sub-collection rather than diffing every element field-by-field), never
/// a whole-DOCUMENT replacement: every `🧬️mutations/*create-block/*delete-block/*move-block-to-step`
/// triad leaf builds this by cloning only the touched step(s)' own `blocks` Vec, not `FormsSnapshot`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsStepPatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub blocks: Option<Vec<FormQuestion>>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
/// 🧬️ Pure `Vec<FormStep>` transform — UNCHANGED by the composition migration (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM): every mutation triad still builds a
/// `FormsStepsDelta` exactly as before and applies it here; only the CALLER now sources `items`
/// from the working-scene accessor (`forms_steps`/`forms_artifact_steps`) instead of a snapshot
/// field, and wraps the result into composed children via [`forms_diff_from_delta`] below.
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
pub fn forms_diff_from_delta(delta: &FormsStepsDelta, base: &FormsSnapshot) -> FormsDiff {
    let next_steps = apply_steps_delta(&forms_steps(base), delta);
    let (structure, results) = forms_children_from_steps(&next_steps);
    FormsDiff { structure: Some(structure), results: Some(results), ..Default::default() }
}

impl FormsDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &FormsArtifact) -> protocol::MutationApplyResult<FormsArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.child-identity".into(), message, target: Vec::new() })?;
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
            next
        })
    }
}

impl MutationDiff<FormsSnapshot> for FormsDiff {
    fn apply(&self, snapshot: &FormsSnapshot) -> protocol::MutationApplyResult<FormsSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.child-identity".into(), message, target: Vec::new() })?;
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
    fn absorb(&mut self, other: Self) {
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
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🔎️ Sparse diff between two full snapshots, expressed via the SAME granular
/// `FormsStepsDelta` every mutation triad builds — never a whole-document replace (that vocabulary
/// is banned; see `FormsDiff`'s own doc comment for the composition-era shape).
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
pub fn steps_collection_delta(before: &[FormStep], after: &[FormStep]) -> FormsStepsDelta {
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


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
