//! 🎬️ EN 1998 structural scene: the pure projection of the seismic snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1998_model as model;
extern crate semio_s_artifact_norm_en1998_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

pub use semio_s_artifact_norm_en1998_scene_geometry::{entities_of, entity_dependencies, entity_kind_of, ground_and_bridges, handles, horizontal_elements, kit, layout, tracks, walls, EntityKindSpec, Layout, Out, Placement, SceneInput, ENTITY_KINDS};
pub use semio_s_artifact_norm_en1998_scene_analysis::{assessment, capacity, geotechnical, response, storey_response};

#[path = "../🏢️buildings/🦀️.rs"]
pub mod buildings;

extern crate semio_s_artifact_norm_en1998_scene_structures as structure_views;

#[path = "../🏭️vessels-and-towers/🦀️.rs"]
pub mod vessels_and_towers;

#[path = "../🔩️joint-rows/🦀️.rs"]
pub(crate) mod joint_rows;

#[path = "../🛞️isolation/🦀️.rs"]
pub mod isolation;

pub use model::En1998Snapshot;
pub use norm::CheckReport;
pub use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
pub use scene::projection_kit;
pub use scene::tessellation::Lod;
pub use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
pub use std::collections::BTreeMap;

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1998Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let grips = handles::index(&handles::all(input.document, &layout));
    let site = response::site(input.document);
    let mut out = Out::scoped(part);
    buildings::project(input, &layout, &site, &grips, &mut out);
    joint_rows::project(input, &layout, &grips, &mut out);
    horizontal_elements::project(input, &layout, &site, &grips, &mut out);
    vessels_and_towers::project(input, &layout, &site, &grips, &mut out);
    ground_and_bridges::project(input, &layout, &grips, &mut out);
    isolation::project(input, &layout, &grips, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1998, delegating to the modules above.
#[derive(Default)]
pub struct En1998Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1998Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1998Kit {
    type Snapshot = En1998Snapshot;

    fn basis(doc: &KitDoc<'_, En1998Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1998, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1998Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1998Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1998Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1998Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1998Snapshot>) -> BTreeMap<String, String> {
        kit::layout_stamps(doc.snapshot, &Layout::of(doc.snapshot, doc.placements))
    }
}

live_scene!(pub mod live = Kit<En1998Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1998Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let storeys: usize = d.buildings.iter().map(|b| b.storeys.len()).sum();
    let others = d.bridges.len() + d.silos.len() + d.tanks.len() + d.towers.len() + d.foundations.len() + d.retaining_walls.len();
    LocalizedCopy::new(
        format!("EN 1998 seismic model with {} buildings, {storeys} storeys and {others} further structures; worst utilisation {:.0} %.", d.buildings.len(), worst * 100.0),
        format!("EN-1998-Erdbebenmodell mit {} Geb\u{e4}uden, {storeys} Geschossen und {others} weiteren Bauwerken; gr\u{f6}\u{df}te Ausnutzung {:.0} %.", d.buildings.len(), worst * 100.0),
    )
}

