import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
drop = ("kind", "x", "y", "w", "h", "r", "points", "closed", "d", "cx", "cy", "rx", "ry", "rotationDeg")
for sc in [int(a) for a in sys.argv[1:]]:
    vol = {it_id: d.obj(d.solid(sc, it_id)) for it_id in ("mirror-l", "mirror-r", "headlamp-l", "headlamp-r")
           if any(json.loads(d.t[v["s"]:v["e"]]) == it_id for it in d.at("asset", "volumes", str(sc), "body")["items"] for k, v in it["pairs"] if k == "id")}
    for sid, o in vol.items():
        if o.get("kind") != "ellipsoid": continue
        rx, ry, rz = o["radii"]; cx, cy, cz = o["transform"]["translation"]
        side = "l" if cz > 0 else "r"
        base = sid.split("-")[0]
        spec = {"elevation": (cx, cy, rx, ry, base), "front": (cz, cy, rz, ry, sid), "plan": (cx, cz, rx, rz, sid)}
        for view, (ax, ay, arx, ary, did) in spec.items():
            if view == "elevation" and side == "r": continue
            try:
                node = d.shape(sc, view, did)
            except KeyError:
                if view == "elevation":
                    d.add_shapes(sc, view, "body-shell", "body-shell", [{"kind": "ellipse", "cx": ax, "cy": ay, "rx": arx, "ry": ary, "id": did}], role="detail")
                continue
            d.replace_shape(node, {"kind": "ellipse", "cx": round(ax, 4), "cy": round(ay, 4), "rx": arx, "ry": ary, **{k: v for k, v in d.obj(node).items() if k not in drop}})
d.commit()
