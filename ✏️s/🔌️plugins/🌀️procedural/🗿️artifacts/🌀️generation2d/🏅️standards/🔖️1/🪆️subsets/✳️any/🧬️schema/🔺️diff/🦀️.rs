//! 🧬️ Generation2d diff schema — sparse keyed and positional delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::FormGeneration;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Generation2dDiff
/// 🧬️ Sparse delta of the generation2d artifact: keyed widget, synapse, layout and generation rows plus owned scalar fields.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.generation2d")]
pub struct Generation2dDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub camera: Option<CameraJson>,
    #[state(artifact)]
    pub widgets: Option<Generation2dWidgetsDelta>,
    #[state(artifact)]
    pub synapses: Option<Generation2dSynapsesDelta>,
    #[state(artifact)]
    pub layout: Option<Generation2dLayoutDelta>,
    #[state(artifact)]
    pub generations: Option<Generation2dGenerationsDelta>,
    #[state(artifact)]
    pub selected_generation: Option<Generation2dSelectionChange>,
    #[state(artifact)]
    pub preview_text: Option<Generation2dPreviewChange>,
}
//#endregion 🔖️Generation2dDiff

//#region 🔖️DeltaHelpers

protocol::list_delta! {
    /// 🧩 Positional rows of the widget list: `removed: [{id, index}]` (base index), `inserted: [{index, row}]` (after index), `moved: [{id, from, to}]`, keyed `modified` entries `{id, patch}`.
    pub Generation2dWidgetsDelta { removal: Generation2dWidgetRemoval, insertion: Generation2dWidgetInsertion, relocation: Generation2dWidgetRelocation, modification: Generation2dWidgetModification, row: Widget, patch: Generation2dWidgetPatch, list: Vec<Widget>, key: String = by Generation2dWidgetKeys, values_only }
}

/// 🩹 How one widget changes: replaced wholesale, or only the numeric fields of an input slider.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Generation2dWidgetPatch {
    Replace { widget: Widget },
    Slider { value: f64, min: f64, max: f64, step: f64 },
}

protocol::list_delta! {
    /// 🧩 Positional rows of the synapse list: `removed: [{id, index}]` (base index), `inserted: [{index, row}]` (after index), `moved: [{id, from, to}]`, keyed `modified` entries `{id, patch}`.
    pub Generation2dSynapsesDelta { removal: Generation2dSynapseRemoval, insertion: Generation2dSynapseInsertion, relocation: Generation2dSynapseRelocation, modification: Generation2dSynapseModification, row: SynapseSpec, patch: Generation2dSynapsePatch, list: Vec<SynapseSpec>, key: String = by Generation2dSynapseKeys, values_only }
}

/// 📍️ One widget position row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dLayoutRow {
    pub id: String,
    pub layout: WidgetLayout,
}

/// 🧩 Id-keyed rows of the widget positions.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dLayoutDelta {
    pub added: Vec<Generation2dLayoutRow>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation2dLayoutRow>,
}

protocol::list_delta! {
    /// 🧩 Positional rows of the generation roster: `removed: [{id, index}]` (base index), `inserted: [{index, row}]` (after index), `moved: [{id, from, to}]`, keyed `modified` entries `{id, patch}`.
    pub Generation2dGenerationsDelta { removal: Generation2dGenerationRemoval, insertion: Generation2dGenerationInsertion, relocation: Generation2dGenerationRelocation, modification: Generation2dGenerationModification, row: FormGeneration, patch: Generation2dGenerationPatch, list: Vec<FormGeneration>, key: String = by Generation2dGenerationKeys, values_only }
}

/// 🩹 Owned-field patch of one generation: its name and keyed answer rows.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dGenerationPatch {
    pub name: Option<String>,
    pub values: Option<Generation2dValuesDelta>,
}

/// 🧾️ One answer row of a generation.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dValueRow {
    pub question_id: String,
    pub value: DslValue,
}

/// 🧩 Question-keyed rows of the answers of one generation.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dValuesDelta {
    pub added: Vec<Generation2dValueRow>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation2dValueRow>,
}

/// 👆️ A present change of the selected generation; the inner `None` clears the selection.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dSelectionChange {
    pub id: Option<String>,
}

/// 📝️ A present change of the generate preview text; the inner `None` clears it.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dPreviewChange {
    pub text: Option<String>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::any::schema::Generation2dArtifact;
//#endregion 🔁️Re-exports

