#!/usr/bin/env python3
"""🧱️ Spec of group G7: reactor-lifecycle, runtime-instances, reactor-patches (WP4 ranks above the tasks); lifecycle and patches own root-block statics."""
from lib import *
import groupdrv as G
import statics as ST
import translate as TR

LIFE = ["REACTOR_CLOSES", "REACTOR_CLOSE_CURSOR"]
PATCHES = ["PATCHES"]
TR.STATIC_OWNER.update({**{n: "reactor-lifecycle" for n in LIFE}, "PATCHES": "reactor-patches"})


class S(G.Spec):
    name = "g7"
    group = ["reactor-lifecycle", "runtime-instances", "reactor-patches"]
    roots_b = {"reactor-lifecycle": PLUG + "/⚛️reactor/📨️pending/🦀️.rs", "runtime-instances": PLUG + "/⚛️reactor/🏃️runtime/📦️instances/🦀️.rs", "reactor-patches": PLUG + "/⚛️reactor/🩹️patches/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/⚛️wp4-reactor/🦀️.rs"
    hub_units = {}
    root_extra = {
        "reactor-lifecycle": ST.owned_block(LIFE, "use std::cell::Cell;\nuse life_reactor_close::ReactorCloseRegistry;\n"),
        "reactor-patches": ST.owned_block(PATCHES, "#[cfg(test)]\nextern crate semio_framework_os_kernel as dsl;\n", rewrite=[("patches::PatchTracker", "PatchTracker")]),
    }
    dev_texts = {"reactor-patches": [PLUG + "/🧪️testing/🎟️caller/🦀️.rs"]}
    extra_edits = {
        ST.ROOT: ST.leave_root({"semio_framework_plugin_reactor_lifecycle": LIFE, "semio_framework_plugin_reactor_patches": PATCHES}),
        PLUG + "/⚛️reactor/📨️pending/🔒️reactor-close/🦀️.rs": ST.widen_types(["ReactorCloseRegistry"]),
    }


if __name__ == "__main__":
    G.main(S)
