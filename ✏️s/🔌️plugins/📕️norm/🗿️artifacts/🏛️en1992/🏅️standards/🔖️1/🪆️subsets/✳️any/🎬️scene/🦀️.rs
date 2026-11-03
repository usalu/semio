//! 🎬️ EN 1992 structural scene: the pure projection of the reinforced-concrete snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1992_checks as checks;
extern crate semio_s_artifact_norm_en1992_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_artifact_norm_en1992_scene_diagrams as diagram_set;
extern crate semio_s_artifact_norm_en1992_scene_foundation as foundation;
extern crate semio_s_plugin_norm_structural_scene as scene;

pub use foundation::{layout, mechanics, reinforcement, tracks, SceneInput};
pub use foundation::kit::{entity_dependencies, EntityKindSpec, ENTITY_KINDS};
pub use foundation::layout::{Layout, Placement};
use foundation::kit;
pub use diagram_set::{bond, crack_width_moment, grades, integrity, ledger, material_laws, prestress_limit, second_order, shear, ties, torsion_interaction};
use diagram_set::{crack_pattern, creep_curve, fatigue_range, fire_capacity, liquid_pressure, prestress_loss, strut_and_tie};

#[path = "🏛️members/🦀️.rs"]
mod members;

#[path = "🕳️punching/🦀️.rs"]
mod punching;

#[path = "⚓️anchors/🦀️.rs"]
mod anchors;

#[path = "🗂️cases/🦀️.rs"]
pub mod cases;

#[path = "🕹️handles/🦀️.rs"]
mod handle_set;

use kit::Out;
use model::En1992Snapshot;
use norm::CheckReport;
use scene::projection::{canonical_order, merge, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, Diagram2d, EntityRef, Handle, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;
use std::sync::Arc;

/// 📦️ One complete projection: the scene, its diagrams and the handles its entities reference.
#[derive(Clone, Debug, PartialEq)]
pub struct Built {
    pub scene: StructuralScene,
    pub diagrams: Vec<Diagram2d>,
    pub handles: Vec<Handle>,
}

/// 🆔️ Every document entity in outliner order: the members, the anchors and the cases that declare geometry.
pub fn entities_of(document: &En1992Snapshot) -> Vec<EntityRef> {
    kit::entities_where(document, cases::declared)
}

/// 🏁️ The global parts of the merged scene: the kit's legends and layers, then the field tracks of the fire time, the age and the prestress.
fn finish(scene: &mut StructuralScene, description: LocalizedCopy) {
    kit::finish(scene, description);
    tracks::finish(scene);
    prestress_loss::finish(scene);
}

fn run(input: &SceneInput, layout: &Layout, part: Part<'_>) -> Out {
    let mut out = Out::scoped(part);
    members::project(input, layout, &mut out);
    punching::project(input, layout, &mut out);
    anchors::project(input, layout, &mut out);
    shear::project(input, layout, &mut out);
    tracks::project(input, layout, &mut out);
    grades::project(input, layout, &mut out);
    bond::project(input, layout, &mut out);
    prestress_loss::project(input, layout, &mut out);
    fatigue_range::project(input, layout, &mut out);
    strut_and_tie::project(input, layout, &mut out);
    cases::project(input, layout, &mut out);
    out
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    projection_kit::highlight_owners(run(input, &Layout::of(input.document, input.placements), part).into_fragment(), input.report, part)
}

/// 🎬️ The scene, diagrams and handles of one document revision in a single deterministic pass (the three accessors below recompute; hosts call this once); the scene is the same fragment the live scene assembles per entity.
pub fn build(input: &SceneInput) -> Built {
    let layout = Layout::of(input.document, input.placements);
    let mut out = run(input, &layout, Part::All);
    let handles = std::mem::take(&mut out.handles);
    let mut scene = merge(&[Arc::new(projection_kit::highlight_owners(out.into_fragment(), input.report, Part::All))], input.revision.clone());
    canonical_order(&mut scene, &entities_of(input.document));
    finish(&mut scene, summary(input));
    Built { scene, diagrams: diagram_set::diagrams(input, &layout), handles }
}

/// 🧲️ The live-scene contract of EN 1992, delegating to the modules above.
#[derive(Default)]
pub struct En1992Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1992Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1992Kit {
    type Snapshot = En1992Snapshot;

    fn basis(doc: &KitDoc<'_, En1992Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1992, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1992Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1992Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1992Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1992Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1992Snapshot>) -> BTreeMap<String, String> {
        let layout = Layout::of(doc.snapshot, doc.placements);
        let members = doc.snapshot.members.iter().filter_map(|e| layout.member(&e.id).map(|pose| (e.id.clone(), format!("{pose:?}"))));
        let anchors = doc.snapshot.anchors.iter().filter_map(|e| layout.anchor(&e.id).map(|origin| (e.id.clone(), format!("{origin:?}"))));
        let cases = doc.snapshot.cases.iter().filter_map(|e| layout.case(&e.id).map(|pose| (e.id.clone(), format!("{pose:?}"))));
        members.chain(anchors).chain(cases).collect()
    }
}

live_scene!(pub mod live = Kit<En1992Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1992Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

/// 🎬️ The validated, locale-free scene of one document revision.
pub fn project(input: &SceneInput) -> StructuralScene {
    build(input).scene
}

/// 📈️ The `Diagram2d`s of one document at default placements; with a `focus` entity id only the diagrams of that entity (derived ids such as `beam:bar-0` belong to `beam`), all of them when `None`.
pub fn diagrams(document: &En1992Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1992, document.annex))
}

fn diagrams_unbased(document: &En1992Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let input = SceneInput { document, report, placements: &[], revision: SceneRevision { document_seq: 0, report_seq: 0 } };
    let mut all = diagram_set::diagrams(&input, &Layout::of(document, &[]));
    all.extend(document.cases.iter().filter_map(|case| projection_kit::design_case::case_diagram(&case.id, &LocalizedCopy::new(&case.label_en, &case.label_de), report, None)));
    let Some(base) = focus.map(|id| id.split_once(':').map_or(id, |(base, _)| base)) else { return all };
    all.into_iter().filter(|d| d.focus.as_ref().is_some_and(|f| f.id == base)).collect()
}

/// 🕹️ The drag handles of one document revision.
pub fn handles(input: &SceneInput) -> Vec<Handle> {
    build(input).handles
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    LocalizedCopy::new(
        format!("EN 1992 concrete model with {} members and {} anchors; worst utilisation {:.0} %.", d.members.len(), d.anchors.len(), worst * 100.0),
        format!("EN-1992-Betonmodell mit {} Bauteilen und {} Dübeln; größte Ausnutzung {:.0} %.", d.members.len(), d.anchors.len(), worst * 100.0),
    )
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "🧪️tests/🔬️live/🦀️.rs"]
mod live_law;

#[cfg(test)]
#[path = "🧪️tests/🔭️design-views/🦀️.rs"]
mod design_views;

#[cfg(test)]
#[path = "🧪️tests/🪝️detailing/🦀️.rs"]
mod detailing;

#[cfg(test)]
#[path = "🧪️tests/🗂️cases/🦀️.rs"]
mod case_law;

#[cfg(test)]
#[path = "🧪️tests/🧵️grade-families/🦀️.rs"]
mod grade_families;
