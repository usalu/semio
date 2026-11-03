//! 🎬️ EN 1997 structural scene: the pure projection of the geotechnical snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1997_checks as checks;
extern crate semio_s_artifact_norm_en1997_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "../🧰️kit/🦀️.rs"]
pub mod kit;
pub use kit::{entities_of, entity_dependencies, entity_kind_of, EntityKindSpec, ENTITY_KINDS};

#[path = "../🧮️mechanics/🦀️.rs"]
pub mod mechanics;

#[path = "../📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};

#[path = "../🪨️ground/🦀️.rs"]
pub mod ground;

#[path = "../🦶️footings/🦀️.rs"]
pub mod footings;

#[path = "../🧱️walls/🦀️.rs"]
pub mod walls;

#[path = "../⛰️slopes/🦀️.rs"]
pub mod slopes;

#[path = "../🏗️piles/🦀️.rs"]
pub mod piles;

#[path = "../💧uplift/🦀️.rs"]
pub mod uplift;

#[path = "../🧩️entities/🦀️.rs"]
pub mod entities;

#[path = "../🗂️cases/🦀️.rs"]
pub mod cases;

#[path = "../🕳️boreholes/🦀️.rs"]
pub mod boreholes;

#[path = "../🛡️safety/🦀️.rs"]
pub mod safety;

#[path = "../🎞️tracks/🦀️.rs"]
pub mod tracks;

#[path = "../📉️settlement/🦀️.rs"]
pub mod settlement;

#[path = "../🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

pub use kit::Out;
pub use model::En1997Snapshot;
pub use norm::CheckReport;
pub use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
pub use scene::projection_kit;
pub use scene::tessellation::Lod;
pub use scene::{live_scene, EntityRef, Handle, LocalizedCopy, SceneRevision, StructuralScene};
pub use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone, Copy)]
pub struct SceneInput<'a> {
    pub document: &'a En1997Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

pub struct Ctx<'a> {
    pub input: &'a SceneInput<'a>,
    pub layout: &'a Layout,
    pub handles: &'a [Handle],
}

/// 🌍️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1997Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let grips = handles::all(input.document, &layout);
    let ctx = Ctx { input, layout: &layout, handles: &grips };
    let mut out = Out::scoped(part);
    ground::project(&ctx, &mut out);
    if out.own("units", kit::UNIT_WATER) {
        ground::water(&ctx, &mut out);
    }
    footings::project(&ctx, &mut out);
    walls::project(&ctx, &mut out);
    slopes::project(&ctx, &mut out);
    piles::project(&ctx, &mut out);
    uplift::project(&ctx, &mut out);
    entities::embedded_walls(&ctx, &mut out);
    entities::seepage(&ctx, &mut out);
    entities::pore_pressure(&ctx, &mut out);
    entities::hydraulic_gradient(&ctx, &mut out);
    entities::heave_prism(&ctx, &mut out);
    entities::flow_arrows(&ctx, &mut out);
    entities::anchors(&ctx, &mut out);
    entities::pile_groups(&ctx, &mut out);
    entities::lab_tests(&ctx, &mut out);
    entities::layer_waters(&ctx, &mut out);
    entities::characteristic_bases(&ctx, &mut out);
    cases::project(&ctx, &mut out);
    boreholes::project(&ctx, &mut out);
    safety::project(&ctx, &mut out);
    if out.own("units", kit::UNIT_SETTLEMENT) {
        settlement::project(&ctx, &mut out);
    }
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1997, delegating to the modules above.
#[derive(Default)]
pub struct En1997Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1997Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1997Kit {
    type Snapshot = En1997Snapshot;

    fn basis(doc: &KitDoc<'_, En1997Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1997, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1997Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1997Snapshot, id: &str) -> Vec<String> {
        if id.starts_with("unit-") { kit::unit_dependencies(snapshot, id) } else { entity_dependencies(snapshot, id) }
    }

    fn fragment(doc: &KitDoc<'_, En1997Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1997Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        let layout = Layout::of(doc.snapshot, doc.placements);
        kit::finish(scene, &layout, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1997Snapshot>) -> BTreeMap<String, String> {
        kit::layout_stamps(doc.snapshot, &Layout::of(doc.snapshot, doc.placements))
    }
}

live_scene!(pub mod live = Kit<En1997Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1997Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let counts = (d.layers.len(), d.footings.len(), d.piles.len() + d.pile_groups.len(), d.retaining_walls.len() + d.embedded_walls.len(), d.slopes.len());
    LocalizedCopy::new(
        format!("EN 1997 ground model with {} layers, {} footings, {} pile groups, {} walls and {} slopes; worst utilisation {:.0} %.", counts.0, counts.1, counts.2, counts.3, counts.4, worst * 100.0),
        format!("EN-1997-Baugrundmodell mit {} Schichten, {} Fundamenten, {} Pfahlgruppen, {} Wänden und {} Böschungen; größte Ausnutzung {:.0} %.", counts.0, counts.1, counts.2, counts.3, counts.4, worst * 100.0),
    )
}

