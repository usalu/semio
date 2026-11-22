"""🥁️ The authoritative generator of the used five-piece drum kit candidate `🔣️.json`: rig, anchors, the four handcrafted scales (2D in three views and the body, thermal and structure solids), poses, clips, thermal and structural models, provenance.

Run: `python3 author-drum-kit.py [--report]` in this folder. The hand-chosen numbers live in `author_kit_layout.py` (rig, layout) and `author_kit_parts.py` (parts per scale).
"""
import json
import math
import sys
import author_kit_layout as L
from author_kit_layout import T
from author_kit_parts import Kit, FEATURE
from author_kit_math import dir_local, to_world, to_local, hull, circle_points, add, sub, mul, unit, norm, cross, comb, dot, frames

SCALES = (50, 100, 200, 500)
PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

MATERIALS = [
    ("pe-case", T("Rotomoulded polyethylene case wall, black, scuffed", "Rotationsgeformte Polyethylen-Kofferwand, schwarz, verkratzt"), 0.92, 0.9, 950, 0.40, 1900, 0.8e9, "iso-10456"),
    ("plywood", T("Phenolic-faced birch plywood deck, worn", "Birkensperrholz mit Phenolbeschichtung, abgenutzt"), 0.9, 0.65, 700, 0.17, 1600, 9e9, "iso-10456"),
    ("steel-powder", T("Powder-coated steel, blue, chipped", "Pulverbeschichteter Stahl, blau, abgeplatzt"), 0.9, 0.7, 7850, 50, 460, 210e9, "iso-10456"),
    ("tpr-wheel", T("Thermoplastic rubber castor tread", "Lenkrollenlauffläche aus thermoplastischem Gummi"), 0.92, 0.9, 1150, 0.2, 1700, None, "iso-10456"),
    ("chrome-steel", T("Chrome-plated steel tube, dulled and pitted", "Verchromtes Stahlrohr, matt und mit Narben"), 0.16, 0.42, 7850, 50, 460, 210e9, "incropera-2018"),
    ("alu-cast", T("Cast aluminium footboard, worn bare at the toe", "Aluminiumguss-Trittplatte, an der Spitze blank gelaufen"), 0.3, 0.45, 2700, 160, 900, 70e9, "iso-10456"),
    ("rubber", T("Black rubber feet, hardened with age", "Schwarze Gummifüße, gealtert und verhärtet"), 0.92, 0.9, 1200, 0.17, 1900, None, "iso-10456"),
    ("felt", T("Pressed wool felt (beater, cymbal felts)", "Gepresster Wollfilz (Schlägel, Beckenfilze)"), 0.9, 0.6, 300, 0.05, 1300, None, "authored-estimate"),
    ("vinyl-seat", T("Black vinyl over foam, cracked", "Schwarzes Kunstleder über Schaum, rissig"), 0.9, 0.9, 120, 0.04, 1500, None, "authored-estimate"),
    ("bronze-b20", T("B20 cymbal bronze, tarnished", "B20-Beckenbronze, angelaufen"), 0.35, 0.6, 8600, 45, 380, 105e9, "incropera-2018"),
    ("birch-ply", T("Birch plywood shell under a faded PVC wrap", "Birkensperrholzkessel unter verblasster PVC-Folie"), 0.9, 0.55, 680, 0.13, 1600, 12e9, "iso-10456"),
    ("carpet", T("Polypropylene needle-felt rug, trodden flat", "Polypropylen-Nadelfilzteppich, platt getreten"), 0.9, 0.7, 250, 0.06, 1400, None, "iso-10456"),
    ("pet-head", T("Coated polyester drumhead film, dented", "Beschichtetes Polyester-Fell, verbeult"), 0.9, 0.35, 1390, 0.15, 1170, 4e9, "rossing-2000"),
]
FILL = {"chrome-steel": 0.0, "alu-cast": 0.45, "steel-powder": 0.25, "tpr-wheel": 0.7, "felt": 1.0, "rubber": 0.8}
TUBE_WALL = 0.0016

REGIONS = [
    ("rug", T("Drum rug", "Schlagzeugteppich"), "carpet", "polymer", 0.006, 2.0, "horizontal-plate-up", 0.6, False),
    ("cases", T("Hard case shells, lids and skirts", "Hartschalen, Deckel und Krempen der Koffer"), "pe-case", "polymer", 0.004, 0.5, "vertical-plate", 0.5, False),
    ("trolley-deck", T("Trolley deck", "Ladefläche des Plattformwagens"), "plywood", "timber", 0.018, 1.0, "horizontal-plate-up", 0.4, False),
    ("trolley-frame", T("Trolley frame, castor forks and push handle", "Wagenrahmen, Lenkrollengabeln und Schiebebügel"), "steel-powder", "metal", 0.003, 0.03, "cylinder", 0.4, False),
    ("trolley-wheels", T("Castor wheels", "Lenkrollen"), "tpr-wheel", "polymer", 0.015, 0.125, "cylinder", 0.2, False),
    ("stand-chrome", T("Stands, throne base, pedals and beater", "Ständer, Hockerbasis, Pedale und Schlägel"), "chrome-steel", "metal", 0.0012, 0.022, "cylinder", 0.5, False),
    ("throne-seat", T("Throne cushion", "Hockerpolster"), "vinyl-seat", "polymer", 0.08, 0.35, "horizontal-plate-up", 0.7, False),
    ("cymbals", T("Hi-hat, crash and ride cymbals with their felts", "Hi-Hat-, Crash- und Ride-Becken mit ihren Filzen"), "bronze-b20", "metal", 0.0015, 0.4, "horizontal-plate-up", 0.8, False),
    ("drum-shells", T("Drum shells with their faded wrap and the bass drum hoops", "Trommelkessel mit verblasster Folie und die Bassdrum-Spannreifen"), "birch-ply", "timber", 0.007, 0.4, "cylinder", 0.5, True),
    ("drum-heads", T("Drumheads, batter and resonant", "Trommelfelle, Schlag- und Resonanzseite"), "pet-head", "polymer", 0.00025, 0.35, "vertical-plate", 0.5, True),
    ("drum-hardware", T("Hoops, lugs, mounts, spurs, floor tom legs and the steel snare shell", "Spannreifen, Böckchen, Halterungen, Sporen, Standtombeine und der Stahlkessel der Snare"), "chrome-steel", "metal", 0.002, 0.03, "cylinder", 0.5, True),
]
HIDDEN = {r[0] for r in REGIONS if r[8]}
MAT = {m[0]: m for m in MATERIALS}


def rnd(v, n=6):
    """🔢️ Rounds a number or nested list."""
    if isinstance(v, (list, tuple)):
        return [rnd(x, n) for x in v]
    r = round(v, n)
    return 0.0 if r == 0 else r


def part_points(kit, p, frame):
    """🧮️ World sample points of a part placed by a bone frame (convex parts: rims of cylinders, box corners, profile rings)."""
    g = p["g"]
    W = lambda q: to_world(frame, q)
    loc = lambda q: q
    if p["kind"] in ("cyl", "tube"):
        r = g["r"] if p["kind"] == "cyl" else g["ro"]
        axis = sub(g["b"], g["a"])
        pts = circle_points(g["a"], axis, r, 48) + circle_points(g["b"], axis, r, 48)
        return [W(q) for q in pts]
    if p["kind"] == "box":
        c, s = g["c"], g["s"]
        return [W([c[0] + i * s[0] / 2, c[1] + j * s[1] / 2, c[2] + k * s[2] / 2]) for i in (-1, 1) for j in (-1, 1) for k in (-1, 1)]
    if p["kind"] == "ext":
        pts = []
        for u in (g["u0"], g["u1"]):
            pts += [[u, g["r"] * math.cos(2 * math.pi * k / 48), g["r"] * math.sin(2 * math.pi * k / 48)] for k in range(48)]
        return [W(q) for q in pts]
    if p["kind"] == "rev":
        pts = []
        for a, r in g["profile"]:
            centre = add(g["o"], mul(g["axis"], a))
            pts += circle_points(centre, g["axis"], r, 48) if r > 0 else [centre]
        return [W(q) for q in pts]
    raise ValueError(p["kind"])


def view_xy(view, w, frame=None):
    """🔭️ The 2D coordinates of a world point in a view: elevation in bone-local (u, v), front (z, y), plan (x, z)."""
    if view == "elevation":
        q = to_local(frame, w)
        return (q[0], q[1])
    if view == "front":
        return (w[2], w[1])
    return (w[0], w[2])


def profile_poly(p, view, frame):
    """🏺️ The exact outline of a revolved part whose axis is vertical at rest: the profile and its mirror."""
    g = p["g"]
    axis = g["axis"]
    pts_r, pts_l = [], []
    if view == "elevation":
        ax = (axis[0], axis[1])
        pp = (-ax[1], ax[0])
        o = (g["o"][0], g["o"][1])
        for a, r in g["profile"]:
            pts_r.append((o[0] + a * ax[0] + r * pp[0], o[1] + a * ax[1] + r * pp[1]))
            pts_l.append((o[0] + a * ax[0] - r * pp[0], o[1] + a * ax[1] - r * pp[1]))
    else:
        ow = to_world(frame, g["o"])
        aw = sub(to_world(frame, add(g["o"], axis)), ow)
        assert abs(aw[1]) > 0.999, p["id"]
        for a, r in g["profile"]:
            y = ow[1] + a * aw[1]
            pts_r.append((ow[2] + r, y))
            pts_l.append((ow[2] - r, y))
    poly = pts_r + list(reversed(pts_l))
    out = []
    for q in poly:
        if not out or abs(out[-1][0] - q[0]) > 1e-7 or abs(out[-1][1] - q[1]) > 1e-7:
            out.append(q)
    if abs(out[0][0] - out[-1][0]) < 1e-7 and abs(out[0][1] - out[-1][1]) < 1e-7:
        out.pop()
    return out


def plan_circle(p, frame):
    """⭕️ Plan outline of a vertical-axis revolved part: the circle of its widest radius."""
    g = p["g"]
    ow = to_world(frame, g["o"])
    R = max(r for _, r in g["profile"])
    return [(ow[0] + R * math.cos(2 * math.pi * k / 64), ow[2] + R * math.sin(2 * math.pi * k / 64)) for k in range(64)]


