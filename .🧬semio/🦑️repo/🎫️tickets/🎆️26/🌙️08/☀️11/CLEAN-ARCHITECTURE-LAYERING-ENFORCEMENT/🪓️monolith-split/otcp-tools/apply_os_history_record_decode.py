import os, sys, json
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib, genmirror

O = "🧰️framework/🛍️products/💻️os/🔨️modules/"
D = O + "📡️spr/📜️history/🛬️decode/"
ROOTF = D + "📋️record-set/🦀️.rs"
mods = ["retained_history_records", "retained_history_paged_records", "retained_history_operation", "retained_history_id", "retained_history_message", "retained_history_origin", "retained_history_metadata", "retained_history_transition", "retained_history_edit", "retained_history_identity_records", "retained_history_conflict", "retained_history_transition_validation"]
real = {m: "testkit::" + m for m in mods}
real["retained_history_origin::json"] = "testkit::retained_history_origin::json"
ctx = applylib.Ctx("os-history-record-decode", "semio_framework_os_history_record_decode", D + "📋️record-set/📦️packages/🦀️rust/Cargo.toml", "semio-framework-os-history-record-decode", real_map=real)
R = E.ROOT
C = D + "🧪️tests/🧿️companion/"
keys = list(ctx.mounts)
if "--p" in sys.argv:
    ctx.remove_mounts()
    old = '''
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[cfg(test)]
use semio_framework_os_history_testing_retirement as test_retirement;
'''
    kit = "\n/// 🪝️ Test hook of the companion crate `semio-framework-os-history-record-decode-tests`: the private decoder modules, public only behind the `testkit` feature.\n#[cfg(feature = \"testkit\")]\n#[doc(hidden)]\npub mod testkit {\n"
    for m in mods:
        if m == "retained_history_origin":
            kit += "    pub mod retained_history_origin {\n        pub use crate::retained_history_origin::*;\n        pub use crate::retained_history_origin::testkit::json;\n    }\n"
        else:
            kit += f"    pub mod {m} {{\n        pub use crate::{m}::*;\n    }}\n"
    kit += "}\n"
    E.edit(ROOTF, old, kit)
    E.edit(D + "🔀️origin/🦀️.rs", "#[path=\"🔣️json/🦀️.rs\"]\nmod json;\n", "#[path=\"🔣️json/🦀️.rs\"]\nmod json;\n\n/// 🪝️ Test hook: the private JSON reader, public only behind the `testkit` feature.\n#[cfg(feature = \"testkit\")]\n#[doc(hidden)]\npub mod testkit {\n    pub mod json {\n        pub use super::super::json::*;\n    }\n}\n")
    pc = D + "📋️record-set/📦️packages/🦀️rust/Cargo.toml"
    t = E.read(pc)
    i = t.index("\n[dev-dependencies]\n")
    E.edit(pc, t[i:], "")
    E.edit(pc, "[lib]\npath = \"../../🦀️.rs\"\n", "[lib]\npath = \"../../🦀️.rs\"\n\n[features]\ntestkit = []\n")
alloc = "#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;"
extra = ["use semio_framework_os_history_testing_retirement as test_retirement;"]
if "--show" in sys.argv:
    print(ctx.render_root(R + "/" + C, keys, "Companion tests of os-history-record-decode: the retained History record decoders.", "🧿️", alloc, extra))
if "--create" in sys.argv:
    print(ctx.companion(C, keys, "semio-framework-os-history-record-decode-tests", "os-history-record-decode-tests", "@semio-tech/os-history-record-decode-tests-rs", "Companion tests of os-history-record-decode: the retained History record decoders", "Companion tests of os-history-record-decode: the retained History record decoders.", "🧿️", p_features=("testkit",), allocator=alloc, extra=extra))
