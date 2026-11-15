"""Author 22: the bicycle of the cyclist per scale, derived from the 1:50 frame solids with the scale's own tube diameters (pen of the 2D strokes), merged parts and dropped sub-feature details."""
import copy, json, math
import author_07_lib as L

def r(s, rad=None, floor=0.0):
    s = copy.deepcopy(s)
    if s["kind"] == "capsule":
        s["r0"] = s["r1"] = max(rad or s["r0"], floor)
    return s

def floor_loft(s, f):
    s = copy.deepcopy(s)
    for q in s["sections"]:
        q["rx"] = max(q["rx"], f); q["ry"] = max(q["ry"], f)
    return s

def widen_fender(s, centre, gap):
    s = copy.deepcopy(s)
    pts = s["profile"]; n = len(pts) // 2
    cx, cy = centre
    out = pts[:n]
    ro = sum(math.hypot(p[0] - cx, p[1] - cy) for p in out) / n
    inner = []
    for p in pts[n:]:
        a = math.atan2(p[1] - cy, p[0] - cx)
        inner.append([cx + (ro - gap) * math.cos(a), cy + (ro - gap) * math.sin(a)])
    s["profile"] = out + inner
    return s

FENDER = {
 "rear": "M 0.3399 0.5207 C 0.3295 0.5372 0.3203 0.5546 0.3086 0.5702 C 0.297 0.5857 0.284 0.6006 0.2702 0.6143 C 0.2563 0.6279 0.2413 0.6406 0.2255 0.652 C 0.2098 0.6634 0.1929 0.6737 0.1756 0.6826 C 0.1583 0.6915 0.1402 0.6991 0.1217 0.7053 C 0.1033 0.7114 0.0841 0.7162 0.065 0.7195 C 0.0458 0.7228 0.0262 0.7246 0.0067 0.7249 C -0.0127 0.7253 -0.0324 0.7241 -0.0517 0.7215 C -0.0709 0.7189 -0.0902 0.7148 -0.1089 0.7093 C -0.1275 0.7038 -0.146 0.6968 -0.1636 0.6885 C -0.1812 0.6803 -0.1983 0.6705 -0.2145 0.6597 C -0.2306 0.6489 -0.2461 0.6367 -0.2605 0.6235 C -0.2748 0.6104 -0.2882 0.596 -0.3004 0.5808 C -0.3126 0.5656 -0.3224 0.5486 -0.3334 0.5325",
 "front": "M 1.3834 0.5325 C 1.3724 0.5486 1.3626 0.5656 1.3504 0.5808 C 1.3382 0.596 1.3248 0.6104 1.3105 0.6235 C 1.2961 0.6367 1.2806 0.6489 1.2645 0.6597 C 1.2483 0.6705 1.2312 0.6803 1.2136 0.6885 C 1.196 0.6968 1.1775 0.7038 1.1589 0.7093 C 1.1402 0.7148 1.1209 0.7189 1.1017 0.7215 C 1.0824 0.7241 1.0627 0.7253 1.0433 0.7249 C 1.0238 0.7246 1.0042 0.7228 0.985 0.7195 C 0.9659 0.7162 0.9467 0.7114 0.9283 0.7053 C 0.9098 0.6991 0.8917 0.6915 0.8744 0.6826 C 0.8571 0.6737 0.8402 0.6634 0.8245 0.652 C 0.8087 0.6406 0.7937 0.6279 0.7798 0.6143 C 0.766 0.6006 0.753 0.5857 0.7414 0.5702 C 0.7297 0.5546 0.7205 0.5372 0.7101 0.5207",
}

TUBE = {
 100: dict(seat=.017, top=.016, down=.018, head=.022, chain=.011, seatstay=.010, fork=.012, stem=.014, post=.011, bar=.014, grip=.018, cross=.012, bb=.02, crank=.0125, ring_n=24),
 200: dict(seat=.018, top=.018, down=.018, head=.018, chain=.018, seatstay=.018, fork=.018, stem=.018, post=.018, bar=.018, grip=.0175, cross=.0155, bb=.02, crank=.0155, ring_n=20),
 500: dict(seat=.0328, top=.0328, down=.0328, head=.0328, chain=.0328, seatstay=.0328, fork=.0328, stem=.0328, post=.0328, bar=.05, grip=.05, cross=.0328, bb=.0328, crank=.0328, ring_n=16),
}

