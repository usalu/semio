"""Date palm 1:50: the rings of the upper stem were drawn 0.93 mm wide (4.7 cm) and 3 cm beyond the bark; they get the pen of the lower rings and end inside the stem."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
d = mv.D("🌳️vegetation/date-palm")
node = d.shape("50", "elevation", "stem-3-rings")
cur = d.obj(node)
path = " ".join(f"M {x:g} 0.2 Q {x + (0.0368 if k % 2 == 0 else -0.0368):.4f} 0 {x:g} -0.2" for k, x in enumerate((0.3004, 0.7504, 1.2004, 1.6504, 2.1004)))
d.set(node, "d", path); d.set(node, "penMm", 0.222)
d.commit(); print("rings")
