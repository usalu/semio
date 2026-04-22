// Sparse structural diffs (mirror Semio.cs: removed → updated → added). Included by `pub mod diff` in lib.rs.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::attribute::{AttributeFullDto, AttributeIdDto};
use crate::author::{AuthorFullDto, AuthorIdDto};
use crate::benchmark::{BenchmarkFullDto, BenchmarkIdDto};
use crate::concept::{ConceptFullDto, ConceptIdDto};
use crate::connection::{ConnectionFullDto, ConnectionIdDto};
use crate::connector::{ConnectorFullDto, ConnectorIdDto};
use crate::design::DesignFullDto;
use crate::file::{FileFullDto, FileIdDto};
use crate::folder::{FolderFullDto, FolderIdDto};
use crate::geom::{Camera, Coordinate, Location, Plane, Vector};
use crate::group::{GroupFullDto, GroupIdDto};
use crate::id::Id;
use crate::kit::KitIdDto;
use crate::layer::{LayerFullDto, LayerIdDto};
use crate::piece::{PieceFullDto, PieceIdDto};
use crate::port::{PortFullDto, PortIdDto};
use crate::prop::{PropFullDto, PropIdDto};
use crate::quality::{QualityFullDto, QualityIdDto};
use crate::representation::{RepresentationFullDto, RepresentationIdDto};
use crate::side::SideMetadataDto;
use crate::stat::{StatFullDto, StatIdDto};
use crate::tag::{TagFullDto, TagIdDto};
use crate::typ::{TypeFullDto, TypeIdDto};

#[inline]
pub fn merge_opt<T: Clone>(a: &Option<T>, b: &Option<T>) -> Option<T> {
    b.clone().or_else(|| a.clone())
}

#[inline]
pub fn merge_opt_nested<T: Clone>(a: &Option<T>, b: &Option<T>, f: impl FnOnce(&T, &T) -> T) -> Option<T> {
    match (a.as_ref(), b.as_ref()) {
        (None, None) => None,
        (Some(x), None) => Some(x.clone()),
        (None, Some(y)) => Some(y.clone()),
        (Some(x), Some(y)) => Some(f(x, y)),
    }
}

/// Forward + backward sparse design deltas (replaces old `DesignChange`).
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DesignDiffPair {
    pub forward: DesignDiff,
    pub backward: DesignDiff,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<DesignFullDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<DesignFullDto>,
}

/// Back-compat name for [`DesignDiffPair`].
pub type DesignChange = DesignDiffPair;

// --- Attribute ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDiff {
    pub key: Option<String>,
    pub value: Option<String>,
    pub definition: Option<Option<String>>,
}

