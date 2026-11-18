"""Excavator 1:50: lateral extent of the front and plan pins, bosses, eyes and feet equals the volume revolve (origin z to origin z plus length); the boss without volume solid is dropped from the front and the plan."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
vol = {x["id"]: x for x in d.val("asset", "volumes", "50", "body")}
IDS = ["boom-pin", "boom-cyl-foot", "boom-tip-boss", "boom-rod-eye", "stick-cyl-foot", "stick-root-boss", "stick-tip-boss", "stick-rod-eye", "bucket-cyl-foot", "bucket-pin", "bucket-ear"]
for v in ("front", "plan"):
    for sid in IDS:
        x = vol[sid]; z0 = x["origin"][2]; z1 = round(z0 + x["profile"][1][0], 4)
        n = d.shape(50, v, sid)
        d.setv(sp.child(sp.child(n, "solid"), "z0"), z0); d.setv(sp.child(sp.child(n, "solid"), "z1"), z1)
    d.delete_item(d.shape(50, v, "boom-root-boss"))
d.commit()
