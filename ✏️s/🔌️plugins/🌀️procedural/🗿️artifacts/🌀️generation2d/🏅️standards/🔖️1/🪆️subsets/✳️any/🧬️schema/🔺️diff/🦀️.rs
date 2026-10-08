//! 🧬️ Generation2d diff schema — sparse keyed delta over the artifact.

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
/// 🧩 Id-keyed rows of the widget list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dWidgetsDelta {
    pub added: Vec<Widget>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation2dWidgetPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched widget row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dWidgetPatchEntry {
    pub id: String,
    pub patch: Generation2dWidgetPatch,
}

/// 🩹 How one widget changes: replaced wholesale, or only the numeric fields of an input slider.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Generation2dWidgetPatch {
    Replace { widget: Widget },
    Slider { value: f64, min: f64, max: f64, step: f64 },
}

/// 🧩 Id-keyed rows of the synapse list.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dSynapsesDelta {
    pub added: Vec<SynapseSpec>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation2dSynapsePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched synapse row (whole-synapse replacement).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dSynapsePatchEntry {
    pub id: String,
    pub item: SynapseSpec,
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

/// 🧩 Id-keyed rows of the generation roster.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation2dGenerationsDelta {
    pub added: Vec<FormGeneration>,
    pub removed: Vec<String>,
    pub patched: Vec<Generation2dGenerationPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched generation row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Generation2dGenerationPatchEntry {
    pub id: String,
    pub patch: Generation2dGenerationPatch,
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

//#region 🔖️Insertion
/// 📍 Complete id order that places `id` at `index` among `ids`; `None` when it lands last, because appending is already the natural order of an added row.
pub fn insertion_order<'a>(ids: impl IntoIterator<Item = &'a str>, id: &str, index: Option<usize>) -> Option<Vec<String>> {
    let at = index?;
    let mut order: Vec<String> = ids.into_iter().map(str::to_owned).collect();
    (at < order.len()).then(|| {
        order.insert(at, id.to_owned());
        order
    })
}
//#endregion 🔖️Insertion

//#region 🔖️RowAlgebra
/// 🧩 One keyed row: its id and how a displaced owned value is closed instead of dropped.
pub(crate) trait Row: Clone {
    fn id(&self) -> &str;
    fn retire(self) {}
}

impl Row for Widget {
    fn id(&self) -> &str {
        widget_id(self)
    }
    fn retire(self) {
        self.retire_cold();
    }
}

impl Row for SynapseSpec {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Row for Generation2dLayoutRow {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Row for FormGeneration {
    fn id(&self) -> &str {
        &self.id
    }
    fn retire(self) {
        semio_framework_value::FromValue::retire_decoded(self);
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
pub(crate) trait RowPatch<T>: Clone + Sized {
    fn applied(&self, item: &T) -> MutationApplyResult<T>;
    fn inverse_against(&self, base: &T) -> Self;
    fn composed(self, later: Self) -> Self;
    fn between(base: &T, other: &T) -> Option<Self>;
    fn retire(self);
}

impl<T: Row + PartialEq> RowPatch<T> for T {
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
    fn between(base: &T, other: &T) -> Option<Self> {
        (base != other).then(|| other.clone())
    }
    fn retire(self) {
        Row::retire(self);
    }
}

impl RowPatch<Widget> for Generation2dWidgetPatch {
    fn applied(&self, item: &Widget) -> MutationApplyResult<Widget> {
        match self {
            Self::Replace { widget } => Ok(widget.clone()),
            Self::Slider { value, min, max, step } => match item {
                Widget::InputSlider { id, label, .. } => Ok(Widget::InputSlider { id: id.clone(), label: label.clone(), value: *value, min: *min, max: *max, step: *step }),
                _ => Err(MutationApplyError::new("mutation.apply.mismatched-target", "slider patch needs an input slider")),
            },
        }
    }
    fn inverse_against(&self, base: &Widget) -> Self {
        Self::Replace { widget: base.clone() }
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
                RowPatch::<Widget>::retire(earlier);
                later
            }
        }
    }
    fn between(base: &Widget, other: &Widget) -> Option<Self> {
        (base != other).then(|| Self::Replace { widget: other.clone() })
    }
    fn retire(self) {
        if let Self::Replace { widget } = self {
            widget.retire_cold();
        }
    }
}

impl RowPatch<FormGeneration> for Generation2dGenerationPatch {
    fn applied(&self, item: &FormGeneration) -> MutationApplyResult<FormGeneration> {
        let mut next = item.clone();
        if let Some(name) = &self.name {
            next.name.clone_from(name);
        }
        if let Some(delta) = &self.values {
            apply_values(&mut next, delta).map_err(|error| error.under(["values"]))?;
        }
        Ok(next)
    }
    fn inverse_against(&self, base: &FormGeneration) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), values: self.values.as_ref().map(|delta| inverse_values(delta, base)) }
    }
    fn composed(self, later: Self) -> Self {
        let values = match (self.values, later.values) {
            (Some(first), Some(second)) => Some(absorb_delta(first, second)),
            (first, None) => first,
            (None, second) => second,
        };
        Self { name: later.name.or(self.name), values }
    }
    fn between(base: &FormGeneration, other: &FormGeneration) -> Option<Self> {
        let patch = Self { name: (base.name != other.name).then(|| other.name.clone()), values: between_values(base, other) };
        (patch != Self::default()).then_some(patch)
    }
    fn retire(self) {
        if let Some(values) = self.values {
            values.retire_cold();
        }
    }
}

