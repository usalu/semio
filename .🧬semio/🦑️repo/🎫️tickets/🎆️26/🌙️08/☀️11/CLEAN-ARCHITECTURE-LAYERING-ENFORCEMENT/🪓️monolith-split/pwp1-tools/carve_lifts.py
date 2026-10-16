#!/usr/bin/env python3
# 🛬️ WP1 as source of lifts (Appendix J): the three completion files and `cancellation_result_lane` move into the decl-app lift files; `EphemeralParts` goes to window-transient.
import os, re
from carvelib import *

APP = PLUG + "/📜️declaration/🧩️app/🛬️lifted"
COMP = PLUG + "/📬️completion"


def lift(src, dst, doc, transform=lambda t: t):
    text = read(src)
    text = transform(text)
    first = text.split("\n", 1)
    if first[0].startswith("//!"):
        text = first[1] if len(first) > 1 else ""
    write(dst, doc.rstrip("\n") + "\n" + text)


def rel_include(dst, target):
    return os.path.relpath(target, os.path.dirname(dst))


# L2 fault
src, dst = COMP + "/⚠️fault/🦀️.rs", APP + "/⚠️fault/🦀️.rs"
fix = rel_include(dst, COMP + "/⚠️fault/🧪️tests/🧫️fixtures/🔣️.json")
lift(src, dst, "//! ⚠️ Accepted completion faults retain their original diagnostic and build a bounded borrowed report without allocation (lifted from the WP1 completion tree; WP2 wires it into decl-app).",
     lambda t: t.replace('include_str!("🧪️tests/🧫️fixtures/🔣️.json")', f'include_str!("{fix}")'))
os.remove(src)

# L3 refusal
src, dst = COMP + "/♻️retirement/🚫️refusal/🦀️.rs", APP + "/🚫️refusal/🦀️.rs"
lift(src, dst, "//! 🚫️ Completion refusal retains the complete original emission, ephemeral lanes and diagnostic (lifted from the WP1 completion tree; WP2 wires it into decl-app).")
os.remove(src)

# L1 retirement; EphemeralParts goes to window-transient
src, dst = COMP + "/♻️retirement/🦀️.rs", APP + "/♻️retirement/🦀️.rs"
old = read(src)
parts = '#[derive(semio_framework_value::RetireOwned)]\nstruct EphemeralParts<P,T>{presence:Vec<P>,transient:Vec<T>,window_transient:Vec<crate::window_transient::WindowTransientMutation>}\n'
assert old.count(parts) == 1
body = old.replace(parts, "")
body = "use crate::window_transient::EphemeralParts;\n" + body
write(dst, "//! ♻️ Completion cells and the typed retirement of tool completions (lifted from the WP1 completion tree; WP2 wires it into decl-app).\n" + body)
os.remove(src)
TR = PLUG + "/🪟️window/🫧️transient/🦀️.rs"
tt = read(TR)
anchor = "//#region 🔖️TransientRoot\n"
assert tt.count(anchor) == 1
tt = tt.replace(anchor, '''/// 📦️ The ephemeral lanes of one completion payload: presence and transient mutations plus the window-transient mutations, retired as one owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct EphemeralParts<P, T> {
    pub presence: Vec<P>,
    pub transient: Vec<T>,
    pub window_transient: Vec<WindowTransientMutation>,
}

''' + anchor, 1)
write(TR, tt)

hub_edit('''#[path="../../📬️completion/⚠️fault/🦀️.rs"]
pub(crate) mod completion_fault;
#[path="../../📬️completion/♻️retirement/🚫️refusal/🦀️.rs"]
pub(crate) mod completion_refusal_retirement;
#[path="../../📬️completion/♻️retirement/🦀️.rs"]
pub(crate) mod completion_retirement;
''', '''#[path="../../📜️declaration/🧩️app/🛬️lifted/⚠️fault/🦀️.rs"]
pub(crate) mod completion_fault;
#[path="../../📜️declaration/🧩️app/🛬️lifted/🚫️refusal/🦀️.rs"]
pub(crate) mod completion_refusal_retirement;
#[path="../../📜️declaration/🧩️app/🛬️lifted/♻️retirement/🦀️.rs"]
pub(crate) mod completion_retirement;
''')

# L4 cancellation_result_lane
OP = PLUG + "/⏳️operation-progress/🦀️.rs"
ot = read(OP)
fn = '''pub fn cancellation_result_lane(user_requested: bool, worker_fault: bool) -> TypedOperationResultLane {
    if user_requested && !worker_fault {
        TypedOperationResultLane::Terminal
    } else {
        TypedOperationResultLane::Fault
    }
}

'''
assert ot.count(fn) == 1
ot = ot.replace(fn, '''#[path = "../📜️declaration/🧩️app/🛬️lifted/⏳️operation-progress/🦀️.rs"]
mod lifted_cancellation_result_lane;
pub use lifted_cancellation_result_lane::cancellation_result_lane;

''')
write(OP, ot)
write(APP + "/⏳️operation-progress/🦀️.rs", "//! ⏳️ The result lane of an operation cancellation (lifted from the WP1 operation-progress file; WP2 wires it into decl-app).\nuse crate::tool_contract::TypedOperationResultLane;\n\n" + fn.rstrip("\n") + "\n")
