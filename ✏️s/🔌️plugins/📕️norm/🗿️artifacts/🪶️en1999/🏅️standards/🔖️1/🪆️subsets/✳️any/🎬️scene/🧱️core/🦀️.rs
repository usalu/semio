//! 🎬️ EN 1999 structural scene: the pure projection of the aluminium snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1999_checks as checks;
extern crate semio_s_artifact_norm_en1999_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "../🧰️kit/🦀️.rs"]
pub mod kit;
pub use kit::{CheckRoute, EntityKindSpec, Router, Rows, DRAWN_FAMILIES, ENTITY_KINDS, OUTCOME_FAMILIES, entities_of, entity_dependencies, entity_kind_of, focus_of_outcome, is_document_check, is_drawn_family, owner_entity, route_of};

#[path = "../📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};

#[path = "../🧮️facts/🦀️.rs"]
pub mod facts;

#[path = "../🌀️eigen-modes/🦀️.rs"]
pub mod eigen;

#[path = "../🎞️tracks/🦀️.rs"]
pub mod tracks;

#[path = "../🏗️members/🦀️.rs"]
pub mod members;

#[path = "../🦴️web-stiffeners/🦀️.rs"]
pub mod web_stiffeners;

#[path = "../🔗️connections/🦀️.rs"]
pub mod connections;

#[path = "../🥫️sheets-and-shells/🦀️.rs"]
pub mod sheets_and_shells;

#[path = "../🔥️fire-and-fatigue/🦀️.rs"]
pub mod fire_and_fatigue;

#[path = "../🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

pub use kit::Out;
pub use model::En1999Snapshot;
pub use norm::CheckReport;
pub use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
pub use scene::projection_kit;
pub use scene::tessellation::Lod;
pub use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
pub use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a En1999Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1999Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let mut out = Out::scoped(part).routed(Router::of(input.document, input.report));
    members::project(input, &layout, &mut out);
    web_stiffeners::project(input, &layout, &mut out);
    connections::project(input, &layout, &mut out);
    sheets_and_shells::project(input, &layout, &mut out);
    fire_and_fatigue::project(input, &layout, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1999, delegating to the modules above.
#[derive(Default)]
pub struct En1999Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1999Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1999Kit {
    type Snapshot = En1999Snapshot;

    fn basis(doc: &KitDoc<'_, En1999Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1999, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1999Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1999Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1999Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1999Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
        projection_kit::tracks::push_combination_track(scene, tracks::combination_captions(doc.snapshot));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1999Snapshot>) -> BTreeMap<String, String> {
        kit::layout_stamps(doc.snapshot, &Layout::of(doc.snapshot, doc.placements))
    }
}

live_scene!(pub mod live = Kit<En1999Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1999Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let counts = (d.members.len(), d.connections.len(), d.cold_formed.len() + d.shells.len());
    LocalizedCopy::new(
        format!("EN 1999 aluminium model with {} members, {} connections and {} further elements; worst utilisation {:.0} %.", counts.0, counts.1, counts.2, worst * 100.0),
        format!("EN-1999-Aluminiummodell mit {} Bauteilen, {} Anschlüssen und {} weiteren Elementen; größte Ausnutzung {:.0} %.", counts.0, counts.1, counts.2, worst * 100.0),
    )
}

