//! 🧬️ Forms diff schema — sparse keyed edits over the artifact.

use crate::schema::mutations::change_block_field::mutation::BlockField;
use crate::schema::response::FormsResponse;
use crate::{forms_children_from_steps, FormQuestion, FormStep, FormsResultsChild, FormsSnapshot, FormsStructureChild};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️Diff
/// 🔺️ Sparse durable domain edits and their derived child projections: scalar slots, id-keyed step and response row deltas
/// (steps carry nested id-keyed question deltas) and the regenerated owned-child handles.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
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
    pub title: Option<FormsOptionalText>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub steps: Option<FormsStepsDelta>,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub responses: Option<FormsResponsesDelta>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub structure: Option<FormsStructureChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub results: Option<FormsResultsChild>,
}

impl semio_framework_value::FromValue for FormsDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut result = Self::default();
        let mut seen = 0u8;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            let bit = match key.as_str() {
                "schema" => 1,
                "id" => 2,
                "version" => 4,
                "title" => 8,
                "steps" => 16,
                "responses" => 32,
                "structure" => 64,
                "results" => 128,
                _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Forms diff field {key}"))),
            };
            if seen & bit != 0 {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("duplicate Forms diff field {key}")));
            }
            seen |= bit;
            match key.as_str() {
                "schema" => result.schema = Some(semio_framework_value::FromValue::from_value(value)?),
                "id" => result.id = Some(semio_framework_value::FromValue::from_value(value)?),
                "version" => result.version = Some(semio_framework_value::FromValue::from_value(value)?),
                "title" => result.title = Some(semio_framework_value::FromValue::from_value(value)?),
                "steps" => result.steps = Some(semio_framework_value::FromValue::from_value(value)?),
                "responses" => result.responses = Some(semio_framework_value::FromValue::from_value(value)?),
                "structure" => result.structure = Some(semio_framework_value::FromValue::from_value(value)?),
                _ => result.results = Some(semio_framework_value::FromValue::from_value(value)?),
            }
        }
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
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
/// 🧱️ Carries an optional text as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsOptionalText {
    pub value: Option<String>,
}

/// 🩹 Field patch of one question: the kind when it changes plus the typed field settings, at most one per field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsQuestionPatch {
    pub id: String,
    pub kind: Option<String>,
    pub changes: Vec<BlockField>,
}

/// 🩹 Field patch of one step: title, description and the nested id-keyed question delta.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsStepPatch {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<FormsOptionalText>,
    pub blocks: Option<FormsQuestionsDelta>,
}

/// 🩹 The (always empty) patch of an immutable response.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsResponsePatch {
    pub id: String,
}

/// 🧩 Id-keyed row delta of the questions of one step.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsQuestionsDelta {
    pub added: Vec<FormQuestion>,
    pub removed: Vec<String>,
    pub patched: Vec<FormsQuestionPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Id-keyed row delta of the steps.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsStepsDelta {
    pub added: Vec<FormStep>,
    pub removed: Vec<String>,
    pub patched: Vec<FormsStepPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Id-keyed row delta of the immutable responses.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct FormsResponsesDelta {
    pub added: Vec<FormsResponse>,
    pub removed: Vec<String>,
    pub patched: Vec<FormsResponsePatch>,
    pub reordered: Option<Vec<String>>,
}

//#endregion 🔖️DeltaHelpers

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

impl KeyedDelta for FormsQuestionsDelta {
    type Row = FormQuestion;
    type Patch = FormsQuestionPatch;
    fn added(&self) -> &[FormQuestion] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[FormsQuestionPatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<FormQuestion>, removed: Vec<String>, patched: Vec<FormsQuestionPatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &FormQuestion) -> &str {
        &row.id
    }
    fn patch_key(patch: &FormsQuestionPatch) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &FormsQuestionPatch, row: &mut FormQuestion) -> Result<(), protocol::MutationApplyError> {
        if let Some(kind) = &patch.kind {
            row.kind = kind.clone();
        }
        for change in &patch.changes {
            *row = change.applied(row);
        }
        Ok(())
    }
    fn patch_compose(first: &FormsQuestionPatch, later: &FormsQuestionPatch) -> FormsQuestionPatch {
        let mut changes = first.changes.clone();
        for change in &later.changes {
            match changes.iter_mut().find(|existing| std::mem::discriminant(*existing) == std::mem::discriminant(change)) {
                Some(slot) => *slot = change.clone(),
                None => changes.push(change.clone()),
            }
        }
        FormsQuestionPatch { id: first.id.clone(), kind: later.kind.clone().or_else(|| first.kind.clone()), changes }
    }
    fn patch_inverse(patch: &FormsQuestionPatch, base: &FormQuestion) -> FormsQuestionPatch {
        FormsQuestionPatch { id: patch.id.clone(), kind: patch.kind.as_ref().map(|_| base.kind.clone()), changes: patch.changes.iter().map(|change| change.read(base)).collect() }
    }
    fn patch_between(base: &FormQuestion, other: &FormQuestion) -> Option<FormsQuestionPatch> {
        let patch = FormsQuestionPatch { id: other.id.clone(), kind: (base.kind != other.kind).then(|| other.kind.clone()), changes: BlockField::changes(base, other) };
        (!Self::patch_is_empty(&patch)).then_some(patch)
    }
    fn patch_is_empty(patch: &FormsQuestionPatch) -> bool {
        patch.kind.is_none() && patch.changes.is_empty()
    }
}

