#!/usr/bin/env python3
"""🧱️ Spec of group G11: tool-run and time-travel (WP5 ranks 41, 42). Their `instance_turns` mounts leave the two roots; the two turns files stay facade modules of hub 7 until `time-travel-turns` and `tool-run-turns` flip."""
from lib import *
import groupdrv as G
import flipcore as F

TT = PLUG + "/⏪️time-travel"
TR = PLUG + "/⏯️tool-run"
HUB7 = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🦀️.rs"
ROOT = PLUG + "/🦀️.rs"

TURNS_MOUNT = """//#region ⏯️TurnsOfWp5
#[path = "../../⏪️time-travel/🧵️instance/🦀️.rs"]
pub(crate) mod time_travel_turns;
#[path = "../../⏯️tool-run/🧵️instance/🦀️.rs"]
pub(crate) mod tool_run_turns;
//#endregion ⏯️TurnsOfWp5
"""


def hub7(text):
    assert "TurnsOfWp5" not in text
    return text.rstrip("\n") + "\n" + TURNS_MOUNT


def root_ext(text):
    old = "(tool_jobs::tool_run::ToolRunExtension, (tool_jobs::time_travel::TimeTravelExtension, ()))"
    assert text.count(old) == 1
    return text.replace(old, "(tool_run_turns::ToolRunExtension, (time_travel_turns::TimeTravelExtension, ()))")


def macros_export(text):
    import re
    for n in ("selected_entry", "selected_entry_mut"):
        assert text.count(f"macro_rules! {n} {{") == 1, n
        text, k = re.subn(r"(?m)^pub(?:\(crate\))? use (?:crate::)?" + n + r";\n", "", text)
        assert k == 1, (n, k)
        text = text.replace(f"macro_rules! {n} {{", f"#[macro_export]\n#[doc(hidden)]\nmacro_rules! {n} {{")
    return text


def request_id(text):
    assert "crate::RequestId" in text
    return text.replace("crate::RequestId", "semio_framework::kernel::RequestId")


def turns_macros(text):
    pos = F.header_pos(text)
    return text[:pos] + "use semio_framework_plugin_tool_run::{selected_entry, selected_entry_mut};\n" + text[pos:]


class S(G.Spec):
    name = "g11"
    group = ["tool-run", "time-travel"]
    roots_b = {"tool-run": TR + "/🦀️.rs", "time-travel": TT + "/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/⏯️wp5-features/🦀️.rs"
    hub_units = {}
    rehome = [TT + "/🧵️instance/🦀️.rs", TR + "/🧵️instance/🦀️.rs"]
    extra_edits = {HUB7: hub7, ROOT: root_ext, TR + "/🦀️.rs": macros_export, TR + "/📮️port/♻️retirement/🧪️tests/🦀️.rs": request_id, TR + "/📮️port/🎟️admission/🧪️tests/🦀️.rs": request_id}


if __name__ == "__main__":
    G.main(S)
