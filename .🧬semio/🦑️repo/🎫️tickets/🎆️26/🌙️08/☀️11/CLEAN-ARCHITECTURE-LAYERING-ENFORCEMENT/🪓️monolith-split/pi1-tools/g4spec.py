#!/usr/bin/env python3
"""🧱️ Spec of group G4: command and peer (both depend on job only)."""
from lib import *
import groupdrv as G

COMMAND_HISTORY = """#[path = "../../⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs"]
pub(crate) mod command_history_panel_lift;
pub(crate) mod command_history {
    pub use semio_framework_plugin_command::*;
    pub use super::command_history_panel_lift::*;
}"""


class S(G.Spec):
    name = "g4"
    group = ["command", "peer"]
    roots_b = {"command": PLUG + "/🎮️command/🦀️.rs", "peer": PLUG + "/👥️peer/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
    hub_units = {"command_history": COMMAND_HISTORY}
    rehome = [PLUG + "/⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs"]
    host_reexport_into = {"command": "command_history"}


if __name__ == "__main__":
    G.main(S)
