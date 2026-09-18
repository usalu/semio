//! 🔺️ PdfDiff (1.7) — sparse diff over the typed document model. Ticket
//! 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened it with the model: `pages` is an index-keyed
//! triple of per-field page patches whose `content` and `annotations` are themselves
//! index-keyed triples (an inserted operator is one added row, not a re-sent stream); every
//! document collection (`fonts`, `images`, `forms`, graphics states, shadings, patterns, named
//! colour spaces and property lists, embedded files) is an id-keyed triple of whole values;
//! `outlines`, `named_destinations`, `page_labels`, `output_intents` are index-keyed triples;
//! the catalog scalars are tri-state (`Clear`/`Set`); `info` is whole-value replaced; and the
//! retained COS lanes keep their recursive `PdfValueDiff` patches (`objects` keyed by `ObjRef`,
//! `trailer`/`catalog_extra` name-keyed).
//!
//! Every triple follows the recipe: `removed`/`modified` address BASE state (removals processed
//! descending), `added` addresses FINAL state (ascending insert).
//!
//! Codec: the derive-owned value encoding is the wire — one JSON line as text, the container-less
//! pack record body as binary — so every lane rides one codec instead of a hand-rolled one per
//! field (see `🔖️DiffCodec`).

use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

//#region 🔖️TriState
/// 🎚️ A change to an optional field: cleared, or set to `value`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PdfSet<T> {
    Clear,
    Set { value: T },
}

