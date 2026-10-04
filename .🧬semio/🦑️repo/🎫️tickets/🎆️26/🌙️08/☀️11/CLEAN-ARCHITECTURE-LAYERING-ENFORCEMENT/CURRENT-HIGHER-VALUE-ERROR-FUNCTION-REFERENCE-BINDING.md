# Higher Value Constructor Function Reference Binding

Removed obsolete ValueError constructor-function rebuilding around actual NativeEncodeControl/NativeDecodeControl results. Canonical control errors propagate directly. Actual private Forms/Curation/DAG validators were read with child/definition/response producers and own explicit InvalidValue schema admission conversions. Full original methods, retirement behavior and success laws retained; native whole higher replay pending.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/higher-value-error-function-reference-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

```rust
//! 🧬️ Forms diff schema — sparse field delta over the artifact.

use crate::{forms_children_from_steps, forms_steps, FormQuestion, FormStep, FormsResultsChild, FormsStructureChild, FormsSnapshot};
use crate::schema::FormsArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse durable domain edits and their derived child projections.
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
    #[value(skip_serializing_if = "Option::is_none")]
    pub definition: Option<crate::schema::definition::FormsDefinition>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<crate::schema::response::FormsResponse>>,
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
        let mut definition = None;
        let mut responses = None;
        let mut structure = None;
        let mut results = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "id" if id.is_none() => id = Some(dsl::FromValue::from_value(value)?),
                "version" if version.is_none() => version = Some(dsl::FromValue::from_value(value)?),
                "title" if title.is_none() => title = Some(dsl::FromValue::from_value(value)?),
                "definition" if definition.is_none() => definition = Some(dsl::FromValue::from_value(value)?),
                "responses" if responses.is_none() => responses = Some(dsl::FromValue::from_value(value)?),
                "structure" if structure.is_none() => structure = Some(dsl::FromValue::from_value(value)?),
                "results" if results.is_none() => results = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Forms field {key}"))),
            }
        }
        let result = Self { schema, id, version, title, definition, responses, structure, results };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}

impl FormsDiff {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema.as_deref().is_some_and(|value| value != "forms.form") { return Err("invalid Forms document marker".into()); }
        if let Some(definition) = &self.definition { definition.validate()?; }
        if let Some(responses) = &self.responses {
            let mut ids = std::collections::HashSet::new();
            for response in responses { response.validate()?; if !ids.insert(&response.id) { return Err("duplicate response id".into()); } }
        }
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
    let (structure, _) = forms_children_from_steps(&next_steps);
    FormsDiff { definition: Some(crate::schema::definition::FormsDefinition { steps: next_steps }), structure: Some(structure), ..Default::default() }
}

impl FormsDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &FormsArtifact) -> protocol::MutationApplyResult<FormsArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
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
            if let Some(definition) = &self.definition { next.definition = definition.clone(); }
            if let Some(responses) = &self.responses { next.responses = responses.clone(); }
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
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
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
            if let Some(definition) = &self.definition { next.definition = definition.clone(); }
            if let Some(responses) = &self.responses { next.responses = responses.clone(); }
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
        take!(definition);
        take!(responses);
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
    if before.responses != after.responses { diff.responses = Some(after.responses.clone()); diff.results = Some(after.results.clone()); }
    let before_steps = forms_steps(before);
    let after_steps = forms_steps(after);
    if before_steps != after_steps {
        let (structure, _) = forms_children_from_steps(&after_steps);
        diff.definition = Some(after.definition.clone());
        diff.structure = Some(structure);
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

```

## ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ Forms snapshot schema — artifact-lane fields only.

use crate::{forms_snapshot_with_state, FormsResultsChild, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
#[path="📦️pack/🦀️.rs"]pub(crate) mod native_pack;

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

//#region 🔖️Snapshot
/// 📸️ Durable form definition and immutable responses with derived value/table child projections.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "forms")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    pub definition: crate::schema::definition::FormsDefinition,
    #[state(artifact)]
    pub responses: Vec<crate::schema::response::FormsResponse>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub results: FormsResultsChild,
}

