"""Author 60: outer area, exposure and deviation per region (product validator after the C28 swap). python3 author-60-outer.py <slug> [scales]"""
import subprocess, sys, glob, os, re, json
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
slug = sys.argv[1]; scales = sys.argv[2:] or ["50", "100", "200", "500"]
a = json.load(open(glob.glob(f"/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/{slug}/🔣️.json")[0]))["asset"]
decl = {r["id"]: r["areaM2"] for r in a["thermal"]["regions"]}
for sc in scales:
    out = subprocess.run([BUN, "volumes-check.ts", f"humans/{slug}", "--scale", sc, "--rules", "volumes-thermal-coverage", "--product"], cwd=T, capture_output=True, text=True).stdout
    g = lambda k: dict(re.findall(rf"info:{k}:([\w-]+)=([\d.e-]+)", out))
    outer, expo, tgt = g("outerAreaM2"), g("exposureMeasured"), g("exposureTarget")
    print(f"1:{sc} " + "  ".join(f"{k} {float(v):.3f}/{decl[k]:.3f} ({(float(v)/decl[k]-1)*100:+.0f}%) e{float(expo.get(k,0)):.2f}/{float(tgt.get(k,1)):.2f}" for k, v in outer.items() if k in decl))
