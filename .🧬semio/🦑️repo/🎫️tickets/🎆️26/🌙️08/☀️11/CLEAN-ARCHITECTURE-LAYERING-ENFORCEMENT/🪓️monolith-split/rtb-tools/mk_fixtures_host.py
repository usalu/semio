"""Adds the `host` module (os-host component close fixtures) to the fixtures crate. usage: mk_fixtures_host.py [--apply]"""
import sys
from build import *
import mk_fixtures as M

APPLY = "--apply" in sys.argv
SRC = E + "/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs"
NAMES = ["finish_world_fixture_component_close", "finish_cpu_only_fixture_component_close", "finish_retired_cpu_only_map_fixture_close"]
DST = M.FX_DIR + "/🖥️host/🦀️.rs"


def build():
    set_hosts([])
    new_src, moved, want = extract(SRC, NAMES)
    pieces = [rebase_paths(m, SRC, DST) for m in moved]
    body = M.transform_moved("\n\n".join(pieces))
    body, _ = sibling_refs(body, set(want))
    tmp = SCRATCH + "/fx-host-body.rs"
    write(tmp, body)
    aliases, use_lines = imports("fixtures", [tmp])
    uniq = aliases + merge_import_blocks(file_use_blocks(SRC, body) + use_lines, simplify=True, exclude_locals=set(want))
    header = "//! 🖥️ Host fixtures of the test bins: the component close drivers several laws end their retained worlds with.\n//!\n//! 🧭️ Moved out of the os-host law file that held them, so that laws in different bins name them through this crate.\n\n"
    module = header + "\n".join(uniq) + "\n\n" + body.rstrip("\n") + "\n"
    return module, new_src, want


def file_use_blocks(src, body):
    return render_file_uses(file_uses(src, tokens(body)))


if __name__ == "__main__":
    module, new_src, want = build()
    print("moves", want, "module lines", len(module.split("\n")))
    if not APPLY:
        print(module[:3000])
    else:
        write(DST, module)
        edit(SRC, new_src)
        root = M.FX_DIR + "/🦀️.rs"
        t = read(root)
        if "pub mod host;" not in t:
            edit(root, t.rstrip("\n") + '\n\n#[path = "🖥️host/🦀️.rs"]\npub mod host;\n')
        names = set(want)
        for f in all_test_files():
            if f == SRC:
                t2 = new_src
            else:
                t2 = read(f)
            t2, used = sibling_refs(t2, names)
            still = {n for n in names if uses_name(t2, n) and not re.search(r"(?m)^\s*(?:pub(?:\([^)]*\))? )?(?:async )?(?:fn|struct|const|static) " + re.escape(n) + r"\b", t2)}
            need = sorted(used | still)
            if need:
                body = need[0] if len(need) == 1 else "{" + ", ".join(need) + "}"
                t2 = add_use(t2, f"use {M.FX_LIB}::host::{body};")
            if t2 != read(f):
                edit(f, t2)
                print("edited", os.path.relpath(f, E)[-60:], need)
