import sys
from union import *
import carve_all
flags_sets = {"native": [], "test": ["--test"], "wasip2": ["--target", "wasm32-wasip2"], "wasm": ["--target", "wasm32-unknown-unknown"], "wasm-test": ["--target", "wasm32-unknown-unknown", "--test"]}
TREE = REPO + "/" + SCENE_REL
which = sys.argv[1:] or ["native", "test"]
bl = build_union(BASE, SP + "/ub-all", [], rewrite=False)
import shutil, os
# baseline: original crate root is the lib root
shutil.rmtree(SP + "/ub-all", ignore_errors=True)
shutil.copytree(BASE, SP + "/ub-all/🎬️scene")
oracle.patch_tree(SP + "/ub-all")
bl = SP + "/ub-all/🎬️scene/📦️packages/🦀️rust/🦀️.rs"
order = ["algebra", "camera", "mesh", "pass", "picking", "pack", "doc", "modelling", "world3d", "node-graph", "plane", "views", "surface"]
roots = [(PFX + k.replace("-", "_"), CRATES[k]["node"] + "/🦀️.rs") for k in order]
roots.append(("semio_framework_ui_scene", "📦️packages/🦀️rust/🦀️.rs"))
nl = build_union(TREE, SP + "/un-all", roots)
EXT = ("semio_framework_geometry", "semio_framework_mesh_engine", "semio_framework_ui_viewport", "semio_framework_pack_json", "semio_framework_value", "serde", "serde_json", "protocol", "ui_contract", "fnv")
def filt(out):
    c = collections.Counter()
    for (code, msg, loc), n in out.items():
        if any(("`%s`" % e) in msg for e in EXT) and ("unresolved" in msg or "cannot find module or crate" in msg or "use of unresolved" in msg):
            continue
        if "aborting due to" in msg:
            continue
        c[(code, msg, loc)] += n
    return c
for w in which:
    print("###", w)
    b, _ = rustc_run(SP + "/ub-all", bl, "scene", flags_sets[w])
    n, e = rustc_run(SP + "/un-all", nl, "scene", flags_sets[w])
    fb, fn_ = filt(b), filt(n)
    nb = sum(v for (c, m, l), v in b.items() if "couldn't read" in m)
    nn = sum(v for (c, m, l), v in n.items() if "couldn't read" in m)
    fb = collections.Counter({k: v for k, v in fb.items() if "couldn't read" not in k[1]})
    fn_ = collections.Counter({k: v for k, v in fn_.items() if "couldn't read" not in k[1]})
    print("couldn't-read (out-of-sandbox fixture) baseline", nb, "new", nn)
    compare(fb, fn_, show=60)
