import json, subprocess, glob, os, re
T = os.path.dirname(os.path.abspath(__file__))
BUN = glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun"))[0]
slugs = ["bar-stool","bookshelf","cafe-chair","dining-table","folding-deck-chair","height-adjustable-desk","office-swivel-chair","operating-table","park-bench","picnic-table","single-bed","three-seat-sofa"]
POSES = {}
out = {}
for s in slugs:
    patched = subprocess.run(["python3", "authoring-furniture-patch.py", s], cwd=T, capture_output=True, text=True).stdout.strip()
    res = {}
    for key, target in (("live", s), ("w", patched)):
        r = subprocess.run([BUN, "volume-check.ts", target, "--json"], cwd=T, capture_output=True, text=True)
        j = json.loads(r.stdout)
        res[key] = {"solids": j["solids"], "bbox": [round(b["relativeError"] * 100, 1) for b in j["bbox"]], "iou": [round(v["iou"], 3) for v in j["views"]], "ok": j["ok"]}
    out[s] = res
json.dump(out, open(os.path.join(T, "🗑️generated", "authoring-furniture", "collect.json"), "w"), indent=1)
for s, r in out.items(): print(s, r["live"]["solids"], "w:", r["w"]["bbox"], r["w"]["iou"], "ok" if r["w"]["ok"] else "FAIL", "| live:", r["live"]["iou"], "ok" if r["live"]["ok"] else "FAIL")