/// 🧩 The shared shape of every keyed collection delta.
pub(crate) trait Delta: Default + Clone {
    type Item: Row;
    type Patch: RowPatch<Self::Item>;
    fn added(&self) -> &[Self::Item];
    fn removed(&self) -> &[String];
    fn patched(&self) -> Vec<(&str, &Self::Patch)>;
    fn reordered(&self) -> Option<&[String]>;
    fn into_parts(self) -> (Vec<Self::Item>, Vec<String>, Vec<(String, Self::Patch)>, Option<Vec<String>>);
    fn from_parts(added: Vec<Self::Item>, removed: Vec<String>, patched: Vec<(String, Self::Patch)>, reordered: Option<Vec<String>>) -> Self;
}

macro_rules! impl_delta {
    ($delta:ty, $item:ty, $patch:ty, $entry:ident, $field:ident) => {
        impl Delta for $delta {
            type Item = $item;
            type Patch = $patch;
            fn added(&self) -> &[$item] {
                &self.added
            }
            fn removed(&self) -> &[String] {
                &self.removed
            }
            fn patched(&self) -> Vec<(&str, &$patch)> {
                self.patched.iter().map(|entry| (entry.id.as_str(), &entry.$field)).collect()
            }
            fn reordered(&self) -> Option<&[String]> {
                self.reordered.as_deref()
            }
            fn into_parts(self) -> (Vec<$item>, Vec<String>, Vec<(String, $patch)>, Option<Vec<String>>) {
                (self.added, self.removed, self.patched.into_iter().map(|entry| (entry.id, entry.$field)).collect(), self.reordered)
            }
            fn from_parts(added: Vec<$item>, removed: Vec<String>, patched: Vec<(String, $patch)>, reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched: patched.into_iter().map(|(id, $field)| $entry { id, $field }).collect(), reordered }
            }
        }
    };
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
            fn reordered(&self) -> Option<&[String]> {
                None
            }
            fn into_parts(self) -> (Vec<$item>, Vec<String>, Vec<(String, $item)>, Option<Vec<String>>) {
                (self.added, self.removed, self.patched.into_iter().map(|row| (Row::id(&row).to_string(), row)).collect(), None)
            }
            fn from_parts(added: Vec<$item>, removed: Vec<String>, patched: Vec<(String, $item)>, _reordered: Option<Vec<String>>) -> Self {
                Self { added, removed, patched: patched.into_iter().map(|(_, row)| row).collect() }
            }
        }
    };
}

impl_delta!(Generation2dWidgetsDelta, Widget, Generation2dWidgetPatch, Generation2dWidgetPatchEntry, patch);
impl_delta!(Generation2dSynapsesDelta, SynapseSpec, SynapseSpec, Generation2dSynapsePatchEntry, item);
impl_delta!(Generation2dGenerationsDelta, FormGeneration, Generation2dGenerationPatch, Generation2dGenerationPatchEntry, patch);
impl_unordered_delta!(Generation2dLayoutDelta, Generation2dLayoutRow);
impl_unordered_delta!(Generation2dValuesDelta, Generation2dValueRow);