impl<T: Clone> PdfSet<T> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_option(value: &Option<T>) -> Self {
        match value {
            Some(value) => PdfSet::Set { value: value.clone() },
            None => PdfSet::Clear,
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn into_option(self) -> Option<T> {
        match self {
            PdfSet::Set { value } => Some(value),
            PdfSet::Clear => None,
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn tri<T: Clone + PartialEq>(a: &Option<T>, b: &Option<T>) -> Option<PdfSet<T>> {
    (a != b).then(|| PdfSet::from_option(b))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_tri<T: Clone>(slot: &mut Option<T>, diff: &Option<PdfSet<T>>) {
    if let Some(set) = diff {
        *slot = set.clone().into_option();
    }
}
//#endregion 🔖️TriState

//#region 🔖️IndexedTriple
/// 📦️ An index-keyed triple of whole values (positional collections: content operators,
/// annotations, outlines, named destinations, page labels, output intents).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfIndexedItem<T> {
    pub index: usize,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfIndexedDiff<T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfIndexedItem<T>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfIndexedItem<T>>,
}

impl<T> Default for PdfIndexedDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

impl<T: Clone + PartialEq> PdfIndexedDiff<T> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    /// 🧭️ Positional delta: common prefix of equal items is untouched; the divergent tail is
    /// matched by position (modified where both sides have an item, removed/added beyond).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(a: &[T], b: &[T]) -> Self {
        let mut diff = Self::default();
        let common = a.len().min(b.len());
        for index in 0..common {
            if a[index] != b[index] {
                diff.modified.push(PdfIndexedItem { index, value: b[index].clone() });
            }
        }
        for index in common..a.len() {
            diff.removed.push(index);
        }
        for index in common..b.len() {
            diff.added.push(PdfIndexedItem { index, value: b[index].clone() });
        }
        diff
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate(&self, base_len: usize, field: &str) -> MutationApplyResult<()> {
        validate_index_triple(base_len, &self.removed, &self.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &self.added.iter().map(|item| item.index).collect::<Vec<_>>(), field)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[T]) -> Vec<T> {
        let mut next: Vec<T> = base.to_vec();
        for item in &self.modified {
            if let Some(slot) = next.get_mut(item.index) {
                *slot = item.value.clone();
            }
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            if index < next.len() {
                next.remove(index);
            }
        }
        let mut added = self.added.clone();
        added.sort_by_key(|item| item.index);
        for item in added {
            let index = item.index.min(next.len());
            next.insert(index, item.value);
        }
        next
    }

    /// ➕️ Sequential coalesce: `self` then `other`, transported through `self`'s index moves.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(self, other: Self) -> Self {
        // 🧮 Rebuild through a virtual base: every index the pair ever touches is materialized
        // as a slot, both diffs applied in order, and the combined triple read back off it.
        let max_base = self.removed.iter().copied().chain(self.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0).max(self.added.len() + other.removed.iter().copied().chain(other.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0));
        let base: Vec<Slot<T>> = (0..max_base + self.added.len() + other.added.len() + 1).map(|index| Slot::Base(index)).collect();
        let mid = self.map_slots(&base);
        let end = other.map_slots(&mid);
        Self::from_slots(&base, &end)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn map_slots(&self, base: &[Slot<T>]) -> Vec<Slot<T>> {
        let mut next: Vec<Slot<T>> = base.to_vec();
        for item in &self.modified {
            if let Some(slot) = next.get_mut(item.index) {
                *slot = match slot {
                    Slot::Base(index) => Slot::Modified(*index, item.value.clone()),
                    Slot::Modified(index, _) => Slot::Modified(*index, item.value.clone()),
                    Slot::Added(_) => Slot::Added(item.value.clone()),
                };
            }
        }
        let mut removed = self.removed.clone();
        removed.sort_unstable_by(|a, b| b.cmp(a));
        for index in removed {
            if index < next.len() {
                next.remove(index);
            }
        }
        let mut added = self.added.clone();
        added.sort_by_key(|item| item.index);
        for item in added {
            next.insert(item.index.min(next.len()), Slot::Added(item.value));
        }
        next
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_slots(base: &[Slot<T>], end: &[Slot<T>]) -> Self {
        let mut diff = Self::default();
        let surviving: HashSet<usize> = end
            .iter()
            .filter_map(|slot| match slot {
                Slot::Base(index) | Slot::Modified(index, _) => Some(*index),
                Slot::Added(_) => None,
            })
            .collect();
        for slot in base {
            if let Slot::Base(index) = slot {
                if !surviving.contains(index) {
                    diff.removed.push(*index);
                }
            }
        }
        for (position, slot) in end.iter().enumerate() {
            match slot {
                Slot::Modified(index, value) => diff.modified.push(PdfIndexedItem { index: *index, value: value.clone() }),
                Slot::Added(value) => diff.added.push(PdfIndexedItem { index: position, value: value.clone() }),
                Slot::Base(_) => {}
            }
        }
        diff
    }
}

#[derive(Clone)]
enum Slot<T> {
    Base(usize),
    Modified(usize, T),
    Added(T),
}
//#endregion 🔖️IndexedTriple

//#region 🔖️KeyedTriple
/// 🆔 An id-keyed triple of whole values (the document collections keyed by `id`/`name`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfKeyedItem<T> {
    pub key: String,
    pub value: T,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfKeyedDiff<T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfKeyedItem<T>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfIndexedItem<T>>,
}

impl<T> Default for PdfKeyedDiff<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

/// 🔑 A collection item that carries its own key.
pub trait Keyed {
    fn key(&self) -> &str;
}

macro_rules! keyed {
    ($($ty:ty => $field:ident),* $(,)?) => {
        $(impl Keyed for $ty {
            fn key(&self) -> &str {
                &self.$field
            }
        })*
    };
}
keyed!(PdfFont => id, PdfImage => id, PdfFormXObject => id, PdfExtGState => id, PdfShading => id, PdfPattern => id, PdfNamedColorSpace => name, PdfNamedProperties => name, PdfEmbeddedFile => id);

impl<T: Clone + PartialEq + Keyed> PdfKeyedDiff<T> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn between(a: &[T], b: &[T]) -> Self {
        let mut diff = Self::default();
        let b_keys: HashSet<&str> = b.iter().map(Keyed::key).collect();
        for item in a {
            if !b_keys.contains(item.key()) {
                diff.removed.push(item.key().to_string());
            }
        }
        for (index, item) in b.iter().enumerate() {
            match a.iter().find(|candidate| candidate.key() == item.key()) {
                Some(existing) if existing == item => {}
                Some(_) => diff.modified.push(PdfKeyedItem { key: item.key().to_string(), value: item.clone() }),
                None => diff.added.push(PdfIndexedItem { index, value: item.clone() }),
            }
        }
        diff
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn validate(&self, base: &[T], field: &str) -> MutationApplyResult<()> {
        let keys: Vec<String> = base.iter().map(|item| item.key().to_string()).collect();
        validate_named_keys(&keys, &self.removed, &self.modified.iter().map(|item| item.key.clone()).collect::<Vec<_>>(), &self.added.iter().map(|item| item.value.key().to_string()).collect::<Vec<_>>(), field)?;
        for item in &self.modified {
            if item.value.key() != item.key {
                return Err(MutationApplyError::new("mutation.apply.conflicting-target", "keyed modification renames its own key").at([field]));
            }
        }
        let removed_indices: Vec<usize> = (0..self.removed.len()).collect();
        validate_index_triple(base.len(), &removed_indices, &[], &self.added.iter().map(|item| item.index).collect::<Vec<_>>(), field)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, base: &[T]) -> Vec<T> {
        let mut next: Vec<T> = base.iter().filter(|item| !self.removed.iter().any(|key| key == item.key())).cloned().collect();
        for item in &self.modified {
            if let Some(slot) = next.iter_mut().find(|candidate| candidate.key() == item.key) {
                *slot = item.value.clone();
            }
        }
        let mut added = self.added.clone();
        added.sort_by_key(|item| item.index);
        for item in added {
            next.insert(item.index.min(next.len()), item.value);
        }
        next
    }

    /// ➕️ Sequential coalesce by key: later removals win over earlier additions/modifications,
    /// later modifications replace earlier ones, additions of a key removed earlier become
    /// modifications of the base.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn absorb(self, other: Self) -> Self {
        let mut out = Self::default();
        let first_added: HashSet<String> = self.added.iter().map(|item| item.value.key().to_string()).collect();
        for key in self.removed.iter().chain(other.removed.iter()) {
            if first_added.contains(key) && other.removed.contains(key) {
                continue;
            }
            if other.added.iter().any(|item| item.value.key() == key) && self.removed.contains(key) {
                continue;
            }
            if !out.removed.contains(key) {
                out.removed.push(key.clone());
            }
        }
        for item in &self.modified {
            if !other.removed.contains(&item.key) && !other.modified.iter().any(|later| later.key == item.key) {
                out.modified.push(item.clone());
            }
        }
        for item in &other.modified {
            if let Some(added) = self.added.iter().find(|added| added.value.key() == item.key) {
                out.added.push(PdfIndexedItem { index: added.index, value: item.value.clone() });
            } else {
                out.modified.push(item.clone());
            }
        }
        for item in &self.added {
            if !other.removed.contains(&item.value.key().to_string()) && !other.modified.iter().any(|later| later.key == item.value.key()) {
                out.added.push(item.clone());
            }
        }
        for item in &other.added {
            if self.removed.contains(&item.value.key().to_string()) {
                out.modified.push(PdfKeyedItem { key: item.value.key().to_string(), value: item.value.clone() });
            } else {
                out.added.push(item.clone());
            }
        }
        out.added.sort_by_key(|item| item.index);
        out
    }
}
//#endregion 🔖️KeyedTriple

//#region 🔖️PageDiff
/// 📄️ Sparse patch for one `PdfPage`: scalar fields are set/tri-state, `content` and
/// `annotations` are index-keyed triples, `extra` is a name-keyed dictionary patch.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub media_box: Option<PdfRect>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub crop_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bleed_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub trim_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub art_box: Option<PdfSet<PdfRect>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotate: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub user_unit: Option<PdfSet<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<PdfIndexedDiff<PdfOp>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<PdfIndexedDiff<PdfAnnotation>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<PdfSet<PdfTransparencyGroup>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub struct_parents: Option<PdfSet<u32>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<PdfSet<Vec<PdfDictEntry>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<PdfSet<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfSet<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub additional_actions: Option<Vec<PdfDictEntry>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<PdfDictDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_page_diff(page: &mut PdfPage, diff: &PdfPageDiff) {
    if let Some(v) = diff.media_box {
        page.media_box = v;
    }
    apply_tri(&mut page.crop_box, &diff.crop_box);
    apply_tri(&mut page.bleed_box, &diff.bleed_box);
    apply_tri(&mut page.trim_box, &diff.trim_box);
    apply_tri(&mut page.art_box, &diff.art_box);
    if let Some(v) = diff.rotate {
        page.rotate = v;
    }
    apply_tri(&mut page.user_unit, &diff.user_unit);
    if let Some(content) = &diff.content {
        page.content = content.apply(&page.content);
    }
    if let Some(annotations) = &diff.annotations {
        page.annotations = annotations.apply(&page.annotations);
    }
    apply_tri(&mut page.group, &diff.group);
    apply_tri(&mut page.thumbnail, &diff.thumbnail);
    apply_tri(&mut page.struct_parents, &diff.struct_parents);
    apply_tri(&mut page.transition, &diff.transition);
    apply_tri(&mut page.duration, &diff.duration);
    apply_tri(&mut page.metadata, &diff.metadata);
    if let Some(v) = &diff.additional_actions {
        page.additional_actions = v.clone();
    }
    if let Some(extra) = &diff.extra {
        page.extra = apply_dict_diff(extra, &page.extra);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_diff_between(a: &PdfPage, b: &PdfPage) -> PdfPageDiff {
    let content = PdfIndexedDiff::between(&a.content, &b.content);
    let annotations = PdfIndexedDiff::between(&a.annotations, &b.annotations);
    let extra = dict_diff_between(&a.extra, &b.extra);
    PdfPageDiff {
        media_box: (a.media_box != b.media_box).then_some(b.media_box),
        crop_box: tri(&a.crop_box, &b.crop_box),
        bleed_box: tri(&a.bleed_box, &b.bleed_box),
        trim_box: tri(&a.trim_box, &b.trim_box),
        art_box: tri(&a.art_box, &b.art_box),
        rotate: (a.rotate != b.rotate).then_some(b.rotate),
        user_unit: tri(&a.user_unit, &b.user_unit),
        content: (!content.is_empty()).then_some(content),
        annotations: (!annotations.is_empty()).then_some(annotations),
        group: tri(&a.group, &b.group),
        thumbnail: tri(&a.thumbnail, &b.thumbnail),
        struct_parents: tri(&a.struct_parents, &b.struct_parents),
        transition: tri(&a.transition, &b.transition),
        duration: tri(&a.duration, &b.duration),
        metadata: tri(&a.metadata, &b.metadata),
        additional_actions: (a.additional_actions != b.additional_actions).then(|| b.additional_actions.clone()),
        extra: (!extra.is_empty()).then_some(extra),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_page_diff(diff: &PdfPageDiff, base: &PdfPage) -> MutationApplyResult<()> {
    if let Some(content) = &diff.content {
        content.validate(base.content.len(), "content")?;
    }
    if let Some(annotations) = &diff.annotations {
        annotations.validate(base.annotations.len(), "annotations")?;
    }
    if let Some(extra) = &diff.extra {
        validate_dict_diff(extra, &base.extra)?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_page_diff(base: &mut PdfPageDiff, other: PdfPageDiff) {
    if other.media_box.is_some() {
        base.media_box = other.media_box;
    }
    for (slot, next) in [(&mut base.crop_box, other.crop_box), (&mut base.bleed_box, other.bleed_box), (&mut base.trim_box, other.trim_box), (&mut base.art_box, other.art_box)] {
        if next.is_some() {
            *slot = next;
        }
    }
    if other.rotate.is_some() {
        base.rotate = other.rotate;
    }
    if other.user_unit.is_some() {
        base.user_unit = other.user_unit;
    }
    base.content = match (base.content.take(), other.content) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            let merged = a.absorb(b);
            (!merged.is_empty()).then_some(merged)
        }
    };
    base.annotations = match (base.annotations.take(), other.annotations) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            let merged = a.absorb(b);
            (!merged.is_empty()).then_some(merged)
        }
    };
    if other.group.is_some() {
        base.group = other.group;
    }
    if other.thumbnail.is_some() {
        base.thumbnail = other.thumbnail;
    }
    if other.struct_parents.is_some() {
        base.struct_parents = other.struct_parents;
    }
    if other.transition.is_some() {
        base.transition = other.transition;
    }
    if other.duration.is_some() {
        base.duration = other.duration;
    }
    if other.metadata.is_some() {
        base.metadata = other.metadata;
    }
    if other.additional_actions.is_some() {
        base.additional_actions = other.additional_actions;
    }
    base.extra = match (base.extra.take(), other.extra) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            let merged = absorb_dict_diff(a, b);
            (!merged.is_empty()).then_some(merged)
        }
    };
}
//#endregion 🔖️PageDiff

//#region 🔖️PagesTriple
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageModified {
    pub index: usize,
    pub diff: PdfPageDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPageAdded {
    pub index: usize,
    pub page: PdfPage,
}

/// 📦️ Index-keyed `pages` triple (positional -- the recipe's "index usize" key kind).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PdfPagesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<PdfPageModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<PdfPageAdded>,
}

impl PdfPagesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Apply semantics (normative): `removed`/`modified` indices refer to BASE state (removals
/// processed descending); `added` indices refer to FINAL state (ascending insert).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_pages_diff(diff: &PdfPagesDiff, base: &[PdfPage]) -> Vec<PdfPage> {
    let mut next: Vec<PdfPage> = base.to_vec();
    for modified in &diff.modified {
        if let Some(page) = next.get_mut(modified.index) {
            apply_page_diff(page, &modified.diff);
        }
    }
    let mut removed = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < next.len() {
            next.remove(index);
        }
    }
    let mut added = diff.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        next.insert(item.index.min(next.len()), item.page);
    }
    next
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pages_diff_between(a: &[PdfPage], b: &[PdfPage]) -> PdfPagesDiff {
    let mut diff = PdfPagesDiff::default();
    let common = a.len().min(b.len());
    for index in 0..common {
        if a[index] != b[index] {
            diff.modified.push(PdfPageModified { index, diff: page_diff_between(&a[index], &b[index]) });
        }
    }
    for index in common..a.len() {
        diff.removed.push(index);
    }
    for index in common..b.len() {
        diff.added.push(PdfPageAdded { index, page: b[index].clone() });
    }
    diff
}

