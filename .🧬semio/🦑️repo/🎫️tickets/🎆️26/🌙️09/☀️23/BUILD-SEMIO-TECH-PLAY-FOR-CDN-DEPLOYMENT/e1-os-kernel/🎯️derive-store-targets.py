#!/usr/bin/env python3
"""🎯️ Rewrites the store section of `🎯️target-crates.json` from an automatic layering (`🧭️facet-map.py layers 4 --emit`)
plus the semantic placement decisions of wave E1 (engine-held views, codec, io, history, composition, preparation).
Usage: derive-store-targets.py LAYERS_JSON"""
import json, re, sys
L = json.load(open(sys.argv[1]))
P = "os_store::component::"
short = lambda n: n.replace(P, "").replace("store-root/", "#")
assign = {n: {"0": "base", "1": "codec", "2": "engine", "3": "preparation"}[k] for k, ns in L.items() for n in ns}
engine = {"#🧬️OpaqueSnapshotRead", "#🔖️Composition", "#SpaceHost", "durable_group", "initialization_authority", "#🔖️CodecRegistry"}
composition = {"composition_coordinator", "composition_group", "composition_links", "member_open::genesis", "owned_document_closure", "initial_child_identity"}
for n in assign:
    s = short(n)
    if s in engine: assign[n] = "engine"
    elif s in composition: assign[n] = "composition"
    elif s.startswith("space_history::"): assign[n] = "history"
    if n == "io" or n.startswith("io::io_mechanism"): assign[n] = "io"
    if n.startswith("io::bindings"): assign[n] = "codec"
names = {"base": "semio-framework-os-store-base", "codec": "semio-framework-os-store-codec", "io": "semio-framework-os-io", "engine": "semio-framework-os-store",
         "history": "semio-framework-os-store-history", "preparation": "semio-framework-os-store-preparation", "composition": "semio-framework-os-store-composition"}
deps = {"base": ["semio-framework-os-spr", "semio-framework-os-dsl", "semio-framework-os-directory", "semio-framework-os-inference", "semio-framework-os-extension", "semio-framework-os-list-delta"],
        "codec": ["semio-framework-os-store-base"], "io": ["semio-framework-os-store-codec"], "engine": ["semio-framework-os-io"],
        "history": ["semio-framework-os-store"], "preparation": ["semio-framework-os-store"], "composition": ["semio-framework-os-store-history", "semio-framework-os-store-preparation"]}
t = json.load(open("🎯️target-crates.json"))
keep = [c for c in t["crates"] if not c["name"].startswith(("semio-framework-os-store", "semio-framework-os-io", "semio-framework-os-kernel"))]
for k in ["base", "codec", "io", "engine", "history", "preparation", "composition"]:
    keep.append({"name": names[k], "deps": deps[k], "nodes": sorted("^" + re.escape(n) + "$" for n, v in assign.items() if v == k)})
keep.append({"name": "semio-framework-os-store-sync", "deps": ["semio-framework-os-store-composition"], "nodes": ["^os_store::sync", "^os_store::worker"]})
keep.append({"name": "semio-framework-os-kernel", "deps": ["semio-framework-os-store-sync"], "nodes": ["^$", "^os_store$", "^os_dsl$"]})
t["crates"] = keep
json.dump(t, open("🎯️target-crates.json", "w"), ensure_ascii=False, indent=1)
