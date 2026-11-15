"""Reviewer 2: runs the product validator (C29 outer area) per asset of my decisions and compares info:outerAreaM2 at 1:50 with the decided areaM2 and with my closed-surface sum.
Usage: python3 areareview2-cross.py [slug ...]   (all decided assets when none given); output 🗑️generated/areareview2/cross.json and cross.txt"""
import concurrent.futures, glob, importlib.util, json, os, subprocess, sys
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
GEN = os.path.join(T, "🗑️generated", "areareview2")
s = importlib.util.spec_from_file_location("dec2", os.path.join(T, "areareview2-decisions.py")); dec = importlib.util.module_from_spec(s); s.loader.exec_module(dec)
slugs = sys.argv[1:] or sorted({k[0] for k in dec.R})

def run(slug):
    p = glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]
    cat = os.path.basename(os.path.dirname(os.path.dirname(p)))
    cat = cat.split("️")[-1] or cat
    cat = os.path.basename(os.path.dirname(p))
    cat = os.path.basename(os.path.dirname(p)) if False else p.split("/")[-3]
    cat = "".join(ch for ch in cat if ch.isascii() and ch.isalpha())
    tmp = os.path.join(GEN, f"vc-{slug}.json")
    subprocess.run(f'"{BUN}" volumes-check.ts {cat}/{slug} --json --product > "{tmp}" 2>/dev/null', shell=True, cwd=T)
    try:
        d = json.load(open(tmp))
    except Exception:
        return slug, None
    doc = json.load(open(p))["asset"]
    area = {r["id"]: r["areaM2"] for r in doc["thermal"]["regions"]}
    res = {}
    for m in d["measurements"]:
        n = m.get("name", "")
        if n.startswith("info:outerAreaM2:") and m["scale"] == 50:
            res[n.split(":", 2)[2]] = m["value"]
    return slug, {r: (area.get(r), v) for r, v in res.items()}

with concurrent.futures.ThreadPoolExecutor(4) as ex:
    results = dict(ex.map(run, slugs))
json.dump(results, open(os.path.join(GEN, "cross.json"), "w"), indent=1)
lines = []
for slug, r in sorted(results.items()):
    if r is None:
        lines.append(f"{slug}: validator failed")
        continue
    for reg, (a, v) in sorted(r.items()):
        if a and abs(v - a) / a > 0.10 and (slug, reg) in dec.R:
            lines.append(f"{slug} {reg}: decided {a}  validator outer {v:.3f}  ratio {v/a:.2f}  act {dec.R[(slug, reg)][0]}")
open(os.path.join(GEN, "cross.txt"), "w").write("\n".join(lines))
print("\n".join(lines))
