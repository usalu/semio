//! 🎬️ EN 1994 structural scene: the pure projection of the composite snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1994_checks as checks;
extern crate semio_s_artifact_norm_en1994_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;
extern crate semio_s_artifact_norm_en1994_scene_base as base;

pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS};

pub use layout::{Layout, Placement};

#[path = "🌉️beams/🦀️.rs"]
mod beams;

#[path = "🏛️columns/🦀️.rs"]
mod columns;

#[path = "🎞️tracks/🦀️.rs"]
mod tracks;

#[path = "🧱️slabs/🦀️.rs"]
mod slabs;

#[path = "🧷️slip-arrows/🦀️.rs"]
mod slip_arrows;

#[path = "🌈️fields/🦀️.rs"]
mod fields;

#[path = "🔥️fire-section/🦀️.rs"]
mod fire_section;

#[path = "🔁️fatigue/🦀️.rs"]
mod fatigue;

#[path = "🗂️cases/🦀️.rs"]
pub mod cases;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

pub use handles::handles;

pub use base::{facts, handles, layout};
use base::{kit, statics};
use kit::Out;
use model::En1994Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a En1994Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1994Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let mut out = Out::scoped(part);
    beams::project(input, &layout, &mut out);
    columns::project(input, &layout, &mut out);
    slabs::project(input, &layout, &mut out);
    slip_arrows::project(input, &layout, &mut out);
    fatigue::highlight(input, &layout, &mut out);
    cases::project(input, &layout, &mut out);
    fields::project(input, &layout, &mut out);
    tracks::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1994, delegating to the modules above.
#[derive(Default)]
pub struct En1994Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1994Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1994Kit {
    type Snapshot = En1994Snapshot;

    fn basis(doc: &KitDoc<'_, En1994Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1994, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1994Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1994Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1994Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1994Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
        tracks::finish(scene);
    }

    fn layout_stamps(doc: &KitDoc<'_, En1994Snapshot>) -> BTreeMap<String, String> {
        let layout = Layout::of(doc.snapshot, doc.placements);
        let modes = tracks::offers_modes(doc.snapshot, &layout);
        let d = doc.snapshot;
        d.beams.iter().map(|e| &e.id).chain(d.columns.iter().map(|e| &e.id)).chain(d.slabs.iter().map(|e| &e.id)).filter_map(|id| layout.pose(id).map(|pose| (id.clone(), format!("{pose:?}|modes={modes}|fire={}|board={}", d.fire_rating, d.insulation_thickness_m)))).chain(d.cases.iter().filter_map(|c| layout.case(&c.id).map(|pose| (c.id.clone(), format!("{pose:?}"))))).collect()
    }
}

live_scene!(pub mod live = Kit<En1994Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1994Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let kind = if d.structure_kind == semio_s_artifact_norm_en1994_model::StructureKind::Bridge { ("bridge", "Brücke") } else { ("building", "Hochbau") };
    LocalizedCopy::new(
        format!("EN 1994 composite {} model with {} beams, {} columns and {} slabs; worst utilisation {:.0} %.", kind.0, d.beams.len(), d.columns.len(), d.slabs.len(), worst * 100.0),
        format!("EN-1994-Verbundmodell ({}) mit {} Trägern, {} Stützen und {} Decken; größte Ausnutzung {:.0} %.", kind.1, d.beams.len(), d.columns.len(), d.slabs.len(), worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;

#[cfg(test)]
#[path = "🧪️tests/🗂️cases/🦀️.rs"]
mod cases_tests;
