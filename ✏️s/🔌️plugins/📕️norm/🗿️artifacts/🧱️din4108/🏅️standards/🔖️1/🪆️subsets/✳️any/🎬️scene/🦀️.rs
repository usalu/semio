//! 🎬️ DIN 4108 structural scene: the pure projection of the building-envelope snapshot and its check report to the locale-free `StructuralScene` (layer sections, thermal zones, thermal bridges), its `Diagram2d` set (temperature, Glaser vapour pressure, U-values) and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_din4108_checks as checks;
extern crate semio_s_artifact_norm_din4108_inferences as inferences;
extern crate semio_s_artifact_norm_din4108_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
pub use kit::{CheckRoute, EntityKindSpec, DOCUMENT_CHECKS, ENTITY_KINDS, binding, entities_of, entity_dependencies, entity_kind_of, focus_of_outcome, owner_entity, route_of};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::Layout;

#[path = "🧱️elements/🦀️.rs"]
mod elements;
pub use elements::{role_of, u_value_of};

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use kit::Out;
use model::Din4108Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit, ViewOptions};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};

/// 🎯️ Everything one projection reads: the document, its settled report and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a Din4108Snapshot,
    pub report: &'a CheckReport,
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<Din4108Kit>(&KitDoc { snapshot: input.document, placements: &[], view: &ViewOptions::default() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document);
    let mut out = Out::scoped(part);
    elements::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of DIN 4108, delegating to the modules above.
#[derive(Default)]
pub struct Din4108Kit;

fn input_of<'a>(doc: &KitDoc<'a, Din4108Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for Din4108Kit {
    type Snapshot = Din4108Snapshot;

    fn basis(_doc: &KitDoc<'_, Din4108Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::Din4108, norm::AnnexChoice::De))
    }

    fn entities(snapshot: &Din4108Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &Din4108Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, Din4108Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, Din4108Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, Din4108Snapshot>) -> std::collections::BTreeMap<String, String> {
        Layout::of(doc.snapshot).poses().map(|(id, pose)| (id.clone(), format!("{pose:?}"))).collect()
    }
}

live_scene!(pub mod live = Kit<Din4108Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &Din4108Snapshot, report: &CheckReport) -> StructuralScene {
    let revision = projection_kit::revision_of(document, &[], report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &[], view: &ViewOptions::default() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    LocalizedCopy::new(
        format!("DIN 4108 envelope with {} elements, {} thermal zones and {} thermal bridges; worst utilisation {:.0} %.", d.elements.len(), d.zones.len(), d.thermal_bridges.len(), worst * 100.0),
        format!("DIN-4108-Gebäudehülle mit {} Bauteilen, {} thermischen Zonen und {} Wärmebrücken; größte Ausnutzung {:.0} %.", d.elements.len(), d.zones.len(), d.thermal_bridges.len(), worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