def diameter(pts):
    """📏️ The two points of a point set farthest apart (the centre line of a thin outline)."""
    best, pair = -1, (pts[0], pts[0])
    for i in range(len(pts)):
        for j in range(i + 1, len(pts)):
            d = (pts[i][0] - pts[j][0]) ** 2 + (pts[i][1] - pts[j][1]) ** 2
            if d > best:
                best, pair = d, (pts[i], pts[j])
    return pair


def thin_centre(pts):
    """〰️ The centre line of a thin convex outline: its long extremes moved onto the middle of its short extent."""
    a, b = diameter(pts)
    d = (b[0] - a[0], b[1] - a[1])
    n = math.hypot(*d)
    if n < 1e-9:
        return [a, b]
    t = (d[0] / n, d[1] / n)
    nn = (-t[1], t[0])
    offs = [(q[0] - a[0]) * nn[0] + (q[1] - a[1]) * nn[1] for q in pts]
    m = (max(offs) + min(offs)) / 2
    return [(a[0] + m * nn[0], a[1] + m * nn[1]), (b[0] + m * nn[0], b[1] + m * nn[1])]


def box_min(pts):
    """📦️ Shorter side of the axis-aligned box of 2D points."""
    xs, ys = [q[0] for q in pts], [q[1] for q in pts]
    return min(max(xs) - min(xs), max(ys) - min(ys))


def shapes_for(kit, scale, parts):
    """✏️ The three views of one scale: member bars for structural parts, hull polygons, exact profiles, strokes at 1:500, plus texture strokes of the traces of use at 1:50."""
    views = {"elevation": [], "front": [], "plan": []}
    rest = kit.rest
    for p in parts:
        if p["flags"].get("thermalOnly") or p["draw"] == "none":
            continue
        frame = rest[p["bone"]]
        if scale >= 200 and "-head-" in p["id"]:
            continue
        if scale == 500 and p["id"].startswith("floor-leg"):
            continue
        fam = member_family(p) and not (scale >= 200 and thinnest(p) < {200: 0.0071, 500: 0.072}[scale]) and not (scale == 500 and p["id"].endswith(("-body", "-wall")))
        if fam:
            p = dict(p, draw="member")
        base = {"id": p["id"], "bone": p["bone"], "role": p["role"], "filled": fam or not outlined(p, scale), "tone": p["tone"], "region": p["region"], "material": p["mat"]}
        for view in views:
            shape = None
            if p["draw"] == "member":
                g = p["g"]
                if p["kind"] in ("cyl", "tube"):
                    shape = dict(base, kind="member", **{"from": {"x": rnd(g["a"][0]), "y": rnd(g["a"][1]), "z": rnd(g["a"][2])}, "to": {"x": rnd(g["b"][0]), "y": rnd(g["b"][1]), "z": rnd(g["b"][2])}}, section={"kind": "circle", "d": rnd(2 * (g["r"] if p["kind"] == "cyl" else g["ro"]))})
                else:
                    c, s = g["c"], g["s"]
                    shape = dict(base, kind="member", **{"from": {"x": rnd(c[0] - s[0] / 2), "y": rnd(c[1]), "z": rnd(c[2])}, "to": {"x": rnd(c[0] + s[0] / 2), "y": rnd(c[1]), "z": rnd(c[2])}}, section={"kind": "rect", "w": rnd(s[1]), "h": rnd(s[2])})
            elif p["draw"] in ("profile", "profile500"):
                if view == "plan":
                    pts = plan_circle(p, frame)
                    shape = dict(base, kind="poly", points=rnd(pts), closed=True)
                elif p["draw"] == "profile500":
                    shape = dict(base, kind="poly", points=rnd(profile_poly(p, view, frame)), closed=True, filled=False)
                else:
                    shape = dict(base, kind="poly", points=rnd(profile_poly(p, view, frame)), closed=True)
            elif p["draw"] in ("hull", "stroke"):
                pts = [view_xy(view, w, frame) for w in part_points(kit, p, frame)]
                hl = hull(pts)
                if len(hl) >= 3 and (p["draw"] == "stroke" or (scale == 500 and box_min(hl) < 0.072)):
                    shape = dict(base, kind="poly", points=rnd(hl), closed=True, filled=False)
                elif len(hl) >= 3:
                    shape = dict(base, kind="poly", points=rnd(hl), closed=True)
            if shape is not None:
                ws = part_points(kit, p, frame)
                near = -min(w[2] for w in ws) if view == "elevation" else (max(w[0] for w in ws) if view == "front" else max(w[1] for w in ws))
                views[view].append((near, shape))
    if scale == 50:
        for view, sh in traces(kit).items():
            views[view] += sh
    for view in views:
        views[view] = [sh for _, sh in sorted(views[view], key=lambda e: e[0])]
    for view, lst in views.items():
        for i, s in enumerate(lst):
            s["z"] = i + 1
    return views


def traces(kit):
    """🩹️ Texture strokes of the traces of use at 1:50: case stickers and scuffed rims, gaffer tape on the floor tom case, strap bands; stick wear on the snare batter head; tarnish rings on the cymbals."""
    out = {"elevation": [], "front": [], "plan": []}
    rest = kit.rest
    near = {}
    tex = lambda i, bone, pts, closed=False: (near[i], {"id": i, "bone": bone, "role": "texture", "filled": False, "tone": 0.5, "kind": "poly", "points": rnd(pts), "closed": closed})
    for cid in ("case-kick", "case-floor"):
        c = L.CASES[cid]
        o = rest[cid][0]
        R, H = c["R"], c["H"]
        el = lambda x, y: tuple(to_local(rest[cid], [x, y, o[2] - R])[:2])
        for suffix in ("sticker", "scuff-1", "scuff-2", "strap-1", "strap-2"):
            near[f"{cid}-{suffix}"] = -(o[2] - R) + 0.001
        for suffix in ("tape", "scuff-3"):
            near[f"{cid}-{suffix}"] = o[0] + R + 0.001
        out["elevation"].append(tex(f"{cid}-sticker", cid, [el(o[0] - 0.10, o[1] + 0.22), el(o[0] + 0.02, o[1] + 0.22), el(o[0] + 0.02, o[1] + 0.30), el(o[0] - 0.10, o[1] + 0.30)], True))
        out["elevation"].append(tex(f"{cid}-scuff-1", cid, [el(o[0] - R + 0.02, o[1] + 0.012), el(o[0] - R + 0.10, o[1] + 0.03), el(o[0] - R + 0.16, o[1] + 0.015)]))
        out["elevation"].append(tex(f"{cid}-scuff-2", cid, [el(o[0] + R - 0.03, o[1] + H - 0.09), el(o[0] + R - 0.12, o[1] + H - 0.075)]))
        for k, dx in enumerate((-0.6, 0.6)):
            out["elevation"].append(tex(f"{cid}-strap-{k + 1}", cid, [el(o[0] + dx * R, o[1] + 0.006), el(o[0] + dx * R, o[1] + H - 0.006)]))
        out["front"].append(tex(f"{cid}-tape", cid, [(o[2] - 0.12, o[1] + 0.14), (o[2] + 0.06, o[1] + 0.16), (o[2] + 0.07, o[1] + 0.21), (o[2] - 0.11, o[1] + 0.19)], True))
        out["front"].append(tex(f"{cid}-scuff-3", cid, [(o[2] + R - 0.02, o[1] + 0.02), (o[2] + R - 0.15, o[1] + 0.04)]))
    so = to_world(rest["snare"], [L.SNARE["D"] + L.HEAD_T, 0, 0])
    near["snare-stick-wear"] = so[1] + 0.0001
    out["plan"].append(tex("snare-stick-wear", "snare", [(so[0] - 0.02 + 0.05 * math.cos(2 * math.pi * k / 16), so[2] + 0.03 + 0.035 * math.sin(2 * math.pi * k / 16)) for k in range(16)], True))
    for cb, R in (("crash-cymbal", 0.203), ("ride-cymbal", 0.254)):
        o = rest[cb][0]
        near[f"{cb}-tarnish"] = o[1] + 0.04
        out["plan"].append(tex(f"{cb}-tarnish", cb, [(o[0] + 0.7 * R * math.cos(2 * math.pi * k / 32), o[2] + 0.7 * R * math.sin(2 * math.pi * k / 32)) for k in range(32)], True))
    return out


def solid(p, sid=None):
    """🧊️ The volume solid of a part in its bone's local frame."""
    g = p["g"]
    base = {"id": sid or p["id"], "bone": p["bone"], "material": p["mat"], "role": p["role"], "tone": p["tone"]}
    for f in ("slender", "sheet"):
        if p["flags"].get(f):
            base[f] = True
    if p["kind"] in ("cyl", "tube"):
        axis = sub(g["b"], g["a"])
        n = norm(axis)
        if p["kind"] == "cyl":
            prof = [[0.0, g["r"]], [n, g["r"]]]
        else:
            prof = [[0.0, g["ri"]], [0.0, g["ro"]], [n, g["ro"]], [n, g["ri"]], [0.0, g["ri"]]]
        return dict(base, kind="revolve", profile=rnd(prof), axis=rnd(unit(axis), 7), origin=rnd(g["a"]))
    if p["kind"] == "box":
        return dict(base, kind="box", size=rnd(g["s"]), transform={"translation": rnd(g["c"])})
    if p["kind"] == "ext":
        hv, hw, hr = g["hole"]
        ring = [[rnd(g["r"] * math.cos(2 * math.pi * k / 64)), rnd(g["r"] * math.sin(2 * math.pi * k / 64))] for k in range(64)]
        hole = [[rnd(hv + hr * math.cos(2 * math.pi * k / 32)), rnd(hw + hr * math.sin(2 * math.pi * k / 32))] for k in range(32)]
        return dict(base, kind="extrude", profile=ring, holes=[hole], plane="yz", **{"from": rnd(g["u0"]), "to": rnd(g["u1"])})
    if p["kind"] == "rev":
        return dict(base, kind="revolve", profile=rnd(g["profile"]), axis=rnd(g["axis"]), origin=rnd(g["o"]))
    raise ValueError(p["kind"])


def pappus(profile):
    """🫙️ Volume and axial centroid of a profile revolved about the axis, closed through the axis when open."""
    poly = list(profile)
    if poly[0] != poly[-1]:
        poly = [[poly[0][0], 0.0]] + poly + [[poly[-1][0], 0.0]]
    A = cx = cr = 0.0
    for (a0, r0), (a1, r1) in zip(poly, poly[1:] + poly[:1]):
        c = a0 * r1 - a1 * r0
        A += c
        cx += (a0 + a1) * c
        cr += (r0 + r1) * c
    A /= 2
    if abs(A) < 1e-15:
        return 0.0, 0.0
    ca, crr = cx / (6 * A), cr / (6 * A)
    return abs(2 * math.pi * crr * A), ca


