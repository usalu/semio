"""Ivy on trellis: the pen overrides of the stems (1:50), battens and leaf mat outlines (1:100) drew members 2 to 4 times wider than their volumes (penMm of 0.35 to 1.99 mm is 0.035 to 0.2 m at 1:100); each pen is the width of its volume member again, a mat outline the pen of its role; the wiggly stem polylines of 1:50 (up to 0.04 m beside the capsules) follow the capsule axes."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
d = mv.D("🌳️vegetation/ivy-on-trellis"); a = d.val("asset"); n = 0
for sc, rep in a["representations"].items():
    geo = rep.get("geometry", {}).get("elevation", [])
    vol = {s["id"]: s for s in a["volumes"][sc]["body"]} if "body" in a["volumes"][sc] else {}
    for s in geo:
        if "penMm" not in s or s.get("filled"): continue
        new = None
        if s["id"].startswith("stem-") and s["role"] == "detail":
            rs = [v for k, v in vol.items() if k.startswith(s["id"] + "-") and v["kind"] == "capsule"]
            if rs: new = round(2 * sum((v["r0"] + v["r1"]) / 2 for v in rs) / len(rs) * 1000 / int(sc), 3)
        elif s["id"].startswith("batten-h"):
            v = vol.get(s["id"]); new = round(v["size"][1] * 1000 / int(sc), 3) if v else None
        elif s["id"].startswith("batten-v"):
            v = vol.get(s["id"]) or vol.get("batten-v"); new = round(v["size"][0] * 1000 / int(sc), 3) if v else None
        elif s["id"].startswith("mat-") and s.get("closed"):
            new = rep["pens"].get(s["role"])
        if new is None or new >= s["penMm"]: continue
        d.setv(sp.walk(d.shape(sc, "elevation", s["id"]), ["penMm"]), new); n += 1
m = 0
for sc, rep in a["representations"].items():
    geo = rep.get("geometry", {}).get("elevation", [])
    vol = {s["id"]: s for s in a["volumes"][sc]["body"]} if "body" in a["volumes"][sc] else {}
    for s in geo:
        if s["id"].startswith("stem-") and s["kind"] == "poly" and (s["id"] + "-lo") in vol and (s["id"] + "-hi") in vol:
            lo, hi = vol[s["id"] + "-lo"], vol[s["id"] + "-hi"]
            pts = [lo["p0"][:2], lo["p1"][:2], hi["p1"][:2]]
            d.setv(sp.walk(d.shape(sc, "elevation", s["id"]), ["points"]), [[round(x, 4), round(y, 4)] for x, y in pts]); m += 1
d.commit(); print("pens fixed", n, "stem lines", m)
import subprocess, os
HERE = os.path.dirname(os.path.abspath(__file__))
print(subprocess.run(["python3", HERE + "/vegetation-inflate.py", "ivy-on-trellis", "50"], capture_output=True, text=True).stdout.strip())
