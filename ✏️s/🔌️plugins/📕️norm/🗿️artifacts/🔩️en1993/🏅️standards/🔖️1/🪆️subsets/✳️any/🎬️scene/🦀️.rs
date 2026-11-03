//! 🎬️ EN 1993 structural scene: the pure projection of the steel snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1993_checks as checks;
extern crate semio_s_artifact_norm_en1993_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

extern crate semio_s_artifact_norm_en1993_scene_members as scene_members;
extern crate semio_s_artifact_norm_en1993_scene_members_projection as projection;

use scene_members::{kit, tracks};
use projection::{detail_sketches, fatigue_views};
pub use scene_members::kit::{EntityKindSpec, ENTITY_KINDS, entities_of, entity_dependencies, entity_kind_of};
pub use scene_members::layout;
pub use scene_members::layout::{Layout, Placement};
pub use scene_members::handles;
pub use scene_members::handles::handles;
pub use scene_members::SceneInput;
pub use projection::fragment;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

use model::En1993Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1993Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧲️ The live-scene contract of EN 1993, delegating to the modules above.
#[derive(Default)]
pub struct En1993Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1993Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1993Kit {
    type Snapshot = En1993Snapshot;

    fn basis(doc: &KitDoc<'_, En1993Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1993, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1993Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1993Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1993Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1993Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1993Snapshot>) -> BTreeMap<String, String> {
        kit::layout_stamps(doc.snapshot, &Layout::of(doc.snapshot, doc.placements))
    }
}

live_scene!(pub mod live = Kit<En1993Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1993Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let counts = (d.members.len(), d.joints.len(), d.plated_panels.len() + d.silo_shells.len() + d.piles.len() + d.tension_components.len() + d.cold_formed_members.len());
    LocalizedCopy::new(
        format!("EN 1993 steel model with {} members, {} joints and {} further elements; worst utilisation {:.0} %.", counts.0, counts.1, counts.2, worst * 100.0),
        format!("EN-1993-Stahlmodell mit {} Bauteilen, {} Anschlüssen und {} weiteren Elementen; größte Ausnutzung {:.0} %.", counts.0, counts.1, counts.2, worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;

#[cfg(test)]
#[path = "🧪️tests/🧮️norm-views/🦀️.rs"]
mod norm_views_tests;
