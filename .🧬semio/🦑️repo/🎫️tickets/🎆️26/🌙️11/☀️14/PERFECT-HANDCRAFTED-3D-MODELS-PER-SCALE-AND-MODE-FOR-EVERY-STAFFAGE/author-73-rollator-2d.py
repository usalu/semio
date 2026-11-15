"""Author 73: rollator 1:500 front and plan redrawn as an open frame (strokes at the pen floor, filled wheels/seat) instead of one filled block."""
from author_73_lib import *
slug = "person-with-rollator"
PEN = 0.1
def front(g, rep):
    old = g["front"]
    keep = [s for s in old if s["id"] != "frame"]
    W = 0.0325
    def rect(id, x0, y0, x1, y1, region, z, tone=0.4, bone="frame", **k):
        return {"kind": "rect", "x": round(x0, 4), "y": round(y0, 4), "w": round(x1 - x0, 4), "h": round(y1 - y0, 4), "id": id, "bone": bone, "role": "body", "z": z, "filled": True, "tone": tone, "region": region, **k}
    new = [
        rect("leg-l", 0.27 - W, 0.2, 0.27 + W, 0.85, "frame", 9.1, penMm=PEN),
        rect("leg-r", -0.27 - W, 0.2, -0.27 + W, 0.85, "frame", 9.1, penMm=PEN),
        rect("handlebar", -0.29, 0.848 - W, 0.29, 0.848 + W, "frame", 9.2, penMm=PEN),
        rect("lower-rail", -0.27, 0.25 - W, 0.27, 0.25 + W, "frame", 9.05, penMm=PEN),
        rect("seat", -0.2, 0.46, 0.2, 0.535, "seat-basket", 9.0, tone=0.35),
        rect("bag", -0.15, 0.3, 0.15, 0.46, "bag", 8.96, tone=0.78),
        rect("grip-l", 0.29 - 0.0385, 0.848 - 0.0385, 0.29 + 0.0385, 0.848 + 0.0385, "grips", 9.4, tone=0.15, penMm=PEN),
        rect("grip-r", -0.29 - 0.0385, 0.848 - 0.0385, -0.29 + 0.0385, 0.848 + 0.0385, "grips", 9.4, tone=0.15, penMm=PEN),
    ]
    for s, z0, z1 in (("l", 0.2365, 0.3135), ("r", -0.3135, -0.2365)):
        new.append({"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.1, "ry": 0.1, "id": f"tyre-{s}", "bone": "wheel-front", "role": "body", "z": 9.3, "filled": True, "tone": 0.12, "region": "tyres", "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}})
    g["front"] = keep + new
def plan(g, rep):
    old = g["plan"]
    keep = [s for s in old if s["id"] != "frame"]
    r = 0.0325
    def cap(id, x0, y0, x1, y1, region, z, rr=r, tone=0.4, **k):
        return {"kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": rr, "r1": rr, "id": id, "bone": "frame", "role": "body", "z": z, "filled": True, "tone": tone, "region": region, **k}
    new = [
        cap("rail-l", 0.38, 0.27, 0.86, 0.27, "frame", 0.8, penMm=PEN),
        cap("rail-r", 0.38, -0.27, 0.86, -0.27, "frame", 0.8, penMm=PEN),
        cap("handlebar", 0.375, -0.29, 0.375, 0.29, "frame", 1, penMm=PEN),
        cap("front-bar", 0.86, -0.27, 0.86, 0.27, "frame", 1, penMm=PEN),
        {"kind": "rect", "x": 0.42, "y": -0.19, "w": 0.435, "h": 0.38, "r": 0.03, "id": "seat", "bone": "frame", "role": "body", "z": 0.9, "filled": True, "tone": 0.35, "region": "seat-basket"},
        cap("grip-l", 0.24, 0.29, 0.38, 0.29, "grips", 4.1, rr=0.0385, tone=0.15, penMm=PEN),
        cap("grip-r", 0.24, -0.29, 0.38, -0.29, "grips", 4.1, rr=0.0385, tone=0.15, penMm=PEN),
    ]
    for w, bone in (("rear", "wheel-rear"), ("front", "wheel-front")):
        for s, z0, z1 in (("l", 0.2365, 0.3135), ("r", -0.3135, -0.2365)):
            new.append({"kind": "ellipse", "cx": 0, "cy": 0, "rx": 0.1, "ry": 0.1, "id": f"tyre-{w}-{s}", "bone": bone, "role": "body", "z": 0.5, "filled": True, "tone": 0.12, "region": "tyres", "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}})
    g["plan"] = keep + new
def elev(g, rep):
    if any(s["id"] == "lower-rail" for s in g["elevation"]): return
    i = next(i for i, s in enumerate(g["elevation"]) if s["id"] == "seat")
    g["elevation"].insert(i, {"id": "lower-rail", "bone": "frame", "kind": "poly", "points": [[0.015, 0.26], [0.5, 0.24]], "closed": False, "role": "body", "z": 9, "filled": False, "tone": 0.5, "region": "frame", "penMm": 0.1})
def both(g, rep): front(g, rep); plan(g, rep); elev(g, rep)
print(patch_rep(slug, 500, both))
