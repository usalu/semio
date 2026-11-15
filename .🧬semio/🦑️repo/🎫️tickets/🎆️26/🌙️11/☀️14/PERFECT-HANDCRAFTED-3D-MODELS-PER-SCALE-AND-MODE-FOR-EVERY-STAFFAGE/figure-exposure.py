"""Outer area, exposed area, measured vs pose exposure per region (C29 numbers): python3 figure-exposure.py <slug> [scale...]"""
import json, subprocess, sys, glob, os, re
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
slug = sys.argv[1]; scales = sys.argv[2:] or ["50", "100"]
a = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/🔣️.json")[0]))["asset"]
decl = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
for sc in scales:
    out = subprocess.run([BUN, "volumes-check.ts", f"humans/{slug}", "--scale", sc, "--rules", "volumes-thermal-coverage"], cwd=T, capture_output=True, text=True).stdout
    g = lambda k: dict(re.findall(rf"info:{k}:([\w-]+)=([\d.e-]+)", out))
    outer, exp, meas, tgt = g("outerAreaM2"), g("exposedAreaM2"), g("exposureMeasured"), g("exposureTarget")
    print(f"1:{sc}")
    for k in outer:
        if k in decl: print(f"  {k:12s} declared {decl[k]:.3f}  outer {float(outer[k]):.3f} ({(float(outer[k])/decl[k]-1)*100:+.0f}%)  exposed {float(exp[k]):.3f}  exposure measured {float(meas[k]):.2f} target {float(tgt[k]):.2f}")
