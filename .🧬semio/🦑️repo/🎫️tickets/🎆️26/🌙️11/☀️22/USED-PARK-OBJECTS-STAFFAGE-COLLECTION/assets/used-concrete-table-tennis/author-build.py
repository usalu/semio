"""🏓️ Builds the asset document of the used concrete table tennis table from hand-chosen numbers.

Run `python3 author-build.py`; it writes `🔣️.json` next to this file. Every number is chosen by hand for the real object
(ITTF table, catalogue-typical precast outdoor table, see the report `../../reports/used-concrete-table-tennis.md`).
World frame: x ahead along the 2.74 m length, y up, z to the left across the 1.525 m width; ground at y = 0.
"""
import json
import math
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

L, W = 2.74, 1.525
HL, HW = L / 2, W / 2
TOP = 0.76
CONCRETE_TOP = 0.7595
UNDER = 0.70
SPLIT = 0.735
FOOT_X = 0.78
FOOTING_W = 0.40
FOOTING_D = 0.80
FOOTING_L = 1.30
RHO_FOOTING = 2300
FOOT_T = 0.12
FOOT_BASE, FOOT_HEAD = 0.55, 0.45
HAUNCH_Y, HAUNCH_W = 0.62, 0.05
NET_H = 0.1525
NET_T = 0.006
POST_X, POST_T, POST_Y0 = 0.04, 0.010, 0.705
BOLT_AF, BOLT_H, BOLT_X = 0.019, 0.008, 0.025
LIFT_Z = 0.38
DOWEL_Z, DOWEL_R, DOWEL_H = 0.30, 0.010, 0.12
RHO_RC, RHO_STEEL, RHO_COAT = 2500.0, 7850.0, 1300.0
COAT_T = TOP - CONCRETE_TOP
CORNER_R = 0.015


def t(en, de):
    """💬️ A bilingual text."""
    return {"en": en, "de": de}


def r6(v):
    """🎯️ Rounds a number to micrometres for stable output."""
    return round(v + 0.0, 6)


def pts(points):
    """📍️ Rounds a point list."""
    return [[r6(a), r6(b)] for a, b in points]


def area(poly):
    """📐️ Shoelace area of a simple polygon."""
    return abs(sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1] for i in range(len(poly)))) / 2


def perimeter(poly):
    """📏️ Perimeter of a closed polygon."""
    return sum(math.dist(poly[i], poly[(i + 1) % len(poly)]) for i in range(len(poly)))


def arc(cx, cz, r, a0, a1, n=4):
    """🌙️ Points of a corner arc from angle a0 to a1 (degrees), ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cz + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


CHIPS = {
    "c3": [(-0.45, -HW), (-0.44, -HW + 0.022), (-0.37, -HW + 0.018), (-0.36, -HW)],
    "c2": [(1.31, -HW), (1.335, -0.738), (HL, -0.712)],
    "c1": [(HL, 0.20), (HL - 0.025, 0.21), (HL - 0.030, 0.27), (HL, 0.29)],
    "c5": [(-1.325, HW), (-1.345, 0.74), (-HL, 0.7225)],
    "c4": [(-HL, -0.27), (-HL + 0.025, -0.28), (-HL + 0.028, -0.335), (-HL, -0.35)],
}


def plate_outline(chips, rounded=True):
    """🟫️ The plan outline (x, z) of the plate, counter-clockwise, with the named arris chips cut into it."""
    r = CORNER_R if rounded else 0.0
    out = []
    if "c3" in chips:
        out += [(-HL + r, -HW)] + CHIPS["c3"]
    else:
        out += [(-HL + r, -HW)]
    if "c2" in chips:
        out += CHIPS["c2"]
    else:
        out += arc(HL - r, -HW + r, r, -90, 0) if r else [(HL, -HW)]
    if "c1" in chips:
        out += CHIPS["c1"]
    if "c5" in chips:
        out += (arc(HL - r, HW - r, r, 0, 90) if r else [(HL, HW)]) + CHIPS["c5"]
    else:
        out += (arc(HL - r, HW - r, r, 0, 90) if r else [(HL, HW)]) + (arc(-HL + r, HW - r, r, 90, 180) if r else [(-HL, HW)])
    if "c4" in chips:
        out += CHIPS["c4"]
    out += arc(-HL + r, -HW + r, r, 180, 270) if r else [(-HL, -HW)]
    clean = []
    for p in out:
        if not clean or math.dist(clean[-1], p) > 1e-9:
            clean.append(p)
    if math.dist(clean[0], clean[-1]) < 1e-9:
        clean.pop()
    return clean


def foot_profile(spall):
    """🦶️ Front profile (z, y) of a foot slab: trapezoid 1.10 m at the base, 0.90 m under the plate, optionally the mower spall at the near base corner."""
    if spall:
        return [(-0.505, 0.0), (FOOT_BASE, 0.0), (FOOT_HEAD, UNDER), (-FOOT_HEAD, UNDER), (-0.545, 0.035), (-0.515, 0.028)]
    return [(-FOOT_BASE, 0.0), (FOOT_BASE, 0.0), (FOOT_HEAD, UNDER), (-FOOT_HEAD, UNDER)]


def foot_profile_top(top):
    """🦶️ Front profile of a foot ending at the height `top` (1:500, where the plate is drawn 75 mm thick)."""
    half = FOOT_BASE - (FOOT_BASE - FOOT_HEAD) * top / UNDER
    return [(-FOOT_BASE, 0.0), (FOOT_BASE, 0.0), (half, top), (-half, top)]


def circle(cx, cy, r, n=12):
    """⭕️ A polygon approximating a circle."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


HOLE_R = 0.0125
HOLES = [(-0.725 + 0.05 * i, u) for u in (0.050, 0.1025) for i in range(30)]


def net_outline(dent):
    """🕸️ Face-on outline (z, height above the playing surface) of the perforated net plate, optionally with the bent-down top edge."""
    top = [(HW, NET_H)]
    if dent:
        top += [(0.40, NET_H), (0.355, 0.146), (0.31, 0.1405), (0.265, 0.146), (0.22, NET_H)]
    return [(-HW, 0.0), (HW, 0.0)] + top + [(-HW, NET_H)]


def hexagon(cx, cy, af):
    """⬡️ A hexagon with the across-flats width `af` (flats top and bottom)."""
    rr = af / math.sqrt(3)
    return [(cx + rr * math.cos(math.radians(60 * k)), cy + rr * math.sin(math.radians(60 * k))) for k in range(6)]


def el_table(x, y):
    """↔️ World (x, y) to the elevation frame of the table bone."""
    return (x + HL, y - 0.73)


def el_foot(fx):
    """↕️ World (x, y) to the elevation frame of the foot bone standing at x = fx."""
    return lambda x, y: (UNDER - y, x - fx)


def el_net(x, y):
    """⬆️ World (x, y) to the elevation frame of the net bone."""
    return (y - TOP, -x)


def tr(f, points):
    """🔁️ Maps a world point list through a frame map."""
    return pts([f(a, b) for a, b in points])


def rect_pts(x0, y0, x1, y1):
    """▭️ Corner points of an axis-aligned rectangle."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def shape(id, bone, role, z, filled, tone, geom, **extra):
    """✏️ One authored 2D shape."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    s.update(geom)
    s.update(extra)
    return s


def P(points, closed=True):
    """🔷️ A poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def R(x, y, w, h):
    """⬛️ A rect geometry."""
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def E(cx, cy, rx, ry):
    """⚪️ An ellipse geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry)}


def D(rings):
    """〰️ A path geometry of closed rings (even-odd)."""
    return {"kind": "path", "d": " ".join("M " + " L ".join(f"{r6(a)} {r6(b)}" for a, b in ring) + " Z" for ring in rings)}


def ground_line(view, depth):
    """🟤️ The ground surface with the hollows the players have worn into the water-bound gravel, as ground strokes on the ground bone."""
    if view == "elevation":
        line = [(-2.45, 0), (-2.40, 0), (-2.25, -0.025), (-1.95, -0.035), (-1.70, -0.02), (-1.55, 0), (1.55, 0), (1.70, -0.02), (1.95, -0.035), (2.25, -0.025), (2.40, 0), (2.45, 0)]
    else:
        line = [(-0.95, 0), (0.95, 0)]
    out = [shape("ground-surface", "ground", "ground", 0.1, False, 0.5, P(line, False))]
    return out


def footing_shapes(view, scale):
    """🧱️ The two strip footings cut below the ground line, tops flush with the gravel: filled bodies on their own bones that stay behind when the table is lifted; with the ground line at the two coarse scales, where the worn hollows are below the reading."""
    extra = {"region": "footings", "material": "concrete-footing", **({"hatch": "concrete"} if scale in (50, 100) else {})}
    hw, hl = FOOTING_W / 2, FOOTING_L / 2
    if view == "elevation":
        out = [shape(f"footing-{s}", f"footing-{s}", "body", 1, True, 0.78, P(rect_pts(0, -hw, FOOTING_D, hw)), **extra) for s in ("a", "b")]
        line = [(0, FOOT_X - 2.45), (0, FOOT_X + 2.45)]
    else:
        out = [shape("footing-b", "footing-b", "body", 1, True, 0.78, P(rect_pts(-hl, -FOOTING_D, hl, 0)), **extra)]
        line = [(-0.95, 0), (0.95, 0)]
    if scale in (200, 500):
        out.append(shape("ground-surface", "footing-a" if view == "elevation" else "footing-b", "ground", 0.1, False, 0.5, P(line, False)))
    return out


LINES50 = {
    "side-near": [(-HL, -1.12), (-0.98, -0.455), (-0.355, -0.003), (0.003, 0.55), (0.71, 1.305)],
    "side-far": [(-1.32, -0.30), (-0.12, -0.003), (0.003, 1.08), (1.22, HL)],
    "end-b": [(-0.705, -0.30), (0.12, 0.195), (0.295, 0.7425)],
    "end-a": [(-0.7425, -0.355), (-0.265, -0.05), (0.28, 0.715)],
    "centre": [(-1.35, -0.62), (-0.45, -0.003), (0.003, 0.90), (1.05, 1.35)],
}
LINES100 = {
    "side-near": [(-HL, -0.003), (0.003, 1.305)],
    "side-far": [(-HL, -0.003), (0.003, HL)],
    "end-b": [(-0.705, 0.195), (0.295, 0.7425)],
    "end-a": [(-0.7425, -0.355), (-0.265, 0.7425)],
}


def line_rects(lines):
    """🏳️ The painted lines as plan rectangles (x0, z0, x1, z1) per segment."""
    out = []
    for key, segs in lines.items():
        for i, (a, b) in enumerate(segs):
            if key == "side-near":
                out.append((f"line-{key}-{i}", a, -HW, b, -HW + 0.02))
            elif key == "side-far":
                out.append((f"line-{key}-{i}", a, HW - 0.02, b, HW))
            elif key == "end-b":
                out.append((f"line-{key}-{i}", HL - 0.02, a, HL, b))
            elif key == "end-a":
                out.append((f"line-{key}-{i}", -HL, a, -HL + 0.02, b))
            else:
                out.append((f"line-{key}-{i}", a, -0.0015, b, 0.0015))
    return out


WORN = [
    ("worn-end-b", [(1.05, -0.18), (1.30, -0.22), (1.335, 0.05), (1.26, 0.16), (1.08, 0.12)]),
    ("worn-end-a", [(-1.33, -0.10), (-1.12, -0.16), (-1.02, 0.04), (-1.10, 0.22), (-1.31, 0.18)]),
    ("worn-side", [(-0.62, 0.60), (-0.30, 0.58), (-0.26, 0.725), (-0.64, 0.73)]),
]
GUM = [(0.42, -0.31, 0.011), (0.66, 0.18, 0.009), (-0.55, 0.12, 0.012), (-0.92, -0.48, 0.010), (1.12, 0.52, 0.010), (-0.21, -0.62, 0.009)]


