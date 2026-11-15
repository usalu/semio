"""Author 60: edit-and-measure harness. python3 author-60-exp.py <asset> <scale> '<python statement on s (the volume dict)>' [--keep]"""
import json, subprocess, sys, re, glob, os, shutil
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
asset, sc, stmt = sys.argv[1], sys.argv[2], sys.argv[3]
keep = "--keep" in sys.argv
F = f"{T}/volumes/{asset}.json"
raw = open(F).read()
src = json.loads(raw)
s = src["volumes"][sc]
exec(stmt)
json.dump(src, open(F, "w"), ensure_ascii=False, indent=2)
try:
    out = subprocess.run([BUN, "volumes-check.ts", asset, "--scale", sc, "--product"], cwd=T, capture_output=True, text=True).stdout
finally:
    if not keep:
        open(F, "w").write(raw)
for l in out.splitlines():
    if l.startswith(("FAIL", "ok")):
        parts = l.split()
        keys = re.findall(r"(iou:\w+=[\d.]+|areaDeviation:[\w-]+=[\d.]+|info:exposedAreaM2:[\w-]+=[\d.]+)", l)
        print(parts[0], parts[1], " ".join(keys) if keys else "")
