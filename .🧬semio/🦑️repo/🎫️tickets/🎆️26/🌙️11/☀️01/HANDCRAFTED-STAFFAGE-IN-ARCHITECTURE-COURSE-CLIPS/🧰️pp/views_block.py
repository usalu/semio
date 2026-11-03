def rect(sid, x, y, w, h, z, tone, region_, mat, role="body", r=None, filled=True):
    s = {"id": sid, "bone": "case", "role": role, "z": z, "filled": filled, "tone": tone, "kind": "rect", "x": x, "y": y, "w": w, "h": h}
    if r:
        s["r"] = r
    s["region"] = region_
    s["material"] = mat
    return s


def path(sid, d, z, tone, region_=None, mat=None, regions=None, role="body", filled=True):
    s = {"id": sid, "bone": "case", "role": role, "z": z, "filled": filled, "tone": tone, "kind": "path", "d": d}
    if regions:
        s["regions"] = regions
    else:
        s["region"] = region_
    if mat:
        s["material"] = mat
    return s


def lines(sid, segments, z, region_):
    d = " ".join(f"M {a:.4f} {b:.4f} L {c:.4f} {e:.4f}" for a, b, c, e in segments)
    return {"id": sid, "bone": "case", "role": "detail", "z": z, "filled": False, "kind": "path", "d": d, "region": region_}


CASE_PATH = "M -0.225 0.012 L 0.225 0.012 L 0.225 0.443 C 0.225 0.448 0.222 0.45 0.217 0.45 L -0.217 0.45 C -0.222 0.45 -0.225 0.448 -0.225 0.443 Z"
S = "side-panel"
ALL_CASE = ["side-panel", "rear-panel", "front-bezel", "psu-vent", "cpu-grille"]

