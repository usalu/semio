//! 🎬️ EN 1990 structural scene: the pure projection of the basis-of-design snapshot and its check report to the locale-free `StructuralScene`, its `Diagram2d` set and its drag handles. See [`semio_s_plugin_norm_structural_scene`].

extern crate semio_s_artifact_norm_en1990_checks as checks;
extern crate semio_s_artifact_norm_en1990_inferences as inferences;
extern crate semio_s_artifact_norm_en1990_model as model;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🧰️kit/🦀️.rs"]
mod kit;
pub use kit::{EntityKindSpec, CheckRoute, CHECK_OWNERS, COMBINATION_ENTITY, DOCUMENT_CHECKS, ENTITY_KINDS, check_owner, entities_of, entity_dependencies, entity_kind_of, focus_of_outcome, owner_entity, route_of};

#[path = "📍️layout/🦀️.rs"]
pub mod layout;
pub use layout::{Layout, Placement};

#[path = "🧮️combinations/🦀️.rs"]
pub mod combinations;

#[path = "🎞️tracks/🦀️.rs"]
mod dynamics;

#[path = "🏗️members/🦀️.rs"]
mod members;

#[path = "⚓️actions/🦀️.rs"]
mod actions;

#[path = "🌈️gamma-bands/🦀️.rs"]
mod gamma_bands;

#[path = "📊️analyses/🦀️.rs"]
mod analyses;

#[path = "🗂️cases/🦀️.rs"]
pub mod cases;

#[path = "📈️diagrams/🦀️.rs"]
pub mod diagrams;
pub use diagrams::diagrams;

#[path = "🕹️handles/🦀️.rs"]
pub mod handles;
pub use handles::{handles, COMBINATION_PATH, TRAFFIC_GROUP_PATH};
pub use actions::{present_groups, traffic_group_at};

use kit::Out;
use model::{En1990Snapshot, LoadGroup};
use norm::CheckReport;
use scene::projection::{project_whole, Fragment, Kit, KitDoc, Part, ProjectionContext, SceneKit, ViewKey, ViewOptions, ViewValue};
use scene::projection_kit;
use scene::tessellation::Lod;
use scene::{live_scene, EntityRef, LocalizedCopy, SceneRevision, StructuralScene};
use std::collections::BTreeMap;

/// 🎯️ Everything one projection reads: the document, its settled report, the persisted placements, the revision stamp and the id of the active combination column (`None` selects the governing STR/GEO combination).
#[derive(Clone)]
pub struct SceneInput<'a> {
    pub document: &'a En1990Snapshot,
    pub report: &'a CheckReport,
    pub placements: &'a [Placement],
    pub revision: SceneRevision,
    pub combination: Option<&'a str>,
    pub traffic_group: Option<LoadGroup>,
}

/// 👁️ The typed view options of an input: its combination column id and its traffic group id.
pub fn view_of(combination: Option<&str>, traffic_group: Option<LoadGroup>) -> ViewOptions {
    ViewOptions::default().with_some(ViewKey::Combination, combination.map(|id| ViewValue::Id(id.to_owned()))).with_some(ViewKey::TrafficGroup, traffic_group.map(|group| ViewValue::Id(group.as_str().to_owned())))
}

/// 🧭️ The view options of the live scene against the document: the position of the combination (`CombinationItem`) becomes the id of that column and the position of the traffic group (`TrafficGroupItem`) the id of that group; explicit ids win, positions outside the document are dropped, so a stale lane follows the governing combination.
pub fn resolved_view(document: &En1990Snapshot, view: &ViewOptions) -> ViewOptions {
    let columns = combinations::columns(document);
    let combination = view.combination().map(str::to_owned).or_else(|| view.combination_item().and_then(|at| columns.get(at as usize)).map(|column| column.id.clone()));
    let group = view.traffic_group().and_then(LoadGroup::from_id).or_else(|| traffic_group_at(document, view.traffic_group_item()));
    let resolved = view_of(combination.as_deref(), group).with_some(ViewKey::Track, view.track().map(|track| ViewValue::Id(track.to_owned())));
    match view.track() {
        Some(track) => resolved.with_some(ViewKey::TrackValue, view.track_value(track).map(ViewValue::Number)),
        None => resolved,
    }
}

/// 🎬️ The complete scene of one document revision: deterministic, locale-free, validated by the golden and law tests; the same fragments the live scene assembles per entity.
pub fn project(input: &SceneInput) -> StructuralScene {
    project_whole::<En1990Kit>(&KitDoc { snapshot: input.document, placements: input.placements, view: &view_of(input.combination, input.traffic_group) }, input.report, input.revision.clone())
}

