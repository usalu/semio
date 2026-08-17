//! 🖱️ 🖱️ VCS play app commands command — `no-operation`.

use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::artifacts::vcs::{op::VcsDemoMutation, VcsSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "no-operation")]
pub struct NoMutation {}

pub fn handle(_payload: &NoMutation, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel};
    use crate::editor::vcs::VcsCommand;

    #[test]
    fn vcs_demo_command_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::NoMutation(NoMutation {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}));
        store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {}));
    }
}
//#endregion 🧪️Tests
