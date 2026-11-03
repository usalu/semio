//! 🧲️ Scene kit adapter: an artifact's existing projection modules become a [`StructuralProjection`] by implementing the small [`SceneKit`] contract; the same modules produce the whole scene ([`project_whole`]) and the per-entity fragments of the live scene, so incremental equals full by construction.

use super::*;
use crate::projection_kit::{base_id, Placement};
use crate::{Diagram2d, EntityRef, Handle, SceneRevision, StructuralScene, SCENE_SCHEMA_ID};
use norm::CheckReport;
use std::collections::{BTreeMap, BTreeSet};
use crate::OwnedKind;
use semio_framework_value::{DslValue, Number, ToValue};
use std::marker::PhantomData;
use std::sync::Arc;

/// 🔑️ What a view option selects; one variant per option any artifact reads, so a new option is a new variant and every consumer that matches on the key is a compile error until it decides.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewKey {
    /// 🧮️ The id of the active combination column.
    Combination,
    /// 🧮️ The position of the active combination or variant in the artifact's list (`None` follows the governing one).
    CombinationItem,
    /// 🚦️ The id of the chosen traffic group (`GR2`).
    TrafficGroup,
    /// 🚦️ The position of the chosen traffic group among the groups the document declares (`None` follows the active combination).
    TrafficGroupItem,
    /// 🎞️ The id of the selected parameter track (`fire`, `construction`, ...).
    Track,
    /// 🎞️ The value on the selected track, in the unit of the track (seconds for `fire`).
    TrackValue,
}

/// 🏷️ The typed value of one view option.
#[derive(Clone, Debug, PartialEq)]
pub enum ViewValue {
    Id(String),
    Index(u32),
    Number(f64),
}

/// 👁️ The typed view options of a projection: which combination, traffic group, track and track value the viewer looks at. A map keyed by [`ViewKey`], so it extends without a string codec; every accessor returns the option in its own type.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ViewOptions {
    values: BTreeMap<ViewKey, ViewValue>,
}

static NO_VIEW: ViewOptions = ViewOptions { values: BTreeMap::new() };

impl ViewOptions {
    /// 👁️ The options of a projection without any view choice.
    pub fn none() -> &'static ViewOptions {
        &NO_VIEW
    }

    /// ✍️ Sets `key` to `value`.
    pub fn with(mut self, key: ViewKey, value: ViewValue) -> Self {
        self.values.insert(key, value);
        self
    }

    /// ✍️ Sets `key` to `value` when there is one, else leaves the option unset.
    pub fn with_some(self, key: ViewKey, value: Option<ViewValue>) -> Self {
        match value {
            Some(value) => self.with(key, value),
            None => self,
        }
    }

    fn id(&self, key: ViewKey) -> Option<&str> {
        if let Some(ViewValue::Id(id)) = self.values.get(&key) { Some(id.as_str()) } else { None }
    }

    fn index(&self, key: ViewKey) -> Option<u32> {
        if let Some(ViewValue::Index(index)) = self.values.get(&key) { Some(*index) } else { None }
    }

    /// 🧮️ The id of the active combination column.
    pub fn combination(&self) -> Option<&str> {
        self.id(ViewKey::Combination)
    }

    /// 🧮️ The position of the active combination or variant.
    pub fn combination_item(&self) -> Option<u32> {
        self.index(ViewKey::CombinationItem)
    }

    /// 🚦️ The id of the chosen traffic group.
    pub fn traffic_group(&self) -> Option<&str> {
        self.id(ViewKey::TrafficGroup)
    }

    /// 🚦️ The position of the chosen traffic group.
    pub fn traffic_group_item(&self) -> Option<u32> {
        self.index(ViewKey::TrafficGroupItem)
    }

    /// 🎞️ The id of the selected track.
    pub fn track(&self) -> Option<&str> {
        self.id(ViewKey::Track)
    }

    /// 🎞️ The value on the selected track when it is the track `track` (the fire time in seconds for `fire`).
    pub fn track_value(&self, track: &str) -> Option<f64> {
        match (self.track(), self.values.get(&ViewKey::TrackValue)) {
            (Some(selected), Some(ViewValue::Number(value))) if selected == track => Some(*value),
            (_, _) => None,
        }
    }

    /// #️⃣ The canonical text of the options; part of the global fingerprint, so a view change re-projects exactly the entities that read it.
    pub fn stamp_text(&self) -> String {
        self.values.iter().map(|(key, value)| format!("{key:?}={value:?}")).collect::<Vec<_>>().join(";")
    }
}