el50 = [
    rect("foot-front", 0.16, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-rear", -0.2, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    path("case-side", CASE_PATH, 3.1, 0.55, S, "chassis-steel"),
    rect("front-bezel", 0.2, 0.024, 0.025, 0.414, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, 0.012, 0.012, 0.438, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.225, 0.33, 0.026, 0.108, 3.4, 0.38, "psu-vent", "chassis-steel"),
    rect("cpu-grille", -0.07, 0.202, 0.17, 0.13, 3.4, 0.38, "cpu-grille", "chassis-steel"),
    lines("grille-slits", [(-0.04 + 0.03 * i, 0.212, -0.04 + 0.03 * i, 0.322) for i in range(5)], 3.6, "cpu-grille"),
    lines("psu-slits", [(-0.225, 0.342 + 0.009 * i, -0.199, 0.342 + 0.009 * i) for i in range(11)], 3.6, "psu-vent"),
    lines("intake-slits", [(0.2, 0.062 + 0.03 * i, 0.225, 0.062 + 0.03 * i) for i in range(12)], 3.6, "front-bezel"),
    {"id": "fan-ring", "bone": "case", "role": "detail", "z": 3.6, "filled": False, "kind": "ellipse", "cx": -0.222, "cy": 0.2, "rx": 0.003, "ry": 0.05, "region": "rear-panel"},
]
fr50 = [
    rect("foot-left", 0.06, 0, 0.03, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-right", -0.09, 0, 0.03, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("front-bezel", -0.1, 0.012, 0.2, 0.438, 3.1, 0.18, "front-bezel", "bezel-abs", r=0.005),
    rect("wall-left", 0.092, 0.012, 0.008, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("wall-right", -0.1, 0.012, 0.008, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("intake-mesh", -0.085, 0.05, 0.17, 0.34, 3.3, 0.3, "front-bezel", "bezel-abs", role="detail"),
    lines("intake-ribs", [(-0.085, 0.062 + 0.03 * i, 0.085, 0.062 + 0.03 * i) for i in range(12)], 3.5, "front-bezel"),
    rect("drive-bay", -0.07, 0.395, 0.14, 0.03, 3.4, 0.4, "front-bezel", "bezel-abs", role="detail"),
    rect("power-button", 0.05, 0.03, 0.012, 0.012, 3.4, 0.7, "front-bezel", "bezel-abs", role="detail"),
]
pl50 = [
    rect("top-sheet", -0.225, -0.1, 0.45, 0.2, 3.1, 0.55, S, "chassis-steel", r=0.004),
    rect("front-bezel", 0.2, -0.1, 0.025, 0.2, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, -0.1, 0.012, 0.2, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.2, -0.085, 0.09, 0.17, 3.4, 0.38, "psu-vent", "chassis-steel"),
    lines("psu-top-slits", [(-0.195 + 0.015 * i, -0.075, -0.195 + 0.015 * i, 0.075) for i in range(6)], 3.6, "psu-vent"),
    rect("cpu-edge", -0.07, 0.092, 0.17, 0.008, 3.5, 0.38, "cpu-grille", "chassis-steel", role="detail"),
]
el100 = [
    rect("foot-front", 0.15, 0, 0.05, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-rear", -0.2, 0, 0.05, 0.012, 3, 0.15, "feet", "rubber-foot"),
    path("case-side", CASE_PATH, 3.1, 0.55, S, "chassis-steel"),
    rect("front-bezel", 0.195, 0.024, 0.03, 0.414, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, 0.012, 0.014, 0.438, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.225, 0.33, 0.03, 0.108, 3.4, 0.38, "psu-vent", "chassis-steel"),
    rect("cpu-grille", -0.07, 0.202, 0.17, 0.13, 3.4, 0.38, "cpu-grille", "chassis-steel"),
    lines("grille-slits", [(-0.04 + 0.04 * i, 0.212, -0.04 + 0.04 * i, 0.322) for i in range(4)], 3.6, "cpu-grille"),
]
fr100 = [
    rect("foot-left", 0.05, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("foot-right", -0.09, 0, 0.04, 0.012, 3, 0.15, "feet", "rubber-foot"),
    rect("front-bezel", -0.1, 0.012, 0.2, 0.438, 3.1, 0.18, "front-bezel", "bezel-abs", r=0.006),
    rect("wall-left", 0.088, 0.012, 0.012, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("wall-right", -0.1, 0.012, 0.012, 0.438, 3.2, 0.55, S, "chassis-steel"),
    rect("intake-mesh", -0.08, 0.05, 0.16, 0.34, 3.3, 0.3, "front-bezel", "bezel-abs", role="detail"),
]
pl100 = [
    rect("top-sheet", -0.225, -0.1, 0.45, 0.2, 3.1, 0.55, S, "chassis-steel", r=0.005),
    rect("front-bezel", 0.195, -0.1, 0.03, 0.2, 3.3, 0.18, "front-bezel", "bezel-abs"),
    rect("rear-panel", -0.225, -0.1, 0.014, 0.2, 3.2, 0.5, "rear-panel", "chassis-steel"),
    rect("psu-vent", -0.2, -0.085, 0.09, 0.17, 3.4, 0.38, "psu-vent", "chassis-steel"),
    rect("cpu-edge", -0.07, 0.088, 0.17, 0.012, 3.5, 0.38, "cpu-grille", "chassis-steel", role="detail"),
]
el200 = [
    path("plinth", "M -0.2 0 L 0.2 0 L 0.2 0.024 L -0.2 0.024 Z", 3, 0.3, "feet", "rubber-foot"),
    path("case", CASE_PATH, 3.1, 0.5, regions=ALL_CASE, mat="chassis-steel"),
]
fr200 = [
    path("plinth", "M -0.08 0 L 0.08 0 L 0.08 0.024 L -0.08 0.024 Z", 3, 0.3, "feet", "rubber-foot"),
    rect("case", -0.1, 0.012, 0.2, 0.438, 3.1, 0.4, None, "chassis-steel"),
]
fr200[1].pop("region")
fr200[1]["regions"] = ["front-bezel", "side-panel"]
pl200 = [
    rect("top", -0.225, -0.1, 0.45, 0.2, 3.1, 0.5, None, "chassis-steel"),
]
pl200[0].pop("region")
pl200[0]["regions"] = ALL_CASE
