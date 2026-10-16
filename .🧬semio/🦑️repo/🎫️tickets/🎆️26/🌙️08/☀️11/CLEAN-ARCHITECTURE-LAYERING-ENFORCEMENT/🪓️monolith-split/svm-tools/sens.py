"""Oracle sensitivity table (svm): for every `pub(super)` added by the split, drop it in a sandbox copy and report whether the patched differential notices.
usage: sens.py <plan.py> [cfg,cfg]. A `detected` row proves the widening is necessary; an `undetected` row is covered only by the static reference proof."""
import collections, json, os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import oracle
from split import load_plan, read, write, ROOT, F, WORK


def main():
    plan = load_plan(sys.argv[1])
    cfgs = sys.argv[2].split(",") if len(sys.argv) > 2 else ["lib", "test"]
    o = plan["oracle"]
    name = plan["name"]
    wa, wb = f"{WORK}/{name}/SA", f"{WORK}/{name}/SB"
    for w in (wa, wb):
        oracle.under(w, WORK)
        if os.path.exists(w):
            shutil.rmtree(w)
    real_dir = os.path.dirname(ROOT + "/" + plan["real"])
    new_dirs = [os.path.join(real_dir, c["folder"]) for c in plan["children"]]
    for tree in o["trees"]:
        oracle.copy_tree(tree, wa, skip_dirs=new_dirs)
        oracle.copy_tree(tree, wb)
    shutil.copyfile(F + "/" + plan["orig"], os.path.join(wa, plan["real"]))
    oracle.patch_tree(wa, o["crate"])
    oracle.patch_tree(wb, o["crate"])
    ct = o.get("crate_type", "lib")

    def keys(root, c):
        e = oracle.run_rustc(root, plan["real_lib"], o["crate"], oracle.CFG[c], [], ct).replace(root, "<S>")
        return collections.Counter((k[0], k[1], k[2]) for k in oracle.parse(e))

    base = {c: keys(wa, c) for c in cfgs}
    files = [os.path.join(wb, os.path.relpath(f"{real_dir}/{c['folder']}/🦀️.rs", ROOT)) for c in plan["children"]] + [os.path.join(wb, plan["real"])]
    rows = []
    for fp in files:
        oracle.under(fp, WORK)
        lines = read(fp).split("\n")
        for i, l in enumerate(lines):
            if "pub(super) " not in l:
                continue
            mutated = list(lines)
            mutated[i] = l.replace("pub(super) ", "", 1)
            write(fp, "\n".join(mutated))
            detected = any(keys(wb, c) != base[c] for c in cfgs)
            rows.append((os.path.basename(os.path.dirname(fp)), i + 1, l.strip()[:90], detected))
            write(fp, "\n".join(lines))
    det = sum(1 for r in rows if r[3])
    print(f"[{name}] widened declarations probed={len(rows)} detected-by-oracle={det} undetected={len(rows) - det}")
    out = [f"{'DETECTED  ' if r[3] else 'undetected'} {r[0]}:{r[1]}  {r[2]}" for r in rows]
    write(f"{F}/🗑️generated/svm-{name}-sensitivity.txt", "\n".join(out) + "\n")
    for l in out:
        print("  ", l)


if __name__ == "__main__":
    main()
