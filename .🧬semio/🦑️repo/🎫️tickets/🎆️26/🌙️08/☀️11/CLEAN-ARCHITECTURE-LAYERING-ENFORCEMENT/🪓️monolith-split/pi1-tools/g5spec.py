#!/usr/bin/env python3
"""🧱️ Spec of group G5: retained-command (the last WP3 crate that depends only on flipped crates; extension waits for the reactor chain)."""
from lib import *
import groupdrv as G


def add_uses(*lines):
    def fn(text):
        import flipcore as F
        pos = F.header_pos(text)
        return text[:pos] + "".join(l + "\n" for l in lines) + text[pos:]
    return fn


class S(G.Spec):
    name = "g5"
    group = ["retained-command"]
    roots_b = {"retained-command": PLUG + "/🧵️retained-command/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
    hub_units = {"owned_context": "pub(crate) use semio_framework_plugin_decl_app::owned_context;"}
    delete = [PLUG + "/🧵️retained-command/🧬️context/🦀️.rs"]
    skip_files = {PLUG + "/🧵️retained-command/🧬️context/🦀️.rs"}
    skip_mounts = {"owned_context"}
    host_reexport_into = {}
    extra_edits = {
        PLUG + "/🧵️retained-command/🔄️full-operation/🦀️.rs": add_uses("use semio_framework_plugin_operation::{ToolCancellationLeaseRetirement, original_lease_cursor_admit, original_lease_cursor_release};", "use semio_framework_value::retirement::controlled::ControlledRetirement;"),
        PLUG + "/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs": add_uses("use semio_framework_value::retirement::controlled::ControlledRetirement;"),
    }


if __name__ == "__main__":
    G.main(S)