/// ➕️ Sequential coalesce of two page triples: `d1`'s modified pages absorb `d2`'s patches
/// where both touch a page that survives, `d2`'s removals of `d1`'s additions cancel, and the
/// rest is transported through `d1`'s index moves via the same virtual-base slot algorithm the
/// generic triple uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_law_pages_diff(d1: PdfPagesDiff, d2: &PdfPagesDiff) -> PdfPagesDiff {
    #[derive(Clone, PartialEq)]
    struct PageSlotValue {
        page: Option<PdfPage>,
        patch: PdfPageDiff,
    }
    let first: PdfIndexedDiff<PageSlotValue> = PdfIndexedDiff { removed: d1.removed.clone(), modified: d1.modified.iter().map(|m| PdfIndexedItem { index: m.index, value: PageSlotValue { page: None, patch: m.diff.clone() } }).collect(), added: d1.added.iter().map(|a| PdfIndexedItem { index: a.index, value: PageSlotValue { page: Some(a.page.clone()), patch: PdfPageDiff::default() } }).collect() };
    let second: PdfIndexedDiff<PageSlotValue> = PdfIndexedDiff { removed: d2.removed.clone(), modified: d2.modified.iter().map(|m| PdfIndexedItem { index: m.index, value: PageSlotValue { page: None, patch: m.diff.clone() } }).collect(), added: d2.added.iter().map(|a| PdfIndexedItem { index: a.index, value: PageSlotValue { page: Some(a.page.clone()), patch: PdfPageDiff::default() } }).collect() };
    let max_base = first.removed.iter().copied().chain(first.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0).max(first.added.len() + second.removed.iter().copied().chain(second.modified.iter().map(|m| m.index)).max().map(|m| m + 1).unwrap_or(0));
    let base: Vec<Slot<PageSlotValue>> = (0..max_base + first.added.len() + second.added.len() + 1).map(Slot::Base).collect();
    let mut mid = base.clone();
    for item in &first.modified {
        if let Some(slot) = mid.get_mut(item.index) {
            *slot = match slot {
                Slot::Base(index) | Slot::Modified(index, _) => Slot::Modified(*index, item.value.clone()),
                Slot::Added(_) => Slot::Added(item.value.clone()),
            };
        }
    }
    let mut removed = first.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < mid.len() {
            mid.remove(index);
        }
    }
    let mut added = first.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        mid.insert(item.index.min(mid.len()), Slot::Added(item.value));
    }
    let mut end = mid.clone();
    for item in &second.modified {
        if let Some(slot) = end.get_mut(item.index) {
            *slot = match slot {
                Slot::Base(index) => Slot::Modified(*index, item.value.clone()),
                Slot::Modified(index, previous) => {
                    let mut patch = previous.patch.clone();
                    absorb_page_diff(&mut patch, item.value.patch.clone());
                    Slot::Modified(*index, PageSlotValue { page: None, patch })
                }
                Slot::Added(previous) => {
                    let mut page = previous.page.clone().unwrap_or_default();
                    apply_page_diff(&mut page, &item.value.patch);
                    Slot::Added(PageSlotValue { page: Some(page), patch: PdfPageDiff::default() })
                }
            };
        }
    }
    let mut removed = second.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < end.len() {
            end.remove(index);
        }
    }
    let mut added = second.added.clone();
    added.sort_by_key(|item| item.index);
    for item in added {
        end.insert(item.index.min(end.len()), Slot::Added(item.value));
    }
    let surviving: HashSet<usize> = end
        .iter()
        .filter_map(|slot| match slot {
            Slot::Base(index) | Slot::Modified(index, _) => Some(*index),
            Slot::Added(_) => None,
        })
        .collect();
    let mut out = PdfPagesDiff::default();
    for slot in &base {
        if let Slot::Base(index) = slot {
            if !surviving.contains(index) {
                out.removed.push(*index);
            }
        }
    }
    for (position, slot) in end.iter().enumerate() {
        match slot {
            Slot::Modified(index, value) => out.modified.push(PdfPageModified { index: *index, diff: value.patch.clone() }),
            Slot::Added(value) => out.added.push(PdfPageAdded { index: position, page: value.page.clone().unwrap_or_default() }),
            Slot::Base(_) => {}
        }
    }
    out
}
//#endregion 🔖️PagesTriple

