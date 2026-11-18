"""Date palm: the drawn fronds are flat plates in the (u, w) plane of their bone (extruded profile), the volume fronds were ribbons tilted by 7 to 8 degrees and arched by 0.1 m in the thin direction; the volume ribbons lie in that plane again (rotation of the sections 90 degrees about the bone normal, no offset along the thin direction and none across: the plate is symmetric about the bone axis like the drawing)."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
d = mv.D("🌳️vegetation/date-palm"); a = d.val("asset"); n = 0
for sc in ("50", "100", "200"):
    for i, s in enumerate(a["volumes"][sc]["body"]):
        if s["kind"] != "loft" or not s["bone"].startswith("fr-"): continue
        for k, sec in enumerate(s["sections"]):
            base = ["asset", "volumes", sc, "body", str(i), "sections", str(k)]
            d.setv(sp.walk(d.root, base + ["at", "1"]), 0.0)
            d.setv(sp.walk(d.root, base + ["at", "2"]), 0.0)
            d.setv(sp.walk(d.root, base + ["rotation"]), [0.70711, 0, 0.70711, 0]); n += 1
d.commit(); print("fronds flattened", n)
