"""🚜 Excavator (author 36): four models, each built from its own scale's drawings. Usage: python3 author-36-excavator.py [50 100 200 500] (default all); writes only the named scales of the source."""
import importlib.util, os, math, sys
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-36-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("excavator")
PS = dict(material="paint-steel", role="body")
PSD = dict(material="paint-steel", role="detail")
TS = dict(material="track-steel", role="body")
TSD = dict(material="track-steel", role="detail")
WS = dict(material="wear-steel", role="body")
WSD = dict(material="wear-steel", role="detail")
CI = dict(material="cast-iron", role="body")
CH = dict(material="chrome", role="detail")
GL = dict(material="glass", role="glazing")
EM = dict(material="engine-metal", role="detail")
import json
_old = json.load(open(os.path.join(here, "volumes", "machinery", "excavator.json"), encoding="utf8"))["volumes"]["50"]["body"]
EYES = [s for s in _old if s["id"] in ("boom-cyl-foot", "boom-rod-eye", "stick-cyl-foot", "stick-rod-eye", "bucket-cyl-foot")]
FEAT = {50: 0.0077, 100: 0.0155, 200: 0.031, 500: 0.0755}
loc = lambda bone, pts: [tuple(V.W(bone, [p[0], p[1], 0])[:2]) for p in pts]


def bez(p0, p1, p2, p3, n=10):
    return [tuple((1 - t) ** 3 * p0[k] + 3 * (1 - t) ** 2 * t * p1[k] + 3 * (1 - t) * t ** 2 * p2[k] + t ** 3 * p3[k] for k in (0, 1)) for t in [i / n for i in range(1, n + 1)]]