/// 📄️ What a kit-backed projection reads: the snapshot, its persisted placements and the typed view options (EN 1990: the active combination and traffic group).
pub struct KitDoc<'a, S> {
    pub snapshot: &'a S,
    pub placements: &'a [Placement],
    pub view: &'a ViewOptions,
}

impl<S> Clone for KitDoc<'_, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for KitDoc<'_, S> {}

/// 🎯️ Which part of the scene a fragment run produces: everything, the fragment of one document entity, or the global rest (annotations, tracks and entities without an owner row).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part<'a> {
    All,
    Entity(&'a str),
    Global,
}

impl Part<'_> {
    /// 🎯️ Whether the entity with scene id `id` belongs to this part.
    pub fn wants(&self, id: &str) -> bool {
        match self {
            Part::All => true,
            Part::Entity(entity) => base_id(id) == *entity,
            Part::Global => false,
        }
    }

    /// 🔗️ Whether an annotation or track owned by `owner` (`None`: global) belongs to this part.
    pub fn keeps(&self, owner: Option<&str>) -> bool {
        match (self, owner) {
            (Part::All, _) => true,
            (Part::Entity(entity), Some(owner)) => base_id(owner) == *entity,
            (Part::Global, None) => true,
            _ => false,
        }
    }
}

/// 🧲️ The delegating contract of one artifact; each method forwards to the artifact's kit and modules.
pub trait SceneKit: Default {
    type Snapshot: Clone + PartialEq + ToValue + Send + 'static;

    /// 🆔️ Every document entity in document order, the order of the outliner and of the canonical scene.
    fn entities(snapshot: &Self::Snapshot) -> Vec<EntityRef>;
    /// 🔗️ The ids of the entities `id` reads besides itself.
    fn dependencies(snapshot: &Self::Snapshot, id: &str) -> Vec<String>;
    /// 🧩️ Runs the artifact modules restricted to `part`; `rows` holds the check rows the part may read.
    fn fragment(doc: &KitDoc<'_, Self::Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment;
    /// 🏁️ Sets layers, legends, description and bounds of the merged scene; `summary` carries the report summary and no rows.
    fn finish(doc: &KitDoc<'_, Self::Snapshot>, summary: &CheckReport, scene: &mut StructuralScene);
    /// 🧭️ The annex and standard edition the document selects, shown as the basis badge of the scene and its diagrams; `None` shows no badge.
    fn basis(_doc: &KitDoc<'_, Self::Snapshot>) -> Option<Basis> {
        None
    }
    /// 📍️ What an entity's pose derives from other entities (default layout rules); part of its fingerprint.
    fn layout_stamps(_doc: &KitDoc<'_, Self::Snapshot>) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
    /// 🗂️ The entity a top-level scalar field of the snapshot belongs to; `None` files it under the global bucket that every entity fingerprint contains.
    fn scalar_owner(_field: &str) -> Option<&'static str> {
        None
    }
    /// 🔇️ Whether the string field `field` of a row of `collection` merely points at another entity and must not file the row under it.
    fn inert_reference(_collection: &str, _field: &str) -> bool {
        false
    }
}

const FNV_BASIS: u64 = 0xcbf2_9ce4_8422_2325;

fn mix(hash: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(hash, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3))
}

fn mix_text(hash: u64, text: &str) -> u64 {
    mix(mix(hash, &(text.len() as u64).to_le_bytes()), text.as_bytes())
}

/// #️⃣ Structural FNV-1a over a value tree, without rendering it to text.
fn hash_tree(hash: u64, value: &DslValue) -> u64 {
    match value {
        DslValue::Null => mix(hash, &[0]),
        DslValue::Bool(flag) => mix(hash, &[1, u8::from(*flag)]),
        DslValue::Number(Number::UInt(n)) => mix(mix(hash, &[2]), &n.to_le_bytes()),
        DslValue::Number(Number::Int(n)) => mix(mix(hash, &[3]), &n.to_le_bytes()),
        DslValue::Number(Number::Float(n)) => mix(mix(hash, &[4]), &n.to_bits().to_le_bytes()),
        DslValue::String(text) => mix_text(mix(hash, &[5]), text),
        DslValue::Bytes(bytes) => mix(mix(mix(hash, &[6]), &(bytes.len() as u64).to_le_bytes()), bytes),
        DslValue::Array(items) => items.iter().fold(mix(mix(hash, &[7]), &(items.len() as u64).to_le_bytes()), hash_tree),
        DslValue::Object(entries) => entries.iter().fold(mix(mix(hash, &[8]), &(entries.len() as u64).to_le_bytes()), |hash, (key, item)| hash_tree(mix_text(hash, key), item)),
    }
}

/// #️⃣ The stamp of the hashes `parts`.
fn stamp(parts: impl IntoIterator<Item = u64>) -> u64 {
    parts.into_iter().fold(FNV_BASIS, |hash, part| mix(hash, &part.to_le_bytes()))
}

/// 📇️ Row hashes by entity id: the entity's own rows and every row naming it in a string field; rows that touch no entity are global and listed under `""`, as are the scalar fields of the snapshot. Hashed structurally, one pass over the value tree and no per-row text.
fn row_texts<S: Clone + PartialEq + ToValue + Send + 'static>(snapshot: &S, ids: &BTreeSet<&str>, owner: fn(&str) -> Option<&'static str>, inert: fn(&str, &str) -> bool) -> BTreeMap<String, Vec<u64>> {
    let mut texts: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    let tree = norm::incremental_evaluation::shared_value_tree(snapshot);
    let DslValue::Object(fields) = &*tree else { return texts };
    for (name, field) in fields {
        let base = mix_text(FNV_BASIS, name);
        let DslValue::Array(rows) = field else {
            texts.entry(owner(name).map_or_else(String::new, str::to_owned)).or_default().push(hash_tree(base, field));
            continue;
        };
        for row in rows {
            let hash = hash_tree(base, row);
            let DslValue::Object(entries) = row else {
                texts.entry(String::new()).or_default().push(hash);
                continue;
            };
            let named: BTreeSet<&str> = entries.iter().filter_map(|(key, v)| if let (DslValue::String(s), false) = (v, inert(name, key)) { Some(s.as_str()) } else { None }).filter(|s| ids.contains(s)).collect();
            if named.is_empty() {
                texts.entry(String::new()).or_default().push(hash);
            } else {
                named.into_iter().for_each(|id| texts.entry(id.to_owned()).or_default().push(hash));
            }
        }
    }
    texts
}

