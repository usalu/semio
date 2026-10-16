import sys, shutil
from union import *
import carve_math
flags_sets = {"native": [], "test": ["--test"], "wasip2": ["--target", "wasm32-wasip2"], "wasm": ["--target", "wasm32-unknown-unknown"], "wasm-test": ["--target", "wasm32-unknown-unknown", "--test"]}
carve_math.fresh_new()
carve_math.write_math()
which = sys.argv[1:] or ["native", "test"]
bl = build_union(BASE, SP + "/ub-math", [("math", "📐️math/🦀️.rs")], rewrite=False)
roots = [(PFX + k, CRATES[k]["node"] + "/🦀️.rs") for k in ["algebra", "camera", "mesh", "pass", "picking"]]
nl = build_union(NEW, SP + "/un-math", roots)
for w in which:
    print("###", w)
    b, _ = rustc_run(SP + "/ub-math", bl, "scene_math", flags_sets[w])
    n, e = rustc_run(SP + "/un-math", nl, "scene_math", flags_sets[w])
    compare(b, n)
