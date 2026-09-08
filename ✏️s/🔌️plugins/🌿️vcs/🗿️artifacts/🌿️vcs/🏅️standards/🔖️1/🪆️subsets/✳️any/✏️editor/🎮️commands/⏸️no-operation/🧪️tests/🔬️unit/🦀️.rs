
use super::*;
use crate::editor::vcs::VcsCommand;
use crate::editor::vcs::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel};

#[semio_framework_async_macros::async_test]
async fn vcs_demo_command_op_text_round_trips() {
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::NoMutation(NoMutation {}));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}));
    store::os_store::test_support::assert_op_line_round_trip(&VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {}));
}
