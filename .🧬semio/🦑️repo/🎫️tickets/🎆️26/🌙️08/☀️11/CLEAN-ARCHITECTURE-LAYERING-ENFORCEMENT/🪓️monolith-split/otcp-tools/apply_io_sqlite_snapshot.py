import os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E, applylib

FW = "🧰️framework/"
D = FW + "🔨️modules/🚪️io/🪶️sqlite-snapshot/"
ctx = applylib.Ctx("io-sqlite-snapshot", "semio_framework_io_sqlite_snapshot", D + "📦️packages/🦀️rust/Cargo.toml", "semio-framework-io-sqlite-snapshot", real_map={"component": "testkit::component"})
R = E.ROOT
C = D + "🧪️tests/🌅️companion/"
ART = D + "🧩️artifact/🦀️.rs"
A_NEW = D + "🧩️artifact/🧪️tests/🔬️unit/🦀️.rs"
B_NEW = D + "🔢️ieee754/🧪️tests/🔬️unit/🦀️.rs"
kA = ctx.add_synthetic(("component", "artifact"), "tests", R + "/" + ART, R + "/" + A_NEW)
kB = ctx.add_synthetic(("component", "artifact"), "ieee_tests", R + "/" + ART, R + "/" + B_NEW)
keys = [k for k in ctx.mounts]
alloc = "#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;"
extra = ["use semio_framework_io_control as original_control;"]
ctx.node_extras[("component", "artifact")] = {"uses": ["pub use semio_framework_io_sqlite_snapshot::testkit::component::artifact::testkit::row_writer;"], "includes": []}


def block(text, head):
    """(start, end) of the item starting at the unique `head` line, through its matching closing brace line."""
    i = text.index(head)
    j = text.index("{", i)
    d = 0
    k = j
    while True:
        if text[k] == "{": d += 1
        elif text[k] == "}":
            d -= 1
            if d == 0: break
        k += 1
    return i, k + 1


if "--p" in sys.argv:
    t = E.read(ART)
    a0, a1 = block(t, "mod tests {")
    b0, b1 = block(t, "mod ieee_tests {")
    body_a = t[t.index("{", a0) + 1:a1 - 1]
    body_b = t[t.index("{", b0) + 1:b1 - 1]

    def dedent(body):
        out = []
        for l in body.strip("\n").split("\n"):
            out.append(l[4:] if l.startswith("    ") else l)
        return "\n".join(out) + "\n"
    ta = "//! 🔬️ Unit tests of the artifact snapshot reader: paid row index, projection and cancellation.\n" + dedent(body_a).replace('include_str!("🧪️tests/🧫️fixtures/', 'include_str!("../🧫️fixtures/')
    tb = "//! 🔢️ Authored IEEE 754 field tests of the artifact snapshot reader.\n" + dedent(body_b).replace('include_str!("../🔢️ieee754/🧪️tests/🧫️fixtures/', 'include_str!("../🧫️fixtures/')
    assert "include_str!(\"🧪️tests/" not in ta and "../🔢️ieee754/" not in tb
    E.put(A_NEW, ta)
    E.put(B_NEW, tb)
    # cut the inline modules (with their #[cfg(test)] line and one trailing blank line)
    for head in ("#[cfg(test)]\nmod tests {", "#[cfg(test)]\nmod ieee_tests {"):
        t = E.read(ART)
        s0 = t.index(head)
        _, s1 = block(t, head)
        s1 = s1 + 1 if t[s1:s1 + 1] == "\n" else s1
        if t[s1:s1 + 1] == "\n" and t[s0 - 1:s0] == "\n":
            s1 += 1
        E.pre_copy(ART)
        E.write(ART, t[:s0] + t[s1:])
        E.note("edit", ART)
    ctx.remove_mounts()
    glue = D + "📦️packages/🦀️rust/🦀️.rs"
    E.edit(glue, "\n#[cfg(test)]\n#[global_allocator]\nstatic TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;\n", "")
    E.edit(glue, "#[cfg(test)]\npub(crate) use semio_framework_io_control as original_control;\n", "/// 🪝️ Test hook of the companion crate `semio-framework-io-sqlite-snapshot-tests`: the private snapshot modules, public only behind the `testkit` feature.\n#[cfg(feature = \"testkit\")]\n#[doc(hidden)]\npub mod testkit {\n    pub mod component {\n        pub use crate::component::*;\n    }\n}\n")
    pc = D + "📦️packages/🦀️rust/Cargo.toml"
    t = E.read(pc)
    i = t.index("\n[dev-dependencies]\n")
    E.edit(pc, t[i:], "")
    E.edit(pc, "[[bin]]", "[features]\ntestkit = []\n\n[[bin]]")
title = "Companion tests of io-sqlite-snapshot: snapshot unit, limits, refusal, retirement, receiving and artifact reader tests."
if "--show" in sys.argv:
    print(ctx.render_root(R + "/" + C, keys, title, "🌅️", alloc, extra))
if "--create" in sys.argv:
    print(ctx.companion(C, keys, "semio-framework-io-sqlite-snapshot-tests", "io-sqlite-snapshot-tests", "@semio-tech/io-sqlite-snapshot-tests-rs", "Companion tests of io-sqlite-snapshot: snapshot unit, limits, refusal, retirement, receiving and artifact reader tests", title, "🌅️", p_features=("testkit",), allocator=alloc, extra=extra, framework=True))
