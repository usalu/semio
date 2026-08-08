//! 👁️ Procedural2d play app commands — the show-mode display toggle and canvas pointer/wheel events
//! (config-only or no-ops; never document operations).

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetShowMode
pub mod set_show_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-show-mode")]
    pub struct SetShowMode {
        pub value: String,
    }

    pub fn handle(payload: &SetShowMode, _doc: &DocumentView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Procedural2dConfigMutation::SetShowMode { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetShowMode

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {}

    pub fn handle(_payload: &CanvasPointerDown, _doc: &DocumentView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasPointerDown

//#region 🔖️CanvasPointerMove
pub mod canvas_pointer_move {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-move")]
    pub struct CanvasPointerMove {}

    pub fn handle(_payload: &CanvasPointerMove, _doc: &DocumentView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasPointerMove

//#region 🔖️CanvasPointerUp
pub mod canvas_pointer_up {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-up")]
    pub struct CanvasPointerUp {}

    pub fn handle(_payload: &CanvasPointerUp, _doc: &DocumentView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasPointerUp

//#region 🔖️CanvasWheel
pub mod canvas_wheel {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-wheel")]
    pub struct CanvasWheel {}

    pub fn handle(_payload: &CanvasWheel, _doc: &DocumentView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasWheel

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn set_show_mode_is_config_only() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural2dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wire".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
