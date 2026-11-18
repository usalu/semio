"""Writes the DIMS lines of vegetation-bbox-plan.py: python3 vegetation-dims-apply.py <slug> <plan.txt>"""
import sys, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
slug = sys.argv[1]
d = mv.D("🌳️vegetation/" + slug)
key = {"width": "widthM", "height": "heightM", "depth": "depthM"}
for line in open(sys.argv[2]):
    p = line.split()
    if len(p) == 6 and p[0] == "DIMS" and p[1] == slug and p[3] != p[5]:
        d.setv(sp.walk(d.root, ["asset", "dimensions", key[p[2]]]), float(p[5])); print("dims", p[2], p[3], "->", p[5])
d.commit()