impl dsl::FromValue for FormsSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut schema = None;
        let mut id = None;
        let mut version = None;
        let mut title = None;
        let mut definition = None;
        let mut responses = None;
        let mut structure = None;
        let mut results = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "id" if id.is_none() => id = Some(dsl::FromValue::from_value(value)?),
                "version" if version.is_none() => version = Some(dsl::FromValue::from_value(value)?),
                "title" if title.is_none() => title = Some(dsl::FromValue::from_value(value)?),
                "definition" if definition.is_none() => definition = Some(dsl::FromValue::from_value(value)?),
                "responses" if responses.is_none() => responses = Some(dsl::FromValue::from_value(value)?),
                "structure" if structure.is_none() => structure = Some(dsl::FromValue::from_value(value)?),
                "results" if results.is_none() => results = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Forms field {key}"))),
            }
        }
        let result = Self { schema: schema.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms schema"))?, id: id.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms id"))?, version: version.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms version"))?, title: title.unwrap_or(None), definition: definition.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms definition"))?, responses: responses.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms responses"))?, structure: structure.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms structure"))?, results: results.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Forms results"))? };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}

impl FormsSnapshot {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema != "forms.form" { return Err("invalid Forms document marker".into()); }
        self.definition.validate()?;
        let mut responses = std::collections::HashSet::new();
        for response in &self.responses { response.validate()?; if !responses.insert(&response.id) { return Err("duplicate response id".into()); } }
        validate_semio_child_identity(&self.structure.child_id, &self.structure.target, "value")?;
        validate_semio_child_identity(&self.results.child_id, &self.results.target, "table")?;
        Ok(())
    }
}


impl Default for FormsSnapshot {
    fn default() -> Self {
        forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[])
    }
}
//#endregion 🔖️Snapshot

// 🧬️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (design.md §1 CORRECTION): the native
// `store::ArtifactDsl`/`store::ArtifactPack` codec impls (and their hex/LEB128 primitives) moved to
// `🚪️io/📸️snapshot/{📝️text,💾️binary}` — this facet root keeps only the struct + pure defaults, no
// codecs (design.md rule: `🧬️schema` is types + pure transforms only). Their round-trip tests moved
// with them.

#[cfg(test)]
#[path = "../🧪️tests/💾️persistence/🦀️.rs"]
mod persistence_tests;

```

## ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

```rust
//! 🧬️ Curation diff schema — sparse field delta over the artifact.

use crate::{CuratedItem, ObjectKindExtra, CurationSnapshot};
use crate::schema::CurationArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse parent delta for catalog identity, sourcing entries and selection.
/// Kit content changes belong to the child's own mutation history.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::CurationArtifact>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub stock_extra: Option<CurationStockExtraDelta>,
    #[state(artifact)]
    pub curated: Option<CurationCuratedDelta>,
}
impl dsl::FromValue for CurationDiff {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut result = Self::default();
        let mut seen = 0u8;
        for (key, value) in dsl::DslValue::into_object(value)? {
            let bit = match key.as_str() { "artifact" => 1, "catalog" => 2, "stockExtra" => 4, "curated" => 8, _ => return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Curation diff field {key}"))) };
            if seen & bit != 0 { return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("duplicate Curation diff field {key}"))); }
            seen |= bit;
            match key.as_str() {
                "artifact" => result.artifact = dsl::FromValue::from_value(value)?,
                "catalog" => result.catalog = dsl::FromValue::from_value(value)?,
                "stockExtra" => result.stock_extra = dsl::FromValue::from_value(value)?,
                "curated" => result.curated = dsl::FromValue::from_value(value)?,
                _ => unreachable!(),
            }
        }
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}
impl CurationDiff {
    /// 🛡 Checks typed parent replacements before applying document changes.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if let Some(artifact) = &self.artifact { validate_semio_child_identity(&artifact.catalog.child_id, &artifact.catalog.target, "kit")?; }
        if let Some(catalog) = &self.catalog { validate_semio_child_identity(&catalog.child_id, &catalog.target, "kit")?; }
        Ok(())
    }
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🩹 One patched stock-extra entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationObjectKindExtraPatchEntry {
    pub id: String,
    pub extra: ObjectKindExtra,
}

/// 🧩 Identified-collection delta for `stock_extra`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CurationStockExtraDelta {
    pub added: Vec<ObjectKindExtra>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationObjectKindExtraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationCuratedPatchEntry {
    pub object_id: String,
    pub count: Option<u32>,
}

