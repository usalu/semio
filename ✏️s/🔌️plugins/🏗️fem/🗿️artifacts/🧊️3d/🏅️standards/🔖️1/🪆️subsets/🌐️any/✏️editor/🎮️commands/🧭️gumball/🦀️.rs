//! 🧭️ Fem3d transform gumball commands — translate, rotate and scale the selection through the gumball TOOL
//! (`🕹️interaction/🧭️gumball`): one dispatch without a `phase` is ONE tool transaction; a host streaming a gesture
//! (the fem3d selection record asks `World3dHost` for `gumballLiveDispatch`) sends `phase: "stream"` ticks that
//! accumulate in the window's ONE open transaction (previewed by both windows — the results window re-solves the
//! moved structure while the drag goes on — never history) until `phase: "commit"` commits the net `move-selection`
//! leaf as one edit or `phase: "abort"` (with a `reason`) drops it with zero trace — plus the per-window handle flags
//! the Transform utility's options rail toggles.

use crate::editor::fem3d::interaction::gumball::{fem3d_gumball_tick, Fem3dGumballMotion, Fem3dGumballTool};
use crate::editor::fem3d::modes::edit::windows::{model as model_window, results as results_window};
use crate::editor::fem3d::Fem3dPlayApp;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
use semio_framework_plugin::{AppOperationContext, ArtifactView, ConfigView, EditorApp, Emit, EphemeralEmit, Fault, NoConfig, NoConfigMutation, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_fem_2d::editor::fem2d::transient::{fem_gumball_drive, FemGumballPhase, FemGumballTransientMutation};

type Fem3dSnapshot = crate::Fem3dSnapshot;

/// 🪟️ The dirty scope every transform dispatch declares: both world bodies (the model shows the moved
/// geometry, the results window re-solves it) and the panels that read the moved entity.
pub fn fem3d_transform_dirty_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: vec![model_window::FEM3D_BODY_MODEL.into(), results_window::FEM3D_BODY_RESULTS.into()],
        panel_bodies: vec![crate::editor::fem3d::panels::artifact::BODY_KEY.into(), crate::editor::fem3d::panels::inspection::BODY_KEY.into()],
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🛠️ The retained route of every gumball verb: builds the dispatch's relative tick on the committed `snapshot`
/// (an id-less payload moves the live `fem3d` selection), drives the dispatching window's gumball tool through its
/// phase against the gesture the FEM gumball transient holds for that window, and publishes the committed
/// transaction as ONE edit stamped with its ref plus the window's next transient.
#[expect(clippy::too_many_arguments, reason = "One retained gumball dispatch: verb, motion, payload ids and phase, plus the retained inputs it reads.")]
pub fn gumball_step(
    verb: &str,
    motion: Fem3dGumballMotion,
    ids: &[String],
    phase: Option<&str>,
    reason: Option<&str>,
    snapshot: &Fem3dSnapshot,
    selected: &[String],
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Fem3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<Fem3dPlayApp>>, Fault> {
    let phase = FemGumballPhase::parse(phase, reason).ok_or_else(|| Fault::from("fem3d.gumball.phase-unknown"))?;
    let tick = fem3d_gumball_tick(snapshot, if ids.is_empty() { selected } else { ids }, motion);
    let window = context.and_then(|context| context.view_state.as_ref()).and_then(|view| view.window_id.clone()).unwrap_or_default();
    let transient = context.map(|context| context.transient.as_ref().clone()).unwrap_or_default();
    let base_revision: String = operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect();
    let drive = fem_gumball_drive::<Fem3dGumballTool>(&transient, &window, verb, phase, tick, &operation.authoring_seed, &base_revision);
    let emit = match drive.committed {
        Some((reference, mutations)) if !operation.authoring_seed.is_empty() => Emit { ui_scope: fem3d_transform_dirty_scope(), ..Emit::commit_transaction(reference, mutations) },
        Some((_, mutations)) => Emit { ui_scope: fem3d_transform_dirty_scope(), ..Emit::mutations(mutations) },
        None => Emit { ui_scope: if drive.transient.is_some() { fem3d_transform_dirty_scope() } else { UiDirtyScope::None }, ..Emit::default() },
    };
    Ok(match drive.transient {
        Some(transient) => ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![FemGumballTransientMutation::Snapshot { transient }], window_transient: Vec::new() } },
        None => ArtifactCommandWorkStep::Complete(emit),
    })
}

