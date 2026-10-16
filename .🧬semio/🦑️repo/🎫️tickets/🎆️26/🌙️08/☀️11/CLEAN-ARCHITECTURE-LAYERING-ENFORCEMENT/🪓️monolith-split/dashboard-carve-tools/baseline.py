"""📏 Baseline of the original monolith for the dashboard differential (stores base.pkl)."""
import os, pickle, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import oracle as o
base = o.SCR + "/S0"
o.make_sandbox(base, o.SCR + "/orig-dashboard")
o.patch_tree(base)
lib = f"{base}/{o.DASH}/📦️packages/🦀️rust/🦀️.rs"
res = {}
for tag, extra in {"native": [], "test": ["--test"]}.items():
    t = time.time()
    c, err = o.run(base, lib, "semio_framework_repo_dashboard", extra)
    res[tag] = c
    open(f"{o.SCR}/err-S0-{tag}.txt", "w").write(err)
    print(tag, sum(c.values()), len(c), round(time.time() - t, 1), "s")
pickle.dump(res, open(o.SCR + "/base.pkl", "wb"))
