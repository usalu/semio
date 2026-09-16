//! 🧭️ Fem3d transform gumball commands — translate, rotate, scale the live selection with coalesced
//! undo steps, and the per-window handle flags the Transform utility's options rail toggles.
//!
//! The host drags its gumball and, because the fem3d selection record asks for `gumballLiveDispatch`,
//! dispatches one INCREMENTAL step per pointer move — each step lands here as a delta from the pose
//! the previous step already committed, spelled as whole-record `ReplaceNode`/`ReplaceSolid`
//! mutations on a coalesce key so the whole drag is ONE undo step, and refreshes both window bodies
//! so the results window re-solves the moved structure while the drag is still going.

use crate::editor::fem3d::interaction::gumball::{fem3d_rotate_selection_mutations, fem3d_scale_selection_mutations, fem3d_translate_selection_mutations};
use crate::editor::fem3d::modes::edit::windows::{model as model_window, results as results_window};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

pub const COALESCE_TRANSLATE: &str = "gumball-translate";
pub const COALESCE_ROTATE: &str = "gumball-rotate";
pub const COALESCE_SCALE: &str = "gumball-scale";

/// 🪟️ The dirty scope every transform step declares: both world bodies (the model shows the moved
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

fn fem3d_gumball_emit(mutations: Vec<Fem3dMutation>, coalesce_key: &str) -> Emit<Fem3dMutation, NoConfigMutation> {
    if mutations.is_empty() {
        return Emit::default();
    }
    Emit { artifact_mutations: mutations, coalesce_key: Some(coalesce_key.into()), ui_scope: fem3d_transform_dirty_scope(), ..Default::default() }
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
    }

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        Ok(fem3d_gumball_emit(fem3d_translate_selection_mutations(doc.snapshot, &payload.ids, [payload.dx, payload.dy, payload.dz]), COALESCE_TRANSLATE))
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
    }

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        Ok(fem3d_gumball_emit(fem3d_rotate_selection_mutations(doc.snapshot, &payload.ids, [payload.ax, payload.ay, payload.az], payload.angle), COALESCE_ROTATE))
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
    }

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        Ok(fem3d_gumball_emit(fem3d_scale_selection_mutations(doc.snapshot, &payload.ids, [payload.sx, payload.sy, payload.sz]), COALESCE_SCALE))
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

//#region 🔖️TransformBrackets
/// 🧲️ `transformBegin`/`transformEnd` are the host's brackets around one gumball drag. The drag itself
/// carries no app-side session — every pose lands as an incremental `translateSelection`/
/// `rotateSelection`/`scaleSelection` above — so both brackets deliberately complete EMPTY; they are
/// declared so the `World3dHost` may dispatch them without the shell refusing an undeclared action.
pub mod transform_begin {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "transform-begin")]
    pub struct TransformBegin {}

    pub fn handle(_payload: &TransformBegin, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        Ok(Emit { ui_scope: UiDirtyScope::None, ..Default::default() })
    }
}

pub mod transform_end {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "transform-end")]
    pub struct TransformEnd {}

    pub fn handle(_payload: &TransformEnd, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
        Ok(Emit { ui_scope: UiDirtyScope::None, ..Default::default() })
    }
}
//#endregion 🔖️TransformBrackets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