/// 🛠️ The unmounted route of a gumball verb (no retained context, so no persisted gesture): a one-shot or a commit at
/// rest is ONE tool transaction; a streamed phase needs the retained route's transient.
fn gumball_once(verb: &str, motion: Fem3dGumballMotion, ids: &[String], phase: Option<&str>, reason: Option<&str>, doc: &ArtifactView<'_, Fem3dSnapshot>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    if !matches!(FemGumballPhase::parse(phase, reason), Some(FemGumballPhase::Once | FemGumballPhase::Commit)) {
        return Err(Fault::from("fem3d.gumball.transient-context-required"));
    }
    let seed = doc.operation_optional().map(|operation| operation.authoring_seed.clone()).unwrap_or_default();
    let tick = fem3d_gumball_tick(doc.snapshot, ids, motion);
    let drive = fem_gumball_drive::<Fem3dGumballTool>(&Default::default(), "", verb, FemGumballPhase::Once, tick, &seed, "");
    Ok(match drive.committed {
        Some((reference, mutations)) if !seed.is_empty() => Emit { ui_scope: fem3d_transform_dirty_scope(), ..Emit::commit_transaction(reference, mutations) },
        Some((_, mutations)) => Emit { ui_scope: fem3d_transform_dirty_scope(), ..Emit::mutations(mutations) },
        None => Emit::default(),
    })
}

//#region 🔖️TranslateSelection
pub mod translate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "translate-selection")]
    pub struct TranslateSelection {
        pub ids: Vec<String>,
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        gumball_once("translateSelection", Fem3dGumballMotion::Translate { dx: payload.dx, dy: payload.dy, dz: payload.dz }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), doc)
    }
}
//#endregion 🔖️TranslateSelection

//#region 🔖️RotateSelection
pub mod rotate_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "rotate-selection")]
    pub struct RotateSelection {
        pub ids: Vec<String>,
        pub ax: f64,
        pub ay: f64,
        pub az: f64,
        pub angle: f64,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        gumball_once("rotateSelection", Fem3dGumballMotion::Rotate { axis: [payload.ax, payload.ay, payload.az], angle: payload.angle }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), doc)
    }
}
//#endregion 🔖️RotateSelection

//#region 🔖️ScaleSelection
pub mod scale_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "scale-selection")]
    pub struct ScaleSelection {
        pub ids: Vec<String>,
        pub sx: f64,
        pub sy: f64,
        pub sz: f64,
        pub phase: Option<String>,
        pub reason: Option<String>,
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        gumball_once("scaleSelection", Fem3dGumballMotion::Scale { sx: payload.sx, sy: payload.sy, sz: payload.sz }, &payload.ids, payload.phase.as_deref(), payload.reason.as_deref(), doc)
    }
}
//#endregion 🔖️ScaleSelection

//#region 🔖️SetTransformGumballFlag
pub mod set_transform_gumball_flag {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "set-transform-gumball-flag")]
    pub struct SetTransformGumballFlag {
        pub flag: String,
        pub pressed: Option<bool>,
    }

    pub fn handle(_payload: &SetTransformGumballFlag, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        Err(Fault::from("fem3d.gumball-flag.window-context-required"))
    }

    /// 🎚️ Toggles one handle flag of the addressed MODEL window's gumball — window config, so the
    /// choice survives a refresh and a split layout keeps one gumball per pane.
    pub fn handle_window(payload: &SetTransformGumballFlag, cfg: &ConfigView<'_, NoConfig>, view: &ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        let window_id = view.window_id.as_deref().ok_or_else(|| Fault::from("fem3d.gumball-flag.window-required"))?;
        let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from("fem3d.gumball-flag.window-stale"))?;
        if kind != model_window::FEM3D_WINDOW_MODEL {
            return Err(Fault::from("fem3d.gumball-flag.window-kind: the transform gumball lives on the model window"));
        }
        let mut next = model_window::config::current(cfg);
        next.gumball.set_flag(&payload.flag, payload.pressed)?;
        Ok(Emit {
            window_config_mutations: vec![model_window::config::addressed_to(window_id, next)],
            ui_scope: UiDirtyScope::Partial { window_bodies: vec![model_window::FEM3D_BODY_MODEL.into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: true, labels: false },
            ..Default::default()
        })
    }
}
//#endregion 🔖️SetTransformGumballFlag

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
