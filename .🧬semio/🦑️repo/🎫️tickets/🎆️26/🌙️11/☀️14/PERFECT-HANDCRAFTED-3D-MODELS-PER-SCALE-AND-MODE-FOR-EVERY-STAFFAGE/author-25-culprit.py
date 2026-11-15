"""Usage: python3 author-25-culprit.py <asset> <slug> <scale> — greedy search for the structure solids that stick out of the body in plan/elevation (removes solids one by one in a scratch copy, then restores the source)."""
import json, subprocess, sys, glob, shutil, os, re
asset, slug, sc = sys.argv[1], sys.argv[2], sys.argv[3]
here = os.path.dirname(os.path.abspath(__file__))
bun = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
src = f"{here}/volumes/{asset.split('/')[0]}/{slug}.json"
backup = f"{here}/🗑️generated/author-25/{slug}.culprit-backup.json"
shutil.copy(src, backup)
def measure():
    out = subprocess.run([bun, "volumes-check.ts", asset, "--scale", sc, "--rules", "volumes-structure-coverage", "--quiet"], cwd=here, capture_output=True, text=True).stdout
    return {v: float(x) for v, x in re.findall(r"outsideBodyM:(\w+)=([0-9.e-]+)", out)}
try:
    d = json.load(open(backup))
    ids = [s["id"] for s in d["volumes"][sc]["structure"]]
    removed = []
    base = measure()
    print("base", base)
    for _ in range(8):
        worst = max(base, key=base.get)
        if base[worst] <= 0.0075 * (int(sc) / 50) * 1.0:
            break
        best = None
        for i in ids:
            if i in removed:
                continue
            d2 = json.load(open(backup))
            d2["volumes"][sc]["structure"] = [s for s in d2["volumes"][sc]["structure"] if s["id"] not in removed + [i]]
            json.dump(d2, open(src, "w"))
            m = measure()
            if best is None or m.get(worst, 9) < best[1].get(worst, 9):
                best = (i, m)
        removed.append(best[0]); base = best[1]
        print("remove", best[0], base)
finally:
    shutil.copy(backup, src)
print("culprits", removed)