use crate::widget_id;
use crate::Generation2dSnapshot;
use protocol::{DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;

//#region 🔖️RowAlgebra
/// 🧩 One keyed row: its id and how a displaced owned value is closed instead of dropped.
pub(crate) trait Row: Clone {
    fn id(&self) -> &str;
    fn retire(self) {}
}

impl Row for Generation2dLayoutRow {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Row for Generation2dValueRow {
    fn id(&self) -> &str {
        &self.question_id
    }
    fn retire(self) {
        <DslValue as semio_framework_value::FromValue>::retire_decoded(self.value);
    }
}

/// 🩹 How one patch row edits one item: applied, inverted against a base item, composed with a later row, rebuilt between two items, and closed when displaced.
pub(crate) trait CellPatch<T>: Clone + Sized {
    fn applied(&self, item: &T) -> MutationApplyResult<T>;
    fn inverse_against(&self, base: &T) -> Self;
    fn composed(self, later: Self) -> Self;
    fn retire(self);
}

impl<T: Row + PartialEq> CellPatch<T> for T {
    fn applied(&self, _item: &T) -> MutationApplyResult<T> {
        Ok(self.clone())
    }
    fn inverse_against(&self, base: &T) -> Self {
        base.clone()
    }
    fn composed(self, later: Self) -> Self {
        Row::retire(self);
        later
    }
    fn retire(self) {
        Row::retire(self);
    }
}

/// 🔑️ The key extractor and cold disposal of the widget rows: a widget owns fail-closed roots, so it is closed, never dropped.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Generation2dWidgetKeys;

impl protocol::list_delta::KeyOf<Widget> for Generation2dWidgetKeys {
    type Key = String;
    fn key_of(row: &Widget) -> String {
        widget_id(row).to_string()
    }
    fn retire_cold(row: Widget) {
        row.retire_cold();
    }
}

/// 🔑️ The key extractor of the synapse rows (plain data: a drop is a close).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Generation2dSynapseKeys;

impl protocol::list_delta::KeyOf<SynapseSpec> for Generation2dSynapseKeys {
    type Key = String;
    fn key_of(row: &SynapseSpec) -> String {
        row.id.clone()
    }
}

/// 🔑️ The key extractor and cold disposal of the generation rows: their answers own fail-closed roots.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Generation2dGenerationKeys;

impl protocol::list_delta::KeyOf<FormGeneration> for Generation2dGenerationKeys {
    type Key = String;
    fn key_of(row: &FormGeneration) -> String {
        row.id.clone()
    }
    fn retire_cold(row: FormGeneration) {
        semio_framework_value::FromValue::retire_decoded(row);
    }
}

/// 🩹 A synapse is patched by replacing it wholesale; the wire form is the synapse itself.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(transparent)]
pub struct Generation2dSynapsePatch(pub SynapseSpec);

impl protocol::list_delta::RowPatch<SynapseSpec> for Generation2dSynapsePatch {
    fn commit_into(&self, row: &mut SynapseSpec, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        row.clone_from(&self.0);
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        *self = later;
    }
    fn inverse(&self, row: &SynapseSpec) -> Self {
        Self(row.clone())
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl Generation2dWidgetPatch {
    fn applied(&self, item: &Widget) -> MutationApplyResult<Widget> {
        match self {
            Self::Replace { widget } => Ok(widget.clone()),
            Self::Slider { value, min, max, step } => match item {
                Widget::InputSlider { id, label, .. } => Ok(Widget::InputSlider { id: id.clone(), label: label.clone(), value: *value, min: *min, max: *max, step: *step }),
                _ => Err(MutationApplyError::new("mutation.apply.mismatched-target", "slider patch needs an input slider")),
            },
        }
    }

    fn composed(self, later: Self) -> Self {
        match (self, later) {
            (Self::Replace { widget }, Self::Slider { value, min, max, step }) => match &widget {
                Widget::InputSlider { id, label, .. } => {
                    let merged = Widget::InputSlider { id: id.clone(), label: label.clone(), value, min, max, step };
                    widget.retire_cold();
                    Self::Replace { widget: merged }
                }
                _ => Self::Replace { widget },
            },
            (earlier, later) => {
                protocol::list_delta::RowPatch::<Widget>::retire_cold(earlier);
                later
            }
        }
    }
}

impl protocol::list_delta::RowPatch<Widget> for Generation2dWidgetPatch {
    fn commit_into(&self, row: &mut Widget, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        let next = self.applied(row)?;
        std::mem::replace(row, next).retire_cold();
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let earlier = std::mem::replace(self, Self::Slider { value: 0.0, min: 0.0, max: 0.0, step: 0.0 });
        *self = earlier.composed(later);
    }
    fn inverse(&self, row: &Widget) -> Self {
        Self::Replace { widget: row.clone() }
    }
    fn is_empty(&self) -> bool {
        false
    }
    fn retire_cold(self) {
        if let Self::Replace { widget } = self {
            widget.retire_cold();
        }
    }
}

impl protocol::list_delta::RowPatch<FormGeneration> for Generation2dGenerationPatch {
    fn commit_into(&self, row: &mut FormGeneration, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        let mut next = row.clone();
        if let Some(name) = &self.name {
            next.name.clone_from(name);
        }
        if let Some(delta) = &self.values {
            if let Err(error) = apply_values(&mut next, delta) {
                semio_framework_value::FromValue::retire_decoded(next);
                return Err(error.under(["values"]));
            }
        }
        semio_framework_value::FromValue::retire_decoded(std::mem::replace(row, next));
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.values = match (self.values.take(), later.values) {
            (Some(first), Some(second)) => Some(absorb_delta(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        self.name = later.name.or(self.name.take());
    }
    fn inverse(&self, row: &FormGeneration) -> Self {
        Self { name: self.name.as_ref().map(|_| row.name.clone()), values: self.values.as_ref().map(|delta| inverse_values(delta, row)) }
    }
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.values.as_ref().is_none_or(is_empty_delta)
    }
    fn retire_cold(self) {
        if let Some(values) = self.values {
            values.retire_cold();
        }
    }
}

/// 🧩 The shared shape of every unordered keyed collection delta (rows of a map: they carry no position).
pub(crate) trait Delta: Default + Clone {
    type Item: Row;
    type Patch: CellPatch<Self::Item>;
    fn added(&self) -> &[Self::Item];
    fn removed(&self) -> &[String];
    fn patched(&self) -> Vec<(&str, &Self::Patch)>;
    fn into_parts(self) -> (Vec<Self::Item>, Vec<String>, Vec<(String, Self::Patch)>);
    fn from_parts(added: Vec<Self::Item>, removed: Vec<String>, patched: Vec<(String, Self::Patch)>) -> Self;
}

macro_rules! impl_unordered_delta {
    ($delta:ty, $item:ty) => {
        impl Delta for $delta {
            type Item = $item;
            type Patch = $item;
            fn added(&self) -> &[$item] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> Vec<(&str, &$item)> {
                self.patched.iter().map(|row| (Row::id(row), row)).collect()
            }
            fn into_parts(self) -> (Vec<$item>, Vec<String>, Vec<(String, $item)>) {
                (self.added, self.removed, self.patched.into_iter().map(|row| (Row::id(&row).to_string(), row)).collect())
            }
            fn from_parts(added: Vec<$item>, removed: Vec<String>, patched: Vec<(String, $item)>) -> Self {
                Self { added, removed, patched: patched.into_iter().map(|(_, row)| row).collect() }
            }
        }
    };
}

impl_unordered_delta!(Generation2dLayoutDelta, Generation2dLayoutRow);
impl_unordered_delta!(Generation2dValuesDelta, Generation2dValueRow);

impl Generation2dValuesDelta {
    fn retire_cold(self) {
        let (added, _, modified) = Delta::into_parts(self);
        added.into_iter().chain(modified.into_iter().map(|(_, row)| row)).for_each(Row::retire);
    }
}

//#endregion 🔖️RowAlgebra

//#region 🔖️DeltaAlgebra
fn rejection(code: &str, message: &str, at: [String; 2]) -> MutationApplyError {
    MutationApplyError::new(code, message).at(at)
}

fn is_empty_delta<D: Delta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.modified().is_empty()
}

enum Net<T, P> {
    Patch(P),
    Remove,
    Add(T),
    Replace(T),
}

impl<T: Row, P: CellPatch<T>> Net<T, P> {
    fn retire(self) {
        match self {
            Self::Patch(patch) => patch.retire(),
            Self::Remove => {}
            Self::Add(item) | Self::Replace(item) => item.retire(),
        }
    }
}

/// ➕️ Composes the unordered `first` then `second` per id (patch∘patch → one patch, add∘remove → nothing, remove∘add → replace) in a canonical row order; every displaced owned value is closed.
fn absorb_delta<D: Delta>(first: D, second: D) -> D {
    let mut nets: std::collections::BTreeMap<String, Net<D::Item, D::Patch>> = std::collections::BTreeMap::new();
    let mut appended: Vec<String> = Vec::new();
    for delta in [first, second] {
        let (added, removed, patched) = delta.into_parts();
        for id in removed {
            match nets.remove(&id) {
                Some(Net::Add(item)) => {
                    item.retire();
                    appended.retain(|existing| existing != &id);
                }
                Some(Net::Replace(item)) => {
                    item.retire();
                    appended.retain(|existing| existing != &id);
                    nets.insert(id, Net::Remove);
                }
                Some(other) => {
                    other.retire();
                    nets.insert(id, Net::Remove);
                }
                None => {
                    nets.insert(id, Net::Remove);
                }
            }
        }
        for item in added {
            let id = item.id().to_string();
            let net = match nets.remove(&id) {
                Some(Net::Remove) => Net::Replace(item),
                Some(other) => {
                    other.retire();
                    Net::Add(item)
                }
                None => Net::Add(item),
            };
            appended.retain(|existing| existing != &id);
            appended.push(id.clone());
            nets.insert(id, net);
        }
        for (id, patch) in patched {
            let net = match nets.remove(&id) {
                None => Net::Patch(patch),
                Some(Net::Remove) => {
                    patch.retire();
                    Net::Remove
                }
                Some(Net::Patch(earlier)) => Net::Patch(earlier.composed(patch)),
                Some(Net::Add(item)) => match patch.applied(&item) {
                    Ok(patched) => {
                        item.retire();
                        patch.retire();
                        Net::Add(patched)
                    }
                    Err(_) => {
                        patch.retire();
                        Net::Add(item)
                    }
                },
                Some(Net::Replace(item)) => match patch.applied(&item) {
                    Ok(patched) => {
                        item.retire();
                        patch.retire();
                        Net::Replace(patched)
                    }
                    Err(_) => {
                        patch.retire();
                        Net::Replace(item)
                    }
                },
            };
            nets.insert(id, net);
        }
    }
    let mut removed = Vec::new();
    let mut patched = Vec::new();
    let mut adds: std::collections::BTreeMap<String, D::Item> = std::collections::BTreeMap::new();
    for (id, net) in nets {
        match net {
            Net::Patch(patch) => patched.push((id, patch)),
            Net::Remove => removed.push(id),
            Net::Add(item) => {
                adds.insert(id, item);
            }
            Net::Replace(item) => {
                removed.push(id.clone());
                adds.insert(id, item);
            }
        }
    }
    let mut added: Vec<D::Item> = Vec::with_capacity(adds.len());
    for id in &appended {
        if let Some(item) = adds.remove(id) {
            added.push(item);
        }
    }
    D::from_parts(added, removed, patched)
}

fn absorb_optional<D: Delta>(first: &mut Option<D>, second: Option<D>) {
    let Some(second) = second else { return };
    let merged = absorb_delta(first.take().unwrap_or_default(), second);
    *first = (!is_empty_delta(&merged)).then_some(merged);
}

fn apply_values(generation: &mut FormGeneration, delta: &Generation2dValuesDelta) -> MutationApplyResult<()> {
    for (index, id) in delta.removed.iter().enumerate() {
        if !generation.values.contains_key(id) {
            return Err(rejection("mutation.apply.missing-target", "removed answer does not exist", ["removed".into(), index.to_string()]));
        }
    }
    for (index, row) in delta.patched.iter().enumerate() {
        if !generation.values.contains_key(&row.question_id) {
            return Err(rejection("mutation.apply.missing-target", "patched answer does not exist", ["patched".into(), index.to_string()]));
        }
    }
    for (index, row) in delta.added.iter().enumerate() {
        if generation.values.contains_key(&row.question_id) && !delta.removed.contains(&row.question_id) {
            return Err(rejection("mutation.apply.duplicate-target", "added answer already exists", ["added".into(), index.to_string()]));
        }
    }
    for id in &delta.removed {
        retire_displaced(generation.values.remove(id));
    }
    for row in delta.added.iter().chain(&delta.patched) {
        retire_displaced(generation.values.insert(row.question_id.clone(), row.value.clone()));
    }
    Ok(())
}

fn retire_displaced(value: Option<std::sync::Arc<DslValue>>) {
    if let Some(value) = value.and_then(std::sync::Arc::into_inner) {
        <DslValue as semio_framework_value::FromValue>::retire_decoded(value);
    }
}

fn inverse_values(delta: &Generation2dValuesDelta, base: &FormGeneration) -> Generation2dValuesDelta {
    let held = |id: &str| base.values.get(id).map(|value| Generation2dValueRow { question_id: id.to_string(), value: value.clone() });
    Generation2dValuesDelta {
        added: delta.removed.iter().filter_map(|id| held(id)).collect(),
        removed: delta.added.iter().map(|row| row.question_id.clone()).collect(),
        patched: delta.patched.iter().filter_map(|row| held(&row.question_id)).collect(),
    }
}

//#endregion 🔖️DeltaAlgebra

//#region 🔖️Retirement
impl Generation2dDiff {
    /// 🧊️ Explicit cold-only disposal of a detached sparse delta. Every owned `Widget`, generation and answer row carries a fail-closed
    /// root that rejects a bare drop, so an owned diff is CLOSED, never dropped.
    pub fn retire_cold(self) {
        if let Some(widgets) = self.widgets {
            widgets.retire_cold();
        }
        if let Some(generations) = self.generations {
            generations.retire_cold();
        }
    }
}

/// 🔺️ A sparse-delta read that CLOSES itself — the ONE shape a test holds an owned
/// [`Generation2dDiff`] in, the diff twin of `Generation2dSnapshotRead`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
pub struct Generation2dDiffRead(Option<Generation2dDiff>);

#[cfg(test)]
impl Generation2dDiffRead {
    pub fn new(diff: Generation2dDiff) -> Self {
        Self(Some(diff))
    }
}

#[cfg(test)]
impl std::ops::Deref for Generation2dDiffRead {
    type Target = Generation2dDiff;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("a delta read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl Drop for Generation2dDiffRead {
    fn drop(&mut self) {
        if let Some(diff) = self.0.take() {
            diff.retire_cold();
        }
    }
}

#[cfg(test)]
impl std::fmt::Debug for Generation2dDiffRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, formatter)
    }
}
//#endregion 🔖️Retirement

//#region 🔖️Apply
/// ➕️ Composes an optional positional delta field with the later one; a delta that cancels out is closed, never dropped.
macro_rules! absorb_rows {
    ($field:expr, $later:expr) => {
        if let Some(later) = $later {
            let mut merged = $field.take().unwrap_or_default();
            merged.absorb(later);
            if merged.is_empty() {
                merged.retire_cold();
            } else {
                $field = Some(merged);
            }
        }
    };
}

impl Generation2dDiff {
    fn apply_layout(&self, snapshot: &mut Generation2dSnapshot) -> MutationApplyResult<()> {
        let Some(delta) = &self.layout else { return Ok(()) };
        let layout = &mut snapshot.host_snapshot.layout;
        for (index, id) in delta.removed.iter().enumerate() {
            if layout.remove(id).is_none() {
                return Err(rejection("mutation.apply.missing-target", "removed layout entry does not exist", ["layout".into(), index.to_string()]));
            }
        }
        for (index, row) in delta.added.iter().enumerate() {
            if layout.contains_key(&row.id) {
                return Err(rejection("mutation.apply.duplicate-target", "added layout entry already exists", ["layout".into(), index.to_string()]));
            }
            layout.insert(row.id.clone(), row.layout.clone());
        }
        for (index, row) in delta.patched.iter().enumerate() {
            if !layout.contains_key(&row.id) {
                return Err(rejection("mutation.apply.missing-target", "patched layout entry does not exist", ["layout".into(), index.to_string()]));
            }
            layout.insert(row.id.clone(), row.layout.clone());
        }
        Ok(())
    }

    fn apply_generation(&self, snapshot: &mut Generation2dSnapshot, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        if self.generations.is_none() && self.selected_generation.is_none() && self.preview_text.is_none() {
            return Ok(());
        }
        let mut state = (*snapshot.generation).clone();
        if let Some(delta) = &self.generations {
            let committed = delta.commit_onto(&state.generations, capability).map_err(|error| error.under(["generations"]))?;
            Generation2dGenerationsDelta::retire_list(std::mem::replace(&mut state.generations, committed));
        }
        if let Some(change) = &self.selected_generation {
            state.selected_generation_id.clone_from(&change.id);
        }
        if let Some(change) = &self.preview_text {
            state.preview_text.clone_from(&change.text);
        }
        std::mem::replace(&mut snapshot.generation, GenerationPlayRoot::from(state)).retire_cold();
        Ok(())
    }
}

impl MutationDiff<Generation2dSnapshot> for Generation2dDiff {
    fn apply(&self, snapshot: &Generation2dSnapshot, capability: protocol::ApplyCapability) -> MutationApplyResult<Generation2dSnapshot> {
        let mut next = snapshot.clone();
        let applied = (|| {
            if let Some(schema) = &self.schema {
                next.host_snapshot.schema.clone_from(schema);
            }
            if let Some(camera) = &self.camera {
                next.host_snapshot.camera = camera.clone();
            }
            if let Some(delta) = &self.widgets {
                let committed = delta.commit_onto(&next.host_snapshot.widgets, capability).map_err(|error| error.under(["widgets"]))?;
                Generation2dWidgetsDelta::retire_list(std::mem::replace(&mut next.host_snapshot.widgets, committed));
            }
            if let Some(delta) = &self.synapses {
                let committed = delta.commit_onto(&next.host_snapshot.synapses, capability).map_err(|error| error.under(["synapses"]))?;
                Generation2dSynapsesDelta::retire_list(std::mem::replace(&mut next.host_snapshot.synapses, committed));
            }
            self.apply_layout(&mut next)?;
            self.apply_generation(&mut next, capability)
        })();
        match applied {
            Ok(()) => Ok(next),
            Err(error) => {
                next.retire_cold();
                Err(error)
            }
        }
    }

    /// ➕️ Sequential coalesce: same-key rows fold (patch∘patch → one patch, add∘remove → nothing, remove∘add → replace). Every
    /// value this displaces is RETIRED, never dropped: owned widgets and answers reject a bare drop.
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        absorb_rows!(self.widgets, other.widgets);
        absorb_rows!(self.synapses, other.synapses);
        absorb_optional(&mut self.layout, other.layout);
        absorb_rows!(self.generations, other.generations);
        if other.selected_generation.is_some() {
            self.selected_generation = other.selected_generation;
        }
        if other.preview_text.is_some() {
            self.preview_text = other.preview_text;
        }
    }

    /// 🧊️ The generic replay seams (`os_vcs::apply_mutation`, the store's history folds) build a
    /// delta and throw it away; an inhabited widget row owns a fail-closed root, so the contract routes here.
    fn retire_cold(self) {
        Generation2dDiff::retire_cold(self);
    }

    /// 🧊️ Same law for the scratch projections a history fold displaces between steps.
    fn retire_projection(projection: Generation2dSnapshot) {
        projection.retire_cold();
    }
}

impl DiffAlgebra<Generation2dSnapshot> for Generation2dDiff {
    fn inverse(&self, base: &Generation2dSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.host_snapshot.schema.clone()),
            camera: self.camera.as_ref().map(|_| base.host_snapshot.camera.clone()),
            widgets: self.widgets.as_ref().map(|delta| delta.inverse(&base.host_snapshot.widgets)),
            synapses: self.synapses.as_ref().map(|delta| delta.inverse(&base.host_snapshot.synapses)),
            layout: self.layout.as_ref().map(|delta| inverse_layout(delta, base)),
            generations: self.generations.as_ref().map(|delta| delta.inverse(&base.generation.generations)),
            selected_generation: self.selected_generation.as_ref().map(|_| Generation2dSelectionChange { id: base.generation.selected_generation_id.clone() }),
            preview_text: self.preview_text.as_ref().map(|_| Generation2dPreviewChange { text: base.generation.preview_text.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.camera.is_none()
            && self.widgets.as_ref().is_none_or(|delta| delta.is_empty())
            && self.synapses.as_ref().is_none_or(|delta| delta.is_empty())
            && self.layout.as_ref().is_none_or(is_empty_delta)
            && self.generations.as_ref().is_none_or(|delta| delta.is_empty())
            && self.selected_generation.is_none()
            && self.preview_text.is_none()
    }
}

fn inverse_layout(delta: &Generation2dLayoutDelta, base: &Generation2dSnapshot) -> Generation2dLayoutDelta {
    let held = |id: &str| base.host_snapshot.layout.get(id).map(|layout| Generation2dLayoutRow { id: id.to_string(), layout: layout.clone() });
    Generation2dLayoutDelta {
        added: delta.removed.iter().filter_map(|id| held(id)).collect(),
        removed: delta.added.iter().map(|row| row.id.clone()).collect(),
        patched: delta.patched.iter().filter_map(|row| held(&row.id)).collect(),
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
