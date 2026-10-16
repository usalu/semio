#!/usr/bin/env python3
# 🪪️ Extern-prelude aliases of the old renderer root (`dsl`, `protocol`, `store`, `store_sync`, `dsl_core`, `infinite_world`, `wasm_bindgen_futures`) a carved crate names anywhere in its module tree.
import re, sys, json
from common import *
import modtree, needs
from rs import blank


def used_aliases(files):
    _, externs = needs.root_block()
    found = []
    text = "\n".join(blank(read(f)) for f in files)
    for cfg, crate, alias in externs:
        if re.search(r"(?<![A-Za-z0-9_:.$])" + re.escape(alias) + r"::", text):
            found.append((cfg, crate, alias))
    cam = bool(re.search(r"(?<![A-Za-z0-9_:.$])camera_storage::", text))
    return found, cam


def fix(suffix, apply=True):
    root = T + "/" + CRATES[suffix] + "/🦀️.rs"
    files = modtree.tree(root)
    found, cam = used_aliases(files)
    t = read(root)
    add = []
    for cfg, crate, alias in found:
        if f" as {alias};" in t:
            continue
        add.append((cfg + "\n" if cfg else "") + f"extern crate {crate} as {alias};")
    if cam and "camera_storage" not in t:
        add.append("use semio_framework_os_renderer_camera as camera_storage;")
    if add and apply:
        marker = "\n#[path"
        i = t.index(marker)
        j = t.rfind("\n\n", 0, i)
        # insert before the first mount block: after the import block, i.e. at the start of the imports (directly after the doc header)
        k = t.index("\n\n") + 2
        t = t[:k] + "\n".join(add) + "\n" + t[k:]
        write(root, t)
    return add


if __name__ == "__main__":
    for c in (sys.argv[1:] or json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))):
        print(c, fix(c))
