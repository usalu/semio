//! 🎬️ EN 1996 structural scene: the pure projection of the masonry snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1996_model as model;
extern crate semio_s_artifact_norm_en1996_schema as schema;
extern crate semio_s_artifact_norm_en1996_scene_kit as kit;
extern crate semio_s_artifact_norm_en1996_scene_layout as layout;
extern crate semio_s_artifact_norm_en1996_scene_profile as profile;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;

use schema::checks::masonry;

#[path = "🎞️tracks/🦀️.rs"]
pub mod tracks;

#[path = "🧱️walls/🦀️.rs"]
mod walls;

#[path = "🧩️bond/🦀️.rs"]
mod bond;

#[path = "🏋️loads/🦀️.rs"]
mod loads;

#[path = "🔩️reinforcement/🦀️.rs"]
mod reinforcement;

#[path = "🌀️lateral/🦀️.rs"]
mod lateral;

#[path = "🧩️members/🦀️.rs"]
pub mod members;

#[path = "🧭️spreading/🦀️.rs"]
pub mod spreading;

#[path = "🔥️isotherms/🦀️.rs"]
pub mod isotherms;

#[path = "📉️reduction/🦀️.rs"]
pub mod reduction;

#[path = "📐️yield-lines/🦀️.rs"]
mod yield_lines;

#[path = "🏹️arching/🦀️.rs"]
mod arching;

#[path = "🪚️details/🦀️.rs"]
mod details;

#[path = "🥞️diaphragm/🦀️.rs"]
mod diaphragm;

#[path = "🗂️cases/🦀️.rs"]
pub mod cases;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use kit::{entities_of, entity_dependencies, Out};
use layout::Layout;
use scene::projection_kit::Placement;
use model::En1996Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a En1996Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

/// 🧱️ Number of bond units one wall may generate before its bond falls back to the joint lines only.
pub const UNIT_BUDGET: usize = 20_000;

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per wall.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1996Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let mut out = Out::scoped(part);
    walls::project(input, &layout, &mut out);
    bond::project(input, &layout, &mut out);
    loads::project(input, &layout, &mut out);
    reinforcement::project(input, &layout, &mut out);
    members::project(input, &layout, &mut out);
    spreading::project(input, &layout, &mut out);
    isotherms::project(input, &layout, &mut out);
    reduction::project(input, &layout, &mut out);
    yield_lines::project(input, &layout, &mut out);
    arching::project(input, &layout, &mut out);
    details::project(input, &layout, &mut out);
    diaphragm::project(input, &layout, &mut out);
    lateral::project(input, &layout, &mut out);
    tracks::project(input, &layout, &mut out);
    cases::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1996, delegating to the modules above.
#[derive(Default)]
pub struct En1996Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1996Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1996Kit {
    type Snapshot = En1996Snapshot;

    fn basis(doc: &KitDoc<'_, En1996Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1996, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1996Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1996Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1996Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1996Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1996Snapshot>) -> BTreeMap<String, String> {
        let layout = Layout::of(doc.snapshot, doc.placements);
        doc.snapshot.walls.iter().filter_map(|w| layout.pose(&w.id).map(|pose| (w.id.clone(), format!("{pose:?}")))).chain(doc.snapshot.cases.iter().filter_map(|c| layout.case(&c.id).map(|pose| (c.id.clone(), format!("{pose:?}"))))).collect()
    }
}

live_scene!(pub mod live = Kit<En1996Kit>);

/// ⚡️ The scene editor and viewer paint: per-wall fragments from the retained live scene, so an edit re-projects only the walls it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1996Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let openings: usize = d.walls.iter().map(|w| w.openings.len()).sum();
    LocalizedCopy::new(
        format!("EN 1996 masonry model with {} walls, {} openings and {} storeys; worst utilisation {:.0} %.", d.walls.len(), openings, d.storeys, worst * 100.0),
        format!("EN-1996-Mauerwerksmodell mit {} Wänden, {} Öffnungen und {} Geschossen; größte Ausnutzung {:.0} %.", d.walls.len(), openings, d.storeys, worst * 100.0),
    )
}

#[cfg(test)]
extern crate semio_s_artifact_norm_en1996_scene_diagrams as diagrams;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;

#[cfg(test)]
#[path = "🧪️tests/🗂️cases/🦀️.rs"]
mod case_law;
