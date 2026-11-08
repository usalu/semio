//! 🎬️ VDI 3805 structural scene: the pure projection of the manufacturer dataset and its check report to the locale-free `StructuralScene` (catalogue products and parametric geometry), its `Diagram2d` set (characteristic curves, radiator output, valve pressure drop, catalogue utilisation) and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_vdi3805_checks as checks;
extern crate semio_s_artifact_norm_vdi3805_inferences as inferences;
extern crate semio_s_artifact_norm_vdi3805_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
pub use kit::{CheckRoute, EntityKindSpec, CURVES, ENTITY_KINDS, GEOMETRY, PRODUCTS, binding, entities_of, entity_dependencies, entity_kind_of, focus_of_outcome, owner_entity, route_of};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::Layout;

#[path = "🧱️products/🦀️.rs"]
mod products;
pub use products::{detail_of, role_of};

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use kit::Out;
use model::Vdi3805Snapshot;
use norm::CheckReport;
use scene::projection::{self as projection, project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit, StructuralProjection, ViewOptions};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};

/// 🎯️ Everything one projection reads: the document, its settled report and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a Vdi3805Snapshot,
    pub report: &'a CheckReport,
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<Vdi3805Kit>(&KitDoc { snapshot: input.document, placements: &[], view: ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document);
    let mut out = Out::scoped(part);
    products::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of VDI 3805, delegating to the modules above.
#[derive(Default)]
pub struct Vdi3805Kit;

fn input_of<'a>(doc: &KitDoc<'a, Vdi3805Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for Vdi3805Kit {
    type Snapshot = Vdi3805Snapshot;

    fn basis(_doc: &KitDoc<'_, Vdi3805Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::Vdi3805, checks::ANNEX))
    }

    fn entities(snapshot: &Vdi3805Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &Vdi3805Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, Vdi3805Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, Vdi3805Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, Vdi3805Snapshot>) -> std::collections::BTreeMap<String, String> {
        Layout::of(doc.snapshot).poses().map(|(id, pose)| (id.clone(), format!("{pose:?}"))).collect()
    }
}

/// 🧲️ The kit projection with per-row fingerprints: the shared kit hashes top-level lists, but the products live under `catalog` and the geometries under an id-keyed map, so the fingerprint of an entity is its own row, the geometry row it references, its pose and the view stamp; an edit re-projects only the entities it touches.
#[derive(Default)]
pub struct Vdi3805Projection(Kit<Vdi3805Kit>);

impl StructuralProjection for Vdi3805Projection {
    type Document<'a> = KitDoc<'a, Vdi3805Snapshot>;

    fn schema() -> &'static str {
        <Kit<Vdi3805Kit> as StructuralProjection>::schema()
    }

    fn entity_kinds() -> &'static [projection::EntityKindSpec] {
        <Kit<Vdi3805Kit> as StructuralProjection>::entity_kinds()
    }

    fn entities(&self, doc: &KitDoc<'_, Vdi3805Snapshot>) -> Vec<EntityRef> {
        self.0.entities(doc)
    }

    fn fingerprints(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, entities: &[EntityRef]) -> Vec<String> {
        let d = doc.snapshot;
        let layout = Layout::of(d);
        let view = doc.view.stamp_text();
        entities
            .iter()
            .map(|entity| {
                let id = projection_kit::base_id(&entity.id);
                let product = d.catalog.products.iter().find(|p| p.id == id);
                let geometry = product.and_then(|p| p.configuration.geometry_ref.as_deref()).unwrap_or(id);
                let texts = [format!("{product:?}"), format!("{:?}", d.geometry.get(geometry)), format!("{:?}", layout.pose(id)), view.clone()];
                format!("{:016x}", projection::fnv64(texts.iter().map(String::as_str)))
            })
            .collect()
    }

    fn entity_dependencies(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, entity: &EntityRef) -> Vec<String> {
        self.0.entity_dependencies(doc, entity)
    }

    fn project_entity(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, entity: &EntityRef, eval: &projection::EntityEvaluation, rows: &CheckReport, ctx: &ProjectionContext, out: &mut projection::SceneBuilder) {
        self.0.project_entity(doc, entity, eval, rows, ctx, out)
    }

    fn project_global(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, eval: &projection::Evaluation, ctx: &ProjectionContext, out: &mut projection::SceneBuilder) {
        self.0.project_global(doc, eval, ctx, out)
    }

    fn finish(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, eval: &projection::Evaluation, ctx: &ProjectionContext, scene: &mut StructuralScene) {
        self.0.finish(doc, eval, ctx, scene)
    }

    fn basis(&self, doc: &KitDoc<'_, Vdi3805Snapshot>) -> Option<scene::projection::Basis> {
        self.0.basis(doc)
    }

    fn diagrams(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, eval: &projection::Evaluation, focus: &projection::Focus, ctx: &ProjectionContext) -> Vec<scene::Diagram2d> {
        self.0.diagrams(doc, eval, focus, ctx)
    }

    fn handles(&self, doc: &KitDoc<'_, Vdi3805Snapshot>, selection: &[EntityRef], ctx: &ProjectionContext) -> Vec<scene::Handle> {
        self.0.handles(doc, selection, ctx)
    }
}

live_scene!(pub mod live = Vdi3805Projection);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &Vdi3805Snapshot, report: &CheckReport) -> StructuralScene {
    let revision = projection_kit::revision_of(document, &[], report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &[], view: ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    LocalizedCopy::new(
        format!("VDI 3805 manufacturer dataset with {} products, {} geometries and {} characteristic curves; worst utilisation {:.0} %.", d.catalog.products.len(), d.geometry.len(), d.curves.len(), worst * 100.0),
        format!("VDI-3805-Herstellerdatensatz mit {} Produkten, {} Geometrien und {} Kennlinien; größte Ausnutzung {:.0} %.", d.catalog.products.len(), d.geometry.len(), d.curves.len(), worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