/// 🧲️ The [`StructuralProjection`] of a kit-backed artifact.
pub struct Kit<K>(PhantomData<fn() -> K>);

impl<K> Default for Kit<K> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<K: SceneKit> StructuralProjection for Kit<K> {
    type Document<'a> = KitDoc<'a, K::Snapshot>;

    fn schema() -> &'static str {
        SCENE_SCHEMA_ID
    }

    fn entity_kinds() -> &'static [EntityKindSpec] {
        &[]
    }

    fn entities(&self, doc: &KitDoc<'_, K::Snapshot>) -> Vec<EntityRef> {
        K::entities(doc.snapshot)
    }

    fn fingerprints(&self, doc: &KitDoc<'_, K::Snapshot>, entities: &[EntityRef]) -> Vec<String> {
        let all = K::entities(doc.snapshot);
        let ids: BTreeSet<&str> = all.iter().map(|e| base_id(&e.id)).collect();
        let texts = row_texts(doc.snapshot, &ids, K::scalar_owner, K::inert_reference);
        let layout = K::layout_stamps(doc);
        let view = doc.view.stamp_text();
        let global = stamp(texts.get("").into_iter().flatten().copied().chain([fnv64([view.as_str()])]));
        entities.iter().map(|e| hex(stamp([global, fnv64([layout.get(&e.id).map_or("", String::as_str)])].into_iter().chain(texts.get(base_id(&e.id)).into_iter().flatten().copied())))).collect()
    }

    fn entity_dependencies(&self, doc: &KitDoc<'_, K::Snapshot>, entity: &EntityRef) -> Vec<String> {
        K::dependencies(doc.snapshot, &entity.id)
    }

    fn project_entity(&self, doc: &KitDoc<'_, K::Snapshot>, entity: &EntityRef, _eval: &EntityEvaluation, rows: &CheckReport, _ctx: &ProjectionContext, out: &mut SceneBuilder) {
        out.absorb(K::fragment(doc, rows, Part::Entity(&entity.id)));
    }

    fn project_global(&self, doc: &KitDoc<'_, K::Snapshot>, eval: &Evaluation, _ctx: &ProjectionContext, out: &mut SceneBuilder) {
        out.owned_by(None);
        out.absorb(K::fragment(doc, &eval.reports.slice(&[String::new()]), Part::Global));
    }

    fn finish(&self, doc: &KitDoc<'_, K::Snapshot>, eval: &Evaluation, _ctx: &ProjectionContext, scene: &mut StructuralScene) {
        K::finish(doc, &CheckReport { summary: eval.reports.summary().clone(), checks: Vec::new() }, scene);
    }

    fn basis(&self, doc: &KitDoc<'_, K::Snapshot>) -> Option<Basis> {
        K::basis(doc)
    }

    fn diagrams(&self, _doc: &KitDoc<'_, K::Snapshot>, _eval: &Evaluation, _focus: &Focus, _ctx: &ProjectionContext) -> Vec<Diagram2d> {
        Vec::new()
    }

    fn handles(&self, _doc: &KitDoc<'_, K::Snapshot>, _selection: &[EntityRef], _ctx: &ProjectionContext) -> Vec<Handle> {
        Vec::new()
    }
}