def mass_of(p):
    """⚖️ Mass and local centroid of a part: section times density, chrome tubes as 1.2 mm walls, castings and frames by their fill."""
    g = p["g"]
    rho = MAT[p["mat"]][4]
    fill = FILL.get(p["mat"], 1.0)
    if p["kind"] == "cyl":
        n = norm(sub(g["b"], g["a"]))
        r = g["r"]
        if p["mat"] == "chrome-steel" and r >= 0.007 and n > 3 * r:
            vol = math.pi * (r * r - (r - TUBE_WALL) ** 2) * n
        elif p["mat"] == "chrome-steel" and r >= 0.007:
            vol = math.pi * r * r * n * 0.35
        else:
            vol = math.pi * r * r * n * (fill if fill > 0 else 1.0)
        return vol * rho, mul(add(g["a"], g["b"]), 0.5)
    if p["kind"] == "tube":
        n = norm(sub(g["b"], g["a"]))
        vol = math.pi * (g["ro"] ** 2 - g["ri"] ** 2) * n
        if p["mat"] == "chrome-steel":
            vol *= 0.65
        return vol * rho, mul(add(g["a"], g["b"]), 0.5)
    if p["kind"] == "box":
        s = g["s"]
        vol = s[0] * s[1] * s[2] * (fill if p["mat"] != "chrome-steel" else 0.3)
        return vol * rho, g["c"]
    if p["kind"] == "ext":
        t = g["u1"] - g["u0"]
        return (math.pi * (g["r"] ** 2 - g["hole"][2] ** 2) * t) * rho, [(g["u0"] + g["u1"]) / 2, 0.0, 0.0]
    v, ca = pappus(g["profile"])
    return v * rho * (fill if p["mat"] != "bronze-b20" else 1.0), add(g["o"], mul(g["axis"], ca))


def area_of(p):
    """📐️ Full surface of a part (cylinder with ends, tube inner and outer, box, revolved profile)."""
    g = p["g"]
    if p["kind"] == "cyl":
        n = norm(sub(g["b"], g["a"]))
        return 2 * math.pi * g["r"] * n + 2 * math.pi * g["r"] ** 2
    if p["kind"] == "tube":
        n = norm(sub(g["b"], g["a"]))
        return 2 * math.pi * (g["ro"] + g["ri"]) * n + 2 * math.pi * (g["ro"] ** 2 - g["ri"] ** 2)
    if p["kind"] == "box":
        s = g["s"]
        return 2 * (s[0] * s[1] + s[1] * s[2] + s[0] * s[2])
    if p["kind"] == "ext":
        hr = g["hole"][2]
        t = g["u1"] - g["u0"]
        return 2 * math.pi * (g["r"] ** 2 - hr ** 2) + 2 * math.pi * (g["r"] + hr) * t
    poly = list(g["profile"])
    if poly[0] != poly[-1]:
        poly = [[poly[0][0], 0.0]] + poly + [[poly[-1][0], 0.0]]
    return sum(math.pi * (r0 + r1) * math.hypot(a1 - a0, r1 - r0) for (a0, r0), (a1, r1) in zip(poly, poly[1:]))


def anchors(kit):
    """📍️ Contacts, supports and load points of the kit, in the local frames of their bones."""
    out = []
    rest = kit.rest

    def a(i, bone, world=None, local=None, kind="contact", title=None, fp=(0.032, 0.032), mu=0.6, space="rest"):
        q = local if local is not None else to_local((rest if space == "rest" else kit.played)[bone], world)
        d = {"id": i, "bone": bone, "x": rnd(q[0]), "y": rnd(q[1]), "kind": kind, "title": title}
        if abs(q[2]) > 1e-9:
            d["z"] = rnd(q[2])
        if kind == "contact":
            d.update({"support": "contact", "footprintLengthM": fp[0], "footprintWidthM": fp[1], "frictionCoefficient": mu})
        out.append(d)

    def feet(bone, cx, cz, reach, azs, en, de):
        for k, az in enumerate(azs):
            a(f"{bone}-foot-{k + 1}", bone, [cx + reach * math.cos(math.radians(az)), 0.0, cz + reach * math.sin(math.radians(az))], title=T(f"{en}, foot {k + 1}", f"{de}, Fuß {k + 1}"))

    t = L.THRONE
    feet("throne", 0.0, 0.0, t["reach"], t["legs"], "Throne", "Hocker")
    for key, en, de in (("hihat", "Hi-hat stand", "Hi-Hat-Maschine"), ("snare-stand", "Snare stand", "Snareständer"), ("crash", "Crash stand", "Crash-Ständer"), ("ride", "Ride stand", "Ride-Ständer")):
        s = L.STANDS[key]
        feet("snare-stand" if key == "snare-stand" else f"{key}-stand", s["at"][0], s["at"][1], s["reach"], s["legs"], en, de)
    a("pedal-heel", "kick-pedal", [0.30, 0.0, 0.0], title=T("Bass drum pedal, heel plate", "Fußmaschine, Fersenplatte"), fp=(0.06, 0.09))
    a("pedal-front", "kick-pedal", [0.565, 0.0, 0.0], title=T("Bass drum pedal, front of the base plate", "Fußmaschine, vorderes Ende der Grundplatte"), fp=(0.06, 0.10))
    a("hihat-heel", "hihat-stand", [0.15, 0.0, L.STANDS["hihat"]["at"][1]], title=T("Hi-hat pedal, heel plate", "Hi-Hat-Pedal, Fersenplatte"), fp=(0.06, 0.09))
    x0, y0, z0 = L.TROLLEY
    for key, xs in (("front", 1), ("rear", -1)):
        for side, zs in (("l", 1), ("r", -1)):
            a(f"trolley-{key}-{side}", "trolley", [x0 + 0.48 * xs, 0.0, z0 + 0.31 * zs], title=T(f"Trolley castor, {key} {'left' if zs > 0 else 'right'}", f"Lenkrolle des Wagens, {'vorn' if xs > 0 else 'hinten'} {'links' if zs > 0 else 'rechts'}"), fp=(0.03, 0.035), mu=0.02)
    for side, zs in (("l", 1), ("r", -1)):
        a(f"kick-spur-{side}-tip", f"kick-spur-{side}", local=[L.SPUR_L, -L.SPUR_TIP_R, 0.0], title=T(f"Bass drum spur tip, {'left' if zs > 0 else 'right'}", f"Spornspitze der Bassdrum, {'links' if zs > 0 else 'rechts'}"), fp=(0.02, 0.02), mu=0.7)
    a("kick-hoop-batter", "kick", local=[0.003, -L.KICK["hoopO"], 0.0], title=T("Bass drum batter hoop on the floor", "Schlagfellreifen der Bassdrum am Boden"), fp=(0.03, 0.05), mu=0.5)
    a("kick-hoop-reso", "kick", local=[L.KICK["D"] - 0.003, -L.KICK["hoopO"], 0.0], title=T("Bass drum resonant hoop on the floor", "Resonanzfellreifen der Bassdrum am Boden"), fp=(0.03, 0.05), mu=0.5)
    for j, az in enumerate((90, 210, 330)):
        a(f"floor-leg-{j + 1}", "floor-tom-legs", local=[-0.006, 0.232 * math.cos(math.radians(az)), 0.232 * math.sin(math.radians(az))], title=T(f"Floor tom leg {j + 1}", f"Standtombein {j + 1}"), fp=(0.026, 0.026), mu=0.7)
    a("seat-point", "throne-seat", local=[L.THRONE["seatRest"] + 0.10 - L.THRONE["tubeTop"], 0.0, 0.0], kind="load", title=T("Seat of the drummer", "Sitzpunkt des Schlagzeugers"))
    a("kick-pedal-toe", "kick-footboard", local=[0.20, 0.018, 0.0], kind="load", title=T("Right foot on the bass drum pedal", "Rechter Fuß auf der Fußmaschine"))
    a("hihat-pedal-toe", "hihat-footboard", local=[0.18, 0.018, 0.0], kind="load", title=T("Left foot on the hi-hat pedal", "Linker Fuß auf dem Hi-Hat-Pedal"))
    a("snare-batter", "snare", local=[L.SNARE["D"] + L.HEAD_T, 0.04, 0.0], kind="load", title=T("Stick on the snare batter head", "Stock auf dem Schlagfell der Snare"))
    a("hihat-stick", "hihat-rod", local=[kit_hihat_top_local(kit), 0.12, 0.0], kind="load", title=T("Stick on the top hi-hat cymbal", "Stock auf dem oberen Hi-Hat-Becken"))
    a("crash-edge", "crash-cymbal", local=[-0.18, 0.012, 0.0], kind="load", title=T("Stick on the crash edge", "Stock am Rand des Crashbeckens"))
    a("trolley-grip", "trolley", [x0 - 0.615, 0.935, z0], kind="load", title=T("Hands on the push handle", "Hände am Schiebebügel"))
    return out


def kit_hihat_top_local(kit):
    """🎩️ Height of the top hi-hat cymbal surface in the frame of the pull rod (local u)."""
    st = L.STANDS["hihat"]
    top = st["tubeTop"] + st["upperRest"] + 0.012 - 0.030 + 0.0015 + L.HIHAT_OPEN + 0.014
    return to_local(kit.rest["hihat-rod"], [st["at"][0], top, st["at"][1]])[0]


SUPPORT_IDS = None


PLAYED_ONLY = ("kick-spur", "kick-hoop", "floor-leg")


def contact_ids(anch):
    """🦶️ The ids of every contact anchor."""
    return [x["id"] for x in anch if x["kind"] == "contact"]


def support_ids(anch):
    """🧷️ The permanent structural supports: the contacts every pose and clip keeps (the drums' own feet touch the floor only when played)."""
    return [i for i in contact_ids(anch) if not i.startswith(PLAYED_ONLY)]


STORED_CONTACTS = None


def contacts_of(anch, pose):
    """🦶️ The contacts active in a pose: the drums' own feet only when played."""
    return [i for i in contact_ids(anch) if pose == "played" or not i.startswith(PLAYED_ONLY)]