impl AttributeDiff {
    pub fn is_empty(&self) -> bool {
        self.key.is_none() && self.value.is_none() && self.definition.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self { key: merge_opt(&self.key, &b.key), value: merge_opt(&self.value, &b.value), definition: merge_opt_nested(&self.definition, &b.definition, |_x, y| y.clone()) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDiffUpdate {
    pub id: AttributeIdDto,
    #[serde(flatten)]
    pub diff: AttributeDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AttributesDiff {
    #[serde(default)]
    pub removed: Vec<AttributeIdDto>,
    #[serde(default)]
    pub updated: Vec<AttributeDiffUpdate>,
    #[serde(default)]
    pub added: Vec<AttributeFullDto>,
}

impl AttributesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    fn merge_updated(a: &[AttributeDiffUpdate], b: &[AttributeDiffUpdate]) -> Vec<AttributeDiffUpdate> {
        let mut m: HashMap<Id, AttributeDiffUpdate> = HashMap::new();
        for u in a {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in b {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| AttributeDiffUpdate { id: u.id.clone(), diff: AttributeDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        m.into_values().collect()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        Self { removed, updated: Self::merge_updated(&self.updated, &b.updated), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Prop ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PropDiff {
    pub key: Option<String>,
    pub value: Option<String>,
    pub unit: Option<Option<String>>,
}

impl PropDiff {
    pub fn is_empty(&self) -> bool {
        self.key.is_none() && self.value.is_none() && self.unit.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self { key: merge_opt(&self.key, &b.key), value: merge_opt(&self.value, &b.value), unit: merge_opt_nested(&self.unit, &b.unit, |_, y| y.clone()) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PropDiffUpdate {
    pub id: PropIdDto,
    #[serde(flatten)]
    pub diff: PropDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PropsDiff {
    #[serde(default)]
    pub removed: Vec<PropIdDto>,
    #[serde(default)]
    pub updated: Vec<PropDiffUpdate>,
    #[serde(default)]
    pub added: Vec<PropFullDto>,
}

impl PropsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, PropDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| PropDiffUpdate { id: u.id.clone(), diff: PropDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Benchmark ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkDiff {
    pub name: Option<String>,
    pub min: Option<Option<f64>>,
    pub max: Option<Option<f64>>,
    #[serde(rename = "minExcluded", default)]
    pub min_excluded: Option<Option<bool>>,
    #[serde(rename = "maxExcluded", default)]
    pub max_excluded: Option<Option<bool>>,
}

impl BenchmarkDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.min.is_none() && self.max.is_none() && self.min_excluded.is_none() && self.max_excluded.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            min: merge_opt_nested(&self.min, &b.min, |_, y| y.clone()),
            max: merge_opt_nested(&self.max, &b.max, |_, y| y.clone()),
            min_excluded: merge_opt_nested(&self.min_excluded, &b.min_excluded, |_, y| y.clone()),
            max_excluded: merge_opt_nested(&self.max_excluded, &b.max_excluded, |_, y| y.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkDiffUpdate {
    pub id: BenchmarkIdDto,
    #[serde(flatten)]
    pub diff: BenchmarkDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarksDiff {
    #[serde(default)]
    pub removed: Vec<BenchmarkIdDto>,
    #[serde(default)]
    pub updated: Vec<BenchmarkDiffUpdate>,
    #[serde(default)]
    pub added: Vec<BenchmarkFullDto>,
}

impl BenchmarksDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, BenchmarkDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| BenchmarkDiffUpdate { id: u.id.clone(), diff: BenchmarkDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Quality ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QualityDiff {
    pub key: Option<String>,
    pub value: Option<Option<String>>,
    pub unit: Option<Option<String>>,
    pub definition: Option<Option<String>>,
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub benchmarks: Option<BenchmarksDiff>,
}

impl QualityDiff {
    pub fn is_empty(&self) -> bool {
        self.key.is_none() && self.value.is_none() && self.unit.is_none() && self.definition.is_none() && self.description.is_none() && self.benchmarks.as_ref().map_or(true, |b| b.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            key: merge_opt(&self.key, &b.key),
            value: merge_opt_nested(&self.value, &b.value, |_, y| y.clone()),
            unit: merge_opt_nested(&self.unit, &b.unit, |_, y| y.clone()),
            definition: merge_opt_nested(&self.definition, &b.definition, |_, y| y.clone()),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            benchmarks: merge_opt_nested(&self.benchmarks, &b.benchmarks, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QualityDiffUpdate {
    pub id: QualityIdDto,
    #[serde(flatten)]
    pub diff: QualityDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QualitiesDiff {
    #[serde(default)]
    pub removed: Vec<QualityIdDto>,
    #[serde(default)]
    pub updated: Vec<QualityDiffUpdate>,
    #[serde(default)]
    pub added: Vec<QualityFullDto>,
}

impl QualitiesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, QualityDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| QualityDiffUpdate { id: u.id.clone(), diff: QualityDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Author / Concept / Tag (design & type scoped copies) ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDiff {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Option<String>>,
    pub rank: Option<Option<i64>>,
}

impl AuthorDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.email.is_none() && self.role.is_none() && self.rank.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            email: merge_opt(&self.email, &b.email),
            role: merge_opt_nested(&self.role, &b.role, |_, y| y.clone()),
            rank: merge_opt_nested(&self.rank, &b.rank, |_, y| y.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDiffUpdate {
    pub id: AuthorIdDto,
    #[serde(flatten)]
    pub diff: AuthorDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorsDiff {
    #[serde(default)]
    pub removed: Vec<AuthorIdDto>,
    #[serde(default)]
    pub updated: Vec<AuthorDiffUpdate>,
    #[serde(default)]
    pub added: Vec<AuthorFullDto>,
}

impl AuthorsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, AuthorDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| AuthorDiffUpdate { id: u.id.clone(), diff: AuthorDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDiff {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub order: Option<Option<i64>>,
}

impl ConceptDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.description.is_none() && self.order.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            order: merge_opt_nested(&self.order, &b.order, |_, y| y.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDiffUpdate {
    pub id: ConceptIdDto,
    #[serde(flatten)]
    pub diff: ConceptDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConceptsDiff {
    #[serde(default)]
    pub removed: Vec<ConceptIdDto>,
    #[serde(default)]
    pub updated: Vec<ConceptDiffUpdate>,
    #[serde(default)]
    pub added: Vec<ConceptFullDto>,
}

impl ConceptsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, ConceptDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| ConceptDiffUpdate { id: u.id.clone(), diff: ConceptDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TagDiff {
    pub name: Option<String>,
    pub order: Option<Option<i64>>,
}

impl TagDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.order.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self { name: merge_opt(&self.name, &b.name), order: merge_opt_nested(&self.order, &b.order, |_, y| y.clone()) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TagDiffUpdate {
    pub id: TagIdDto,
    #[serde(flatten)]
    pub diff: TagDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TagsDiff {
    #[serde(default)]
    pub removed: Vec<TagIdDto>,
    #[serde(default)]
    pub updated: Vec<TagDiffUpdate>,
    #[serde(default)]
    pub added: Vec<TagFullDto>,
}

impl TagsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, TagDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| TagDiffUpdate { id: u.id.clone(), diff: TagDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Stat ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatDiff {
    pub key: Option<String>,
    pub value: Option<String>,
    pub unit: Option<Option<String>>,
    pub description: Option<Option<String>>,
}

impl StatDiff {
    pub fn is_empty(&self) -> bool {
        self.key.is_none() && self.value.is_none() && self.unit.is_none() && self.description.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            key: merge_opt(&self.key, &b.key),
            value: merge_opt(&self.value, &b.value),
            unit: merge_opt_nested(&self.unit, &b.unit, |_, y| y.clone()),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatDiffUpdate {
    pub id: StatIdDto,
    #[serde(flatten)]
    pub diff: StatDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatsDiff {
    #[serde(default)]
    pub removed: Vec<StatIdDto>,
    #[serde(default)]
    pub updated: Vec<StatDiffUpdate>,
    #[serde(default)]
    pub added: Vec<StatFullDto>,
}

impl StatsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, StatDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| StatDiffUpdate { id: u.id.clone(), diff: StatDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Layer / Group ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayerDiff {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub color: Option<Option<String>>,
    pub order: Option<Option<i64>>,
    pub visible: Option<Option<bool>>,
    pub locked: Option<Option<bool>>,
}

impl LayerDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.description.is_none() && self.color.is_none() && self.order.is_none() && self.visible.is_none() && self.locked.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            color: merge_opt_nested(&self.color, &b.color, |_, y| y.clone()),
            order: merge_opt_nested(&self.order, &b.order, |_, y| y.clone()),
            visible: merge_opt_nested(&self.visible, &b.visible, |_, y| y.clone()),
            locked: merge_opt_nested(&self.locked, &b.locked, |_, y| y.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayerDiffUpdate {
    pub id: LayerIdDto,
    #[serde(flatten)]
    pub diff: LayerDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LayersDiff {
    #[serde(default)]
    pub removed: Vec<LayerIdDto>,
    #[serde(default)]
    pub updated: Vec<LayerDiffUpdate>,
    #[serde(default)]
    pub added: Vec<LayerFullDto>,
}

impl LayersDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, LayerDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| LayerDiffUpdate { id: u.id.clone(), diff: LayerDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupDiff {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub color: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    #[serde(default)]
    pub pieces: Option<Vec<PieceIdDto>>,
}

impl GroupDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && self.description.is_none() && self.color.is_none() && self.icon.is_none() && self.pieces.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            color: merge_opt_nested(&self.color, &b.color, |_, y| y.clone()),
            icon: merge_opt_nested(&self.icon, &b.icon, |_, y| y.clone()),
            pieces: merge_opt(&self.pieces, &b.pieces),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupDiffUpdate {
    pub id: GroupIdDto,
    #[serde(flatten)]
    pub diff: GroupDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GroupsDiff {
    #[serde(default)]
    pub removed: Vec<GroupIdDto>,
    #[serde(default)]
    pub updated: Vec<GroupDiffUpdate>,
    #[serde(default)]
    pub added: Vec<GroupFullDto>,
}

impl GroupsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, GroupDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| GroupDiffUpdate { id: u.id.clone(), diff: GroupDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Piece ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PieceDiff {
    pub name: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub plane: Option<Option<Plane>>,
    pub center: Option<Option<Coordinate>>,
    pub scale: Option<Option<f64>>,
    #[serde(rename = "mirrorPlane", default)]
    pub mirror_plane: Option<Option<Plane>>,
    pub hidden: Option<Option<bool>>,
    pub locked: Option<Option<bool>>,
    pub color: Option<Option<String>>,
    #[serde(rename = "type", default)]
    pub r#type: Option<Option<TypeIdDto>>,
    pub design: Option<Option<crate::design::DesignIdDto>>,
    #[serde(default)]
    pub props: Option<PropsDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl PieceDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.plane.is_none()
            && self.center.is_none()
            && self.scale.is_none()
            && self.mirror_plane.is_none()
            && self.hidden.is_none()
            && self.locked.is_none()
            && self.color.is_none()
            && self.r#type.is_none()
            && self.design.is_none()
            && self.props.as_ref().map_or(true, |p| p.is_empty())
            && self.attributes.as_ref().map_or(true, |a| a.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt_nested(&self.name, &b.name, |_, y| y.clone()),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            plane: merge_opt_nested(&self.plane, &b.plane, |_, y| y.clone()),
            center: merge_opt_nested(&self.center, &b.center, |_, y| y.clone()),
            scale: merge_opt_nested(&self.scale, &b.scale, |_, y| y.clone()),
            mirror_plane: merge_opt_nested(&self.mirror_plane, &b.mirror_plane, |_, y| y.clone()),
            hidden: merge_opt_nested(&self.hidden, &b.hidden, |_, y| y.clone()),
            locked: merge_opt_nested(&self.locked, &b.locked, |_, y| y.clone()),
            color: merge_opt_nested(&self.color, &b.color, |_, y| y.clone()),
            r#type: merge_opt_nested(&self.r#type, &b.r#type, |_, y| y.clone()),
            design: merge_opt_nested(&self.design, &b.design, |_, y| y.clone()),
            props: merge_opt_nested(&self.props, &b.props, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PieceDiffUpdate {
    pub id: PieceIdDto,
    #[serde(flatten)]
    pub diff: PieceDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PiecesDiff {
    #[serde(default)]
    pub removed: Vec<PieceIdDto>,
    #[serde(default)]
    pub updated: Vec<PieceDiffUpdate>,
    #[serde(default)]
    pub added: Vec<PieceFullDto>,
}

impl PiecesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, PieceDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| PieceDiffUpdate { id: u.id.clone(), diff: PieceDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Connection ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDiff {
    pub connected: Option<SideMetadataDto>,
    pub connecting: Option<SideMetadataDto>,
    pub gap: Option<Option<f64>>,
    pub shift: Option<Option<f64>>,
    pub rise: Option<Option<f64>>,
    pub rotation: Option<Option<f64>>,
    pub turn: Option<Option<f64>>,
    pub tilt: Option<Option<f64>>,
    pub x: Option<Option<f64>>,
    pub y: Option<Option<f64>>,
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl ConnectionDiff {
    pub fn is_empty(&self) -> bool {
        self.connected.is_none()
            && self.connecting.is_none()
            && self.gap.is_none()
            && self.shift.is_none()
            && self.rise.is_none()
            && self.rotation.is_none()
            && self.turn.is_none()
            && self.tilt.is_none()
            && self.x.is_none()
            && self.y.is_none()
            && self.description.is_none()
            && self.attributes.as_ref().map_or(true, |a| a.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            connected: merge_opt(&self.connected, &b.connected),
            connecting: merge_opt(&self.connecting, &b.connecting),
            gap: merge_opt_nested(&self.gap, &b.gap, |_, y| y.clone()),
            shift: merge_opt_nested(&self.shift, &b.shift, |_, y| y.clone()),
            rise: merge_opt_nested(&self.rise, &b.rise, |_, y| y.clone()),
            rotation: merge_opt_nested(&self.rotation, &b.rotation, |_, y| y.clone()),
            turn: merge_opt_nested(&self.turn, &b.turn, |_, y| y.clone()),
            tilt: merge_opt_nested(&self.tilt, &b.tilt, |_, y| y.clone()),
            x: merge_opt_nested(&self.x, &b.x, |_, y| y.clone()),
            y: merge_opt_nested(&self.y, &b.y, |_, y| y.clone()),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDiffUpdate {
    pub id: ConnectionIdDto,
    #[serde(flatten)]
    pub diff: ConnectionDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsDiff {
    #[serde(default)]
    pub removed: Vec<ConnectionIdDto>,
    #[serde(default)]
    pub updated: Vec<ConnectionDiffUpdate>,
    #[serde(default)]
    pub added: Vec<ConnectionFullDto>,
}

impl ConnectionsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, ConnectionDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| ConnectionDiffUpdate { id: u.id.clone(), diff: ConnectionDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Port / Connector / Representation ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PortDiff {
    #[serde(default)]
    pub id: Option<Id>,
    pub family: Option<Option<String>>,
    #[serde(rename = "compatibleFamilies", default)]
    pub compatible_families: Option<Vec<String>>,
    pub mandatory: Option<Option<bool>>,
    pub t: Option<Option<f64>>,
    pub description: Option<Option<String>>,
    pub point: Option<Option<Coordinate>>,
    pub direction: Option<Option<Vector>>,
    #[serde(default)]
    pub qualities: Option<QualitiesDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl PortDiff {
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.family.is_none()
            && self.compatible_families.is_none()
            && self.mandatory.is_none()
            && self.t.is_none()
            && self.description.is_none()
            && self.point.is_none()
            && self.direction.is_none()
            && self.qualities.as_ref().map_or(true, |q| q.is_empty())
            && self.attributes.as_ref().map_or(true, |a| a.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            id: merge_opt(&self.id, &b.id),
            family: merge_opt_nested(&self.family, &b.family, |_, y| y.clone()),
            compatible_families: merge_opt(&self.compatible_families, &b.compatible_families),
            mandatory: merge_opt_nested(&self.mandatory, &b.mandatory, |_, y| y.clone()),
            t: merge_opt_nested(&self.t, &b.t, |_, y| y.clone()),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            point: merge_opt_nested(&self.point, &b.point, |_, y| y.clone()),
            direction: merge_opt_nested(&self.direction, &b.direction, |_, y| y.clone()),
            qualities: merge_opt_nested(&self.qualities, &b.qualities, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PortDiffUpdate {
    pub id: PortIdDto,
    #[serde(flatten)]
    pub diff: PortDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PortsDiff {
    #[serde(default)]
    pub removed: Vec<PortIdDto>,
    #[serde(default)]
    pub updated: Vec<PortDiffUpdate>,
    #[serde(default)]
    pub added: Vec<PortFullDto>,
}

impl PortsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, PortDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| PortDiffUpdate { id: u.id.clone(), diff: PortDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorDiff {
    pub code: Option<String>,
    pub description: Option<Option<String>>,
    pub port: Option<Option<PortIdDto>>,
    #[serde(default)]
    pub qualities: Option<QualitiesDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl ConnectorDiff {
    pub fn is_empty(&self) -> bool {
        self.code.is_none() && self.description.is_none() && self.port.is_none() && self.qualities.as_ref().map_or(true, |q| q.is_empty()) && self.attributes.as_ref().map_or(true, |a| a.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            code: merge_opt(&self.code, &b.code),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            port: merge_opt_nested(&self.port, &b.port, |_, y| y.clone()),
            qualities: merge_opt_nested(&self.qualities, &b.qualities, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorDiffUpdate {
    pub id: ConnectorIdDto,
    #[serde(flatten)]
    pub diff: ConnectorDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorsDiff {
    #[serde(default)]
    pub removed: Vec<ConnectorIdDto>,
    #[serde(default)]
    pub updated: Vec<ConnectorDiffUpdate>,
    #[serde(default)]
    pub added: Vec<ConnectorFullDto>,
}

impl ConnectorsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, ConnectorDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| ConnectorDiffUpdate { id: u.id.clone(), diff: ConnectorDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepresentationDiff {
    pub url: Option<String>,
    pub description: Option<Option<String>>,
    pub file: Option<Option<FileIdDto>>,
    #[serde(default)]
    pub tags: Option<TagsDiff>,
    #[serde(default)]
    pub qualities: Option<QualitiesDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl RepresentationDiff {
    pub fn is_empty(&self) -> bool {
        self.url.is_none()
            && self.description.is_none()
            && self.file.is_none()
            && self.tags.as_ref().map_or(true, |t| t.is_empty())
            && self.qualities.as_ref().map_or(true, |q| q.is_empty())
            && self.attributes.as_ref().map_or(true, |a| a.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            url: merge_opt(&self.url, &b.url),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            file: merge_opt_nested(&self.file, &b.file, |_, y| y.clone()),
            tags: merge_opt_nested(&self.tags, &b.tags, |x, y| x.merge(y)),
            qualities: merge_opt_nested(&self.qualities, &b.qualities, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepresentationDiffUpdate {
    pub id: RepresentationIdDto,
    #[serde(flatten)]
    pub diff: RepresentationDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepresentationsDiff {
    #[serde(default)]
    pub removed: Vec<RepresentationIdDto>,
    #[serde(default)]
    pub updated: Vec<RepresentationDiffUpdate>,
    #[serde(default)]
    pub added: Vec<RepresentationFullDto>,
}

impl RepresentationsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, RepresentationDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| RepresentationDiffUpdate { id: u.id.clone(), diff: RepresentationDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Type ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeDiff {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    pub image: Option<Option<String>>,
    pub variant: Option<Option<String>>,
    pub stock: Option<Option<i64>>,
    #[serde(rename = "typeVirtual", default)]
    pub type_virtual: Option<Option<bool>>,
    pub unit: Option<Option<String>>,
    pub location: Option<Option<Location>>,
    pub created: Option<Option<String>>,
    pub updated: Option<Option<String>>,
    #[serde(default)]
    pub ports: Option<PortsDiff>,
    #[serde(default)]
    pub connectors: Option<ConnectorsDiff>,
    #[serde(default)]
    pub representations: Option<RepresentationsDiff>,
    #[serde(default)]
    pub authors: Option<AuthorsDiff>,
    #[serde(default)]
    pub concepts: Option<ConceptsDiff>,
    #[serde(default)]
    pub tags: Option<TagsDiff>,
    #[serde(default)]
    pub qualities: Option<QualitiesDiff>,
    #[serde(default)]
    pub props: Option<PropsDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
}

impl TypeDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.icon.is_none()
            && self.image.is_none()
            && self.variant.is_none()
            && self.stock.is_none()
            && self.type_virtual.is_none()
            && self.unit.is_none()
            && self.location.is_none()
            && self.created.is_none()
            && self.updated.is_none()
            && self.ports.as_ref().map_or(true, |x| x.is_empty())
            && self.connectors.as_ref().map_or(true, |x| x.is_empty())
            && self.representations.as_ref().map_or(true, |x| x.is_empty())
            && self.authors.as_ref().map_or(true, |x| x.is_empty())
            && self.concepts.as_ref().map_or(true, |x| x.is_empty())
            && self.tags.as_ref().map_or(true, |x| x.is_empty())
            && self.qualities.as_ref().map_or(true, |x| x.is_empty())
            && self.props.as_ref().map_or(true, |x| x.is_empty())
            && self.attributes.as_ref().map_or(true, |x| x.is_empty())
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            icon: merge_opt_nested(&self.icon, &b.icon, |_, y| y.clone()),
            image: merge_opt_nested(&self.image, &b.image, |_, y| y.clone()),
            variant: merge_opt_nested(&self.variant, &b.variant, |_, y| y.clone()),
            stock: merge_opt_nested(&self.stock, &b.stock, |_, y| y.clone()),
            type_virtual: merge_opt_nested(&self.type_virtual, &b.type_virtual, |_, y| y.clone()),
            unit: merge_opt_nested(&self.unit, &b.unit, |_, y| y.clone()),
            location: merge_opt_nested(&self.location, &b.location, |_, y| y.clone()),
            created: merge_opt_nested(&self.created, &b.created, |_, y| y.clone()),
            updated: merge_opt_nested(&self.updated, &b.updated, |_, y| y.clone()),
            ports: merge_opt_nested(&self.ports, &b.ports, |x, y| x.merge(y)),
            connectors: merge_opt_nested(&self.connectors, &b.connectors, |x, y| x.merge(y)),
            representations: merge_opt_nested(&self.representations, &b.representations, |x, y| x.merge(y)),
            authors: merge_opt_nested(&self.authors, &b.authors, |x, y| x.merge(y)),
            concepts: merge_opt_nested(&self.concepts, &b.concepts, |x, y| x.merge(y)),
            tags: merge_opt_nested(&self.tags, &b.tags, |x, y| x.merge(y)),
            qualities: merge_opt_nested(&self.qualities, &b.qualities, |x, y| x.merge(y)),
            props: merge_opt_nested(&self.props, &b.props, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypeDiffUpdate {
    pub id: TypeIdDto,
    #[serde(flatten)]
    pub diff: TypeDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypesDiff {
    #[serde(default)]
    pub removed: Vec<TypeIdDto>,
    #[serde(default)]
    pub updated: Vec<TypeDiffUpdate>,
    #[serde(default)]
    pub added: Vec<TypeFullDto>,
}

impl TypesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, TypeDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| TypeDiffUpdate { id: u.id.clone(), diff: TypeDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- File / Folder (kit scoped) ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileDiff {
    pub url: Option<String>,
    pub mime: Option<Option<String>>,
    pub size: Option<Option<i64>>,
    pub hash: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub created: Option<Option<String>>,
    pub updated: Option<Option<String>>,
}

impl FileDiff {
    pub fn is_empty(&self) -> bool {
        self.url.is_none() && self.mime.is_none() && self.size.is_none() && self.hash.is_none() && self.description.is_none() && self.created.is_none() && self.updated.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self {
            url: merge_opt(&self.url, &b.url),
            mime: merge_opt_nested(&self.mime, &b.mime, |_, y| y.clone()),
            size: merge_opt_nested(&self.size, &b.size, |_, y| y.clone()),
            hash: merge_opt_nested(&self.hash, &b.hash, |_, y| y.clone()),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            created: merge_opt_nested(&self.created, &b.created, |_, y| y.clone()),
            updated: merge_opt_nested(&self.updated, &b.updated, |_, y| y.clone()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileDiffUpdate {
    pub id: FileIdDto,
    #[serde(flatten)]
    pub diff: FileDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FilesDiff {
    #[serde(default)]
    pub removed: Vec<FileIdDto>,
    #[serde(default)]
    pub updated: Vec<FileDiffUpdate>,
    #[serde(default)]
    pub added: Vec<FileFullDto>,
}

impl FilesDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, FileDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| FileDiffUpdate { id: u.id.clone(), diff: FileDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FolderDiff {
    pub path: Option<String>,
    pub description: Option<Option<String>>,
}

impl FolderDiff {
    pub fn is_empty(&self) -> bool {
        self.path.is_none() && self.description.is_none()
    }
    pub fn merge(&self, b: &Self) -> Self {
        Self { path: merge_opt(&self.path, &b.path), description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FolderDiffUpdate {
    pub id: FolderIdDto,
    #[serde(flatten)]
    pub diff: FolderDiff,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FoldersDiff {
    #[serde(default)]
    pub removed: Vec<FolderIdDto>,
    #[serde(default)]
    pub updated: Vec<FolderDiffUpdate>,
    #[serde(default)]
    pub added: Vec<FolderFullDto>,
}

impl FoldersDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.updated.is_empty() && self.added.is_empty()
    }
    pub fn merge(&self, b: &Self) -> Self {
        let mut removed: Vec<_> = self.removed.iter().chain(b.removed.iter()).cloned().collect();
        removed.sort_by(|x, y| x.id.cmp(&y.id));
        removed.dedup_by(|x, y| x.id == y.id);
        let mut m: HashMap<Id, FolderDiffUpdate> = HashMap::new();
        for u in &self.updated {
            m.insert(u.id.id.clone(), u.clone());
        }
        for u in &b.updated {
            let e = m.entry(u.id.id.clone()).or_insert_with(|| FolderDiffUpdate { id: u.id.clone(), diff: FolderDiff::default() });
            e.diff = e.diff.merge(&u.diff);
        }
        Self { removed, updated: m.into_values().collect(), added: [self.added.as_slice(), b.added.as_slice()].concat() }
    }
}

// --- Design (scoped diff, no design id) ---
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DesignDiff {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    pub image: Option<Option<String>>,
    pub variant: Option<Option<String>>,
    pub view: Option<Option<String>>,
    pub location: Option<Option<Location>>,
    pub camera: Option<Option<Camera>>,
    pub unit: Option<Option<String>>,
    pub created: Option<Option<String>>,
    pub updated: Option<Option<String>>,
    pub kit: Option<Option<KitIdDto>>,
    #[serde(default)]
    pub pieces: Option<PiecesDiff>,
    #[serde(default)]
    pub connections: Option<ConnectionsDiff>,
    #[serde(default)]
    pub layers: Option<LayersDiff>,
    #[serde(default)]
    pub groups: Option<GroupsDiff>,
    #[serde(default)]
    pub authors: Option<AuthorsDiff>,
    #[serde(default)]
    pub concepts: Option<ConceptsDiff>,
    #[serde(default)]
    pub tags: Option<TagsDiff>,
    #[serde(default)]
    pub qualities: Option<QualitiesDiff>,
    #[serde(default)]
    pub props: Option<PropsDiff>,
    #[serde(default)]
    pub attributes: Option<AttributesDiff>,
    #[serde(default)]
    pub stats: Option<StatsDiff>,
}

impl DesignDiff {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.icon.is_none()
            && self.image.is_none()
            && self.variant.is_none()
            && self.view.is_none()
            && self.location.is_none()
            && self.camera.is_none()
            && self.unit.is_none()
            && self.created.is_none()
            && self.updated.is_none()
            && self.kit.is_none()
            && self.pieces.as_ref().map_or(true, |x| x.is_empty())
            && self.connections.as_ref().map_or(true, |x| x.is_empty())
            && self.layers.as_ref().map_or(true, |x| x.is_empty())
            && self.groups.as_ref().map_or(true, |x| x.is_empty())
            && self.authors.as_ref().map_or(true, |x| x.is_empty())
            && self.concepts.as_ref().map_or(true, |x| x.is_empty())
            && self.tags.as_ref().map_or(true, |x| x.is_empty())
            && self.qualities.as_ref().map_or(true, |x| x.is_empty())
            && self.props.as_ref().map_or(true, |x| x.is_empty())
            && self.attributes.as_ref().map_or(true, |x| x.is_empty())
            && self.stats.as_ref().map_or(true, |x| x.is_empty())
    }

    pub fn merge(&self, b: &Self) -> Self {
        Self {
            name: merge_opt(&self.name, &b.name),
            description: merge_opt_nested(&self.description, &b.description, |_, y| y.clone()),
            icon: merge_opt_nested(&self.icon, &b.icon, |_, y| y.clone()),
            image: merge_opt_nested(&self.image, &b.image, |_, y| y.clone()),
            variant: merge_opt_nested(&self.variant, &b.variant, |_, y| y.clone()),
            view: merge_opt_nested(&self.view, &b.view, |_, y| y.clone()),
            location: merge_opt_nested(&self.location, &b.location, |_, y| y.clone()),
            camera: merge_opt_nested(&self.camera, &b.camera, |_, y| y.clone()),
            unit: merge_opt_nested(&self.unit, &b.unit, |_, y| y.clone()),
            created: merge_opt_nested(&self.created, &b.created, |_, y| y.clone()),
            updated: merge_opt_nested(&self.updated, &b.updated, |_, y| y.clone()),
            kit: merge_opt_nested(&self.kit, &b.kit, |_, y| y.clone()),
            pieces: merge_opt_nested(&self.pieces, &b.pieces, |x, y| x.merge(y)),
            connections: merge_opt_nested(&self.connections, &b.connections, |x, y| x.merge(y)),
            layers: merge_opt_nested(&self.layers, &b.layers, |x, y| x.merge(y)),
            groups: merge_opt_nested(&self.groups, &b.groups, |x, y| x.merge(y)),
            authors: merge_opt_nested(&self.authors, &b.authors, |x, y| x.merge(y)),
            concepts: merge_opt_nested(&self.concepts, &b.concepts, |x, y| x.merge(y)),
            tags: merge_opt_nested(&self.tags, &b.tags, |x, y| x.merge(y)),
            qualities: merge_opt_nested(&self.qualities, &b.qualities, |x, y| x.merge(y)),
            props: merge_opt_nested(&self.props, &b.props, |x, y| x.merge(y)),
            attributes: merge_opt_nested(&self.attributes, &b.attributes, |x, y| x.merge(y)),
            stats: merge_opt_nested(&self.stats, &b.stats, |x, y| x.merge(y)),
        }
    }

    /// DTO-level delta (pieces/connections and design metadata) for tests / tooling.
    pub fn between(before: &DesignFullDto, after: &DesignFullDto) -> Self {
        Self::between_dto(before, after)
    }

    /// DTO-level delta (pieces/connections and design metadata) for tests / tooling.
    pub fn between_dto(before: &DesignFullDto, after: &DesignFullDto) -> Self {
        let mut d = DesignDiff::default();
        if before.name != after.name {
            d.name = Some(after.name.clone());
        }
        if before.description != after.description {
            d.description = Some(after.description.clone());
        }
        if before.icon != after.icon {
            d.icon = Some(after.icon.clone());
        }
        if before.image != after.image {
            d.image = Some(after.image.clone());
        }
        if before.variant != after.variant {
            d.variant = Some(after.variant.clone());
        }
        if before.view != after.view {
            d.view = Some(after.view.clone());
        }
        if before.location != after.location {
            d.location = Some(after.location);
        }
        if before.camera != after.camera {
            d.camera = Some(after.camera);
        }
        if before.unit != after.unit {
            d.unit = Some(after.unit.clone());
        }
        if before.created != after.created {
            d.created = Some(after.created.clone());
        }
        if before.updated != after.updated {
            d.updated = Some(after.updated.clone());
        }
        if before.kit != after.kit {
            d.kit = Some(after.kit.clone());
        }
        let bp: HashMap<Id, &PieceFullDto> = before.pieces.iter().map(|p| (p.id.clone(), p)).collect();
        let ap: HashMap<Id, &PieceFullDto> = after.pieces.iter().map(|p| (p.id.clone(), p)).collect();
        let kb: std::collections::HashSet<Id> = bp.keys().cloned().collect();
        let ka: std::collections::HashSet<Id> = ap.keys().cloned().collect();
        let mut pd = PiecesDiff::default();
        for g in kb.difference(&ka) {
            pd.removed.push(PieceIdDto { id: g.clone() });
        }
        for g in ka.difference(&kb) {
            pd.added.push((*ap[g]).clone());
        }
        for g in ka.intersection(&kb) {
            if bp[g] != ap[g] {
                pd.updated.push(PieceDiffUpdate { id: PieceIdDto { id: g.clone() }, diff: piece_full_delta(bp[g], ap[g]) });
            }
        }
        if !pd.is_empty() {
            d.pieces = Some(pd);
        }
        let bc: HashMap<Id, &ConnectionFullDto> = before.connections.iter().map(|c| (c.id.clone(), c)).collect();
        let ac: HashMap<Id, &ConnectionFullDto> = after.connections.iter().map(|c| (c.id.clone(), c)).collect();
        let kbc: std::collections::HashSet<Id> = bc.keys().cloned().collect();
        let kac: std::collections::HashSet<Id> = ac.keys().cloned().collect();
        let mut cd = ConnectionsDiff::default();
        for g in kbc.difference(&kac) {
            cd.removed.push(ConnectionIdDto { id: g.clone() });
        }
        for g in kac.difference(&kbc) {
            cd.added.push((*ac[g]).clone());
        }
        for g in kac.intersection(&kbc) {
            if bc[g] != ac[g] {
                cd.updated.push(ConnectionDiffUpdate { id: ConnectionIdDto { id: g.clone() }, diff: connection_full_delta(bc[g], ac[g]) });
            }
        }
        if !cd.is_empty() {
            d.connections = Some(cd);
        }
        d.layers = layers_between(&before.layers, &after.layers);
        d.groups = groups_between(&before.groups, &after.groups);
        d.authors = authors_between(&before.authors, &after.authors);
        d.concepts = concepts_between(&before.concepts, &after.concepts);
        d.tags = tags_between(&before.tags, &after.tags);
        d.qualities = qualities_between(&before.qualities, &after.qualities);
        let pr = props_between(&before.props, &after.props);
        if !pr.is_empty() {
            d.props = Some(pr);
        }
        let at = attributes_between(&before.attributes, &after.attributes);
        if !at.is_empty() {
            d.attributes = Some(at);
        }
        d.stats = stats_between(&before.stats, &after.stats);
        d
    }
}

fn piece_full_delta(b: &PieceFullDto, a: &PieceFullDto) -> PieceDiff {
    let mut d = PieceDiff::default();
    if b.name != a.name {
        d.name = Some(a.name.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.plane != a.plane {
        d.plane = Some(a.plane);
    }
    if b.center != a.center {
        d.center = Some(a.center);
    }
    if b.scale != a.scale {
        d.scale = Some(a.scale);
    }
    if b.mirror_plane != a.mirror_plane {
        d.mirror_plane = Some(a.mirror_plane);
    }
    if b.hidden != a.hidden {
        d.hidden = Some(a.hidden);
    }
    if b.locked != a.locked {
        d.locked = Some(a.locked);
    }
    if b.color != a.color {
        d.color = Some(a.color.clone());
    }
    if b.r#type != a.r#type {
        d.r#type = Some(a.r#type.clone());
    }
    if b.design != a.design {
        d.design = Some(a.design.clone());
    }
    let pp = props_between(&b.props, &a.props);
    if !pp.is_empty() {
        d.props = Some(pp);
    }
    let at = attributes_between(&b.attributes, &a.attributes);
    if !at.is_empty() {
        d.attributes = Some(at);
    }
    d
}

fn connection_full_delta(b: &ConnectionFullDto, a: &ConnectionFullDto) -> ConnectionDiff {
    let mut d = ConnectionDiff::default();
    if b.connected != a.connected {
        d.connected = Some(a.connected.clone());
    }
    if b.connecting != a.connecting {
        d.connecting = Some(a.connecting.clone());
    }
    if b.gap != a.gap {
        d.gap = Some(a.gap);
    }
    if b.shift != a.shift {
        d.shift = Some(a.shift);
    }
    if b.rise != a.rise {
        d.rise = Some(a.rise);
    }
    if b.rotation != a.rotation {
        d.rotation = Some(a.rotation);
    }
    if b.turn != a.turn {
        d.turn = Some(a.turn);
    }
    if b.tilt != a.tilt {
        d.tilt = Some(a.tilt);
    }
    if b.x != a.x {
        d.x = Some(a.x);
    }
    if b.y != a.y {
        d.y = Some(a.y);
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    let at = attributes_between(&b.attributes, &a.attributes);
    if !at.is_empty() {
        d.attributes = Some(at);
    }
    d
}

fn layers_between(before: &[LayerFullDto], after: &[LayerFullDto]) -> Option<LayersDiff> {
    let mut d = LayersDiff::default();
    let bm: HashMap<Id, &LayerFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &LayerFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(LayerIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = LayerDiff::default();
            if b.name != a.name {
                df.name = Some(a.name.clone());
            }
            if b.description != a.description {
                df.description = Some(a.description.clone());
            }
            if b.color != a.color {
                df.color = Some(a.color.clone());
            }
            if b.order != a.order {
                df.order = Some(a.order);
            }
            if b.visible != a.visible {
                df.visible = Some(a.visible);
            }
            if b.locked != a.locked {
                df.locked = Some(a.locked);
            }
            d.updated.push(LayerDiffUpdate { id: LayerIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

fn groups_between(before: &[GroupFullDto], after: &[GroupFullDto]) -> Option<GroupsDiff> {
    let mut d = GroupsDiff::default();
    let bm: HashMap<Id, &GroupFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &GroupFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(GroupIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = GroupDiff::default();
            if b.name != a.name {
                df.name = Some(a.name.clone());
            }
            if b.description != a.description {
                df.description = Some(a.description.clone());
            }
            if b.color != a.color {
                df.color = Some(a.color.clone());
            }
            if b.icon != a.icon {
                df.icon = Some(a.icon.clone());
            }
            if b.pieces != a.pieces {
                df.pieces = Some(a.pieces.clone());
            }
            d.updated.push(GroupDiffUpdate { id: GroupIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

fn authors_between(before: &[AuthorFullDto], after: &[AuthorFullDto]) -> Option<AuthorsDiff> {
    let mut d = AuthorsDiff::default();
    let bm: HashMap<Id, &AuthorFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &AuthorFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(AuthorIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = AuthorDiff::default();
            if b.name != a.name {
                df.name = Some(a.name.clone());
            }
            if b.email != a.email {
                df.email = Some(a.email.clone());
            }
            if b.role != a.role {
                df.role = Some(a.role.clone());
            }
            if b.rank != a.rank {
                df.rank = Some(a.rank);
            }
            d.updated.push(AuthorDiffUpdate { id: AuthorIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

fn concepts_between(before: &[ConceptFullDto], after: &[ConceptFullDto]) -> Option<ConceptsDiff> {
    let mut d = ConceptsDiff::default();
    let bm: HashMap<Id, &ConceptFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &ConceptFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(ConceptIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = ConceptDiff::default();
            if b.name != a.name {
                df.name = Some(a.name.clone());
            }
            if b.description != a.description {
                df.description = Some(a.description.clone());
            }
            if b.order != a.order {
                df.order = Some(a.order);
            }
            d.updated.push(ConceptDiffUpdate { id: ConceptIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

fn tags_between(before: &[TagFullDto], after: &[TagFullDto]) -> Option<TagsDiff> {
    let mut d = TagsDiff::default();
    let bm: HashMap<Id, &TagFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &TagFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(TagIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = TagDiff::default();
            if b.name != a.name {
                df.name = Some(a.name.clone());
            }
            if b.order != a.order {
                df.order = Some(a.order);
            }
            d.updated.push(TagDiffUpdate { id: TagIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

fn qualities_between(before: &[QualityFullDto], after: &[QualityFullDto]) -> Option<QualitiesDiff> {
    let mut d = QualitiesDiff::default();
    let bm: HashMap<Id, &QualityFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &QualityFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(QualityIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = QualityDiff::default();
            if b.key != a.key {
                df.key = Some(a.key.clone());
            }
            if b.value != a.value {
                df.value = Some(a.value.clone());
            }
            if b.unit != a.unit {
                df.unit = Some(a.unit.clone());
            }
            if b.definition != a.definition {
                df.definition = Some(a.definition.clone());
            }
            if b.description != a.description {
                df.description = Some(a.description.clone());
            }
            let bm = benchmarks_between(&b.benchmarks, &a.benchmarks);
            if !bm.is_empty() {
                df.benchmarks = Some(bm);
            }
            d.updated.push(QualityDiffUpdate { id: QualityIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

fn benchmarks_between(before: &[BenchmarkFullDto], after: &[BenchmarkFullDto]) -> BenchmarksDiff {
    let mut d = BenchmarksDiff::default();
    let bm: HashMap<Id, &BenchmarkFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &BenchmarkFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(BenchmarkIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = BenchmarkDiff::default();
            if b.name != a.name {
                df.name = Some(a.name.clone());
            }
            if b.min != a.min {
                df.min = Some(a.min);
            }
            if b.max != a.max {
                df.max = Some(a.max);
            }
            if b.min_excluded != a.min_excluded {
                df.min_excluded = Some(a.min_excluded);
            }
            if b.max_excluded != a.max_excluded {
                df.max_excluded = Some(a.max_excluded);
            }
            d.updated.push(BenchmarkDiffUpdate { id: BenchmarkIdDto { id: g.clone() }, diff: df });
        }
    }
    d
}

fn props_between(before: &[PropFullDto], after: &[PropFullDto]) -> PropsDiff {
    let mut d = PropsDiff::default();
    let bm: HashMap<Id, &PropFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &PropFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(PropIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = PropDiff::default();
            if b.key != a.key {
                df.key = Some(a.key.clone());
            }
            if b.value != a.value {
                df.value = Some(a.value.clone());
            }
            if b.unit != a.unit {
                df.unit = Some(a.unit.clone());
            }
            d.updated.push(PropDiffUpdate { id: PropIdDto { id: g.clone() }, diff: df });
        }
    }
    d
}

fn attributes_between(before: &[AttributeFullDto], after: &[AttributeFullDto]) -> AttributesDiff {
    let mut d = AttributesDiff::default();
    let bm: HashMap<Id, &AttributeFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &AttributeFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(AttributeIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = AttributeDiff::default();
            if b.key != a.key {
                df.key = Some(a.key.clone());
            }
            if b.value != a.value {
                df.value = Some(a.value.clone());
            }
            if b.definition != a.definition {
                df.definition = Some(a.definition.clone());
            }
            d.updated.push(AttributeDiffUpdate { id: AttributeIdDto { id: g.clone() }, diff: df });
        }
    }
    d
}

fn stats_between(before: &[StatFullDto], after: &[StatFullDto]) -> Option<StatsDiff> {
    let mut d = StatsDiff::default();
    let bm: HashMap<Id, &StatFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &StatFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(StatIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            let mut df = StatDiff::default();
            if b.key != a.key {
                df.key = Some(a.key.clone());
            }
            if b.value != a.value {
                df.value = Some(a.value.clone());
            }
            if b.unit != a.unit {
                df.unit = Some(a.unit.clone());
            }
            if b.description != a.description {
                df.description = Some(a.description.clone());
            }
            d.updated.push(StatDiffUpdate { id: StatIdDto { id: g.clone() }, diff: df });
        }
    }
    if d.is_empty() {
        None
    } else {
        Some(d)
    }
}

pub fn attribute_full_delta(b: &AttributeFullDto, a: &AttributeFullDto) -> AttributeDiff {
    let mut d = AttributeDiff::default();
    if b.key != a.key {
        d.key = Some(a.key.clone());
    }
    if b.value != a.value {
        d.value = Some(a.value.clone());
    }
    if b.definition != a.definition {
        d.definition = Some(a.definition.clone());
    }
    d
}

pub fn prop_full_delta(b: &PropFullDto, a: &PropFullDto) -> PropDiff {
    let mut d = PropDiff::default();
    if b.key != a.key {
        d.key = Some(a.key.clone());
    }
    if b.value != a.value {
        d.value = Some(a.value.clone());
    }
    if b.unit != a.unit {
        d.unit = Some(a.unit.clone());
    }
    d
}

pub fn author_full_delta(b: &AuthorFullDto, a: &AuthorFullDto) -> AuthorDiff {
    let mut d = AuthorDiff::default();
    if b.name != a.name {
        d.name = Some(a.name.clone());
    }
    if b.email != a.email {
        d.email = Some(a.email.clone());
    }
    if b.role != a.role {
        d.role = Some(a.role.clone());
    }
    if b.rank != a.rank {
        d.rank = Some(a.rank);
    }
    d
}

pub fn concept_full_delta(b: &ConceptFullDto, a: &ConceptFullDto) -> ConceptDiff {
    let mut d = ConceptDiff::default();
    if b.name != a.name {
        d.name = Some(a.name.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.order != a.order {
        d.order = Some(a.order);
    }
    d
}

pub fn tag_full_delta(b: &TagFullDto, a: &TagFullDto) -> TagDiff {
    let mut d = TagDiff::default();
    if b.name != a.name {
        d.name = Some(a.name.clone());
    }
    if b.order != a.order {
        d.order = Some(a.order);
    }
    d
}

pub fn quality_full_delta(b: &QualityFullDto, a: &QualityFullDto) -> QualityDiff {
    let mut d = QualityDiff::default();
    if b.key != a.key {
        d.key = Some(a.key.clone());
    }
    if b.value != a.value {
        d.value = Some(a.value.clone());
    }
    if b.unit != a.unit {
        d.unit = Some(a.unit.clone());
    }
    if b.definition != a.definition {
        d.definition = Some(a.definition.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    let bm = benchmarks_between(&b.benchmarks, &a.benchmarks);
    if !bm.is_empty() {
        d.benchmarks = Some(bm);
    }
    d
}

pub fn file_full_delta(b: &FileFullDto, a: &FileFullDto) -> FileDiff {
    let mut d = FileDiff::default();
    if b.url != a.url {
        d.url = Some(a.url.clone());
    }
    if b.mime != a.mime {
        d.mime = Some(a.mime.clone());
    }
    if b.size != a.size {
        d.size = Some(a.size);
    }
    if b.hash != a.hash {
        d.hash = Some(a.hash.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.created != a.created {
        d.created = Some(a.created.clone());
    }
    if b.updated != a.updated {
        d.updated = Some(a.updated.clone());
    }
    d
}

pub fn folder_full_delta(b: &FolderFullDto, a: &FolderFullDto) -> FolderDiff {
    let mut d = FolderDiff::default();
    if b.path != a.path {
        d.path = Some(a.path.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    d
}

pub fn type_full_delta(b: &TypeFullDto, a: &TypeFullDto) -> TypeDiff {
    let mut d = TypeDiff::default();
    if b.name != a.name {
        d.name = Some(a.name.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.icon != a.icon {
        d.icon = Some(a.icon.clone());
    }
    if b.image != a.image {
        d.image = Some(a.image.clone());
    }
    if b.variant != a.variant {
        d.variant = Some(a.variant.clone());
    }
    if b.stock != a.stock {
        d.stock = Some(a.stock);
    }
    if b.virtual_ != a.virtual_ {
        d.type_virtual = Some(a.virtual_);
    }
    if b.unit != a.unit {
        d.unit = Some(a.unit.clone());
    }
    if b.location != a.location {
        d.location = Some(a.location);
    }
    if b.created != a.created {
        d.created = Some(a.created.clone());
    }
    if b.updated != a.updated {
        d.updated = Some(a.updated.clone());
    }
    let pd = ports_between(&b.ports, &a.ports);
    if !pd.is_empty() {
        d.ports = Some(pd);
    }
    let cd = connectors_between(&b.connectors, &a.connectors);
    if !cd.is_empty() {
        d.connectors = Some(cd);
    }
    let rd = representations_between(&b.representations, &a.representations);
    if !rd.is_empty() {
        d.representations = Some(rd);
    }
    let ad = authors_between(&b.authors, &a.authors);
    if ad.is_some() {
        d.authors = ad;
    }
    let co = concepts_between(&b.concepts, &a.concepts);
    if co.is_some() {
        d.concepts = co;
    }
    let tg = tags_between(&b.tags, &a.tags);
    if tg.is_some() {
        d.tags = tg;
    }
    let ql = qualities_between(&b.qualities, &a.qualities);
    if ql.is_some() {
        d.qualities = ql;
    }
    let pr = props_between(&b.props, &a.props);
    if !pr.is_empty() {
        d.props = Some(pr);
    }
    let at = attributes_between(&b.attributes, &a.attributes);
    if !at.is_empty() {
        d.attributes = Some(at);
    }
    d
}

fn ports_between(before: &[PortFullDto], after: &[PortFullDto]) -> PortsDiff {
    let mut d = PortsDiff::default();
    let bm: HashMap<Id, &PortFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &PortFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(PortIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            d.updated.push(PortDiffUpdate { id: PortIdDto { id: g.clone() }, diff: port_full_delta(b, a) });
        }
    }
    d
}

fn port_full_delta(b: &PortFullDto, a: &PortFullDto) -> PortDiff {
    let mut d = PortDiff::default();
    if b.id != a.id {
        d.id = Some(a.id.clone());
    }
    if b.family != a.family {
        d.family = Some(a.family.clone());
    }
    if b.compatible_families != a.compatible_families {
        d.compatible_families = Some(a.compatible_families.clone());
    }
    if b.mandatory != a.mandatory {
        d.mandatory = Some(a.mandatory);
    }
    if b.t != a.t {
        d.t = Some(a.t);
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.point != a.point {
        d.point = Some(a.point);
    }
    if b.direction != a.direction {
        d.direction = Some(a.direction);
    }
    let ql = qualities_between(&b.qualities, &a.qualities);
    if ql.is_some() {
        d.qualities = ql;
    }
    let at = attributes_between(&b.attributes, &a.attributes);
    if !at.is_empty() {
        d.attributes = Some(at);
    }
    d
}

fn connectors_between(before: &[ConnectorFullDto], after: &[ConnectorFullDto]) -> ConnectorsDiff {
    let mut d = ConnectorsDiff::default();
    let bm: HashMap<Id, &ConnectorFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &ConnectorFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(ConnectorIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            d.updated.push(ConnectorDiffUpdate { id: ConnectorIdDto { id: g.clone() }, diff: connector_full_delta(b, a) });
        }
    }
    d
}

fn connector_full_delta(b: &ConnectorFullDto, a: &ConnectorFullDto) -> ConnectorDiff {
    let mut d = ConnectorDiff::default();
    if b.code != a.code {
        d.code = Some(a.code.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.port != a.port {
        d.port = Some(a.port.clone());
    }
    let ql = qualities_between(&b.qualities, &a.qualities);
    if ql.is_some() {
        d.qualities = ql;
    }
    let at = attributes_between(&b.attributes, &a.attributes);
    if !at.is_empty() {
        d.attributes = Some(at);
    }
    d
}

fn representations_between(before: &[RepresentationFullDto], after: &[RepresentationFullDto]) -> RepresentationsDiff {
    let mut d = RepresentationsDiff::default();
    let bm: HashMap<Id, &RepresentationFullDto> = before.iter().map(|x| (x.id.clone(), x)).collect();
    let am: HashMap<Id, &RepresentationFullDto> = after.iter().map(|x| (x.id.clone(), x)).collect();
    let kb: std::collections::HashSet<Id> = bm.keys().cloned().collect();
    let ka: std::collections::HashSet<Id> = am.keys().cloned().collect();
    for g in kb.difference(&ka) {
        d.removed.push(RepresentationIdDto { id: g.clone() });
    }
    for g in ka.difference(&kb) {
        d.added.push((*am[g]).clone());
    }
    for g in ka.intersection(&kb) {
        let b = bm[g];
        let a = am[g];
        if b != a {
            d.updated.push(RepresentationDiffUpdate { id: RepresentationIdDto { id: g.clone() }, diff: representation_full_delta(b, a) });
        }
    }
    d
}

fn representation_full_delta(b: &RepresentationFullDto, a: &RepresentationFullDto) -> RepresentationDiff {
    let mut d = RepresentationDiff::default();
    if b.url != a.url {
        d.url = Some(a.url.clone());
    }
    if b.description != a.description {
        d.description = Some(a.description.clone());
    }
    if b.file != a.file {
        d.file = Some(a.file.clone());
    }
    let tg = tags_between(&b.tags, &a.tags);
    if tg.is_some() {
        d.tags = tg;
    }
    let ql = qualities_between(&b.qualities, &a.qualities);
    if ql.is_some() {
        d.qualities = ql;
    }
    let at = attributes_between(&b.attributes, &a.attributes);
    if !at.is_empty() {
        d.attributes = Some(at);
    }
    d
}
