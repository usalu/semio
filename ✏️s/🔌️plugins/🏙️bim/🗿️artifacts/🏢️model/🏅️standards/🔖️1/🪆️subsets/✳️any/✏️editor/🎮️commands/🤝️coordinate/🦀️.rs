//! 🤝️ The coordination commands. `viewClash` frames the pair of a clash in the addressed 3D window and isolates it (window state, never the document); `raiseIssue` writes the issue of a clash with the pair as its
//! elements, the clash it was raised from and a viewpoint framing the clash box (camera, section box, isolated pair); `captureViewpoint` stores the camera, the section box and the isolated elements of the addressed 3D window
//! in an issue and `restoreViewpoint` puts a stored viewpoint back into the window. Running the checks is the `analyseModel` job and the BCF export is `exportModel` with the format `bcf`; selecting the pair is `selectFindings`.

use crate::editor::bim::entities::coordination::latest_moment;
use crate::editor::bim::entities::id_taken;
use crate::editor::bim::kit::{fault, IdMint};
use crate::editor::bim::modes::edit::windows::world;
use crate::editor::bim::panels::coordination::name_of;
use crate::editor::bim::BimDispatchCtx;
use crate::standards::v1::subsets::any::schema::inferences::clash_sets::{Clash, ClashKind};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use crate::{Assigned, ClashRef, Issue, IssuePatch, IssuePriority, IssueStatus, IssueViewpoint, ModelMutation, ModelSnapshot, Point2, Point3, SectionBox, ViewCamera};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🔭️ How far around the clash box the section box of a raised issue reaches, in metres.
pub const SECTION_MARGIN: f64 = 1.0;
/// 🎥️ The direction the camera frames a clash from: the azimuth and the pitch that look at it from the south-east above.
pub const FRAMING: (f64, f64) = (0.8, 0.5);
/// 🎥️ The camera is this many diagonals of the clash box away.
pub const FRAMING_DIAGONALS: f64 = 2.5;
/// 🎥️ The closest the camera frames a clash, in metres.
pub const MIN_DISTANCE: f64 = 4.0;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "view-clash")]
pub struct ViewClash {
    pub first: String,
    pub second: String,
    /// `zoom` frames the pair, `isolate` shows only the pair, `both` does both and `clear` shows everything again.
    pub mode: String,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "raise-issue")]
pub struct RaiseIssue {
    pub set: String,
    pub first: String,
    pub second: String,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "capture-viewpoint")]
pub struct CaptureViewpoint {
    pub issue: String,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "restore-viewpoint")]
pub struct RestoreViewpoint {
    pub issue: String,
}

//#region 🔖️Framing
/// 🔎️ The clash between `first` and `second` (in either order) among the results of the clash sets, the one of `set` when it names one that exists.
pub fn find_clash(instance: registry::Instance<'_>, doc: &ArtifactView<'_, ModelSnapshot>, set: &str, first: &str, second: &str) -> Result<Clash, Fault> {
    let found = registry::try_with_inference(instance, doc.snapshot, |inferred| {
        let matches = |clash: &&Clash| (clash.first == first && clash.second == second) || (clash.first == second && clash.second == first);
        inferred
            .clash_sets
            .iter()
            .filter(|(id, _)| set.is_empty() || id.as_str() == set)
            .find_map(|(_, result)| result.clashes.iter().find(matches).cloned())
    })
    .map_err(|error| fault("bim.clash.inference", format!("the clashes could not be inferred: {error}")))?;
    found.ok_or_else(|| fault("bim.clash.missing", format!("no clash between '{first}' and '{second}'")))
}

/// 🎥️ The orbit camera that frames the box of a clash: centred on it, far enough away to show it whole.
pub fn framing(clash: &Clash) -> ViewCamera {
    let centre = [(clash.min.x + clash.max.x) / 2.0, (clash.min.y + clash.max.y) / 2.0, (clash.min.z + clash.max.z) / 2.0];
    let diagonal = ((clash.max.x - clash.min.x).powi(2) + (clash.max.y - clash.min.y).powi(2) + (clash.max.z - clash.min.z).powi(2)).sqrt();
    ViewCamera { target: Point2 { x: centre[0], y: centre[1] }, target_height: centre[2], azimuth: FRAMING.0, pitch: FRAMING.1, distance: (diagonal * FRAMING_DIAGONALS).max(MIN_DISTANCE) }
}