impl Generation2dGenerationsDelta {
    fn retire_cold(self) {
        let (added, _, patched, _) = Delta::into_parts(self);
        added.into_iter().for_each(Row::retire);
        patched.into_iter().for_each(|(_, patch)| RowPatch::<FormGeneration>::retire(patch));
    }
}

impl Generation2dValuesDelta {
    fn retire_cold(self) {
        let (added, _, patched, _) = Delta::into_parts(self);
        added.into_iter().chain(patched.into_iter().map(|(_, row)| row)).for_each(Row::retire);
    }
}

impl Generation2dWidgetsDelta {
    fn retire_cold(self) {
        let (added, _, patched, _) = Delta::into_parts(self);
        added.into_iter().for_each(Row::retire);
        patched.into_iter().for_each(|(_, patch)| RowPatch::<Widget>::retire(patch));
    }
}
//#endregion 🔖️RowAlgebra

//#region 🔖️DeltaAlgebra
fn rejection(code: &str, message: &str, at: [String; 2]) -> MutationApplyError {
    MutationApplyError::new(code, message).at(at)
}

fn apply_delta<D: Delta>(items: &[D::Item], delta: &D) -> MutationApplyResult<Vec<D::Item>> {
    for (index, id) in delta.removed().iter().enumerate() {
        if !items.iter().any(|item| item.id() == id) {
            return Err(rejection("mutation.apply.missing-target", "removed item does not exist", ["removed".into(), index.to_string()]));
        }
        if delta.removed()[..index].contains(id) {
            return Err(rejection("mutation.apply.duplicate-target", "item is removed more than once", ["removed".into(), index.to_string()]));
        }
    }
    for (index, item) in delta.added().iter().enumerate() {
        let survives = items.iter().any(|existing| existing.id() == item.id()) && !delta.removed().iter().any(|id| id == item.id());
        if survives || delta.added()[..index].iter().any(|existing| existing.id() == item.id()) {
            return Err(rejection("mutation.apply.duplicate-target", "added item identity already exists", ["added".into(), index.to_string()]));
        }
    }
    let patched = delta.patched();
    for (index, (id, _)) in patched.iter().enumerate() {
        if !items.iter().any(|existing| existing.id() == *id) {
            return Err(rejection("mutation.apply.missing-target", "patched item does not exist", ["patched".into(), index.to_string()]));
        }
        if delta.removed().iter().any(|removed| removed == id) {
            return Err(rejection("mutation.apply.conflicting-target", "item cannot be removed and patched", ["patched".into(), index.to_string()]));
        }
        if patched[..index].iter().any(|(prior, _)| prior == id) {
            return Err(rejection("mutation.apply.duplicate-target", "item is patched more than once", ["patched".into(), index.to_string()]));
        }
    }
    let mut next: Vec<D::Item> = items.iter().filter(|item| !delta.removed().iter().any(|id| id == item.id())).cloned().collect();
    next.extend(delta.added().iter().cloned());
    for (id, patch) in patched {
        if let Some(position) = next.iter().position(|existing| existing.id() == id) {
            next[position] = patch.applied(&next[position]).map_err(|error| error.under(["patched", id]))?;
        }
    }
    if let Some(order) = delta.reordered() {
        if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|item| item.id() == id)) {
            return Err(MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
        }
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(position) = next.iter().position(|item| item.id() == id) {
                ordered.push(next.remove(position));
            }
        }
        next = ordered;
    }
    Ok(next)
}

fn is_empty_delta<D: Delta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().is_empty() && delta.reordered().is_none()
}

enum Net<T, P> {
    Patch(P),
    Remove,
    Add(T),
    Replace(T),
}

impl<T: Row, P: RowPatch<T>> Net<T, P> {
    fn retire(self) {
        match self {
            Self::Patch(patch) => patch.retire(),
            Self::Remove => {}
            Self::Add(item) | Self::Replace(item) => item.retire(),
        }
    }
}

