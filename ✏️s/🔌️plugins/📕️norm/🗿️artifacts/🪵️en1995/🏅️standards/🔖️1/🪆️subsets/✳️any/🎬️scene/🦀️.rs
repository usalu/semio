//! 🎬️ EN 1995 structural scene: the pure projection of the timber snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1995_model as model;
extern crate semio_s_artifact_norm_en1995_scene_entities as entities;
extern crate semio_s_artifact_norm_en1995_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_mechanics as mechanics;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
pub use kit::{CheckBinding, CHECK_BINDINGS, EntityKindSpec, ENTITY_KINDS, entities_of, entity_dependencies, entity_kind_of};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};

pub use semio_s_artifact_norm_en1995_scene_facts as facts;

pub use semio_s_artifact_norm_en1995_scene_modes as modes;

#[path = "🪵️members/🦀️.rs"]
mod members;

#[path = "🎞️tracks/🦀️.rs"]
pub mod tracks;

#[path = "🔩️connections/🦀️.rs"]
mod connections;

#[path = "🗂️cases/🦀️.rs"]
pub mod cases;

pub use semio_s_artifact_norm_en1995_scene_diagrams as diagrams;
pub use diagrams::diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::handles;

use kit::Out;
use model::En1995Snapshot;
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements and the revision stamp.
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a En1995Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1995Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: scene::projection::ViewOptions::none() }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let mut out = Out::scoped(part);
    members::project(input, &layout, &mut out);
    connections::project(input, &layout, &mut out);
    cases::project(input, &layout, &mut out);
    entities::project(&entities::Input { document: input.document, report: input.report }, &|id| layout.entity(id).cloned(), &|id, keys| handles::tagged_entity(input.document, &layout, id, keys), &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1995, delegating to the modules above.
#[derive(Default)]
pub struct En1995Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1995Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } }
}

impl SceneKit for En1995Kit {
    type Snapshot = En1995Snapshot;

    fn basis(doc: &KitDoc<'_, En1995Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1995, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1995Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1995Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1995Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1995Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        kit::finish(scene, summary(&input_of(doc, summary_only)));
        projection_kit::tracks::push_combination_track(scene, facts::combination_captions(doc.snapshot));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1995Snapshot>) -> BTreeMap<String, String> {
        let layout = Layout::of(doc.snapshot, doc.placements);
        let members = doc.snapshot.members.iter().filter_map(|e| layout.member(&e.id).map(|pose| (e.id.clone(), format!("{pose:?}"))));
        let connections = doc.snapshot.connections.iter().filter_map(|e| layout.connection(&e.id).map(|origin| (e.id.clone(), format!("{origin:?}"))));
        let cases = doc.snapshot.cases.iter().filter_map(|e| layout.case(&e.id).map(|pose| (e.id.clone(), format!("{pose:?}"))));
        let entities = schema::checks::timber::ENTITY_TABLES.iter().flat_map(|table| schema::checks::timber::entity_ids(doc.snapshot, table)).filter_map(|id| layout.entity(&id).map(|origin| (id.clone(), format!("{origin:?}"))));
        members.chain(connections).chain(cases).chain(entities).collect()
    }
}

live_scene!(pub mod live = Kit<En1995Kit>);

/// ⚡️ The scene editor and viewer paint: per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1995Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision }))
}

fn summary(input: &SceneInput) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    LocalizedCopy::new(
        format!("EN 1995 timber model with {} members, {} connections and {} catalogue cases; worst utilisation {:.0} %.", d.members.len(), d.connections.len(), d.cases.len(), worst * 100.0),
        format!("EN-1995-Holzmodell mit {} Bauteilen, {} Verbindungen und {} Katalogfällen; größte Ausnutzung {:.0} %.", d.members.len(), d.connections.len(), d.cases.len(), worst * 100.0),
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
mod case_law;

#[cfg(test)]
#[path = "🧪️tests/🧩️curves/🦀️.rs"]
mod curve_law;
