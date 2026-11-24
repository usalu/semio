"""🏗️ The one authoritative script that writes 🔣️.json of the reclaimed trapezoidal sheet stack from the numbers of author-geometry.py.

Run: python3 author-asset.py  (writes ./🔣️.json next to this script)
@see ./author-geometry.py
@see ../../brief.md
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("geometry", os.path.join(HERE, "author-geometry.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r6(x):
    """🔢 Rounded metres."""
    return round(x + 0.0, 6)


def pts(seq):
    """🔢 Rounded point list."""
    return [[r6(a), r6(b)] for a, b in seq]


def vw(seq):
    """🔁 (z, v) section points as extrude-plane (v, w) points."""
    return [[r6(v), r6(z)] for z, v in seq]


STEEL_RHO = 7850
WOOD_RHO = 450
PET_RHO = 1380
PE_RHO = 950
SLING_RHO = 1100

A_SHEET, CV_SHEET, I_GROSS = g.area_centroid_inertia(g.sheet_ring())
A_CUT, _, _ = g.area_centroid_inertia(g.sheet_ring(True))
SHEET_KG = STEEL_RHO * (A_SHEET * (g.L - g.CUT) + A_CUT * g.CUT)
I_EFF = 12.6e-8 * 1.035
C_SHEET = max(CV_SHEET, g.H - CV_SHEET)
BEARER_KG = WOOD_RHO * g.BEARER * g.BEARER * g.BEARER_LEN
BATTEN_KG = WOOD_RHO * g.BEARER * g.BEARER * 2 * g.BATTEN_HALF
HULL = g.strap_hull()
STRAP_OUT = g.offset_convex(HULL, g.STRAP_T)
STRAP_KG = PET_RHO * g.STRAP_W * g.STRAP_T * g.ring_length(HULL)
PROT_SECTION = (g.PROT_V1 - g.PROT_V0) * g.PROT_T + 2 * 0.020 * g.PROT_T
PROT_KG = PE_RHO * PROT_SECTION * g.PROT_X
LOOP = g.loop_hull()
LOOP_OUT = g.offset_convex(LOOP, g.LOOP_T)
LOOP_KG = SLING_RHO * g.LOOP_W * g.LOOP_T * g.ring_length(LOOP) * 0.5
LEG_KG = SLING_RHO * g.LEG_W * g.LEG_T * g.LEG_LEN * 0.5
LASHING_KG = 4 * BATTEN_KG + 4 * STRAP_KG + 8 * PROT_KG + 2 * LOOP_KG
TOTAL_KG = 30 * SHEET_KG + 4 * BEARER_KG + LASHING_KG + 2 * LEG_KG
SNOW = 0.68e3
SUCTION = 0.60e3
LASHING_X = 3.0

REF = [
    ("en-1993-1-3", "EN 1993-1-3:2006 Eurocode 3, Part 1-3: supplementary rules for cold-formed members and sheeting (effective section of trapezoidal profiles, S280GD)."),
    ("en-10346", "EN 10346:2015 Continuously hot-dip coated steel flat products: S280GD+Z275, yield strength 280 MPa, zinc coating 275 g/m2 (both sides)."),
    ("en-14782", "EN 14782:2006 Self-supporting metal sheet for roofing, cladding and internal lining (tolerances of trapezoidal profiles)."),
    ("weckman-w35", "Weckman Steel, profile W-35/1035 (35/207), Belastungstabellen Dach (catalogue reprint www.von-lien.de, p. 119): 0.75 mm, self-weight 0.068 kN/m2; three-span permissible load 2.42 kN/m2 at 2.00 m span; single-span L/300 load 0.85 kN/m2 at 2.00 m, from which an effective I of 12.6 cm4/m is back-calculated (5 q L^4 / 384 E I = L/300)."),
    ("kloeckner-35-207", "Kloeckner & Co, Lieferprogramm Dach und Wand: trapezoidal profile KAS 35/207, cover width 1035 mm, sheet width 1070 mm, 0.75 mm, 7.30 kg/m2."),
    ("en-1991-1-3", "EN 1991-1-3:2003 with DIN EN 1991-1-3/NA:2019 Eurocode 1, snow loads: zone 2, low altitude, s_k = 0.85 kN/m2, roof shape coefficient mu_1 = 0.8 gives s = 0.68 kN/m2."),
    ("en-1991-1-4", "EN 1991-1-4:2005 with DIN EN 1991-1-4/NA:2010 Eurocode 1, wind actions: peak velocity pressure about 0.5 kN/m2 (zone 2, inland, h < 10 m) and external pressure coefficient c_pe of about -1.2 on the edge zone of a flat roof, giving a suction of 0.6 kN/m2."),
    ("en-338", "EN 338:2016 Structural timber, strength classes: C24 softwood, E0,mean 11 GPa, mean density 420 kg/m3 (weathered yard timber taken at 450 kg/m3 with residual moisture)."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: steel 50 W/mK, 450 J/kgK; softwood 0.13 W/mK, 1600 J/kgK; polyethylene 0.40 W/mK, 1800 J/kgK."),
    ("ashrae-2021-radiation", "ASHRAE Handbook Fundamentals (2021), Chapter 4 and Chapter 33 tables of surface emittance and solar absorptance: galvanised steel new 0.04-0.13, weathered and dull 0.25-0.28 with a solar absorptance of about 0.65; weathered wood 0.90."),
    ("en-iso-13786", "EN ISO 13786:2017 Thermal performance of building components, dynamic characteristics: effective storage depth of an exposed timber face about 36 mm for 24 h."),
    ("en-1492-2", "EN 1492-2:2000+A1:2008 Textile slings, round slings of man-made fibres: working load limit 1000 kg coded violet, choked hitch factor 0.8."),
    ("en-12195-2", "EN 12195-2:2000 Load restraint assemblies on road vehicles, web lashing made from man-made fibres (composite polyester strapping about 32 x 1.1 mm)."),
    ("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts (hatching)."),
    ("din-1356-1", "DIN 1356-1:1995 Building drawings, types of drawings and basic principles of presentation (scales, line groups)."),
    ("ivv-bauteilboerse", "Bauteilnetz Deutschland / bauteilboerse-bremen.de, Annahme- und Lagerhinweise fuer gebrauchte Trapezbleche: Lagerung auf Kanthoelzern mit Gefaelle, Abdeckung, Paketbildung mit Umreifung; Bohrungen der alten Befestigung bleiben sichtbar (authored summary of yard practice)."),
]


def ref_list():
    """📚 The references of the asset."""
    return [{"id": i, "citation": c} for i, c in REF]


MATERIALS = [
    {"id": "galvanised-steel-weathered", "title": T("Hot-dip galvanised steel S280GD+Z275, weathered", "Feuerverzinkter Stahl S280GD+Z275, bewittert"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": STEEL_RHO, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "strengthPa": 280e6, "source": "literature", "reference": "ashrae-2021-radiation"},
    {"id": "softwood-weathered", "title": T("Squared softwood C24, weathered grey", "Kantholz Nadelholz C24, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": WOOD_RHO, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
    {"id": "polyester-strapping", "title": T("Composite polyester strapping and polyethylene edge protectors", "Polyester-Umreifungsband und Kantenschutz aus Polyethylen"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": PET_RHO, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "en-12195-2"},
    {"id": "polyester-round-sling", "title": T("Polyester round sling, 1 t, violet", "Polyester-Rundschlinge, 1 t, violett"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": SLING_RHO, "conductivityWmK": 0.1, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "en-1492-2"},
]

BONES = []


def bone(id_, parent, en, de, x, y, length, joint, mass=None, section=None, angle=0):
    """🦴 One bone of the rig."""
    b = {"id": id_, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y), "angleDeg": angle, "lengthM": length, "joint": joint}
    if mass is not None:
        b["mass"] = mass
    if section is not None:
        b["section"] = section
    BONES.append(b)


def mass(kg, comx, comy, dist, src="literature", ref="kloeckner-35-207"):
    """⚖️ A segment mass."""
    return {"kg": round(kg, 3), "comX": r6(comx), "comY": r6(comy), "distribution": dist, "source": src, "reference": ref}


bone("yard", None, "Yard ground frame (massless)", "Lagerplatz-Bezugsrahmen (masselos)", 0, 0, 0, {"type": "fixed"})
bone("bundle", "yard", "Bundle of sheets 1 to 29", "Paket der Bleche 1 bis 29", 0, g.BEARER, g.L, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 2.0},
     mass(29 * SHEET_KG, g.L / 2, (28 * g.NEST + g.H) / 2, "uniform"),
     {"areaM2": r6(29 * A_SHEET * 1e3) / 1e3, "secondMomentM4": 29 * I_EFF, "extremeFibreM": r6(C_SHEET), "material": "galvanised-steel-weathered", "title": T("29 nested sheets 35/207 x 0.75 mm acting side by side (effective I 12.6 cm4/m each)", "29 geschachtelte Bleche 35/207 x 0,75 mm, nebeneinander wirkend (wirksames I je 12,6 cm4/m)")})
for k, xb in enumerate(g.XB, 1):
    bone(f"bearer-{k}", "bundle", f"Bearer {k} (stays on the ground)", f"Unterleger {k} (bleibt am Boden)", xb - g.BEARER / 2, -g.BEARER, g.BEARER, {"type": "prismatic", "axisDeg": 90, "minM": -2.0, "maxM": 0},
         mass(BEARER_KG, g.BEARER / 2, g.BEARER / 2, "point", "authored-estimate", "en-338"),
         {"areaM2": 0.01, "secondMomentM4": r6(g.BEARER ** 4 / 12 * 1e9) / 1e9, "extremeFibreM": 0.05, "material": "softwood-weathered", "title": T("Squared timber 100 x 100 mm", "Kantholz 100 x 100 mm")})
bone("lashing", "bundle", "Battens, straps, edge protectors and sling chokes", "Kanthölzer, Umreifung, Kantenschutz und Schlingenschnürung", LASHING_X, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -0.002, "maxM": 0},
     mass(LASHING_KG, 0, g.BATTEN_V0, "point", "authored-estimate", "en-338"))
bone("sheet-30", "bundle", "Sheet 30, the top sheet that is reinstalled", "Blech 30, das oberste, wieder eingebaute Blech", 0, g.V30, g.L, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.2},
     mass(SHEET_KG, g.L / 2, CV_SHEET, "uniform"),
     {"areaM2": round(A_SHEET, 7), "secondMomentM4": I_EFF, "extremeFibreM": r6(C_SHEET), "material": "galvanised-steel-weathered", "title": T("Trapezoidal sheet 35/207 x 0.75 mm, effective I 12.6 cm4/m", "Trapezblech 35/207 x 0,75 mm, wirksames I 12,6 cm4/m")})
LEG_V = g.TOP + g.LEG_T / 2
bone("sling-leg-a", "bundle", "Round sling leg at the left lifting point", "Rundschlingenstrang am linken Anschlagpunkt", g.XL[0] - g.LOOP_W / 2, LEG_V, g.LEG_LEN, {"type": "revolute", "minDeg": -90, "maxDeg": 0},
     mass(LEG_KG, g.LEG_LEN / 2, 0, "uniform", "authored-estimate", "en-1492-2"),
     {"areaM2": round(g.LEG_W * g.LEG_T, 6), "secondMomentM4": round(g.LEG_W * g.LEG_T ** 3 / 12, 12), "extremeFibreM": g.LEG_T / 2, "material": "polyester-round-sling", "title": T("Round sling 1 t, flattened 50 x 16 mm", "Rundschlinge 1 t, flach 50 x 16 mm")}, angle=180)
bone("sling-leg-b", "bundle", "Round sling leg at the right lifting point", "Rundschlingenstrang am rechten Anschlagpunkt", g.XL[1] + g.LOOP_W / 2, LEG_V, g.LEG_LEN, {"type": "revolute", "minDeg": 0, "maxDeg": 90},
     mass(LEG_KG, g.LEG_LEN / 2, 0, "uniform", "authored-estimate", "en-1492-2"),
     {"areaM2": round(g.LEG_W * g.LEG_T, 6), "secondMomentM4": round(g.LEG_W * g.LEG_T ** 3 / 12, 12), "extremeFibreM": g.LEG_T / 2, "material": "polyester-round-sling", "title": T("Round sling 1 t, flattened 50 x 16 mm", "Rundschlinge 1 t, flach 50 x 16 mm")})

ANCHORS = []
for k in range(1, 5):
    ANCHORS.append({"id": f"bearer-{k}-ground", "bone": f"bearer-{k}", "x": 0.05, "y": 0, "kind": "contact", "title": T(f"Bearer {k} on the yard surface", f"Unterleger {k} auf dem Lagerplatz"), "support": "contact", "footprintLengthM": 0.1, "footprintWidthM": g.BEARER_LEN, "frictionCoefficient": 0.5})
ANCHORS.append({"id": "hook-a", "bone": "sling-leg-a", "x": g.LEG_LEN, "y": 0, "kind": "support", "title": T("Left hook of the spreader beam", "Linker Haken der Traverse"), "support": "pin"})
ANCHORS.append({"id": "hook-b", "bone": "sling-leg-b", "x": g.LEG_LEN, "y": 0, "kind": "support", "title": T("Right hook of the spreader beam", "Rechter Haken der Traverse"), "support": "roller", "normalDeg": 90})
for k, xs in enumerate(g.XS, 1):
    ANCHORS.append({"id": f"purlin-{k}", "bone": "sheet-30", "x": xs, "y": 0, "kind": "support", "title": T(f"Purlin line {k} (screwed valley bearing)", f"Pfettenlinie {k} (verschraubtes Untergurtauflager)"), "support": "pin" if k == 1 else "roller", **({} if k == 1 else {"normalDeg": 90})})

REGIONS = ["top-sheet", "bundle-sheets", "bearers", "battens", "lashing-straps", "round-slings"]


def rect(id_, bone_, x, y, w, h, tone, region=None, regions=None, material=None, role="body", z=1, filled=True, hatch=None, pen=None):
    """⬛ A rectangle shape."""
    s = {"id": id_, "bone": bone_, "role": role, "z": z, "filled": filled, "tone": tone}
    if regions:
        s["regions"] = regions
    elif region:
        s["region"] = region
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen:
        s["penMm"] = pen
    s.update({"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)})
    return s


def poly(id_, bone_, points, tone, closed=True, filled=True, region=None, regions=None, material=None, role="body", z=1, pen=None):
    """🔷 A polygon or polyline shape."""
    s = {"id": id_, "bone": bone_, "role": role, "z": z, "filled": filled, "tone": tone}
    if regions:
        s["regions"] = regions
    elif region:
        s["region"] = region
    if material:
        s["material"] = material
    if pen:
        s["penMm"] = pen
    s.update({"kind": "poly", "points": pts(points), "closed": closed})
    return s


def circle(id_, bone_, cx, cy, r, tone, region, role="detail", z=8):
    """⭕ A small circle (screw hole)."""
    return {"id": id_, "bone": bone_, "role": role, "z": z, "filled": False, "tone": tone, "region": region, "kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r, "ry": r}


GS, WD, ST, SL = "galvanised-steel-weathered", "softwood-weathered", "polyester-strapping", "polyester-round-sling"
TONE_TOP, TONE_PACK, TONE_WOOD, TONE_STRAP, TONE_PROT, TONE_SLING = 0.78, 0.66, 0.52, 0.88, 0.22, 0.34
WB = g.BEARER / 2
Y0 = g.BEARER


def world_v(v):
    """🌍 World height of a bundle-frame height."""
    return Y0 + v


def lu(x):
    """↔️ Lashing-frame u of a world x."""
    return x - LASHING_X


def bearer_shapes(view, scale):
    """🪵 The four bearers in one view."""
    out = []
    for k, xb in enumerate(g.XB, 1):
        b = f"bearer-{k}"
        if view == "elevation":
            out.append(rect(f"bearer-{k}-end", b, 0, 0, 0.1, 0.1, TONE_WOOD, "bearers", material=WD, z=1, hatch="end-grain" if scale == 50 else None))
        elif view == "front":
            out.append(rect(f"bearer-{k}-side", b, -g.BEARER_LEN / 2, 0, g.BEARER_LEN, 0.1, TONE_WOOD, "bearers", material=WD, z=1))
        else:
            out.append(rect(f"bearer-{k}-top", b, xb - WB, -g.BEARER_LEN / 2, 0.1, g.BEARER_LEN, TONE_WOOD, "bearers", material=WD, z=1))
    return out


def side_runs(hull, thickness):
    """➰ The two side runs of a band tensioned round a hull, as centre lines from the lowest corner up to the top corner on each side: the bottom run lies under the valleys and the top run on the battens, where the band is hidden in the end view."""
    ring = g.offset_convex(hull, thickness / 2)
    right = sorted([p for p in ring if p[0] > 0], key=lambda p: p[1])
    left = sorted([p for p in ring if p[0] < 0], key=lambda p: p[1])
    return [[(z, world_v(v)) for z, v in right], [(z, world_v(v)) for z, v in left]]


def band_end(id_, hull, thickness, tone, region, material, z):
    """➿ Front section of a band tensioned round a hull, as the body has it: the filled ring between the hull and its offset by the band thickness (a thin band drawn as a pen stroke encloses the void under its taut run, which the silhouette guard then reads as closed)."""
    ring = lambda r: "M " + " L ".join(f"{r6(z)} {r6(world_v(v))}" for z, v in r) + " Z"
    return {"id": id_, "bone": "lashing", "role": "body", "z": z, "filled": True, "tone": tone, "region": region, "material": material, "kind": "path", "d": ring(g.offset_convex(hull, thickness)) + " " + ring(hull)}


def band_sides(id_, hull, thickness, tone, region, material, z):
    """➰ Front strokes of the two side runs of a band."""
    return [poly(f"{id_}-{side}", "lashing", run, tone, closed=False, filled=False, region=region, material=material, role="detail", z=z) for side, run in zip(("l", "r"), side_runs(hull, thickness))]


def notch_plan(bone_, id_, tone, region, z, regions=None):
    """✂️ Plan outline of a sheet with the corner trim at the eaves end of its right edge crest."""
    e = g.EDGE
    return poly(id_, bone_, [(0, -0.5), (g.CUT, -0.5), (g.CUT, -e), (g.L, -e), (g.L, e), (0, e)], tone, region=region, regions=regions, material=GS, z=z)


def shapes_50():
    """✏️ The 1:50 drawings."""
    el, fr, pl = bearer_shapes("elevation", 50), bearer_shapes("front", 50), bearer_shapes("plan", 50)
    el.append(rect("bundle-side", "bundle", 0, 0, g.L, g.V29TOP, TONE_PACK, "bundle-sheets", material=GS, z=2))
    el.append(poly("bundle-edge-crest-line", "bundle", [(g.CUT, g.H - g.T), (g.L, g.H - g.T)], 0.3, closed=False, filled=False, region="bundle-sheets", role="detail", z=2.5))
    el.append(poly("bundle-corner-trim", "bundle", [(g.CUT, g.H - g.T), (g.CUT, g.V29TOP)], 0.3, closed=False, filled=False, region="bundle-sheets", role="detail", z=2.5))
    for k in (10, 20):
        v = (k - 1) * g.NEST + g.H
        el.append(poly(f"bundle-layer-{k}", "bundle", [(g.CUT, v), (g.L, v)], 0.4, closed=False, filled=False, region="bundle-sheets", role="texture", z=2.4))
    el.append(rect("sheet-30-side", "sheet-30", 0, 0, g.L, g.H, TONE_TOP, "top-sheet", material=GS, z=3))
    el.append(poly("sheet-30-edge-crest-line", "sheet-30", [(g.CUT, g.H - g.T), (g.L, g.H - g.T)], 0.3, closed=False, filled=False, region="top-sheet", role="detail", z=3.5))
    el.append(poly("sheet-30-corner-trim", "sheet-30", [(g.CUT, g.H - g.T), (g.CUT, g.H)], 0.3, closed=False, filled=False, region="top-sheet", role="detail", z=3.5))
    for i, xs in enumerate(g.XS, 1):
        el.append(rect(f"batten-{i}-end", "lashing", lu(xs) - WB, g.BATTEN_V0, 0.1, 0.1, TONE_WOOD, "battens", material=WD, z=4, hatch="end-grain"))
        el.append(rect(f"protector-{i}", "lashing", lu(xs) - g.PROT_X / 2, g.PROT_V0, g.PROT_X, g.PROT_V1 - g.PROT_V0, TONE_PROT, "lashing-straps", material=ST, z=5))
        el.append(rect(f"strap-{i}", "lashing", lu(xs) - g.STRAP_W / 2, -g.STRAP_T, g.STRAP_W, g.STRAP_TOP + g.STRAP_T, TONE_STRAP, "lashing-straps", material=ST, z=6))
    for i, xl in enumerate(g.XL):
        el.append(rect(f"sling-choke-{'ab'[i]}", "lashing", lu(xl) - g.LOOP_W / 2, -g.LOOP_T, g.LOOP_W, g.TOP + 2 * g.LOOP_T, TONE_SLING, "round-slings", material=SL, z=5))
    for leg in ("a", "b"):
        el.append(rect(f"sling-leg-{leg}-side", f"sling-leg-{leg}", 0, -g.LEG_T / 2, g.LEG_LEN, g.LEG_T, TONE_SLING, "round-slings", material=SL, z=6))
    fr.append(poly("bundle-end", "bundle", [(z, world_v(v)) for z, v in g.pack_ring(29)], TONE_PACK, region="bundle-sheets", material=GS, z=2))
    up30, _ = g.sheet_surfaces()
    fr.append(poly("sheet-30-end", "sheet-30", [(z, world_v(v + g.V30 - g.T / 2)) for z, v in g.mid_polyline()], TONE_TOP, closed=False, filled=False, region="top-sheet", material=GS, role="detail", z=3))
    fr.append(rect("battens-end", "lashing", -g.BATTEN_HALF, world_v(g.BATTEN_V0), 2 * g.BATTEN_HALF, 0.1, TONE_WOOD, "battens", material=WD, z=4, hatch="long-grain"))
    for side, sgn in (("r", -1), ("l", 1)):
        z0, z1 = sorted([sgn * (g.EDGE - 0.020), sgn * (g.EDGE + g.PROT_T)])
        fr.append(rect(f"protector-{side}-end", "lashing", z0, world_v(g.PROT_V0), z1 - z0, g.PROT_V1 - g.PROT_V0, TONE_PROT, "lashing-straps", material=ST, z=5))
    fr.append(band_end("strap-end", HULL, g.STRAP_T, TONE_STRAP, "lashing-straps", ST, 6))
    fr.extend(band_sides("sling-choke-side", LOOP, g.LOOP_T, TONE_SLING, "round-slings", SL, 5.5))
    for leg in ("a", "b"):
        fr.append(rect(f"sling-leg-{leg}-end", f"sling-leg-{leg}", -g.LEG_W / 2, world_v(g.TOP), g.LEG_W, g.LEG_T, TONE_SLING, "round-slings", material=SL, z=6.5))
    pl.append(notch_plan("bundle", "bundle-top", TONE_PACK, "bundle-sheets", 2))
    pl.append(notch_plan("sheet-30", "sheet-30-top", TONE_TOP, "top-sheet", 3))
    for k, c in enumerate(g.CROWNS):
        lo = max(c - g.CREST / 2, -g.EDGE)
        hi = min(c + g.CREST / 2, g.EDGE)
        x0 = g.CUT if k == 0 else 0
        if k > 0:
            pl.append(poly(f"sheet-30-crest-{k}-r", "sheet-30", [(x0, lo), (g.L, lo)], 0.4, closed=False, filled=False, region="top-sheet", role="texture", z=3.2))
        if k < 5:
            pl.append(poly(f"sheet-30-crest-{k}-l", "sheet-30", [(x0, hi), (g.L, hi)], 0.4, closed=False, filled=False, region="top-sheet", role="texture", z=3.2))
    hole = 0
    for j, xo in enumerate(g.OLD_PURLINS):
        crowns = range(6) if j in (0, len(g.OLD_PURLINS) - 1) else (0, 2, 4)
        for k in crowns:
            if k == 0 and xo < g.CUT:
                continue
            hole += 1
            pl.append(circle(f"old-screw-hole-{hole:02d}", "sheet-30", xo, g.CROWNS[k], g.HOLE_D / 2, 0.2, "top-sheet"))
    for i, xs in enumerate(g.XS, 1):
        pl.append(rect(f"batten-{i}-top", "lashing", xs - WB, -g.BATTEN_HALF, 0.1, 2 * g.BATTEN_HALF, TONE_WOOD, "battens", material=WD, z=4))
        for side, sgn in (("r", -1), ("l", 1)):
            z0, z1 = sorted([sgn * (g.EDGE - 0.020), sgn * (g.EDGE + g.PROT_T)])
            pl.append(rect(f"protector-{i}-{side}-top", "lashing", xs - g.PROT_X / 2, z0, g.PROT_X, z1 - z0, TONE_PROT, "lashing-straps", material=ST, z=5))
        pl.append(rect(f"strap-{i}-top", "lashing", xs - g.STRAP_W / 2, -(g.EDGE + g.PROT_T + g.STRAP_T), g.STRAP_W, 2 * (g.EDGE + g.PROT_T + g.STRAP_T), TONE_STRAP, "lashing-straps", material=ST, z=6))
    for i, xl in enumerate(g.XL):
        pl.append(rect(f"sling-choke-{'ab'[i]}-top", "lashing", xl - g.LOOP_W / 2, -(g.EDGE + g.LOOP_T), g.LOOP_W, 2 * (g.EDGE + g.LOOP_T), TONE_SLING, "round-slings", material=SL, z=5))
    pl.append(rect("sling-leg-a-top", "sling-leg-a", g.XL[0] - g.LOOP_W / 2 - g.LEG_LEN, -g.LEG_W / 2, g.LEG_LEN, g.LEG_W, TONE_SLING, "round-slings", material=SL, z=6.5))
    pl.append(rect("sling-leg-b-top", "sling-leg-b", g.XL[1] + g.LOOP_W / 2, -g.LEG_W / 2, g.LEG_LEN, g.LEG_W, TONE_SLING, "round-slings", material=SL, z=6.5))
    return {"elevation": el, "front": fr, "plan": pl}


PROT100 = 0.015
LEG_S500 = 0.036


def hull100():
    """🎗️ Strap hull at 1:100: protectors generalised to 15 mm plates outside the edges."""
    return g.strap_hull(g.EDGE + PROT100)


def shapes_100():
    """✏️ The 1:100 drawings."""
    el, fr, pl = bearer_shapes("elevation", 100), bearer_shapes("front", 100), bearer_shapes("plan", 100)
    el.append(rect("bundle-side", "bundle", 0, 0, g.L, g.V29TOP, TONE_PACK, "bundle-sheets", material=GS, z=2))
    el.append(poly("bundle-edge-crest-line", "bundle", [(0, g.H - g.T), (g.L, g.H - g.T)], 0.3, closed=False, filled=False, region="bundle-sheets", role="detail", z=2.5))
    el.append(rect("sheet-30-side", "sheet-30", 0, 0, g.L, g.H, TONE_TOP, "top-sheet", material=GS, z=3))
    for i, xs in enumerate(g.XS, 1):
        el.append(rect(f"batten-{i}-end", "lashing", lu(xs) - WB, g.BATTEN_V0, 0.1, 0.1, TONE_WOOD, "battens", material=WD, z=4))
        el.append(rect(f"protector-{i}", "lashing", lu(xs) - g.PROT_X / 2, g.PROT_V0, g.PROT_X, g.PROT_V1 - g.PROT_V0, TONE_PROT, "lashing-straps", material=ST, z=5))
        el.append(rect(f"strap-{i}", "lashing", lu(xs) - g.STRAP_W / 2, -g.STRAP_T, g.STRAP_W, g.STRAP_TOP + g.STRAP_T, TONE_STRAP, "lashing-straps", material=ST, z=6))
    for i, xl in enumerate(g.XL):
        el.append(rect(f"sling-choke-{'ab'[i]}", "lashing", lu(xl) - g.LOOP_W / 2, -g.LOOP_T, g.LOOP_W, g.TOP + 2 * g.LOOP_T, TONE_SLING, "round-slings", material=SL, z=5))
    for leg in ("a", "b"):
        el.append(rect(f"sling-leg-{leg}-side", f"sling-leg-{leg}", 0, -g.LEG_T / 2, g.LEG_LEN, g.LEG_T, TONE_SLING, "round-slings", material=SL, z=6))
    fr.append(poly("bundle-end", "bundle", [(z, world_v(v)) for z, v in g.pack_ring(29)], TONE_PACK, region="bundle-sheets", material=GS, z=2))
    fr.append(poly("sheet-30-end", "sheet-30", [(z, world_v(v + g.V30 - g.T / 2)) for z, v in g.mid_polyline()], TONE_TOP, closed=False, filled=False, region="top-sheet", material=GS, role="detail", z=3))
    fr.append(rect("battens-end", "lashing", -g.BATTEN_HALF, world_v(g.BATTEN_V0), 2 * g.BATTEN_HALF, 0.1, TONE_WOOD, "battens", material=WD, z=4))
    for side, sgn in (("r", -1), ("l", 1)):
        z0, z1 = sorted([sgn * g.EDGE, sgn * (g.EDGE + PROT100)])
        fr.append(rect(f"protector-{side}-end", "lashing", z0, world_v(g.PROT_V0), z1 - z0, g.PROT_V1 - g.PROT_V0, TONE_PROT, "lashing-straps", material=ST, z=5))
    fr.append(band_end("strap-end", hull100(), g.STRAP_T, TONE_STRAP, "lashing-straps", ST, 6))
    fr.extend(band_sides("sling-choke-side", LOOP, g.LOOP_T, TONE_SLING, "round-slings", SL, 5.5))
    for leg in ("a", "b"):
        fr.append(rect(f"sling-leg-{leg}-end", f"sling-leg-{leg}", -g.LEG_W / 2, world_v(g.TOP), g.LEG_W, g.LEG_T, TONE_SLING, "round-slings", material=SL, z=6.5))
    pl.append(notch_plan("bundle", "bundle-top", TONE_PACK, "bundle-sheets", 2))
    pl.append(notch_plan("sheet-30", "sheet-30-top", TONE_TOP, "top-sheet", 3))
    for k, c in enumerate(g.CROWNS):
        pl.append(poly(f"sheet-30-crest-{k}", "sheet-30", [(g.CUT if k == 0 else 0, c), (g.L, c)], 0.4, closed=False, filled=False, region="top-sheet", role="texture", z=3.2))
    for i, xs in enumerate(g.XS, 1):
        pl.append(rect(f"batten-{i}-top", "lashing", xs - WB, -g.BATTEN_HALF, 0.1, 2 * g.BATTEN_HALF, TONE_WOOD, "battens", material=WD, z=4))
        for side, sgn in (("r", -1), ("l", 1)):
            z0, z1 = sorted([sgn * g.EDGE, sgn * (g.EDGE + PROT100)])
            pl.append(rect(f"protector-{i}-{side}-top", "lashing", xs - g.PROT_X / 2, z0, g.PROT_X, z1 - z0, TONE_PROT, "lashing-straps", material=ST, z=5))
        pl.append(rect(f"strap-{i}-top", "lashing", xs - g.STRAP_W / 2, -(g.EDGE + PROT100 + g.STRAP_T), g.STRAP_W, 2 * (g.EDGE + PROT100 + g.STRAP_T), TONE_STRAP, "lashing-straps", material=ST, z=6))
    for i, xl in enumerate(g.XL):
        pl.append(rect(f"sling-choke-{'ab'[i]}-top", "lashing", xl - g.LOOP_W / 2, -(g.EDGE + g.LOOP_T), g.LOOP_W, 2 * (g.EDGE + g.LOOP_T), TONE_SLING, "round-slings", material=SL, z=5))
    pl.append(rect("sling-leg-a-top", "sling-leg-a", g.XL[0] - g.LOOP_W / 2 - g.LEG_LEN, -g.LEG_W / 2, g.LEG_LEN, g.LEG_W, TONE_SLING, "round-slings", material=SL, z=6.5))
    pl.append(rect("sling-leg-b-top", "sling-leg-b", g.XL[1] + g.LOOP_W / 2, -g.LEG_W / 2, g.LEG_LEN, g.LEG_W, TONE_SLING, "round-slings", material=SL, z=6.5))
    return {"elevation": el, "front": fr, "plan": pl}


F200 = 0.030


def hull200():
    """⬜ Strap hull at 1:200 around the generalised flat stack."""
    return g.flat_hull(g.TOP)


def loop200():
    """🪢 The generalised choke loop hull at 1:200."""
    return g.ccw([(-g.EDGE, 0.0), (g.EDGE, 0.0), (g.EDGE, g.TOP), (-g.EDGE, g.TOP)])


def shapes_200():
    """✏️ The 1:200 drawings."""
    el, fr, pl = bearer_shapes("elevation", 200), bearer_shapes("front", 200), bearer_shapes("plan", 200)
    el.append(rect("bundle-side", "bundle", 0, 0, g.L, g.V30, TONE_PACK, "bundle-sheets", material=GS, z=2))
    el.append(rect("sheet-30-side", "sheet-30", 0, 0, g.L, g.H, TONE_TOP, "top-sheet", material=GS, z=3))
    for i, xs in enumerate(g.XS, 1):
        el.append(rect(f"batten-{i}-end", "lashing", lu(xs) - WB, g.BATTEN_V0, 0.1, 0.1, TONE_WOOD, "battens", material=WD, z=4))
        el.append(rect(f"strap-{i}", "lashing", lu(xs) - g.STRAP_W / 2, -g.STRAP_T, g.STRAP_W, g.STRAP_TOP + g.STRAP_T, TONE_STRAP, "lashing-straps", material=ST, z=6))
    for i, xl in enumerate(g.XL):
        el.append(rect(f"sling-choke-{'ab'[i]}", "lashing", lu(xl) - F200 / 2, -g.LOOP_T, F200, g.TOP + 2 * g.LOOP_T, TONE_SLING, "round-slings", material=SL, z=5))
    for leg in ("a", "b"):
        el.append(poly(f"sling-leg-{leg}-side", f"sling-leg-{leg}", [(0, LEG_S500 / 2 - g.LEG_T / 2), (g.LEG_LEN, LEG_S500 / 2 - g.LEG_T / 2)], TONE_SLING, closed=False, filled=False, region="round-slings", material=SL, z=6))
    fr.append(rect("bundle-end", "bundle", -g.EDGE, world_v(0), 2 * g.EDGE, g.V30, TONE_PACK, "bundle-sheets", material=GS, z=2))
    fr.append(rect("sheet-30-end", "sheet-30", -g.EDGE, world_v(g.V30), 2 * g.EDGE, g.H, TONE_TOP, "top-sheet", material=GS, z=3))
    fr.append(rect("battens-end", "lashing", -g.BATTEN_HALF, world_v(g.BATTEN_V0), 2 * g.BATTEN_HALF, 0.1, TONE_WOOD, "battens", material=WD, z=4))
    fr.append(poly("strap-loop", "lashing", [(z, world_v(v)) for z, v in g.offset_convex(hull200(), g.STRAP_T / 2)], TONE_STRAP, filled=False, region="lashing-straps", material=ST, role="detail", z=6))
    fr.append(poly("sling-choke-loop", "lashing", [(z, world_v(v)) for z, v in g.offset_convex(loop200(), g.LOOP_T / 2)], TONE_SLING, filled=False, region="round-slings", material=SL, role="detail", z=5.5))
    pl.append(rect("bundle-top", "bundle", 0, -g.EDGE, g.L, 2 * g.EDGE, TONE_PACK, "bundle-sheets", material=GS, z=2))
    pl.append(notch_plan("sheet-30", "sheet-30-top", TONE_TOP, "top-sheet", 3))
    for i, xs in enumerate(g.XS, 1):
        pl.append(rect(f"batten-{i}-top", "lashing", xs - WB, -g.BATTEN_HALF, 0.1, 2 * g.BATTEN_HALF, TONE_WOOD, "battens", material=WD, z=4))
        pl.append(rect(f"strap-{i}-top", "lashing", xs - g.STRAP_W / 2, -(g.EDGE + g.STRAP_T), g.STRAP_W, 2 * (g.EDGE + g.STRAP_T), TONE_STRAP, "lashing-straps", material=ST, z=6))
    for i, xl in enumerate(g.XL):
        pl.append(rect(f"sling-choke-{'ab'[i]}-top", "lashing", xl - F200 / 2, -(g.EDGE + g.LOOP_T), F200, 2 * (g.EDGE + g.LOOP_T), TONE_SLING, "round-slings", material=SL, z=5))
    pl.append(poly("sling-leg-a-top", "sling-leg-a", [(g.XL[0] - g.LOOP_W / 2 - g.LEG_LEN, 0), (g.XL[0] - g.LOOP_W / 2, 0)], TONE_SLING, closed=False, filled=False, region="round-slings", material=SL, z=6.5))
    pl.append(poly("sling-leg-b-top", "sling-leg-b", [(g.XL[1] + g.LOOP_W / 2, 0), (g.XL[1] + g.LOOP_W / 2 + g.LEG_LEN, 0)], TONE_SLING, closed=False, filled=False, region="round-slings", material=SL, z=6.5))
    return {"elevation": el, "front": fr, "plan": pl}


def shapes_500():
    """✏️ The 1:500 drawings: the stack generalised to bearers, one sheet block and the battens."""
    el, fr, pl = bearer_shapes("elevation", 500), bearer_shapes("front", 500), bearer_shapes("plan", 500)
    el.append(rect("bundle-side", "bundle", 0, 0, g.L, g.TOP, TONE_PACK, regions=["bundle-sheets", "top-sheet"], material=GS, z=2))
    for i, xs in enumerate(g.XS, 1):
        el.append(rect(f"batten-{i}-end", "lashing", lu(xs) - WB, g.BATTEN_V0, 0.1, 0.1, TONE_WOOD, regions=["battens", "lashing-straps"], material=WD, z=4))
    for leg in ("a", "b"):
        el.append(poly(f"sling-leg-{leg}-side", f"sling-leg-{leg}", [(0, 0), (g.LEG_LEN, 0)], TONE_SLING, closed=False, filled=False, region="round-slings", material=SL, z=6))
    fr.append(poly("bundle-end", "bundle", [(-g.EDGE, world_v(0)), (g.EDGE, world_v(0)), (g.EDGE, world_v(g.TOP)), (g.BATTEN_HALF, world_v(g.TOP)), (g.BATTEN_HALF, world_v(g.BATTEN_V1)), (-g.BATTEN_HALF, world_v(g.BATTEN_V1)), (-g.BATTEN_HALF, world_v(g.TOP)), (-g.EDGE, world_v(g.TOP))], TONE_PACK, regions=["bundle-sheets", "top-sheet", "battens", "lashing-straps", "round-slings"], material=GS, z=2))
    pl.append(rect("bundle-top", "bundle", 0, -g.EDGE, g.L, 2 * g.EDGE, TONE_PACK, "bundle-sheets", material=GS, z=2))
    pl.append(rect("sheet-30-top", "sheet-30", 0, -g.EDGE, g.L, 2 * g.EDGE, TONE_TOP, "top-sheet", material=GS, z=3))
    for i, xs in enumerate(g.XS, 1):
        pl.append(rect(f"batten-{i}-top", "lashing", xs - WB, -g.BATTEN_HALF, 0.1, 2 * g.BATTEN_HALF, TONE_WOOD, regions=["battens", "lashing-straps"], material=WD, z=4))
    pl.append(poly("sling-leg-a-top", "sling-leg-a", [(g.XL[0] - g.LOOP_W / 2 - g.LEG_LEN, 0), (g.XL[0] - g.LOOP_W / 2, 0)], TONE_SLING, closed=False, filled=False, region="round-slings", material=SL, z=6.5))
    pl.append(poly("sling-leg-b-top", "sling-leg-b", [(g.XL[1] + g.LOOP_W / 2, 0), (g.XL[1] + g.LOOP_W / 2 + g.LEG_LEN, 0)], TONE_SLING, closed=False, filled=False, region="round-slings", material=SL, z=6.5))
    return {"elevation": el, "front": fr, "plan": pl}


HATCH_END = {"id": "end-grain", "title": T("Timber, end grain", "Holz, Hirnholz"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}
HATCH_LONG = {"id": "long-grain", "title": T("Timber, long grain", "Holz, Langholz"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1}

REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation of the 6.00 m bundle on its four 100 mm squared-timber bearers, 120 mm on paper: the band of 29 nested sheets (77 mm), the top sheet above it with its 35 mm edge crest, the four battens across the top and the four composite straps with their edge protectors, the two choked violet round slings at 1.50 m and 4.50 m with their legs lying on the top sheet. Front view: the true 35/207 profile of the bundle end over the bearer, the top sheet as its profile line, the battens and the strap and sling loops tensioned round the stack. Plan: the top sheet with its six crests, the eaves corner trim of the right edge crest and the old screw holes.", "Seitenansicht des 6,00 m langen Pakets auf vier Kanthölzern 100 mm, 120 mm auf Papier: das Band aus 29 geschachtelten Blechen (77 mm), darüber das oberste Blech mit seiner 35 mm hohen Randrippe, die vier Kanthölzer quer obenauf und die vier Polyesterbänder mit Kantenschutz, die beiden geschnürten violetten Rundschlingen bei 1,50 m und 4,50 m mit ihren auf dem obersten Blech liegenden Strängen. Vorderansicht: das wahre Profil 35/207 des Paketendes über dem Unterleger, das oberste Blech als Profillinie, die Kanthölzer sowie Band- und Schlingenschlaufen um den Stapel. Draufsicht: das oberste Blech mit seinen sechs Rippen, dem Traufeckschnitt der rechten Randrippe und den alten Schraublöchern."),
        "retained": T("Every crest of the profile (35 mm high, 35 mm wide, 207 mm pitch) in the front view, the 150 mm corner trim of the edge crest at the eaves end, the 6.5 mm screw holes of the old fastening (end purlin lines in every crest, intermediate lines in every second crest, 1.45 m old purlin spacing) as circles in plan, the straps at 32 mm, the 2 mm edge protectors and the 25 mm sling chokes at their true widths.", "Jede Rippe des Profils (35 mm hoch, 35 mm breit, Teilung 207 mm) in der Vorderansicht, der 150 mm lange Eckschnitt der Randrippe am Traufende, die 6,5-mm-Schraublöcher der alten Befestigung (Endpfettenlinien in jeder Rippe, Zwischenpfetten in jeder zweiten, alter Pfettenabstand 1,45 m) als Kreise in der Draufsicht, die Bänder mit 32 mm, der 2 mm starke Kantenschutz und die 25 mm breiten Schlingenschnürungen in wahrer Breite."),
        "merged": T("The 29 nested sheets of the bundle are one band in side elevation and one profiled band in the front view; their 1.5 mm nesting pitch is below the 7.5 mm smallest feature, so every tenth layer is a texture line.", "Die 29 geschachtelten Bleche des Pakets sind in der Seitenansicht ein Band und in der Vorderansicht ein profiliertes Band; ihre Schachtelteilung von 1,5 mm liegt unter der kleinsten Einzelheit von 7,5 mm, daher ist jede zehnte Lage eine Texturlinie."),
        "omitted": T("The 0.75 mm sheet thickness as a filled area (drawn as the profile line of the top sheet), the strap buckles, the zinc white-rust spots (a lighter tone of the top sheet).", "Die Blechdicke von 0,75 mm als Fläche (als Profillinie des obersten Blechs gezeichnet), die Bandverschlüsse, die Weißrostflecken der Verzinkung (ein hellerer Ton des obersten Blechs)."),
        "thinElements": T("The strap in the front view is its true 1 mm section round bundle and battens; the sling loops are the centre lines of their two side runs, whose bottom run under the valleys and top run on the battens are hidden in the end view; the top sheet profile is a detail line.", "Das Band ist in der Vorderansicht sein wahrer Querschnitt von 1 mm um Paket und Kanthölzer; die Schlingenschlaufen sind die Mittellinien ihrer beiden Seitenläufe, deren unterer Lauf unter den Untergurten und oberer auf den Kanthölzern in der Stirnansicht verdeckt sind; das Profil des obersten Blechs ist eine Detaillinie."),
        "hatching": T("End grain of bearers and battens cross-hatched 0.5 mm, the long grain of the battens in the front view hatched in lines.", "Hirnholz von Unterlegern und Kanthölzern kreuzschraffiert 0,5 mm, das Langholz der Kanthölzer in der Vorderansicht mit Linien schraffiert."),
        "styles": T("Weathered zinc in two light greys (the top sheet lighter), timber mid grey, the straps nearly white, edge protectors and slings dark.", "Bewittertes Zink in zwei hellen Grautönen (das oberste Blech heller), Holz mittelgrau, die Bänder fast weiß, Kantenschutz und Schlingen dunkel."),
        "overlays": T("Supports, hooks and purlin lines appear in the structural views only.", "Auflager, Haken und Pfettenlinien erscheinen nur in den statischen Ansichten."),
    },
    100: {
        "silhouette": T("The same bundle at 60 mm on paper: bearers, the 29-sheet band, the top sheet, battens, straps, generalised edge protectors and the sling chokes and legs; front view with the true 35/207 profile.", "Dasselbe Paket mit 60 mm auf Papier: Unterleger, das Band aus 29 Blechen, das oberste Blech, Kanthölzer, Bänder, vereinfachter Kantenschutz sowie Schlingenschnürungen und -stränge; Vorderansicht mit dem wahren Profil 35/207."),
        "retained": T("The crests in the front view (0.35 mm on paper), the six crest lines of the top sheet in plan, the corner trim, the straps (0.32 mm) and the sling chokes.", "Die Rippen der Vorderansicht (0,35 mm auf Papier), die sechs Rippenlinien des obersten Blechs in der Draufsicht, der Eckschnitt, die Bänder (0,32 mm) und die Schlingenschnürungen."),
        "merged": T("The U-shaped edge protectors become 15 mm plates outside the sheet edges (the smallest feature); the layer texture of the bundle side is dropped.", "Der U-förmige Kantenschutz wird zu 15 mm starken Platten außen an den Blechkanten (die kleinste Einzelheit); die Lagentextur der Paketseite entfällt."),
        "omitted": T("The 6.5 mm screw holes (0.065 mm on paper) and the corner-trim tick in side elevation.", "Die 6,5-mm-Schraublöcher (0,065 mm auf Papier) und der Eckschnitt-Strich in der Seitenansicht."),
        "thinElements": T("The strap as its true section, the sling loops as the centre lines of their side runs; the top sheet profile as a detail line.", "Das Band als wahrer Querschnitt, die Schlingenschlaufen als Mittellinien ihrer Seitenläufe; das Profil des obersten Blechs als Detaillinie."),
        "hatching": T("No hatches; the timber reads by its tone.", "Keine Schraffuren; das Holz liest sich über seinen Ton."),
        "styles": T("As at 1:50 with the 0.25 mm body pen.", "Wie 1:50 mit der 0,25-mm-Körperfeder."),
        "overlays": T("Supports and loads in the structural views only.", "Auflager und Lasten nur in den statischen Ansichten."),
    },
    200: {
        "silhouette": T("The bundle at 30 mm on paper as bearers, a flat block of 29 sheets (43.5 mm) under the top sheet (35 mm), the battens, straps, sling chokes and legs; the profile is generalised to its envelope.", "Das Paket mit 30 mm auf Papier als Unterleger, ein flacher Block aus 29 Blechen (43,5 mm) unter dem obersten Blech (35 mm), Kanthölzer, Bänder, Schlingenschnürungen und -stränge; das Profil ist auf seine Hüllform vereinfacht."),
        "retained": T("The four bearers, four battens and four straps at their true places; the corner trim in plan (150 x 35 mm, 0.75 x 0.18 mm on paper).", "Die vier Unterleger, vier Kanthölzer und vier Bänder an ihrer wahren Stelle; der Eckschnitt in der Draufsicht (150 x 35 mm, 0,75 x 0,18 mm auf Papier)."),
        "merged": T("The trapezoidal profile becomes its rectangular envelope (the 25 mm web run is below the 30 mm feature); edge protectors merge into the strap loop; sling chokes are drawn 30 mm wide, the legs as lines.", "Das Trapezprofil wird zu seiner rechteckigen Hülle (der Steglauf von 25 mm liegt unter der Einzelheit von 30 mm); der Kantenschutz geht in der Bandschlaufe auf; die Schlingenschnürungen sind 30 mm breit gezeichnet, die Stränge als Linien."),
        "omitted": T("Crest lines, screw holes and the layer lines.", "Rippenlinien, Schraublöcher und Lagenlinien."),
        "thinElements": T("Straps at 32 mm are filled bands in side elevation and plan, an outline in the front view.", "Die Bänder mit 32 mm sind in Seitenansicht und Draufsicht gefüllte Bänder, in der Vorderansicht ein Umriss."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:100, body pen 0.18 mm.", "Töne wie 1:100, Körperfeder 0,18 mm."),
        "overlays": T("None in the tone views.", "Keine in den Tonansichten."),
    },
    500: {
        "silhouette": T("The bundle at 12 mm on paper: the four bearers, one block of all thirty sheets (78.5 mm), the four battens on top and the sling legs as lines.", "Das Paket mit 12 mm auf Papier: die vier Unterleger, ein Block aus allen dreißig Blechen (78,5 mm), die vier Kanthölzer obenauf und die Schlingenstränge als Linien."),
        "retained": T("Bearers and battens at 100 mm (0.2 mm on paper), the sheet block, whose upper face is the top sheet.", "Unterleger und Kanthölzer mit 100 mm (0,2 mm auf Papier), der Blechblock, dessen Oberseite das oberste Blech ist."),
        "merged": T("The bundle and the top sheet are one block in side view, and in the end view one outline with the battens (178.5 mm), so it prints at 1:1000; straps and slings are merged into these tones.", "Paket und oberstes Blech sind in der Seitenansicht ein Block und in der Stirnansicht ein Umriss mit den Kanthölzern (178,5 mm), damit er bei 1:1000 druckt; Bänder und Schlingen gehen in diesen Tönen auf."),
        "omitted": T("Straps (32 mm), edge protectors, sling chokes and the corner trim: all below the 75 mm smallest feature.", "Bänder (32 mm), Kantenschutz, Schlingenschnürungen und der Eckschnitt: alle unter der kleinsten Einzelheit von 75 mm."),
        "thinElements": T("The sling legs as body lines.", "Die Schlingenstränge als Körperlinien."),
        "hatching": T("None.", "Keine."),
        "styles": T("One body pen of 0.13 mm.", "Eine Körperfeder von 0,13 mm."),
        "overlays": T("None.", "Keine."),
    },
}

PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
SHAPES = {50: shapes_50, 100: shapes_100, 200: shapes_200, 500: shapes_500}


def representation(s):
    """🖨️ The authored representation of one scale."""
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": RANGES[s][0], "maxDenominator": RANGES[s][1]}, "pens": PENS[s], "hatches": [HATCH_END, HATCH_LONG] if s == 50 else [], "geometry": SHAPES[s](), "decisions": REP_DECISIONS[s]}


def solid(id_, bone_, kind, material, tone, role="body", **kw):
    """🧊 A volume solid."""
    s = {"id": id_, "kind": kind, "bone": bone_, "material": material, "role": role, "tone": tone}
    s.update(kw)
    return s


def box(id_, bone_, size, centre, material, tone, **kw):
    """📦 A centred box."""
    return solid(id_, bone_, "box", material, tone, size=[r6(a) for a in size], transform={"translation": [r6(a) for a in centre]}, **kw)


def ext_u(id_, bone_, ring, u0, u1, material, tone, holes=None, **kw):
    """➡️ An extrude of a (z, v) section along u."""
    extra = {"holes": [vw(h) for h in holes]} if holes else {}
    return solid(id_, bone_, "extrude", material, tone, profile=vw(ring), plane="yz", **{"from": r6(u0), "to": r6(u1)}, **extra, **kw)


def bearer_solids(prefix=""):
    """🪵 The four bearers."""
    return [box(f"{prefix}bearer-{k}", f"bearer-{k}", [0.1, 0.1, g.BEARER_LEN], [0.05, 0.05, 0], WD, TONE_WOOD) for k in range(1, 5)]


def repeat_x(count, step):
    """🔁 A repeat along u."""
    return {"count": count, "translation": [r6(step), 0, 0]}


def u_protector(sign):
    """🛡️ The U-shaped edge protector section (z, v) on one side."""
    e, t = g.EDGE, g.PROT_T
    z = lambda a: sign * a
    ring = [(z(e - 0.020), g.PROT_V0), (z(e + t), g.PROT_V0), (z(e + t), g.PROT_V1), (z(e - 0.020), g.PROT_V1), (z(e - 0.020), g.PROT_V1 - t), (z(e), g.PROT_V1 - t), (z(e), g.PROT_V0 + t), (z(e - 0.020), g.PROT_V0 + t)]
    return g.ccw(ring)


def body_50():
    """🧊 1:50 body."""
    out = bearer_solids()
    up, air = len(g.sheet_surfaces(True)[0]), g.NEST - g.T
    pack = [(z, v + air) if i < up else (z, v) for i, (z, v) in enumerate(g.pack_ring(29, True))]
    crests = [(z, v + air) if v > g.H else (z, v) for z, v in g.edge_crest_pack_ring(29)]
    out.append(ext_u("sheets-1-29", "bundle", pack, 0, g.L, GS, TONE_PACK))
    out.append(ext_u("sheets-1-29-edge-crest", "bundle", crests, g.CUT, g.L, GS, TONE_PACK))
    out.append(ext_u("sheet-30", "sheet-30", g.sheet_ring(True), 0, g.L, GS, TONE_TOP, sheet=True))
    out.append(ext_u("sheet-30-edge-crest", "sheet-30", g.edge_crest_ring(), g.CUT, g.L, GS, TONE_TOP, sheet=True))
    out.append(box("battens", "lashing", [0.1, 0.1, 2 * g.BATTEN_HALF], [lu(g.XS[0]), g.BATTEN_V0 + 0.05, 0], WD, TONE_WOOD, repeat=repeat_x(4, 1.8)))
    out.append(ext_u("straps", "lashing", STRAP_OUT, lu(g.XS[0]) - g.STRAP_W / 2, lu(g.XS[0]) + g.STRAP_W / 2, ST, TONE_STRAP, holes=[HULL], repeat=repeat_x(4, 1.8)))
    out.append(ext_u("protectors-r", "lashing", u_protector(-1), lu(g.XS[0]) - g.PROT_X / 2, lu(g.XS[0]) + g.PROT_X / 2, ST, TONE_PROT, repeat=repeat_x(4, 1.8)))
    out.append(ext_u("protectors-l", "lashing", u_protector(1), lu(g.XS[0]) - g.PROT_X / 2, lu(g.XS[0]) + g.PROT_X / 2, ST, TONE_PROT, repeat=repeat_x(4, 1.8)))
    out.append(ext_u("sling-chokes", "lashing", LOOP_OUT, lu(g.XL[0]) - g.LOOP_W / 2, lu(g.XL[0]) + g.LOOP_W / 2, SL, TONE_SLING, holes=[LOOP], repeat=repeat_x(2, 3.0)))
    out.append(box("sling-leg-a", "sling-leg-a", [g.LEG_LEN, g.LEG_T, g.LEG_W], [g.LEG_LEN / 2, 0, 0], SL, TONE_SLING))
    out.append(box("sling-leg-b", "sling-leg-b", [g.LEG_LEN, g.LEG_T, g.LEG_W], [g.LEG_LEN / 2, 0, 0], SL, TONE_SLING))
    return out


def body_100():
    """🧊 1:100 body."""
    h = hull100()
    out = bearer_solids("s100-")
    out.append(ext_u("pack-1-29", "bundle", g.pack_ring(29, True), 0, g.L, GS, TONE_PACK))
    out.append(ext_u("pack-1-29-edge-crests", "bundle", g.edge_crest_pack_ring(29), g.CUT, g.L, GS, TONE_PACK))
    out.append(ext_u("s100-sheet-30", "sheet-30", g.sheet_ring(True), 0, g.L, GS, TONE_TOP, sheet=True))
    out.append(ext_u("s100-sheet-30-edge-crest", "sheet-30", g.edge_crest_ring(), g.CUT, g.L, GS, TONE_TOP, sheet=True))
    out.append(box("s100-battens", "lashing", [0.1, 0.1, 2 * g.BATTEN_HALF], [lu(g.XS[0]), g.BATTEN_V0 + 0.05, 0], WD, TONE_WOOD, repeat=repeat_x(4, 1.8)))
    out.append(ext_u("s100-straps", "lashing", g.offset_convex(h, g.STRAP_T), lu(g.XS[0]) - g.STRAP_W / 2, lu(g.XS[0]) + g.STRAP_W / 2, ST, TONE_STRAP, holes=[h], repeat=repeat_x(4, 1.8)))
    for side, sgn in (("r", -1), ("l", 1)):
        out.append(box(f"s100-protectors-{side}", "lashing", [g.PROT_X, g.PROT_V1 - g.PROT_V0, PROT100], [lu(g.XS[0]), (g.PROT_V0 + g.PROT_V1) / 2, sgn * (g.EDGE + PROT100 / 2)], ST, TONE_PROT, repeat=repeat_x(4, 1.8)))
    out.append(ext_u("s100-sling-chokes", "lashing", LOOP_OUT, lu(g.XL[0]) - g.LOOP_W / 2, lu(g.XL[0]) + g.LOOP_W / 2, SL, TONE_SLING, holes=[LOOP], repeat=repeat_x(2, 3.0)))
    out.append(box("s100-sling-leg-a", "sling-leg-a", [g.LEG_LEN, g.LEG_T, g.LEG_W], [g.LEG_LEN / 2, 0, 0], SL, TONE_SLING))
    out.append(box("s100-sling-leg-b", "sling-leg-b", [g.LEG_LEN, g.LEG_T, g.LEG_W], [g.LEG_LEN / 2, 0, 0], SL, TONE_SLING))
    return out


def body_200():
    """🧊 1:200 body."""
    h = hull200()
    lp = loop200()
    out = bearer_solids("s200-")
    out.append(box("block-1-29", "bundle", [g.L, g.V30, 2 * g.EDGE], [g.L / 2, g.V30 / 2, 0], GS, TONE_PACK))
    out.append(box("s200-sheet-30", "sheet-30", [g.L, g.H, 2 * g.EDGE], [g.L / 2, g.H / 2, 0], GS, TONE_TOP, sheet=True))
    out.append(box("s200-battens", "lashing", [0.1, 0.1, 2 * g.BATTEN_HALF], [lu(g.XS[0]), g.BATTEN_V0 + 0.05, 0], WD, TONE_WOOD, repeat=repeat_x(4, 1.8)))
    out.append(ext_u("s200-straps", "lashing", g.offset_convex(h, g.STRAP_T), lu(g.XS[0]) - g.STRAP_W / 2, lu(g.XS[0]) + g.STRAP_W / 2, ST, TONE_STRAP, holes=[h], repeat=repeat_x(4, 1.8)))
    out.append(ext_u("s200-sling-chokes", "lashing", g.offset_convex(lp, g.LOOP_T), lu(g.XL[0]) - F200 / 2, lu(g.XL[0]) + F200 / 2, SL, TONE_SLING, holes=[lp], repeat=repeat_x(2, 3.0)))
    for leg in ("a", "b"):
        out.append(box(f"s200-sling-leg-{leg}", f"sling-leg-{leg}", [g.LEG_LEN, LEG_S500, LEG_S500], [g.LEG_LEN / 2, LEG_S500 / 2 - g.LEG_T / 2, 0], SL, TONE_SLING, slender=True))
    return out




def body_500():
    """🧊 1:500 body."""
    out = bearer_solids("s500-")
    out.append(box("block-1-29", "bundle", [g.L, g.V30, 2 * g.EDGE], [g.L / 2, g.V30 / 2, 0], GS, TONE_PACK, sheet=True))
    out.append(box("s500-sheet-30", "sheet-30", [g.L, g.H, 2 * g.EDGE], [g.L / 2, g.H / 2, 0], GS, TONE_TOP, sheet=True))
    out.append(box("s500-battens", "lashing", [0.1, 0.1, 2 * g.BATTEN_HALF], [lu(g.XS[0]), g.BATTEN_V0 + 0.05, 0], WD, TONE_WOOD, repeat=repeat_x(4, 1.8)))
    for leg in ("a", "b"):
        out.append(box(f"s500-sling-leg-{leg}", f"sling-leg-{leg}", [g.LEG_LEN, LEG_S500, LEG_S500], [g.LEG_LEN / 2, LEG_S500 / 2 - g.LEG_T / 2, 0], SL, TONE_SLING, slender=True))
    return out


def thermal_refs(pairs):
    """🌡️ Thermal references to body solids."""
    return [{"id": f"th-{of}", "of": of, "region": region} for of, region in pairs]


def thermal(scale):
    """🌡️ The thermal list of one scale."""
    if scale == 50:
        pairs = [(f"bearer-{k}", "bearers") for k in range(1, 5)] + [("sheet-30", "top-sheet"), ("sheet-30-edge-crest", "top-sheet"), ("battens", "battens"), ("straps", "lashing-straps"), ("protectors-r", "lashing-straps"), ("protectors-l", "lashing-straps"), ("sling-chokes", "round-slings"), ("sling-leg-a", "round-slings"), ("sling-leg-b", "round-slings")]
        out = thermal_refs(pairs)
        out.append(dict(ext_u("th-bundle-pack", "bundle", g.pack_ring(29, True), 0, g.L, GS, TONE_PACK), region="bundle-sheets"))
        out.append(dict(ext_u("th-bundle-pack-edge-crests", "bundle", g.edge_crest_pack_ring(29), g.CUT, g.L, GS, TONE_PACK), region="bundle-sheets"))
        return out
    p = {100: "s100-", 200: "s200-", 500: "s500-"}[scale]
    pairs = [(f"{p}bearer-{k}", "bearers") for k in range(1, 5)] + [(f"{p}sheet-30", "top-sheet"), (f"{p}battens", "battens")]
    if scale == 100:
        pairs += [("s100-sheet-30-edge-crest", "top-sheet"), ("pack-1-29", "bundle-sheets"), ("pack-1-29-edge-crests", "bundle-sheets"), ("s100-straps", "lashing-straps"), ("s100-protectors-r", "lashing-straps"), ("s100-protectors-l", "lashing-straps"), ("s100-sling-chokes", "round-slings"), ("s100-sling-leg-a", "round-slings"), ("s100-sling-leg-b", "round-slings")]
        return thermal_refs(pairs)
    if scale == 200:
        pairs += [("block-1-29", "bundle-sheets"), ("s200-straps", "lashing-straps"), ("s200-sling-chokes", "round-slings"), ("s200-sling-leg-a", "round-slings"), ("s200-sling-leg-b", "round-slings")]
        return thermal_refs(pairs)
    pairs += [("block-1-29", "bundle-sheets"), ("s500-sling-leg-a", "round-slings"), ("s500-sling-leg-b", "round-slings")]
    out = thermal_refs(pairs)
    h = g.flat_hull(g.TOP)
    out.append(dict(ext_u("th500-straps", "lashing", g.offset_convex(h, g.STRAP_T), lu(g.XS[0]) - g.STRAP_W / 2, lu(g.XS[0]) + g.STRAP_W / 2, ST, TONE_STRAP, holes=[h], slender=True, repeat=repeat_x(4, 1.8)), region="lashing-straps"))
    return out


def member(of, m, id_=None):
    """🏗️ A member reference."""
    return {"id": id_ or f"st-{of}", "of": of, "member": m, "part": "member"}


def structure(scale):
    """🏗️ The structure list of one scale."""
    if scale == 50:
        out = [member(f"bearer-{k}", f"bearer-{k}") for k in range(1, 5)]
        out += [member("sheets-1-29", "bundle"), member("sheets-1-29-edge-crest", "bundle"), member("sheet-30", "sheet-30"), member("sling-leg-a", "sling-leg-a"), member("sling-leg-b", "sling-leg-b")]
        return out
    p = {100: "s100-", 200: "s200-", 500: "s500-"}[scale]
    pack = {100: "pack-1-29", 200: "block-1-29", 500: "block-1-29"}[scale]
    out = [member(f"{p}bearer-{k}", f"bearer-{k}") for k in range(1, 5)]
    out += [member(pack, "bundle"), member(f"{p}sheet-30", "sheet-30")]
    if scale in (100, 200):
        out += [member(f"{p}sling-leg-a", "sling-leg-a"), member(f"{p}sling-leg-b", "sling-leg-b")]
    else:
        out += [member("s500-sling-leg-a", "sling-leg-a"), member("s500-sling-leg-b", "sling-leg-b")]
    return out


VOL_DECISIONS = {
    50: [T("Body: the bundle as it lies: the 29 nested sheets of the true 35/207 profile are one pack extruded from the envelope of their sections (lower face of sheet 1 to the lower face of the top sheet 30, so no air is left under it either), in two pieces because the right edge crest is trimmed back 150 mm at the eaves end (envelope without it over the full 6 m, the edge crests from 0.15 m on). Ruling of the coordinator of 2026-11-23: the 0.75 mm of air between nested sheets is far below the 7.5 mm smallest feature of 1:50, so a body of 1:50 does not model it (29 separate sheets read IoU 0.914 against the band drawing in the front view, the pack 0.972); the top sheet 30 on its own bone; four 100 x 100 x 1200 mm bearers; four 100 x 100 x 1030 mm battens on the top sheet; four composite straps (32 x 1 mm) tensioned round battens, stack and the U-shaped polyethylene edge protectors (2 mm); two violet round slings choked round the stack at 1.50 m and 4.50 m, their legs (50 x 16 mm) lying on the top sheet.", "Körper: das Paket, wie es liegt: die 29 geschachtelten Bleche im wahren Profil 35/207 sind ein Paket, extrudiert aus der Hülle ihrer Querschnitte (Unterseite Blech 1 bis Unterseite des obersten Blechs 30, sodass auch darunter keine Luft bleibt), zweiteilig, weil die rechte Randrippe am Traufende 150 mm zurückgeschnitten ist (Hülle ohne sie über die vollen 6 m, die Randrippen ab 0,15 m). Entscheid der Koordination vom 23.11.2026: die 0,75 mm Luft zwischen geschachtelten Blechen liegen weit unter der kleinsten Einzelheit von 7,5 mm bei 1:50, ein Körper 1:50 bildet sie also nicht ab (29 einzelne Bleche ergeben in der Vorderansicht IoU 0,914 gegen die Bandzeichnung, das Paket 0,972); das oberste Blech 30 auf eigenem Knochen; vier Unterleger 100 x 100 x 1200 mm; vier Kanthölzer 100 x 100 x 1030 mm auf dem obersten Blech; vier Polyesterbänder (32 x 1 mm) um Kanthölzer, Stapel und den U-förmigen Kantenschutz aus Polyethylen (2 mm); zwei violette Rundschlingen bei 1,50 m und 4,50 m um den Stapel geschnürt, ihre Stränge (50 x 16 mm) auf dem obersten Blech liegend."),
       T("The 6.5 mm old screw holes are below the 7.5 mm feature and are drawn in plan only; the white-rust patina is the lighter tone of the top sheet.", "Die alten Schraublöcher von 6,5 mm liegen unter der Einzelheit von 7,5 mm und sind nur in der Draufsicht gezeichnet; die Weißrostpatina ist der hellere Ton des obersten Blechs."),
       T("Thermal: references to the body solids, one region per part family, except the 29 nested sheets, whose 0.75 mm nesting gaps exchange no air with the yard: their region is the envelope pack of the bundle (lower face of sheet 1 to upper face of sheet 29, corner trim kept); structure: the bundle, the top sheet, the bearers and the sling legs as members on their own bones (the sheets are the member), the legs end at the hooks.", "Thermik: Verweise auf die Körper, eine Region je Teilefamilie, außer den 29 geschachtelten Blechen, deren Schachtelspalte von 0,75 mm keine Luft mit dem Lagerplatz tauschen: ihre Region ist das Hüllpaket des Bündels (Unterseite Blech 1 bis Oberseite Blech 29, mit Eckschnitt); Statik: Paket, oberstes Blech, Unterleger und Schlingenstränge als Stäbe auf eigenen Knochen (die Bleche sind der Stab), die Stränge enden an den Haken.")],
    100: [T("Body: the 29 nested sheets merge into one pack extruded from the envelope of their true profile (lower face of sheet 1 to upper face of sheet 29), still trimmed at the eaves corner; the top sheet keeps its true profile and its own corner trim (two pieces, as at 1:50); the edge protectors become 15 mm plates outside the sheet edges and the straps run round them; slings and legs keep their sizes (25 and 16 mm, above the 15 mm feature).", "Körper: die 29 geschachtelten Bleche verschmelzen zu einem Paket, extrudiert aus der Hülle ihres wahren Profils (Unterseite Blech 1 bis Oberseite Blech 29), weiterhin mit dem Traufeckschnitt; das oberste Blech behält sein wahres Profil und seinen eigenen Traufeckschnitt (zwei Teile wie bei 1:50); der Kantenschutz wird zu 15 mm starken Platten außen an den Kanten, die Bänder laufen darum; Schlingen und Stränge behalten ihre Maße (25 und 16 mm, über der Einzelheit von 15 mm)."),
        T("Thermal and structure: as at 1:50 on these solids; the pack is the bundle member.", "Thermik und Statik: wie 1:50 auf diesen Körpern; das Paket ist der Paketstab.")],
    200: [T("Body: the profile is generalised to its rectangular envelope (the 25 mm web run is below the 30 mm feature): the bundle is a 43.5 mm block, the top sheet a 35 mm declared sheet; straps run round the block and the battens; edge protectors merge into the straps; sling chokes are drawn and modelled 30 mm wide, the legs are slender 36 mm bars drawn as lines.", "Körper: das Profil ist auf seine rechteckige Hülle vereinfacht (der Steglauf von 25 mm liegt unter der Einzelheit von 30 mm): das Paket ist ein Block von 43,5 mm, das oberste Blech eine deklarierte 35-mm-Platte; die Bänder laufen um Block und Kanthölzer; der Kantenschutz geht in den Bändern auf; Schlingenschnürungen sind 30 mm breit gezeichnet und modelliert, die Stränge sind schlanke Stäbe von 36 mm, als Linien gezeichnet."),
        T("Thermal and structure on these solids.", "Thermik und Statik auf diesen Körpern.")],
    500: [T("Body: bearers and battens (100 mm), the bundle block and the top sheet as declared sheets (43.5 and 35 mm, more than twenty times as wide); the sling legs as slender bars of 36 mm; straps, edge protectors and chokes are below the 75 mm feature and left out of the body.", "Körper: Unterleger und Kanthölzer (100 mm), der Paketblock und das oberste Blech als deklarierte Platten (43,5 und 35 mm, mehr als zwanzigmal so breit); die Schlingenstränge als schlanke Stäbe von 36 mm; Bänder, Kantenschutz und Schnürungen liegen unter der Einzelheit von 75 mm und fehlen im Körper."),
        T("Thermal: the straps stay as slender hoops so their region keeps a surface.", "Thermik: die Bänder bleiben als schlanke Reifen, damit ihre Region eine Fläche behält."),
        T("Minimal: the structure list is already one box member per load-bearing bone (bundle block, top sheet, four bearers, two sling legs as 36 mm slender bars); no bone can be merged into another, because the bearers stand 100 mm below the bundle and the legs leave it upwards, so the list keeps the eight members of 1:200 with its own solids.", "Minimal: die Statikliste ist bereits ein Quader-Stab je tragendem Knochen (Paketblock, oberstes Blech, vier Unterleger, zwei Schlingenstränge als schlanke Stäbe von 36 mm); kein Knochen lässt sich mit einem anderen zusammenfassen, weil die Unterleger 100 mm unter dem Paket stehen und die Stränge es nach oben verlassen, daher behält die Liste die acht Stäbe von 1:200 mit eigenen Körpern.")],
}

BODIES = {50: body_50, 100: body_100, 200: body_200, 500: body_500}


def volume(s):
    """🧊 The authored volume of one scale."""
    return {"status": "authored", "denominator": s, "body": BODIES[s](), "thermal": thermal(s), "structure": structure(s), "decisions": VOL_DECISIONS[s]}


AREAS = {"top-sheet": 15.1, "bundle-sheets": 15.4, "bearers": 2.0, "battens": 1.73, "lashing-straps": 0.85, "round-slings": 0.55}
REGION_DEFS = [
    ("top-sheet", T("Top sheet 30, both faces", "Oberstes Blech 30, beide Seiten"), "surface", GS, "horizontal-plate-up", 0.207, 0.5, SHEET_KG * 460, "ashrae-2021-radiation"),
    ("bundle-sheets", T("Sheets 1 to 29 of the bundle", "Bleche 1 bis 29 des Pakets"), "surface", GS, "horizontal-plate-down", 0.207, 0.1, 29 * SHEET_KG * 460, "ashrae-2021-radiation"),
    ("bearers", T("Squared-timber bearers", "Kantholz-Unterleger"), "surface", WD, "horizontal-plate-up", 0.1, 0.3, None, "en-iso-13786"),
    ("battens", T("Battens on top of the bundle", "Kanthölzer auf dem Paket"), "surface", WD, "horizontal-plate-up", 0.1, 0.9, None, "en-iso-13786"),
    ("lashing-straps", T("Straps and edge protectors", "Umreifungsbänder und Kantenschutz"), "surface", ST, "vertical-plate", 0.032, 0.5, None, "en-12195-2"),
    ("round-slings", T("Round slings", "Rundschlingen"), "surface", SL, "cylinder", 0.03, 0.5, None, "en-1492-2"),
]


def regions():
    """🔥 The thermal regions."""
    out = []
    for rid, title, kind, mat, geom, lc, sky, cap_total, ref in REGION_DEFS:
        area = AREAS[rid]
        cap = round(cap_total / area) if cap_total else (round(WOOD_RHO * 1600 * 0.036) if mat == WD else round(1200 * 1200 * 0.001))
        reg = {"id": rid, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": area, "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": ref}
        if rid == "top-sheet":
            reg["ratedPowerW"] = 0
            reg["ratedPowerBasis"] = "passive"
        out.append(reg)
    return out


def binding(over):
    """🌬️ The thermal binding of a pose."""
    base = {"top-sheet": (0.5, 0.5), "bundle-sheets": (0.5, 0.0), "bearers": (0.7, 0.1), "battens": (0.95, 0.5), "lashing-straps": (1.0, 0.4), "round-slings": (1.0, 0.4)}
    base.update(over)
    return {"regions": {k: {"exposedFraction": v[0], "sunlitFraction": v[1]} for k, v in base.items()}, "emitters": {}}


BEARER_CONTACTS = [f"bearer-{k}-ground" for k in range(1, 5)]
PURLINS = [f"purlin-{k}" for k in range(1, 5)]
LIFTED_BONES = {"bundle": {"dy": g.LIFT}, **{f"bearer-{k}": {"dy": -g.LIFT} for k in range(1, 5)}, "sling-leg-a": {"angleDeg": -90}, "sling-leg-b": {"angleDeg": 90}}
TENSIONED_BONES = {"sling-leg-a": {"angleDeg": -90}, "sling-leg-b": {"angleDeg": 90}}
REINSTALLED_BONES = {"sheet-30": {"dy": r6(g.STRAP_TOP - 0.0015 - g.V30)}, "lashing": {"dy": -0.0015}}


def snow_load():
    """❄️ Snow on the reinstalled sheet."""
    return {"id": "snow", "kind": "snow", "title": T("Snow 0.68 kN/m2 on the 1.035 m cover width", "Schnee 0,68 kN/m2 auf 1,035 m Deckbreite"), "fxN": 0, "fyN": 0, "distributed": {"bone": "sheet-30", "fromM": 0, "toM": g.L, "wxNPerM": 0, "wyNPerM": -round(SNOW * 1.035)}, "source": "literature", "reference": "en-1991-1-3"}


def suction_load():
    """🌬️ Wind suction on the reinstalled sheet."""
    return {"id": "wind-suction", "kind": "wind", "title": T("Wind suction 0.60 kN/m2 on the 1.035 m cover width", "Windsog 0,60 kN/m2 auf 1,035 m Deckbreite"), "fxN": 0, "fyN": 0, "distributed": {"bone": "sheet-30", "fromM": 0, "toM": g.L, "wxNPerM": 0, "wyNPerM": round(SUCTION * 1.035)}, "source": "literature", "reference": "en-1991-1-4"}


def yard_reactions(kg_total, en, de):
    """🪨 The determinate yard reactions under the four bearers of a body that is supported separately, applied as known carried loads (the solver is one rigid body)."""
    share = round(kg_total * 9.81 / 4, 1)
    return [{"id": f"yard-under-bearer-{k}", "kind": "carried", "title": T(f"{en} under bearer {k}: {share / 1000:.3f} kN".replace(".", "."), f"{de} unter Unterleger {k}: {share / 1000:.3f} kN".replace(".", ",")), "anchor": f"bearer-{k}-ground", "fxN": 0, "fyN": share, "source": "authored-estimate", "reference": "en-338"} for k in range(1, 5)]


YARD_REINSTALLED = yard_reactions(TOTAL_KG - SHEET_KG, "Yard reaction of the remaining bundle", "Lagerplatzreaktion des übrigen Pakets")
YARD_LIFTED = yard_reactions(4 * BEARER_KG, "Yard reaction of the bearer left behind", "Lagerplatzreaktion des zurückgebliebenen Unterlegers")


def pose(id_, en, de, den, dde, bones, contacts, loads=(), key=False, therm=None):
    """📥 One pose."""
    p = {"id": id_, "title": T(en, de), "description": T(den, dde)}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "authored", "contacts": contacts, "thermal": binding(therm or {}), "loads": list(loads)})
    return p


REIN_TH = {"top-sheet": (1.0, 0.5), "bundle-sheets": (0.5, 0.0), "battens": (0.9, 0.3)}
POSES = [
    pose("stored", "Stored on bearers", "Auf Unterlegern gelagert", "Thirty nested sheets on four squared-timber bearers in the yard, strapped with battens on top, the two round slings choked and their legs laid on the top sheet.", "Dreißig geschachtelte Bleche auf vier Kanthölzern im Lager, mit Kanthölzern obenauf umreift, die beiden Rundschlingen geschnürt und ihre Stränge auf das oberste Blech gelegt.", {}, BEARER_CONTACTS),
    pose("slings-tensioned", "Slings tensioned", "Schlingen gespannt", "The sling legs are raised to the hooks of the spreader beam before the bundle leaves the bearers.", "Die Schlingenstränge sind zu den Haken der Traverse hochgezogen, bevor das Paket die Unterleger verlässt.", TENSIONED_BONES, BEARER_CONTACTS, key=True),
    pose("lifted", "Lifted on two slings", "An zwei Schlingen gehoben", "The strapped bundle hangs 1.5 m up from two vertical round-sling legs choked at 1.50 m and 4.50 m (0.25 L from the ends); the four bearers stay on the ground.", "Das umreifte Paket hängt 1,5 m hoch an zwei senkrechten Rundschlingensträngen, geschnürt bei 1,50 m und 4,50 m (0,25 L von den Enden); die vier Unterleger bleiben am Boden.", LIFTED_BONES, ["hook-a", "hook-b"], YARD_LIFTED, therm={"bearers": (0.95, 0.5), "bundle-sheets": (0.6, 0.0)}),
    pose("reinstalled", "Top sheet reinstalled on purlins", "Oberstes Blech auf Pfetten wieder eingebaut", "Sheet 30 is taken off the bundle and screwed down with its valleys on the four battens, which serve as purlin lines at 1.80 m spacing (three spans, 0.30 m overhangs); the rest of the bundle stays strapped on its bearers.", "Blech 30 ist vom Paket genommen und mit seinen Untergurten auf die vier Kanthölzer geschraubt, die als Pfettenlinien im Abstand von 1,80 m dienen (drei Felder, 0,30 m Überstand); das übrige Paket bleibt umreift auf seinen Unterlegern.", REINSTALLED_BONES, PURLINS, YARD_REINSTALLED, therm=REIN_TH),
    pose("reinstalled-snow", "Reinstalled under snow", "Wieder eingebaut unter Schnee", "The reinstalled sheet carries the design snow load of 0.68 kN/m2 (zone 2, low altitude, flat roof).", "Das wieder eingebaute Blech trägt die Schneelast von 0,68 kN/m2 (Zone 2, geringe Höhe, Flachdach).", REINSTALLED_BONES, PURLINS, [snow_load(), *YARD_REINSTALLED], therm=REIN_TH),
    pose("reinstalled-wind-suction", "Reinstalled under wind suction", "Wieder eingebaut unter Windsog", "Wind suction of 0.60 kN/m2 on the edge zone lifts the reinstalled sheet against its screws.", "Windsog von 0,60 kN/m2 im Randbereich hebt das wieder eingebaute Blech gegen seine Schrauben.", REINSTALLED_BONES, PURLINS, [suction_load(), *YARD_REINSTALLED], therm=REIN_TH),
]

CLIPS = [
    {"id": "lift", "title": T("Lift off the bearers", "Abheben von den Unterlegern"), "description": T("The sling legs rise to the hooks, then the bundle is lifted 1.5 m clear and the bearers stay behind.", "Die Schlingenstränge steigen zu den Haken, dann wird das Paket 1,5 m angehoben und die Unterleger bleiben liegen."),
     "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "slings-tensioned", "ease": "smooth"}, {"t": 6, "pose": "lifted", "ease": "smooth"}],
     "contacts": [*({"anchor": a, "fromS": 0, "toS": 2.2} for a in BEARER_CONTACTS), {"anchor": "hook-a", "fromS": 2, "toS": 6}, {"anchor": "hook-b", "fromS": 2, "toS": 6}],
     "phases": [{"id": "raise-legs", "fromS": 0, "toS": 2, "title": T("Raise the sling legs", "Schlingenstränge hochziehen")}, {"id": "hoist", "fromS": 2, "toS": 6, "title": T("Hoist", "Heben")}],
     "events": [{"t": 2, "kind": "contact-on", "title": T("Slings take the load", "Schlingen übernehmen die Last"), "target": "hook-a"}, {"t": 2.2, "kind": "contact-off", "title": T("Bundle leaves the bearers", "Paket verlässt die Unterleger"), "target": "bearer-1-ground"}],
     "tracks": []},
    {"id": "snow-loading", "title": T("Snow builds up on the reinstalled sheet", "Schnee baut sich auf dem wieder eingebauten Blech auf"), "description": T("The snow load on the reinstalled sheet rises from zero to 0.68 kN/m2 and holds.", "Die Schneelast auf dem wieder eingebauten Blech steigt von null auf 0,68 kN/m2 und bleibt."),
     "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-snow", "ease": "smooth"}, {"t": 6, "pose": "reinstalled-snow", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in PURLINS],
     "phases": [{"id": "snowfall", "fromS": 0, "toS": 5, "title": T("Snowfall", "Schneefall")}, {"id": "hold", "fromS": 5, "toS": 6, "title": T("Hold", "Halten")}],
     "events": [{"t": 5, "kind": "load-on", "title": T("Snow load at full value", "Schneelast vollständig"), "target": "snow"}],
     "tracks": [{"target": "load:snow", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 5, "v": 1}, {"t": 6, "v": 1}]}]},
]


def document():
    """📄 The whole asset document."""
    height = g.BEARER + g.STRAP_TOP
    asset = {
        "id": "reclaimed-trapezoidal-sheet-stack",
        "category": "components",
        "title": T("Reclaimed Trapezoidal Sheet Stack", "Gebrauchter Trapezblechstapel"),
        "summary": T("Thirty nested galvanised trapezoidal roof sheets 35/207, 0.75 mm, 6.00 x 1.07 m, taken off an old roof: screw holes of the old fastening in the crests, the edge crest trimmed at the eaves corner, weathered zinc; strapped with four battens on four squared-timber bearers, lifted on two choked round slings, and the top sheet reinstalled on four purlin lines under snow and wind suction.",
                   "Dreißig geschachtelte feuerverzinkte Trapezbleche 35/207, 0,75 mm, 6,00 x 1,07 m, von einem alten Dach genommen: Schraublöcher der alten Befestigung in den Obergurten, die Randrippe an der Traufecke zurückgeschnitten, bewittertes Zink; mit vier Kanthölzern auf vier Unterlegern umreift, an zwei geschnürten Rundschlingen gehoben und das oberste Blech auf vier Pfettenlinien unter Schnee und Windsog wieder eingebaut."),
        "tags": ["component", "reclaimed", "steel", "galvanised", "sheet-metal", "roofing", "trapezoidal-sheet", "stack", "timber"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": ref_list(),
                       "notes": T(f"Profile 35/207: 35 mm rib, 207 mm pitch, five pitches plus overlap crest, cover width 1.035 m, sheet width 1.070 m, crest 35 mm, web run 25 mm, valley 122 mm; 0.75 mm nominal thickness; gross section {A_SHEET * 1e6:.0f} mm2, {SHEET_KG:.1f} kg per 6 m sheet (7.08 kg/m2, catalogue 6.93 to 7.30); effective I 12.6 cm4/m back-calculated from the W-35 load table (gross {I_GROSS / 1.035 * 1e8:.1f} cm4/m). Nesting pitch 1.5 mm (geometric minimum 1.29 mm set by the 54.5 degree webs, plus dents and burrs of the reclaimed sheets: authored estimate). Old purlin spacing 1.45 m from the hole rows; new purlin lines 1.80 m.",
                                  f"Profil 35/207: 35 mm Rippe, 207 mm Teilung, fünf Teilungen plus Überlappungsrippe, Deckbreite 1,035 m, Blechbreite 1,070 m, Obergurt 35 mm, Steglauf 25 mm, Untergurt 122 mm; Nenndicke 0,75 mm; Bruttoquerschnitt {A_SHEET * 1e6:.0f} mm2, {SHEET_KG:.1f} kg je 6-m-Blech (7,08 kg/m2, Katalog 6,93 bis 7,30); wirksames I 12,6 cm4/m aus der Belastungstabelle W-35 rückgerechnet (brutto {I_GROSS / 1.035 * 1e8:.1f} cm4/m). Schachtelteilung 1,5 mm (geometrisches Minimum 1,29 mm durch die 54,5-Grad-Stege, dazu Dellen und Grate der gebrauchten Bleche: Schätzung). Alter Pfettenabstand 1,45 m aus den Lochreihen; neue Pfettenlinien 1,80 m.")},
        "dimensions": {"heightM": r6(height), "widthM": g.L, "depthM": g.BEARER_LEN, "massKg": round(TOTAL_KG, 2)},
        "materials": MATERIALS,
        "rig": {"kind": "structural", "root": "yard", "bones": BONES},
        "anchors": ANCHORS,
        "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
        "symbols": [],
        "poses": POSES,
        "clips": CLIPS,
        "thermal": {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
                    "operating": [], "regions": regions(), "emitters": [],
                    "assumptions": [T("All surfaces are passive: the stack is an inert object in the yard; weathered zinc (emittance 0.28, solar absorptance 0.65) is far more emissive than new zinc (0.04 to 0.13), so a reclaimed sheet runs cooler in the sun than a new one.", "Alle Flächen sind passiv: der Stapel ist ein träger Gegenstand im Lager; bewittertes Zink (Emissionsgrad 0,28, solare Absorption 0,65) strahlt weit stärker als neues Zink (0,04 bis 0,13), ein gebrauchtes Blech bleibt in der Sonne daher kühler als ein neues."),
                                    T("Sheet regions carry the whole heat capacity of their steel on their outer area; the bundle stores the heat of 29 sheets and reacts slowly, the single top sheet within minutes.", "Die Blechregionen tragen die ganze Wärmekapazität ihres Stahls auf ihrer Außenfläche; das Paket speichert die Wärme von 29 Blechen und reagiert träge, das einzelne oberste Blech innerhalb von Minuten."),
                                    T("Timber stores heat in an effective depth of 36 mm (EN ISO 13786); straps and slings are thin polymer surfaces.", "Holz speichert Wärme in einer wirksamen Tiefe von 36 mm (EN ISO 13786); Bänder und Schlingen sind dünne Kunststoffflächen.")],
                    "passengers": []},
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [],
                      "assumptions": [T("Stored: the bundle rests on its four bearers, which stand on the yard (unilateral contacts with friction 0.5); the 29 nested sheets act side by side, each with the effective I of 12.6 cm4/m (EN 1993-1-3 effective section from the W-35 table).", "Gelagert: das Paket liegt auf seinen vier Unterlegern, die auf dem Lagerplatz stehen (einseitige Kontakte, Reibung 0,5); die 29 geschachtelten Bleche wirken nebeneinander, jedes mit dem wirksamen I von 12,6 cm4/m (wirksamer Querschnitt nach EN 1993-1-3 aus der Tabelle W-35)."),
                                      T("Lifted: two vertical sling legs to a spreader beam (pin and roller at the hooks) carry the bundle at 0.25 L from its ends; the solver treats the asset as one rigid body, so the bearers left on the ground stand on their known yard reactions (5.4 kg each), applied as carried loads, and the hooks carry the bundle alone.", "Gehoben: zwei senkrechte Schlingenstränge zu einer Traverse (Fest- und Loslager an den Haken) tragen das Paket bei 0,25 L von den Enden; der Löser behandelt das Objekt als einen starren Körper, daher stehen die am Boden zurückgebliebenen Unterleger auf ihren bekannten Lagerplatzreaktionen (je 5,4 kg), als getragene Lasten angesetzt, und die Haken tragen allein das Paket."),
                                      T("Reinstalled: sheet 30 is a three-span beam on four purlin lines at 1.80 m (pin at the first, rollers at the others, all bilateral as screwed valleys); the remaining bundle stands on its bearers with its determinate yard reactions (a quarter of 1315.5 kg each, symmetric bearers) applied as known carried loads, so the purlin lines carry the sheet alone; among the four purlin lines the solver still shares by the equal-stiffness rule, not by continuous-beam coefficients.", "Wieder eingebaut: Blech 30 ist ein Dreifeldträger auf vier Pfettenlinien im Abstand von 1,80 m (Festlager an der ersten, Loslager an den übrigen, alle zweiseitig als verschraubte Untergurte); das übrige Paket steht auf seinen Unterlegern, mit seinen statisch bestimmten Lagerplatzreaktionen (je ein Viertel von 1315,5 kg, symmetrische Unterleger) als bekannte getragene Lasten, daher tragen die Pfettenlinien allein das Blech; unter den vier Pfettenlinien teilt der Löser weiterhin nach der Regel gleicher Steifigkeit, nicht nach Durchlaufträgerbeiwerten.")],
                      "clinical": False},
        "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("[DEBUG] mass", round(TOTAL_KG, 2), "sheet", round(SHEET_KG, 2), "height", g.BEARER + g.STRAP_TOP)