/// 🧩️ The part of the scene of `input` that belongs to `part`; `input.report` may be restricted to the rows the part reads.
pub fn fragment(input: &SceneInput, part: Part<'_>) -> Fragment {
    let layout = Layout::of(input.document, input.placements);
    let columns = combinations::columns(input.document);
    let active = combinations::active(&columns, input.combination);
    let mut out = Out::scoped(part);
    members::project(input, &layout, &columns, active, &mut out);
    gamma_bands::project(input, &layout, &columns, active, &mut out);
    actions::project(input, &layout, active, &mut out);
    analyses::project(input, &layout, &mut out);
    cases::project(input, &layout, &mut out);
    dynamics::project(&columns, active, &mut out);
    scene::projection_kit::highlight_owners(out.into_fragment(), input.report, part)
}

/// 🧲️ The live-scene contract of EN 1990, delegating to the modules above.
#[derive(Default)]
pub struct En1990Kit;

fn input_of<'a>(doc: &KitDoc<'a, En1990Snapshot>, report: &'a CheckReport) -> SceneInput<'a> {
    SceneInput { document: doc.snapshot, report, placements: doc.placements, revision: SceneRevision { document_seq: 0, report_seq: 0 }, combination: doc.view.combination(), traffic_group: doc.view.traffic_group().and_then(LoadGroup::from_id) }
}

impl SceneKit for En1990Kit {
    type Snapshot = En1990Snapshot;

    fn basis(doc: &KitDoc<'_, En1990Snapshot>) -> Option<scene::projection::Basis> {
        Some(scene::projection::Basis::new(norm::NormFamilyId::En1990, doc.snapshot.annex))
    }

    fn entities(snapshot: &En1990Snapshot) -> Vec<EntityRef> {
        entities_of(snapshot)
    }

    fn dependencies(snapshot: &En1990Snapshot, id: &str) -> Vec<String> {
        entity_dependencies(snapshot, id)
    }

    fn fragment(doc: &KitDoc<'_, En1990Snapshot>, rows: &CheckReport, part: Part<'_>) -> Fragment {
        fragment(&input_of(doc, rows), part)
    }

    fn finish(doc: &KitDoc<'_, En1990Snapshot>, summary_only: &CheckReport, scene: &mut StructuralScene) {
        let input = input_of(doc, summary_only);
        kit::finish(scene, summary(&input, combinations::columns(doc.snapshot).len()));
    }

    fn layout_stamps(doc: &KitDoc<'_, En1990Snapshot>) -> BTreeMap<String, String> {
        let layout = Layout::of(doc.snapshot, doc.placements);
        layout.members().map(|(id, pose)| (id.to_owned(), format!("{pose:?}"))).chain(layout.case_poses().map(|(id, pose)| (id.to_owned(), format!("{pose:?}")))).collect()
    }
}

live_scene!(pub mod live = Kit<En1990Kit>);

/// ⚡️ The scene editor and viewer paint under the typed `view` (the ephemeral view lane): per-entity fragments from the retained live scene, so an edit re-projects only the entities it touches; a projection the validator refuses falls back to the plain pass.
pub fn live_scene_of(document: &En1990Snapshot, report: &CheckReport, view: &ViewOptions) -> StructuralScene {
    let placements = projection_kit::placements_of(document);
    let revision = projection_kit::revision_of(document, &placements, report);
    let ctx = ProjectionContext::new(revision.document_seq, Lod::Lod1);
    let view = resolved_view(document, view);
    live::scene(&KitDoc { snapshot: document, placements: &placements, view: &view }, report, &ctx).unwrap_or_else(|_| project(&SceneInput { document, report, placements: &placements, revision, combination: view.combination(), traffic_group: view.traffic_group().and_then(LoadGroup::from_id) }))
}

fn summary(input: &SceneInput, combinations: usize) -> LocalizedCopy {
    let d = input.document;
    let worst = input.report.summary.worst_utilization;
    let actions = d.permanents.len() + d.variables.len() + d.accidentals.len() + d.seismics.len();
    let analyses = d.fatigue_spectra.len() + d.reliability_analyses.len() + d.test_series.len();
    LocalizedCopy::new(
        format!("EN 1990 basis of design with {} members, {actions} actions, {analyses} analysis rows and {combinations} combinations; worst utilisation {:.0} %.", d.members.len(), worst * 100.0),
        format!("EN-1990-Tragwerksplanung mit {} Bauteilen, {actions} Einwirkungen, {analyses} Analysezeilen und {combinations} Kombinationen; größte Ausnutzung {:.0} %.", d.members.len(), worst * 100.0),
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
