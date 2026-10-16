#!/usr/bin/env python3
# 🪪️ Extern-prelude aliases of the old renderer root (`infinite_world`, `dsl`, `dsl_core`, `protocol`, `store`, `store_sync`, `camera_storage`, `wasm_bindgen_futures`) a crate names anywhere in its tree must be declared by its own root.
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import modtree
from rs import blank
ALIASES = ["infinite_world", "dsl_core", "dsl", "protocol", "store_sync", "store", "camera_storage", "wasm_bindgen_futures"]
table = crate_table()
apply = "--apply" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]
real = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/real.json"))
SPEC = {"infinite_world": "extern crate infinite_canvas as infinite_world;", "dsl": "extern crate semio_framework_os_kernel as dsl;", "dsl_core": "extern crate semio_framework_os_kernel as dsl_core;", "protocol": "extern crate semio_framework_os_kernel as protocol;", "store": "extern crate semio_framework_os_kernel as store;", "store_sync": "extern crate semio_framework_os_kernel as store_sync;", "camera_storage": "use semio_framework_os_renderer_camera as camera_storage;", "wasm_bindgen_futures": "extern crate semio_framework_async as wasm_bindgen_futures;"}
bad = 0
for c in only or real:
    root = T + "/" + table[c] + "/🦀️.rs"
    rtext = read(root)
    rclean = blank(rtext)
    used = set()
    for f in modtree.tree(root):
        code = blank(read(f))
        local = set(re.findall(r"\bmod\s+(\w+)", code)) | set(re.findall(r"\bas\s+(\w+)", code))
        for a in ALIASES:
            if re.search(r"(?<![\w:.$])" + a + r"::", code) or re.search(r"\buse\s+" + a + r"\b", code):
                used.add(a)
    for a in sorted(used):
        declared = re.search(r"\bas\s+" + a + r"\s*;", rclean)
        if not declared:
            bad += 1
            print("UNDECLARED", c, a)
            if apply:
                i = rtext.index("\n\n") + 2
                rtext = rtext[:i] + SPEC[a] + "\n" + rtext[i:]
    if apply:
        write(root, rtext)
print("undeclared:", bad)
