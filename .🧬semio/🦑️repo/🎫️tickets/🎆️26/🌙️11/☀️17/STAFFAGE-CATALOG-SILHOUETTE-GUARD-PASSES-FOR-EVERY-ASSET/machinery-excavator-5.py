"""Excavator 1:50: the second boom cylinder (right side) is labelled as a member too, because the front and the plan draw one circular member shape per cylinder side and the structure coverage rule wants one member entry per drawn member of a bone."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
for sid in ("boom-cyl-barrel", "boom-cyl-rod"):
    n = d.find("50", f"member-{sid}", "structure")
    o = d.obj(n); o["id"] = f"member-{sid}-right"
    d.insert_after(n, o)
d.commit()
