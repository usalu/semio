//! 🖱️ VCS play app commands — the history-graph canvas pointer/wheel events and the explicit no-op.
//!
//! All five commands are pure surface plumbing: they exist so the host has a command to dispatch for
//! every canvas gesture, but none of them mutate document or config state.

use crate::apps::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️NoMutation
pub mod no_operation {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "no-operation")]
    pub struct NoMutation {}

    pub fn handle(_payload: &NoMutation, _doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️NoMutation

//#region 🔖️CanvasPointerDown
pub mod canvas_pointer_down {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "canvas-pointer-down")]
    pub struct CanvasPointerDown {}

    pub fn handle(_payload: &CanvasPointerDown, _doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
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

    pub fn handle(_payload: &CanvasPointerMove, _doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
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

    pub fn handle(_payload: &CanvasPointerUp, _doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
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

    pub fn handle(_payload: &CanvasWheel, _doc: &DocumentView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        Ok(Emit::default())
    }
}
//#endregion 🔖️CanvasWheel

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::VcsCommand;

    #[test]
    fn vcs_demo_command_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::NoMutation(no_operation::NoMutation {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {}));
    }
}
//#endregion 🧪️Tests
