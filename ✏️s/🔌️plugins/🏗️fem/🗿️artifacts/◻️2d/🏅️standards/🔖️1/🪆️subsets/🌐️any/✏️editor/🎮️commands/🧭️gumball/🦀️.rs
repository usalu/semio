//! 🧭️ Fem2d transform gumball commands — translate, rotate, scale selection with coalesced undo steps.

use crate::editor::fem2d::interaction::gumball::{fem2d_rotate_selection_mutations, fem2d_scale_selection_mutations, fem2d_translate_selection_mutations, set_gumball_flag};
use crate::editor::fem2d::modes::edit::windows::{model as model_window, results as results_window};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation, ViewModel};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

const COALESCE_TRANSLATE: &str = "gumball-translate";
const COALESCE_ROTATE: &str = "gumball-rotate";
const COALESCE_SCALE: &str = "gumball-scale";

fn fem2d_gumball_canvas_emit(mutations: Vec<Fem2dMutation>, coalesce_key: &str) -> Emit<Fem2dMutation, NoConfigMutation> {
    Emit {
        artifact_mutations: mutations,
        coalesce_key: Some(coalesce_key.into()),
        ui_scope: UiDirtyScope::Partial {
            window_bodies: vec![model_window::BODY_KEY.into(), results_window::BODY_KEY.into()],
            panel_bodies: Vec::new(),
            utilities: false,
            tools: false,
            engagements: false,
            measures: false,
            labels: false,
        },
        ..Default::default()
    }
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

    pub fn handle(payload: &TranslateSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        let mutations = fem2d_translate_selection_mutations(doc.snapshot, &payload.ids, payload.dx, payload.dy);
        if mutations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(fem2d_gumball_canvas_emit(mutations, COALESCE_TRANSLATE))
        }
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

    pub fn handle(payload: &RotateSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        let mutations = fem2d_rotate_selection_mutations(doc.snapshot, &payload.ids, payload.angle);
        if mutations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(fem2d_gumball_canvas_emit(mutations, COALESCE_ROTATE))
        }
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

    pub fn handle(payload: &ScaleSelection, doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        let mutations = fem2d_scale_selection_mutations(doc.snapshot, &payload.ids, payload.sx, payload.sy);
        if mutations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(fem2d_gumball_canvas_emit(mutations, COALESCE_SCALE))
        }
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

    pub fn handle(_payload: &SetTransformGumballFlag, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        Err(Fault::from("fem2d.gumball-flag.window-context-required"))
    }

    pub fn handle_window(payload: &SetTransformGumballFlag, view: &ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
        let window_id = view.window_id.clone().ok_or_else(|| Fault::from("fem2d.gumball-flag.window-required"))?;
        set_gumball_flag(&window_id, &payload.flag, payload.pressed);
        Ok(Emit {
            ui_scope: UiDirtyScope::Partial {
                window_bodies: vec![model_window::BODY_KEY.into(), results_window::BODY_KEY.into()],
                panel_bodies: Vec::new(),
                utilities: false,
                tools: false,
                engagements: false,
                measures: true,
                labels: false,
            },
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
