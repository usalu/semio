//! 🎬️ DIN V 18599 structural scene: the pure projection of the building snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_din18599_checks as checks;
extern crate semio_s_artifact_norm_din18599_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS};
pub use diagrams::{diagrams, flows};
pub use handles::handles;

#[path = "🧰️kit/🦀️.rs"]
pub mod kit;

#[path = "📍️layout/🦀️.rs"]
pub mod layout;

#[path = "🧱️envelope/🦀️.rs"]
mod envelope;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;

use kit::Out;
use layout::Layout;
use model::Din18599Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};

/// 🎯️ Everything one projection reads: the document, its settled report and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a Din18599Snapshot,
    pub report: &'a CheckReport,
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<Din18599Kit>(&KitDoc { snapshot: input.document, placements: &[], view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document);
    let mut out = Out::scoped(part);
    envelope::project(input, &layout, &mut out);
    projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

fn fresh_id(spec: &str, taken: Vec<&str>) -> String {
    (taken.len() + 1..).map(|n| format!("{spec}-{n}")).find(|id| !taken.contains(&id.as_str())).unwrap_or_default()
}

/// 👻️ The ghost of the insert tool: the scene entities the inserted row `spec` (`zone` or `element`, a copy of the last row under a fresh id) would add, drawn translucent and hatched without utilisation colour; empty when the collection has no row to copy.
pub fn insert_ghost(document: &Din18599Snapshot, report: &CheckReport, spec: &str) -> Vec<scene::SceneEntity> {
    let mut preview = document.clone();
    let id = match spec {
        "zone" => preview.zones.last().cloned().map(|mut row| {
            row.id = fresh_id(spec, preview.zones.iter().map(|z| z.id.as_str()).collect());
            let id = row.id.clone();
            preview.zones.push(row);
            id
        }),
        "element" => preview.elements.last().cloned().map(|mut row| {
            row.id = fresh_id(spec, preview.elements.iter().map(|e| e.id.as_str()).collect());
            let id = row.id.clone();
            preview.elements.push(row);
            id
        }),
        _ => None,
    };
    let Some(id) = id else { return Vec::new() };
    let input = SceneInput { document: &preview, report, revision: SceneRevision { document_seq: 0, report_seq: 0 } };
    fragment(&input, Part::Entity(&id))
        .entities
        .into_iter()
        .map(|mut entity| {
            entity.appearance.style = scene::StyleState::Ghost;
            entity.appearance.channel = scene::ColourChannel::Role;
            entity.appearance.opacity = 0.35;
            entity.appearance.hatch = Some(scene::Hatch::Dots);
            entity.utilisation = None;
            entity.handles.clear();
            entity.sealed()
        })
        .collect()
}

/// 🧲️ The live-scene contract of DIN V 18599, delegating to the modules above.
#[derive(Default)]
pub struct Din18599Kit;

fn input_of<'a>(doc: &KitDoc<'a, Din18599Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for Din18599Kit {
    type Snapshot = Din18599Snapshot;

    fn entities(snapshot: &Din18599Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &Din18599Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, Din18599Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, Din18599Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }
}

live_scene!(pub mod live = Kit<Din18599Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &Din18599Snapshot, report: &CheckReport) -> StructuralScene {
    let revision = projection_kit::revision_of(document, &[], report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &[], view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    LocalizedCopy::new(
        format!("DIN V 18599 building model with {} thermal zones and {} envelope elements; worst utilisation {:.0} %.", d.zones.len(), d.elements.len(), worst * 100.0),
        format!("DIN-V-18599-Gebäudemodell mit {} thermischen Zonen und {} Hüllbauteilen; größte Ausnutzung {:.0} %.", d.zones.len(), d.elements.len(), worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
