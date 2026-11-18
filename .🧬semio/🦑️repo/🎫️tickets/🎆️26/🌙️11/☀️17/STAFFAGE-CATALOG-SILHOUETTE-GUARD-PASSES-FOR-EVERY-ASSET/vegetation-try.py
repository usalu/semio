"""Tries profile scales and prints the guard line of one view: python3 vegetation-try.py <slug> <scale> <view> <id> <su> <sv> [<id> <su> <sv> ...]  (restores vegetation-work/backup3/<slug>.try.json before; saves the current file first as that)"""
import sys, json, subprocess, shutil, os
slug, scale, view = sys.argv[1:4]
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/" + slug + "/🔣️.json"
HERE = os.path.dirname(os.path.abspath(__file__)); G = HERE + "/vegetation-work/"
snap = G + "backup3/" + slug + ".try.json"
if not os.path.exists(snap): shutil.copy(CAT, snap)
shutil.copy(snap, CAT)
rows = []
a = sys.argv[4:]
for i in range(0, len(a), 3): rows.append({"scale": int(scale), "view": view, "id": a[i], "bone": a[i], "su": float(a[i + 1]), "sv": float(a[i + 2]), "iou0": 0, "iou1": 1})
json.dump(rows, open(G + "fit-manual.json", "w"))
subprocess.run(["python3", HERE + "/vegetation-fit-apply.py", slug, G + "fit-manual.json", "0"], capture_output=True)
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
r = subprocess.run([B, HERE + "/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/silhouette-diag.ts", slug, scale, view], capture_output=True, text=True, cwd="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript")
print(" ".join(a), [l for l in r.stdout.splitlines() if l.startswith("==")][0:1])
