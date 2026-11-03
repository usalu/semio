//! 🎯️ Projection framework: the pure per-artifact `StructuralProjection` contract and the incremental `SceneProjector` that caches fragments by content hash and emits minimal deltas.

#[path = "🧱️scene-builder/🦀️.rs"]
pub mod scene_builder;
#[path = "🧩️fragment-cache/🦀️.rs"]
pub mod fragment_cache;
#[path = "🔗️dependency-index/🦀️.rs"]
pub mod dependency_index;
#[path = "🗣️description/🦀️.rs"]
pub mod description;
#[path = "🛰️live-scene/🦀️.rs"]
pub mod live_scene;
#[path = "🗂️scene-memo/🦀️.rs"]
pub mod scene_memo;
#[path = "🗃️report-index/🦀️.rs"]
pub mod report_index;
#[path = "🧲️scene-kit/🦀️.rs"]
pub mod scene_kit;
#[path = "🧭️basis/🦀️.rs"]
pub mod basis;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod unit;

pub use basis::{basis_title, with_basis, Basis, BASIS_LEGEND_ID};
pub use dependency_index::DependencyIndex;
pub use live_scene::{evaluation_of, LiveScene};
pub use scene_memo::{memo_diagrams, SceneMemo};
pub use fragment_cache::{fnv64, FragmentCache};
pub use report_index::ReportIndex;
pub use scene_kit::{canonical_order, project_parts, project_whole, Kit, SITE_ENTITY, KitDoc, Part, SceneKit, ViewKey, ViewOptions, ViewValue};
pub use scene_builder::{apply_visibility, assemble, assemble_with, merge, EntityDraft, Fragment, SceneBuilder};

use crate::diff::diff;
use crate::tessellation::Lod;
use crate::{canonical_json, CheckStatus, Diagram2d, EntityKind, EntityRef, Handle, LocalizedCopy, OwnedKind, PickPolicy, SceneDelta, SceneRevision, StationValue, StructuralScene, Violation, SCENE_SCHEMA_ID};
use norm::CheckReport;
use std::collections::{BTreeMap, BTreeSet};

/// 🗂️ Static description of an entity kind: the single source for outliner, picking granularity and the add-entity palette.
#[derive(Clone, Debug, PartialEq)]
pub struct EntityKindSpec {
    pub id: &'static str,
    pub kind: EntityKind,
    pub label_en: &'static str,
    pub label_de: &'static str,
    pub icon: &'static str,
    pub collection: &'static str,
    pub pick: PickPolicy,
}

/// ⚙️ Knobs that change what is projected; everything except `document_seq` is part of every cache key.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct ProjectionContext {
    pub document_seq: u64,
    pub detail: Lod,
    pub deformation_scale: f64,
    pub load_case_filter: Option<Vec<String>>,
    pub visible_layers: Option<Vec<String>>,
}

/// 🚦️ One check outcome of an entity as the projection needs it; the artifact maps its report rows onto this.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct CheckSample {
    pub check_id: String,
    pub utilisation: f64,
    pub status: CheckStatus,
}

/// 🌡️ Evaluation state of one entity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue)]
#[value(rename_all = "camelCase")]
pub struct EntityEvaluation {
    pub checks: Vec<CheckSample>,
    pub stale: bool,
    pub stations: Option<Vec<StationValue>>,
}

/// 📊️ Evaluation of the whole document at one report revision; `reports` is the incrementally maintained per-subject index of the check rows.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Evaluation {
    pub report_seq: u64,
    pub entities: BTreeMap<String, EntityEvaluation>,
    pub global: EntityEvaluation,
    pub reports: ReportIndex,
}

/// 🎯️ What the user is looking at, for diagram and handle selection.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Focus {
    pub selection: Vec<EntityRef>,
}

/// ✏️ Entities whose document subtree changed since the previous projection; `all` forces a full pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChangedSet {
    pub all: bool,
    pub entities: BTreeSet<String>,
}

impl EntityEvaluation {
    /// 🏆️ The check with the highest utilisation; the first wins ties.
    pub fn worst(&self) -> Option<&CheckSample> {
        self.checks.iter().fold(None, |best: Option<&CheckSample>, c| match best {
            Some(b) if b.utilisation.total_cmp(&c.utilisation).is_ge() => Some(b),
            _ => Some(c),
        })
    }
}

impl Evaluation {
    pub fn of(&self, id: &str) -> EntityEvaluation {
        self.entities.get(id).cloned().unwrap_or_default()
    }
}

impl ChangedSet {
    pub fn all() -> Self {
        Self { all: true, entities: BTreeSet::new() }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn of<I: IntoIterator<Item = S>, S: Into<String>>(ids: I) -> Self {
        Self { all: false, entities: ids.into_iter().map(Into::into).collect() }
    }
}

impl ProjectionContext {
    pub fn new(document_seq: u64, detail: Lod) -> Self {
        Self { document_seq, detail, deformation_scale: 1.0, load_case_filter: None, visible_layers: None }
    }

