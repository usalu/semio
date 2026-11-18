"""Applies profile scales (id su sv) of solid-drawn plates to the snapshot and prints the guard line: python3 vegetation-try2.py <slug> <scale> <view> <snapshot> <id> <su> <sv> ..."""
import json, subprocess, shutil, sys, os
slug, scale, view, snap = sys.argv[1:5]; a = sys.argv[5:]
HERE = os.path.dirname(os.path.abspath(__file__))
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/" + slug + "/🔣️.json"
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
shutil.copy(snap, CAT)
rows = [{"scale": int(scale), "view": view, "id": a[i], "bone": a[i].replace("-blade", ""), "su": float(a[i + 1]), "sv": float(a[i + 2]), "iou0": 0, "iou1": 1} for i in range(0, len(a), 3)]
json.dump(rows, open(HERE + "/vegetation-work/fit-try.json", "w"))
subprocess.run(["python3", HERE + "/vegetation-fit-apply.py", slug, HERE + "/vegetation-work/fit-try.json", "0"], capture_output=True)
d = subprocess.run([B, HERE + "/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-diag.ts", slug, scale, view], capture_output=True, text=True, cwd="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript")
print(" ".join(a), [l for l in d.stdout.splitlines() if l.startswith("==")])