JOINTS = [
    ("throne-seat", [0.0, 0.0, 0.0]), ("kick-footboard", [0.0, 0.0, 0.0]), ("kick-beater", [0.0, 0.0, 0.0]),
    ("hihat-upper", [0.0, 0.0, 0.0]), ("snare-basket", [0.0, 0.0, 0.0]), ("crash-upper", [0.0, 0.0, 0.0]), ("ride-upper", [0.0, 0.0, 0.0]),
    ("hihat-footboard", [0.0, 0.0, 0.0]), ("crash-boom", [0.0, 0.0, 0.0]), ("ride-boom", [0.0, 0.0, 0.0]),
    ("kick-spur-l", [0.0, 0.0, 0.0]), ("kick-spur-r", [0.0, 0.0, 0.0]),
]
SECTIONS = {
    "throne": ("chrome-steel", 0.0175, TUBE_WALL, T("Outer tube 35 x 1.6 mm", "Außenrohr 35 x 1,6 mm")),
    "throne-seat": ("chrome-steel", 0.014, 0.0015, T("Seat post 28 x 1.5 mm", "Sitzrohr 28 x 1,5 mm")),
    "hihat-stand": ("chrome-steel", 0.0127, TUBE_WALL, T("Lower tube 25.4 x 1.6 mm", "Unterrohr 25,4 x 1,6 mm")),
    "snare-stand": ("chrome-steel", 0.011, TUBE_WALL, T("Lower tube 22 x 1.6 mm", "Unterrohr 22 x 1,6 mm")),
    "crash-stand": ("chrome-steel", 0.0127, TUBE_WALL, T("Lower tube 25.4 x 1.6 mm", "Unterrohr 25,4 x 1,6 mm")),
    "ride-stand": ("chrome-steel", 0.0127, TUBE_WALL, T("Lower tube 25.4 x 1.6 mm", "Unterrohr 25,4 x 1,6 mm")),
    "hihat-upper": ("chrome-steel", 0.0095, TUBE_WALL, T("Upper tube 19 x 1.6 mm", "Oberrohr 19 x 1,6 mm")),
    "crash-upper": ("chrome-steel", 0.0095, TUBE_WALL, T("Upper tube 19 x 1.6 mm", "Oberrohr 19 x 1,6 mm")),
    "ride-upper": ("chrome-steel", 0.0095, TUBE_WALL, T("Upper tube 19 x 1.6 mm", "Oberrohr 19 x 1,6 mm")),
    "crash-boom": ("chrome-steel", 0.0095, TUBE_WALL, T("Boom tube 19 x 1.6 mm", "Galgenrohr 19 x 1,6 mm")),
    "ride-boom": ("chrome-steel", 0.0095, TUBE_WALL, T("Boom tube 19 x 1.6 mm", "Galgenrohr 19 x 1,6 mm")),
}
MEMBER_R = {"kick-holder": 0.011, "kick-spur-l": 0.008, "kick-spur-r": 0.008}


def tube_section(mat, r, t, title):
    """📏️ Section of a thin round tube."""
    ri = r - t
    return {"areaM2": rnd(math.pi * (r * r - ri * ri), 9), "secondMomentM4": rnd(math.pi / 4 * (r ** 4 - ri ** 4), 12), "extremeFibreM": r, "material": mat, "title": title}


MASS_MARKERS = {"kick", "tom10", "tom12", "floor-tom", "snare", "case-kick", "case-floor", "case-tom12", "case-tom10", "case-snare"}


MEMBER_BONES = ("kick", "tom10", "tom12", "floor-tom", "case-", "trolley", "rug")


def member_family(p):
    """🧱️ Whether a part is drawn as a 2D member bar at every scale: the parts of the drums, cases, trolley and rug, which are packed inside one another or turn between the poses, so their outlines stay true in every pose and their painting order follows their real depth."""
    return p["kind"] in ("cyl", "tube", "box") and (p["bone"].startswith(MEMBER_BONES) or p["bone"] == "snare") and p["bone"] not in ("kick-pedal", "kick-footboard", "kick-beater")


def thinnest(p):
    """📏️ The smallest extent of a part."""
    g = p["g"]
    if p["kind"] == "cyl":
        return min(2 * g["r"], norm(sub(g["b"], g["a"])))
    if p["kind"] == "tube":
        return min(2 * g["ro"], norm(sub(g["b"], g["a"])))
    if p["kind"] == "box":
        return min(g["s"])
    return 1.0


def structural(p):
    """🏗️ Whether a body part carries load: everything drawn as a member bar, the tubes, legs and boards; not the cymbals with their felts and the cushion."""
    if member_family(p):
        return True
    return not (p["id"].endswith(tuple(f"-lug-{k}" for k in range(1, 11))) or p["region"] in ("cymbals", "throne-seat"))


OUTLINE = ("rug", "-lug-", "-hoop-", "-head-", "-bracket", "kick-mount", "holder-", "kick-spur-", "castor-plate", "snare-strainer", "snare-butt")


def outlined(p, scale):
    """✒️ Whether a part is drawn as an unfilled closed outline: the small or covered parts that only show as slivers beside their neighbours, and at 1:500 every drum and case."""
    if p["id"].startswith("case-") and p["id"].endswith(("-lid", "-floor", "-skirt")):
        return True
    if scale == 500 and p["id"].endswith(("-body", "-wall")):
        return True
    if scale >= 200 and p["id"].endswith("-shell"):
        return True
    if scale >= 200 and p["kind"] == "cyl" and p["g"]["r"] < 0.006:
        return True
    return any(k in p["id"] for k in OUTLINE) and not p["id"].startswith("floor-leg")


def eligible(bones, height):
    """🏗️ Load-bearing bones as the structural view colours them: a section, or at least 5 % of the drawn height long."""
    return [b["id"] for b in bones if "section" in b or b["lengthM"] >= 0.05 * height]


def volume(kit, scale, parts, anch, bones, height):
    """🧊️ The body, thermal and structure lists of one scale."""
    f = FEATURE[scale]
    body = [solid(p) for p in parts if not p["flags"].get("thermalOnly")]
    thermal = []
    for p in parts:
        if p["flags"].get("noThermal"):
            continue
        if p["region"] in HIDDEN or p["flags"].get("thermalOnly"):
            thermal.append(dict(solid(p, "th-" + p["id"]), region=p["region"]))
        else:
            thermal.append({"id": "th-" + p["id"], "of": p["id"], "region": p["region"]})
    structure = []
    covered = set()
    for p in parts:
        if (p["struct"] or structural(p)) and not p["flags"].get("thermalOnly"):
            structure.append({"id": "st-" + p["id"], "of": p["id"], "member": p["bone"], "part": "member"})
            covered.add(p["bone"])
    by = {b["id"]: b for b in bones}
    for bid in eligible(bones, height):
        if bid in covered:
            continue
        r = MEMBER_R.get(bid, 0.01)
        ln = by[bid]["lengthM"]
        structure.append({"id": f"st-{bid}-member", "kind": "revolve", "bone": bid, "material": "chrome-steel", "role": "body", "tone": 0.5, "slender": True, "profile": rnd([[0.0, r], [ln, r]]), "axis": [1, 0, 0], "origin": [0, 0, 0], "member": bid, "part": "member"})
    jr = max(f / 2, 0.012)
    for bid, q in JOINTS[: {50: 12, 100: 10, 200: 8, 500: 4}[scale]]:
        structure.append({"id": f"joint-{bid}", "kind": "ellipsoid", "bone": bid, "material": "chrome-steel", "role": "detail", "tone": 0.25, "radii": [jr, jr, jr], "transform": {"translation": rnd(q)}, "member": bid, "part": "joint"})
    for x in anch:
        if x["id"] not in support_ids(anch):
            continue
        fx = max(x["footprintLengthM"], f)
        fz = max(x["footprintWidthM"], f)
        loc = [x["x"], x["y"], x.get("z", 0.0)]
        frame = kit.rest[x["bone"]]
        dn = dir_local(frame, [0.0, -1.0, 0.0])
        size = [fx, 0.006, fz]
        if abs(dn[0]) > 0.9:
            size = [0.006, fx, fz]
        c = add(loc, mul(dn, 0.003))
        structure.append({"id": f"support-{x['id']}", "kind": "box", "bone": x["bone"], "material": "rubber", "role": "ground", "tone": 0.15, "size": rnd(size), "transform": {"translation": rnd(c)}, "member": x["bone"], "part": "support", "anchor": x["id"]})
    for b in bones:
        m = b.get("mass")
        if m and m["kg"] >= 2.0 and b["id"] in MASS_MARKERS:
            structure.append({"id": f"mass-{b['id']}", "kind": "ellipsoid", "bone": b["id"], "material": "chrome-steel", "role": "detail", "tone": 0.2, "radii": [f / 2, f / 2, f / 2], "transform": {"translation": rnd([m["comX"], m["comY"], 0.0])}, "member": b["id"], "part": "mass"})
    return body, thermal, structure


def world_bbox(kit, parts):
    """📦️ The rest-pose world box of the body parts."""
    xs, ys, zs = [], [], []
    for p in parts:
        if p["flags"].get("thermalOnly"):
            continue
        for w in part_points(kit, p, kit.rest[p["bone"]]):
            xs.append(w[0]); ys.append(w[1]); zs.append(w[2])
    return (min(xs), max(xs)), (min(ys), max(ys)), (min(zs), max(zs))


def region_areas(parts):
    """📐️ Area per thermal region of the 1:50 parts: the full surface of the hidden contents, the envelope surface of what the air reaches."""
    out = {}
    for p in parts:
        out[p["region"]] = out.get(p["region"], 0.0) + area_of(p)
    return out


def bone_masses(kit, parts):
    """⚖️ Mass and local centre of every bone from the 1:50 parts."""
    acc = {}
    for p in parts:
        if p["flags"].get("thermalOnly"):
            continue
        m, c = mass_of(p)
        e = acc.setdefault(p["bone"], [0.0, [0.0, 0.0, 0.0]])
        e[0] += m
        e[1] = add(e[1], mul(c, m))
    return {k: (v[0], mul(v[1], 1 / v[0])) for k, v in acc.items() if v[0] > 0}


def binding(pose):
    """🌡️ The region bindings of a pose: indoors without sun; the drums' regions shut in their cases unless played."""
    out = {}
    for r in REGIONS:
        exp = 1.0
        if r[0] in HIDDEN and pose != "played":
            exp = 0.0
        if r[0] == "trolley-deck":
            exp = 0.45
        out[r[0]] = {"exposedFraction": exp, "sunlitFraction": 0.0}
    return {"regions": out, "emitters": {}}


