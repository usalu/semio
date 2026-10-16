#!/usr/bin/env python3
# 🗺️ Name -> owning crate path of the two facades (Scenes, EngineCanvas) and of the root-level re-exports: the index the Interpreter flip rewrites `crate::scenes::NAME` with.
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
from ed import *
from rs import uses, blank

def facade_map(path):
    text = read(path)
    aliases = {}
    names = {}
    ents = uses(text, top_only=True)
    for cfg, vis, p, local, glob in ents:
        if glob or not p or not p[0].startswith("semio_framework_os_renderer_wgpu_"):
            continue
        if len(p) == 2 and p[1] and local == p[1] and p[1] not in names:
            aliases.setdefault(local, p)
    for cfg, vis, p, local, glob in ents:
        if glob or not p:
            continue
        if p[0] in aliases:
            full = aliases[p[0]] + p[1:]
        elif p[0].startswith("semio_framework_os_renderer_wgpu_"):
            full = p
        else:
            continue
        if not (vis.startswith("pub")):
            continue
        names.setdefault(local, []).append((cfg, full))
    for a, p in aliases.items():
        names.setdefault(a, [("", p)])
    return names

def scenes():
    return facade_map(EL + "/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs")

def engine_canvas():
    return facade_map(EL + "/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs")

if __name__ == "__main__":
    for nm, f in (("scenes", scenes), ("engine_canvas", engine_canvas)):
        m = f()
        print(nm, len(m))
        for k in list(m)[:6]:
            print("  ", k, m[k])