/// 🧺 Identified-collection delta for `curated`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CurationCuratedDelta {
    pub added: Vec<CuratedItem>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationCuratedPatchEntry>,
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
pub fn apply_stock_extra_delta(stock_extra: &[ObjectKindExtra], delta: &CurationStockExtraDelta) -> protocol::MutationApplyResult<Vec<ObjectKindExtra>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "stock entry is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if !stock_extra.iter().any(|extra| &extra.id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed stock entry does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<_> = stock_extra.iter().map(|extra| extra.id.clone()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, extra) in delta.added.iter().enumerate() {
        if !identities.insert(extra.id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added stock entry identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "stock entry is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.id.as_str()) || !identities.contains(&entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched stock entry does not exist").at(["patched".to_string(), index.to_string()]));
        }
        if entry.extra.id != entry.id {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "stock entry patch cannot change its identity").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<_> = stock_extra.iter().filter(|extra| !removed.contains(extra.id.as_str())).cloned().collect();
    next.extend(delta.added.iter().cloned());
    for entry in &delta.patched {
        let target = next
            .iter_mut()
            .find(|extra| extra.id == entry.id)
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched stock entry does not exist after structural edits").at(["patched".to_string(), entry.id.clone()]))?;
        *target = entry.extra.clone();
    }
    reorder_named(next, delta.reordered.as_deref(), |extra| extra.id.as_str())
}

pub fn apply_curated_delta(curated: &[CuratedItem], delta: &CurationCuratedDelta) -> protocol::MutationApplyResult<Vec<CuratedItem>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "curated item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if !curated.iter().any(|item| &item.object_id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed curated item does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<_> = curated.iter().map(|item| item.object_id.clone()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, item) in delta.added.iter().enumerate() {
        if !identities.insert(item.object_id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added curated item identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.object_id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "curated item is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.object_id.as_str()) || !identities.contains(&entry.object_id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched curated item does not exist").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<_> = curated.iter().filter(|item| !removed.contains(item.object_id.as_str())).cloned().collect();
    next.extend(delta.added.iter().cloned());
    for entry in &delta.patched {
        let target = next
            .iter_mut()
            .find(|item| item.object_id == entry.object_id)
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched curated item does not exist after structural edits").at(["patched".to_string(), entry.object_id.clone()]))?;
        if let Some(count) = entry.count {
            target.count = count;
        }
    }
    reorder_named(next, delta.reordered.as_deref(), |item| item.object_id.as_str())
}

fn reorder_named<T>(items: Vec<T>, order: Option<&[String]>, id: impl for<'a> Fn(&'a T) -> &'a str) -> protocol::MutationApplyResult<Vec<T>> {
    let Some(order) = order else {
        return Ok(items);
    };
    if order.len() != items.len() || order.iter().enumerate().any(|(index, target)| order[..index].contains(target) || !items.iter().any(|item| id(item) == target)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
    }
    let mut remaining = items;
    let mut ordered = Vec::with_capacity(order.len());
    for target in order {
        let index = remaining.iter().position(|item| id(item) == target).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered item does not exist").at(["reordered".to_string(), target.clone()]))?;
        ordered.push(remaining.remove(index));
    }
    Ok(ordered)
}

impl CurationDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &CurationArtifact) -> protocol::MutationApplyResult<CurationArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.child-identity", message))?;
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(handle) = &self.catalog {
                next.catalog = handle.clone();
            }
            if let Some(delta) = &self.stock_extra {
                next.stock_extra = apply_stock_extra_delta(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
            }
            if let Some(delta) = &self.curated {
                next.curated = apply_curated_delta(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
            }
            next
        })
    }
}