def load(i, en, de, anchor, fy, fx=0.0, src="authored-estimate", ref="kit-setup"):
    """🧲️ An applied load at an anchor."""
    return {"id": i, "kind": "live" if i != "occupant" else "occupant", "title": T(en, de), "fxN": fx, "fyN": fy, "source": src, "anchor": anchor, "reference": ref}


def timeline(points, t):
    """📈️ Piecewise-linear value of a keyframe list at time t."""
    for (t0, v0), (t1, v1) in zip(points, points[1:]):
        if t0 <= t <= t1:
            return v0 + (v1 - v0) * (t - t0) / (t1 - t0) if t1 > t0 else v1
    return points[-1][1]


GROOVE = {
    "kick-beater": ("angleDeg", [(0, L.BEATER_STRIKE), (0.14, 0), (0.86, 0), (1.0, L.BEATER_STRIKE), (1.14, 0), (1.86, 0), (2.0, L.BEATER_STRIKE)]),
    "kick-footboard": ("angleDeg", [(0, -9), (0.14, 0), (0.86, 0), (1.0, -9), (1.14, 0), (1.86, 0), (2.0, -9)]),
    "hihat-rod": ("dx", [(0, -L.HIHAT_OPEN), (1.68, -L.HIHAT_OPEN), (1.75, 0.0), (1.85, 0.0), (1.95, -L.HIHAT_OPEN), (2.0, -L.HIHAT_OPEN)]),
    "hihat-footboard": ("angleDeg", [(0, -6), (1.68, -6), (1.75, 0), (1.85, 0), (1.95, -6), (2.0, -6)]),
    "crash-cymbal": ("angleDeg", [(0, -49), (0.15, -42), (0.3, -47.5), (0.45, -43.5), (0.6, -45), (1.95, -45), (2.0, -49)]),
}


def clips(anch, kit):
    """🎞️ The clips `carry` (the case stack rolled away on its trolley) and `groove` (one bar of a rock beat: bass drum strike and rebound, backbeat, hi-hat chick, crash wobble)."""
    carry_contacts = [{"anchor": i, "fromS": 0, "toS": 6} for i in contacts_of(anch, "carried")]
    wheel = lambda dx: -dx / 0.0625 * 180 / math.pi
    keys = []
    for t, dx in ((0.0, 0.0), (1.0, 0.0), (5.0, 2.0), (6.0, 2.0)):
        keys.append({"t": t, "pose": "carried", "ease": "smooth", "bones": {"trolley": {"dx": dx}, "trolley-wheels-front": {"angleDeg": rnd(wheel(dx), 4)}, "trolley-wheels-rear": {"angleDeg": rnd(wheel(dx), 4)}}})
    carry = {"id": "carry", "title": T("Roll the case stack away", "Kofferstapel wegrollen"), "description": T("The drums packed in their hard cases are pushed away on the platform trolley, 2 m in four seconds: the castors roll, the hardware stays standing.", "Die in ihren Hartschalenkoffern verpackten Trommeln werden auf dem Plattformwagen weggeschoben, 2 m in vier Sekunden: die Lenkrollen rollen, die Ständer bleiben stehen."), "durationS": 6, "fps": 10, "loop": False, "rootMotion": "none", "keys": keys, "contacts": carry_contacts, "phases": [{"id": "start", "fromS": 0, "toS": 1, "title": T("Taking hold of the handle", "Bügel greifen")}, {"id": "roll", "fromS": 1, "toS": 5, "title": T("Rolling", "Rollen")}, {"id": "parked", "fromS": 5, "toS": 6, "title": T("Parked", "Abgestellt")}], "events": [{"t": 1.0, "kind": "motion", "title": T("The trolley starts to roll", "Der Wagen rollt an"), "target": "trolley"}, {"t": 5.0, "kind": "motion", "title": T("The trolley stops", "Der Wagen hält"), "target": "trolley"}], "tracks": [{"target": "load:push", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 0}, {"t": 1.5, "v": 1}, {"t": 4.2, "v": 0.5}, {"t": 5, "v": 0}, {"t": 6, "v": 0}]}]}
    base = L.played_offsets(kit.plays)
    times = sorted({t for _, pts in GROOVE.values() for t, _ in pts} | {0.5, 1.5, 0.25, 0.75, 1.25})
    gkeys = []
    for t in times:
        bones = {}
        for bid, (field, pts) in GROOVE.items():
            v = timeline(pts, t)
            bones[bid] = {field: rnd(v, 5)}
        gkeys.append({"t": rnd(t, 4), "pose": "played", "ease": "linear", "bones": bones})
    def pulse(hits, w=0.05):
        ks = [{"t": 0.0, "v": 1 if 0 in hits else 0}]
        if 0 in hits:
            ks.append({"t": w, "v": 0})
        for h in hits:
            if h > 0:
                ks += [{"t": rnd(h - w, 3), "v": 0}, {"t": h, "v": 1}, {"t": rnd(h + w, 3), "v": 0}]
        if 0 in hits:
            ks.append({"t": rnd(2.0 - w, 3), "v": 0})
        ks.append({"t": 2.0, "v": 1 if 0 in hits else 0})
        return ks

    norm_keys = lambda ks: sorted({k["t"]: k for k in ks}.values(), key=lambda k: k["t"])
    groove_contacts = [{"anchor": i, "fromS": 0, "toS": 2} for i in contacts_of(anch, "played")]
    hats = [round(0.25 * k, 2) for k in range(8)]
    groove = {"id": "groove", "title": T("Rock groove, one bar", "Rock-Groove, ein Takt"), "description": T("One bar of a straight rock beat at 120 bpm: the felt beater strikes the bass drum head on one and three and rebounds, the stick hits the snare on two and four, eighth notes on the closed hi-hat with the pedal opening on four-and, the crash on one swings on its felts.", "Ein Takt eines geraden Rockbeats mit 120 bpm: der Filzschlägel trifft das Bassdrumfell auf eins und drei und federt zurück, der Stock schlägt die Snare auf zwei und vier, Achtel auf der geschlossenen Hi-Hat mit Öffnen des Pedals auf vier-und, das Crash auf eins schwingt auf seinen Filzen."), "durationS": 2.0, "fps": 25, "loop": True, "rootMotion": "none", "keys": gkeys, "contacts": groove_contacts, "phases": [{"id": "beat-1", "fromS": 0, "toS": 0.5, "title": T("One: bass drum and crash", "Eins: Bassdrum und Crash")}, {"id": "beat-2", "fromS": 0.5, "toS": 1.0, "title": T("Two: snare", "Zwei: Snare")}, {"id": "beat-3", "fromS": 1.0, "toS": 1.5, "title": T("Three: bass drum", "Drei: Bassdrum")}, {"id": "beat-4", "fromS": 1.5, "toS": 2.0, "title": T("Four: snare and open hi-hat", "Vier: Snare und offene Hi-Hat")}], "events": [{"t": 0, "kind": "motion", "title": T("Beater strikes the bass drum head", "Schlägel trifft das Bassdrumfell"), "target": "kick-beater"}, {"t": 0.5, "kind": "load-on", "title": T("Backbeat on the snare", "Backbeat auf der Snare"), "target": "snare-stroke"}, {"t": 1.0, "kind": "motion", "title": T("Beater strikes again", "Schlägel trifft erneut"), "target": "kick-beater"}, {"t": 1.5, "kind": "load-on", "title": T("Backbeat on the snare", "Backbeat auf der Snare"), "target": "snare-stroke"}, {"t": 1.75, "kind": "motion", "title": T("Hi-hat opens", "Hi-Hat öffnet"), "target": "hihat-rod"}], "tracks": [
        {"target": "load:kick-foot", "ease": "linear", "keys": [{"t": 0, "v": 3}, {"t": 0.08, "v": 1}, {"t": 0.92, "v": 1}, {"t": 1.0, "v": 3}, {"t": 1.08, "v": 1}, {"t": 1.92, "v": 1}, {"t": 2.0, "v": 3}]},
        {"target": "load:snare-stroke", "ease": "linear", "keys": norm_keys(pulse([0.5, 1.5]))},
        {"target": "load:hihat-stroke", "ease": "linear", "keys": norm_keys(pulse(hats))},
        {"target": "load:crash-stroke", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 0.05, "v": 0}, {"t": 1.95, "v": 0}, {"t": 2.0, "v": 1}]},
        {"target": "load:hihat-foot", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 1.68, "v": 1}, {"t": 1.75, "v": 0.2}, {"t": 1.85, "v": 0.2}, {"t": 1.95, "v": 1}, {"t": 2.0, "v": 1}]},
    ]}
    return [carry, groove]


def poses(anch, kit):
    """🧍️ The named poses `stored` (default), `carried` and `played`."""
    played_loads = [
        load("occupant", "Seated drummer (75 kg, 600 N on the seat)", "Sitzender Schlagzeuger (75 kg, 600 N auf dem Sitz)", "seat-point", -600.0),
        load("kick-foot", "Right foot resting on the bass drum pedal", "Rechter Fuß ruht auf der Fußmaschine", "kick-pedal-toe", -40.0),
        load("hihat-foot", "Left foot holding the hi-hat closed", "Linker Fuß hält die Hi-Hat geschlossen", "hihat-pedal-toe", -60.0),
        load("snare-stroke", "Stick stroke on the snare (peak)", "Stockschlag auf die Snare (Spitze)", "snare-batter", -120.0, ref="rossing-2000"),
        load("hihat-stroke", "Stick stroke on the hi-hat (peak)", "Stockschlag auf die Hi-Hat (Spitze)", "hihat-stick", -50.0, ref="rossing-2000"),
        load("crash-stroke", "Stick stroke on the crash edge (peak)", "Stockschlag auf den Crashrand (Spitze)", "crash-edge", -80.0, ref="rossing-2000"),
    ]
    wheel = -2.0 / 0.0625 * 180 / math.pi
    return [
        {"id": "stored", "title": T("Stored", "Eingelagert"), "description": T("At the dealer: the five drums packed in their hard cases, stacked on a platform trolley; stands collapsed with the cymbals left on them, throne lowered, pedal on the floor.", "Beim Händler: die fünf Trommeln in ihren Hartschalenkoffern auf einem Plattformwagen gestapelt; Ständer eingeschoben, die Becken darauf belassen, Hocker abgesenkt, Fußmaschine am Boden."), "bones": {}, "ground": "auto", "contacts": contacts_of(anch, "stored"), "thermal": binding("stored"), "loads": []},
        {"id": "carried", "title": T("Carried on the trolley", "Auf dem Wagen gerollt"), "description": T("The case stack pushed 2 m on its trolley with 40 N on the handle; the hardware stays where it stands.", "Der Kofferstapel 2 m auf seinem Wagen geschoben, 40 N am Bügel; die Ständer bleiben stehen."), "bones": {"trolley": {"dx": 2.0}, "trolley-wheels-front": {"angleDeg": rnd(wheel, 4)}, "trolley-wheels-rear": {"angleDeg": rnd(wheel, 4)}}, "ground": "auto", "contacts": contacts_of(anch, "carried"), "thermal": binding("carried"), "loads": [load("push", "Push on the trolley handle", "Schub am Schiebebügel", "trolley-grip", 0.0, fx=40.0, ref="en-12530")]},
        {"id": "played", "title": T("Played", "Gespielt"), "description": T("Set up and played seated at the instant of a backbeat: drums out of their cases on stands, holder and spurs, stands extended, booms swung out, throne raised; the empty cases stay stacked on the trolley.", "Aufgebaut und sitzend gespielt im Augenblick eines Backbeats: die Trommeln aus den Koffern auf Ständern, Halter und Sporen, Ständer ausgezogen, Galgen ausgeschwenkt, Hocker angehoben; die leeren Koffer bleiben auf dem Wagen gestapelt."), "bones": L.played_offsets(kit.plays), "ground": "auto", "contacts": contacts_of(anch, "played"), "thermal": binding("played"), "loads": played_loads},
    ]


