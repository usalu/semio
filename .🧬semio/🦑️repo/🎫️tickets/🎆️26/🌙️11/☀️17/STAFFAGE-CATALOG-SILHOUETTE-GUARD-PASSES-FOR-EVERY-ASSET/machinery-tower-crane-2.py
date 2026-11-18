"""Tower crane 1:500: slew platform starts at 0.5 m above the slewing ring (ring drawn), head sheave, jib tip sheave and winch vent stack drawn as in the 1:200 (role body, the 1:500 detail pen is null)."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/tower-crane")
src = D("🏗️machinery/tower-crane")
for v in ("elevation", "front", "plan"):
    n = d.shape(500, v, "slew-platform"); o = d.obj(n)
    o["y"] = 0.5; o["h"] = 0.7; o["regions"] = ["house"]
    d.replace_obj(n, o)
    last = n
    for sid in ("slew-ring", "head-sheave", "jib-tip-sheave", "winch-vent-stack"):
        if v == "plan" and sid == "slew-ring": continue
        c = src.obj(src.shape(200, v, sid)); c["role"] = "body"
        d.insert_after(n, c)
d.commit()