/// 🔭️ The section box around a clash: its box grown by [`SECTION_MARGIN`].
pub fn section_around(clash: &Clash) -> SectionBox {
    let grow = |point: &Point3, sign: f64| Point3 { x: point.x + sign * SECTION_MARGIN, y: point.y + sign * SECTION_MARGIN, z: point.z + sign * SECTION_MARGIN };
    SectionBox { min: grow(&clash.min, -1.0), max: grow(&clash.max, 1.0) }
}

fn orbit_of(camera: &ViewCamera) -> store::Viewport3dOrbit {
    let (position, target) = camera.eye_and_target();
    store::Viewport3dOrbit { position, target, zoom: 1.0, up: None }
}

fn world_window(ctx: &BimDispatchCtx) -> Result<semio_framework_plugin::ViewModel, Fault> {
    let view = ctx.view.clone().ok_or_else(|| fault("bim.clash.window-required", "this belongs to one open 3D window"))?;
    if ctx.window_kind != world::WINDOW_KIND_ID {
        return Err(fault("bim.clash.world-required", "this belongs to a 3D window"));
    }
    Ok(view)
}

fn box_values(section: &SectionBox) -> Vec<f64> {
    vec![section.min.x, section.min.y, section.min.z, section.max.x, section.max.y, section.max.z]
}

/// 🔭️ The section box a window config holds, none when it holds none.
pub fn box_of(config: &world::config::BimWorldWindowConfig) -> Option<SectionBox> {
    let values = &config.section_box;
    (values.len() == 6).then(|| SectionBox { min: Point3 { x: values[0], y: values[1], z: values[2] }, max: Point3 { x: values[3], y: values[4], z: values[5] } })
}
//#endregion 🔖️Framing