/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub fn diff_set_snapshot(snapshot: &CurationSnapshot) -> CurationDiff {
    CurationDiff { artifact: Some(Box::new(CurationArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

impl MutationDiff<CurationSnapshot> for CurationDiff {
    fn apply(&self, snapshot: &CurationSnapshot) -> protocol::MutationApplyResult<CurationSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.child-identity", message))?;
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(handle) = &self.catalog {
                next.catalog = handle.clone();
            }
            if let Some(delta) = &self.stock_extra {
                next.stock_extra = apply_stock_extra_delta(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
            }
            if let Some(delta) = &self.curated {
                next.curated = apply_curated_delta(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
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
        take!(catalog);
        match (&mut self.stock_extra, other.stock_extra) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);








                let removed_ids: std::collections::BTreeSet<&str> = dst.removed.iter().map(String::as_str).collect();
                dst.patched.retain(|entry| !removed_ids.contains(entry.id.as_str()));
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.stock_extra = Some(src),
            _ => {}
        }
        match (&mut self.curated, other.curated) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);

                dst.patched.retain(|entry| !dst.removed.iter().any(|id| id == &entry.object_id));
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.curated = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

```

## ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ Curation snapshot schema — artifact-lane fields only.

use crate::{CuratedItem, ObjectKindExtra};
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Snapshot
/// 📸️ Persisted Kit catalog child, sourcing geometry and availability, and ordered selection.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "curation.curation", layout = "lines")]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationSnapshot {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub curated: Vec<CuratedItem>,
}

impl Default for CurationSnapshot {
    /// 🌱 Builds the empty catalog and selection through the document's child constructor.
    fn default() -> Self {
        Self { catalog: crate::catalog_child_handle(&[]), stock_extra: Vec::new(), curated: Vec::new() }
    }
}

impl dsl::FromValue for CurationSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut catalog = None;
        let mut stock_extra = None;
        let mut curated = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "catalog" if catalog.is_none() => catalog = Some(dsl::FromValue::from_value(value)?),
                "stockExtra" if stock_extra.is_none() => stock_extra = Some(dsl::FromValue::from_value(value)?),
                "curated" if curated.is_none() => curated = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Curation field {key}"))),
            }
        }
        let result = Self { catalog: catalog.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing catalog"))?, stock_extra: stock_extra.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing stockExtra"))?, curated: curated.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing curated"))? };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}
impl CurationSnapshot {
    /// 🪆 Requires the persisted child target to use the Kit dialect.
    pub fn validate(&self) -> Result<(), String> {
        semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&self.catalog.child_id, &self.catalog.target, "kit")
    }
}
/// 🧺 Formats a persisted ordered selection for scenario diagnostics.
pub fn curation_selection_summary(snapshot: &CurationSnapshot) -> String {
    snapshot.curated.iter().map(|item| format!("{}x{}", item.object_id, item.count)).collect::<Vec<_>>().join(" ")
}
#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;
#[cfg(test)]
#[path = "../🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

```

## ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ Jack snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes`/`edges` are gone from this STRUCT —
//! replaced by a single composed `content: JackContentChild` slot (`s.stdio.semio.graph`). See
//! `🗿️artifacts/🔌️jack/🦀️.rs`'s `🔖️ContentBridge`/`🔖️WorkingScene` regions for the
//! converter/handle/cache machinery this field depends on.

use crate::{Camera, JackContentChild, Manifest};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted trinity graph document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub name: String,
    #[state(artifact)]
    pub manifest_id: Option<String>,
    #[state(artifact)]
    pub manifest: Manifest,
    #[state(artifact)]
    pub camera: Camera,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: JackContentChild,
    #[state(artifact)]
    pub root_node_id: Option<String>,
    /// 🔎️ The document's Jack query — the query editor's text, document content like the graph it runs against.
    #[state(artifact)]
    pub query: String,
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `content` is a `store::ArtifactChild<S>` composed-artifact
/// handle — see `JackArtifact`'s identical trap in the sibling `🦀️.rs` (this struct's
/// non-child fields carried `#[serde(default, skip_serializing_if = "Option::is_none")]`
/// before this wave; `manifest_id`/`root_node_id` are `Option<String>`, and the blanket
/// `impl<T: FromValue> FromValue for Option<T>` already treats a missing key as `None` via the
/// derive macro's own generated `missing` arm being unreachable here since every field is
/// present below — no separate default handling needed in a hand-written impl).
impl dsl::ToValue for JackSnapshot {
    fn to_value_controlled(&self, c: &mut dsl::NativeEncodeControl<'_>) -> Result<dsl::DslValue, dsl::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8).map_err(dsl::ValueError::new)?;
            let mut fields = dsl::DslValue::object_encoding_controlled(8, c)?;
            macro_rules! field {
                ($key:literal,$value:expr) => {{
                    let child = dsl::ToValue::to_value_controlled($value, c)?;
                    dsl::DslValue::push_encoding_controlled(fields.get_mut(), $key, child, c)?;
                    c.step().map_err(dsl::ValueError::new)?;
                }};
            }
            field!("schema", &self.schema);
            field!("name", &self.name);
            if let Some(value) = &self.manifest_id {
                field!("manifestId", value)
            } else {
                c.step().map_err(dsl::ValueError::new)?;
            }
            field!("manifest", &self.manifest);
            field!("camera", &self.camera);
            field!("content", &self.content);
            if let Some(value) = &self.root_node_id {
                field!("rootNodeId", value)
            } else {
                c.step().map_err(dsl::ValueError::new)?;
            }
            field!("query", &self.query);
            Ok(dsl::DslValue::Object(fields.take()))
        })
    }

    /// 🕳️ `manifestId`/`rootNodeId` are SKIPPED while `None` (the old `skip_serializing_if` law the
    /// committed `📸️snapshot` fixture vectors are written against): decode→encode of a committed
    /// snapshot is a fixed point only if an absent id stays absent instead of surfacing as `null`.
    fn to_value(&self) -> dsl::DslValue {
        let mut entries: Vec<(String, dsl::DslValue)> = Vec::with_capacity(8);
        entries.push(("schema".to_string(), dsl::ToValue::to_value(&self.schema)));
        entries.push(("name".to_string(), dsl::ToValue::to_value(&self.name)));
        if let Some(manifest_id) = self.manifest_id.as_ref() {
            entries.push(("manifestId".to_string(), dsl::ToValue::to_value(manifest_id)));
        }
        entries.push(("manifest".to_string(), dsl::ToValue::to_value(&self.manifest)));
        entries.push(("camera".to_string(), dsl::ToValue::to_value(&self.camera)));
        entries.push(("content".to_string(), semio_framework_value::ToValue::to_value(&self.content)));
        if let Some(root_node_id) = self.root_node_id.as_ref() {
            entries.push(("rootNodeId".to_string(), dsl::ToValue::to_value(root_node_id)));
        }
        entries.push(("query".to_string(), dsl::ToValue::to_value(&self.query)));
        dsl::DslValue::object(entries)
    }
}
impl dsl::FromValue for JackSnapshot {
    fn from_value_controlled(value: &dsl::DslValue, c: &mut dsl::NativeDecodeControl<'_>) -> Result<Self, dsl::ValueError> {
        c.scoped_stage(|c| {
            c.begin_stage(8).map_err(dsl::ValueError::new)?;
            let fields = value.object_controlled(c)?;
            c.charge(size_of::<Self>()).map_err(dsl::ValueError::new)?;
            fn optional_field<T: dsl::FromValue + Default>(fields: &[(String, dsl::DslValue)], key: &str, c: &mut dsl::NativeDecodeControl<'_>) -> Result<T, dsl::ValueError> {
                let value = match dsl::DslValue::field_controlled(fields, key, c)? {
                    Some(value) => T::from_value_controlled(value, c)?,
                    None => T::default(),
                };
                let value = dsl::DecodedValue::new(value, T::retire_decoded);
                c.step().map_err(dsl::ValueError::new)?;
                Ok(value.take())
            }
            let schema = optional_field(fields, "schema", c)?;
            let name = optional_field(fields, "name", c)?;
            let manifest_id = optional_field(fields, "manifestId", c)?;
            let manifest = dsl::DecodedValue::new(optional_field::<Manifest>(fields, "manifest", c)?, <Manifest as dsl::FromValue>::retire_decoded);
            let camera = optional_field(fields, "camera", c)?;
            let child = dsl::DslValue::field_controlled(fields, "content", c)?.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field content"))?;
            let content = dsl::DecodedValue::new(<JackContentChild as dsl::FromValue>::from_value_controlled(child, c)?, <JackContentChild as dsl::FromValue>::retire_decoded);
            c.step().map_err(dsl::ValueError::new)?;
            let root_node_id = optional_field(fields, "rootNodeId", c)?;
            let query = dsl::DslValue::field_controlled(fields, "query", c)?.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field query"))?;
            let query = <String as dsl::FromValue>::from_value_controlled(query, c)?;
            let output = dsl::DecodedValue::new(Self { schema, name, manifest_id, manifest: manifest.take(), camera, content: content.take(), root_node_id, query }, Self::retire_decoded);
            c.step().map_err(dsl::ValueError::new)?;
            Ok(output.take())
        })
    }
    fn retire_decoded(self) {
        super::sqlite::retire_snapshot(self)
    }

    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        Ok(Self {
            schema: match get("schema") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            name: match get("name") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            manifest_id: match get("manifestId") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => None,
            },
            manifest: match get("manifest") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            camera: match get("camera") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => Default::default(),
            },
            content: semio_framework_value::FromValue::from_value(get("content").ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `content`"))?)?,
            root_node_id: match get("rootNodeId") {
                Some(v) => dsl::FromValue::from_value(v)?,
                None => None,
            },
            query: dsl::FromValue::from_value(get("query").ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `query`"))?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

impl Default for JackSnapshot {
    fn default() -> Self {
        Self {
            schema: crate::TRINITY_GRAPH_SCHEMA.into(),
            name: String::new(),
            manifest_id: None,
            manifest: Manifest::default(),
            camera: Camera::default(),
            content: crate::jack_content_child_with_owner(Vec::new(), Vec::new()),
            root_node_id: None,
            query: crate::TRINITY_JACK_DEFAULT_QUERY.into(),
        }
    }
}

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`JackSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🔌️mutate-jack-1`'s scenarios are measured through, and the shape the committed
/// `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a scene, and
/// that handle's `childId` is a digest of the child — so it moves if and only if the working scene
/// moved, which is what makes it a usable observability surface here.
///
/// A thin `pack::json` wrapper over [`JackSnapshot`]'s own `ToValue`, bridged through
/// `pack::json_from_dsl_value` since `DslValue` and `pack::json::Value` are sibling trees (used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_jack_snapshot_json(snapshot: &JackSnapshot) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&crate::standards::v1::subsets::any::io::json_native::convert(dsl::ToValue::to_value(snapshot), false).expect("typed Jack camera words")))
}

/// 📥️ The inverse of [`encode_jack_snapshot_json`] — decodes those committed specification vectors
/// into real [`JackSnapshot`] values, so `🔌️mutate-jack-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it.
pub fn decode_jack_snapshot_json(text: &str) -> Result<JackSnapshot, String> {
    let parsed = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    <JackSnapshot as dsl::FromValue>::from_value(crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_pack_json::to_dsl_value(&parsed), true)?).map_err(|error| error.to_string())
}

/// 📝️ Parses the literal Jack parent and its independent content-child address.
/// Child materialization belongs to the host's composed artifact boundary.
pub fn parse_jack_dsl(text: &str) -> Result<JackSnapshot, String> {
    <JackSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the literal parent record with its native document preamble.
pub fn print_jack_dsl(snapshot: &JackSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The scene's node names and `source -> target` edge ids the document's composed child currently
/// resolves to — the readable half of a divergence message, so a failing scenario names WHICH piece
/// moved rather than only that two content digests differ.
pub fn jack_scene_summary(snapshot: &JackSnapshot) -> String {
    let scene = crate::jack_working_scene(snapshot);
    let nodes = scene.nodes.iter().map(|node| format!("{}({})", node.name, node.id)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>().join(" ");
    format!("nodes[{nodes}] edges[{edges}]")
}
//#endregion 🌉️ExternalCodecBridge

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

```

## ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs

```rust
//! 🧬️ DAG diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `nodes: Option<DagNodesDelta>` /
//! `edges: Option<DagEdgesDelta>` / `set_nodes` / `set_edges` are all gone — the composed child is
//! opaque (a parent's diff never embeds a child diff, per `📓️design-full-plan.md` §1's CHILD/LINK
//! split), so every triad now diffs by minting a whole new `content` handle
//! (`diff_replace_content`, see `🔺️diff/📝️text`) rather than building a structured delta. Single
//! `Option<DagContentChild>` — the slot is never absent, only ever replaced, matching writer's
//! `document`/flow's `content` field shape, not lowpoly's optional-slot `Option<Option<_>>`.
//!
//! `artifact: Option<Box<DagArtifact>>` (a whole-artifact-replace escape hatch) is also gone — it was
//! already dead (never constructed anywhere; `DagPlayApp` never overrides `whole_document_operation`)
//! and is exactly the forbidden whole-document-replace-via-diff shape `📌️important.md`'s vocabulary
//! policy bans. `DagNodesDelta`/`DagEdgesDelta`/`DagNodePatchEntry`/`DagNodeExtraPatch*`/
//! `DagEdgePatchEntry`/`DagNodeSpecList`/`DagHostSnapshotEdgeList` are all dead with it — confirmed zero
//! remaining references after this pass.

use crate::{DagContentChild, DagHostSnapshotEdge, DagNodeSpec, DagSnapshot};
use crate::schema::DagArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the DAG artifact.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagDiff {
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub content: Option<DagContentChild>,
}

impl dsl::FromValue for DagDiff {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut schema = None;
        let mut content = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "content" if content.is_none() => content = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Dag field {key}"))),
            }
        }
        let result = Self { schema, content };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}

impl DagDiff {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema.as_deref().is_some_and(|value| value != "dag.dag") { return Err("invalid Dag document marker".into()); }
        if let Some(child) = &self.content { validate_semio_child_identity(&child.child_id, &child.target, "graph")?; }
        Ok(())
    }
}

//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DagStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️ReplaceContent
/// 🏗️ Every mutation triad's `🔺️diff` builder goes through this: read the current scene off `base`
/// via `crate::dag_working_scene`, apply its own specific semantics to a clone of
/// that scene, then mint+cache a whole new content handle here — the "mint+cache whole handle, never
/// apply-then-capture" pattern flow's `diff_replace_content`/writer's `diff_set_text` established.
pub fn diff_replace_content(nodes: Vec<DagNodeSpec>, edges: Vec<DagHostSnapshotEdge>) -> DagDiff {
    DagDiff { content: Some(crate::dag_content_child_with_owner(nodes, edges)), ..Default::default() }
}
//#endregion 🔖️ReplaceContent

//#region 🔖️Apply
impl DagDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DagArtifact) -> protocol::MutationApplyResult<DagArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
        Ok({
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            next
        })
    }
}

impl MutationDiff<DagSnapshot> for DagDiff {
    fn apply(&self, snapshot: &DagSnapshot) -> protocol::MutationApplyResult<DagSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
        Ok({
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
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
        take!(content);
    }
}
//#endregion 🔖️Apply


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

```

## ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🕸️ Persisted DAG marker and literal Graph child; local working scenes are separate owners.
use crate::{DagContentChild, DagHostSnapshotEdge, DagNodeSpec};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted DAG document snapshot — schema tag plus the composed `graph` content child.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "dag.dag", layout = "lines")]
#[artifact_schema(id = "s.dag.dag")]
pub struct DagSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: DagContentChild,
}

impl dsl::FromValue for DagSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut schema = None;
        let mut content = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "schema" if schema.is_none() => schema = Some(dsl::FromValue::from_value(value)?),
                "content" if content.is_none() => content = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown or duplicate Dag field {key}"))),
            }
        }
        let result = Self { schema: schema.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag schema"))?, content: content.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag content"))? };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
    fn from_value_controlled(value: &dsl::DslValue, control: &mut dsl::NativeDecodeControl<'_>) -> Result<Self, dsl::ValueError> {
        let entries = value.object_controlled(control)?;
        dsl::DslValue::deny_fields_controlled(entries, &["schema", "content"], control)?;
        control.charge(std::mem::size_of::<Self>()).map_err(dsl::ValueError::new)?;
        let schema = dsl::DslValue::field_controlled(entries, "schema", control)?.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag schema"))?;
        let schema = <String as dsl::FromValue>::from_value_controlled(schema, control)?;
        let child = dsl::DslValue::field_controlled(entries, "content", control)?.ok_or_else(|| dsl::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing Dag content"))?;
        let child = <DagContentChild as dsl::FromValue>::from_value_controlled(child, control)?;
        let result = dsl::DecodedValue::new(Self { schema, content: child }, Self::retire_decoded);
        result.get().validate().map_err(dsl::ValueError::new)?;
        control.checkpoint().map_err(dsl::ValueError::new)?;
        Ok(result.take())
    }
    fn retire_decoded(self) {
        let factory = semio_framework_value::retirement::OwnedValueRetirementFactory::<DagContentChild>::default();
        let mut cursor = store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, self.content);
        loop {
            match cursor.close_step(256, 65536).expect("DAG child closes with its exact owner") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(cursor.terminal_is_empty());
                    break;
                }
                store::SnapshotRetirementStep::Pending { .. } => {}
                store::SnapshotRetirementStep::Blocked => panic!("DAG child retirement blocked"),
            }
        }
    }
}

impl DagSnapshot {
    /// 🪆️ Enforces the document marker and exact owned-child coordinates.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if self.schema != "dag.dag" {
            return Err("invalid Dag document marker".into());
        }
        validate_semio_child_identity(&self.content.child_id, &self.content.target, "graph")?;
        Ok(())
    }
}

impl Default for DagSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> DagSnapshot {
    crate::examples::demo::snapshot()
}
/// 🫙️ The empty document — schema marker plus an owned `graph` child with no nodes or edges. The
/// example picker's "no example" selection loads this, so it must satisfy `validate()` exactly the
/// way `default_snapshot()` does.
pub fn empty_snapshot() -> DagSnapshot {
    DagSnapshot { schema: crate::DAG_DOCUMENT_SCHEMA.into(), content: crate::dag_content_child_with_owner(Vec::new(), Vec::new()) }
}
//#endregion 🔖️Snapshot

//#region 🔖️FrameworkBridge
/// 🌉 `semio_framework_artifact_infinite_dag::DagSnapshot` is the FRAMEWORK's own separate persisted
/// projection (backs `DagHostSnapshot`/`DagHost`), unrelated to and unaware of this plugin's composed
/// child — the bridge goes through the working-scene converter, never through `nodes`/`edges` fields
/// (this struct no longer has any).
impl From<DagSnapshot> for semio_framework_artifact_infinite_dag::DagSnapshot {
    fn from(value: DagSnapshot) -> Self {
        let scene = crate::dag_working_scene(&value);
        Self { schema: value.schema, nodes: scene.nodes, edges: scene.edges }
    }
}

impl From<semio_framework_artifact_infinite_dag::DagSnapshot> for DagSnapshot {
    fn from(value: semio_framework_artifact_infinite_dag::DagSnapshot) -> Self {
        let content = crate::dag_content_child_with_owner(value.nodes, value.edges);
        Self { schema: value.schema, content }
    }
}

impl From<&DagSnapshot> for semio_framework_artifact_infinite_dag::DagSnapshot {
    fn from(value: &DagSnapshot) -> Self {
        value.clone().into()
    }
}

/// 🧾️ Node/edge accessors matching the OLD field-access call-site shape (`document.nodes`), now
/// reading through the exact child owner. Kept as methods on `DagSnapshot` itself so call sites do
/// not need to import `dag_working_scene`.
impl DagSnapshot {
    pub fn nodes(&self) -> Vec<DagNodeSpec> {
        crate::dag_working_scene(self).nodes
    }
    pub fn edges(&self) -> Vec<DagHostSnapshotEdge> {
        crate::dag_working_scene(self).edges
    }
}
//#endregion 🔖️FrameworkBridge

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`DagSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🌳️mutate-dag-1`'s scenarios are measured through, and the shape the committed
/// `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a graph, which
/// is exactly what makes it a usable observability surface here: the handle's `childId` is a digest
/// of the child's content, so it moves if and only if the working scene moved.
///
/// A thin `dsl::json` wrapper (this facet's own first-party `DslValue` JSON codec, used behind
/// this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_dag_snapshot_json(snapshot: &DagSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_dag_snapshot_json`] — decodes those committed specification vectors
/// into real [`DagSnapshot`] values, so `🌳️mutate-dag-1`'s adapter reads the committed fixture rather
/// than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that adapter is
/// impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_dag_snapshot_json(text: &str) -> Result<DagSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `.dag.dsl.semio` text into a [`DagSnapshot`], attaching the working scene to the child
/// handle it mints — a named, non-async pass-through of this type's own `store::ArtifactDsl` impl,
/// whose trait and error type are both unnameable outside this crate. This is the only way an
/// external caller can obtain a dag document whose composed `s.stdio.semio.graph` child actually
/// resolves, which is what `🌳️mutate-dag-1` needs before any kind can have a visible effect.
pub fn parse_dag_dsl(text: &str) -> Result<DagSnapshot, String> {
    <DagSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`DagSnapshot`] back as `.dag.dsl.semio` text — the inverse of [`parse_dag_dsl`],
/// preamble included, which is what makes a printed document comparable to the committed one byte
/// for byte.
pub fn print_dag_dsl(snapshot: &DagSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The node ids and `source -> target` endpoints the document's composed child currently
/// resolves to, in scene order — the human-readable half of a divergence message, so a failing
/// `mutate-<kind>` or `inverse-<kind>` names WHICH node moved rather than only that two content
/// digests differ.
pub fn dag_scene_summary(snapshot: &DagSnapshot) -> String {
    let scene = crate::dag_working_scene(snapshot);
    let nodes = scene.nodes.iter().map(|node| format!("{}({},{})", node.id, node.x, node.y)).collect::<Vec<_>>().join(" ");
    let edges = scene.edges.iter().map(|edge| format!("{}:{}->{}", edge.id, edge.source, edge.target)).collect::<Vec<_>>().join(" ");
    format!("nodes[{nodes}] edges[{edges}]")
}
//#endregion 🌉️ExternalCodecBridge

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

```

