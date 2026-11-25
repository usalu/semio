"""🪑️ Ops writer of `used-bentwood-dining-chairs`: the two upper chairs turn by 180 degrees about the lateral axis when they are carried and set down, and every front and plan shape of them was a flat drawing of the stored, upside-down chair. Seat ring, cane, seat and leg ring become their plan outline swept over the thickness of the side elevation; legs, bow and inner loop become their front outline swept along the normal of their bone over the thickness the side elevation draws, and the plan takes the same bodies. At 1:200 the bow that ran into the rear legs is the bow alone and the rear legs are round bars on their own bone; at 1:500 the strokes become the tubes themselves: bow with rear legs and the front legs are the lofts of the 1:500 volume, the ring is swept: `python3 make/used-bentwood-dining-chairs.py [--with-500]` writes its ops; without the flag the 1:500 band stays as it is, because the silhouette guard compares a slender solid with a loft drawing at the pen width (front 0.905 against 0.92, request `multiview MU` in `requests.md`)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *
a = asset("used-bentwood-dining-chairs")
frames, ops, after = {k: ((x - 0.27 if k.endswith("-b") else x + 0.27 if k.endswith("-d") else x), y, z, g) for k, (x, y, z, g) in rest3(a).items()}, [], []
only_plan, only_front, twins = [], [], {}
for c, z0 in (("b", 0.27), ("d", -0.27)):
    paint = lambda scale, view, sid: {k: v for k, v in shape(a, scale, view, sid).items() if k not in GEOMETRY}
    for part in ("seat-ring", "cane", "seat"):
        only_plan.append(f"{c}-{part}")
    for leg, two in (("rear-leg", "rear-legs"), ("front-leg", "front-legs")):
        for side in "lr":
            only_front.append(f"{c}-{leg}-{side}")
            twins[f"{c}-{leg}-{side}"] = f"{c}-{two}"
    only_front += [f"{c}-bow", f"{c}-inner-loop"]
    f_ring = frames[f"leg-ring-{c}"]
    for scale in ("50", "100", "200", "500"):
        ring = shape(a, scale, "plan", f"{c}-leg-ring")
        if "solid" in ring:
            outer, flat = ring_of(ring), geometry(ring)
        else:
            conv = lambda run: [[local(f_ring, (x, 0))[0], round(w - z0, 6)] for x, w in run]
            outer = conv(ring_of(ring))
            flat = {"kind": "path", "d": " ".join("M " + " L ".join(f"{u} {w}" for u, w in conv(run)) + " Z" for run in segments(ring["d"]))} if ring["kind"] == "path" else {"kind": "poly", "points": outer, "closed": True}
        t = {"50": 0.012, "100": 0.012, "200": 0.015, "500": 0.012}[scale]
        thick = {"filled": True} if scale == "500" else {}
        ops.append(op(scale, "plan", f"{c}-leg-ring", {**paint(scale, "plan", f"{c}-leg-ring"), **flat, "solid": extruded("v", -t, t)}))
        ops.append(op(scale, "front", f"{c}-leg-ring", {**paint(scale, "front", f"{c}-leg-ring"), **thick, "kind": "poly", "points": outer, "closed": True, "solid": extruded("v", -t, t)}))
    for scale in ("50", "100", "200", "500"):
        parts = ["seat"] if scale in ("200", "500") else ["seat-ring", "cane"]
        for part in parts:
            f_seat = frames[f"seat-{c}"]
            s = shape(a, scale, "plan", f"{c}-{part}")
            outer = ring_of(s) if "solid" in s else [[local(f_seat, (x, 0))[0], round(w - z0, 6)] for x, w in ring_of(s)]
            e = box(shape(a, scale, "elevation", f"{c}-{part}"))
            after.append(op(scale, "front", f"{c}-{part}", {**paint(scale, "front", f"{c}-{part}"), "kind": "poly", "points": outer, "closed": True, "solid": extruded("v", e[2], e[3])}))
    for scale in ("50", "100"):
        for sid in [f"{c}-{leg}-{side}" for leg in ("rear-leg", "front-leg") for side in "lr"] + [f"{c}-bow", f"{c}-inner-loop"]:
            after.append({"op": "solid-from", "scale": scale, "view": "plan", "id": sid, "from": "front"})
    for sid in (f"{c}-front-leg-l", f"{c}-front-leg-r", f"{c}-inner-loop"):
        after.append({"op": "solid-from", "scale": "200", "view": "plan", "id": sid, "from": "front"})
    import math
    fr, fb = frames[f"rear-legs-{c}"], frames[f"back-{c}"]
    def on_back(u, v=0.0):
        """🦵️ A point of the rear legs in the frame of the back, the bone their bodies are bound to at 1:200 and 1:500."""
        g = math.radians(fr[3])
        return local(fb, (fr[0] + math.cos(g) * u - math.sin(g) * v, fr[1] + math.sin(g) * u + math.cos(g) * v))
    for view in ("front", "plan"):
        p = paint("200", view, f"{c}-frame-bow")
        after.append({"op": "bow-200", "view": view, "c": c, "shape": p})
        for side, w in (("l", 1), ("r", -1)):
            (u0, v0), (u1, v1) = on_back(0.001), on_back(0.4558, 0.0045)
            bar = {**p, "id": f"{c}-rear-leg-{side}", "z": p["z"] - 0.5, "kind": "loft", "sections": [{"at": [u0, v0, w * 0.2], "rx": 0.015, "ry": 0.015, "rotation": SQUARE}, {"at": [u1, v1, w * 0.13], "rx": 0.015, "ry": 0.015, "rotation": SQUARE}]}
            after.append({"op": "shape-add", "scale": "200", "view": view, "after": f"{c}-frame-bow", "shape": bar})
    solids = {v["id"]: v for v in a["volumes"]["500"]["body"]}
    for view in ("front", "plan"):
        for sid in (f"{c}-frame-bow", f"{c}-front-leg-l", f"{c}-front-leg-r"):
            after.append(op("500", view, sid, {**paint("500", view, sid), "filled": True, "kind": "loft", "sections": solids[sid]["sections"]}))
        for side in "lr":
            after.append({"op": "shape-drop", "scale": "500", "view": view, "id": f"{c}-rear-leg-{side}"})
done = []
for o in after:
    if o["op"] != "bow-200":
        done.append(o)
        continue
    c = o["c"]
    z0 = 0.27 if c == "b" else -0.27
    f = frames[f"back-{c}"]
    s = shape(a, "200", "front", f"{c}-frame-bow")
    if "solid" in s:
        pts = [list(p) for p in s["points"]]
    else:
        import math
        lean = math.sin(math.radians(f[3]))
        world = [p for p in s["points"] if p[1] < 0.5]
        pts = [[round((y - f[1]) / lean * (1 - 0.0086 / 0.4), 6), round(x - z0, 6)] for x, y in world]
    done.append(op("200", o["view"], f"{c}-frame-bow", {**o["shape"], "kind": "poly", "points": pts, "closed": True, "solid": extruded("v", -0.015, 0.015)}))
rules = [
    {"views": ["plan"], "bones": ["seat-b", "seat-d"], "mode": "axis", "only": only_plan},
    {"views": ["front"], "bones": [f"{b}-{c}" for c in "bd" for b in ("rear-legs", "front-legs", "back")], "mode": "axis", "nearDeg": 18, "only": only_front, "scales": ["50", "100", "200"], "twins": twins, "ranges": {f"{c}-{part}": [-0.005, 0.005] for c in "bd" for part in ("bow", "inner-loop")}},
]
if "--with-500" not in sys.argv:
    ops, done = [o for o in ops if o.get("scale") != "500"], [o for o in done if o.get("scale") != "500"]
    rules = [{**r, "scales": ["50", "100", "200"]} for r in rules]
write("used-bentwood-dining-chairs", ops, solidify=rules, after=done)
