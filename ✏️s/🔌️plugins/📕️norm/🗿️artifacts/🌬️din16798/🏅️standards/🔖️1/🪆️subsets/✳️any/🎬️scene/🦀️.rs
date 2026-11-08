//! 🎬️ DIN EN 16798 structural scene: the pure projection of the indoor-environment snapshot and its check report to the locale-free `StructuralScene` (rooms coloured by comfort category, ventilation systems), its `Diagram2d` set (adaptive comfort, operative bands, ventilation rates, CO₂, humidity, capacity) and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_din16798_checks as checks;
extern crate semio_s_artifact_norm_din16798_inferences as inferences;
extern crate semio_s_artifact_norm_din16798_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
pub use kit::{CheckRoute, EntityKindSpec, DOCUMENT_CHECKS, ENTITY_KINDS, binding, entities_of, entity_dependencies, entity_kind_of, focus_of_outcome, owner_entity, route_of};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::Layout;

#[path = "🧱️rooms/🦀️.rs"]
mod rooms;
pub use rooms::{category_name, category_number};

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use kit::Out;
use model::Din16798Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit, ViewOptions};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};

/// 🎯️ Everything one projection reads: the document, its settled report and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a Din16798Snapshot,
    pub report: &'a CheckReport,
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<Din16798Kit>(&KitDoc { snapshot: input.document, placements: &[], view: &ViewOptions::default() }, input.report, input.revision.clone())
}

/// 🎬️ The scene projected from scratch, without retained fragments: the baseline of the incremental-rebuild gate of the latency bench.
pub fn full_scene_of(document: &Din16798Snapshot, report: &CheckReport) -> StructuralScene {
    project(&SceneInput { document, report, revision: projection_kit::revision_of(document, &[], report) })
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document);
    let mut out = Out::scoped(part);
    rooms::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of DIN EN 16798, delegating to the modules above.
#[derive(Default)]
pub struct Din16798Kit;

fn input_of<'a>(doc: &KitDoc<'a, Din16798Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for Din16798Kit {
    type Snapshot = Din16798Snapshot;

    fn basis(doc: &KitDoc<'_, Din16798Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::DinEn16798, doc.snapshot.annex))
    }

    fn entities(snapshot: &Din16798Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &Din16798Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, Din16798Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, Din16798Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, Din16798Snapshot>) -> std::collections::BTreeMap<String, String> {
        Layout::of(doc.snapshot).poses().map(|(id, pose)| (id.clone(), format!("{pose:?}"))).collect()
    }
}

live_scene!(pub mod live = Kit<Din16798Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &Din16798Snapshot, report: &CheckReport) -> StructuralScene {
    let revision = projection_kit::revision_of(document, &[], report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &[], view: &ViewOptions::default() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    LocalizedCopy::new(
        format!("DIN EN 16798 indoor environment with {} rooms and {} ventilation systems; worst utilisation {:.0} %.", d.zones.len(), d.vent_systems.len(), worst * 100.0),
        format!("DIN-EN-16798-Raumklima mit {} Räumen und {} Lüftungsanlagen; größte Ausnutzung {:.0} %.", d.zones.len(), d.vent_systems.len(), worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
