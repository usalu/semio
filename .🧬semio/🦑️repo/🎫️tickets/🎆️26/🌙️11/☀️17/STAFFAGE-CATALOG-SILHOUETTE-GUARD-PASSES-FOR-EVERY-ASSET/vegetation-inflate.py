"""Closes hairline seams between the lofts of one scale: every non-slender loft section grows by 0.75 cell of the scale (f/4 * 0.75 = 3/16 feature) in rx and ry.  python3 vegetation-inflate.py <slug> <scale>..."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
slug = sys.argv[1]
d = mv.D("🌳️vegetation/" + slug); a = d.val("asset"); n = 0
for sc in sys.argv[2:]:
    grow = FEATURE[sc] * 3 / 16
    for i, s in enumerate(a["volumes"][sc]["body"]):
        if s["kind"] != "loft" or s.get("slender"): continue
        for k, sec in enumerate(s["sections"]):
            for key in ("rx", "ry"):
                node = sp.walk(d.root, ["asset", "volumes", sc, "body", str(i), "sections", str(k), key])
                d.setv(node, round(sec[key] + grow, 5)); n += 1
d.commit(); print("inflated", n, "values")
