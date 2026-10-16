#!/usr/bin/env python3
# 🧪️ Hub-hosts the window-mutation / window-config / window-transient tests that name facade-only fixtures (see host_clusters.py).
import os, re
from carvelib import *
import host_clusters as H

WIN = "🪟️window"
CFG = WIN + "/🎚️config"

# --- window_mutation tests ------------------------------------------------------------------
H.make_host("📬️window-mutation", "window_mutation_tests", WIN + "/📬️mutation/🦀️.rs", "crate::window_mutation",
            [("tests", WIN + "/📬️mutation/🧪️tests/🦀️.rs", "")],
            "Test host of `window_mutation`: the erased-payload and refresh laws and the close helpers the window tests share.",
            extra=["pub use crate::window_mutation::refresh::{refresh_demands, refresh_transfer, WindowRefreshSnapshot};",
                   "pub(crate) use tests::{close_original as close_test_original, close_with as close_test_original_with_pump};"],
            unmount=[(WIN + "/📬️mutation/🦀️.rs", '#[cfg(test)]\n#[path="🧪️tests/🦀️.rs"]\nmod tests;\n#[cfg(test)]\npub(crate) use tests::{close_original as close_test_original,close_with as close_test_original_with_pump};\n'),
                     (WIN + "/📬️mutation/🦀️.rs", "#[cfg(test)]\nuse semio_framework_plugin_window_config::window_mutation::refresh::{refresh_demands, refresh_transfer, WindowRefreshSnapshot};\n")])
H.mount_in_hub("window_mutation_tests", "📬️window-mutation")

# --- window_config tests --------------------------------------------------------------------
RET = PLUG + "/" + CFG + "/📥️retained/🦀️.rs"
rt = read(RET)
a = rt.index("#[cfg(test)]\nmod physical_ownership_tests {\n")
b = rt.index("\n}\n", a) + 3
inner = rt[a + len("#[cfg(test)]\nmod physical_ownership_tests {\n"):b - 3]
newfile = PLUG + "/" + CFG + "/🧪️tests/🧬️physical-ownership/🦀️.rs"
assert not os.path.exists(newfile)
body = "\n".join(l[4:] if l.startswith("    ") else l for l in inner.split("\n"))
body = body.replace('include_str!("../🗂️registry/🧪️tests/🧫️fixtures/🔣️.json")', 'include_str!("' + os.path.relpath(PLUG + "/" + CFG + "/🗂️registry/🧪️tests/🧫️fixtures/🔣️.json", os.path.dirname(newfile)) + '")')
write(newfile, "//! 🧬️ Physical ownership laws of the retained window-config state, hosted by the facade test build (they name the window-config test fixtures).\n" + body.rstrip("\n") + "\n")
write(RET, rt[:a] + rt[b:])

H.make_host("🎚️window-config/🧬️retained", "retained_tests", CFG + "/📥️retained/🦀️.rs", "crate::window_config::retained",
            [("physical_ownership_tests", CFG + "/🧪️tests/🧬️physical-ownership/🦀️.rs", "")],
            "Test host of the window-config `retained` module.")
H.make_host("🎚️window-config", "window_config_tests", CFG + "/🦀️.rs", "crate::window_config",
            [("original_preparation_tests", CFG + "/🧬️preparation/🧪️tests/🦀️.rs", ""),
             ("tests", CFG + "/🧪️tests/🪟️retained-window-config/🦀️.rs", ""),
             ("pack_identity_tests", CFG + "/🧪️tests/🪪️pack-identity/🦀️.rs", ""),
             ("retained_pack_load_tests", CFG + "/🧪️tests/📥️retained-pack-load/🦀️.rs", ""),
             ("retained_tests", ("host", "🧬️retained"), "")],
            "Test host of `window_config`: the laws that name the facade test fixtures run in the facade test build.",
            extra=["pub use crate::window_config::retained::*;"],
            unmount=[(CFG + "/🦀️.rs", '#[cfg(test)]\n#[path="🧬️preparation/🧪️tests/🦀️.rs"]\nmod original_preparation_tests;\n\n'),
                     (CFG + "/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/🪟️retained-window-config/🦀️.rs"]\nmod tests;\n\n'),
                     (CFG + "/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/🪪️pack-identity/🦀️.rs"]\nmod pack_identity_tests;\n\n'),
                     (CFG + "/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/📥️retained-pack-load/🦀️.rs"]\nmod retained_pack_load_tests;\n')])
H.mount_in_hub("window_config_tests", "🎚️window-config")

# --- window_transient tests -------------------------------------------------------------------
TR = WIN + "/🫧️transient"
H.make_host("🫧️window-transient", "window_transient_tests", TR + "/🦀️.rs", "crate::window_transient",
            [("retained_window_input_tests", TR + "/🧪️tests/🪟️retained-window-input/🦀️.rs", ""),
             ("transient_root_tests", TR + "/🧪️tests/🧪️transient-root/🦀️.rs", "")],
            "Test host of `window_transient`: the laws that name the facade publication fixtures and the exported transient macros.",
            unmount=[(TR + "/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/🪟️retained-window-input/🦀️.rs"]\nmod retained_window_input_tests;\n\n'),
                     (TR + "/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/🧪️transient-root/🦀️.rs"]\nmod transient_root_tests;\n\n')])
H.mount_in_hub("window_transient_tests", "🫧️window-transient")

# --- the close helpers moved into the window_mutation host --------------------------------------
for rel in (CFG + "/🧪️tests/🪪️pack-identity/🦀️.rs", CFG + "/🧪️tests/🪟️retained-window-config/🦀️.rs", TR + "/🧪️tests/🪟️retained-window-input/🦀️.rs"):
    t = read(PLUG + "/" + rel)
    n = t.count("crate::component::window_mutation::close_test_original")
    t = t.replace("crate::component::window_mutation::close_test_original", "crate::component::window_mutation_tests::close_test_original")
    write(PLUG + "/" + rel, t)
    print(rel, n)
