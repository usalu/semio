#!/usr/bin/env python3
"""🧱️ Spec of group G9: the top of WP4 (reactor-core, reactor-jobs, runtime-exchange, reactor-turn, reactor-wit-bridge, reactor-context)."""
from lib import *
import groupdrv as G

import re
import flipcore as F
import statics as ST

R = PLUG + "/⚛️reactor"


def add_uses(*lines):
    def fn(text):
        pos = F.header_pos(text)
        return text[:pos] + "".join(l + "\n" for l in lines) + text[pos:]
    return fn


def core_statics(text):
    text = re.sub(r"(\n    )static (ARMED_TIMERS|PACKAGE_ASSETS|PRESENCE):", r"\1pub static \2:", text)
    return re.sub(r"\nstruct FixedTimerRegistry\b", "\n#[doc(hidden)]\npub struct FixedTimerRegistry", text)


class S(G.Spec):
    name = "g9"
    group = ["reactor-core", "reactor-jobs", "runtime-exchange", "reactor-turn", "reactor-wit-bridge", "reactor-context"]
    roots_b = {"reactor-core": R + "/🦀️.rs", "reactor-jobs": R + "/💼️jobs/🦀️.rs", "reactor-turn": R + "/🔄️turn/🦀️.rs", "reactor-wit-bridge": R + "/🧬️wit-bridge/🦀️.rs", "reactor-context": R + "/🎟️context/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/⚛️wp4-reactor/🦀️.rs"
    extra_edits = {
        ST.ROOT: core_statics,
        R + "/💼️jobs/🏛️reserved/🦀️.rs": add_uses("use semio_framework_actor::OriginalJobStep as JobStep;"),
    }
    hub_units = {"reactor": """pub mod reactor {
    pub use semio_framework_plugin_reactor_core::*;
    pub use semio_framework_plugin_reactor_context as original_actor_context;
    pub use semio_framework_plugin_reactor_jobs as jobs;
    pub use semio_framework_plugin_reactor_turn as turn;
    pub use semio_framework_plugin_reactor_turn::turn_close_step::*;
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub use semio_framework_plugin_reactor_turn::drain_task_resumes;
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub use semio_framework_plugin_reactor_wit_bridge::wit_bridge::{poll, stage_cold_pair_page, stage_command_page};
    pub use semio_framework_plugin_reactor_turn::{last_turn_more_work_sources, poll_kernel, retained_command_ingress_occupancy, retained_extension_invocation_occupancy, TurnMoreWorkSources};
}"""}


if __name__ == "__main__":
    G.main(S)
