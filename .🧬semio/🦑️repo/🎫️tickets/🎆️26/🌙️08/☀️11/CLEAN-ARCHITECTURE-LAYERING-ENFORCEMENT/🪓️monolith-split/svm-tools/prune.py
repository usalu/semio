"""Minimise auto-widened visibility with the full-fidelity oracle (only for crates whose closure is std-only, so typeck is complete).

usage: prune.py <plan.py> [cfg,cfg]  -> prints the no_widen strings that the oracle proves unnecessary."""
import json, os, re, shutil, subprocess, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import oracle
from split import load_plan, read, write, ROOT, F, WORK


def main():
    plan_path = sys.argv[1]
    cfgs = sys.argv[2].split(",") if len(sys.argv) > 2 else ["lib", "test"]
    plan = load_plan(plan_path)
    o = plan["oracle"]
    name = plan["name"]
    real_dir = os.path.dirname(ROOT + "/" + plan["real"])
    new_dirs = [os.path.join(real_dir, c["folder"]) for c in plan["children"]]
    wa, wb = f"{WORK}/{name}/PA", f"{WORK}/{name}/PB"
    for w in (wa, wb):
        oracle.under(w, WORK)
        if os.path.exists(w):
            shutil.rmtree(w)
    for tree in o["trees"]:
        oracle.copy_tree(tree, wa, skip_dirs=new_dirs)
        oracle.copy_tree(tree, wb, skip_dirs=new_dirs)
    shutil.copyfile(F + "/" + plan["orig"], os.path.join(wa, plan["real"]))
    oracle.patch_tree(wa, o["crate"])
    base = {}
    for c in cfgs:
        e = oracle.run_rustc(wa, plan["real_lib"], o["crate"], oracle.CFG[c], []).replace(wa, "<S>")
        base[c] = collections.Counter((k[0], k[1], k[2]) for k in oracle.parse(e))

    def stage_and_check(extra):
        env = dict(os.environ, SVM_EXTRA_NO_WIDEN=json.dumps(extra))
        subprocess.run([sys.executable, HERE + "/split.py", plan_path, "--quiet"], env=env, check=True, capture_output=True)
        stage = f"{WORK}/{name}/stage"
        for dp, dn, fn in os.walk(stage):
            for f in fn:
                p = os.path.join(dp, f)
                dst = os.path.join(wb, os.path.relpath(p, stage))
                oracle.under(dst, WORK)
                os.makedirs(os.path.dirname(dst), exist_ok=True)
                shutil.copyfile(p, dst)
        oracle.patch_tree(wb, o["crate"])
        res = {}
        for c in cfgs:
            e = oracle.run_rustc(wb, plan["real_lib"], o["crate"], oracle.CFG[c], []).replace(wb, "<S>")
            res[c] = collections.Counter((k[0], k[1], k[2]) for k in oracle.parse(e))
        return all(res[c] == base[c] for c in cfgs), res

    ok, _ = stage_and_check([])
    assert ok, "starting point is not identical to the baseline"
    wj = json.load(open(f"{WORK}/{name}/widen.json", encoding="utf-8"))
    reasons = []
    for k, v in wj.items():
        key = v.split(" (used in")[0].split(" (named in")[0]
        if "forced" in v:
            continue
        reasons.append(key)
    removable = []
    for r in reasons:
        ok, res = stage_and_check(removable + [r])
        print(("REMOVABLE " if ok else "needed    ") + r, flush=True)
        if ok:
            removable.append(r)
    ok, res = stage_and_check(removable)
    print("final combined:", "IDENTICAL" if ok else "DIFFERENT")
    json.dump(removable, open(f"{F}/🗑️generated/svm-{name}-prune.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(json.dumps(removable, ensure_ascii=False))


if __name__ == "__main__":
    main()