    fn cache_key(&self) -> String {
        canonical_json(&Self { document_seq: 0, ..self.clone() })
    }
}

/// 🎯 Pure per-artifact mapping from a document and its evaluation to scene fragments; no I/O, no locale, no framework types.
pub trait StructuralProjection {
    type Document<'a>;

    fn schema() -> &'static str;
    fn entity_kinds() -> &'static [EntityKindSpec];
    fn entities(&self, doc: &Self::Document<'_>) -> Vec<EntityRef>;
    /// #️⃣ One content fingerprint per entity of `entities`, in order: everything the entity's fragment reads of the document.
    fn fingerprints(&self, doc: &Self::Document<'_>, entities: &[EntityRef]) -> Vec<String>;
    /// 🔗️ The ids of the entities `entity` reads besides itself.
    fn entity_dependencies(&self, doc: &Self::Document<'_>, entity: &EntityRef) -> Vec<String>;
    /// 🧩️ The fragment of one entity; `rows` holds only the check rows of the entity, the entities it reads and the global ones, in report order.
    fn project_entity(&self, doc: &Self::Document<'_>, entity: &EntityRef, eval: &EntityEvaluation, rows: &CheckReport, ctx: &ProjectionContext, out: &mut SceneBuilder);
    fn project_global(&self, doc: &Self::Document<'_>, eval: &Evaluation, ctx: &ProjectionContext, out: &mut SceneBuilder);
    /// 🏁️ Rebuilds the global parts (layers, legends, description, bounds) of the assembled scene from its entities and the report summary; the default keeps the neutral parts of the fragments.
    fn finish(&self, _doc: &Self::Document<'_>, _eval: &Evaluation, _ctx: &ProjectionContext, _scene: &mut StructuralScene) {}
    /// 🧭️ The annex and standard edition the document selects; the projector puts the badge into the scene after `finish`, and the artifact's `diagrams` put the same basis into every diagram header ([`with_basis`]). `None` shows no badge.
    fn basis(&self, _doc: &Self::Document<'_>) -> Option<Basis> {
        None
    }
    fn diagrams(&self, doc: &Self::Document<'_>, eval: &Evaluation, focus: &Focus, ctx: &ProjectionContext) -> Vec<Diagram2d>;
    fn handles(&self, doc: &Self::Document<'_>, selection: &[EntityRef], ctx: &ProjectionContext) -> Vec<Handle>;
}

const GLOBAL: &str = "$global";

/// 🧩 Runner: caches one fragment per (entity, content key), re-projects only what changed and yields the full scene or the delta from the previous one.
pub struct SceneProjector<P: StructuralProjection> {
    projection: P,
    cache: FragmentCache,
    index: DependencyIndex,
    fingerprints: BTreeMap<String, String>,
    previous: Option<StructuralScene>,
}

fn key_of<'a>(keys: &'a BTreeMap<&str, String>, default: &'a str, id: &str) -> &'a str {
    keys.get(id).map_or(default, String::as_str)
}

fn hex(n: u64) -> String {
    format!("{n:016x}")
}

impl<P: StructuralProjection> SceneProjector<P> {
    pub fn new(projection: P) -> Self {
        Self { projection, cache: FragmentCache::new(), index: DependencyIndex::new(), fingerprints: BTreeMap::new(), previous: None }
    }

    pub fn projection(&self) -> &P {
        &self.projection
    }

    pub fn cache(&self) -> &FragmentCache {
        &self.cache
    }

    pub fn cache_mut(&mut self) -> &mut FragmentCache {
        &mut self.cache
    }

    pub fn previous(&self) -> Option<&StructuralScene> {
        self.previous.as_ref()
    }

    fn run(&mut self, doc: &P::Document<'_>, changed: &ChangedSet, eval: &Evaluation, ctx: &ProjectionContext) -> Result<StructuralScene, Vec<Violation>> {
        let refs = self.projection.entities(doc);
        let live: BTreeSet<String> = refs.iter().map(|r| r.id.clone()).collect();
        let gone: Vec<String> = self.fingerprints.keys().filter(|id| !live.contains(*id)).cloned().collect();
        for id in &gone {
            self.fingerprints.remove(id);
            self.index.remove(id);
        }
        let mut keep = live.clone();
        keep.insert(GLOBAL.into());
        self.cache.retain(&keep);
        let dirty: BTreeSet<String> = if changed.all { live.clone() } else {
            let mut d = self.index.dependents(&changed.entities);
            d.extend(live.iter().filter(|id| !self.fingerprints.contains_key(*id)).cloned());
            d
        };
        let todo: Vec<EntityRef> = refs.iter().filter(|r| dirty.contains(&r.id)).cloned().collect();
        for (r, print) in todo.iter().zip(self.projection.fingerprints(doc, &todo)) {
            if self.fingerprints.insert(r.id.clone(), print.clone()).is_none_or(|old| old != print) {
                self.index.set(&r.id, self.projection.entity_dependencies(doc, r));
            }
        }
        let ctx_key = ctx.cache_key();
        let default_key = hex(fnv64([canonical_json(&EntityEvaluation::default()).as_str()]));
        let eval_keys: BTreeMap<&str, String> = eval.entities.iter().map(|(id, ev)| (id.as_str(), hex(fnv64([canonical_json(ev).as_str()])))).collect();
        let eval_key = |id: &str| key_of(&eval_keys, &default_key, id);
        let mut keys: Vec<u64> = Vec::with_capacity(refs.len());
        let mut fragments = Vec::with_capacity(refs.len() + 1);
        let mut parts: Vec<std::sync::Arc<Fragment>> = Vec::new();
        for r in &refs {
            let mut subjects: Vec<String> = vec![r.id.clone(), String::new()];
            let mut dependencies: Vec<String> = Vec::new();
            for dep in self.index.dependencies(&r.id) {
                dependencies.push(format!("{dep}={}|{}", self.fingerprints.get(&dep).map_or("", String::as_str), eval_key(&dep)));
                subjects.push(dep);
            }
            let reports = hex(eval.reports.stamp(&subjects));
            let key = fnv64([self.fingerprints[&r.id].as_str(), eval_key(&r.id), ctx_key.as_str()].into_iter().chain(dependencies.iter().map(String::as_str)).chain([reports.as_str()]));
            keys.push(key);
            let projection = &self.projection;
            fragments.push(self.cache.get_or_build(&r.id, key, || {
                let mut b = SceneBuilder::new();
                b.owned_by(Some(r.clone()));
                projection.project_entity(doc, r, &eval.of(&r.id), &eval.reports.slice(&subjects), ctx, &mut b);
                b.into_fragment()
            }));
        }
        let global_text: Vec<String> = keys.iter().map(|k| hex(*k)).chain(eval_keys.iter().map(|(id, key)| format!("{id}={key}"))).chain([canonical_json(&eval.global), hex(eval.reports.version()), ctx_key]).collect();
        let global_key = fnv64(global_text.iter().map(String::as_str));
        let projection = &self.projection;
        parts.push(self.cache.get_or_build(GLOBAL, global_key, || {
            let mut b = SceneBuilder::new();
            projection.project_global(doc, eval, ctx, &mut b);
            b.into_fragment()
        }));
        parts.extend(fragments);
        let projection = &self.projection;
        assemble_with(&parts, SceneRevision { document_seq: ctx.document_seq, report_seq: eval.report_seq }, ctx, |scene| {
            projection.finish(doc, eval, ctx, scene);
            if let Some(basis) = projection.basis(doc) {
                basis.apply(scene);
            }
        })
    }

    /// 🎬️ Full pass: every fingerprint is recomputed, fragments still come from the cache when their key matches.
    pub fn project(&mut self, doc: &P::Document<'_>, eval: &Evaluation, ctx: &ProjectionContext) -> Result<StructuralScene, Vec<Violation>> {
        let scene = self.run(doc, &ChangedSet::all(), eval, ctx)?;
        self.previous = Some(scene.clone());
        Ok(scene)
    }

    /// 🎬️ [`Self::project`] without retaining the scene as the base of the next delta: no copy of the whole scene, for callers that diff on their own (the scene memo).
    pub fn project_untracked(&mut self, doc: &P::Document<'_>, eval: &Evaluation, ctx: &ProjectionContext) -> Result<StructuralScene, Vec<Violation>> {
        self.run(doc, &ChangedSet::all(), eval, ctx)
    }

    /// 🩹️ Incremental pass: only `changed` and the entities reading them are fingerprinted again; the delta is against the previous scene (an empty scene on first use).
    pub fn project_delta(&mut self, doc: &P::Document<'_>, changed: &ChangedSet, eval: &Evaluation, ctx: &ProjectionContext) -> Result<(StructuralScene, SceneDelta), Vec<Violation>> {
        let scene = self.run(doc, changed, eval, ctx)?;
        let base = self.previous.take().unwrap_or_else(empty_scene);
        let delta = diff(&base, &scene);
        self.previous = Some(scene.clone());
        Ok((scene, delta))
    }
}

/// 🫥️ The scene a consumer holds before the first projection.
pub fn empty_scene() -> StructuralScene {
    use crate::{Aabb, Vec3};
    let mut scene = StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 0, report_seq: 0 },
        bounds: Aabb::new(Vec3::ZERO, Vec3::ZERO),
        entities: Vec::new(),
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: Vec::new(),
        owners: Vec::new(),
        description: LocalizedCopy::new("Scene", "Szene"),
    };
    scene.description = description::describe(&scene);
    scene
}
