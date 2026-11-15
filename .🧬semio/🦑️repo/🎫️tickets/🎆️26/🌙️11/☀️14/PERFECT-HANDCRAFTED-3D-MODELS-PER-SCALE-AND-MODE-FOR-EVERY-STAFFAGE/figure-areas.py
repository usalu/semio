"""Exposed area per region against areaM2 (signed): python3 figure-areas.py <slug> [scale...]"""
import json, subprocess, sys, glob, os, re
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
slug = sys.argv[1]; scales = sys.argv[2:] or ["50", "100", "200", "500"]
a = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/🔣️.json")[0]))["asset"]
decl = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
for sc in scales:
    out = subprocess.run([BUN, "volumes-check.ts", f"humans/{slug}", "--scale", sc, "--rules", "volumes-thermal-coverage"], cwd=T, capture_output=True, text=True).stdout
    ex = dict(re.findall(r"info:exposedAreaM2:([\w-]+)=([\d.e-]+)", out))
    print(f"1:{sc}", "  ".join(f"{k} {float(v):.3f}/{decl[k]:.3f} ({(float(v)/decl[k]-1)*100:+.0f}%)" for k, v in ex.items() if k in decl))
