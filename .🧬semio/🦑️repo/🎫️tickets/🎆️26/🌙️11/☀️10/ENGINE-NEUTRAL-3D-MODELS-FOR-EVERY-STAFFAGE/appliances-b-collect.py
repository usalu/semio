import json, subprocess, glob, os, re
T = os.path.dirname(os.path.abspath(__file__))
BUN = glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun"))[0]
OUT = os.path.join(T, "🗑️generated", "authoring-appliances-b")
slugs = "kitchen-extractor-hood laptop mvhr-unit pendant-lamp room-thermostat server-rack shower-head split-ac-indoor-unit thermostatic-radiator-valve toaster tower-server vacuum-cleaner".split()
rows = {}
for s in slugs:
    chk = subprocess.run([BUN, "volume-check.ts", s, "--json"], cwd=T, capture_output=True, text=True)
    c = json.loads(chk.stdout)
    val = subprocess.run([BUN, "appliances-b-validate.ts", s], cwd=T, capture_output=True, text=True).stdout
    open(os.path.join(OUT, s + "-validate.txt"), "w").write(val)
    poses = [l for l in val.splitlines() if l.startswith("pose:")]
    clips = [l for l in val.splitlines() if l.startswith("clip:")]
    lost = [l.split(" bbox")[0] for l in poses + clips if "lostContacts 0" not in l]
    vol = re.search(r"volume-\* (\d+)", val).group(1)
    ajv = re.search(r"^ajv (.*)$", val, re.M).group(1)[:60]
    rows[s] = dict(solids=c["solids"], bbox=[round(b["relativeError"] * 100, 1) for b in c["bbox"]], iou=[round(v["iou"], 3) for v in c["views"]], ok=c["ok"], volIssues=int(vol), ajv=ajv, poses=[l.split(" bbox")[0][5:] for l in poses], clips=len(clips), lost=lost, posebox={l.split(" bbox")[0][5:]: l.split(" bbox ")[1].split(" components")[0] for l in poses})
json.dump(rows, open(os.path.join(OUT, "summary.json"), "w"), indent=1, ensure_ascii=False)
for s, r in rows.items():
    print(s, r["solids"], r["bbox"], r["iou"], r["ok"], r["volIssues"], r["ajv"], r["poses"], r["clips"], r["lost"][:3])