/// ➕️ Composes `first` then `second` per id (patch∘patch → one patch, add∘remove → nothing, remove∘add → replace) in a canonical row order; every displaced owned value is closed.
fn absorb_delta<D: Delta>(first: D, second: D) -> D {
    let mut nets: std::collections::BTreeMap<String, Net<D::Item, D::Patch>> = std::collections::BTreeMap::new();
    let mut appended: Vec<String> = Vec::new();
    let second_removed: Vec<String> = second.removed().to_vec();
    let second_added: Vec<String> = second.added().iter().map(|item| item.id().to_string()).collect();
    let first_order = first.reordered().map(<[String]>::to_vec);
    let second_order = second.reordered().map(<[String]>::to_vec);
    for delta in [first, second] {
        let (added, removed, patched, _) = delta.into_parts();
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
    let reordered = match (second_order, first_order) {
        (Some(order), _) => Some(order),
        (None, Some(order)) => Some(order.into_iter().filter(|id| !second_removed.contains(id)).chain(second_added.into_iter().filter(|id| nets.contains_key(id))).collect()),
        (None, None) => None,
    };
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
    if reordered.is_some() {
        added.extend(adds.into_values());
    } else {
        for id in &appended {
            if let Some(item) = adds.remove(id) {
                added.push(item);
            }
        }
    }
    D::from_parts(added, removed, patched, reordered)
}

fn absorb_optional<D: Delta>(first: &mut Option<D>, second: Option<D>) {
    let Some(second) = second else { return };
    let merged = absorb_delta(first.take().unwrap_or_default(), second);
    *first = (!is_empty_delta(&merged)).then_some(merged);
}

fn forward_order<D: Delta>(base_ids: &[String], delta: &D) -> Vec<String> {
    let mut ids: Vec<String> = base_ids.iter().filter(|id| !delta.removed().contains(id)).cloned().collect();
    ids.extend(delta.added().iter().map(|item| item.id().to_string()));
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => ids,
    }
}

/// 🔁️ The negative delta against `base`: patches restore base rows, adds become removes, removes re-add base rows.
fn inverse_delta<D: Delta>(delta: &D, base: &[D::Item]) -> D {
    let find = |id: &str| base.iter().find(|item| item.id() == id);
    let removed: Vec<String> = delta.added().iter().map(|item| item.id().to_string()).collect();
    let added: Vec<D::Item> = delta.removed().iter().filter_map(|id| find(id).cloned()).collect();
    let patched: Vec<(String, D::Patch)> = delta.patched().into_iter().filter_map(|(id, patch)| find(id).map(|item| (id.to_string(), patch.inverse_against(item)))).collect();
    let base_ids: Vec<String> = base.iter().map(|item| item.id().to_string()).collect();
    let mut simulated: Vec<String> = forward_order(&base_ids, delta).into_iter().filter(|id| !removed.contains(id)).collect();
    simulated.extend(added.iter().map(|item| item.id().to_string()));
    let reordered = (simulated != base_ids).then_some(base_ids);
    D::from_parts(added, removed, patched, reordered)
}

fn inverse_optional<D: Delta>(delta: &Option<D>, base: &[D::Item]) -> Option<D> {
    delta.as_ref().map(|delta| inverse_delta(delta, base))
}

