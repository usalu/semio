//! 🎬️ ISO 16757 product scene: the pure projection of the catalogue snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_iso16757_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};

#[path = "🧊️objects/🦀️.rs"]
mod objects;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use kit::Out;
use model::Iso16757Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a Iso16757Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per object.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<Iso16757Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let mut out = Out::scoped(part);
    objects::project(input, &layout, &mut out);
    projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of ISO 16757, delegating to the modules above.
#[derive(Default)]
pub struct Iso16757Kit;

fn input_of<'a>(doc: &KitDoc<'a, Iso16757Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for Iso16757Kit {
    type Snapshot = Iso16757Snapshot;

    fn entities(snapshot: &Iso16757Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &Iso16757Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, Iso16757Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, Iso16757Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, Iso16757Snapshot>) -> BTreeMap<String, String> {
        let layout = Layout::of(doc.snapshot, doc.placements);
        doc.snapshot.geometry.objects.keys().filter_map(|id| layout.pose(id).map(|pose| (id.clone(), format!("{pose:?}")))).collect()
    }
}

live_scene!(pub mod live = Kit<Iso16757Kit>);

/// ⚡️ The scene editor and viewer paint: per-object fragments from the retained live scene, so an edit re-projects only the objects it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &Iso16757Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let (objects, products, variants) = (d.geometry.objects.len(), d.catalogue.products.len(), d.catalogue.products.iter().map(|p| p.variants.len()).sum::<usize>());
    LocalizedCopy::new(
        format!("ISO 16757 catalogue with {products} products, {variants} variants and {objects} geometry objects; worst utilisation {:.0} %.", worst * 100.0),
        format!("ISO-16757-Katalog mit {products} Produkten, {variants} Varianten und {objects} Geometrieobjekten; größte Ausnutzung {:.0} %.", worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