REFERENCES = [
    {"id": "hornbostel-sachs-1914", "citation": "von Hornbostel, E. M.; Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590: drums of the kit 211.212.1 (individual double-skin cylindrical drums, struck), snare drum with snares; suspended cymbals 111.24 (percussion vessels), hi-hat 111.142 (concussion vessels)."},
    {"id": "fletcher-rossing-1998", "citation": "Fletcher, N. H.; Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed., Springer, chapters 18 (drums) and 20 (cymbals and gongs): membrane modes, bass drum and tom construction, cymbal profiles and B20 bronze."},
    {"id": "rossing-2000", "citation": "Rossing, T. D. (2000). Science of Percussion Instruments, World Scientific: drumheads of polyester film, stick and beater contact, cymbal construction and weights."},
    {"id": "iso-10456", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (timber, plywood, polyethylene, steel, aluminium, rubber)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P.; DeWitt, D. P.; Bergman, T. L.; Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of chrome, bronze and polymers; free convection)."},
    {"id": "en-12530", "citation": "EN 12530:1999 Castors and wheels - Castors and wheels for manually propelled general purpose applications (125 mm castors, rolling resistance)."},
    {"id": "kit-catalogue", "citation": "Catalogue-typical data of second-hand student and rock drum kits of the 1980s to 2000s (five-piece 'rock' configuration 22 x 18, 10 x 8, 12 x 9, 16 x 16 and 14 x 5.5 inch; 6 to 8 ply birch or basswood shells 6 to 7 mm, steel snare shell, 2.3 mm triple-flange hoops, wood bass drum hoops; single-ply 10 mil polyester heads; B20 cymbals 14 inch hi-hat, 16 inch crash, 20 inch ride; double-braced stands with 25.4 and 19 mm tubes; round rotomoulded polyethylene hard cases) as offered by used-instrument dealers; no maker, model or logo is reproduced."},
    {"id": "kit-setup", "citation": "Authored estimate of a seated drummer's set-up (seat 0.56 m, snare batter head 0.65 m, hi-hat 0.89 m, toms tilted 10 degrees, crash 1.11 m, ride 1.04 m) and of the playing loads (seated 600 N, foot on a pedal 40 to 120 N, stick strokes 50 to 120 N peak)."},
]


def decisions_rep(scale, dims):
    """📝️ The authored design decisions of one representation."""
    k = 1000 / scale
    w, h, d = dims["widthM"] * k, dims["heightM"] * k, dims["depthM"] * k
    f = {50: 7.5, 100: 15, 200: 30, 500: 75}[scale]
    common = (f"Side elevation {w:.0f} mm wide and {h:.0f} mm high, front {d:.0f} mm wide, plan {w:.0f} by {d:.0f} mm", f"Seitenansicht {w:.0f} mm breit und {h:.0f} mm hoch, Vorderansicht {d:.0f} mm breit, Draufsicht {w:.0f} auf {d:.0f} mm")
    if scale == 50:
        return {
            "silhouette": T(f"{common[0]}: the case stack on the platform trolley (bass drum case with the two rack tom cases, floor tom case with the snare case, push handle) beside the collapsed hardware: double-braced tripods, telescoped tubes, the throne lowered, the hi-hat closed to 12 mm, crash and ride on their raised booms; played, every drum stands on its stand, holder, spurs or legs with hoops, heads and lugs.", f"{common[1]}: der Kofferstapel auf dem Plattformwagen (Bassdrumkoffer mit den zwei Hängetomkoffern, Standtomkoffer mit dem Snarekoffer, Schiebebügel) neben den eingeschobenen Ständern: doppelt verstrebte Dreibeine, eingeschobene Rohre, der abgesenkte Hocker, die Hi-Hat auf 12 mm geschlossen, Crash und Ride auf ihren aufgerichteten Galgen; gespielt steht jede Trommel auf Ständer, Halter, Sporen oder Beinen mit Reifen, Fellen und Böckchen."),
            "retained": T("Every case wall, lid, skirt and floor; deck, rails, castor forks and wheels; every tube, leg, brace, collar, foot, clamp, boom tilter and felt; the cymbals in their true bowed section with the bell; shells, hoops, heads, tube lugs, mounts, brackets, spurs and floor tom legs of every drum.", "Jede Kofferwand mit Deckel, Krempe und Boden; Ladefläche, Rahmen, Rollengabeln und Rollen; jedes Rohr, Bein, jede Strebe, Muffe, Fuß, Klemme, Galgenneiger und Filz; die Becken in ihrem wahren gewölbten Schnitt mit der Kuppe; Kessel, Reifen, Felle, Rohrböckchen, Halterungen, Sporen und Standtombeine jeder Trommel."),
            "merged": T("Tension rods and claws merge into the full-length tube lugs, the snare wires into the bottom head, the pedal cam and spring into the axle housing, the wing nuts into the top felts.", "Spannschrauben und Klauen gehen in den durchgehenden Rohrböckchen auf, der Snareteppich im Resonanzfell, Exzenter und Feder der Fußmaschine im Achsgehäuse, die Flügelmuttern in den oberen Filzen."),
            "omitted": T("Air vents, screws, rivets and the snare throw-off lever; the drums inside their closed cases in the stored drawing are drawn but covered by the case walls.", "Entlüftungslöcher, Schrauben, Nieten und der Abhebehebel der Snare; die Trommeln in ihren geschlossenen Koffern sind in der eingelagerten Zeichnung gezeichnet, aber von den Kofferwänden verdeckt."),
            "thinElements": T("The 7 mm hi-hat pull rod, the 8 mm beater shaft, the 10.5 mm floor tom legs and the 13 mm holder arms keep their true sections; cymbals and heads are true-thickness sections inside the pen.", "Die 7-mm-Zugstange der Hi-Hat, der 8-mm-Schlägelschaft, die 10,5-mm-Standtombeine und die 13-mm-Halterarme behalten ihre wahren Querschnitte; Becken und Felle sind Schnitte in wahrer Dicke innerhalb der Linie."),
            "hatching": T("No hatches; materials read by tone: black cases, plywood deck, blue frame, chrome, bronze, white heads, faded wrap.", "Keine Schraffuren; die Stoffe lesen sich über den Ton: schwarze Koffer, Sperrholzdeck, blauer Rahmen, Chrom, Bronze, weiße Felle, verblasste Folie."),
            "styles": T("Body pen 0.5 mm, detail 0.25 mm for lugs and braces, texture 0.13 mm for the traces of use.", "Körperlinie 0,5 mm, Detail 0,25 mm für Böckchen und Streben, Textur 0,13 mm für die Gebrauchsspuren."),
            "overlays": T("Traces of use as texture strokes: stickers, scuffed rims, gaffer tape and strap bands on the cases, a stick-wear patch on the snare batter head, tarnish rings on crash and ride; the port hole cut into the bass drum's resonant head is real geometry of the 3D model.", "Gebrauchsspuren als Texturstriche: Aufkleber, verkratzte Ränder, Gewebeband und Gurtbänder an den Koffern, eine Stockspur auf dem Snare-Schlagfell, Anlaufringe auf Crash und Ride; das in das Resonanzfell der Bassdrum geschnittene Loch ist echte Geometrie des 3D-Modells."),
        }
    if scale == 100:
        return {
            "silhouette": T(f"{common[0]}: the same case stack, trolley and hardware as at 1:50 without the bracing details.", f"{common[1]}: derselbe Kofferstapel, Wagen und dieselben Ständer wie bei 1:50 ohne die Verstrebungsdetails."),
            "retained": T("Cases with lids and skirts, deck, rails, castors, handle; tubes, legs, feet, hubs and clamps; cymbals in section; shells, hoops, heads, brackets, spurs, legs; the 20 mm bass drum lugs.", "Koffer mit Deckeln und Krempen, Ladefläche, Rahmen, Rollen, Bügel; Rohre, Beine, Füße, Naben und Klemmen; Becken im Schnitt; Kessel, Reifen, Felle, Halterungen, Sporen, Beine; die 20-mm-Böckchen der Bassdrum."),
            "merged": T("The 14 to 18 mm lugs of toms, floor tom and snare merge into the shells, braces into the legs, the felts into the cymbal mounts, the castor top plates into the forks.", "Die 14- bis 18-mm-Böckchen von Toms, Standtom und Snare gehen in den Kesseln auf, die Streben in den Beinen, die Filze in den Beckenhaltern, die Kopfplatten der Rollen in den Gabeln."),
            "omitted": T("Texture of the traces of use, the pedal drive strap, the hi-hat seat cup.", "Textur der Gebrauchsspuren, das Antriebsband der Fußmaschine, der Sitzteller der Hi-Hat."),
            "thinElements": T("Pull rod, beater shaft, floor tom legs keep their true sections as slender parts; the snare basket arms and holder arms are drawn at 15 mm.", "Zugstange, Schlägelschaft und Standtombeine behalten als schlanke Teile ihre wahren Querschnitte; Korbarme und Halterarme sind mit 15 mm gezeichnet."),
            "hatching": T("No hatches; tones as at 1:50.", "Keine Schraffuren; Töne wie bei 1:50."),
            "styles": T("Body pen 0.25 mm, detail 0.13 mm.", "Körperlinie 0,25 mm, Detail 0,13 mm."),
            "overlays": T("None.", "Keine."),
        }
    if scale == 200:
        return {
            "silhouette": T(f"{common[0]}: case cylinders on the trolley, the tripods and tubes of the hardware, the throne cushion, the cymbals as bowed sections.", f"{common[1]}: Kofferzylinder auf dem Wagen, die Dreibeine und Rohre der Ständer, das Hockerpolster, die Becken als gewölbte Schnitte."),
            "retained": T("Case walls with flush lids, deck, rails, castors and handle; legs, feet, tubes, hubs, booms; shells, hoops, heads, brackets, spurs, holder.", "Kofferwände mit bündigen Deckeln, Ladefläche, Rahmen, Rollen und Bügel; Beine, Füße, Rohre, Naben, Galgen; Kessel, Reifen, Felle, Halterungen, Sporen, Halter."),
            "merged": T("Lid skirts into the walls, all lugs into the shells, felts and tilters into one mount, the hi-hat pair into two thin bows.", "Deckelkrempen in den Wänden, alle Böckchen in den Kesseln, Filze und Neiger in einem Halter, das Hi-Hat-Paar in zwei dünnen Wölbungen."),
            "omitted": T("Braces, castor plates, drive strap, strainer details below 30 mm.", "Streben, Rollenplatten, Antriebsband, Abhebungsdetails unter 30 mm."),
            "thinElements": T("Tubes and legs are drawn as filled outlines of their true 19 to 35 mm sections (0.1 to 0.18 mm on paper) with the 0.18 mm pen.", "Rohre und Beine sind als gefüllte Umrisse ihrer wahren Querschnitte von 19 bis 35 mm (0,1 bis 0,18 mm auf dem Papier) mit der 0,18-mm-Linie gezeichnet."),
            "hatching": T("No hatches.", "Keine Schraffuren."),
            "styles": T("Body pen 0.18 mm, detail 0.13 mm.", "Körperlinie 0,18 mm, Detail 0,13 mm."),
            "overlays": T("None.", "Keine."),
        }
    return {
        "silhouette": T(f"{common[0]}: the case stack as cylinders on the deck stroke and four castor blocks, the hardware as strokes with 75 mm legs and posts, the cymbals as strokes in elevation and discs in plan.", f"{common[1]}: der Kofferstapel als Zylinder auf dem Strich der Ladefläche und vier Rollenblöcken, die Ständer als Striche mit 75-mm-Beinen und Stützen, die Becken als Striche in der Ansicht und Scheiben in der Draufsicht."),
        "retained": T("Five case cylinders, deck, castor blocks, handle; legs, tubes, pedals, booms, cymbals, throne cushion; each drum as one turned mass of its hoop diameter.", "Fünf Kofferzylinder, Ladefläche, Rollenblöcke, Bügel; Beine, Rohre, Pedale, Galgen, Becken, Hockerpolster; jede Trommel als ein gedrehter Körper ihres Reifendurchmessers."),
        "merged": T("Hoops, heads and shell of each drum into one cylinder, castor forks with wheels into blocks, base plate and footboard of each pedal into one block.", "Reifen, Felle und Kessel jeder Trommel in einem Zylinder, Rollengabeln mit Rollen in Blöcken, Grundplatte und Trittplatte jedes Pedals in einem Block."),
        "omitted": T("Lugs, brackets, holder, spurs, feet, collars, felts, clutch, basket: all below 75 mm.", "Böckchen, Halterungen, Halter, Sporen, Füße, Muffen, Filze, Clutch, Korb: alle unter 75 mm."),
        "thinElements": T("Tubes, rods, deck and cymbals that are thinner than 72 mm in a view are 0.13 mm strokes along their centre lines; legs, the throne post and short tubes are drawn and modelled at the 75 mm feature.", "Rohre, Stangen, Ladefläche und Becken, die in einer Ansicht dünner als 72 mm sind, sind 0,13-mm-Striche auf ihren Mittellinien; Beine, die Hockerstange und kurze Rohre sind mit dem 75-mm-Merkmal gezeichnet und modelliert."),
        "hatching": T("No hatches.", "Keine Schraffuren."),
        "styles": T("Body pen 0.13 mm only.", "Nur Körperlinie 0,13 mm."),
        "overlays": T("None.", "Keine."),
    }