fn between_delta<D: Delta>(base: &[D::Item], other: &[D::Item]) -> Option<D> {
    let removed: Vec<String> = base.iter().filter(|item| !other.iter().any(|candidate| candidate.id() == item.id())).map(|item| item.id().to_string()).collect();
    let added: Vec<D::Item> = other.iter().filter(|item| !base.iter().any(|candidate| candidate.id() == item.id())).cloned().collect();
    let patched: Vec<(String, D::Patch)> = base
        .iter()
        .filter_map(|item| other.iter().find(|candidate| candidate.id() == item.id()).and_then(|candidate| D::Patch::between(item, candidate)).map(|patch| (item.id().to_string(), patch)))
        .collect();
    let mut natural: Vec<String> = base.iter().filter(|item| !removed.iter().any(|id| id == item.id())).map(|item| item.id().to_string()).collect();
    natural.extend(added.iter().map(|item| item.id().to_string()));
    let target: Vec<String> = other.iter().map(|item| item.id().to_string()).collect();
    let reordered = (natural != target).then_some(target);
    (!(removed.is_empty() && added.is_empty() && patched.is_empty() && reordered.is_none())).then(|| D::from_parts(added, removed, patched, reordered))
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

fn between_values(base: &FormGeneration, other: &FormGeneration) -> Option<Generation2dValuesDelta> {
    let rows = |generation: &FormGeneration| -> Vec<Generation2dValueRow> { generation.values.iter().map(|(id, value)| Generation2dValueRow { question_id: id.clone(), value: value.clone() }).collect() };
    between_delta::<Generation2dValuesDelta>(&rows(base), &rows(other))
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

    fn apply_generation(&self, snapshot: &mut Generation2dSnapshot) -> MutationApplyResult<()> {
        if self.generations.is_none() && self.selected_generation.is_none() && self.preview_text.is_none() {
            return Ok(());
        }
        let mut state = (*snapshot.generation).clone();
        if let Some(delta) = &self.generations {
            state.generations = apply_delta(&state.generations, delta).map_err(|error| error.under(["generations"]))?;
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
    fn apply(&self, snapshot: &Generation2dSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<Generation2dSnapshot> {
        let mut next = snapshot.clone();
        let applied = (|| {
            if let Some(schema) = &self.schema {
                next.host_snapshot.schema.clone_from(schema);
            }
            if let Some(camera) = &self.camera {
                next.host_snapshot.camera = camera.clone();
            }
            if let Some(delta) = &self.widgets {
                next.host_snapshot.widgets = apply_delta(&next.host_snapshot.widgets, delta).map_err(|error| error.under(["widgets"]))?;
            }
            if let Some(delta) = &self.synapses {
                next.host_snapshot.synapses = apply_delta(&next.host_snapshot.synapses, delta).map_err(|error| error.under(["synapses"]))?;
            }
            self.apply_layout(&mut next)?;
            self.apply_generation(&mut next)
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
        absorb_optional(&mut self.widgets, other.widgets);
        absorb_optional(&mut self.synapses, other.synapses);
        absorb_optional(&mut self.layout, other.layout);
        absorb_optional(&mut self.generations, other.generations);
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
            widgets: inverse_optional(&self.widgets, &base.host_snapshot.widgets),
            synapses: inverse_optional(&self.synapses, &base.host_snapshot.synapses),
            layout: self.layout.as_ref().map(|delta| inverse_layout(delta, base)),
            generations: inverse_optional(&self.generations, &base.generation.generations),
            selected_generation: self.selected_generation.as_ref().map(|_| Generation2dSelectionChange { id: base.generation.selected_generation_id.clone() }),
            preview_text: self.preview_text.as_ref().map(|_| Generation2dPreviewChange { text: base.generation.preview_text.clone() }),
        }
    }
    fn between(base: &Generation2dSnapshot, other: &Generation2dSnapshot) -> Self {
        let layout_rows = |snapshot: &Generation2dSnapshot| -> Vec<Generation2dLayoutRow> { snapshot.host_snapshot.layout.iter().map(|(id, layout)| Generation2dLayoutRow { id: id.clone(), layout: layout.clone() }).collect() };
        Self {
            schema: (base.host_snapshot.schema != other.host_snapshot.schema).then(|| other.host_snapshot.schema.clone()),
            camera: (base.host_snapshot.camera != other.host_snapshot.camera).then(|| other.host_snapshot.camera.clone()),
            widgets: between_delta(&base.host_snapshot.widgets, &other.host_snapshot.widgets),
            synapses: between_delta(&base.host_snapshot.synapses, &other.host_snapshot.synapses),
            layout: between_delta(&layout_rows(base), &layout_rows(other)),
            generations: between_delta(&base.generation.generations, &other.generation.generations),
            selected_generation: (base.generation.selected_generation_id != other.generation.selected_generation_id).then(|| Generation2dSelectionChange { id: other.generation.selected_generation_id.clone() }),
            preview_text: (base.generation.preview_text != other.generation.preview_text).then(|| Generation2dPreviewChange { text: other.generation.preview_text.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.camera.is_none()
            && self.widgets.as_ref().is_none_or(is_empty_delta)
            && self.synapses.as_ref().is_none_or(is_empty_delta)
            && self.layout.as_ref().is_none_or(is_empty_delta)
            && self.generations.as_ref().is_none_or(is_empty_delta)
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