def bike(scale, b50):
    t = TUBE[scale]
    by = {s["id"]: s for s in b50 if s["bone"] in ("frame", "rear-wheel", "front-wheel", "crank-r", "crank-l", "pedal-r", "pedal-l")}
    f = {100: .0078, 200: .0155, 500: .0327}[scale]
    out = []
    def add(i, rad, **kw):
        s = r(by[i], rad, f); s.update(kw); out.append(s)
    add("tube-seat", t["seat"]); add("tube-top", t["top"]); add("tube-down", t["down"]); add("tube-head", t["head"])
    for sd in "lr":
        add(f"tube-chainstay-{sd}", t["chain"]); add(f"tube-seatstay-{sd}", t["seatstay"]); add(f"fork-{sd}", t["fork"])
        add(f"handlebar-{sd}", t["bar"]); add(f"grip-{sd}", t["grip"])
    add("stem", t["stem"])
    if scale != 500:
        add("seatpost", t["post"]); add("handlebar-cross", t["cross"])
    add("bottom-bracket", t["bb"])
    sad = floor_loft(by["saddle"], f)
    out.append(sad)
    if scale == 100:
        out.append(copy.deepcopy(by["headlamp"]))
        out.append(fender_from_path(by["rear-fender"], FENDER["rear"], (0.0, 0.34), 0.016))
        out.append(fender_from_path(by["front-fender"], FENDER["front"], (1.05, 0.34), 0.016))
    for w in ("rear", "front"):
        ty = copy.deepcopy(by[f"{w}-tyre"]); n = t["ring_n"]
        th = 2 * math.pi / n
        rr = {100: .02, 200: .018, 500: .0386}[scale]
        R = {100: .32, 200: .34, 500: .34}[scale] 
        half = R * math.sin(th / 2)
        ty["p0"] = [-half, R * math.cos(th / 2), 0]; ty["p1"] = [half, R * math.cos(th / 2), 0]
        ty["repeat"]["count"] = n; ty["repeat"]["rotationRad"] = round(th, 5)
        ty["r0"] = ty["r1"] = rr
        ty["lateralScale"] = max(1.0, 0.0275 / rr)
        out.append(ty)
        if scale < 500:
            out.append(copy.deepcopy(by[f"{w}-hub"]))
    for sd in "lr":
        c = copy.deepcopy(by[f"crank-arm-{sd}"]); c["r0"] = max(c["r0"], t["crank"]); c["r1"] = max(c["r1"] * 1.0, f); out.append(c) if scale < 500 else None
        p = copy.deepcopy(by[f"pedal-{sd}"])
        if scale == 100:
            out.append(p)
        elif scale == 200:
            p["size"][1] = 0.0306; out.append(p)
    if scale == 500:
        pass
    for s in out:
        s.pop("sections", None) if s["kind"] != "loft" else None
    return out


def floor_any(s, f):
    if s["kind"] == "loft":
        for q in s["sections"]:
            q["rx"] = max(q["rx"], f); q["ry"] = max(q["ry"], f)
    elif s["kind"] == "capsule":
        s["r0"] = max(s["r0"], f); s["r1"] = max(s["r1"], f)
    return s


def fender_from_path(template, d, centre, gap):
    import re
    nums = [float(x) for x in re.findall(r"-?\d+\.?\d*", d)]
    pts = [(nums[i], nums[i + 1]) for i in range(0, len(nums), 2)]
    ends = [pts[0]] + [pts[i] for i in range(3, len(pts), 3)]
    cx, cy = centre
    inner = []
    for x, y in ends:
        rr = math.hypot(x - cx, y - cy)
        k = (rr - gap) / rr
        inner.append([round(cx + (x - cx) * k, 5), round(cy + (y - cy) * k, 5)])
    s = copy.deepcopy(template)
    s["profile"] = [[x, y] for x, y in ends] + inner[::-1]
    return s