def decisions_vol(scale):
    """🧾️ What one scale's 3D model keeps, merges and why."""
    if scale == 50:
        return [
            T("Body: the real kit built part by part: five rotomoulded cases (5 mm wall, 6 mm floor and lid, 4 mm lid skirt), the plywood deck on its steel rails, four castors and the push handle; double-braced tripods with rubber feet, telescoping tubes, memory clamps, boom tilters, felts; the cymbals as revolved thin bowed sections with bell; drums with ply shells, triple-flange or wood hoops, 0.5 mm heads (the resonant bass drum head with its cut port hole), full-length tube lugs, mounts, brackets, folding spurs and sliding floor tom legs.", "Körper: das echte Schlagzeug Teil für Teil: fünf rotationsgeformte Koffer (5-mm-Wand, 6-mm-Boden und -Deckel, 4-mm-Deckelkrempe), die Sperrholz-Ladefläche auf Stahlrahmen, vier Lenkrollen und der Schiebebügel; doppelt verstrebte Dreibeine mit Gummifüßen, Teleskoprohre, Memory-Klemmen, Galgenneiger, Filze; die Becken als gedrehte dünne gewölbte Schnitte mit Kuppe; Trommeln mit Sperrholzkesseln, Dreifachflansch- oder Holzreifen, 0,5-mm-Fellen (das Resonanzfell der Bassdrum mit seinem geschnittenen Loch), durchgehenden Rohrböckchen, Halterungen, klappbaren Sporen und schiebbaren Standtombeinen."),
            T("Thermal: visible regions are the skins of their body solids by reference; the drums' regions lie in the closed cases and are their own hidden solids with their full surface.", "Thermik: sichtbare Regionen sind die Häute ihrer Körper als Verweis; die Regionen der Trommeln liegen in den geschlossenen Koffern und sind eigene verdeckte Körper mit ihrer ganzen Oberfläche."),
            T("Structure: tubes, legs, shells, case walls, deck, rails and footboards are the members; joints at the hinges, clamps and tilters; one support pad at every contact; mass markers for bones of 2 kg or more.", "Tragwerk: Rohre, Beine, Kessel, Kofferwände, Ladefläche, Rahmen und Trittplatten sind die Stäbe; Gelenke an Scharnieren, Klemmen und Neigern; ein Auflagerpolster an jedem Kontakt; Massenmarken für Knochen ab 2 kg."),
        ]
    if scale == 100:
        return [
            T("Body: as 1:50 without the braces, lower collars, castor top plates, pedal drive strap, hi-hat cup and the separate felts and wing nuts; the 14 to 18 mm lugs of toms, floor tom and snare merged into the shells; snare basket arms and holder arms 15 mm.", "Körper: wie 1:50 ohne Streben, untere Muffen, Rollenkopfplatten, Antriebsband, Hi-Hat-Teller und die einzelnen Filze und Flügelmuttern; die 14- bis 18-mm-Böckchen von Toms, Standtom und Snare in den Kesseln aufgegangen; Korbarme und Halterarme 15 mm."),
            T("Thermal and structure as at 1:50 on the parts of this scale.", "Thermik und Tragwerk wie bei 1:50 an den Teilen dieses Maßstabs."),
        ]
    if scale == 200:
        return [
            T("Body: case walls full height with flush lids and floors, no skirts; every lug omitted; the cymbals as simpler bowed sections; tubes, legs and boards at their true sections, drawn as filled outlines of the same widths.", "Körper: Kofferwände in voller Höhe mit bündigen Deckeln und Böden, ohne Krempen; alle Böckchen entfallen; die Becken als einfachere gewölbte Schnitte; Rohre, Beine und Platten in ihren wahren Querschnitten, als gefüllte Umrisse derselben Breiten gezeichnet."),
            T("Thermal and structure on the parts of this scale; hidden drum regions as their own solids.", "Thermik und Tragwerk an den Teilen dieses Maßstabs; verdeckte Trommelregionen als eigene Körper."),
        ]
    return [
        T("Body: each drum one turned cylinder of its hoop diameter; cases as walls with inner discs; the deck a declared 18 mm sheet on four 75 mm castor blocks; tubes slender at their true sections where they are 20 times as long as thick, else (legs, throne post, short tubes, footboards) at the 75 mm feature; cymbals as 12 mm sheet discs; lugs, brackets, holder, spurs, feet, collars, felts, clutch and basket omitted.", "Körper: jede Trommel ein gedrehter Zylinder ihres Reifendurchmessers; Koffer als Wände mit inneren Scheiben; die Ladefläche eine deklarierte 18-mm-Platte auf vier 75-mm-Rollenblöcken; Rohre schlank in wahrem Querschnitt, wo sie 20-mal so lang wie dick sind, sonst (Beine, Hockerstange, kurze Rohre, Trittplatten) mit dem 75-mm-Merkmal; Becken als 12-mm-Scheiben; Böckchen, Halterungen, Halter, Sporen, Füße, Muffen, Filze, Clutch und Korb entfallen."),
        T("Thermal: the hidden shells, hoops and heads of the drums stay as their own thin solids inside the drum cylinders; structure: holder and spurs keep slender members of their true sections.", "Thermik: die verdeckten Kessel, Reifen und Felle der Trommeln bleiben als eigene dünne Körper in den Trommelzylindern; Tragwerk: Halter und Sporen behalten schlanke Stäbe ihrer wahren Querschnitte."),
    ]