/// 🏞️ The id of the domain-neutral project site entity that owns the document-level checks of every artifact; it leads every canonical order, as the global fragment does in the live path.
pub const SITE_ENTITY: &str = "project-site";

/// 🔀️ Puts `scene` into the canonical order shared by the full and the incremental path: entities by the position of their document entity in `order` (unknown ones last), annotations, tracks and ownership rows by the position of their owner (global first); the order inside one group is the emission order.
pub fn canonical_order(scene: &mut StructuralScene, order: &[EntityRef]) {
    let rank: BTreeMap<&str, usize> = order.iter().enumerate().map(|(i, e)| (base_id(&e.id), i + 1)).collect();
    let of = |id: &str| if base_id(id) == SITE_ENTITY { 0 } else { rank.get(base_id(id)).copied().unwrap_or(usize::MAX) };
    let owned: BTreeMap<(bool, &str), usize> = scene.owners.iter().map(|o| ((o.kind == OwnedKind::Track, o.id.as_str()), of(&o.owner.id))).collect();
    let annotations: BTreeMap<String, usize> = scene.annotations.iter().map(|a| (a.id().to_owned(), owned.get(&(false, a.id())).copied().unwrap_or(0))).collect();
    let tracks: BTreeMap<String, usize> = scene.tracks.iter().map(|t| (t.id.clone(), owned.get(&(true, t.id.as_str())).copied().unwrap_or(0))).collect();
    scene.entities.sort_by_key(|e| of(&e.entity.id));
    scene.annotations.sort_by_key(|a| annotations[a.id()]);
    scene.tracks.sort_by_key(|t| tracks[&t.id]);
    scene.owners.sort_by_key(|o| of(&o.owner.id));
}

/// 🎬️ The whole scene of a kit-backed artifact in one pass: one unrestricted fragment, canonical order, the artifact's `finish`. Equal to what [`SceneProjector`] assembles from the per-entity fragments, apart from the revision stamps.
pub fn project_whole<K: SceneKit>(doc: &KitDoc<'_, K::Snapshot>, report: &CheckReport, revision: SceneRevision) -> StructuralScene {
    let mut scene = merge(&[Arc::new(K::fragment(doc, report, Part::All))], revision);
    canonical_order(&mut scene, &K::entities(doc.snapshot));
    K::finish(doc, &CheckReport { summary: report.summary.clone(), checks: Vec::new() }, &mut scene);
    if let Some(basis) = K::basis(doc) {
        basis.apply(&mut scene);
    }
    scene
}

/// 🎬️ The whole scene exactly as the live projector assembles it: the global fragment, then the fragment of every entity in document order, merged and finished. For kits whose scene entity ids do not follow the order of their document entities, where [`project_whole`] would sort them differently.
pub fn project_parts<K: SceneKit>(doc: &KitDoc<'_, K::Snapshot>, report: &CheckReport, revision: SceneRevision) -> StructuralScene {
    let refs = K::entities(doc.snapshot);
    let parts: Vec<Arc<Fragment>> = std::iter::once(Part::Global).chain(refs.iter().map(|e| Part::Entity(&e.id))).map(|part| Arc::new(K::fragment(doc, report, part))).collect();
    let mut scene = merge(&parts, revision);
    K::finish(doc, &CheckReport { summary: report.summary.clone(), checks: Vec::new() }, &mut scene);
    if let Some(basis) = K::basis(doc) {
        basis.apply(&mut scene);
    }
    scene
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
