import json, subprocess, glob, os, re
T = os.path.dirname(os.path.abspath(__file__))
BUN = glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun"))[0]
G = os.path.join(T, "🗑️generated", "authoring-components")
slugs = "cantilever-balcony curtain-wall-bay etics-brick-wall-12 etics-brick-wall-30 external-venetian-blind glass-balustrade glulam-beam masonry-wall-with-lintel panel-radiator sectional-garage-door sliding-door solid-brick-wall-1960 spiral-stair stair-flight steel-column tied-canopy tilt-window wheelchair-ramp".split()
rows = {}
for s in slugs:
    c = subprocess.run([BUN, "volume-check.ts", s, "--json"], cwd=T, capture_output=True, text=True)
    j = json.loads(c.stdout)
    v = subprocess.run([BUN, "authoring-components-validate.ts", s], cwd=T, capture_output=True, text=True).stdout
    issues = int(re.search(r"volume issues (\d+)", v).group(1))
    schema = "ok" if "schema ok" in v else "FAIL"
    poses = re.findall(r"pose ([\w-]+): (\d+) component", v)
    rest = int(re.search(r"rest: (\d+) component", v).group(1))
    rows[s] = {"solids": j["solids"], "bbox": [round(b["relativeError"] * 100, 1) for b in j["bbox"]], "iou": {x["view"]: round(x["iou"], 3) for x in j["views"]}, "ok": j["ok"], "issues": issues, "schema": schema, "rest": rest, "poses": poses}
json.dump(rows, open(os.path.join(G, "summary.json"), "w"), ensure_ascii=False, indent=1)
for s, r in rows.items():
    print(s, r["solids"], r["bbox"], r["iou"], "PASS" if r["ok"] else "MISS", r["issues"], r["schema"], "rest comps", r["rest"], [(p, c) for p, c in r["poses"]])