def materials():
    """🧱️ The materials with weathered surface properties."""
    return [
        {"id": "concrete-precast", "title": t("Precast reinforced concrete C35/45, weathered", "Fertigteil-Stahlbeton C35/45, verwittert"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": RHO_RC, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "elasticModulusPa": 34e9, "strengthPa": 35e6, "source": "literature", "reference": "en-1992-1-1"},
        {"id": "coating-acrylic", "title": t("Acrylic sports coating, dark green, faded and chalked", "Acryl-Sportbeschichtung, dunkelgrün, ausgeblichen und kreidend"), "emissivity": 0.9, "solarAbsorptivity": 0.78, "densityKgM3": RHO_COAT, "conductivityWmK": 0.2, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "paint-white", "title": t("White line paint, worn", "Weiße Linienfarbe, abgenutzt"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1400, "conductivityWmK": 0.2, "specificHeatJKgK": 1500, "source": "authored-estimate", "reference": "incropera-2018"},
        {"id": "steel-net", "title": t("Galvanised and powder-coated steel, faded, rusting at the welds", "Verzinkter und pulverbeschichteter Stahl, ausgeblichen, an den Schweißnähten rostend"), "emissivity": 0.88, "solarAbsorptivity": 0.8, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 235e6, "source": "literature", "reference": "en-1993-1-1"},
        {"id": "concrete-footing", "title": t("Site-cast plain concrete C20/25 of the strip footings, earth-damp", "Ortbeton C20/25 der Streifenfundamente, unbewehrt, erdfeucht"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": RHO_FOOTING, "conductivityWmK": 1.65, "specificHeatJKgK": 1000, "elasticModulusPa": 30e9, "strengthPa": 20e6, "source": "literature", "reference": "en-1992-1-1"},
        {"id": "stainless-dowel", "title": t("Stainless steel dowel 1.4301", "Edelstahldorn 1.4301"), "emissivity": 0.35, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 15, "specificHeatJKgK": 500, "elasticModulusPa": 200e9, "strengthPa": 230e6, "source": "literature", "reference": "en-1993-1-1"},
        {"id": "epdm-cap", "title": t("EPDM recess plug, weathered", "EPDM-Aussparungsstopfen, verwittert"), "emissivity": 0.9, "solarAbsorptivity": 0.9, "densityKgM3": 1150, "conductivityWmK": 0.25, "specificHeatJKgK": 2000, "source": "authored-estimate", "reference": "iso-10456-2007"},
    ]


def masses():
    """⚖️ Masses from section x density, per bone, with the volumes they come from."""
    lower = area(plate_outline([])) * (SPLIT - UNDER)
    upper = area(plate_outline(list(CHIPS))) * (CONCRETE_TOP - SPLIT)
    coat = area(plate_outline(list(CHIPS))) * COAT_T
    lines = sum((x1 - x0) * (z1 - z0) for _, x0, z0, x1, z1 in line_rects(LINES50)) * 0.0003
    caps = 4 * math.pi * 0.03 ** 2 * 0.002
    table = (lower + upper) * RHO_RC + coat * RHO_COAT + lines * 1400 + caps * 1150
    foot = area(foot_profile(False)) * FOOT_T
    haunch = (((FOOT_T + FOOT_T + 2 * HAUNCH_W) / 2 * (UNDER - HAUNCH_Y)) - FOOT_T * (UNDER - HAUNCH_Y)) * 2 * FOOT_HEAD
    foot_a = (foot + haunch) * RHO_RC
    foot_b = (area(foot_profile(True)) * FOOT_T + haunch) * RHO_RC
    netplate = (W * NET_H - len(HOLES) * math.pi * HOLE_R ** 2 - 0.5 * 0.18 * 0.012) * NET_T
    posts = 2 * (2 * POST_X) * POST_T * (TOP + NET_H - POST_Y0)
    bolts = 4 * (math.sqrt(3) / 2 * BOLT_AF ** 2) * BOLT_H
    net = (netplate + posts + bolts) * RHO_STEEL
    return {"table": table, "foot-a": foot_a, "foot-b": foot_b, "net": net, "volumes": {"plate-lower": lower, "plate-upper": upper, "coating": coat, "foot": foot, "haunch": haunch, "netplate": netplate, "posts": posts}}


M = masses()
MASS_TOTAL = M["table"] + M["foot-a"] + M["foot-b"] + M["net"]


def foot_com():
    """🎯️ Centre of mass of a foot along its bone (from the plate underside down)."""
    a, b, h = 2 * FOOT_HEAD, 2 * FOOT_BASE, UNDER
    trap = h * (a + 2 * b) / (3 * (a + b))
    vf = M["volumes"]["foot"]
    vh = M["volumes"]["haunch"]
    return (vf * trap + vh * (UNDER - HAUNCH_Y) / 3) / (vf + vh)


def rig():
    """🦴️ Ground reference, the monolithic table (plate bone with its prismatic lift), two feet and the net."""
    return {
        "kind": "structural",
        "root": "ground",
        "bones": [
            {"id": "ground", "parent": None, "title": t("Ground reference on the footing tops (stays behind with the footings and the dowels)", "Bodenbezug auf den Fundamentoberkanten (bleibt mit den Fundamenten und den Dornen zurück)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
            *({"id": f"footing-{k}", "parent": "ground", "title": t(f"Strip footing under foot {k.upper()}, from its top flush with the gravel down to its base; massless for the solver because its weight goes straight into the soil", f"Streifenfundament unter Fuß {k.upper()}, von der Oberkante bündig mit der Decke bis zur Sohle; für den Löser masselos, weil sein Gewicht direkt in den Boden geht"), "x": fx, "y": 0, "angleDeg": -90, "lengthM": FOOTING_D, "joint": {"type": "fixed"}} for k, fx in (("a", -FOOT_X), ("b", FOOT_X))),
            {"id": "table", "parent": "ground", "title": t("Precast table plate, from the end at x = -1.37 m along its mid-plane; lifted vertically by the crane", "Fertigteil-Tischplatte, vom Ende bei x = -1,37 m entlang ihrer Mittelebene; vom Kran senkrecht gehoben"), "x": -HL, "y": 0.73, "angleDeg": 0, "lengthM": L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}, "mass": {"kg": r6(round(M["table"], 1)), "comX": HL, "comY": 0.0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1991-1-1"}, "section": {"areaM2": r6(W * 0.06), "secondMomentM4": float(f"{W * 0.06 ** 3 / 12:.6e}"), "extremeFibreM": 0.03, "material": "concrete-precast", "title": t("Plate 1525 x 60 mm, C35/45", "Platte 1525 x 60 mm, C35/45")}},
            {"id": "foot-a", "parent": "table", "title": t("Foot slab at x = -0.78 m, cast with the plate, from the plate underside down to the footing", "Fußscheibe bei x = -0,78 m, mit der Platte gegossen, von der Plattenunterseite bis zum Fundament"), "x": HL - FOOT_X, "y": -0.03, "angleDeg": -90, "lengthM": UNDER, "joint": {"type": "fixed"}, "mass": {"kg": r6(round(M["foot-a"], 1)), "comX": r6(foot_com()), "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "en-1991-1-1"}, "section": {"areaM2": 0.12, "secondMomentM4": float(f"{1.0 * FOOT_T ** 3 / 12:.6e}"), "extremeFibreM": 0.06, "material": "concrete-precast", "title": t("Foot slab 120 x 1000 mm (mean), C35/45", "Fußscheibe 120 x 1000 mm (Mittel), C35/45")}},
            {"id": "foot-b", "parent": "table", "title": t("Foot slab at x = +0.78 m, cast with the plate, spalled at its near base corner", "Fußscheibe bei x = +0,78 m, mit der Platte gegossen, an der nahen Fußecke abgeplatzt"), "x": HL + FOOT_X, "y": -0.03, "angleDeg": -90, "lengthM": UNDER, "joint": {"type": "fixed"}, "mass": {"kg": r6(round(M["foot-b"], 1)), "comX": r6(foot_com()), "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "en-1991-1-1"}, "section": {"areaM2": 0.12, "secondMomentM4": float(f"{1.0 * FOOT_T ** 3 / 12:.6e}"), "extremeFibreM": 0.06, "material": "concrete-precast", "title": t("Foot slab 120 x 1000 mm (mean), C35/45", "Fußscheibe 120 x 1000 mm (Mittel), C35/45")}},
            {"id": "net", "parent": "table", "title": t("Perforated steel net on the centre line, welded to two posts bolted to the plate edges", "Gelochtes Stahlnetz auf der Mittellinie, an zwei an die Plattenkanten geschraubte Pfosten geschweißt"), "x": HL, "y": 0.03, "angleDeg": 90, "lengthM": NET_H, "joint": {"type": "fixed"}, "mass": {"kg": r6(round(M["net"], 2)), "comX": 0.07, "comY": 0.0, "distribution": "point", "source": "authored-estimate", "reference": "din-24041"}, "section": {"areaM2": r6(W * NET_T), "secondMomentM4": float(f"{W * NET_T ** 3 / 12:.6e}"), "extremeFibreM": NET_T / 2, "material": "steel-net", "title": t("Perforated plate 1525 x 6 mm", "Lochblech 1525 x 6 mm")}},
        ],
    }


def anchors():
    """📍️ Foot contacts on the footings, the four cast-in lifting anchors and the lean points of the players."""
    out = []
    for f in ("a", "b"):
        for side, z in (("l", 0.275), ("r", -0.275)):
            out.append({"id": f"foot-{f}-{side}", "bone": f"foot-{f}", "x": UNDER, "y": 0.0, "z": z, "kind": "contact", "title": t(f"Foot {f.upper()}, {'left' if side == 'l' else 'right'} half of the base on the mortar bed of its strip footing", f"Fuß {f.upper()}, {'linke' if side == 'l' else 'rechte'} Hälfte der Aufstandsfläche auf dem Mörtelbett seines Streifenfundaments"), "support": "contact", "normalDeg": 90, "footprintLengthM": FOOT_T, "footprintWidthM": 0.55, "frictionCoefficient": 0.7})
    for f, x in (("a", HL - FOOT_X), ("b", HL + FOOT_X)):
        for side, z in (("l", LIFT_Z), ("r", -LIFT_Z)):
            out.append({"id": f"lift-{f}-{side}", "bone": "table", "x": x, "y": 0.03, "z": z, "kind": "support", "title": t(f"Cast-in spherical-head lifting anchor over foot {f.upper()}, {'left' if side == 'l' else 'right'}: one leg of the four-leg chain sling", f"Einbetonierter Kugelkopfanker über Fuß {f.upper()}, {'links' if side == 'l' else 'rechts'}: ein Strang des Vierstrang-Kettengehänges"), "support": "pin", "footprintLengthM": 0.06, "footprintWidthM": 0.06})
    out.append({"id": "lean-b", "bone": "table", "x": L, "y": 0.03, "z": 0.25, "kind": "load", "title": t("Player A leans on end B with both hands", "Spieler A stützt sich mit beiden Händen auf Ende B")})
    out.append({"id": "lean-a", "bone": "table", "x": 0.0, "y": 0.03, "z": -0.2, "kind": "load", "title": t("Player B leans on end A", "Spieler B lehnt sich auf Ende A")})
    out.append({"id": "net-top", "bone": "net", "x": NET_H, "y": 0.0, "kind": "load", "title": t("Top edge of the net", "Oberkante des Netzes")})
    return out


FEET = ["foot-a-l", "foot-a-r", "foot-b-l", "foot-b-r"]
LIFTS = ["lift-a-l", "lift-a-r", "lift-b-l", "lift-b-r"]
REGIONS = ["playing-coating", "plate-concrete", "feet-concrete", "net-steel", "footings"]


def binding(coat_sun, plate_sun, feet_sun, net_sun, feet_contact=True, hands=0.0):
    """🌞️ The thermal binding of every region in a pose."""
    coat = {"exposedFraction": round(1 - hands, 3), "sunlitFraction": coat_sun}
    if hands > 0:
        coat["contact"] = {"fraction": hands, "tempC": 32}
    feet = {"exposedFraction": 0.93 if feet_contact else 1.0, "sunlitFraction": feet_sun}
    if feet_contact:
        feet["contact"] = {"fraction": 0.07, "tempC": 22}
    tops = 2 * FOOTING_W * FOOTING_L / AREA["footings"]
    bases = round(0.07 * AREA["feet-concrete"] / AREA["footings"], 3) if feet_contact else 0.0
    footings = {"exposedFraction": round(tops - bases, 3), "sunlitFraction": 0.03 if feet_contact else 0.1, "contact": {"fraction": round(1 - tops + bases, 3), "tempC": 16}}
    return {"regions": {"playing-coating": coat, "plate-concrete": {"exposedFraction": 0.95 if feet_contact else 1.0, "sunlitFraction": plate_sun}, "feet-concrete": feet, "net-steel": {"exposedFraction": 1.0, "sunlitFraction": net_sun}, "footings": footings}, "emitters": {}}


def poses():
    """🧍️ In service, lifted by the crane, two players at the table, and the evening-shade key of the thermal clip."""
    return [
        {"id": "in-service", "title": t("In service on its strip footings", "Im Einsatz auf seinen Streifenfundamenten"), "description": t("The table stands doweled on the mortar beds of its two strip footings, flush with the water-bound gravel surface, summer afternoon sun on the faded green coating; only its own weight acts.", "Die Platte steht verdübelt auf den Mörtelbetten ihrer zwei Streifenfundamente, bündig mit der wassergebundenen Decke, Sommernachmittagssonne auf der ausgeblichenen grünen Beschichtung; nur das Eigengewicht wirkt."), "bones": {}, "ground": "authored", "contacts": FEET, "thermal": binding(1.0, 0.03, 0.15, 0.45), "loads": []},
        {"id": "maintenance", "title": t("Lifted by the depot crane", "Vom Bauhofkran gehoben"), "description": t("The municipal depot lifts the 1.07 t table on a four-leg chain sling hooked into the four cast-in spherical-head anchors over the feet, 0.40 m clear of its footings; the stainless dowels and the footings stay behind in the ground. The lifting forces are the four pin supports at the anchors.", "Der Bauhof hebt die 1,07 t schwere Platte an einem Vierstrang-Kettengehänge, eingehängt in die vier einbetonierten Kugelkopfanker über den Füßen, 0,40 m über ihre Fundamente; die Edelstahldorne und die Fundamente bleiben im Boden. Die Hubkräfte sind die vier Gelenklager an den Ankern."), "bones": {"table": {"dy": 0.40}}, "ground": "authored", "contacts": LIFTS, "thermal": binding(1.0, 0.05, 0.2, 0.45, feet_contact=False), "loads": []},
        {"id": "in-use", "title": t("Two players at the table", "Zwei Spieler an der Platte"), "description": t("A rally between two players: player A leans with both hands on end B while waiting for the ball (300 N down, 40 N pushing), player B rests a hand on end A (150 N); their shadows fall on part of the coating.", "Ein Ballwechsel zweier Spieler: Spieler A stützt sich beim Warten auf den Ball mit beiden Händen auf Ende B (300 N nach unten, 40 N Schub), Spieler B legt eine Hand auf Ende A (150 N); ihre Schatten fallen auf einen Teil der Beschichtung."), "bones": {}, "ground": "authored", "contacts": FEET, "thermal": binding(0.85, 0.03, 0.12, 0.4, hands=0.01), "loads": [
            {"id": "lean-player-a", "kind": "occupant", "title": t("Player A leaning on end B with both hands", "Spieler A stützt sich mit beiden Händen auf Ende B"), "anchor": "lean-b", "fxN": -40, "fyN": -300, "source": "authored-estimate", "reference": "en-1991-1-1"},
            {"id": "lean-player-b", "kind": "occupant", "title": t("Player B resting a hand on end A", "Spieler B legt eine Hand auf Ende A"), "anchor": "lean-a", "fxN": 0, "fyN": -150, "source": "authored-estimate", "reference": "en-1991-1-1"},
        ]},
        {"id": "evening-shade", "key": True, "title": t("Evening shade", "Abendschatten"), "description": t("The shadow of the park trees has reached the table; no direct sun.", "Der Schatten der Parkbäume hat die Platte erreicht; keine direkte Sonne."), "bones": {}, "ground": "authored", "contacts": FEET, "thermal": binding(0.0, 0.0, 0.0, 0.0), "loads": []},
    ]


def clips():
    """🎞️ The crane lift of the depot, a rally with the players leaning on the ends, and a summer afternoon until the tree shadow arrives."""
    return [
        {"id": "crane-lift", "title": t("Lifted out by the depot crane", "Vom Bauhofkran ausgehoben"), "description": t("The sling is tensioned, the crane takes the load, the table comes off its dowels and rises 0.40 m; footings and dowels stay in the ground.", "Das Gehänge wird gespannt, der Kran nimmt die Last, die Platte löst sich von ihren Dornen und steigt 0,40 m; Fundamente und Dorne bleiben im Boden."), "durationS": 40, "fps": 10, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "in-service", "ease": "linear"}, {"t": 8, "pose": "in-service", "ease": "smooth"}, {"t": 32, "pose": "maintenance", "ease": "linear"}, {"t": 40, "pose": "maintenance", "ease": "linear"}], "contacts": [*[{"anchor": a, "fromS": 0, "toS": 8} for a in FEET], *[{"anchor": a, "fromS": 6, "toS": 40} for a in LIFTS]], "phases": [{"id": "rigging", "fromS": 0, "toS": 6, "title": t("Sling hooked in and tensioned", "Gehänge eingehängt und gespannt")}, {"id": "taking-load", "fromS": 6, "toS": 8, "title": t("Crane takes the load", "Kran nimmt die Last")}, {"id": "lifting", "fromS": 8, "toS": 32, "title": t("Lifting off the dowels", "Abheben von den Dornen")}, {"id": "hanging", "fromS": 32, "toS": 40, "title": t("Hanging clear of the footings", "Frei über den Fundamenten hängend")}], "events": [{"t": 6, "kind": "load-on", "title": t("Crane takes the load at the four anchors", "Kran nimmt die Last an den vier Ankern")}, {"t": 8, "kind": "contact-off", "title": t("Feet leave the mortar beds and the dowels", "Füße verlassen die Mörtelbetten und die Dorne")}, {"t": 32, "kind": "motion", "title": t("Hook stops 0.40 m up", "Haken hält 0,40 m oben")}], "tracks": []},
        {"id": "rally", "title": t("A rally with the players leaning on the ends", "Ein Ballwechsel, die Spieler stützen sich auf die Enden"), "description": t("Twelve seconds of play: between strokes player A leans on end B and player B rests a hand on end A; the lean loads come and go, the table does not move.", "Zwölf Sekunden Spiel: zwischen den Schlägen stützt sich Spieler A auf Ende B und Spieler B legt eine Hand auf Ende A; die Stützlasten kommen und gehen, die Platte bewegt sich nicht."), "durationS": 12, "fps": 25, "loop": True, "rootMotion": "none", "keys": [{"t": 0, "pose": "in-use", "ease": "linear"}, {"t": 12, "pose": "in-use", "ease": "linear"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 12} for a in FEET], "phases": [{"id": "serve", "fromS": 0, "toS": 3, "title": t("Serve and return", "Aufschlag und Rückschlag")}, {"id": "lean-a", "fromS": 3, "toS": 6, "title": t("Player A leans on end B", "Spieler A stützt sich auf Ende B")}, {"id": "rally-on", "fromS": 6, "toS": 9, "title": t("Rally", "Ballwechsel")}, {"id": "lean-b", "fromS": 9, "toS": 12, "title": t("Player B leans on end A", "Spieler B lehnt sich auf Ende A")}], "events": [{"t": 3, "kind": "load-on", "title": t("Player A leans on end B", "Spieler A stützt sich auf Ende B"), "target": "lean-player-a"}, {"t": 6, "kind": "load-off", "title": t("Player A straightens up", "Spieler A richtet sich auf"), "target": "lean-player-a"}, {"t": 9, "kind": "load-on", "title": t("Player B leans on end A", "Spieler B lehnt sich auf Ende A"), "target": "lean-player-b"}, {"t": 11.5, "kind": "load-off", "title": t("Player B lets go", "Spieler B lässt los"), "target": "lean-player-b"}], "tracks": [{"target": "load:lean-player-a", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 3, "v": 0}, {"t": 3.5, "v": 1}, {"t": 5.5, "v": 1}, {"t": 6, "v": 0}, {"t": 12, "v": 0}]}, {"target": "load:lean-player-b", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 9, "v": 0}, {"t": 9.5, "v": 1}, {"t": 11, "v": 1}, {"t": 11.5, "v": 0}, {"t": 12, "v": 0}]}]},
        {"id": "summer-afternoon", "title": t("Summer afternoon until the tree shadow arrives", "Sommernachmittag, bis der Baumschatten kommt"), "description": t("Six hours: the faded dark-green coating and the steel net heat in full sun, the 60 mm plate follows slowly; at four hours the shadow of the park trees reaches the table and the surfaces cool according to their heat capacity.", "Sechs Stunden: die ausgeblichene dunkelgrüne Beschichtung und das Stahlnetz erwärmen sich in voller Sonne, die 60-mm-Platte folgt langsam; nach vier Stunden erreicht der Schatten der Parkbäume die Platte, und die Oberflächen kühlen gemäß ihrer Wärmekapazität ab."), "durationS": 21600, "fps": 0.01, "loop": False, "rootMotion": "none", "keys": [{"t": 0, "pose": "in-service", "ease": "step"}, {"t": 14400, "pose": "in-service", "ease": "linear"}, {"t": 18000, "pose": "evening-shade", "ease": "step"}, {"t": 21600, "pose": "evening-shade", "ease": "step"}], "contacts": [{"anchor": a, "fromS": 0, "toS": 21600} for a in FEET], "phases": [{"id": "full-sun", "fromS": 0, "toS": 14400, "title": t("Full sun", "Volle Sonne")}, {"id": "shadow-arrives", "fromS": 14400, "toS": 18000, "title": t("Tree shadow moves over the table", "Baumschatten zieht über die Platte")}, {"id": "shade", "fromS": 18000, "toS": 21600, "title": t("In shade", "Im Schatten")}], "events": [{"t": 14400, "kind": "thermal", "title": t("Tree shadow reaches end A", "Baumschatten erreicht Ende A")}, {"t": 18000, "kind": "thermal", "title": t("Whole table in shade", "Ganze Platte im Schatten")}], "tracks": [{"target": "region:playing-coating:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 1}, {"t": 14400, "v": 1}, {"t": 18000, "v": 0}, {"t": 21600, "v": 0}]}, {"target": "region:net-steel:sunlitFraction", "ease": "linear", "keys": [{"t": 0, "v": 0.45}, {"t": 14400, "v": 0.45}, {"t": 18000, "v": 0}, {"t": 21600, "v": 0}]}]},
    ]


AREA = {}


def thermal():
    """🌡️ Four passive regions: the coated playing surface, the bare concrete of the plate, the feet and the steel net."""
    lay = lambda id, en, de, kind, mat, th, lam, cap, src="literature", ref="iso-10456-2007": {"id": id, "title": t(en, de), "kind": kind, "material": mat, "thicknessM": th, "resistanceM2KW": r6(th / lam), "heatCapacityJM2K": round(cap), "source": src, "reference": ref}
    return {
        "environment": {"airTempC": 28, "meanRadiantTempC": 30, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325, "skyTempC": 14, "groundTempC": 38},
        "referenceEnvironment": "summer-hot-sun",
        "operating": [],
        "passengers": [],
        "regions": [
            {"id": "playing-coating", "title": t("Playing surface: faded green acrylic coating on the plate", "Spielfläche: ausgeblichene grüne Acrylbeschichtung auf der Platte"), "kind": "surface", "model": "passive", "material": "coating-acrylic", "areaM2": AREA["playing-coating"], "characteristicLengthM": W, "geometry": "horizontal-plate-up", "skyViewFactor": 1.0, "layers": [lay("coating", "Acrylic coating 0.5 mm, chalked", "Acrylbeschichtung 0,5 mm, kreidend", "polymer", "coating-acrylic", COAT_T, 0.2, RHO_COAT * 1500 * COAT_T, "authored-estimate", "incropera-2018")], "heatCapacityJM2K": round(RHO_COAT * 1500 * COAT_T + RHO_RC * 1000 * 0.03), "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "incropera-2018"},
            {"id": "plate-concrete", "title": t("Bare concrete of the plate: underside, edges and chipped arrises", "Blanker Beton der Platte: Unterseite, Kanten und abgeplatzte Kanten"), "kind": "surface", "model": "passive", "material": "concrete-precast", "areaM2": AREA["plate-concrete"], "characteristicLengthM": W, "geometry": "horizontal-plate-down", "skyViewFactor": 0.05, "layers": [lay("slab", "Concrete plate 59.5 mm", "Betonplatte 59,5 mm", "masonry", "concrete-precast", CONCRETE_TOP - UNDER, 2.3, RHO_RC * 1000 * (CONCRETE_TOP - UNDER))], "heatCapacityJM2K": round(RHO_RC * 1000 * 0.0295), "source": "authored-estimate", "reference": "iso-10456-2007"},
            {"id": "feet-concrete", "title": t("Concrete foot slabs with haunches, algae on the shaded faces", "Betonfußscheiben mit Vouten, Algen auf den Schattenseiten"), "kind": "surface", "model": "passive", "material": "concrete-precast", "areaM2": AREA["feet-concrete"], "characteristicLengthM": UNDER, "geometry": "vertical-plate", "skyViewFactor": 0.15, "layers": [lay("foot-half", "Concrete to the mid-plane of the 120 mm slab", "Beton bis zur Mittelebene der 120-mm-Scheibe", "masonry", "concrete-precast", FOOT_T / 2, 2.3, RHO_RC * 1000 * FOOT_T / 2)], "heatCapacityJM2K": round((M["volumes"]["foot"] + M["volumes"]["haunch"]) * RHO_RC * 1000 / (AREA["feet-concrete"] / 2)), "source": "authored-estimate", "reference": "iso-10456-2007"},
            {"id": "net-steel", "title": t("Perforated steel net, posts and bolts", "Gelochtes Stahlnetz, Pfosten und Schrauben"), "kind": "surface", "model": "passive", "material": "steel-net", "areaM2": AREA["net-steel"], "characteristicLengthM": NET_H, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [lay("net-plate", "Steel plate 6 mm, zinc and powder coat", "Stahlblech 6 mm, Zink und Pulverlack", "metal", "steel-net", NET_T, 50, RHO_STEEL * 460 * NET_T)], "heatCapacityJM2K": round(RHO_STEEL * 460 * NET_T / 2), "source": "authored-estimate", "reference": "incropera-2018"},
            {"id": "footings", "title": t("Strip footings: tops flush with the gravel, the rest in the soil", "Streifenfundamente: Oberkanten bündig mit der Decke, der Rest im Boden"), "kind": "surface", "model": "passive", "material": "concrete-footing", "areaM2": AREA["footings"], "characteristicLengthM": FOOTING_W, "geometry": "horizontal-plate-up", "skyViewFactor": 0.05, "layers": [lay("footing-half", "Plain concrete to the mid-plane of the 400 mm wide footing", "Unbewehrter Beton bis zur Mittelebene des 400 mm breiten Fundaments", "masonry", "concrete-footing", FOOTING_W / 2, 1.65, RHO_FOOTING * 1000 * FOOTING_W / 2)], "heatCapacityJM2K": round(FOOTING_W * FOOTING_D * FOOTING_L * RHO_FOOTING * 1000 / (AREA["footings"] / 2)), "source": "authored-estimate", "reference": "iso-10456-2007"},
        ],
        "emitters": [],
        "assumptions": [
            t("All four regions are inert passive surfaces: solar gain, long-wave exchange with sky and ground, forced convection from the park breeze, no internal heat; the table draws no power (passive, 0 W).", "Alle vier Regionen sind träge passive Flächen: Solargewinn, langwelliger Austausch mit Himmel und Boden, erzwungene Konvektion durch den Parkwind, keine innere Wärme; die Platte nimmt keine Leistung auf (passiv, 0 W)."),
            t("The 60 mm plate is split thermally at its mid-plane: the coating region carries the heat capacity of the upper half of the slab, the bare concrete region (underside and edges) the lower half; the coating underside and the concrete top under it are interfaces, not surfaces.", "Die 60-mm-Platte ist thermisch in ihrer Mittelebene geteilt: die Beschichtungsregion trägt die Wärmekapazität der oberen Plattenhälfte, die Region des blanken Betons (Unterseite und Kanten) die der unteren; Beschichtungsunterseite und Betonoberseite darunter sind Grenzflächen, keine Oberflächen."),
            t("Weathered surfaces differ from new ones: the dark green coating has faded and chalked from a solar absorptance of about 0.88 to 0.78, the grey concrete has darkened by dirt and algae from about 0.55 to 0.65, the powder-coated net has faded to 0.80 (authored estimates after the absorptance tables of Incropera et al. and ISO 10456 material data).", "Verwitterte Oberflächen unterscheiden sich von neuen: die dunkelgrüne Beschichtung ist von einer solaren Absorption von etwa 0,88 auf 0,78 ausgeblichen und kreidet, der graue Beton ist durch Schmutz und Algen von etwa 0,55 auf 0,65 nachgedunkelt, das pulverbeschichtete Netz auf 0,80 ausgeblichen (Schätzwerte nach den Absorptionstabellen von Incropera et al. und Stoffwerten nach ISO 10456)."),
            t("The white lines (about 4 % of the playing surface), the EPDM recess plugs and the stainless dowels are merged into the coating and the concrete; the strip footings are a region of their own, 86 % of their surface in the soil at 16 degC and their flush tops shaded under the plate; the foot bases conduct 7 % of the foot area into the footing tops at 22 degC.", "Die weißen Linien (etwa 4 % der Spielfläche), die EPDM-Aussparungsstopfen und die Edelstahldorne gehen in Beschichtung und Beton auf; die Streifenfundamente sind eine eigene Region, 86 % ihrer Oberfläche im Boden bei 16 °C und ihre bündigen Oberkanten im Schatten der Platte; die Fußflächen leiten 7 % der Fußoberfläche bei 22 °C in die Fundamentoberkanten."),
        ],
    }


def structure():
    """🏗️ Quasi-static rigid body on four foot contacts in service and on four pin supports at the lifting anchors on the crane."""
    return {
        "gravityMS2": 9.80665,
        "dynamics": "quasi-static",
        "supports": [],
        "assumptions": [
            t("The precast table is one rigid body: the 60 mm plate (bone table, a beam over the full length) cast monolithically with two 120 mm foot slabs and carrying the bolted net. In service it bears on four contact halves of the foot bases on the mortar beds of two strip footings (C20/25, 1.30 x 0.40 x 0.80 m, about 960 kg each, founded frost-free 0.80 m deep, tops flush with the gravel): they are drawn and modelled with the table because their tops are visible and they stay behind when it is lifted, but their weight goes straight into the soil and is neither part of the lifted rigid body nor of dimensions.massKg; two 20 mm stainless dowels per foot take the shear, idealised by friction 0.7.", "Die Fertigteilplatte ist ein starrer Körper: die 60-mm-Platte (Knochen table, ein Balken über die ganze Länge), monolithisch mit zwei 120-mm-Fußscheiben gegossen, trägt das angeschraubte Netz. Im Einsatz lagert sie auf vier Kontakthälften der Fußflächen auf den Mörtelbetten zweier Streifenfundamente (C20/25, 1,30 x 0,40 x 0,80 m, je etwa 960 kg, frostfrei 0,80 m tief gegründet, Oberkanten bündig mit der Decke): sie sind mit der Platte gezeichnet und modelliert, weil ihre Oberkanten sichtbar sind und sie beim Heben zurückbleiben, ihr Gewicht geht aber direkt in den Boden und gehört weder zum gehobenen starren Körper noch zu dimensions.massKg; je Fuß zwei 20-mm-Edelstahldorne nehmen den Schub auf, idealisiert durch Reibung 0,7."),
            t("Lifting: four pin supports at the cast-in spherical-head anchors over the feet stand for the four chain legs of the depot's sling (lifting anchors after VDI/BV-BS 6205, 2.5 t class each; the sling and hook are depot equipment, not part of the asset). Dead load from section x density: reinforced concrete 25 kN/m3 (EN 1991-1-1, Table A.1), steel 78.5 kN/m3.", "Heben: vier Gelenklager an den einbetonierten Kugelkopfankern über den Füßen stehen für die vier Kettenstränge des Bauhofgehänges (Transportanker nach VDI/BV-BS 6205, je Klasse 2,5 t; Gehänge und Haken sind Bauhofgerät, nicht Teil des Assets). Eigengewicht aus Querschnitt x Dichte: Stahlbeton 25 kN/m3 (EN 1991-1-1, Tabelle A.1), Stahl 78,5 kN/m3."),
            t("Player loads are authored estimates of leaning on the end edge (300 N and 150 N), well inside the stability and strength checks of EN 14468-1 for playing tables; wind on the 0.15 m net and ball impacts are negligible and not applied.", "Spielerlasten sind Schätzwerte für das Abstützen an der Stirnkante (300 N und 150 N), deutlich innerhalb der Standsicherheits- und Festigkeitsprüfungen der EN 14468-1 für Spieltische; Wind auf das 0,15 m hohe Netz und Ballstöße sind vernachlässigbar und nicht angesetzt."),
        ],
        "clinical": False,
    }


def solid(id, kind, bone, material, role, tone, **geom):
    """🧊️ One volume solid."""
    s = {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(geom)
    return s


def plate_solids(scale):
    """🟫️ The plate of a scale in the table frame: plan outlines extruded along v (plane zx: profile w, u)."""
    to_wu = lambda poly: pts([(z, x + HL) for x, z in poly])
    if scale == 50:
        return [
            solid("plate-lower", "extrude", "table", "concrete-precast", "body", 0.7, profile=to_wu(plate_outline([])), plane="zx", **{"from": r6(UNDER - 0.73), "to": r6(SPLIT - 0.73)}),
            solid("plate-upper", "extrude", "table", "concrete-precast", "body", 0.7, profile=to_wu(plate_outline(list(CHIPS))), plane="zx", **{"from": r6(SPLIT - 0.73), "to": r6(CONCRETE_TOP - 0.73)}),
            solid("coating", "extrude", "table", "coating-acrylic", "body", 0.38, sheet=True, profile=to_wu(plate_outline(list(CHIPS))), plane="zx", **{"from": r6(CONCRETE_TOP - 0.73), "to": r6(TOP - 0.73)}),
        ]
    if scale == 100:
        chips = ["c1", "c2", "c4"]
        return [
            solid("plate-lower", "extrude", "table", "concrete-precast", "body", 0.7, profile=to_wu(plate_outline([])), plane="zx", **{"from": r6(UNDER - 0.73), "to": r6(SPLIT - 0.73)}),
            solid("plate-upper", "extrude", "table", "concrete-precast", "body", 0.7, profile=to_wu(plate_outline(chips)), plane="zx", **{"from": r6(SPLIT - 0.73), "to": r6(CONCRETE_TOP - 0.73)}),
            solid("coating", "extrude", "table", "coating-acrylic", "body", 0.38, sheet=True, profile=to_wu(plate_outline(chips)), plane="zx", **{"from": r6(CONCRETE_TOP - 0.73), "to": r6(TOP - 0.73)}),
        ]
    bottom = UNDER if scale == 200 else TOP - 0.075
    return [
        solid("plate", "box", "table", "concrete-precast", "body", 0.7, size=[L, r6(CONCRETE_TOP - bottom), W], transform={"translation": [HL, r6((CONCRETE_TOP + bottom) / 2 - 0.73), 0.0]}),
        solid("coating", "box", "table", "coating-acrylic", "body", 0.38, sheet=True, size=[L, r6(COAT_T), W], transform={"translation": [HL, r6((CONCRETE_TOP + TOP) / 2 - 0.73), 0.0]}),
    ]


def feet_solids(scale):
    """🦶️ The two foot slabs of a scale in their bone frames (front profile in plane zx: w, u; thickness along v) and their haunches."""
    out = []
    for f, fx in (("a", -FOOT_X), ("b", FOOT_X)):
        bone = f"foot-{f}"
        if scale == 500:
            prof = foot_profile_top(TOP - 0.075)
        else:
            prof = foot_profile(scale == 50 and f == "b")
        out.append(solid(bone, "extrude", bone, "concrete-precast", "body", 0.66, profile=pts([(z, UNDER - y) for z, y in prof]), plane="zx", **{"from": -FOOT_T / 2, "to": FOOT_T / 2}))
        if scale in (50, 100):
            hw = FOOT_T / 2 + HAUNCH_W
            out.append(solid(f"haunch-{f}", "extrude", bone, "concrete-precast", "body", 0.66, profile=pts([(UNDER - HAUNCH_Y, -FOOT_T / 2), (UNDER - HAUNCH_Y, FOOT_T / 2), (0.0, hw), (0.0, -hw)]), plane="xy", **{"from": -FOOT_HEAD, "to": FOOT_HEAD}))
    return out


def net_solids(scale):
    """🕸️ The net of a scale in the net frame (u up from the playing surface, v = -x, w = z)."""
    out = []
    if scale in (50, 100):
        holes = [pts(circle(z, u, HOLE_R)) for z, u in HOLES]
        out.append(solid("net-plate", "extrude", "net", "steel-net", "body", 0.3, sheet=True, profile=pts(net_outline(scale == 50)), holes=holes, plane="zx", **{"from": -NET_T / 2, "to": NET_T / 2}))
    elif scale == 200:
        out.append(solid("net-plate", "box", "net", "steel-net", "body", 0.3, sheet=True, size=[NET_H, NET_T, W], transform={"translation": [NET_H / 2, 0.0, 0.0]}))
    else:
        out.append(solid("net-plate", "box", "net", "steel-net", "body", 0.3, sheet=True, size=[NET_H, NET_T, 2 * 0.7805], transform={"translation": [NET_H / 2, 0.0, 0.0]}))
    if scale == 50:
        z0, z1 = HW, HW + POST_T
    elif scale == 100:
        z0, z1 = HW, HW + 0.015
    elif scale == 200:
        z0, z1 = HW - 0.015, HW + 0.015
    else:
        return out
    u0, u1 = POST_Y0 - TOP, NET_H
    for side, sgn in (("l", 1), ("r", -1)):
        lo, hi = (z0, z1) if sgn > 0 else (-z1, -z0)
        out.append(solid(f"post-{side}", "box", "net", "steel-net", "body", 0.32, size=[r6(u1 - u0), 2 * POST_X, r6(hi - lo)], transform={"translation": [r6((u0 + u1) / 2), 0.0, r6((lo + hi) / 2)]}))
    if scale == 50:
        for side, sgn in (("l", 1), ("r", -1)):
            lo, hi = (z1, z1 + BOLT_H) if sgn > 0 else (-z1 - BOLT_H, -z1)
            for k, v in enumerate((-BOLT_X, BOLT_X)):
                out.append(solid(f"bolt-{side}-{k}", "extrude", "net", "steel-net", "detail", 0.22, profile=pts(hexagon(0.73 - TOP, v, BOLT_AF)), plane="xy", **{"from": r6(lo), "to": r6(hi)}))
    return out


def texture_solids(scale):
    """🏳️ The worn white lines and the EPDM plugs of the lifting-anchor recesses, as texture solids on the coating."""
    if scale not in (50, 100):
        return []
    out = []
    y0 = TOP - 0.73
    for id, x0, z0, x1, z1 in line_rects(LINES50 if scale == 50 else LINES100):
        out.append(solid(id, "box", "table", "paint-white", "texture", 0.95, texture=True, size=[r6(x1 - x0), 0.0003, r6(z1 - z0)], transform={"translation": [r6((x0 + x1) / 2 + HL), r6(y0 + 0.00015), r6((z0 + z1) / 2)]}))
    if scale == 50:
        for f, x in (("a", HL - FOOT_X), ("b", HL + FOOT_X)):
            for side, z in (("l", LIFT_Z), ("r", -LIFT_Z)):
                out.append(solid(f"cap-{f}-{side}", "revolve", "table", "epdm-cap", "texture", 0.12, texture=True, profile=[[0.0, 0.0], [0.0, 0.03], [0.002, 0.03], [0.002, 0.0]], axis=[0, 1, 0], origin=[r6(x), r6(y0), z]))
    return out


def dowel_solids(scale):
    """📌️ The four stainless dowels in the footing tops (ground bone: they stay behind when the table is lifted)."""
    if scale not in (50, 100):
        return []
    return [solid(f"dowel-{f}-{side}", "revolve", "ground", "stainless-dowel", "detail", 0.5, profile=[[0.0, 0.0], [0.0, DOWEL_R], [DOWEL_H, DOWEL_R], [DOWEL_H, 0.0]], axis=[0, 1, 0], origin=[fx, 0.0, z]) for f, fx in (("a", -FOOT_X), ("b", FOOT_X)) for side, z in (("l", DOWEL_Z), ("r", -DOWEL_Z))]


def footing_solids():
    """🧱️ The two strip footings 1.30 x 0.40 x 0.80 m of C20/25, the same box at every scale, on their own bones (u down from the flush top, v = x, w = z)."""
    return [solid(f"footing-{k}", "box", f"footing-{k}", "concrete-footing", "body", 0.78, size=[FOOTING_D, FOOTING_W, FOOTING_L], transform={"translation": [FOOTING_D / 2, 0.0, 0.0]}) for k in ("a", "b")]


def body(scale):
    """🧱️ The body list of a scale."""
    return plate_solids(scale) + feet_solids(scale) + net_solids(scale) + texture_solids(scale) + dowel_solids(scale) + footing_solids()


def thermal_list(scale, bodylist):
    """🌡️ The thermal list of a scale: references to the body solids, region per part, interfaces declared as section faces."""
    out = []
    for s in bodylist:
        sid = s["id"]
        if s.get("texture") or s["bone"] == "ground":
            continue
        if sid.startswith("footing"):
            out.append({"id": f"th-{sid}", "of": sid, "region": "footings"})
        elif sid == "coating":
            out.append({"id": "th-coating", "of": sid, "region": "playing-coating", "sectionFaces": ["-v"]})
        elif sid in ("plate-upper", "plate"):
            out.append({"id": f"th-{sid}", "of": sid, "region": "plate-concrete", "sectionFaces": ["+v"]})
        elif sid.startswith("plate"):
            out.append({"id": f"th-{sid}", "of": sid, "region": "plate-concrete"})
        elif sid.startswith("foot") or sid.startswith("haunch"):
            out.append({"id": f"th-{sid}", "of": sid, "region": "feet-concrete"})
        else:
            out.append({"id": f"th-{sid}", "of": sid, "region": "net-steel"})
    return out


def structure_list(scale, bodylist):
    """🏗️ The structure list of a scale: members along table, feet and net, joints at the haunches and posts, masses at the centres; the active supports are the contacts of each pose (feet or lifting anchors), never permanent."""
    f = FEATURE[scale]
    ids = {s["id"] for s in bodylist}
    out = []
    for sid in ("plate-lower", "plate-upper", "plate"):
        if sid in ids:
            out.append({"id": f"m-{sid}", "of": sid, "member": "table", "part": "member"})
    out += [{"id": "m-foot-a", "of": "foot-a", "member": "foot-a", "part": "member"}, {"id": "m-foot-b", "of": "foot-b", "member": "foot-b", "part": "member"}, {"id": "m-net", "of": "net-plate", "member": "net", "part": "member"}, {"id": "m-footing-a", "of": "footing-a", "member": "footing-a", "part": "member"}, {"id": "m-footing-b", "of": "footing-b", "member": "footing-b", "part": "member"}]
    for k in ("a", "b"):
        if f"haunch-{k}" in ids:
            out.append({"id": f"j-haunch-{k}", "of": f"haunch-{k}", "member": f"foot-{k}", "part": "joint"})
    for k in ("l", "r"):
        if f"post-{k}" in ids:
            out.append({"id": f"j-post-{k}", "of": f"post-{k}", "member": "net", "part": "joint"})
    rr = max(0.018, f / 2)
    plate_mid = 0.0 if scale != 500 else r6((TOP - 0.075 + CONCRETE_TOP) / 2 - 0.73)
    out.append({**solid("mass-table", "ellipsoid", "table", "concrete-precast", "detail", 0.3, radii=[rr, min(rr, 0.0295 if scale != 500 else 0.0375), rr], transform={"translation": [HL, plate_mid, 0.0]}), "member": "table", "part": "mass"})
    for fk in ("a", "b"):
        out.append({**solid(f"mass-foot-{fk}", "ellipsoid", f"foot-{fk}", "concrete-precast", "detail", 0.3, radii=[rr, min(rr, FOOT_T / 2), rr], transform={"translation": [r6(foot_com()), 0.0, 0.0]}), "member": f"foot-{fk}", "part": "mass"})
    nr = max(0.01, f / 2)
    out.append({**solid("mass-net", "ellipsoid", "net", "steel-net", "detail", 0.3, radii=[min(nr, 0.07), r6(max(NET_T / 2, f / 2)), nr], transform={"translation": [0.07, 0.0, 0.0]}), "member": "net", "part": "mass"})
    return out




def areas():
    """📐️ Hand calculation of the total outer area of every region from the 1:50 geometry (C29)."""
    lower, upper = plate_outline([]), plate_outline(list(CHIPS))
    coat = area(upper) + perimeter(upper) * COAT_T
    plate = area(lower) + perimeter(lower) * (SPLIT - UNDER) + perimeter(upper) * (CONCRETE_TOP - SPLIT) + (area(lower) - area(upper))
    trap = area(foot_profile(False))
    slant = math.hypot(FOOT_BASE - FOOT_HEAD, UNDER)
    band = 2 * FOOT_HEAD * (UNDER - HAUNCH_Y)
    haunch = 2 * FOOT_HEAD * (FOOT_T + 2 * HAUNCH_W) + 2 * math.hypot(HAUNCH_W, UNDER - HAUNCH_Y) * 2 * FOOT_HEAD + 4 * 0.5 * HAUNCH_W * (UNDER - HAUNCH_Y)
    foot = 2 * (trap - band) + 2 * slant * FOOT_T + 2 * FOOT_BASE * FOOT_T + haunch
    holes = len(HOLES) * math.pi * HOLE_R ** 2
    netplate = 2 * (area(net_outline(True)) - holes) + perimeter(net_outline(True)) * NET_T + len(HOLES) * 2 * math.pi * HOLE_R * NET_T
    ph = TOP + NET_H - POST_Y0
    post = 2 * (2 * POST_X * ph) + 2 * ph * POST_T + 2 * 2 * POST_X * POST_T - NET_H * NET_T
    hexa = 3 * math.sqrt(3) / 2 * (BOLT_AF / math.sqrt(3)) ** 2
    bolt = hexa + 6 * (BOLT_AF / math.sqrt(3)) * BOLT_H
    footing = 2 * (FOOTING_W * FOOTING_L + FOOTING_D * FOOTING_L + FOOTING_W * FOOTING_D)
    return {"playing-coating": round(coat, 3), "plate-concrete": round(plate, 3), "feet-concrete": round(2 * foot, 3), "net-steel": round(netplate + 2 * post + 4 * bolt, 3), "footings": round(2 * footing, 3)}


AREA.update(areas())

CONCRETE_HATCH = lambda: {"id": "concrete", "title": t("Concrete, stippled", "Beton, punktiert"), "pattern": "dots", "angleDeg": 45, "spacingMm": 0.8, "penMm": 0.1}


def elevation(scale):
    """📐️ Side elevation of a scale, seen from the right (-z), every shape in the frame of its bone."""
    g = []
    concrete = {"region": "plate-concrete", "material": "concrete-precast"}
    merged = {"regions": ["plate-concrete", "playing-coating"], "material": "concrete-precast"}
    hatch = {"hatch": "concrete"} if scale in (50, 100) else {}
    bottom = TOP - 0.075 if scale == 500 else UNDER
    g.append(shape("plate", "table", "body", 3, True, 0.7, P(tr(el_table, rect_pts(-HL, bottom, HL, CONCRETE_TOP))), **(concrete if scale in (50, 100) else merged), **hatch))
    if scale in (50, 100):
        g.append(shape("coating-edge", "table", "texture", 3.5, False, 0.38, P(tr(el_table, [(-HL, TOP), (HL, TOP)]), False), region="playing-coating", material="coating-acrylic"))
        g.append(shape("chip-c2", "table", "detail", 3.6, False, 0.5, P(tr(el_table, [(1.31, CONCRETE_TOP), (1.335, SPLIT), (HL, SPLIT)]), False), material="concrete-precast"))
    if scale == 50:
        g.append(shape("chip-c3", "table", "detail", 3.6, False, 0.5, P(tr(el_table, [(-0.45, CONCRETE_TOP), (-0.44, SPLIT), (-0.37, SPLIT), (-0.36, CONCRETE_TOP)]), False), material="concrete-precast"))
        g.append(shape("algae-plate", "table", "texture", 3.55, False, 0.4, P(tr(el_table, [(-1.20, UNDER + 0.006), (-0.95, UNDER + 0.004), (-0.60, UNDER + 0.007), (-0.20, UNDER + 0.004)]), False), material="concrete-precast"))
    for f, fx in (("a", -FOOT_X), ("b", FOOT_X)):
        bone = f"foot-{f}"
        if scale in (50, 100):
            side = [(fx - FOOT_T / 2, 0), (fx + FOOT_T / 2, 0), (fx + FOOT_T / 2, HAUNCH_Y), (fx + FOOT_T / 2 + HAUNCH_W, UNDER), (fx - FOOT_T / 2 - HAUNCH_W, UNDER), (fx - FOOT_T / 2, HAUNCH_Y)]
        else:
            side = rect_pts(fx - FOOT_T / 2, 0, fx + FOOT_T / 2, bottom)
        g.append(shape(bone, bone, "body", 2.5, True, 0.66, P(tr(el_foot(fx), side)), region="feet-concrete", material="concrete-precast", **hatch))
        if scale == 50:
            g.append(shape(f"algae-{f}", bone, "texture", 2.6, False, 0.4, P(tr(el_foot(fx), [(fx - 0.05, 0.05), (fx - 0.02, 0.03), (fx + 0.01, 0.06), (fx + 0.04, 0.035)]), False), material="concrete-precast"))
    if scale == 50:
        g.append(shape("spall-b", "foot-b", "detail", 2.7, False, 0.5, P(tr(el_foot(FOOT_X), [(FOOT_X - FOOT_T / 2, 0.035), (FOOT_X + FOOT_T / 2, 0.028)]), False), material="concrete-precast"))
    netw = 0.003
    g.append(shape("net", "net", "body", 4, True, 0.3, P(tr(el_net, rect_pts(-netw, TOP, netw, TOP + NET_H))), region="net-steel", material="steel-net", **({"penMm": 0.2} if scale in (200, 500) else {})))
    if scale != 500:
        g.append(shape("post", "net", "body", 4.2, True, 0.32, P(tr(el_net, rect_pts(-POST_X, POST_Y0, POST_X, TOP + NET_H))), region="net-steel", material="steel-net"))
    if scale == 50:
        for k, x in enumerate((-BOLT_X, BOLT_X)):
            g.append(shape(f"bolt-{k}", "net", "detail", 4.4, True, 0.22, P(tr(el_net, [(x + q, 0.73 + p) for p, q in hexagon(0, 0, BOLT_AF)])), region="net-steel", material="steel-net"))
        g.append(shape("rust-weld", "net", "texture", 4.5, False, 0.2, P(tr(el_net, [(0.0, TOP + NET_H - 0.004), (0.0, TOP + 0.07)]), False), material="steel-net"))
    if scale in (50, 100):
        for f, fx in (("a", -FOOT_X), ("b", FOOT_X)):
            g.append(shape(f"dowel-{f}", "ground", "body", 1, True, 0.5, P(rect_pts(fx - DOWEL_R, 0, fx + DOWEL_R, DOWEL_H)), material="stainless-dowel"))
        g += ground_line("elevation", scale == 50)
    g += footing_shapes("elevation", scale)
    return g


def front(scale):
    """🪟️ Front elevation of a scale, seen from ahead (+x): paper x is the lateral world z, paper y the height of the rest pose."""
    g = []
    concrete = {"region": "plate-concrete", "material": "concrete-precast"}
    merged = {"regions": ["plate-concrete", "playing-coating"], "material": "concrete-precast"}
    hatch = {"hatch": "concrete"} if scale in (50, 100) else {}
    bottom = TOP - 0.075 if scale == 500 else UNDER
    g.append(shape("plate", "table", "body", 3, True, 0.7, P(rect_pts(-HW, bottom, HW, CONCRETE_TOP)), **(concrete if scale in (50, 100) else merged), **hatch))
    if scale in (50, 100):
        g.append(shape("coating-edge", "table", "texture", 3.5, False, 0.38, P([(-HW, TOP), (HW, TOP)], False), region="playing-coating", material="coating-acrylic"))
        g.append(shape("chip-c1", "table", "detail", 3.6, False, 0.5, P([(0.20, CONCRETE_TOP), (0.21, SPLIT), (0.27, SPLIT), (0.29, CONCRETE_TOP)], False), material="concrete-precast"))
        g.append(shape("chip-c2", "table", "detail", 3.6, False, 0.5, P([(-0.712, CONCRETE_TOP), (-0.738, SPLIT), (-HW, SPLIT)], False), material="concrete-precast"))
    if scale == 500:
        g.append(shape("foot-b", "foot-b", "body", 2.5, True, 0.66, P(foot_profile_top(bottom)), region="feet-concrete", material="concrete-precast"))
    else:
        if scale in (50, 100):
            g.append(shape("foot-a", "foot-a", "body", 2.4, True, 0.6, P(foot_profile(False)), region="feet-concrete", material="concrete-precast", **hatch))
        g.append(shape("foot-b", "foot-b", "body", 2.5, True, 0.66, P(foot_profile(scale == 50)), region="feet-concrete", material="concrete-precast", **hatch))
    if scale == 50:
        g.append(shape("spall-b", "foot-b", "detail", 2.7, False, 0.5, P([(-0.545, 0.035), (-0.515, 0.028), (-0.505, 0.0)], False), material="concrete-precast"))
        tag = [[(-0.22, 0.34), (-0.16, 0.44), (-0.12, 0.33), (-0.06, 0.45), (-0.02, 0.36)], [(0.02, 0.35), (0.05, 0.44), (0.10, 0.40), (0.06, 0.37), (0.13, 0.34)], [(0.16, 0.45), (0.17, 0.34), (0.24, 0.38)]]
        for k, line in enumerate(tag):
            g.append(shape(f"graffiti-{k}", "foot-b", "texture", 2.8, False, 0.15, P(line, False), material="concrete-precast"))
        g.append(shape("algae-foot", "foot-b", "texture", 2.75, False, 0.4, P([(-0.50, 0.04), (-0.3, 0.025), (-0.05, 0.05), (0.2, 0.03), (0.48, 0.045)], False), material="concrete-precast"))
    if scale in (50, 100):
        outer = [(z, TOP + u) for z, u in net_outline(scale == 50)]
        rings = [outer] + [[(z, TOP + u) for z, u in circle(cz, cu, HOLE_R)] for cz, cu in HOLES]
        g.append(shape("net", "net", "body", 4, True, 0.3, D(rings), region="net-steel", material="steel-net"))
        pt = POST_T if scale == 50 else 0.015
        for side, s in (("l", 1), ("r", -1)):
            lo, hi = (HW, HW + pt) if s > 0 else (-HW - pt, -HW)
            g.append(shape(f"post-{side}", "net", "body", 4.2, True, 0.32, P(rect_pts(lo, POST_Y0, hi, TOP + NET_H)), region="net-steel", material="steel-net"))
        if scale == 50:
            rr = BOLT_AF / math.sqrt(3)
            for side, s in (("l", 1), ("r", -1)):
                lo, hi = (HW + POST_T, HW + POST_T + BOLT_H) if s > 0 else (-HW - POST_T - BOLT_H, -HW - POST_T)
                g.append(shape(f"bolt-{side}", "net", "detail", 4.4, True, 0.22, P(rect_pts(lo, 0.73 - rr, hi, 0.73 + rr)), region="net-steel", material="steel-net"))
        for k, z in enumerate((-DOWEL_Z, DOWEL_Z)):
            g.append(shape(f"dowel-{k}", "ground", "body", 1, True, 0.5, P(rect_pts(z - DOWEL_R, 0, z + DOWEL_R, DOWEL_H)), material="stainless-dowel"))
        g += ground_line("front", scale == 50)
    elif scale == 200:
        g.append(shape("net", "net", "body", 4, True, 0.3, P(rect_pts(-HW, TOP, HW, TOP + NET_H)), region="net-steel", material="steel-net"))
        for side, s in (("l", 1), ("r", -1)):
            lo, hi = (HW - 0.015, HW + 0.015) if s > 0 else (-HW - 0.015, -HW + 0.015)
            g.append(shape(f"post-{side}", "net", "body", 4.2, True, 0.32, P(rect_pts(lo, POST_Y0, hi, TOP + NET_H)), region="net-steel", material="steel-net"))
    else:
        g.append(shape("net", "net", "body", 4, True, 0.3, P(rect_pts(-0.7805, TOP, 0.7805, TOP + NET_H)), region="net-steel", material="steel-net"))
    g += footing_shapes("front", scale)
    return g


def plan(scale):
    """🗺️ Plan (top view) of a scale: paper x ahead, paper y the lateral world z (left up)."""
    g = []
    if scale in (50, 100):
        chips = list(CHIPS) if scale == 50 else ["c1", "c2", "c4"]
        g.append(shape("plate", "table", "body", 2, True, 0.7, P(plate_outline([])), region="plate-concrete", material="concrete-precast"))
        g.append(shape("coating", "table", "body", 3, True, 0.38, P(plate_outline(chips)), region="playing-coating", material="coating-acrylic"))
        for k, (id, poly) in enumerate(WORN if scale == 50 else WORN[:2]):
            g.append(shape(id, "table", "texture", 3.2 + 0.01 * k, True, 0.55, P(poly), material="concrete-precast"))
        for k, (id, x0, z0, x1, z1) in enumerate(line_rects(LINES50 if scale == 50 else LINES100)):
            if "centre" in id:
                g.append(shape(id, "table", "texture", 3.4, False, 0.95, P([(x0, 0.0), (x1, 0.0)], False), material="paint-white"))
            else:
                g.append(shape(id, "table", "texture", 3.4 + 0.001 * k, True, 0.95, P(rect_pts(x0, z0, x1, z1)), material="paint-white"))
        if scale == 100:
            g.append(shape("line-centre", "table", "texture", 3.45, False, 0.95, P([(-1.35, 0.0), (1.35, 0.0)], False), material="paint-white"))
        if scale == 50:
            for k, (x, z, r) in enumerate(GUM):
                g.append(shape(f"gum-{k}", "table", "texture", 3.5 + 0.001 * k, True, 0.2, E(x, z, r, r * 0.9), material="coating-acrylic"))
            for f, x in (("a", -FOOT_X), ("b", FOOT_X)):
                for side, z in (("l", LIFT_Z), ("r", -LIFT_Z)):
                    g.append(shape(f"cap-{f}-{side}", "table", "detail", 3.6, True, 0.12, E(x, z, 0.03, 0.03), material="epdm-cap"))
        g.append(shape("net", "net", "body", 4, True, 0.3, P(rect_pts(-0.003, -HW, 0.003, HW)), region="net-steel", material="steel-net"))
        pt = POST_T if scale == 50 else 0.015
        for side, s in (("l", 1), ("r", -1)):
            lo, hi = (HW, HW + pt) if s > 0 else (-HW - pt, -HW)
            g.append(shape(f"post-{side}", "net", "body", 4.2, True, 0.32, P(rect_pts(-POST_X, lo, POST_X, hi)), region="net-steel", material="steel-net"))
        if scale == 50:
            for side, s in (("l", 1), ("r", -1)):
                lo, hi = (HW + POST_T, HW + POST_T + BOLT_H) if s > 0 else (-HW - POST_T - BOLT_H, -HW - POST_T)
                for k, x in enumerate((-BOLT_X, BOLT_X)):
                    g.append(shape(f"bolt-{side}-{k}", "net", "detail", 4.4, True, 0.22, P(rect_pts(x - BOLT_AF / 2, lo, x + BOLT_AF / 2, hi)), region="net-steel", material="steel-net"))
        for f, sgn in (("a", -1), ("b", 1)):
            g.append(shape(f"hollow-{f}", "ground", "ground", 0.1, False, 0.5, P([(sgn * x, z) for x, z in [(1.58, -0.45), (1.75, -0.55), (2.15, -0.56), (2.38, -0.42), (2.40, 0.38), (2.20, 0.54), (1.78, 0.55), (1.58, 0.42)]]), penMm=0.18 if scale == 50 else 0.13))
    elif scale == 200:
        g.append(shape("plate", "table", "body", 3, True, 0.38, P(rect_pts(-HL, -HW, HL, HW)), regions=["playing-coating", "plate-concrete"], material="coating-acrylic"))
        g.append(shape("net", "net", "body", 4, True, 0.3, P(rect_pts(-0.003, -HW, 0.003, HW)), region="net-steel", material="steel-net", penMm=0.2))
        for side, s in (("l", 1), ("r", -1)):
            lo, hi = (HW - 0.015, HW + 0.015) if s > 0 else (-HW - 0.015, -HW + 0.015)
            g.append(shape(f"post-{side}", "net", "body", 4.2, True, 0.32, P(rect_pts(-POST_X, lo, POST_X, hi)), region="net-steel", material="steel-net"))
    else:
        g.append(shape("plate", "table", "body", 3, True, 0.38, P(rect_pts(-HL, -HW, HL, HW)), regions=["playing-coating", "plate-concrete"], material="coating-acrylic"))
        g.append(shape("net", "net", "body", 4, True, 0.3, P(rect_pts(-0.003, -0.7805, 0.003, 0.7805)), region="net-steel", material="steel-net", penMm=0.2))
    return g


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, 500: {"body": 0.14, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": t("The precast table at true size: the 2.74 x 1.525 m plate 60 mm thick with its rounded mould corners, the two 120 mm foot slabs with their haunches under the plate, the 6 mm perforated steel net with its 60 holes of 25 mm and its bent-down top edge, the two 80 x 10 mm posts on the plate edges with their bolt heads; front shows both feet, the near one spalled at its base corner, plan the coated top with all five arris chips.", "Die Fertigteilplatte in wahrer Größe: die 2,74 x 1,525 m große, 60 mm dicke Platte mit gerundeten Schalungsecken, die zwei 120-mm-Fußscheiben mit ihren Vouten unter der Platte, das 6-mm-Lochblechnetz mit seinen 60 Löchern von 25 mm und der nach unten verbogenen Oberkante, die zwei Pfosten 80 x 10 mm an den Plattenkanten mit ihren Schraubenköpfen; die Ansicht zeigt beide Füße, den nahen an seiner Fußecke abgeplatzt, der Grundriss die beschichtete Oberseite mit allen fünf Kantenabplatzungen."),
        "retained": t("Every trace of use above 7.5 mm: five chips of 22 to 30 mm at the arrises, worn-through coating at both ends and on the near side, the worn white lines broken where hands and hips rub, chewing-gum spots, the mower spall at the foot base, algae on the shaded underside and foot bases, a graffiti tag on the end face of foot B, rust at the net weld, the hollows worn into the gravel at both ends, the EPDM plugs of the four lifting anchors.", "Jede Gebrauchsspur über 7,5 mm: fünf Abplatzungen von 22 bis 30 mm an den Kanten, durchgescheuerte Beschichtung an beiden Enden und an der nahen Seite, die abgenutzten weißen Linien, unterbrochen, wo Hände und Hüften reiben, Kaugummiflecken, die Mäherabplatzung am Fuß, Algen auf der beschatteten Unterseite und an den Fußflächen, ein Graffiti-Tag auf der Stirnseite von Fuß B, Rost an der Netznaht, die in den Kies an beiden Enden getretenen Mulden, die EPDM-Stopfen der vier Transportanker."),
        "merged": t("Coating and plate are one outline in elevation and front (the 0.5 mm coating is a texture line on top); the reinforcement and the cast-in anchors are inside the concrete.", "Beschichtung und Platte sind in Seitenansicht und Ansicht ein Umriss (die 0,5-mm-Beschichtung ist eine Texturlinie oben); Bewehrung und einbetonierte Anker liegen im Beton."),
        "omitted": t("The plan does not show the strip footings, which lie wholly under the plate; the dowels are drawn behind the feet and show only when the table is lifted.", "Die Draufsicht zeigt die Streifenfundamente nicht, die ganz unter der Platte liegen; die Dorne liegen hinter den Füßen und zeigen sich erst, wenn die Platte gehoben ist."),
        "thinElements": t("The 6 mm net edge-on is a filled band of 0.12 mm plus pen; the 3 mm centre line is a texture stroke, the 20 mm lines filled 0.4 mm strips.", "Das 6-mm-Netz in Kantenansicht ist ein gefülltes Band von 0,12 mm plus Stift; die 3-mm-Mittellinie ist ein Texturstrich, die 20-mm-Linien gefüllte Streifen von 0,4 mm."),
        "hatching": t("Concrete in elevation and front is stippled (dots 0.8 mm, pen 0.1 mm); steel and coating are toned only.", "Beton ist in Seitenansicht und Ansicht punktiert (Punkte 0,8 mm, Stift 0,1 mm); Stahl und Beschichtung sind nur getönt."),
        "styles": t("Body pen 0.35 mm, detail 0.18 mm for chips, spall and bolt heads, texture 0.1 mm for lines, gum, algae, graffiti and rust, ground 0.5 mm with the hollows; concrete tone 0.7, feet 0.66, faded coating 0.38, worn patches 0.55, white lines 0.95, net 0.3.", "Körperstift 0,35 mm, Detail 0,18 mm für Abplatzungen und Schraubenköpfe, Textur 0,1 mm für Linien, Kaugummi, Algen, Graffiti und Rost, Boden 0,5 mm mit den Mulden; Beton Ton 0,7, Füße 0,66, ausgeblichene Beschichtung 0,38, durchgescheuerte Stellen 0,55, weiße Linien 0,95, Netz 0,3."),
        "overlays": t("The plate carries plate-concrete and its top line playing-coating, the plan top playing-coating, the feet feet-concrete, net, posts and bolts net-steel; force and moment views colour plate, feet and net by their members.", "Die Platte trägt plate-concrete und ihre Oberlinie playing-coating, die Draufsicht playing-coating, die Füße feet-concrete, Netz, Pfosten und Schrauben net-steel; Kraft- und Momentenansichten färben Platte, Füße und Netz nach ihren Stäben."),
    },
    100: {
        "silhouette": t("The table as at 1:50 with the details that read at 15 mm: plate with three of the five chips (c1, c2, c4 of 28 to 60 mm), feet with haunches without the spall, the net with its 60 holes but a straight top edge, posts thickened to 15 mm, no bolt heads.", "Die Platte wie bei 1:50 mit den Einzelheiten, die bei 15 mm lesbar sind: Platte mit drei der fünf Abplatzungen (c1, c2, c4 von 28 bis 60 mm), Füße mit Vouten ohne die Abplatzung, das Netz mit seinen 60 Löchern, aber gerader Oberkante, Pfosten auf 15 mm verstärkt, keine Schraubenköpfe."),
        "retained": t("Rounded mould corners, haunches, the hole pattern of the net (25 mm holes, 25 mm bridges), the worn lines with their two largest gaps, two worn-through patches at the ends, dowels, ground hollows and the two strip footings cut below the ground line.", "Gerundete Schalungsecken, Vouten, das Lochbild des Netzes (25-mm-Löcher, 25-mm-Stege), die abgenutzten Linien mit ihren zwei größten Lücken, zwei durchgescheuerte Stellen an den Enden, Dorne, Bodenmulden und die zwei unter der Bodenlinie geschnittenen Streifenfundamente."),
        "merged": t("The two smallest chips (22 and 40 mm), the bent top edge of the net (12 mm) and the bolt heads (8 mm) merge into their parts; the post grows from 10 to 15 mm outwards.", "Die zwei kleinsten Abplatzungen (22 und 40 mm), die verbogene Netzoberkante (12 mm) und die Schraubenköpfe (8 mm) gehen in ihren Teilen auf; der Pfosten wächst von 10 auf 15 mm nach außen."),
        "omitted": t("Chewing gum, graffiti, algae and rust strokes, the EPDM plugs, the mower spall.", "Kaugummi, Graffiti, Algen- und Roststriche, die EPDM-Stopfen, die Mäherabplatzung."),
        "thinElements": t("The net edge-on is a filled 0.06 mm band plus pen, the 0.5 mm coating a texture line, the centre line one texture stroke.", "Das Netz in Kantenansicht ist ein gefülltes Band von 0,06 mm plus Stift, die 0,5-mm-Beschichtung eine Texturlinie, die Mittellinie ein Texturstrich."),
        "hatching": t("Concrete stippled as at 1:50 (0.8 mm, pen 0.1 mm).", "Beton punktiert wie bei 1:50 (0,8 mm, Stift 0,1 mm)."),
        "styles": t("Body pen 0.25 mm, detail 0.13 mm, texture 0.1 mm, ground 0.35 mm; tones as at 1:50.", "Körperstift 0,25 mm, Detail 0,13 mm, Textur 0,1 mm, Boden 0,35 mm; Töne wie bei 1:50."),
        "overlays": t("Regions as at 1:50.", "Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": t("Plate as one 60 mm slab with square corners, the feet as plain 120 mm slabs (haunches merged), the net as a plain 6 mm plate, the posts as 30 mm bars over the plate edges; front shows the foot trapezoid of 1.10 to 0.90 m.", "Platte als eine 60-mm-Scheibe mit rechtwinkligen Ecken, die Füße als glatte 120-mm-Scheiben (Vouten aufgegangen), das Netz als glatte 6-mm-Platte, die Pfosten als 30-mm-Stäbe über den Plattenkanten; die Ansicht zeigt das Fußtrapez von 1,10 auf 0,90 m."),
        "retained": t("Length, width, the 0.76 m playing height, net height 152.5 mm, foot positions and their taper.", "Länge, Breite, die Spielhöhe 0,76 m, Netzhöhe 152,5 mm, Fußlagen und ihr Anzug."),
        "merged": t("Coating and concrete are one shape carrying both regions; haunches, holes, chips and lines merge into their parts.", "Beschichtung und Beton sind eine Form mit beiden Regionen; Vouten, Löcher, Abplatzungen und Linien gehen in ihren Teilen auf."),
        "omitted": t("Every trace of use below 30 mm, the dowels and the worn hollows of the ground line, which is drawn straight over the two strip footings.", "Jede Gebrauchsspur unter 30 mm, die Dorne und die Mulden der Bodenlinie, die gerade über den zwei Streifenfundamenten gezeichnet ist."),
        "thinElements": t("The 6 mm net edge-on is a filled band drawn with a 0.2 mm pen so it prints to 1:350.", "Das 6-mm-Netz in Kantenansicht ist ein gefülltes Band mit 0,2-mm-Stift, damit es bis 1:350 druckt."),
        "hatching": t("None.", "Keine."),
        "styles": t("Body pen 0.18 mm, detail 0.13 mm, ground 0.25 mm; plate tone 0.7 in the views, the coated top 0.38 in plan.", "Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,25 mm; Platte Ton 0,7 in den Ansichten, die beschichtete Oberseite 0,38 im Grundriss."),
        "overlays": t("The plate shape carries playing-coating and plate-concrete merged, feet feet-concrete, net and posts net-steel.", "Die Plattenform trägt playing-coating und plate-concrete zusammen, Füße feet-concrete, Netz und Pfosten net-steel."),
    },
    500: {
        "silhouette": t("Plate drawn 75 mm thick (the print floor; thickened downwards so the playing height stays 0.76 m), two 120 mm feet, the net as a 6 mm plate that reaches over the posts to the full 1.561 m width.", "Platte 75 mm dick gezeichnet (die Druckgrenze; nach unten verdickt, damit die Spielhöhe 0,76 m bleibt), zwei 120-mm-Füße, das Netz als 6-mm-Platte, die über die Pfosten bis zur vollen Breite von 1,561 m reicht."),
        "retained": t("Length 2.74 m, width with posts 1.561 m, height 0.9125 m to the net top above the ground (dimensions.heightM 1.7125 m includes the 0.80 m of the strip footings below it), playing height 0.76 m, foot positions at +-0.78 m.", "Länge 2,74 m, Breite mit Pfosten 1,561 m, Höhe 0,9125 m bis zur Netzoberkante über Boden (dimensions.heightM 1,7125 m schließt die 0,80 m der Streifenfundamente darunter ein), Spielhöhe 0,76 m, Fußlagen bei +-0,78 m."),
        "merged": t("Posts and net are one plate; coating and concrete one shape.", "Pfosten und Netz sind eine Platte; Beschichtung und Beton eine Form."),
        "omitted": t("All traces of use (below 75 mm), the dowels and the worn hollows of the ground line, which is drawn straight over the two strip footings.", "Alle Gebrauchsspuren (unter 75 mm), die Dorne und die Mulden der Bodenlinie, die gerade über den zwei Streifenfundamenten gezeichnet ist."),
        "thinElements": t("The 60 mm plate would be 0.12 mm on paper: drawn 75 mm (0.15 mm); the net edge-on is a filled band with a 0.2 mm pen so it still prints at 1:1000.", "Die 60-mm-Platte wäre auf dem Papier 0,12 mm: 75 mm gezeichnet (0,15 mm); das Netz in Kantenansicht ist ein gefülltes Band mit 0,2-mm-Stift, damit es auch bei 1:1000 druckt."),
        "hatching": t("None.", "Keine."),
        "styles": t("Body pen 0.14 mm, ground 0.18 mm; detail and texture are not drawn.", "Körperstift 0,14 mm, Boden 0,18 mm; Detail und Textur werden nicht gezeichnet."),
        "overlays": t("As at 1:200.", "Wie bei 1:200."),
    },
}

VOLUME_DECISIONS = {
    50: [t("Body, authored for 7.5 mm: the plate as two extruded layers on its plan outline with 15 mm mould corners, the lower 35 mm full, the upper 24.5 mm with the five arris chips (22 to 30 mm deep, 60 to 90 mm long) cut into it, and the 0.5 mm faded coating as a declared sheet on the chipped outline; the two 120 mm foot slabs as extruded trapezoids (1.10 m at the base, 0.90 m under the plate), foot B with the mower spall at its near base corner, each with its 50 mm haunch under the plate; the 6 mm perforated net plate with its 60 holes of 25 mm and the bent-down top edge, the two 80 x 10 mm posts and four M12 hexagon bolt heads; the worn white lines (0.3 mm, 20 and 3 mm wide, broken where hands, hips and chips took the paint) and the four EPDM recess plugs as texture solids; the four 20 mm stainless dowels in the footing tops on the ground bone, inside the feet in service and standing in the open when the table is lifted.", "Körper, für 7,5 mm gebaut: die Platte als zwei extrudierte Lagen auf ihrem Grundrissumriss mit 15-mm-Schalungsecken, die unteren 35 mm voll, die oberen 24,5 mm mit den fünf eingeschnittenen Kantenabplatzungen (22 bis 30 mm tief, 60 bis 90 mm lang), und die 0,5 mm dicke ausgeblichene Beschichtung als erklärtes Blech auf dem abgeplatzten Umriss; die zwei 120-mm-Fußscheiben als extrudierte Trapeze (1,10 m am Fuß, 0,90 m unter der Platte), Fuß B mit der Mäherabplatzung an seiner nahen Fußecke, jede mit ihrer 50-mm-Voute unter der Platte; das 6-mm-Lochblechnetz mit seinen 60 Löchern von 25 mm und der verbogenen Oberkante, die zwei Pfosten 80 x 10 mm und vier M12-Sechskantköpfe; die abgenutzten weißen Linien (0,3 mm, 20 und 3 mm breit, unterbrochen, wo Hände, Hüften und Abplatzungen die Farbe genommen haben) und die vier EPDM-Stopfen als Textursolids; die vier 20-mm-Edelstahldorne in den Fundamentoberkanten am Bodenknochen, im Einsatz in den Füßen, nach dem Heben frei stehend."), t("Thermal: references to the body solids, one region per part; the coating underside and the concrete top under it are declared interfaces (section faces), so the coating counts its top and the plate its underside and edges. Structure: the plate layers as the member of the table bone, the foot slabs as their members, the net plate as its member, haunches and posts as joints, mass markers at the centres; the supports are not permanent (the feet bear in service, the four lifting anchors on the crane), so they are the active contacts of each pose and carry no support solid.", "Thermik: Verweise auf die Körpersolids, eine Region je Teil; die Beschichtungsunterseite und die Betonoberseite darunter sind erklärte Grenzflächen (Schnittflächen), so zählt die Beschichtung ihre Oberseite und die Platte Unterseite und Kanten. Tragwerk: die Plattenlagen als Stab des Knochens table, die Fußscheiben als ihre Stäbe, das Netzblech als sein Stab, Vouten und Pfosten als Knoten, Massenmarken in den Schwerpunkten; die Auflager sind nicht ständig (im Einsatz tragen die Füße, am Kran die vier Transportanker), daher sind sie die aktiven Kontakte jeder Pose und haben keinen Lagerkörper.")],
    100: [t("Body, authored for 15 mm: the plate as two layers with three chips (c1, c2, c4; the 22 mm side chip and the 40 mm corner chip are below the scale's reading), the coating sheet on that outline, both feet without the spall but with their 50 mm haunches, the perforated net with its 60 holes (25 mm holes and bridges read at 15 mm) and a straight top edge, the posts thickened outwards to 15 mm, no bolt heads; the worn lines with their two largest gaps as texture solids, no plugs; the four dowels stay (20 mm).", "Körper, für 15 mm gebaut: die Platte als zwei Lagen mit drei Abplatzungen (c1, c2, c4; die 22-mm-Seitenabplatzung und die 40-mm-Eckabplatzung liegen unter der Lesbarkeit des Maßstabs), das Beschichtungsblech auf diesem Umriss, beide Füße ohne Abplatzung, aber mit ihren 50-mm-Vouten, das Lochblechnetz mit seinen 60 Löchern (Löcher und Stege von 25 mm sind bei 15 mm lesbar) und gerader Oberkante, die Pfosten nach außen auf 15 mm verstärkt, keine Schraubenköpfe; die abgenutzten Linien mit ihren zwei größten Lücken als Textursolids, keine Stopfen; die vier Dorne bleiben (20 mm)."), t("Thermal and structure as at 1:50 on these solids.", "Thermik und Tragwerk wie bei 1:50 auf diesen Solids.")],
    200: [t("Body, authored for 30 mm: the plate as one 59.5 mm box with the coating sheet on it, the feet as plain extruded trapezoids (haunches of 50 mm merged), the net as a plain 6 mm sheet without holes, the posts as 30 mm bars over the plate edges (generalised: they reach 15 mm into the plate edge and the net end); chips, lines, plugs, bolts and dowels are below 30 mm and left out.", "Körper, für 30 mm gebaut: die Platte als ein 59,5-mm-Quader mit dem Beschichtungsblech darauf, die Füße als glatte extrudierte Trapeze (Vouten von 50 mm aufgegangen), das Netz als glattes 6-mm-Blech ohne Löcher, die Pfosten als 30-mm-Stäbe über den Plattenkanten (generalisiert: sie greifen 15 mm in Plattenkante und Netzende); Abplatzungen, Linien, Stopfen, Schrauben und Dorne liegen unter 30 mm und entfallen."), t("Thermal: one reference per part; structure: plate, feet and net members, post joints, mass markers; supports are the contacts of each pose.", "Thermik: ein Verweis je Teil; Tragwerk: Stäbe von Platte, Füßen und Netz, Pfostenknoten, Massenmarken; Auflager sind die Kontakte jeder Pose.")],
    500: [t("Body, authored for 75 mm: the plate as one box 75 mm thick (thickened downwards to the print floor, top at 0.7595 m), the coating sheet on it, the feet as trapezoids ending under the thickened plate, the net as one 6 mm sheet reaching over the posts to the full width of 1.561 m; the model is minimal at this scale.", "Körper, für 75 mm gebaut: die Platte als ein 75 mm dicker Quader (nach unten bis zur Druckgrenze verdickt, Oberkante bei 0,7595 m), das Beschichtungsblech darauf, die Füße als Trapeze bis unter die verdickte Platte, das Netz als ein 6-mm-Blech, das über die Pfosten bis zur vollen Breite von 1,561 m reicht; das Modell ist bei diesem Maßstab minimal."), t("Minimal: thermal one reference per part, structure the three members and four mass markers.", "Minimal: Thermik ein Verweis je Teil, Tragwerk die drei Stäbe und vier Massenmarken.")],
}


def symbols():
    """🔣️ A true-extent symbol for site plans and a nominal sign for the smallest scales."""
    true_geom = {
        "elevation": [shape("plate", "ground", "body", 3, True, 0.7, P(rect_pts(-HL, UNDER, HL, TOP))), shape("foot-a", "ground", "body", 2, True, 0.66, P(rect_pts(-FOOT_X - 0.06, 0, -FOOT_X + 0.06, UNDER))), shape("foot-b", "ground", "body", 2, True, 0.66, P(rect_pts(FOOT_X - 0.06, 0, FOOT_X + 0.06, UNDER))), shape("footing-a", "ground", "body", 1, True, 0.78, P(rect_pts(-FOOT_X - FOOTING_W / 2, -FOOTING_D, -FOOT_X + FOOTING_W / 2, 0))), shape("footing-b", "ground", "body", 1, True, 0.78, P(rect_pts(FOOT_X - FOOTING_W / 2, -FOOTING_D, FOOT_X + FOOTING_W / 2, 0))), shape("net", "ground", "body", 4, True, 0.3, P(rect_pts(-0.003, TOP, 0.003, TOP + NET_H)))],
        "front": [shape("plate", "ground", "body", 3, True, 0.7, P(rect_pts(-HW, UNDER, HW, TOP))), shape("foot", "ground", "body", 2, True, 0.66, P(foot_profile(False))), shape("footing", "ground", "body", 1, True, 0.78, P(rect_pts(-FOOTING_L / 2, -FOOTING_D, FOOTING_L / 2, 0))), shape("net", "ground", "body", 4, True, 0.3, P(rect_pts(-0.7805, TOP, 0.7805, TOP + NET_H)))],
        "plan": [shape("plate", "ground", "body", 3, True, 0.38, P(rect_pts(-HL, -0.7805, HL, 0.7805))), shape("net", "ground", "body", 4, True, 0.3, P(rect_pts(-0.003, -0.7805, 0.003, 0.7805)))],
    }
    nominal = {
        "elevation": [shape("top", "ground", "body", 3, True, 0.7, P(rect_pts(-2.7, 1.6, 2.7, 2.0))), shape("legs", "ground", "body", 2, False, 0.5, P([(-1.6, 0), (-1.6, 1.6)], False)), shape("legs-b", "ground", "body", 2, False, 0.5, P([(1.6, 0), (1.6, 1.6)], False))],
        "front": [shape("top", "ground", "body", 3, True, 0.7, P(rect_pts(-1.6, 1.6, 1.6, 2.0))), shape("leg", "ground", "body", 2, False, 0.5, P([(0, 0), (0, 1.6)], False))],
        "plan": [shape("top", "ground", "body", 3, False, 0.5, P(rect_pts(-2.7, -1.5, 2.7, 1.5))), shape("net", "ground", "body", 4, False, 0.5, P([(0, -1.8), (0, 1.8)], False))],
    }
    return [
        {"id": "used-concrete-table-tennis-true-extent", "title": t("Table tennis table, true extent", "Tischtennisplatte, wahre Ausdehnung"), "description": t("The concrete table at real size for site plans: the 2.74 x 1.525 m plate with the net line across its middle and the posts at both sides (1.561 m); side view with the two feet on their strip footings 0.80 m into the ground, front view with the tapered foot on its footing.", "Die Betonplatte in wahrer Größe für Lagepläne: die Platte 2,74 x 1,525 m mit der Netzlinie in der Mitte und den Pfosten an beiden Seiten (1,561 m); Seitenansicht mit den zwei Füßen auf ihren 0,80 m in den Boden reichenden Streifenfundamenten, Ansicht mit dem geneigten Fuß auf seinem Fundament."), "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}, "hatches": [], "geometry": true_geom, "dimensionalNote": t("Model metres at real size in the asset frame (x ahead along the length, lateral positive to the left, y up); the side view from the right, the front view from ahead, the top view from above, all in the default pose; net and posts reach 0.9125 m above the ground and 1.561 m, the footings 0.80 m below it (1.7125 m in all).", "Modellmeter in wahrer Größe im Asset-Rahmen (x längs nach vorn, seitlich positiv nach links, y nach oben); Seitenansicht von rechts, Ansicht von vorn, Draufsicht von oben, alle in der Standardhaltung; Netz und Pfosten reichen bis 0,9125 m über Boden und 1,561 m, die Fundamente 0,80 m darunter (zusammen 1,7125 m).")},
        {"id": "used-concrete-table-tennis-sign", "title": t("Table tennis sign", "Tischtennis-Planzeichen"), "description": t("A conventional plan sign for a table tennis table at the smallest scale: an outlined 5.4 x 3.0 mm rectangle crossed by the net line; side view a top bar on two legs, front view a bar on one leg.", "Ein Planzeichen für eine Tischtennisplatte im kleinsten Maßstab: ein umrandetes Rechteck 5,4 x 3,0 mm, von der Netzlinie gekreuzt; Seitenansicht eine Deckleiste auf zwei Beinen, Ansicht eine Leiste auf einem Bein."), "semantics": "nominal", "units": "paper-mm", "denominators": [500], "pens": {"body": 0.25, "detail": None, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [], "geometry": nominal, "dimensionalNote": t("Nominal paper sign in paper millimetres, not to scale (the real table is 5.5 x 3.1 mm at 1:500); insertion point at the centre of the table on the ground.", "Nominelles Papierzeichen in Papiermillimetern, nicht maßstäblich (die reale Platte ist bei 1:500 5,5 x 3,1 mm); Einfügepunkt in der Plattenmitte am Boden.")},
    ]


REFERENCES = [
    {"id": "ittf-handbook", "citation": "International Table Tennis Federation (2024). ITTF Statutes, Chapter 2: The Laws of Table Tennis, 2.1 The Table (playing surface 2.74 m x 1.525 m, upper surface 76 cm above the floor, 2 cm white side and end lines, 3 mm white centre line) and 2.2 The Net Assembly (net 15.25 cm high along its whole length)."},
    {"id": "en-14468-1", "citation": "DIN EN 14468-1:2015 Table tennis - Part 1: Playing tables, requirements and test methods (dimensions, stability and strength of indoor and outdoor tables, ball rebound)."},
    {"id": "en-1991-1-1", "citation": "DIN EN 1991-1-1:2010 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads for buildings (Table A.1: reinforced normal-weight concrete 25 kN/m3, plain concrete 24 kN/m3; Table A.4: steel 77-78.5 kN/m3)."},
    {"id": "en-1992-1-1", "citation": "DIN EN 1992-1-1:2011 Eurocode 2: Design of concrete structures - Part 1-1, Table 3.1 (C35/45: fck 35 MPa, Ecm 34 GPa; C20/25: fck 20 MPa, Ecm 30 GPa)."},
    {"id": "en-1993-1-1", "citation": "DIN EN 1993-1-1:2010 Eurocode 3: Design of steel structures - Part 1-1 (E = 210 GPa, density 7850 kg/m3, S235 fy 235 MPa)."},
    {"id": "din-24041", "citation": "DIN 24041:2002 Perforated plates - Dimensions (round holes in straight rows; here Rv 25 holes at 50 mm pitch in 6 mm plate)."},
    {"id": "vdi-bv-bs-6205", "citation": "VDI/BV-BS 6205:2012 Lifting inserts and lifting insert systems for precast concrete elements - Basic principles, design, use (spherical-head anchors, load classes, multi-leg slings)."},
    {"id": "iso-10456-2007", "citation": "DIN EN ISO 10456:2010 Building materials and products - Hygrothermal properties - Tabulated design values (reinforced concrete 1 % steel: 2400 kg/m3, 2.3 W/(m K), 1000 J/(kg K); EPDM 1150 kg/m3, 0.25 W/(m K))."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley: radiative properties of surfaces (concrete, paints, galvanised and painted steel), convection correlations for plates."},
    {"id": "precast-catalogue-typical", "citation": "Catalogue-typical data of German municipal precast concrete outdoor table tennis tables (types, not brands): monolithic plate 50 to 80 mm on two foot slabs, mass 0.9 to 1.3 t, perforated or bar steel net bolted to the plate edges, four cast-in lifting anchors, set on two strip footings or a gravel bed and doweled."},
]


def representation(scale):
    """🖨️ One authored representation."""
    lo, hi = RANGES[scale]
    hatches = [CONCRETE_HATCH()] if scale in (50, 100) else []
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": hatches, "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": DECISIONS[scale]}


def volume(scale):
    """🧊️ One authored volume."""
    b = body(scale)
    return {"status": "authored", "denominator": scale, "body": b, "thermal": thermal_list(scale, b), "structure": structure_list(scale, b), "decisions": VOLUME_DECISIONS[scale]}


def document():
    """📄️ The whole asset document."""
    mass = round(MASS_TOTAL, 1)
    asset = {
        "id": "used-concrete-table-tennis",
        "category": "objects",
        "title": t("Used Concrete Table Tennis Table", "Gebrauchte Beton-Tischtennisplatte"),
        "summary": t(f"An outdoor precast concrete table tennis table after many park summers: the ITTF playing surface of 2.74 x 1.525 m at 0.76 m on a 60 mm reinforced plate cast in one with two tapered 120 mm foot slabs and a 6 mm perforated steel net bolted to the plate edges, about {mass / 1000:.2f} t doweled onto two strip footings whose tops lie flush with a water-bound gravel surface. Its use shows in five chipped arrises, the faded green coating worn through at both ends, broken white lines, chewing gum, a mower spall, algae, a graffiti tag, a bent net edge and the hollows worn into the gravel where the players stand; the depot lifts it out by crane on its four lifting anchors, and two players lean on its ends.", f"Eine Fertigteil-Tischtennisplatte aus Beton nach vielen Parksommern: die ITTF-Spielfläche von 2,74 x 1,525 m auf 0,76 m auf einer 60-mm-Stahlbetonplatte, in einem Stück mit zwei geneigten 120-mm-Fußscheiben gegossen, und ein 6-mm-Lochblechnetz an den Plattenkanten, etwa {mass / 1000:.2f} t, verdübelt auf zwei Streifenfundamenten, deren Oberkanten bündig mit einer wassergebundenen Decke liegen. Ihr Gebrauch zeigt sich in fünf abgeplatzten Kanten, der an beiden Enden durchgescheuerten ausgeblichenen grünen Beschichtung, unterbrochenen weißen Linien, Kaugummi, einer Mäherabplatzung, Algen, einem Graffiti-Tag, einer verbogenen Netzkante und den in den Kies getretenen Mulden, wo die Spieler stehen; der Bauhof hebt sie an ihren vier Transportankern mit dem Kran aus, und zwei Spieler stützen sich auf ihre Enden."),
        "tags": ["used", "park", "public-space", "sport", "play", "table-tennis", "precast-concrete", "concrete", "steel", "street-furniture"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(f"Modelled to the ITTF table (2.74 x 1.525 m, 0.76 m, net 152.5 mm) and a catalogue-typical German precast outdoor table: 60 mm C35/45 plate cast with two 120 mm foot slabs (1.10 m at the base, 0.90 m under the plate, 50 mm haunches) at +-0.78 m, 6 mm perforated net (Rv 25 at 50 mm) welded to 80 x 10 mm posts bolted to the plate edges with M12, four spherical-head lifting anchors over the feet, two 20 mm stainless dowels per foot into two C20/25 strip footings with flush tops (1.30 x 0.40 x 0.80 m, not part of the asset). Masses from the modelled volumes (reinforced concrete 2500, steel 7850 kg/m3): table {M['table']:.1f} kg, feet {M['foot-a']:.1f} and {M['foot-b']:.1f} kg, net {M['net']:.2f} kg, total {MASS_TOTAL:.1f} kg. All parameters are literature values or authored estimates; nothing is measured.", f"Modelliert nach dem ITTF-Tisch (2,74 x 1,525 m, 0,76 m, Netz 152,5 mm) und einer katalogtypischen deutschen Fertigteil-Außenplatte: 60-mm-Platte C35/45 mit zwei 120-mm-Fußscheiben (1,10 m am Fuß, 0,90 m unter der Platte, 50-mm-Vouten) bei +-0,78 m gegossen, 6-mm-Lochblechnetz (Rv 25 im Abstand 50 mm) an Pfosten 80 x 10 mm geschweißt, die mit M12 an die Plattenkanten geschraubt sind, vier Kugelkopf-Transportanker über den Füßen, je Fuß zwei 20-mm-Edelstahldorne in zwei Streifenfundamente C20/25 mit bündigen Oberkanten (1,30 x 0,40 x 0,80 m, nicht Teil des Assets). Massen aus den modellierten Volumen (Stahlbeton 2500, Stahl 7850 kg/m3): Platte {M['table']:.1f} kg, Füße {M['foot-a']:.1f} und {M['foot-b']:.1f} kg, Netz {M['net']:.2f} kg, gesamt {MASS_TOTAL:.1f} kg. Alle Parameter sind Literaturwerte oder Schätzwerte; nichts ist gemessen.")},
        "dimensions": {"heightM": round(TOP + NET_H + FOOTING_D, 4), "widthM": L, "depthM": round(2 * (HW + POST_T + BOLT_H), 4), "massKg": mass},
        "materials": materials(),
        "rig": rig(),
        "volumes": {str(s): volume(s) for s in SCALES},
        "anchors": anchors(),
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": symbols(),
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure(),
        "presentation": {"defaultPose": "in-service", "defaultClip": "rally", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print("[DEBUG] masses", {k: round(v, 2) for k, v in M.items() if k != "volumes"}, "total", round(MASS_TOTAL, 1))
    print("[DEBUG] areas", AREA)
