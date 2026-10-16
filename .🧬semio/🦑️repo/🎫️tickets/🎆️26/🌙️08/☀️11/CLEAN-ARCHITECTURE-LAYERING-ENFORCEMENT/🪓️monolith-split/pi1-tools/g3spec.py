#!/usr/bin/env python3
"""🧱️ Spec of group G3: job. The still-monolithic children of the old `tool_jobs` module (tool-run, tool-machine, time-travel, the operation-progress leftover) move to hub level and the composed `tool_jobs` module re-exports them with the job crate."""
from lib import *
import groupdrv as G

TOOL_JOBS = """#[path = "../../⏳️operation-progress/🦀️.rs"]
pub(crate) mod tool_jobs_operation_progress;
#[path = "../../⏯️tool-run/🦀️.rs"]
pub(crate) mod tool_jobs_tool_run;
#[path = "../../🛠️tool-machine/🦀️.rs"]
pub(crate) mod tool_jobs_tool_machine;
#[path = "../../⏪️time-travel/🦀️.rs"]
pub(crate) mod tool_jobs_time_travel;
#[cfg(test)]
#[path = "../../🧪️tests/🔬️tool-run/🦀️.rs"]
mod tool_run_tests;
pub(crate) mod tool_jobs {
    pub use semio_framework_plugin_job::*;
    pub(crate) use super::tool_jobs_operation_progress as operation_progress;
    pub(crate) use super::tool_jobs_tool_run as tool_run;
    pub(crate) use super::tool_jobs_tool_machine as tool_machine;
    pub(crate) use super::tool_jobs_time_travel as time_travel;
    pub use tool_run::{is_tool_run_action_id, ToolRunActionOutcome, ToolRunDriver, ToolRunLedger, ToolRunTickReceipt};
    pub use tool_machine::{gesture_emit, ScrubTag, ToolMachineRuntime, ToolTag, TypingTag};
    pub use time_travel::{history_mutation_entry, HistoryMutationPage, HistoryMutationPages, SupersedeLedger, SupersedeRecord, SupersedeRole, TimeTravelActionOutcome, TimeTravelActionRefusal, TimeTravelEditor, TimeTravelEditorPanel, TimeTravelLedger, TimeTravelPanel, HISTORY_ROW_MUTATION_ROWS, TIME_TRAVEL_TURN_WALL_US};
}"""

RESERVED_JOBS = """#[path = "../../🧵️retained-command/🛬️lifted/🏛️reserved/🦀️.rs"]
pub(crate) mod reserved_route_capacity_lift;
pub(crate) mod reserved_jobs {
    pub use semio_framework_plugin_job::reserved_jobs::*;
    pub use super::reserved_route_capacity_lift::*;
}"""


class S(G.Spec):
    name = "g3"
    group = ["job"]
    roots_b = {"job": PLUG + "/👷️job/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
    hub_units = {"tool_jobs": TOOL_JOBS, "reserved_jobs": RESERVED_JOBS}
    rehome = [PLUG + "/⏳️operation-progress/🦀️.rs", PLUG + "/⏯️tool-run/🦀️.rs", PLUG + "/🛠️tool-machine/🦀️.rs", PLUG + "/⏪️time-travel/🦀️.rs"]
    host_reexport_into = {"job": "tool_jobs"}


if __name__ == "__main__":
    G.main(S)
