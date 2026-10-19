import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl, deps
R = rgl.ROOT
T = R + "/🧰️framework/🔨️modules/⏯️tool-run"
N = T + "/🎞️tick/📦️packages/🦀️rust"

def edit(p, old, new, cnt=1):
    s = rgl.read(p)
    assert s.count(old) == cnt, (p, old[:60], s.count(old))
    rgl.write(p, s.replace(old, new))

root = T + "/🦀️.rs"
s = rgl.read(root)
blocks = []
for mod in ["identity_receiving", "step_receiving", "native_tick_preparation", "entity_column_receiving", "progress_receiving"]:
    m = re.search(r'#\[path="🎞️tick/🛬️receive/([^"]+)"\]\nmod ' + mod + r';\npub use ' + mod + r'::[^\n]*;\n', s)
    assert m, mod
    blocks.append(m.group(0).replace('#[path="🎞️tick/', '#[path="'))
    s = s[:m.start()] + s[m.end():]
m = re.search(r'#\[path="🎞️tick/🛫️encode/🦀️.rs"\]\nmod original_tick_encoding;\npub use original_tick_encoding::[^\n]*;\n', s)
assert m
enc = m.group(0).replace('#[path="🎞️tick/', '#[path="')
s = s[:m.start()] + s[m.end():]
m = re.search(r'#\[path="🎞️tick/🛬️receive/🦀️.rs"\]\npub mod tick_receive;\n', s)
assert m
recv = m.group(0).replace('#[path="🎞️tick/', '#[path="')
s = s[:m.start()] + s[m.end():]
tk = '''
    pub mod progress_receiving {
        pub use super::super::progress_receiving::Phase;
    }

    pub mod entity_column_receiving {
        pub use super::super::entity_column_receiving::Phase;
    }
'''
assert tk in s
s = s.replace(tk, "")
rgl.write(root, s)

edit(T + "/📨️payload/🌊️stream/📡️wire/🦀️.rs", '#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;\n', "")

glue = T + "/📦️packages/🦀️rust/🦀️.rs"
g = rgl.read(glue)
g = g.replace("pub use component::tick_receive;\n", "")
for n in ["ToolRunIdentityReceiving", "ToolRunStepReceiving", "ToolRunNativeTickPreparation", "ToolRunEntityColumnReceiving", "ToolRunProgressReceiving", "ToolRunReceivedProgressSource", "ToolRunReceivedTick", "ToolRunReceivedTickSource", "ToolRunReceivedProgress", "ToolRunReceivedTracePage", "ToolRunTickWireCursor", "ToolRunTickPayloadSource"]:
    assert g.count(f"    {n},\n") == 1, n
    g = g.replace(f"    {n},\n", "")
rgl.write(glue, g)

os.makedirs(N, exist_ok=True)
rgl.write(N + "/Cargo.toml", '''[package]
workspace = "../../../../../.."
name = "semio-framework-tool-run-tick-wire"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Tool-run tick wire: the canonical tick record encoder and the funded native receiving of ticks, identities, steps, progress, entity columns and trace pages."

[package.metadata.semio]
role = "framework"
id = "tool-run-tick-wire"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
name = "semio_framework_tool_run_tick_wire"
path = "../../🦀️.rs"

[features]
testkit = []

[dependencies]
semio-framework-tool-run = { path = "../../../📦️packages/🦀️rust" }
semio-framework-pack = { path = "../../../../🎒️pack/📦️packages/🦀️rust" }
semio-framework-value = { path = "../../../../🌱️value/📦️packages/🦀️rust" }
''')
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/framework-tool-run-tick-wire-rs"))
names = "ToolRunCounter, ToolRunId, ToolRunIdentity, ToolRunPayloadUpdate, ToolRunProgress, ToolRunReceivingLimits, ToolRunState, ToolRunStep, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_COUNTERS_MAX, TOOL_RUN_STEP_ARGS_MAX, TOOL_RUN_STEP_RING_CAPACITY, TOOL_RUN_TICK_BYTES_MAX, TOOL_RUN_TRACE_PAGE_BYTES_MAX, TOOL_RUN_TRACE_PAGE_OPS_MAX"
head = '''//! 🎞️ Tool-run tick wire: the canonical tick record encoder and the funded native receiving of ticks, identities, steps, progress, entity columns and trace pages over the tool-run contract.

use semio_framework_tool_run::payload_stream;
use semio_framework_tool_run::{''' + names + '''};
use pack::{PackLimits, record::RetainedRecordBodyCursor};
'''
body = "".join(blocks) + recv + enc
tail = '''
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod progress_receiving {
        pub use super::super::progress_receiving::Phase;
    }

    pub mod entity_column_receiving {
        pub use super::super::entity_column_receiving::Phase;
    }
}

#[cfg(test)]
#[path = "../📨️payload/🌊️stream/📡️wire/🧪️tests/🦀️.rs"]
mod payload_stream_wire_tests;
'''
rgl.write(T + "/🎞️tick/🦀️.rs", head + "\n" + body + tail)
