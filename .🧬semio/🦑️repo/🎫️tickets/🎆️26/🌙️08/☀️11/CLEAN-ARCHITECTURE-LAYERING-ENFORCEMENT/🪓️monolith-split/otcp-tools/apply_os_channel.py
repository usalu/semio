import os, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib

O = "🧰️framework/🛍️products/💻️os/🔨️modules/"
D = O + "📡️spr/🧵️channel/"
priv = ["wire_entries", "archive_load", "presence", "command_decode", "app_command", "app_frame", "codec"]
real = {m: "testkit::" + m for m in priv}
ctx = applylib.Ctx("os-channel", "semio_framework_os_channel", D + "📦️packages/🦀️rust/Cargo.toml", "semio-framework-os-channel", real_map=real)
R = E.ROOT
C = D + "🧪️tests/🪤️companion/"
keys = list(ctx.mounts)
IMPORT = "use ::protocol::wire::command_ingress::{FixedCommandPage, CommandPageSet, PagedCommand, COMMAND_PAGE_MAXIMUM_BYTES, INVOCATION_RESULT_PACK_MAXIMUM_BYTES};"
alloc = "#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;"
if "--p" in sys.argv:
    ctx.remove_mounts()
    E.edit(D + "🦀️.rs", "#[cfg(test)]\n" + IMPORT + "\n\n", "")
    old = "\n\n#[cfg(test)]\n#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;\n"
    kit = "\n/// 🪝️ Test hook of the companion crate `semio-framework-os-channel-tests`: the private codec modules, public only behind the `testkit` feature.\n#[cfg(feature = \"testkit\")]\n#[doc(hidden)]\npub mod testkit {\n" + "".join(f"    pub mod {m} {{\n        pub use crate::{m}::*;\n    }}\n" for m in priv) + "}\n"
    E.edit(D + "🦀️.rs", old, kit)
    pc = D + "📦️packages/🦀️rust/Cargo.toml"
    t = E.read(pc)
    i = t.index("\n[dev-dependencies]\n")
    E.edit(pc, t[i:], "")
    E.edit(pc, "[lib]\npath = \"../../🦀️.rs\"\n", "[lib]\npath = \"../../🦀️.rs\"\n\n[features]\ntestkit = []\n")
extra = []
ioc = 'semio-framework-io-control = { path = "../../../../../../../../../🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust" }'
title = "Companion tests of os-channel: channel codec unit tests and document archive decoding."
if "--show" in sys.argv:
    print(ctx.render_root(R + "/" + C, keys, title, "🪤️", alloc, extra))
if "--create" in sys.argv:
    print(ctx.companion(C, keys, "semio-framework-os-channel-tests", "os-channel-tests", "@semio-tech/os-channel-tests-rs", "Companion tests of os-channel: channel codec unit tests and document archive decoding", title, "🪤️", p_features=("testkit",), allocator=alloc, extra=extra, extra_deps=(ioc,)))