def build():
    """🏗️ The whole asset document."""
    kit = Kit()
    per = {s: kit.parts(s) for s in SCALES}
    bones = [dict(b) for b in kit.bones]
    masses = bone_masses(kit, per[50])
    for b in bones:
        if b["id"] in masses:
            kg, c = masses[b["id"]]
            b["mass"] = {"kg": rnd(kg, 3), "comX": rnd(c[0], 4), "comY": rnd(c[1], 4), "distribution": "point", "source": "authored-estimate", "reference": "kit-catalogue"}
        if b["id"] in SECTIONS:
            m, r, t, title = SECTIONS[b["id"]]
            b["section"] = tube_section(m, r, t, title)
    total = round(sum(b["mass"]["kg"] for b in bones if "mass" in b), 2)
    (x0, x1), (y0, y1), (z0, z1) = world_bbox(kit, per[50])
    dims = {"heightM": rnd(y1, 3), "widthM": rnd(x1 - x0, 3), "depthM": rnd(z1 - z0, 3), "massKg": total}
    ZSHIFT[0] = (z0 + z1) / 2
    anch = anchors(kit)
    height = y1
    reps, vols = {}, {}
    for s in SCALES:
        geo = shapes_for(kit, s, per[s])
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [], "geometry": geo, "decisions": decisions_rep(s, dims)}
        body, thermal, structure = volume(kit, s, per[s], anch, bones, height)
        vols[str(s)] = {"status": "authored", "denominator": s, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions_vol(s)}
    areas = region_areas(per[50])
    regions = []
    for rid, title, mat, kind, t, lc, geom, svf, hidden in REGIONS:
        m = MAT[mat]
        layer = {"id": "skin", "title": title, "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": rnd(t / m[5], 6), "heatCapacityJM2K": rnd(m[4] * m[6] * t, 1), "source": "literature", "reference": "iso-10456" if m[8] == "iso-10456" else "incropera-2018"}
        r = {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": mat, "areaM2": rnd(AREA_OVERRIDE.get(rid, areas.get(rid, 0.01)), 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [layer], "heatCapacityJM2K": layer["heatCapacityJM2K"], "source": "authored-estimate", "reference": "incropera-2018"}
        if hidden:
            r["hidden"] = T("Packed in its closed hard case in the stored default pose; exposed when the kit is set up and played.", "In der eingelagerten Grundpose in ihrem geschlossenen Hartschalenkoffer verpackt; frei, wenn das Schlagzeug aufgebaut und gespielt wird.")
        if rid == "drum-shells":
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        regions.append(r)
    mats = []
    for mid, title, eps, alpha, rho, k, cp, E, ref in MATERIALS:
        m = {"id": mid, "title": title, "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp, "source": "literature" if ref != "authored-estimate" else "authored-estimate"}
        if E:
            m["elasticModulusPa"] = E
        if ref != "authored-estimate":
            m["reference"] = ref
        mats.append(m)
    asset = {
        "id": "used-drum-kit",
        "category": "objects",
        "title": T("Used Five-Piece Drum Kit", "Gebrauchtes Schlagzeug"),
        "summary": T(f"A second-hand five-piece rock drum kit as a used-instrument dealer offers it: a 22 x 18 inch bass drum, 10 and 12 inch rack toms, a 16 inch floor tom and a 14 x 5.5 inch steel snare with faded wrap and dented heads, stored in five round hard cases stacked {dims['heightM']:.2f} m high on a platform trolley beside the collapsed hi-hat, snare, crash and ride stands, throne and pedal; rolled away on the trolley and set up to be played seated, {dims['massKg']:.0f} kg in all.", f"Ein gebrauchtes fünfteiliges Rock-Schlagzeug, wie es ein Gebrauchtinstrumentenhandel anbietet: eine Bassdrum 22 x 18 Zoll, Hängetoms 10 und 12 Zoll, ein Standtom 16 Zoll und eine Stahlsnare 14 x 5,5 Zoll mit verblasster Folie und verbeulten Fellen, in fünf runden Hartschalenkoffern {dims['heightM']:.2f} m hoch auf einem Plattformwagen gestapelt neben den eingeschobenen Hi-Hat-, Snare-, Crash- und Ride-Ständern, Hocker und Fußmaschine; auf dem Wagen weggerollt und zum sitzenden Spiel aufgebaut, zusammen {dims['massKg']:.0f} kg.".replace(".", ",", 0)),
        "tags": ["used", "instrument", "music", "second-hand", "percussion", "drum-kit", "drums", "cymbals", "membranophone", "idiophone", "hard-case", "trolley", "chrome-steel", "birch-plywood", "bronze"],
        "provenance": {"author": "Semio staffage content (used instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(
            "World frame: x ahead from the drummer towards the audience, y up, z to the drummer's left; the throne stands at the origin. Stored (the rest pose): the drums lie in five round polyethylene hard cases, the bass drum flat with the tom holder on its batter head, the spurs folded and the floor tom legs pushed up, stacked on a 1200 x 700 mm platform trolley; the stands stand collapsed where they are used, with the hi-hats, crash and ride left on them. Each drum moves out of its case on three bones: a lift in the plane of the case and a lateral shift (both prismatic), then the drum itself (revolute: the bass drum turns 90 degrees onto its hoops, the toms tilt 10 degrees). Masses are the sum of the parts from their sections and densities (chrome tubes as 1.6 mm walls, castings and frames by their fill).",
            "Weltrahmen: x vom Schlagzeuger zum Publikum, y nach oben, z zur Linken des Schlagzeugers; der Hocker steht im Ursprung. Eingelagert (die Ruhepose): die Trommeln liegen in fünf runden Polyethylen-Hartschalenkoffern, die Bassdrum flach mit dem Tomhalter auf ihrem Schlagfell, die Sporen eingeklappt und die Standtombeine hochgeschoben, gestapelt auf einem Plattformwagen 1200 x 700 mm; die Ständer stehen eingeschoben dort, wo sie gebraucht werden, mit den Hi-Hats, Crash und Ride darauf. Jede Trommel verlässt ihren Koffer über drei Knochen: ein Heben in der Ebene des Koffers und ein seitliches Verschieben (beide prismatisch), dann die Trommel selbst (Drehgelenk: die Bassdrum dreht sich um 90 Grad auf ihre Reifen, die Toms neigen sich um 10 Grad). Massen sind die Summe der Teile aus Querschnitten und Dichten (Chromrohre als 1,6-mm-Wände, Gussteile und Rahmen nach Füllgrad).")},
        "dimensions": dims,
        "materials": mats,
        "rig": {"kind": "mechanical", "root": "rug", "bones": bones},
        "volumes": vols,
        "anchors": anch,
        "representations": reps,
        "symbols": [],
        "poses": poses(anch, kit),
        "clips": clips(anch, kit),
        "thermal": {"environment": {"airTempC": 19, "meanRadiantTempC": 19, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325, "groundTempC": 18}, "referenceEnvironment": "indoor-heated-winter", "passengers": [], "operating": [], "regions": regions, "emitters": [], "assumptions": [
            T("An acoustic instrument: every part is passive (0 W); no element draws power. Indoors without sun; surfaces exchange by convection and radiation with the room.", "Ein akustisches Instrument: jedes Teil ist passiv (0 W); kein Element nimmt Leistung auf. Innen ohne Sonne; die Oberflächen tauschen durch Konvektion und Strahlung mit dem Raum."),
            T("Stored, the drums' shells, heads and hardware exchange only with the air enclosed in their cases (exposed fraction 0); played, every region is exposed. Aged chrome (emissivity 0.16 instead of 0.07 new), tarnished bronze (0.35) and the faded wrap differ from new surfaces.", "Eingelagert tauschen Kessel, Felle und Beschläge der Trommeln nur mit der Luft in ihren Koffern (freier Anteil 0); gespielt ist jede Region frei. Gealtertes Chrom (Emissionsgrad 0,16 statt 0,07 neu), angelaufene Bronze (0,35) und die verblasste Folie unterscheiden sich von neuen Oberflächen."),
            T("The drummer's hands and feet warm sticks and pedals by a few kelvin only; this contact heat is neglected.", "Hände und Füße des Schlagzeugers erwärmen Stöcke und Pedale nur um wenige Kelvin; diese Kontaktwärme wird vernachlässigt."),
        ]},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": support_ids(anch), "assumptions": [
            T("Rigid parts on elastic floor contacts: tripod feet, pedal plates, trolley castors and, when played, the bass drum hoops and spurs and the floor tom legs.", "Starre Teile auf elastischen Bodenkontakten: Dreibeinfüße, Pedalplatten, Lenkrollen des Wagens und, gespielt, die Reifen und Sporen der Bassdrum und die Standtombeine."),
            T("Drumhead tension is an internal self-equilibrated load of each drum (about 2 to 4 kN per head on the hoop) and does not reach the supports; the seated drummer (600 N), the feet on the pedals (40 to 120 N) and the stick strokes (50 to 120 N peak) are loads.", "Die Fellspannung ist eine innere, im Gleichgewicht stehende Last jeder Trommel (etwa 2 bis 4 kN je Fell auf dem Reifen) und erreicht die Auflager nicht; der sitzende Schlagzeuger (600 N), die Füße auf den Pedalen (40 bis 120 N) und die Stockschläge (50 bis 120 N Spitze) sind Lasten."),
        ], "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "groove", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    shift_lateral(asset)
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}, kit, per


def shift_lateral(asset):
    """↔️ Centres the asset on its symmetry plane z = 0: the root bone moves by the lateral centre and the rest-pose front and plan shapes with it (bone-local elevations, members, solids and anchors are unchanged)."""
    zc = ZSHIFT[0]
    asset["rig"]["bones"][0]["z"] = rnd(-zc)
    for rep in asset["representations"].values():
        for s in rep["geometry"]["front"]:
            if s["kind"] == "poly":
                s["points"] = [[rnd(p[0] - zc), p[1]] for p in s["points"]]
        for s in rep["geometry"]["plan"]:
            if s["kind"] == "poly":
                s["points"] = [[p[0], rnd(p[1] - zc)] for p in s["points"]]


AREA_OVERRIDE = {"cases": 3.85, "trolley-deck": 1.23, "stand-chrome": 1.05}
ZSHIFT = [0.0]


if __name__ == "__main__":
    doc, kit, per = build()
    with open("🔣️.json", "w") as fh:
        json.dump(doc, fh, ensure_ascii=False, separators=(",", ":"))
    a = doc["asset"]
    print("[DEBUG] dims", a["dimensions"], "bones", len(a["rig"]["bones"]))
    if "--report" in sys.argv:
        for b in a["rig"]["bones"]:
            if "mass" in b:
                print("[DEBUG] mass", b["id"], b["mass"]["kg"])
        for r in a["thermal"]["regions"]:
            print("[DEBUG] area", r["id"], r["areaM2"])
        for s in SCALES:
            v = a["volumes"][str(s)]
            g = a["representations"][str(s)]["geometry"]
            print("[DEBUG] scale", s, "body", len(v["body"]), "thermal", len(v["thermal"]), "structure", len(v["structure"]), "2D", {k: len(x) for k, x in g.items()})
