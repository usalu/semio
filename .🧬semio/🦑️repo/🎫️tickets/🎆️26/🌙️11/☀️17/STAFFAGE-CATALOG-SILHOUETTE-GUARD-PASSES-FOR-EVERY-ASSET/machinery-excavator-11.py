"""Excavator 1:200 and 1:500: hydraulic cylinder barrels and rods drawn as circular `member` shapes between the volume end points in the three views (rods and, at 1:500, all cylinders newly drawn), the right boom cylinder labelled as a member in the structure."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
CYL = [("boom-cyl-barrel", "boom-cyl", ["-left", "-right"]), ("boom-cyl-rod", "boom-rod", ["-left", "-right"]), ("stick-cyl-barrel", "stick-cyl", [""]), ("stick-cyl-rod", "stick-rod", [""]), ("bucket-cyl-barrel", "bucket-cyl", [""]), ("bucket-cyl-rod", "bucket-rod", [""])]
def member(sid, bone, tmpl, x0, x1, z, r):
    o = {"id": sid, "bone": bone, "role": tmpl["role"], "z": tmpl["z"], "filled": True, "tone": tmpl["tone"], "kind": "member",
         "from": {"x": x0, "y": 0, "z": z}, "to": {"x": x1, "y": 0, "z": z}, "section": {"kind": "circle", "d": round(2 * r, 4)}}
    for k in ("region", "regions", "material"):
        if k in tmpl: o[k] = tmpl[k]
    return o
for SC in (200, 500):
    d = D("🏗️machinery/excavator")
    vol = {x["id"]: x for x in d.val("asset", "volumes", str(SC), "body")}
    for view in ("elevation", "front", "plan"):
        ids = {x["id"] for x in d.val("asset", "representations", str(SC), "geometry", view)}
        anchor = d.shape(SC, view, "boom-body")
        for vid, bone, sides in CYL:
            x = vol[vid]; r = x["profile"][0][1]; x0 = x["origin"][0]; x1 = round(x0 + x["profile"][1][0], 4); zs = [x["origin"][2]]
            if x.get("repeat"): zs.append(round(zs[0] + x["repeat"]["translation"][2], 4))
            names = [vid + s for s in sides] if view != "elevation" else [vid]
            zz = zs if view != "elevation" else zs[:1]
            base = vid.replace("rod", "barrel")
            for nm, z in zip(names, zz):
                tid = next((t for t in (nm, base + nm[len(vid):], base) if t in ids), None)
                tmpl = d.obj(d.shape(SC, view, tid)) if tid else {"role": "body", "z": 3.5, "tone": 0.4, "region": "hydraulics", "material": "chrome" if "rod" in vid else "paint-steel"}
                if SC == 500: tmpl = dict(tmpl, role="body")
                if nm in ids: d.replace_obj(d.shape(SC, view, nm), member(nm, bone, tmpl, x0, x1, z, r))
                else: d.insert_after(anchor, member(nm, bone, tmpl, x0, x1, z, r))
    d.commit()
    d = D("🏗️machinery/excavator")
    for sid in ("boom-cyl-barrel", "boom-cyl-rod"):
        n = d.find(str(SC), f"member-{sid}", "structure")
        o = d.obj(n); o["id"] = f"member-{sid}-right"
        d.insert_after(n, o)
    d.commit()