//#region 🔖️Handlers
/// 🤝️ What a coordination command does against the document and the dispatch context of the addressed window.
pub trait Coordinate {
    fn run(&self, doc: &ArtifactView<'_, ModelSnapshot>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault>;
}

/// 🤝️ The one handler of the coordination commands.
pub fn handle<P: Coordinate>(payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    payload.run(doc, ctx)
}

/// 🔭️ `viewClash`: frames and isolates the pair of a clash in the addressed 3D window.
impl Coordinate for ViewClash {
    fn run(&self, doc: &ArtifactView<'_, ModelSnapshot>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        let payload = self;
        let view = world_window(ctx)?;
        let mut config = ctx.world.clone();
        match payload.mode.as_str() {
            "clear" => {
                config.isolated_elements.clear();
                config.section_box.clear();
            }
            "zoom" | "isolate" | "both" => {
                let clash = find_clash(ctx.gestures.as_ref(), doc, "", &payload.first, &payload.second)?;
                if payload.mode != "isolate" {
                    config.camera = orbit_of(&framing(&clash));
                    config.framed = true;
                }
                if payload.mode != "zoom" {
                    config.isolated_elements = vec![clash.first.clone(), clash.second.clone()];
                    config.section_box = box_values(&section_around(&clash));
                }
            }
            other => return Err(fault("bim.clash.mode-unknown", format!("'{other}' is not a clash view (zoom, isolate, both or clear)"))),
        }
        let mut emit = Emit::default();
        emit.window_config_mutations.push(world::config::addressed(&view, config)?);
        Ok(emit)
    }
}

/// 🚩️ `raiseIssue`: writes the issue of a clash.
impl Coordinate for RaiseIssue {
    fn run(&self, doc: &ArtifactView<'_, ModelSnapshot>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        let payload = self;
        let snapshot = doc.snapshot;
        let clash = find_clash(ctx.gestures.as_ref(), doc, &payload.set, &payload.first, &payload.second)?;
        let set = snapshot.clash_sets.iter().find(|(id, _)| payload.set.is_empty() || id.as_str() == payload.set);
        let id = IdMint::new(doc.operation_optional()).mint("issue", |candidate| id_taken(snapshot, candidate));
        let hard = clash.kind == ClashKind::Hard;
        let issue = Issue {
            title: format!("{}: {} / {}", if hard { "Clash" } else { "Clearance" }, name_of(snapshot, &clash.first), name_of(snapshot, &clash.second)),
            description: format!("{} of the clash set \"{}\": {} by {} mm at {:.2}, {:.2}, {:.2}.", if hard { "Hard clash" } else { "Soft clash" }, set.map_or("", |(_, set)| set.name.as_str()), if hard { "interpenetration" } else { "gap" }, (clash.distance.abs() * 1000.0).round(), clash.point.x, clash.point.y, clash.point.z),
            status: IssueStatus::Open,
            priority: if hard { IssuePriority::High } else { IssuePriority::Normal },
            assignee: String::new(),
            author: if snapshot.project.author.trim().is_empty() { "unknown".to_string() } else { snapshot.project.author.clone() },
            created: latest_moment(snapshot),
            labels: vec!["Clash".to_string()],
            elements: vec![clash.first.clone(), clash.second.clone()],
            clash: Some(ClashRef { set: set.map_or_else(|| payload.set.clone(), |(id, _)| id.clone()), first: clash.first.clone(), second: clash.second.clone() }),
            viewpoint: Some(IssueViewpoint { camera: framing(&clash), section: Some(section_around(&clash)), isolate: vec![clash.first.clone(), clash.second.clone()] }),
        };
        Ok(Emit::mutations(vec![ModelMutation::CreateIssue(crate::mutations::create_issue::CreateIssue { id, issue })]))
    }
}

/// 📷️ `captureViewpoint`: stores the camera, the section box and the isolated elements of the addressed 3D window in an issue.
impl Coordinate for CaptureViewpoint {
    fn run(&self, doc: &ArtifactView<'_, ModelSnapshot>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        let payload = self;
        world_window(ctx)?;
        let issue = doc.snapshot.issues.get(&payload.issue).ok_or_else(|| fault("bim.issue.missing", format!("no issue '{}'", payload.issue)))?;
        let camera = ViewCamera::from_eye_and_target(ctx.world.camera.position, ctx.world.camera.target);
        let viewpoint = IssueViewpoint { camera, section: box_of(&ctx.world), isolate: ctx.world.isolated_elements.iter().filter(|id| crate::is_referable(doc.snapshot, id)).cloned().collect() };
        if issue.viewpoint.as_ref() == Some(&viewpoint) {
            return Ok(Emit::default());
        }
        let patch = IssuePatch { viewpoint: Some(Assigned::new(Some(viewpoint))), ..IssuePatch::default() };
        Ok(Emit::mutations(vec![ModelMutation::SetIssue(crate::mutations::set_issue::SetIssue::from_patch(payload.issue.clone(), patch))]))
    }
}

/// 📷️ `restoreViewpoint`: puts the stored viewpoint of an issue into the addressed 3D window.
impl Coordinate for RestoreViewpoint {
    fn run(&self, doc: &ArtifactView<'_, ModelSnapshot>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
        let payload = self;
        let view = world_window(ctx)?;
        let issue = doc.snapshot.issues.get(&payload.issue).ok_or_else(|| fault("bim.issue.missing", format!("no issue '{}'", payload.issue)))?;
        let viewpoint = issue.viewpoint.as_ref().ok_or_else(|| fault("bim.issue.viewpoint-missing", format!("the issue '{}' has no viewpoint", payload.issue)))?;
        let mut config = ctx.world.clone();
        config.camera = orbit_of(&viewpoint.camera);
        config.framed = true;
        config.isolated_elements = viewpoint.isolate.clone();
        config.section_box = viewpoint.section.as_ref().map(box_values).unwrap_or_default();
        let mut emit = Emit::default();
        emit.window_config_mutations.push(world::config::addressed(&view, config)?);
        Ok(emit)
    }
}
//#endregion 🔖️Handlers

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