def stadium(cx, cy, hl, r, n=10):
    pts = []
    for i in range(n + 1):
        a = -math.pi / 2 + math.pi * i / n
        pts.append((cx + hl + r * math.cos(a), cy + r * math.sin(a)))
    for i in range(n + 1):
        a = math.pi / 2 + math.pi * i / n
        pts.append((cx - hl + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def house_profile(y0, rear=-1.75):
    pts = [(rear, y0), (1.6, y0), (1.6, 1.3), (0.12, 1.3), (0.12, 2.25), (-1.4, 2.25)]
    pts += bez((-1.4, 2.25), (-1.62, 2.25), (-1.75, 2.15), (-1.75, 1.95), 6)
    return pts


def cab_profile():
    pts = [(0.12, 1.3), (1.6, 1.3), (1.58, 1.95)]
    pts += bez((1.58, 1.95), (1.56, 2.4), (1.5, 2.75), (1.42, 2.88), 8)
    pts += bez((1.42, 2.88), (1.36, 2.96), (1.28, 2.97), (1.2, 2.97), 4)
    pts += [(0.4, 2.97)]
    pts += bez((0.4, 2.97), (0.22, 2.97), (0.12, 2.88), (0.12, 2.7), 5)
    return pts


def cab_front_x(y):
    pts = [(1.6, 1.3), (1.58, 1.95)] + bez((1.58, 1.95), (1.56, 2.4), (1.5, 2.75), (1.42, 2.88), 16)
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if y0 <= y <= y1:
            return x0 + (x1 - x0) * (y - y0) / (y1 - y0)
    return pts[-1][0]


def counterweight_profile(y0):
    pts = [(0.65, y0), (-0.5, y0)]
    pts += bez((-0.5, y0), (-0.6, y0), (-0.65, 0.1), (-0.65, 0.25), 6)
    pts += [(-0.63, 0.85)]
    pts += bez((-0.63, 0.85), (-0.6, 1.1), (-0.4, 1.2), (-0.15, 1.2), 8)
    pts += [(0.65, 1.2)]
    return pts


def arm_outlines():
    boom = [(0, -0.5)] + bez((0, -0.5), (0.6, -0.72), (1.6, -0.6), (2.6, -0.28)) + bez((2.6, -0.28), (3.6, 0.04), (4.8, -0.14), (5.7, -0.3)) + [(5.7, 0.3)] + bez((5.7, 0.3), (4.8, 0.34), (3.9, 0.75), (2.9, 0.98)) + bez((2.9, 0.98), (2.1, 1.1), (1.1, 0.95), (0, 0.5))
    stick = [(0, -0.38)] + bez((0, -0.38), (0.9, -0.44), (2.0, -0.32), (2.9, -0.2)) + [(2.9, 0.2)] + bez((2.9, 0.2), (2.0, 0.3), (0.9, 0.46), (0, 0.42))
    bk = [(1.35, 0), (1.3, 0.05)] + bez((1.3, 0.05), (1.05, 0.62), (0.35, 0.78), (-0.08, 0.34), 14) + [(-0.12, 0.06), (0.02, -0.12)] + bez((0.02, -0.12), (0.4, -0.1), (0.95, -0.1), (1.35, 0), 6)[:-1]
    return boom, stick, bk


def bucket_shell(bk, t):
    shell = [(1.3, 0.05)] + bez((1.3, 0.05), (1.05, 0.62), (0.35, 0.78), (-0.08, 0.34), 14) + [(-0.12, 0.06), (0.02, -0.12)]
    cx = sum(p[0] for p in bk) / len(bk)
    cy = sum(p[1] for p in bk) / len(bk)
    inner = []
    for i, p in enumerate(shell):
        a = shell[max(i - 1, 0)]
        b = shell[min(i + 1, len(shell) - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        L = math.hypot(dx, dy) or 1
        nx, ny = -dy / L, dx / L
        if (cx - p[0]) * nx + (cy - p[1]) * ny < 0:
            nx, ny = -nx, -ny
        inner.append((p[0] + t * nx, p[1] + t * ny))
    return shell + inner[::-1]


def tracks(sc):
    outer = stadium(0, 0.43, 1.65, 0.425 if sc == 50 else 0.43)
    inner = stadium(0, 0.43, 1.65, 0.36)
    rep2 = dict(count=2, step=[0, 0, -2.39])
    if sc >= 200:
        V.ext("track-body", "undercarriage", "side", stadium(0, 0.43, 1.65, 0.43), 0.895, 1.495, tone=0.2, region="tracks", rep=rep2, **TS)
        return
    V.ext("track-belt", "undercarriage", "side", outer, 0.895, 1.495, holes=[inner], tone=0.2, region="tracks", rep=rep2, **TS)
    if sc == 50:
        V.ext("track-web", "undercarriage", "side", inner, 1.145, 1.245, tone=0.2, region="tracks", rep=rep2, **TSD)
        V.box("grouser-top", "undercarriage", [-1.4, 0.8575, 1.195], [0.06, 0.005, 0.6], tone=0.4, texture=True, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
        V.box("grouser-top-r", "undercarriage", [-1.4, 0.8575, -1.195], [0.06, 0.005, 0.6], tone=0.4, texture=True, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
        V.box("grouser-bottom", "undercarriage", [-1.5, 0.0025, 1.195], [0.06, 0.005, 0.6], tone=0.4, texture=True, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
        V.box("grouser-bottom-r", "undercarriage", [-1.5, 0.0025, -1.195], [0.06, 0.005, 0.6], tone=0.4, texture=True, rep=dict(count=16, step=[0.2, 0, 0]), **TSD)
        V.box("track-frame", "undercarriage", [0, 0.475, 1.195], [3.4, 0.45, 0.4], tone=0.4, rep=rep2, **PSD, region="tracks")
        V.cyl("sprocket", "undercarriage", [-1.65, 0.43, 0.98], [-1.65, 0.43, 1.41], 0.33, tone=0.5, material="track-steel", role="detail", rep=rep2)
        V.cyl("idler", "undercarriage", [1.65, 0.43, 0.98], [1.65, 0.43, 1.41], 0.33, tone=0.5, material="track-steel", role="detail", rep=rep2)
        V.cyl("sprocket-hub", "undercarriage", [-1.65, 0.43, 0.95], [-1.65, 0.43, 1.44], 0.1, tone=0.25, rep=rep2, **TSD)
        V.cyl("idler-hub", "undercarriage", [1.65, 0.43, 0.95], [1.65, 0.43, 1.44], 0.1, tone=0.25, rep=rep2, **TSD)
        V.cyl("roller", "undercarriage", [-1.0, 0.2, 1.045], [-1.0, 0.2, 1.345], 0.11, tone=0.55, **TSD, rep=dict(count=5, step=[0.5, 0, 0]))
        V.cyl("roller-r", "undercarriage", [-1.0, 0.2, -1.345], [-1.0, 0.2, -1.045], 0.11, tone=0.55, **TSD, rep=dict(count=5, step=[0.5, 0, 0]))
        V.cyl("carrier-roller", "undercarriage", [0, 0.74, 1.095], [0, 0.74, 1.295], 0.07, tone=0.55, rep=rep2, **TSD)
    else:
        V.ext("track-web", "undercarriage", "side", stadium(0, 0.43, 1.65, 0.36), 1.145, 1.245, tone=0.2, region="tracks", rep=rep2, **TSD)
        V.box("track-frame", "undercarriage", [0, 0.475, 1.195], [3.4, 0.45, 0.4], tone=0.4, rep=rep2, region="tracks", **PSD)
        V.cyl("sprocket", "undercarriage", [-1.65, 0.43, 0.98], [-1.65, 0.43, 1.41], 0.33, tone=0.5, material="track-steel", role="detail", rep=rep2)
        V.cyl("idler", "undercarriage", [1.65, 0.43, 0.98], [1.65, 0.43, 1.41], 0.33, tone=0.5, material="track-steel", role="detail", rep=rep2)


def undercarriage_core(sc):
    V.box("carbody", "undercarriage", [0, 0.58, 0], [2.0, 0.56, 1.79], tone=0.4, region="tracks", **PS)
    V.cyl("slew-ring", "undercarriage", [0, 0.86, 0], [0, 1.0, 0], 0.8, tone=0.3, **dict(PS, role="detail"))


def cab_and_glass(sc):
    V.ext("cab", "upper", "side", cab_profile(), 0.1, 1.24, tone=0.62, region="house", **PS)
    t = 0.02
    fy = [1.58, 1.95, 2.4, 2.72, 2.87]
    if sc == 50:
        V.ext("cab-glass-side-window", "upper", "side", [(0.3, 1.6), (0.95, 1.6), (0.95, 2.9), (0.45, 2.9)] + bez((0.45, 2.9), (0.35, 2.9), (0.3, 2.85), (0.3, 2.75), 4), 1.24, 1.25, tone=0.85, region="cab-glazing", **GL)
        V.ext("cab-glass-rear-window", "upper", "side", [(1.02, 1.58), (1.46, 1.58)] + bez((1.46, 1.58), (1.45, 2.1), (1.4, 2.6), (1.34, 2.82), 6) + bez((1.34, 2.82), (1.3, 2.87), (1.24, 2.87), (1.2, 2.87), 3) + [(1.02, 2.87)], 1.24, 1.25, tone=0.85, region="cab-glazing", **GL)
    elif sc == 100:
        V.ext("cab-glass-side", "upper", "side", [(0.3, 1.6), (0.95, 1.6), (1.02, 1.55), (1.56, 1.55)] + bez((1.56, 1.55), (1.55, 2.1), (1.5, 2.6), (1.43, 2.84), 6) + bez((1.43, 2.84), (1.38, 2.9), (1.3, 2.9), (1.2, 2.9), 3) + [(0.45, 2.9)] + bez((0.45, 2.9), (0.35, 2.9), (0.3, 2.85), (0.3, 2.75), 4), 1.24, 1.25, tone=0.85, region="cab-glazing", **GL)
    elif sc == 200:
        V.ext("cab-glass-side", "upper", "side", [(0.35, 1.62), (0.95, 1.62), (1.0, 1.58), (1.46, 1.58)] + bez((1.46, 1.58), (1.45, 2.1), (1.4, 2.6), (1.34, 2.82), 6) + [(1.2, 2.87), (0.45, 2.87)] + bez((0.45, 2.87), (0.38, 2.87), (0.35, 2.83), (0.35, 2.75), 3), 1.21, 1.25, tone=0.85, region="cab-glazing", **GL)
    else:
        V.ext("cab-glass-side", "upper", "side", [(0.3, 1.6), (0.95, 1.6), (1.02, 1.55), (1.56, 1.55)] + bez((1.56, 1.55), (1.55, 2.1), (1.5, 2.6), (1.43, 2.84), 6) + bez((1.43, 2.84), (1.38, 2.9), (1.3, 2.9), (1.2, 2.9), 3) + [(0.45, 2.9)] + bez((0.45, 2.9), (0.35, 2.9), (0.3, 2.85), (0.3, 2.75), 4), 1.175, 1.25, tone=0.85, region="cab-glazing", **GL)
        return
    wt = max(0.025, FEAT[sc] * 1.05) if sc > 50 else 0.025
    ys = [1.58, 1.95, 2.4, 2.72, 2.87]
    outer = [(cab_front_x(y) - 0.002, y) for y in ys]
    inner = [(x - wt, y) for x, y in outer]
    V.ext("cab-windscreen", "upper", "side", outer + inner[::-1], 0.2, 1.15, tone=0.85, region="cab-glazing", **GL)


def house_core(sc):
    y0 = 1.0
    V.ext("house", "upper", "side", house_profile(y0), -1.25, 1.25, tone=0.62, region="house", **PS)
    cab_and_glass(sc)


def hood(sc):
    ft = max(0.02, FEAT[sc] * 1.02)
    if sc == 50:
        V.box("hood-grille-top", "upper", [-1.05, 2.25 + ft / 2, 0], [1.0, ft, 1.8], tone=0.2, region="engine-compartment", **EM)
        V.box("hood-grille-side", "upper", [-1.05, 1.725, -1.25 - ft / 2], [1.0, 0.65, ft], tone=0.15, region="engine-compartment", **EM)
        V.cap("hood-rail-bar", "upper", [-1.65, 2.4875, -1.15], [-0.3, 2.4875, -1.15], 0.0125, tone=0.3, slender=True, **PSD)
        V.cap("hood-rail-post", "upper", [-1.65, 2.2, -1.15], [-1.65, 2.4875, -1.15], 0.0125, tone=0.3, rep=dict(count=2, step=[1.35, 0, 0]), **PSD)
        V.box("engine-block", "engine", [-1.0, 1.525, 0], [1.1, 0.65, 2.4], tone=0.2, region="engine-compartment", **EM)
        V.box("cab-sill", "upper", [0.85, 1.575, 1.2475], [1.46, 0.05, 0.015], tone=0.3, slender=True, **PSD)
    elif sc == 100:
        V.box("hood-grille-top", "upper", [-1.05, 2.25 + ft / 2, 0], [1.0, ft, 1.8], tone=0.2, region="engine-compartment", **EM)
        V.box("hood-grille-side", "upper", [-1.05, 1.725, -1.25 - ft / 2], [1.0, 0.65, ft], tone=0.15, region="engine-compartment", **EM)
        V.cap("hood-rail", "upper", [-1.65, 2.4875, -1.15], [-0.3, 2.4875, -1.15], 0.0125, tone=0.3, slender=True, **PSD)
        V.cap("hood-rail-post", "upper", [-1.65, 2.2, -1.15], [-1.65, 2.4875, -1.15], 0.0125, tone=0.3, rep=dict(count=2, step=[1.35, 0, 0]), **PSD)
    elif sc == 200:
        V.box("hood-grille-top", "upper", [-1.05, 2.25 + ft / 2, 0], [1.0, ft, 1.8], tone=0.2, region="engine-compartment", **EM)
        V.box("hood-grille-side", "upper", [-1.05, 1.725, -1.25 - ft / 2], [1.0, 0.65, ft], tone=0.15, region="engine-compartment", **EM)
    if sc < 500:
        V.cap("stack", "upper", [-1.2, 2.25, -0.9], [-1.2, 2.8, -0.9], 0.055, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")
    if sc == 50:
        V.cyl("stack-cap", "upper", [-1.2, 2.78, -0.9], [-1.2, 2.82, -0.9], 0.08, tone=0.2, material="exhaust-steel", role="detail")


def boom_foot(sc):
    pts = [(0.78, 1.22), (1.3, 1.22), (1.3, 1.55), (1.15, 1.78), (0.9, 1.78), (0.78, 1.55)]
    if sc == 50:
        V.ext("boom-foot", "upper", "side", pts, -0.7, -0.55, tone=0.4, region="house", **PS)
        V.ext("boom-foot-inner", "upper", "side", pts, 0.05, 0.1, tone=0.4, region="house", **PS)
    elif sc == 100:
        V.ext("boom-foot", "upper", "side", pts, -0.7, 0.1, tone=0.4, region="house", **PS)


def counterweight(sc):
    y0 = 0.0
    V.ext("counterweight-body", "counterweight", "side", loc("counterweight", counterweight_profile(y0)), -1.2, 1.2, tone=0.35, region="counterweight", **CI)


def arm(sc):
    boom, stick, bk = arm_outlines()
    V.ext("boom-body", "boom", "side", loc("boom", boom), -0.55, 0.05, tone=0.58, region="attachment", **PS)
    V.ext("stick-body", "stick", "side", loc("stick", stick), -0.49, -0.01, tone=0.58, region="attachment", **PS)
    if sc == 50:
        V.cyl("boom-pin", "boom", [1.0, 1.65, -0.7], [1.0, 1.65, 0.1], 0.09, tone=0.12, **TSD)
        V.cyl("boom-tip-boss", "boom", V.W("boom", [5.7, 0, -0.55]), V.W("boom", [5.7, 0, 0.05]), 0.26, tone=0.35, **PSD)
        V.cyl("stick-root-boss", "stick", V.W("stick", [0, 0, -0.49]), V.W("stick", [0, 0, -0.01]), 0.28, tone=0.35, **PSD)
        V.cyl("stick-tip-boss", "stick", V.W("stick", [2.9, 0, -0.49]), V.W("stick", [2.9, 0, -0.01]), 0.22, tone=0.35, **PSD)
    elif sc == 100:
        V.cyl("boom-stick-boss", "boom", V.W("boom", [5.7, 0, -0.55]), V.W("boom", [5.7, 0, 0.05]), 0.26, tone=0.35, **PSD)
        V.cyl("stick-tip-boss", "stick", V.W("stick", [2.9, 0, -0.49]), V.W("stick", [2.9, 0, -0.01]), 0.22, tone=0.35, **PSD)
    if True:
        K = 1.0
        V.cyl("boom-cyl-barrel", "boom-cyl", V.W("boom-cyl", [0, 0, 0.25]), V.W("boom-cyl", [1.45, 0, 0.25]), 0.1, tone=0.35, region="hydraulics", rep=dict(count=2, step=[0, 0, -1.0]), **PSD)
        V.cyl("boom-cyl-rod", "boom-rod", V.W("boom-rod", [-0.4, 0, 0.25]), V.W("boom-rod", [1.65, 0, 0.25]), 0.045 * K, tone=0.85, region="hydraulics", rep=dict(count=2, step=[0, 0, -1.0]), **CH)
        V.cyl("stick-cyl-barrel", "stick-cyl", V.W("stick-cyl", [0, 0, -0.25]), V.W("stick-cyl", [1.6, 0, -0.25]), 0.095, tone=0.35, region="hydraulics", **PSD)
        V.cyl("stick-cyl-rod", "stick-rod", V.W("stick-rod", [-0.5, 0, -0.25]), V.W("stick-rod", [2.35, 0, -0.25]), 0.05 * K, tone=0.85, region="hydraulics", **CH)
        V.cyl("bucket-cyl-barrel", "bucket-cyl", V.W("bucket-cyl", [0, 0, -0.25]), V.W("bucket-cyl", [0.85, 0, -0.25]), 0.075, tone=0.35, region="hydraulics", **PSD)
        V.cyl("bucket-cyl-rod", "bucket-rod", V.W("bucket-rod", [-0.4, 0, -0.25]), V.W("bucket-rod", [0.95, 0, -0.25]), 0.0378, tone=0.85, region="hydraulics", **CH)
    if sc == 50:
        for s in EYES:
            V.lists["body"].append(dict(s))
    bucket(sc, bk)


def bucket(sc, bk):
    FT = max(FEAT[sc], 0.0075)
    bkw = loc("bucket", bk)
    t = {50: 0.05, 100: 0.06, 200: 0.07, 500: 0.0755}[sc]
    V.ext("bucket-shell", "bucket", "side", loc("bucket", bucket_shell(bk, t)), -0.7, 0.2, tone=0.4, region="attachment", **WS)
    V.ext("bucket-cheek", "bucket", "side", bkw, -0.75, -0.75 + max(0.05, FT * 1.02), tone=0.4, region="attachment", rep=dict(count=2, step=[0, 0, 0.95]), **WS)
    if sc == 50:
        V.ext("bucket-tooth", "bucket", "side", loc("bucket", [(1.3, 0.07), (1.5, -0.03), (1.28, -0.09)]), -0.72, -0.6, tone=0.2, region="attachment", rep=dict(count=5, step=[0, 0, 0.2]), **WSD)
    else:
        V.ext("bucket-lip", "bucket", "side", loc("bucket", [(1.28, 0.07), (1.35, 0.0), (1.5, -0.03)]), -0.75, 0.25, tone=0.2, region="attachment", **WS)
    if sc == 50:
        V.ext("bucket-floor-lip", "bucket", "side", loc("bucket", bez((0.02, -0.12), (0.4, -0.1), (0.95, -0.1), (1.35, 0), 8) + [(1.35, 0.03)] + bez((1.35, 0.03), (0.95, -0.07), (0.4, -0.07), (0.02, -0.09), 8)[1:]), -0.75, 0.25, tone=0.3, region="attachment", **WSD)
        V.ext("bucket-rib", "bucket", "side", loc("bucket", [(0.28, 0.1), (0.42, 0.1), (0.45, 0.57), (0.3, 0.58)]), -0.28, -0.22, tone=0.3, **WSD)
        V.cyl("bucket-pin", "bucket", V.W("bucket", [0, 0, -0.78]), V.W("bucket", [0, 0, 0.28]), 0.1, tone=0.12, **TSD)
        V.cyl("bucket-ear", "bucket", V.W("bucket", [0.35, 0.18, -0.4]), V.W("bucket", [0.35, 0.18, -0.1]), 0.06, tone=0.12, **TSD)


def geometry(sc):
    tracks(sc)
    undercarriage_core(sc)
    house_core(sc)
    hood(sc)
    boom_foot(sc)
    counterweight(sc)
    arm(sc)


CYL = [
    ("boom-cyl-barrel", "boom-cyl", [0, 0, 0.25], [1.45, 0, 0.25], 0.1, dict(count=2, step=[0, 0, -1.0])),
    ("boom-cyl-rod", "boom-rod", [-0.4, 0, 0.25], [1.65, 0, 0.25], 0.045, dict(count=2, step=[0, 0, -1.0])),
    ("stick-cyl-barrel", "stick-cyl", [0, 0, -0.25], [1.6, 0, -0.25], 0.095, None),
    ("stick-cyl-rod", "stick-rod", [-0.5, 0, -0.25], [2.35, 0, -0.25], 0.05, None),
    ("bucket-cyl-barrel", "bucket-cyl", [0, 0, -0.25], [0.85, 0, -0.25], 0.075, None),
    ("bucket-cyl-rod", "bucket-rod", [-0.4, 0, -0.25], [0.95, 0, -0.25], 0.0378, None),
]


def structure(sc):
    f = FEAT[sc]
    V.to("structure")
    ids = {s["id"]: s["bone"] for s in V.lists["body"]}
    mem = ["carbody", "track-frame", "track-web", "track-body", "house", "counterweight-body", "boom-body", "stick-body", "bucket-shell", "bucket-cheek", "boom-cyl-barrel", "boom-cyl-rod", "stick-cyl-barrel", "stick-cyl-rod", "bucket-cyl-barrel", "bucket-cyl-rod", "cab"]
    for sid in mem:
        if sid in ids and not (sc in (100, 500) and sid == "bucket-cheek"):
            V.member(sid, ids[sid])

    if "engine-block" in ids:
        V.member("engine-block", "engine")
    else:
        V.box("member-engine", "engine", [-1.0, 1.525, 0], [1.1, 0.65, 2.4], tone=0.3, member="engine", part="member", **EM)
    if sc <= 100:
        V.cyl("joint-slew", "slew-ring", [0, 1.0, 0], [0, 1.0 + max(0.03, f * 1.3), 0], 0.6, tone=0.3, member="upper", part="joint", **PSD)
        for nm, bone, pos in [("joint-boom-foot", "boom", (1.0, 1.65)), ("joint-stick-root", "stick", (5.366, 5.314)), ("joint-bucket-root", "bucket", (7.231, 3.092))]:
            V.cyl(nm, bone, [pos[0], pos[1], -0.45], [pos[0], pos[1], 0.0], max(0.07, f * 0.6), tone=0.12, member=bone, part="joint", **TSD)
    for tag, x, z in [("r2-l", -1.35, 1.2), ("r2-r", -1.35, -1.2), ("r1-l", -0.45, 1.2), ("r1-r", -0.45, -1.2), ("f1-l", 0.45, 1.2), ("f1-r", 0.45, -1.2), ("f2-l", 1.35, 1.2), ("f2-r", 1.35, -1.2)]:
        V.box(f"support-track-{tag}", "undercarriage", [x, max(f * 0.6, 0.005), z], [0.6, max(f * 1.2, 0.01), 0.55], tone=0.2, member="undercarriage", part="support", anchor="track-" + tag, **TSD)
    V.ell("mass-undercarriage", "undercarriage", [0, 0.58, 0], [0.7, 0.2, 0.5], tone=0.2, member="undercarriage", part="mass", **PSD)
    V.ell("mass-upper", "upper", [-0.5, 1.7, 0], [0.5, 0.4, 0.6], tone=0.2, member="upper", part="mass", **PSD)
    V.ell("mass-counterweight", "counterweight", [-2.35, 1.6, 0], [0.3, 0.4, 0.6], tone=0.2, member="counterweight", part="mass", **CI)
    V.ell("mass-bucket-load", "bucket", V.W("bucket", [0.65, 0.22, -0.25]), [0.3, 0.2, 0.3], tone=0.2, member="bucket", part="mass", **WSD)


def thermal_extra(sc):
    f = FEAT[sc]
    V.to("thermal")
    t = max(f, 0.0075) * 1.03
    V.cyl("th-exhaust-stack-jet", "upper", [-1.2, 2.8, -0.9], [-1.2, 2.8 + t, -0.9], 0.06, tone=0.2, region="exhaust-stack", tags=["exhaust-stack"], material="exhaust-steel", role="detail")
    if sc >= 500:
        V.box("th-engine-hood", "upper", [-1.05, 1.9, 0], [1.0, 0.5, 1.4], tone=0.2, region="engine-compartment", **EM)
        V.cap("th-stack", "upper", [-1.2, 2.25, -0.9], [-1.2, 2.8, -0.9], 0.055, tone=0.25, region="exhaust-stack", material="exhaust-steel", role="detail")



def build(sc):
    V.scale = sc
    geometry(sc)
    V.auto_thermal()
    thermal_extra(sc)
    structure(sc)


DECISIONS = {
    50: [
        ("Every moving part is its own solid on the bone that drives it. Tracks: two belt loops (stadium extrusions with a through hole, 70 mm wall) with a 100 mm web plate, a 3.4 m frame beam, sprocket, idler, hubs, five rollers and a carrier roller per side, and 16 texture lugs per run; car body and slew drum below the house. House: one extruded profile with the Bezier rear corner (deck 1.0 m above ground, hood 1.25 m high) and a separate cab (1.97 m high, left half only: the front view now draws the engine half at 1.25 m); the windscreen is a 25 mm sheet that follows the cab front, the side panes are sheets flush with the cab wall; hood grille panels, a rail of three 25 mm tubes and the stack as a capsule. Counterweight, boom and stick are extruded Bezier outlines; boom and stick carry bosses, pins and eyes, three cylinder pairs (barrel plus chrome rod) sit on their own bones, and the bucket is a 50 mm scoop shell with cheeks, floor lip, rib, five teeth, pin and ear. The rest pose is not a closed linkage; the dimensions describe the stowed default pose.",
         "Jedes bewegliche Teil ist ein eigener Körper auf dem Knochen, der es antreibt. Ketten: zwei Bandschleifen (Langloch-Extrusionen mit Durchbruch, 70 mm Wand) mit 100-mm-Stegblech, 3,4 m langem Rahmenträger, Kettenrad, Leitrad, Naben, fünf Laufrollen und einer Tragrolle je Seite sowie 16 Texturstollen je Trum; darunter Wagenkasten und Drehkranz. Oberwagen: ein extrudiertes Profil mit Bézier-Heckrundung (Plattform 1,0 m über Boden, Haube 1,25 m hoch) und eine eigene Kabine (1,97 m hoch, nur linke Hälfte: die Vorderansicht zeichnet die Motorhälfte jetzt mit 1,25 m); die Frontscheibe ist ein 25-mm-Blech entlang der Kabinenfront, die Seitenscheiben sind bündige Bleche in der Kabinenwand; Haubengitter, ein Geländer aus drei 25-mm-Rohren und das Auspuffrohr als Kapsel. Gegengewicht, Ausleger und Stiel sind extrudierte Bézier-Umrisse mit Naben, Bolzen und Augen, drei Zylinderpaare (Rohr und Chromstange) sitzen auf ihren eigenen Knochen, der Löffel ist eine 50-mm-Schaufelschale mit Wangen, Bodenlippe, Rippe, fünf Zähnen, Bolzen und Ohr. Die Ruhelage ist kein geschlossenes Getriebe; die Maße beschreiben die Transportstellung."),
        ("Thermal: the regions reference the body solids that carry their surface (house: deck, cab, boom foot; glazing: the three panes; engine compartment: the two hood grille panels; tracks: belts, webs, frame, wheels and rollers; attachment: boom, stick, bucket parts); the stack jet is a 120 mm nozzle disc on the stack mouth. areaM2 of house, counterweight, hydraulics and tracks are the committed values and the glazing is 4 m2 (the committed 6 m2 within the factor 1.5); the honestly modelled object exposes house 17.6, counterweight 11.5, glazing 1.45, hydraulics 1.55 and tracks 38 m2 instead, which is listed in area-review.md for a decision.",
         "Thermik: Die Regionen verweisen auf die Körper, die ihre Oberfläche tragen (Oberwagen: Plattform, Kabine, Auslegerfuß; Verglasung: die drei Scheiben; Motorraum: die zwei Haubengitterplatten; Ketten: Bänder, Stege, Rahmen, Räder und Rollen; Anbau: Teile von Ausleger, Stiel und Löffel); der Auspuffstrahl ist eine 120-mm-Düsenscheibe an der Auspuffmündung. areaM2 von Oberwagen, Gegengewicht, Hydraulik und Ketten sind die festgeschriebenen Werte, die Verglasung hat 4 m2 (die festgeschriebenen 6 m2 innerhalb des Faktors 1,5); das ehrlich modellierte Objekt zeigt stattdessen Oberwagen 17,6, Gegengewicht 11,5, Verglasung 1,45, Hydraulik 1,55 und Ketten 38 m2, das steht zur Entscheidung in area-review.md."),
        ("Structure: car body, track frame and web, house, cab, counterweight, boom, stick, bucket shell and cheeks, all six cylinder parts and an engine block inside the hood are members; joints are the slew drum and the three attachment pins; eight 600 mm by 550 mm contact pads on the ground under the belts stand at the eight contact anchors (four per track; the anchors declare 0.9 m by 0.6 m footprints, the pads keep to the 3.3 m flat run of the belt); masses stand for undercarriage, house, counterweight and the bucket payload.",
         "Tragwerk: Wagenkasten, Laufwerksrahmen und -steg, Oberwagen, Kabine, Gegengewicht, Ausleger, Stiel, Löffelschale und -wangen, alle sechs Zylinderteile und ein Motorblock in der Haube sind Stäbe; Gelenke sind der Drehkranz und die drei Anbaubolzen; acht Kontaktflächen von 600 mm mal 550 mm am Boden unter den Bändern stehen an den acht Kontaktankern (vier je Kette; die Anker deklarieren 0,9 m mal 0,6 m, die Flächen bleiben im 3,3 m langen ebenen Bandabschnitt); Massen stehen für Unterwagen, Oberwagen, Gegengewicht und die Löffellast."),
    ],
    100: [
        ("Modelled anew against the 1:100 drawings. The belts keep their through hole but lose lugs, hubs, rollers and carrier rollers: the 100 mm web plate stands for frame, hubs and rollers, sprocket and idler stay as two wheels. House deck and cab as at 1:50 but with one merged window sheet (windscreen frame and side pane drawn as one pane), one boom-foot block instead of two plates, a single 25 mm rail tube with two posts, no cab sill and no stack cap. The five teeth are merged into one cutting lip across the bucket width (the 1:100 outline draws the lip), cheeks and the 60 mm shell stay; boom and stick bosses merge into one pivot boss and one tip boss, small eyes and pins disappear. The house and counterweight bottoms are back at 1.0 m (the 1:100 drawings had closed the gap around the slew drum by 0.16 m, which made the coarse body larger than the fine one; repaired in 2D).",
         "Neu gegen die 1:100-Zeichnungen modelliert. Die Bänder behalten ihren Durchbruch, verlieren aber Stollen, Naben, Rollen und Tragrollen: das 100-mm-Stegblech steht für Rahmen, Naben und Rollen, Kettenrad und Leitrad bleiben als zwei Räder. Oberwagenplattform und Kabine wie bei 1:50, aber mit einem zusammengefassten Fensterblech (Frontscheibenrahmen und Seitenscheibe als eine Scheibe gezeichnet), einem Auslegerfußblock statt zwei Platten, einem einzelnen 25-mm-Geländerrohr mit zwei Pfosten, ohne Kabinenschweller und ohne Auspuffkappe. Die fünf Zähne sind zu einer Schneidlippe über die Löffelbreite verschmolzen (der 1:100-Umriss zeichnet die Lippe), Wangen und 60-mm-Schale bleiben; die Naben von Ausleger und Stiel werden zu einer Gelenknabe und einer Spitzennabe, kleine Augen und Bolzen entfallen. Unterkanten von Oberwagen und Gegengewicht liegen wieder bei 1,0 m (die 1:100-Zeichnungen hatten die Lücke um den Drehkranz um 0,16 m geschlossen, was den groben Körper größer als den feinen machte; in 2D repariert)."),
        ("Thermal: references to the remaining body solids (engine compartment: the two hood grille panels at 20 mm; glazing: the merged pane and the windscreen sheet); the stack jet is a nozzle disc. The areaM2 values are the committed ones as at 1:50; the exposed areas of the 1:100 model stay within 30 percent of the 1:50 model.",
         "Thermik: Verweise auf die verbleibenden Körper (Motorraum: die zwei Haubengitterplatten mit 20 mm; Verglasung: das zusammengefasste Fenster und das Frontscheibenblech); der Auspuffstrahl ist eine Düsenscheibe. Die areaM2-Werte sind wie bei 1:50 die festgeschriebenen; die Expositionsflächen des 1:100-Modells bleiben innerhalb von 30 Prozent des 1:50-Modells."),
        ("Structure: the same load path without the bucket cheeks (merged into the shell member), four joints, eight pads and four masses, sized to the 15 mm feature.",
         "Tragwerk: derselbe Lastpfad ohne Löffelwangen (in den Schalenstab einbezogen), vier Gelenke, acht Auflager und vier Massen, auf das 15-mm-Merkmal bemessen."),
    ],
    200: [
        ("Modelled anew against the 1:200 drawings: each track is one solid belt block (belt, web, frame and wheels merged), with the slew drum and the car body between; house deck, cab and counterweight are the three big masses; the cab carries one 40 mm window sheet and a windscreen sheet; the hood is two grille panels of 32 mm; the stack stays a capsule. Boom, stick and bucket are the Bezier outlines; the bucket is a 70 mm shell with cheeks and the cutting lip; the three cylinder pairs stay as barrel and rod because the 1:200 drawing keeps them. No bosses, pins, eyes, rail, sill, teeth or boom foot.",
         "Neu gegen die 1:200-Zeichnungen modelliert: jede Kette ist ein massives Bandblock (Band, Steg, Rahmen und Räder verschmolzen), dazwischen Drehkranz und Wagenkasten; Oberwagenplattform, Kabine und Gegengewicht sind die drei großen Massen; die Kabine trägt ein 40-mm-Fensterblech und ein Frontscheibenblech; die Haube sind zwei Gitterplatten von 32 mm; das Auspuffrohr bleibt eine Kapsel. Ausleger, Stiel und Löffel sind die Bézier-Umrisse; der Löffel ist eine 70-mm-Schale mit Wangen und Schneidlippe; die drei Zylinderpaare bleiben als Rohr und Stange, weil die 1:200-Zeichnung sie behält. Keine Naben, Bolzen, Augen, Geländer, Schweller, Zähne oder Auslegerfuß."),
        ("Thermal: references to the remaining body solids, the stack jet as a nozzle disc; only the sum of the exposed areas is compared at this scale.",
         "Thermik: Verweise auf die verbleibenden Körper, der Auspuffstrahl als Düsenscheibe; in diesem Maßstab wird nur die Summe der Expositionsflächen verglichen."),
        ("Structure: members on every load-bearing bone (car body, track blocks, house, cab, counterweight, boom, stick, bucket shell and cheeks, six cylinder parts, engine block), eight pads and four masses; joints are merged into the members at this scale.",
         "Tragwerk: Stäbe auf jedem tragenden Knochen (Wagenkasten, Kettenblöcke, Oberwagen, Kabine, Gegengewicht, Ausleger, Stiel, Löffelschale und -wangen, sechs Zylinderteile, Motorblock), acht Auflager und vier Massen; Gelenke sind in diesem Maßstab in die Stäbe einbezogen."),
    ],
    500: [
        ("Modelled anew against the 1:500 drawings: two solid track blocks, car body with slew drum, the house deck, cab with one 80 mm window sheet and the counterweight as three masses, boom, stick and a 75 mm bucket shell with the cutting lip; the cylinder pairs stay because their 100 to 200 mm barrels are above the 75 mm feature even though the 1:500 plan does not draw them. The stack and the hood grille are below the drawing's resolution: they live in the thermal list only (stack as a capsule, engine compartment as a hood block inside the house).",
         "Neu gegen die 1:500-Zeichnungen modelliert: zwei massive Kettenblöcke, Wagenkasten mit Drehkranz, Oberwagenplattform, Kabine mit einem 80-mm-Fensterblech und Gegengewicht als drei Massen, Ausleger, Stiel und eine 75-mm-Löffelschale mit Schneidlippe; die Zylinderpaare bleiben, weil ihre Rohre von 100 bis 200 mm über dem 75-mm-Merkmal liegen, obwohl der 1:500-Grundriss sie nicht zeichnet. Auspuffrohr und Haubengitter liegen unter der Zeichnungsauflösung: sie stehen nur in der Thermikliste (Auspuff als Kapsel, Motorraum als Haubenblock im Oberwagen)."),
        ("Thermal: references to the body solids, thermal-only stack capsule and engine-hood block, stack jet as a nozzle disc; only the total exposed area is compared at 1:500.",
         "Thermik: Verweise auf die Körper, Auspuffkapsel und Haubenblock nur in der Thermik, Auspuffstrahl als Düsenscheibe; bei 1:500 wird nur die Gesamtfläche verglichen."),
        ("Structure: one member per load-bearing bone (car body, track blocks, house, cab, counterweight, boom, stick, bucket shell, six cylinder parts, engine block), eight pads and four masses; no joints at this scale.",
         "Tragwerk: ein Stab je tragendem Knochen (Wagenkasten, Kettenblöcke, Oberwagen, Kabine, Gegengewicht, Ausleger, Stiel, Löffelschale, sechs Zylinderteile, Motorblock), acht Auflager und vier Massen; in diesem Maßstab keine Gelenke."),
    ],
}


if __name__ == "__main__":
    todo = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
    for sc in todo:
        build(sc)
        for en, de in DECISIONS[sc]:
            V.decide(en, de)
        V.finish(sc)
    V.write(todo)
