"""Author 73: stroller 1:500 front and plan redrawn as an open frame (65 mm rails, 75 mm wheels, hood and child masses) instead of the filled sil-body block; elevation: rear tyre filled, basket added."""
from author_73_lib import *
slug = "parent-with-stroller"
T3 = 0.0325
def quad(id, pts, z0, z1, z, region="stroller-frame", pen=0.1, tone=0.5, bone="stroller"):
    return {"kind": "poly", "points": pts, "closed": True, "id": id, "bone": bone, "role": "body", "z": z, "filled": True, "tone": tone, "region": region, **({"penMm": pen} if pen else {}), "solid": {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}}
def shapes(view):
    z = {"front": [7.6, 7.41, 7.9, 8.0, 8.2, 7.45], "plan": [7.0, 7.01, 4.0, 7.6, 7.3, 6.9]}[view]
    out = []
    for s, a, b in (("l", 0.24 - T3, 0.24 + T3), ("r", -0.24 - T3, -0.24 + T3)):
        out.append(quad(f"chassis-{s}", [[0.22, -0.5475], [0.79, -0.5875], [0.79, -0.6525], [0.22, -0.6125]], a, b, z[0]))
        out.append(quad(f"push-bar-{s}", [[0.0, 0], [0.065, 0], [0.2525, -0.58], [0.1875, -0.58]], a, b, z[1]))
    out.append({"kind": "ellipse", "cx": 0, "cy": 0, "rx": T3, "ry": T3, "id": "handle-grip", "bone": "stroller", "role": "body", "z": z[2], "filled": True, "tone": 0.5, "region": "stroller-frame", "penMm": 0.1, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.25, "z1": 0.25}})
    out.append({"kind": "poly", "points": [[0, 0], [0.01, 0.1], [0.08, 0.2], [0.2, 0.23], [0.34, 0.18], [0.44, 0.08], [0.46, 0]], "closed": True, "id": "hood", "bone": "stroller-hood", "role": "body", "z": z[3], "filled": True, "tone": 0.45, "region": "stroller-fabric", "solid": {"kind": "extruded", "sweep": "w", "z0": -0.26, "z1": 0.26}})
    out.append({"kind": "poly", "points": [[-0.07, 0], [-0.0733, 0.1], [-0.095, 0.2433], [-0.08, 0.3], [-0.065, 0.3567], [-0.0067, 0.3533], [0.02, 0.34], [0.0467, 0.3267], [0.0567, 0.26], [0.08, 0.22], [0.1033, 0.18], [0.1233, 0.1233], [0.16, 0.1], [0.34, 0.06], [0.34, 0]], "closed": True, "id": "child-mass", "bone": "child", "role": "body", "z": z[4], "filled": True, "tone": 0.6, "regions": ["child-clothing", "child-skin"], "solid": {"kind": "extruded", "sweep": "w", "z0": -0.1, "z1": 0.1}})
    out.append({"kind": "rect", "x": 0.3, "y": -0.8, "w": 0.38, "h": 0.1, "id": "basket", "bone": "stroller", "role": "body", "z": z[5], "filled": True, "tone": 0.4, "region": "stroller-fabric", "solid": {"kind": "extruded", "sweep": "w", "z0": -0.2, "z1": 0.2}})
    return out
def wheels(view):
    out = []
    for kind, bone, c, r, z in (("rear", "wheel-rear", 0.28, 0.1592, 7.2), ("front", "wheel-front", 0.2, 0.0796, 7.5)):
        for s, k in (("l", 1), ("r", -1)):
            z0, z1 = sorted((k * (c - 0.0375), k * (c + 0.0375)))
            out.append({"kind": "ellipse", "cx": 0, "cy": 0, "rx": r, "ry": r, "id": f"tyre-{kind}-{s}", "bone": bone, "role": "body", "z": z if view == "front" else 7.1, "filled": True, "tone": 0.1, "region": "tyres", "solid": {"kind": "extruded", "sweep": "w", "z0": round(z0, 4), "z1": round(z1, 4)}})
    return out
def edit(g, rep):
    for view in ("front", "plan"):
        keep = [s for s in g[view] if s["id"] != "sil-body" and not s["id"].startswith("tyre-")]
        g[view] = keep + wheels(view) + shapes(view)
    for s in g["elevation"]:
        if s["id"] == "tyre-rear": s["filled"] = True; s["tone"] = 0.1
    if not any(s["id"] == "basket" for s in g["elevation"]):
        i = next(i for i, s in enumerate(g["elevation"]) if s["id"] == "canopy")
        g["elevation"].insert(i, {"id": "basket", "bone": "stroller", "kind": "path", "d": "M 0.3 -0.7 L 0.68 -0.7 L 0.64 -0.8 L 0.34 -0.8 Z", "role": "body", "z": 4.1, "filled": True, "tone": 0.4, "region": "stroller-fabric"})
print(patch_rep(slug, 500, edit))
