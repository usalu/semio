"""Moves the os-host standalone law file into host-winit as an in-crate test of `os_host` and extracts its three component-close fixtures into `os_host::fixtures` (testkit).
usage: mk_host_winit.py [--apply]"""
import sys
from build import *

APPLY = "--apply" in sys.argv
SRC = E + "/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs"
OSH = T + "/🏠️os-host/🦀️.rs"
FXD = T + "/🏠️os-host/🧰️fixtures/🦀️.rs"
NAMES = ["finish_world_fixture_component_close", "finish_cpu_only_fixture_component_close", "finish_retired_cpu_only_map_fixture_close"]
SUFFIX = "host-winit"


def build():
    set_hosts([])
    X.CURRENT[0] = SUFFIX
    new_src, moved, want = extract(SRC, NAMES)
    pieces = [rebase_paths(m, SRC, FXD) for m in moved]
    body = X.rewrite_heads("\n\n".join(pieces))
    body = FL.rewrite_paths(body)
    tmp = SCRATCH + "/hw-fx-body.rs"
    write(tmp, body)
    aliases, lines, unresolved = full_imports(SUFFIX, [tmp], local_defs=set(want))
    fu = render_file_uses(file_uses(SRC, tokens(body)))
    uniq = aliases + merge_import_blocks(fu + lines, simplify=True, exclude_locals=set(want), own=lib_name(SUFFIX))
    header = "//! 🧰️ Test fixtures of the OS host: the component close drivers every law that retires a retained scene, world or map ends with.\n//!\n//! 🧭️ Behind `testkit`: the monolith test unit held these functions in the os-host standalone law file; laws in the test bins name them through this module.\n\nuse super::*;\n"
    module = header + "\n".join(uniq) + "\n\n" + body.rstrip("\n") + "\n"
    return module, new_src, want, unresolved


def test_module(new_src):
    """The remaining standalone law file as a child module of os_host."""
    X.CURRENT[0] = SUFFIX
    body = FL.rewrite_paths(X.rewrite_heads(new_src))
    tmp = SCRATCH + "/hw-test-body.rs"
    write(tmp, body)
    aliases, lines, unresolved = full_imports(SUFFIX, [tmp], local_defs=set())
    fu = render_file_uses(file_uses(SRC, tokens(body)))
    uniq = aliases + merge_import_blocks(fu + lines, simplify=True, own=lib_name(SUFFIX))
    return "use super::*;\n" + "\n".join(uniq) + "\n\n" + body.lstrip("\n"), unresolved


if __name__ == "__main__":
    module, new_src, want, unresolved = build()
    print("moves", want, "lines", len(module.split("\n")), "ambiguous:", unresolved)
    tm, un2 = test_module(new_src)
    print("test module lines", len(tm.split("\n")), "ambiguous:", un2)
    if APPLY:
        import manifest
        write(FXD, module)
        edit(SRC, tm)
        osh = read(OSH)
        marker = "pub use fixtures::{finish_cpu_only_fixture_component_close"
        if marker not in osh:
            osh = osh.rstrip("\n") + """

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️fixtures/🦀️.rs"]
mod fixtures;
#[cfg(any(test, feature = "testkit"))]
pub use fixtures::{finish_cpu_only_fixture_component_close, finish_retired_cpu_only_map_fixture_close, finish_world_fixture_component_close, runtime_with_shell};

#[cfg(test)]
#[path = "../../../🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs"]
mod standalone_tests;
"""
            edit(OSH, osh)
        mp = T + "/🎪️host-winit/📦️packages/🦀️rust/Cargo.toml"
        added = manifest.add_deps(mp, used_idents(module) | used_idents(tm))
        manifest.set_testkit_feature(mp, manifest.first_party_testkit_members(mp))
        print("deps added:", added)
