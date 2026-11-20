"""🪨 Round 3 root fixes of components/masonry-wall-with-lintel outside `volumes`: octametre opening 1.26 m, a constructively real lintel zone (inner and exposed outer precast lintel with 80 mm wool between) and sill (cast stone, 50 mm projection, drip, weathering) filling the full 445 mm wall,
masses and centres from the real build-up, materials mortar and tie steel, thermal layers of lintel and sill zones at their real thickness, field area, texts, references and the 2D drawings of every scale and view redrawn as the true views of that build-up.

Usage: python3 r3-masonry-wall-with-lintel-roots.py [--write]   (default: candidate under 🗑️generated/r3-masonry-wall-with-lintel/🔣️.json with the volumes source merged)
"""
import importlib.util
import json
import os
import sys

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-masonry-wall-with-lintel-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
t, r5, C = K.t, K.r5, K.C
CAP = 0.0045
PLAN_PLANE = 1.55

MATERIALS = [
    {"id": "mortar", "title": t("Lime-cement masonry mortar", "Kalkzement-Mauermörtel"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 1800, "conductivityWmK": 1.0, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "tie-steel", "title": t("Stainless steel wall tie wire 4 mm", "Edelstahl-Drahtanker 4 mm"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7900, "conductivityWmK": 17, "specificHeatJKgK": 500, "elasticModulusPa": 200000000000, "strengthPa": 500000000, "source": "literature", "reference": "en-iso-10456"},
]

REFERENCES = [
    {"id": "din-en-1996-2-na", "citation": "DIN EN 1996-2/NA:2012-01, national annex to Eurocode 6 part 2: two-leaf walls with full-fill cavity insulation, clear cavity at most 150 mm, outer leaf at least 90 mm, at least 7 wall ties of 4 mm stainless wire per m2 plus 3 per metre along free edges of openings, open head joints for drainage at the foot of the outer leaf and above openings."},
    {"id": "en-771-1", "citation": "EN 771-1:2011+A1:2015 Specification for masonry units, clay masonry units; with DIN 105-100: normal format NF 240 x 115 x 71 mm."},
    {"id": "en-771-2", "citation": "EN 771-2:2011+A1:2015 Specification for masonry units, calcium silicate masonry units; with DIN 20000-402: element height 248 mm laid in 2 mm thin-bed mortar (course 0.25 m)."},
    {"id": "en-845-1", "citation": "EN 845-1:2013+A1:2016 Specification for ancillary components for masonry, wall ties: stainless steel wire ties of 4 mm diameter."},
    {"id": "din-18500", "citation": "DIN 18500:2006 Cast stones: precast window sills with weathering of at least 5 degrees, a projection of 40 to 50 mm beyond the face, a drip groove on the underside and raised ends let into the reveals."},
]

THERMAL_LAYERS = {
    "lintel": {
        "lintel-shell": {"title": t("Exposed precast concrete outer lintel 175 mm", "Sichtbeton-Außensturz 175 mm"), "resistanceM2KW": 0.07609, "thicknessM": 0.175, "heatCapacityJM2K": 420000},
    },
    "sill": {
        "sill-concrete": {"title": t("Cast-stone sill 175 mm in the wall plane", "Betonwerkstein-Fensterbank 175 mm in der Wandebene"), "resistanceM2KW": 0.07609, "thicknessM": 0.175, "heatCapacityJM2K": 420000},
    },
}


def stroke(id_, pts, bone="wall", role="texture", region="masonry-field", tone=0.3, z=1.2):
    """✏️ An open stroke."""
    return {"id": id_, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "region": region, "kind": "poly", "points": [[r5(x), r5(y)] for x, y in pts], "closed": False}


def strokes(id_, segs, role="texture", region="masonry-field", tone=0.3, z=1.2, bone="wall"):
    """✏️ Many open segments as one path."""
    d = " ".join(f"M {r5(a[0])} {r5(a[1])} L {r5(b[0])} {r5(b[1])}" for a, b in segs)
    return {"id": id_, "bone": bone, "role": role, "z": z, "filled": False, "tone": tone, "region": region, "kind": "path", "d": d}


def fill(id_, pts, bone, region, material, tone, z=1, role="body", hatch=None, regions=None, pen=None):
    """▰ A filled closed outline."""
    out = {"id": id_, "bone": bone, "role": role, "z": z, "filled": True, "tone": tone}
    if regions:
        out["regions"] = regions
    else:
        out["region"] = region
    out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    out.update({"kind": "poly", "points": [[r5(x), r5(y)] for x, y in pts], "closed": True})
    return out


def path(id_, rings, bone, region, material, tone, z=1, hatch=None, pen=None, regions=None):
    """▰ A filled outline with holes."""
    out = {"id": id_, "bone": bone, "role": "body", "z": z, "filled": True, "tone": tone}
    if regions:
        out["regions"] = regions
    else:
        out["region"] = region
    out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen:
        out["penMm"] = pen
    out["kind"] = "path"
    out["d"] = " ".join("M " + " L ".join(f"{r5(x)} {r5(y)}" for x, y in ring) + " Z" for ring in rings)
    return out


def R(x0, y0, x1, y1):
    return K.rect(x0, y0, x1, y1)


def zigzag(id_, a, b, lo, hi, along_y, region="masonry-field", pitch=0.04):
    """〰️ The insulation zigzag across lo .. hi, running from a to b."""
    n = max(2, round((b - a) / pitch))
    step = (b - a) / n
    pts = []
    for i in range(n + 1):
        c = lo if i % 2 == 0 else hi
        s = a + i * step
        pts.append((c, s) if along_y else (s, c))
    return stroke(id_, pts, region=region)


def ground(scale):
    off = {"50": 0.0, "100": 0.0, "200": 0.0, "500": 0.0}[scale]
    return {"id": "ground", "bone": "wall", "role": "ground", "z": 0, "filled": False, "tone": 0.5, "kind": "poly", "points": [[off, 0], [K.LENGTH + off, 0]], "closed": False}


def elevation50(pen):
    """🧱 1:50 elevation: the face with every bed and head joint of the stretcher bond, the weep joints, the exposed lintel over its bed, the sill with its nose line and the soffit seen through the opening."""
    out = [ground("50"), path("field", [K.WALL, K.FACING_HOLE_50], "wall", "masonry-field", "brick", 0.85, pen=pen),
           fill("lintel", R(0, K.BED, K.LINTEL_LEN, 0.25), "lintel", "lintel", "concrete", 0.35, z=2, hatch="concrete-stipple", pen=pen),
           fill("sill", R(0, K.BED, K.SILL_LEN, C), "sill", "sill", "concrete", 0.35, z=2, hatch="concrete-stipple", pen=pen),
           fill("soffit", R(K.OPEN[0], K.SOFFIT_PLASTER, K.OPEN[1], K.OUTER_SOFFIT), "wall", "masonry-field", "plaster", 0.86, z=1.5, role="detail"),
           stroke("sill-nose", [(CAP, K.V_NOSE), (K.SILL_LEN - CAP, K.V_NOSE)], bone="sill", role="detail", region="sill", z=2.5)]
    for k in range(1, 33):
        y = k * C + K.BED / 2
        segs = [((a + CAP, y), (b - CAP, y)) for a, b in K.bed_runs(k)]
        out.append(strokes(f"bed-{k}", segs))
    for k in range(33):
        y0, y1 = k * C + K.BED, (k + 1) * C
        segs, weeps = [], []
        for kind, a, b in K.course_pieces(k):
            if kind == "head" and abs(b - a - K.HEAD) < 1e-9 and a > 1e-9 and b < K.LENGTH - 1e-9:
                touches = any(abs(a - e) < 1e-6 or abs(b - e) < 1e-6 for e in (K.LINTEL_X[0], K.LINTEL_X[1], K.SILL_X[0], K.SILL_X[1]))
                if not touches:
                    x = (a + b) / 2
                    segs.append(((x, y0 + CAP / 2), (x, y1 - CAP / 2)))
            if kind == "weep":
                weeps.append((a, b))
        if segs:
            out.append(strokes(f"heads-{k}", segs))
        for i, (a, b) in enumerate(weeps):
            out.append(fill(f"weep-{k}-{i}", R(a, y0, b, y1), "wall", "masonry-field", "mortar", 0.1, z=1.3, role="detail"))
    return out


def front50():
    """✂️ 1:50 section A-A at x = 1.25 m: every leaf cut, sill and both lintels in their real profiles, the reveal beyond with plaster return and the raised sill end."""
    W = K
    sill = [(w, v) for v, w in K.sill_profile()]
    lin = [(w, v) for v, w in K.outer_lintel_profile()]
    return [
        fill("reveal", [(W.EXT, W.PARAPET), (W.INT, W.PARAPET), (W.INT, W.SOFFIT_PLASTER), (W.LEAF, W.SOFFIT_PLASTER), (W.LEAF, W.OUTER_SOFFIT), (W.EXT, W.OUTER_SOFFIT)], "wall", "masonry-field", "brick", 0.85, z=0.5),
        fill("sill-end-beyond", [(W.EXT, W.SILL_Y + W.V_FACE), (W.EXT, W.PARAPET), (W.SEAT_W, W.PARAPET)], "sill", "sill", "concrete", 0.55, z=0.6, role="detail"),
        fill("lower-facing", R(W.EXT, 0, W.FILL, W.SILL_Y + W.BED), "wall", "masonry-field", "brick", 0.55, hatch="cut-brick"),
        fill("lower-fill", R(W.FILL, 0, W.LEAF, W.SILL_Y), "wall", "masonry-field", "wool", 0.9),
        fill("lower-block", R(W.LEAF, 0, W.PLASTER, W.PARAPET), "wall", "masonry-field", "block", 0.7, hatch="cut-block"),
        fill("lower-plaster", R(W.PLASTER, 0, W.INT, W.PARAPET), "wall", "masonry-field", "plaster", 0.85),
        fill("sill-wool", R(W.TONGUE, W.SILL_Y, W.LEAF, W.PARAPET), "wall", "sill", "wool", 0.9),
        fill("sill", sill, "sill", "sill", "concrete", 0.3, z=2, hatch="concrete-stipple"),
        fill("lintel-outer", lin, "lintel", "lintel", "concrete", 0.3, z=2, hatch="concrete-stipple"),
        fill("lintel-wool", R(W.TONGUE, W.OUTER_SOFFIT, W.LEAF, W.LINTEL_TOP), "wall", "lintel", "wool", 0.9),
        fill("lintel-inner", R(W.LEAF, W.LINTEL_Y, W.PLASTER, W.LINTEL_TOP), "lintel", "lintel", "concrete", 0.3, z=2, hatch="concrete-stipple"),
        fill("upper-plaster", [(W.LEAF, W.SOFFIT_PLASTER), (W.INT, W.SOFFIT_PLASTER), (W.INT, W.HEIGHT), (W.PLASTER, W.HEIGHT), (W.PLASTER, W.LINTEL_Y), (W.LEAF, W.LINTEL_Y)], "wall", "masonry-field", "plaster", 0.85),
        fill("upper-facing", R(W.EXT, W.LINTEL_TOP, W.FILL, W.HEIGHT), "wall", "masonry-field", "brick", 0.55, hatch="cut-brick"),
        fill("upper-fill", R(W.FILL, W.LINTEL_TOP, W.LEAF, W.HEIGHT), "wall", "masonry-field", "wool", 0.9),
        fill("upper-block", R(W.LEAF, W.LINTEL_TOP, W.PLASTER, W.HEIGHT), "wall", "masonry-field", "block", 0.7, hatch="cut-block"),
        zigzag("lower-fill-zigzag", 0.02, W.SILL_Y - 0.02, W.FILL + 0.01, W.LEAF - 0.01, True),
        zigzag("sill-wool-zigzag", W.SILL_Y + 0.01, W.PARAPET - 0.01, W.TONGUE + 0.01, W.LEAF - 0.01, True, region="sill", pitch=0.03),
        zigzag("lintel-wool-zigzag", W.OUTER_SOFFIT + 0.02, W.LINTEL_TOP - 0.02, W.TONGUE + 0.01, W.LEAF - 0.01, True, region="lintel", pitch=0.03),
        zigzag("upper-fill-zigzag", W.LINTEL_TOP + 0.02, W.HEIGHT - 0.02, W.FILL + 0.01, W.LEAF - 0.01, True),
    ]


def plan_piers(scale, block_open):
    """✂️ The two cut piers of section B-B, leaf by leaf."""
    W = K
    out = []
    for side, (a, b, ka, kb) in (("left", (0.0, W.OPEN[0], 0.0, block_open[0])), ("right", (W.OPEN[1], W.LENGTH, block_open[1], W.LENGTH))):
        out += [fill(f"pier-{side}-facing", R(a, W.EXT, b, W.FILL), "wall", "masonry-field", "brick", 0.55, hatch="cut-brick"),
                fill(f"pier-{side}-fill", R(a, W.FILL, b, W.LEAF), "wall", "masonry-field", "wool", 0.9),
                fill(f"pier-{side}-block", R(ka, W.LEAF, kb, W.PLASTER), "wall", "masonry-field", "block", 0.7, hatch="cut-block" if scale in ("50", "100") else None)]
        if scale == "50":
            if side == "left":
                pl = [(a, W.PLASTER), (kb, W.PLASTER), (kb, W.LEAF), (b, W.LEAF), (b, W.INT), (a, W.INT)]
            else:
                pl = [(a, W.LEAF), (ka, W.LEAF), (ka, W.PLASTER), (b, W.PLASTER), (b, W.INT), (a, W.INT)]
            out.append(fill(f"pier-{side}-plaster", pl, "wall", "masonry-field", "plaster", 0.85))
        else:
            out.append(fill(f"pier-{side}-plaster", R(a, W.PLASTER, b, W.INT), "wall", "masonry-field", "plaster", 0.85))
        out.append(zigzag(f"pier-{side}-fill-zigzag", a + 0.03, b - 0.03, W.FILL + 0.01, W.LEAF - 0.01, False))
    return out


def sill_plan_outline():
    W = K
    return [(W.SILL_X[0], W.NOSE), (W.SILL_X[1], W.NOSE), (W.SILL_X[1], W.EXT), (W.OPEN[1], W.EXT), (W.OPEN[1], W.TONGUE), (W.OPEN[0], W.TONGUE), (W.OPEN[0], W.EXT), (W.SILL_X[0], W.EXT)]


def plan50():
    """✂️ 1:50 section B-B at y = 1.55 m (mid brick of course 18): cut piers with head joints and block joints, plaster returns, the sill, wool, parapet and plaster below the cut."""
    W = K
    out = plan_piers("50", W.KS_OPEN)
    k = int(PLAN_PLANE / C)
    heads = [((a + b) / 2) for kind, a, b in W.course_pieces(k) if kind == "head" and abs(b - a - W.HEAD) < 1e-9]
    out.append(strokes("plan-head-joints", [((x, W.EXT + CAP), (x, W.FILL - CAP)) for x in heads]))
    j = int(PLAN_PLANE / W.KS_H)
    g = 0.0 if j % 2 == 0 else 0.125
    seams = [g + i * W.KS_L for i in range(1, 13) if 0 < g + i * W.KS_L < W.LENGTH and (g + i * W.KS_L < W.KS_OPEN[0] - 1e-9 or g + i * W.KS_L > W.KS_OPEN[1] + 1e-9)]
    out.append(strokes("plan-block-joints", [((x, W.LEAF + CAP), (x, W.PLASTER - CAP)) for x in seams]))
    out += [fill("sill-below", sill_plan_outline(), "sill", "sill", "concrete", 0.45, z=0.5),
            stroke("sill-seat-edge", [(W.OPEN[0] + CAP, W.SEAT_W), (W.OPEN[1] - CAP, W.SEAT_W)], bone="sill", role="detail", region="sill", z=0.6),
            fill("sill-wool-below", R(W.OPEN[0], W.TONGUE, W.OPEN[1], W.LEAF), "wall", "sill", "wool", 0.9, z=0.5),
            fill("parapet-below", R(W.OPEN[0], W.LEAF, W.OPEN[1], W.PLASTER), "wall", "sill", "block", 0.7, z=0.5),
            fill("parapet-plaster-below", R(W.OPEN[0], W.PLASTER, W.OPEN[1], W.INT), "wall", "sill", "plaster", 0.85, z=0.5)]
    return out


def elevation_coarse(scale, pen, hatch):
    W = K
    out = [ground(scale), path("field", [W.WALL, W.FIELD_HOLE], "wall", "masonry-field", "brick", 0.8 if scale == "100" else 0.55, hatch=hatch, pen=pen),
           fill("lintel", R(0, 0, W.LINTEL_LEN, 0.25), "lintel", "lintel", "concrete", 0.35, z=2, hatch="concrete-stipple" if scale == "100" else None, pen=pen),
           fill("sill", R(0, 0, W.SILL_LEN, C), "sill", "sill", "concrete", 0.35, z=2, hatch="concrete-stipple" if scale == "100" else None, pen=pen)]
    return out


def front100():
    W = K
    sill = [(w, v) for v, w in K.sill_profile_100()]
    return [
        fill("reveal", R(W.EXT, W.PARAPET, W.INT, W.LINTEL_Y), "wall", "masonry-field", "brick", 0.85, z=0.5),
        fill("lower-facing", R(W.EXT, 0, W.FILL, W.SILL_Y), "wall", "masonry-field", "brick", 0.55, hatch="cut-brick"),
        fill("lower-fill", R(W.FILL, 0, W.LEAF, W.SILL_Y), "wall", "masonry-field", "wool", 0.9),
        fill("lower-block", R(W.LEAF, 0, W.PLASTER, W.PARAPET), "wall", "masonry-field", "block", 0.7, hatch="cut-block"),
        fill("lower-plaster", R(W.PLASTER, 0, W.INT, W.PARAPET), "wall", "masonry-field", "plaster", 0.85),
        fill("sill-wool", R(W.TONGUE, W.SILL_Y, W.LEAF, W.PARAPET), "wall", "sill", "wool", 0.9),
        fill("sill", sill, "sill", "sill", "concrete", 0.3, z=2, hatch="concrete-stipple"),
        fill("lintel-outer", R(W.EXT, W.LINTEL_Y, W.TONGUE, W.LINTEL_TOP), "lintel", "lintel", "concrete", 0.3, z=2, hatch="concrete-stipple"),
        fill("lintel-wool", R(W.TONGUE, W.LINTEL_Y, W.LEAF, W.LINTEL_TOP), "wall", "lintel", "wool", 0.9),
        fill("lintel-inner", R(W.LEAF, W.LINTEL_Y, W.PLASTER, W.LINTEL_TOP), "lintel", "lintel", "concrete", 0.3, z=2, hatch="concrete-stipple"),
        fill("upper-plaster", R(W.PLASTER, W.LINTEL_Y, W.INT, W.HEIGHT), "wall", "masonry-field", "plaster", 0.85),
        fill("upper-facing", R(W.EXT, W.LINTEL_TOP, W.FILL, W.HEIGHT), "wall", "masonry-field", "brick", 0.55, hatch="cut-brick"),
        fill("upper-fill", R(W.FILL, W.LINTEL_TOP, W.LEAF, W.HEIGHT), "wall", "masonry-field", "wool", 0.9),
        fill("upper-block", R(W.LEAF, W.LINTEL_TOP, W.PLASTER, W.HEIGHT), "wall", "masonry-field", "block", 0.7, hatch="cut-block"),
    ]


def plan100():
    W = K
    return plan_piers("100", W.OPEN) + [fill("sill-below", sill_plan_outline(), "sill", "sill", "concrete", 0.45, z=0.5),
                                         fill("parapet-below", R(W.OPEN[0], W.TONGUE, W.OPEN[1], W.INT), "wall", "sill", "block", 0.7, z=0.5)]


def front200():
    W = K
    return [
        fill("reveal", R(W.EXT, W.PARAPET, W.INT, W.LINTEL_Y), "wall", "masonry-field", "brick", 0.8, z=0.5),
        fill("field-lower", R(W.EXT, 0, W.INT, W.SILL_Y), "wall", "masonry-field", "brick", 0.55),
        fill("sill", R(W.NOSE, W.SILL_Y, W.TONGUE, W.PARAPET), "sill", "sill", "concrete", 0.35, z=2),
        fill("parapet-cap", R(W.TONGUE, W.SILL_Y, W.INT, W.PARAPET), "wall", "sill", "block", 0.55),
        fill("lintel", R(W.EXT, W.LINTEL_Y, W.INT, W.LINTEL_TOP), "lintel", "lintel", "concrete", 0.35, z=2),
        fill("field-upper", R(W.EXT, W.LINTEL_TOP, W.INT, W.HEIGHT), "wall", "masonry-field", "brick", 0.55),
    ]


def plan200():
    W = K
    return [fill("pier-left", R(0, W.EXT, W.OPEN[0], W.INT), "wall", "masonry-field", "brick", 0.55),
            fill("pier-right", R(W.OPEN[1], W.EXT, W.LENGTH, W.INT), "wall", "masonry-field", "brick", 0.55),
            fill("sill-below", R(W.SILL_X[0], W.NOSE, W.SILL_X[1], W.TONGUE), "sill", "sill", "concrete", 0.35, z=0.5),
            fill("parapet-below", R(W.OPEN[0], W.TONGUE, W.OPEN[1], W.INT), "wall", "sill", "block", 0.55, z=0.5)]


def elevation500(pen):
    W = K
    return [ground("500"), path("field", [W.WALL, W.OPENING], "wall", None, "brick", 0.55, regions=["masonry-field", "sill"], pen=pen),
            fill("lintel", R(0, 0, W.LINTEL_LEN, 0.25), "lintel", "lintel", "concrete", 0.3, z=2, pen=pen)]


def front500():
    W = K
    return [fill("reveal", R(W.EXT, W.PARAPET, W.INT, W.LINTEL_Y), "wall", "masonry-field", "brick", 0.8, z=0.5),
            fill("field-lower", R(W.EXT, 0, W.INT, W.SILL_Y), "wall", "masonry-field", "brick", 0.55),
            fill("sill-band", R(W.NOSE, W.SILL_Y, W.INT, W.PARAPET), "sill", "sill", "concrete", 0.45, z=2),
            fill("field-upper", R(W.EXT, W.LINTEL_Y, W.INT, W.HEIGHT), "wall", None, "brick", 0.45, regions=["masonry-field", "lintel"])]


def plan500():
    W = K
    return [fill("pier-left", R(0, W.EXT, W.OPEN[0], W.INT), "wall", "masonry-field", "brick", 0.55),
            fill("pier-right", R(W.OPEN[1], W.EXT, W.LENGTH, W.INT), "wall", "masonry-field", "brick", 0.55),
            fill("sill-below", R(W.SILL_X[0], W.NOSE, W.SILL_X[1], W.INT), "sill", "sill", "concrete", 0.3, z=0.5)]


CONVENTION = t(
    "Convention: the elevation is the exterior face of the wall, seen from outside; the front view is the vertical section A-A at x = 1.25 m through the opening, seen from ahead (lateral x is the asset's z: exterior at the left, x = -0.2225 m, the sill nose at -0.2725 m, interior at +0.2225 m) with the reveal beyond the cut; the plan view is the horizontal section B-B at y = 1.55 m (mid brick of course 18) seen from above (lateral y is z: exterior at the bottom of the sheet) with the sill and the parapet below the cut.",
    "Konvention: die Ansicht ist die Außenfläche der Wand, von außen gesehen; die Vorderansicht ist der Vertikalschnitt A-A bei x = 1,25 m durch die Öffnung, von vorn gesehen (seitliches x ist z des Assets: außen links bei x = -0,2225 m, die Fensterbanknase bei -0,2725 m, innen bei +0,2225 m) mit der Laibung hinter dem Schnitt; die Draufsicht ist der Horizontalschnitt B-B bei y = 1,55 m (Steinmitte der 18. Schicht) von oben gesehen (seitliches y ist z: außen unten auf dem Blatt) mit Fensterbank und Brüstung unter dem Schnitt.")
OVERLAYS = t("Cells: wall field, lintel zone, sill zone (the three regions of the EN ISO 6946 calculation).", "Zellen: Wandfeld, Sturzzone, Fensterbankzone (die drei Regionen der Berechnung nach EN ISO 6946).")


def joined(a, b):
    return t(a["en"] + " " + b["en"], a["de"] + " " + b["de"])


DECISIONS = {
    "50": {
        "silhouette": joined(t("The two-leaf wall in its real construction: the facing leaf in stretcher bond around a 1.26 x 1.333 m opening, the exposed precast lintel over it and the cast-stone sill under it.",
                               "Die zweischalige Wand in ihrer wirklichen Konstruktion: die Vormauerschale im Läuferverband um eine Öffnung von 1,26 x 1,333 m, der sichtbare Fertigteilsturz darüber und die Betonwerkstein-Fensterbank darunter."), CONVENTION),
        "retained": t("Every bed joint (33 courses of 1/12 m) and every head joint of the half bond, the open head joints in the bottom course and above the lintel, the lintel over its 12.3 mm bed, the sill nose and the interior soffit seen through the opening. Section A-A: facing, wool, calcium silicate leaf and plaster cut below and above the opening; the sill profile with projection, drip groove, weathering and seat; the outer lintel with its chamfer, the 80 mm wool and the inner lintel; plaster on the soffit; the reveal and the raised sill end beyond. Section B-B: the cut course with its head joints, the cut block course with its joints, the plaster returns into the reveals, the sill, the wool and the parapet below.",
                      "Jede Lagerfuge (33 Schichten zu 1/12 m) und jede Stoßfuge des halben Verbands, die offenen Stoßfugen in der untersten Schicht und über dem Sturz, der Sturz auf seinem 12,3-mm-Mörtelbett, die Fensterbanknase und die innere Sturzuntersicht durch die Öffnung gesehen. Schnitt A-A: Vormauerung, Dämmung, Kalksandsteinschale und Putz unter und über der Öffnung geschnitten; das Fensterbankprofil mit Überstand, Tropfnase, Gefälle und Anschlag; der Außensturz mit seiner Fase, die 80 mm Dämmung und der Innensturz; Putz an der Untersicht; die Laibung und das hochgezogene Fensterbankende dahinter. Schnitt B-B: die geschnittene Schicht mit ihren Stoßfugen, die geschnittene Steinschicht mit ihren Fugen, die Putzumläufe in die Laibungen, Fensterbank, Dämmung und Brüstung darunter."),
        "merged": t("None.", "Keine."),
        "omitted": t("The wall ties (4 mm, inside the cavity), the damp-proof courses at the foot and over the lintel (1 to 2 mm), the 2 mm thin-bed joints of the calcium silicate leaf in the views, the window (not part of this asset).",
                     "Die Luftschichtanker (4 mm, im Hohlraum), die Sperrbahnen am Fuß und über dem Sturz (1 bis 2 mm), die 2-mm-Dünnbettfugen der Kalksandsteinschale in den Ansichten, das Fenster (nicht Teil dieses Assets)."),
        "thinElements": t("Joints are 0.1 mm lines on their centre lines (a 10 mm head joint is 0.2 mm, a 12.3 mm bed joint 0.25 mm on paper); the 15 mm plaster is 0.3 mm, the soffit strip 0.5 mm.",
                          "Fugen sind 0,1-mm-Linien auf ihren Mittellinien (eine 10-mm-Stoßfuge ist auf dem Papier 0,2 mm, eine 12,3-mm-Lagerfuge 0,25 mm); der 15-mm-Putz ist 0,3 mm, der Untersichtstreifen 0,5 mm."),
        "hatching": t("Cut facing brick 45 degree lines at 0.8 mm, cut calcium silicate 45 degree lines at 1.6 mm (DIN ISO 128-50), cut and seen precast concrete stipple, wool as zigzag line; the brick courses of the elevation are explicit joints, not a hatch.",
                      "Geschnittener Verblender 45-Grad-Linien mit 0,8 mm, geschnittener Kalksandstein 45-Grad-Linien mit 1,6 mm (DIN ISO 128-50), geschnittener und gesehener Beton punktiert, Dämmung als Zickzacklinie; die Ziegelschichten der Ansicht sind ausgezeichnete Fugen, keine Schraffur."),
        "styles": t("Outline 0.35 mm, details 0.18 mm, joints 0.1 mm.", "Kontur 0,35 mm, Einzelheiten 0,18 mm, Fugen 0,1 mm."),
        "overlays": OVERLAYS,
    },
    "100": {
        "silhouette": joined(t("The wall with the coursing as a hatch, the exposed lintel and the sill.", "Die Wand mit der Schichtung als Schraffur, der sichtbare Sturz und die Fensterbank."), CONVENTION),
        "retained": t("Course height as the hatch spacing, lintel 1.75 x 0.25 m, sill 1.4 x 0.083 m with its 50 mm projection and weathering; in both sections the four leaves, the two lintels with the 80 mm wool between them and the sill.",
                      "Schichthöhe als Schraffurabstand, Sturz 1,75 x 0,25 m, Fensterbank 1,4 x 0,083 m mit 50 mm Überstand und Gefälle; in beiden Schnitten die vier Schalen, die zwei Stürze mit den 80 mm Dämmung dazwischen und die Fensterbank."),
        "merged": t("The 12.3 mm bed under the outer lintel into the lintel, the reveal and soffit plaster into the opening outline, the drip groove and the raised sill ends into one sill profile.",
                    "Das 12,3-mm-Mörtelbett unter dem Außensturz in den Sturz, Laibungs- und Untersichtputz in die Öffnungskontur, Tropfnase und hochgezogene Fensterbankenden in ein Fensterbankprofil."),
        "omitted": t("Head joints and weep joints (0.1 mm wide on paper), the lintel chamfer, wall ties, damp-proof courses, block joints.",
                     "Stoßfugen und offene Stoßfugen (auf dem Papier 0,1 mm breit), die Sturzfase, Luftschichtanker, Sperrbahnen, Steinfugen."),
        "thinElements": t("The 15 mm plaster is drawn at 0.15 mm under the body pen of the leaf boundary.", "Der 15-mm-Putz ist mit 0,15 mm unter dem Körperstift der Schalengrenze gezeichnet."),
        "hatching": t("Coursing lines at 0.83 mm in the elevation; cut facing brick 45 degree lines at 0.5 mm, cut calcium silicate at 1.0 mm; precast concrete stipple; wool by tone.",
                      "Schichtlinien mit 0,83 mm in der Ansicht; geschnittener Verblender 45-Grad-Linien mit 0,5 mm, geschnittener Kalksandstein mit 1,0 mm; Beton punktiert; Dämmung durch Ton."),
        "styles": t("Outline 0.25 mm, hatches 0.1 mm.", "Kontur 0,25 mm, Schraffuren 0,1 mm."),
        "overlays": OVERLAYS,
    },
    "200": {
        "silhouette": joined(t("The wall mass with the opening, the lintel band (1.25 mm) and the sill band (0.42 mm) as separate filled shapes.", "Die Wandmasse mit der Öffnung, das Sturzband (1,25 mm) und das Fensterbankband (0,42 mm) als eigene gefüllte Formen."), CONVENTION),
        "retained": t("Lintel with its bearings, sill with its reveal ends and its 50 mm (0.25 mm) projection, opening; in the sections the wall as one cut mass, the lintel band and the sill with the parapet cap behind it.",
                      "Sturz mit Auflagern, Fensterbank mit ihren Enden in den Laibungen und ihrem Überstand von 50 mm (0,25 mm), Öffnung; in den Schnitten die Wand als eine geschnittene Masse, das Sturzband und die Fensterbank mit der Brüstungsabdeckung dahinter."),
        "merged": t("The four leaves into one wall mass, both lintels and the wool between them into one lintel band (the leaves are 0.08 to 0.9 mm on paper and a 1:200 wall is drawn as one poché).",
                    "Die vier Schalen zu einer Wandmasse, beide Stürze und die Dämmung dazwischen zu einem Sturzband (die Schalen sind auf dem Papier 0,08 bis 0,9 mm breit, eine Wand 1:200 wird als eine Füllung gezeichnet)."),
        "omitted": t("Coursing, joints, hatches, the sill weathering.", "Schichtung, Fugen, Schraffuren, das Fensterbankgefälle."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("None: tone only.", "Keine: nur Ton."),
        "styles": t("Wall mid-grey, lintel and sill darker.", "Wand mittelgrau, Sturz und Fensterbank dunkler."),
        "overlays": OVERLAYS,
    },
    "500": {
        "silhouette": joined(t("A 3.0 x 2.75 m wall mass with the 1.26 x 1.33 m opening and the 0.25 m lintel as a darker band above it.", "Eine 3,0 x 2,75 m große Wandmasse mit der Öffnung 1,26 x 1,33 m und dem 0,25 m hohen Sturz als dunklerem Band darüber."), CONVENTION),
        "retained": t("Wall outline, opening, the lintel with its 0.245 m bearings (1.75 m long, 0.5 mm high).", "Wandumriss, Öffnung, der Sturz mit 0,245 m Auflager beidseits (1,75 m lang, 0,5 mm hoch)."),
        "merged": t("In the elevation the wall field and the 0.083 m sill zone (0.17 mm) are one mass listing both regions; in section the lintel zone joins the wall above it, and the sill zone is one band through the wall whose 50 mm nose (0.1 mm) keeps the true depth of the wall.", "In der Ansicht sind das Wandfeld und die 0,083 m hohe Fensterbankzone (0,17 mm) eine Masse mit beiden Regionen; im Schnitt geht die Sturzzone in der Wand darüber auf, und die Fensterbankzone ist ein Band durch die Wand, dessen Nase von 50 mm (0,1 mm) die wirkliche Tiefe der Wand hält."),
        "omitted": t("Coursing, joints, the sill weathering and drip, hatching, all layers.", "Schichtung, Fugen, Gefälle und Tropfnase der Fensterbank, Schraffur, alle Schichten."),
        "thinElements": t("None: the lintel is drawn at its real 0.25 m height.", "Keine: der Sturz ist in seiner wirklichen Höhe von 0,25 m gezeichnet."),
        "hatching": t("None.", "Keine."),
        "styles": t("Mid-grey wall mass with 0.13 mm outline; lintel darker.", "Mittelgraue Wandmasse mit 0,13-mm-Kontur; Sturz dunkler."),
        "overlays": OVERLAYS,
    },
}

SUMMARY = t(
    "A 3.0 m wide, 2.75 m high two-leaf wall with full-fill cavity insulation (15 mm plaster, 175 mm calcium silicate, 140 mm mineral wool, 115 mm facing brick in stretcher bond, U about 0.22 W/m2K) seen from outside, with a 1.26 x 1.33 m opening; over it an inner precast concrete lintel and an exposed outer precast lintel, 1.75 m long with 0.245 m bearings and 80 mm wool between them (U about 0.38), under it a cast-stone sill with 50 mm projection, drip and weathering (U about 0.36); wall ties, weep joints and brick courses of 0.0833 m drawn where printable; floor slab and beam loads with arching over the opening.",
    "Eine 3,0 m breite, 2,75 m hohe zweischalige Wand mit Kerndämmung (15 mm Putz, 175 mm Kalksandstein, 140 mm Mineralwolle, 115 mm Verblender im Läuferverband, U etwa 0.22 W/m2K) von außen gesehen, mit einer Öffnung von 1,26 x 1,33 m; darüber ein innerer Betonfertigsturz und ein sichtbarer äußerer Fertigteilsturz, 1,75 m lang mit 0,245 m Auflager und 80 mm Dämmung dazwischen (U etwa 0.38), darunter eine Betonwerkstein-Fensterbank mit 50 mm Überstand, Tropfnase und Gefälle (U etwa 0.36); Luftschichtanker, offene Stoßfugen und Ziegelschichten von 0,0833 m dort gezeichnet, wo sie druckbar sind; Decken- und Trägerlasten mit Gewölbewirkung über der Öffnung.")

NOTES = t(
    "Octametre dimensions (DIN 4172): 33 courses of 1/12 m, opening 1.26 m wide (n x 0.125 + 0.01, so the half bond closes at both reveals with whole and half bricks) from course 11 to 27, lintel courses 27 to 30, calcium silicate courses of 0.25 m. Layer resistances are d / lambda with design values; masses from the modelled volumes and the declared densities.",
    "Oktametermaße (DIN 4172): 33 Schichten zu 1/12 m, Öffnung 1,26 m breit (n x 0,125 + 0,01, damit der halbe Verband an beiden Laibungen mit ganzen und halben Steinen schließt) von Schicht 11 bis 27, Sturzschichten 27 bis 30, Kalksandsteinschichten zu 0,25 m. Schichtwiderstände sind d / lambda mit Bemessungswerten; Massen aus den modellierten Volumen und den erklärten Dichten.")

THERMAL_ASSUMPTIONS = [
    t("Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2; interior 20 degC with R_si 0.13. U of the field 0.22, of the lintel zone 0.38 (plaster, 175 mm inner lintel, 80 mm wool, 175 mm outer lintel), of the sill zone 0.36 (plaster, 175 mm calcium silicate, 80 mm wool, 175 mm sill) W/m2K.",
      "Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m2; Innenraum 20 °C mit R_si 0,13. U des Feldes 0.22, der Sturzzone 0.38 (Putz, 175 mm Innensturz, 80 mm Dämmung, 175 mm Außensturz), der Fensterbankzone 0.36 (Putz, 175 mm Kalksandstein, 80 mm Dämmung, 175 mm Fensterbank) W/m2K."),
    t("Lintel and sill zones are linear thermal bridges: the precast outer lintel and the sill reach 60 mm into the cavity and leave 80 mm of wool; they are modelled as separate layered strips (EN ISO 6946 layers, after EN ISO 10211), not by a 2D calculation.",
      "Sturz- und Fensterbankzone sind lineare Wärmebrücken: der Fertigteil-Außensturz und die Fensterbank greifen 60 mm in den Hohlraum und lassen 80 mm Dämmung; sie sind als eigene geschichtete Streifen modelliert (Schichten nach EN ISO 6946, nach EN ISO 10211), nicht durch eine 2D-Berechnung."),
    t("The opening is empty (no window modelled); its reveals belong to the window joint and are not regions. Sun and shade come from the pose sunlit fractions.",
      "Die Öffnung ist leer (kein Fenster modelliert); ihre Laibungen gehören zur Fensterfuge und sind keine Regionen. Sonne und Schatten kommen aus den Besonnungsanteilen der Posen."),
]


def apply(doc):
    """🪨 Every root fix, idempotent."""
    a = doc["asset"]
    body = K.body50()
    bm = K.bone_masses(body)
    total = sum(v[0] for v in bm.values())
    a["summary"] = SUMMARY
    prov = a["provenance"]
    prov["references"] = [r for r in prov["references"] if r["id"] not in {x["id"] for x in REFERENCES}] + REFERENCES
    for r in prov["references"]:
        if r["id"] == "en-1996-1-1":
            r["citation"] = "EN 1996-1-1:2005 Eurocode 6, design of masonry structures, part 1-1 (masonry density, lintel bearing length at least 0.1 m, arching above openings; the lintels here bear 0.245 m)."
        if r["id"] == "en-13225":
            r["citation"] = "EN 13225:2013 Precast concrete products, linear structural elements: precast lintels 175 mm wide, 1.75 m long, 0.25 m high (inner) and 0.24 m high over a 12 mm bed (outer, exposed)."
    prov["notes"] = NOTES
    a["dimensions"]["depthM"] = r5(K.INT - K.NOSE)
    a["dimensions"]["massKg"] = round(total, 3)
    a["materials"] = [m for m in a["materials"] if m["id"] not in {x["id"] for x in MATERIALS}] + MATERIALS
    for b in a["rig"]["bones"]:
        if b["id"] in bm:
            m, x, y, _ = bm[b["id"]]
            ox, oy = K.BONE_ORIGIN[b["id"]]
            b["mass"]["kg"] = round(m, 3)
            b["mass"]["comX"] = r5(x - ox)
            b["mass"]["comY"] = r5(y - oy)
        if b["id"] == "lintel":
            b["title"] = t("Precast concrete lintels, inner and exposed outer (1.75 m, 0.245 m bearing each side)", "Betonfertigstürze, innen und sichtbar außen (1,75 m, 0,245 m Auflager je Seite)")
        if b["id"] == "sill":
            b["title"] = t("Cast-stone window sill", "Betonwerkstein-Fensterbank")
    th = a["thermal"]
    for reg in th["regions"]:
        if reg["id"] == "masonry-field":
            reg["areaM2"] = r5(K.LENGTH * K.HEIGHT - (K.OPEN[1] - K.OPEN[0]) * (K.LINTEL_Y - K.PARAPET) - K.LINTEL_LEN * 0.25 - K.SILL_LEN * C)
        if reg["id"] == "lintel":
            reg["title"] = t("Lintel zone (inner and outer precast lintel)", "Sturzzone (innerer und äußerer Fertigteilsturz)")
        if reg["id"] == "sill":
            reg["title"] = t("Sill zone (cast-stone sill)", "Fensterbankzone (Betonwerkstein-Fensterbank)")
        for layer in reg["layers"]:
            fix = THERMAL_LAYERS.get(reg["id"], {}).get(layer["id"])
            if fix:
                for key in ("title", "resistanceM2KW", "thicknessM", "heatCapacityJM2K"):
                    layer[key] = fix[key]
        if reg["id"] in ("lintel", "sill"):
            ins = next(l for l in reg["layers"] if l["kind"] == "insulation")
            outer = reg["layers"][-1]
            reg["heatCapacityJM2K"] = outer["heatCapacityJM2K"] + ins["heatCapacityJM2K"]
    th["assumptions"] = THERMAL_ASSUMPTIONS
    rep = a["representations"]
    for s in ("50", "100", "200", "500"):
        rep[s]["sections"]["plan"]["plane"] = PLAN_PLANE
    pen = {s: rep[s]["pens"]["body"] for s in rep}
    for s, spacing in (("50", 1.6), ("100", 1.0)):
        hs = [h for h in rep[s]["hatches"] if h["id"] != "cut-block"]
        hs.append({"id": "cut-block", "title": t("Cut calcium silicate", "Geschnittener Kalksandstein"), "pattern": "lines", "angleDeg": 45, "spacingMm": spacing, "penMm": 0.1})
        rep[s]["hatches"] = hs
    rep["50"]["geometry"] = {"elevation": elevation50(pen["50"]), "front": front50(), "plan": plan50()}
    rep["100"]["geometry"] = {"elevation": elevation_coarse("100", pen["100"], "coursing"), "front": front100(), "plan": plan100()}
    rep["200"]["geometry"] = {"elevation": elevation_coarse("200", pen["200"], None), "front": front200(), "plan": plan200()}
    rep["500"]["geometry"] = {"elevation": elevation500(pen["500"]), "front": front500(), "plan": plan500()}
    for s, d in DECISIONS.items():
        assert set(d) == set(rep[s]["decisions"]), (s, set(d) ^ set(rep[s]["decisions"]))
        rep[s]["decisions"] = {k: d[k] for k in rep[s]["decisions"]}
    return doc


def main():
    write = "--write" in sys.argv
    doc = K.load(K.CATALOG)
    apply(doc)
    if write:
        text = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
        with open(K.CATALOG, "w", encoding="utf-8") as f:
            f.write(text)
        print("[DEBUG] wrote", K.CATALOG)
    else:
        os.makedirs(K.GEN, exist_ok=True)
        doc["asset"]["volumes"] = K.load(K.SOURCE)["volumes"]
        p = os.path.join(K.GEN, "🔣️.json")
        with open(p, "w", encoding="utf-8") as f:
            f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
        print(p)


if __name__ == "__main__":
    main()