impl KeyedDelta for FormsStepsDelta {
    type Row = FormStep;
    type Patch = FormsStepPatch;
    fn added(&self) -> &[FormStep] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[FormsStepPatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<FormStep>, removed: Vec<String>, patched: Vec<FormsStepPatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &FormStep) -> &str {
        &row.id
    }
    fn patch_key(patch: &FormsStepPatch) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &FormsStepPatch, row: &mut FormStep) -> Result<(), protocol::MutationApplyError> {
        if let Some(title) = &patch.title {
            row.title = title.clone();
        }
        if let Some(description) = &patch.description {
            row.description = description.value.clone();
        }
        if let Some(delta) = &patch.blocks {
            row.blocks = keyed_apply(&row.blocks, delta).map_err(|error| error.under(["blocks"]))?;
        }
        Ok(())
    }
    fn patch_compose(first: &FormsStepPatch, later: &FormsStepPatch) -> FormsStepPatch {
        FormsStepPatch {
            id: first.id.clone(),
            title: later.title.clone().or_else(|| first.title.clone()),
            description: later.description.clone().or_else(|| first.description.clone()),
            blocks: match (&first.blocks, &later.blocks) {
                (Some(first), Some(later)) => Some(keyed_absorb(first, later)),
                (first, later) => later.clone().or_else(|| first.clone()),
            },
        }
    }
    fn patch_inverse(patch: &FormsStepPatch, base: &FormStep) -> FormsStepPatch {
        FormsStepPatch {
            id: patch.id.clone(),
            title: patch.title.as_ref().map(|_| base.title.clone()),
            description: patch.description.as_ref().map(|_| FormsOptionalText { value: base.description.clone() }),
            blocks: patch.blocks.as_ref().map(|delta| keyed_inverse(delta, &base.blocks)),
        }
    }
    fn patch_between(base: &FormStep, other: &FormStep) -> Option<FormsStepPatch> {
        let blocks = keyed_between::<FormsQuestionsDelta>(&base.blocks, &other.blocks);
        let patch = FormsStepPatch {
            id: other.id.clone(),
            title: (base.title != other.title).then(|| other.title.clone()),
            description: (base.description != other.description).then(|| FormsOptionalText { value: other.description.clone() }),
            blocks: (!keyed_is_empty(&blocks)).then_some(blocks),
        };
        (!Self::patch_is_empty(&patch)).then_some(patch)
    }
    fn patch_is_empty(patch: &FormsStepPatch) -> bool {
        patch.title.is_none() && patch.description.is_none() && patch.blocks.as_ref().is_none_or(keyed_is_empty)
    }
}

