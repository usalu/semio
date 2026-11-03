//! 🎬️ EN 1991 structural scene: the pure projection of the actions snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1991_checks as checks;
extern crate semio_s_artifact_norm_en1991_model as model;
extern crate semio_s_artifact_norm_en1991_scene_fields as fields;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

extern crate semio_s_artifact_norm_en1991_scene_members as members;
extern crate semio_s_artifact_norm_en1991_scene_members_projection as projection;

use projection::{deflection, snow, traffic};
use members::{kit, envelope, wind, gravity, thermal_fire, accidental, collections, silo, wave3, wave3_rows, dynamics};
pub use members::kit::{binding_of_owner, entities_of, entity_dependencies, entity_kind_of, family_of, focus_of_outcome, inert_reference, is_outcome, owner_entity, report_owner, route_of, scalar_owner, scene_report, CheckRoute, EntityKindSpec, DOCUMENT_CHECKS, DRAWN_FAMILIES, ENTITY_KINDS, OUTCOME_FAMILIES};
pub use members::layout;
pub use members::layout::{Layout, Placement};
pub use members::handles;
pub use members::SceneInput;
pub use projection::fragment;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

use model::En1991Snapshot;
use norm::CheckReport;
use scene::projection::{project_parts, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🏋️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_parts::<En1991Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, &scene_report(input.report), input.revision.clone())
}

/// 🧲️ The live-scene contract of EN 1991, delegating to the modules above.
#[derive(Default)]
pub struct En1991Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1991Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1991Kit {
    type Snapshot = En1991Snapshot;

    fn basis(doc: &KitDoc<'_, En1991Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1991, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1991Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1991Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1991Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1991Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1991Snapshot>) -> BTreeMap<String, String> {
        kit::stamps(doc.snapshot, &Layout::of(doc.snapshot, doc.placements))
    }

    fn scalar_owner(field: &str) -> Option<&'static str> {
        scalar_owner(field)
    }

    fn inert_reference(collection: &str, field: &str) -> bool {
        inert_reference(collection, field)
    }
}

live_scene!(pub mod live = Kit<En1991Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities whose row, declared dependencies or check slice it changes; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1991Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, &scene_report(report), &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let counts = (d.floors.len(), d.roofs.len(), d.wind_faces.len(), d.accidental_cases.len());
    kit::bi(|de| {
        if de {
            format!("EN-1991-Einwirkungsmodell mit {} Geschossflächen, {} Dächern, {} Windflächen und {} außergewöhnlichen Fällen; größte Ausnutzung {} %.", counts.0, counts.1, counts.2, counts.3, kit::num(worst * 100.0, 0, true))
        } else {
            format!("EN 1991 actions model with {} floors, {} roofs, {} wind faces and {} accidental cases; worst utilisation {} %.", counts.0, counts.1, counts.2, counts.3, kit::num(worst * 100.0, 0, false))
        }
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;

#[cfg(test)]
#[path = "🧪️tests/🗂️cases/🦀️.rs"]
mod case_law;
