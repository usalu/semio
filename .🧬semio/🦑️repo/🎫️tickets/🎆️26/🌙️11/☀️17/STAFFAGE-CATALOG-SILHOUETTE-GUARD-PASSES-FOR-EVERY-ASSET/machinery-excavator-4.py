"""Excavator 1:50: hydraulic cylinder barrels and rods drawn as circular `member` shapes between the volume end points (flat ends like the volume revolves, true foreshortening in the front and the plan), rods also in the front and the plan."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
d = D("🏗️machinery/excavator")
SC = 50
vol = {x["id"]: x for x in d.val("asset", "volumes", str(SC), "body")}
CYL = [("boom-cyl-barrel", "boom-cyl", ["-left", "-right"]), ("boom-cyl-rod", "boom-rod", ["-left", "-right"]), ("stick-cyl-barrel", "stick-cyl", [""]), ("stick-cyl-rod", "stick-rod", [""]), ("bucket-cyl-barrel", "bucket-cyl", [""]), ("bucket-cyl-rod", "bucket-rod", [""])]
def member(sid, bone, tmpl, x0, x1, z, r):
    o = {"id": sid, "bone": bone, "role": tmpl["role"], "z": tmpl["z"], "filled": True, "tone": tmpl["tone"], "kind": "member",
         "from": {"x": x0, "y": 0, "z": z}, "to": {"x": x1, "y": 0, "z": z}, "section": {"kind": "circle", "d": round(2 * r, 4)}}
    for k in ("region", "regions", "material"):
        if k in tmpl: o[k] = tmpl[k]
    return o
for view in ("elevation", "front", "plan"):
    ids = {x["id"] for x in d.val("asset", "representations", str(SC), "geometry", view)}
    anchor = d.shape(SC, view, "boom-body")
    for vid, bone, sides in CYL:
        x = vol[vid]; r = x["profile"][0][1]; x0 = x["origin"][0]; x1 = round(x0 + x["profile"][1][0], 4); zs = [x["origin"][2]]
        if x.get("repeat"): zs.append(round(zs[0] + x["repeat"]["translation"][2], 4))
        names = [vid + s for s in sides] if view != "elevation" else [vid]
        zz = zs if view != "elevation" else zs[:1]
        for nm, z in zip(names, zz):
            tmpl = d.obj(d.shape(SC, view, nm if nm in ids else vid.replace("rod", "barrel") + (sides[0] if sides[0] else "")) if (nm in ids or True) else None)
            if nm in ids:
                d.replace_obj(d.shape(SC, view, nm), member(nm, bone if nm != vid or True else bone, tmpl, x0, x1, z, r))
            else:
                d.insert_after(anchor, member(nm, bone, tmpl, x0, x1, z, r))
d.commit()