impl KeyedDelta for FormsResponsesDelta {
    type Row = FormsResponse;
    type Patch = FormsResponsePatch;
    fn added(&self) -> &[FormsResponse] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[FormsResponsePatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<FormsResponse>, removed: Vec<String>, patched: Vec<FormsResponsePatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &FormsResponse) -> &str {
        &row.id
    }
    fn patch_key(patch: &FormsResponsePatch) -> &str {
        &patch.id
    }
    fn patch_fold(_patch: &FormsResponsePatch, _row: &mut FormsResponse) -> Result<(), protocol::MutationApplyError> {
        Ok(())
    }
    fn patch_compose(first: &FormsResponsePatch, _later: &FormsResponsePatch) -> FormsResponsePatch {
        first.clone()
    }
    fn patch_inverse(patch: &FormsResponsePatch, _base: &FormsResponse) -> FormsResponsePatch {
        patch.clone()
    }
    fn patch_between(_base: &FormsResponse, _other: &FormsResponse) -> Option<FormsResponsePatch> {
        None
    }
    fn patch_is_empty(_patch: &FormsResponsePatch) -> bool {
        true
    }
}

//#region 🔖️Apply
/// 🏗️ Builds a [`FormsDiff`] carrying `delta` and the regenerated `structure` handle, derived from the steps `delta` leaves
/// behind `base` — the standard way every step-editing mutation leaf produces its result.
pub fn forms_diff_from_delta(delta: &FormsStepsDelta, base: &FormsSnapshot) -> FormsDiff {
    let next_steps = keyed_apply(&base.definition.steps, delta).unwrap_or_else(|_| base.definition.steps.clone());
    let (structure, _) = forms_children_from_steps(&next_steps);
    FormsDiff { steps: Some(delta.clone()), structure: Some(structure), ..Default::default() }
}

/// 🏗️ Builds a [`FormsDiff`] carrying `delta` and the regenerated `results` handle, derived from the responses `delta` leaves behind `base`.
pub fn forms_diff_from_responses_delta(delta: &FormsResponsesDelta, base: &FormsSnapshot) -> FormsDiff {
    let next_responses = keyed_apply(&base.responses, delta).unwrap_or_else(|_| base.responses.clone());
    FormsDiff { responses: Some(delta.clone()), results: Some(crate::forms_results_child(&next_responses)), ..Default::default() }
}

impl MutationDiff<FormsSnapshot> for FormsDiff {
    fn apply(&self, snapshot: &FormsSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<FormsSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.child-identity".into(), message, target: Vec::new() })?;
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
            next.title = title.value.clone();
        }
        if let Some(delta) = &self.steps {
            next.definition.steps = keyed_apply(&next.definition.steps, delta).map_err(|error| error.under(["steps"]))?;
        }
        if let Some(delta) = &self.responses {
            next.responses = keyed_apply(&next.responses, delta).map_err(|error| error.under(["responses"]))?;
        }
        if let Some(structure) = &self.structure {
            next.structure = structure.clone();
        }
        if let Some(results) = &self.results {
            next.results = results.clone();
        }
        next.validate().map_err(|message| protocol::MutationApplyError { code: "mutation.apply.invariant".into(), message, target: Vec::new() })?;
        Ok(next)
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
        self.steps = match (self.steps.take(), other.steps) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
        self.responses = match (self.responses.take(), other.responses) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<FormsSnapshot> for FormsDiff {
    fn inverse(&self, base: &FormsSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            version: self.version.as_ref().map(|_| base.version.clone()),
            title: self.title.as_ref().map(|_| FormsOptionalText { value: base.title.clone() }),
            steps: self.steps.as_ref().map(|delta| keyed_inverse(delta, &base.definition.steps)),
            responses: self.responses.as_ref().map(|delta| keyed_inverse(delta, &base.responses)),
            structure: self.structure.as_ref().map(|_| base.structure.clone()),
            results: self.results.as_ref().map(|_| base.results.clone()),
        }
    }

    fn between(base: &FormsSnapshot, other: &FormsSnapshot) -> Self {
        let steps = keyed_between::<FormsStepsDelta>(&base.definition.steps, &other.definition.steps);
        let responses = keyed_between::<FormsResponsesDelta>(&base.responses, &other.responses);
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            version: (base.version != other.version).then(|| other.version.clone()),
            title: (base.title != other.title).then(|| FormsOptionalText { value: other.title.clone() }),
            steps: (!keyed_is_empty(&steps)).then_some(steps),
            responses: (!keyed_is_empty(&responses)).then_some(responses),
            structure: (base.structure != other.structure).then(|| other.structure.clone()),
            results: (base.results != other.results).then(|| other.results.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.version.is_none() && self.title.is_none() && self.steps.as_ref().is_none_or(keyed_is_empty) && self.responses.as_ref().is_none_or(keyed_is_empty) && self.structure.is_none() && self.results.is_none()
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
