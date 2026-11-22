"""🧱️ Authoritative chisel of the reclaimed hollow-core slab: every hand-chosen number of the asset lives here and the script writes `🔣️.json` next to it.

Run: `python3 author-slab.py` (prints the reference numbers of the report).

@see ../../brief.md — the asset contract of the collection
@see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — Volume, Front and plan views, Power
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))

L = 6.0
Y0 = 0.20
RHO_C = 2500.0
RHO_G = 2100.0
RHO_S = 7850.0
RHO_T = 420.0
CORE_R = 0.070
CORE_V = 0.003
CORE_W = [-0.475, -0.285, -0.095, 0.095, 0.285, 0.475]
STRAND_W = [-0.38, -0.19, 0.0, 0.19, 0.38]
STRAND_V = -0.060
STRAND_R = 0.00625
BEARER_X = [0.45, 5.55]
LIFT_X = [0.60, 5.40]
LIFT_Z = [0.475, -0.475]
PIVOT_V = 0.16
BAIL_LEN = 0.085
LIFT_DY = 0.90
REINSTALL_DY = 2.50

EDGE = {
    "50": [(-0.100, 0.598), (-0.070, 0.598), (-0.055, 0.588), (-0.020, 0.588), (-0.005, 0.578), (0.040, 0.578), (0.055, 0.588), (0.100, 0.583)],
    "100": [(-0.100, 0.598), (-0.055, 0.598), (-0.040, 0.588), (0.100, 0.583)],
    "200": [(-0.100, 0.598), (0.100, 0.583)],
    "500": [(-0.100, 0.598), (0.100, 0.598)],
}
GROUT = {
    "50": [(-0.040, 0.588), (-0.020, 0.588), (-0.005, 0.578), (0.040, 0.578), (0.055, 0.588), (0.070, 0.5863), (0.064, 0.597), (0.030, 0.601), (-0.012, 0.600), (-0.034, 0.596)],
    "100": [(-0.040, 0.588), (0.070, 0.5846), (0.064, 0.602), (-0.034, 0.602)],
}
PATCH_L = [(0.35, 1.70), (2.60, 3.30), (4.10, 5.75)]
PATCH_R = [(0.00, 0.95), (1.85, 3.60), (4.70, 5.30)]
FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.030, "500": 0.075}
SKIN = {"50": 0.010, "100": 0.005, "200": 0.004}
EDGE_BAND = {"50": 0.010, "100": 0.015, "200": 0.030}
TUBE = {"50": 0.010, "100": 0.005, "200": 0.004}
FLANGE_TOP = 0.068
FLANGE_BOTTOM = -0.062
END_SLICE = 0.008


def T(en, de):
    return {"en": en, "de": de}


def dn(x, nd):
    return f"{x:.{nd}f}".replace(".", ",")


def r6(x):
    return round(x + 0.0, 6)


def section(scale):
    e = EDGE[scale]
    return [(v, w) for v, w in e] + [(v, -w) for v, w in reversed(e)]


def circle(cv, cw, r, n=32, start=0.0):
    return [(r6(cv + r * math.sin(start + 2 * math.pi * k / n)), r6(cw + r * math.cos(start + 2 * math.pi * k / n))) for k in range(n)]


def cores(n=32, r=CORE_R):
    return [circle(CORE_V, w, r, n) for w in CORE_W]


def shoelace(poly):
    a = cx = cy = ixx = 0.0
    for i in range(len(poly)):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % len(poly)]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
        ixx += (x0 * x0 + x0 * x1 + x1 * x1) * c
    a /= 2.0
    return abs(a), cx / (6 * a), cy / (6 * a), abs(ixx / 12.0)


def section_properties(scale):
    a, vc, wc, iv = shoelace(section(scale))
    holes = [shoelace(h) for h in cores(256)]
    an = a - sum(h[0] for h in holes)
    vcn = (a * vc - sum(h[0] * h[1] for h in holes)) / an
    i0 = iv - sum(h[3] for h in holes)
    ic = i0 - an * vcn * vcn
    return an, vcn, ic, max(0.1 - vcn, vcn + 0.1)


def pts(poly):
    return [[r6(a), r6(b)] for a, b in poly]


def front_of(poly):
    return [[r6(w), r6(v + Y0)] for v, w in poly]


def path_d(rings):
    out = []
    for ring in rings:
        out.append("M " + " L ".join(f"{r6(x)} {r6(y)}" for x, y in ring) + " Z")
    return " ".join(out)


def jag(x0, x1, base, amp, seed, step=0.12):
    n = max(2, int(round((x1 - x0) / step)))
    out = []
    for k in range(n + 1):
        x = x0 + (x1 - x0) * k / n
        d = 0.0 if k in (0, n) else amp * math.sin(seed * 12.9898 + k * 78.233) * 0.9
        out.append((r6(x), r6(base + d)))
    return out


AREA_NET, V_C, I_W, E_FIBRE = section_properties("50")
GROUT_A = shoelace(GROUT["50"])[0]
GROUT_LEN = sum(b - a for a, b in PATCH_L + PATCH_R)
SLAB_KG = AREA_NET * L * RHO_C + GROUT_A * GROUT_LEN * RHO_G
BEARER_KG = 0.10 * 0.10 * 1.40 * RHO_T
BOLT_KG = math.pi * 0.012 ** 2 * 0.25 * RHO_S
WASHERS_KG = 2 * 0.10 * 0.10 * 0.010 * RHO_S
NUT_KG = 0.036 * 0.036 * 0.019 * RHO_S * 0.75
BASE_KG = math.pi * 0.035 ** 2 * 0.040 * RHO_S
BAIL_A = 0.07 * 0.05 + math.pi * 0.035 ** 2 / 2 - (0.03 * 0.035 + math.pi * 0.015 ** 2 / 2)
BAIL_KG = BAIL_A * 0.020 * RHO_S
LIFT_KG = BOLT_KG + WASHERS_KG + NUT_KG + BASE_KG + BAIL_KG
TOTAL_KG = SLAB_KG + 2 * BEARER_KG + 4 * LIFT_KG
HEIGHT = Y0 + PIVOT_V + 0.010

REFS = [
    {"id": "en-1168", "citation": "EN 1168:2005+A3:2011 Precast concrete products, hollow core slabs: geometry of the units (depth, width, longitudinal joints with shear keys), cores, prestressing strands, tolerances and handling and storage on supports near the ends."},
    {"id": "fib-bulletin-6", "citation": "fib Bulletin 6 (2000) Special design considerations for precast prestressed hollow core floors: longitudinal joint profiles with grout keys, bearing length of 70 to 100 mm on supports, handling of hollow core units."},
    {"id": "recreate-reuse", "citation": "ReCreate (EU Horizon 2020 grant 958200, 2021-2025), Reusing precast concrete for a circular economy: pilot deconstruction of hollow core floors, saw-cutting of the units next to the old bearings, lifting through drilled lifting points, intermediate storage on timber bearers."},
    {"id": "en-1992-1-1", "citation": "EN 1992-1-1:2004 Eurocode 2, Table 3.1: concrete C45/55, fck 45 MPa, Ecm 36 GPa."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Table A.1 (reinforced and prestressed normal-weight concrete 25 kN/m3, cement mortar 19 to 23 kN/m3), Table 6.1 and 6.2 (category A floors: qk 2.0 kN/m2, Qk 2.0 kN)."},
    {"id": "en-10138-3", "citation": "EN 10138-3 (draft, as used in national approvals) Prestressing steels, strand Y1860S7 nominal diameter 12.5 mm, cross-section 93 mm2."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes: C24 E0,mean 11 GPa, mean density 420 kg/m3, fm,k 24 MPa."},
    {"id": "din-582", "citation": "DIN 582:2018 Lifting eye nuts and EN 1677-1 components for slings: catalogue-typical M24 lifting points (working load limit about 1 t per point in vertical pull)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties: concrete 2400 kg/m3 lambda 2.0 W/mK, c 1000 J/kgK; softwood 0.13 W/mK, 1600 J/kgK; steel 50 W/mK, 450 J/kgK; cement mortar 1.4 W/mK."},
    {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); 151 mm for concrete over 24 h."},
    {"id": "emissivity-literature", "citation": "ASHRAE Handbook Fundamentals (2021), thermal radiation properties of building materials: weathered concrete about 0.9 to 0.94, white emulsion paint 0.9 with solar absorptance about 0.3, weathered timber 0.9, weathered galvanised steel 0.25 to 0.3."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
    {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for areas on sections and cuts (hatching)."},
]

MATERIALS = [
    {"id": "concrete", "title": T("Prestressed concrete C45/55, weathered", "Spannbeton C45/55, verwittert"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": RHO_C, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "elasticModulusPa": 36e9, "strengthPa": 45e6, "source": "literature", "reference": "en-1992-1-1"},
    {"id": "sawn-concrete", "title": T("Saw-cut concrete face", "Gesägte Betonfläche"), "emissivity": 0.90, "solarAbsorptivity": 0.55, "densityKgM3": RHO_C, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "emissivity-literature"},
    {"id": "soffit-paint", "title": T("Concrete soffit with white emulsion ceiling paint", "Betonuntersicht mit weißer Dispersions-Deckenfarbe"), "emissivity": 0.90, "solarAbsorptivity": 0.30, "densityKgM3": RHO_C, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "emissivity-literature"},
    {"id": "grout", "title": T("Joint grout remnants (cement mortar)", "Fugenvergussreste (Zementmörtel)"), "emissivity": 0.92, "solarAbsorptivity": 0.60, "densityKgM3": RHO_G, "conductivityWmK": 1.4, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-1991-1-1"},
    {"id": "strand", "title": T("Prestressing strand Y1860S7, cut end", "Spannlitze Y1860S7, Schnittende"), "emissivity": 0.60, "solarAbsorptivity": 0.70, "densityKgM3": RHO_S, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 195e9, "strengthPa": 1860e6, "source": "literature", "reference": "en-10138-3"},
    {"id": "softwood", "title": T("Weathered softwood bearer C24", "Verwittertes Nadelholz-Kantholz C24"), "emissivity": 0.90, "solarAbsorptivity": 0.70, "densityKgM3": RHO_T, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
    {"id": "lifting-steel", "title": T("Zinc-plated lifting steel, weathered", "Verzinkter Anschlagstahl, verwittert"), "emissivity": 0.30, "solarAbsorptivity": 0.60, "densityKgM3": RHO_S, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 640e6, "source": "authored-estimate", "reference": "din-582"},
]


def bone(id_, parent, title, x, y, angle, length, joint, z=None, mass=None, sec=None):
    b = {"id": id_, "parent": parent, "title": title, "x": x, "y": y}
    if z is not None:
        b["z"] = z
    b.update({"angleDeg": angle, "lengthM": length, "joint": joint})
    if mass:
        b["mass"] = mass
    if sec:
        b["section"] = sec
    return b


BAILS = [("a", "l", 0), ("a", "r", 1), ("b", "l", 0), ("b", "r", 1)]


def bail_id(e, s):
    return f"bail-{e}-{s}"


def bail_x(e):
    return LIFT_X[0] if e == "a" else LIFT_X[1]


def bail_z(s):
    return LIFT_Z[0] if s == "l" else LIFT_Z[1]


def rig():
    bones = [
        bone("slab", None, T("Hollow-core slab", "Spannbeton-Hohlplatte"), 0, Y0, 0, L, {"type": "fixed"},
             mass={"kg": r6(SLAB_KG), "comX": L / 2, "comY": r6(V_C), "distribution": "uniform", "source": "authored-estimate", "reference": "en-1991-1-1"},
             sec={"areaM2": r6(AREA_NET), "secondMomentM4": float(f"{I_W:.6e}"), "extremeFibreM": r6(E_FIBRE), "material": "concrete", "title": T("Hollow-core 1196 x 200 mm, six cores of 140 mm", "Hohlplatte 1196 x 200 mm, sechs Hohlräume 140 mm")}),
    ]
    for k, x in enumerate(BEARER_X):
        n = "ab"[k]
        bones.append(bone(f"bearer-{n}", "slab", T(f"Timber bearer {n.upper()} (stays on the ground)", f"Kantholz {n.upper()} (bleibt am Boden)"), x, -0.10, -90, 0.10,
                          {"type": "prismatic", "axisDeg": -90, "minM": 0, "maxM": 2.6},
                          mass={"kg": r6(BEARER_KG), "comX": 0.05, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "en-338"},
                          sec={"areaM2": 0.01, "secondMomentM4": 8.333333e-06, "extremeFibreM": 0.05, "material": "softwood", "title": T("Squared timber 100 x 100 mm", "Kantholz 100 x 100 mm")}))
    for e, s, _ in BAILS:
        side_en = "left" if s == "l" else "right"
        side_de = "links" if s == "l" else "rechts"
        a = 0 if e == "a" else 180
        lim = {"type": "revolute", "minDeg": 0, "maxDeg": 90} if e == "a" else {"type": "revolute", "minDeg": -90, "maxDeg": 0}
        bones.append(bone(bail_id(e, s), "slab", T(f"Swivel bail of lifting point {e.upper()} {side_en}", f"Klappbügel des Anschlagpunkts {e.upper()} {side_de}"), bail_x(e), PIVOT_V, a, BAIL_LEN, lim, z=bail_z(s),
                          mass={"kg": r6(LIFT_KG), "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate", "reference": "din-582"}))
    bones.append(bone("load-carrier", "slab", T("Load carrier (massless, moves the point load)", "Lastträger (masselos, bewegt die Einzellast)"), 0.5, 0.0, 0, 0,
                      {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 5.0}))
    return {"kind": "structural", "root": "slab", "bones": bones}


def anchors():
    out = []
    for k in range(2):
        n = "ab"[k]
        out.append({"id": f"bearer-{n}-ground", "bone": f"bearer-{n}", "x": 0.10, "y": 0, "kind": "contact", "title": T(f"Bearer {n.upper()} on the yard ground", f"Kantholz {n.upper()} auf dem Lagerplatz"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 1.40, "frictionCoefficient": 0.5})
    for e, s, _ in BAILS:
        out.append({"id": f"sling-{e}-{s}", "bone": bail_id(e, s), "x": 0.075, "y": 0, "kind": "contact", "title": T(f"Sling hook in bail {e.upper()} {'left' if s == 'l' else 'right'}", f"Kranhaken im Bügel {e.upper()} {'links' if s == 'l' else 'rechts'}"), "support": "pin", "normalDeg": 90})
    out.append({"id": "bearing-a", "bone": "slab", "x": 0.05, "y": -0.10, "kind": "contact", "title": T("Reinstalled bearing A on the wall crown, pin", "Wiedereingebautes Auflager A auf der Wandkrone, Festlager"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 1.196})
    out.append({"id": "bearing-b", "bone": "slab", "x": 5.95, "y": -0.10, "kind": "contact", "title": T("Reinstalled bearing B on the wall crown, roller", "Wiedereingebautes Auflager B auf der Wandkrone, Rollenlager"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.10, "footprintWidthM": 1.196})
    out.append({"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0.10, "kind": "load", "title": T("Concentrated load on the top face", "Einzellast auf der Oberseite")})
    return out


def sh(id_, bone_, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None, solid=None):
    s = {"id": id_, "bone": bone_, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    if hatch:
        s["hatch"] = hatch
    if pen is not None:
        s["penMm"] = pen
    s.update(geom)
    if solid:
        s["solid"] = solid
    return s


def rect(x, y, w, h):
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def poly(points, closed=True):
    return {"kind": "poly", "points": pts(points), "closed": closed}


def path(rings):
    return {"kind": "path", "d": path_d(rings)}


SLAB_REGIONS = ["keyed-edges", "top-face", "soffit"]
ALL_SLAB = ["top-face", "soffit", "keyed-edges", "sawn-ends", "cores", "lifting-steel"]


def bail_shapes(scale, view, thick):
    out = []
    for e, s, _ in BAILS:
        b = bail_id(e, s)
        z = bail_z(s)
        g = rect(0, -thick / 2, BAIL_LEN, thick)
        if view == "elevation":
            out.append(sh(f"{b}-side", b, "body", 6, True, 0.25, g, region="lifting-steel", material="lifting-steel"))
        else:
            out.append(sh(f"{b}-{view}", b, "body", 6, True, 0.25, g, region="lifting-steel", material="lifting-steel", solid={"kind": "extruded", "sweep": "w", "z0": r6(z - 0.035), "z1": r6(z + 0.035)}))
    return out


def bearer_elev(scale):
    out = []
    for n in "ab":
        b = f"bearer-{n}"
        out.append(sh(f"{b}-end", b, "body", 2, True, 0.38, rect(0, -0.05, 0.10, 0.10), region="bearers", material="softwood"))
        if scale == "50":
            ring1 = [(0.065 + 0.022 * math.sin(2 * math.pi * k / 16), 0.015 + 0.022 * math.cos(2 * math.pi * k / 16)) for k in range(17)]
            ring2 = [p for p in [(0.065 + 0.040 * math.sin(2 * math.pi * k / 24), 0.015 + 0.040 * math.cos(2 * math.pi * k / 24)) for k in range(6, 19)] if 0.004 <= p[0] <= 0.096 and -0.046 <= p[1] <= 0.046]
            out.append(sh(f"{b}-ring-1", b, "texture", 2.5, False, 0.2, poly(ring1, False), region="bearers"))
            out.append(sh(f"{b}-ring-2", b, "texture", 2.5, False, 0.2, poly(ring2, False), region="bearers"))
            out.append(sh(f"{b}-check", b, "detail", 2.6, False, 0.2, poly([(0.065, 0.015), (0.012, -0.035)], False), region="bearers"))
    return out


def bearer_front(scale):
    out = []
    for k, n in enumerate("ab"):
        b = f"bearer-{n}"
        out.append(sh(f"{b}-side", b, "body", 2 - 0.1 * k, True, 0.38, rect(-0.70, 0, 1.40, 0.10), region="bearers", material="softwood"))
    if scale in ("50", "100"):
        out.append(sh("bearer-a-grain", "bearer-a", "texture", 2.5, False, 0.2, poly([(-0.69, 0.035), (-0.20, 0.032), (0.30, 0.037), (0.69, 0.034)], False), region="bearers"))
        out.append(sh("bearer-a-grain-2", "bearer-a", "texture", 2.5, False, 0.2, poly([(-0.69, 0.068), (-0.35, 0.071), (0.10, 0.066), (0.69, 0.069)], False), region="bearers"))
    return out


def bearer_plan():
    return [sh(f"bearer-{n}-plan", f"bearer-{n}", "body", 0.5, True, 0.38, rect(x - 0.05, -0.70, 0.10, 1.40), region="bearers", material="softwood") for n, x in zip("ab", BEARER_X)]


def lift_elev(scale):
    out = []
    for e, x in zip("ab", LIFT_X):
        if scale == "50":
            out.append(sh(f"washer-top-{e}", "slab", "body", 4, True, 0.25, rect(x - 0.05, 0.10, 0.10, 0.01), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"base-{e}", "slab", "body", 5, True, 0.22, rect(x - 0.035, 0.11, 0.07, 0.04), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"washer-bottom-{e}", "slab", "body", 4, True, 0.25, rect(x - 0.05, -0.11, 0.10, 0.01), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"nut-{e}", "slab", "body", 4, True, 0.22, rect(x - 0.018, -0.129, 0.036, 0.019), region="lifting-steel", material="lifting-steel"))
        elif scale == "100":
            x0 = x - 0.05 if e == "a" else x - 0.09
            out.append(sh(f"lifting-block-{e}", "slab", "body", 4, True, 0.25, rect(x0, 0.10, 0.14, 0.05), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"washer-bottom-{e}", "slab", "body", 4, True, 0.25, rect(x - 0.05, -0.115, 0.10, 0.015), region="lifting-steel", material="lifting-steel"))
        elif scale == "200":
            x0 = x - 0.05 if e == "a" else x - 0.09
            out.append(sh(f"lifting-block-{e}", "slab", "body", 4, True, 0.25, rect(x0, 0.10, 0.14, 0.045), region="lifting-steel", material="lifting-steel"))
        else:
            x0 = x - 0.05 if e == "a" else x - 0.09
            out.append(sh(f"lifting-block-{e}", "slab", "body", 4, True, 0.25, rect(x0, 0.10, 0.14, 0.075), region="lifting-steel", material="lifting-steel"))
    return out


def lift_front(scale):
    out = []
    for s in "lr":
        z = bail_z(s)
        if scale == "50":
            out.append(sh(f"washer-top-{s}", "slab", "body", 4, True, 0.25, rect(z - 0.05, 0.30, 0.10, 0.01), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"base-{s}", "slab", "body", 5, True, 0.22, rect(z - 0.035, 0.31, 0.07, 0.04), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"bolt-{s}", "slab", "body", 3, True, 0.25, rect(z - 0.012, 0.131, 0.024, 0.144), region="lifting-steel", material="lifting-steel"))
        elif scale == "100":
            out.append(sh(f"lifting-block-{s}", "slab", "body", 4, True, 0.25, rect(z - 0.05, 0.30, 0.10, 0.05), region="lifting-steel", material="lifting-steel"))
            out.append(sh(f"bolt-{s}", "slab", "body", 3, True, 0.25, rect(z - 0.012, 0.131, 0.024, 0.144), region="lifting-steel", material="lifting-steel"))
        elif scale == "200":
            out.append(sh(f"lifting-block-{s}", "slab", "body", 4, True, 0.25, rect(z - 0.05, 0.30, 0.10, 0.045), region="lifting-steel", material="lifting-steel"))
        else:
            out.append(sh(f"lifting-block-{s}", "slab", "body", 4, True, 0.25, rect(z - 0.05, 0.30, 0.10, 0.075), region="lifting-steel", material="lifting-steel"))
    return out


def lift_plan(scale):
    out = []
    for e, x in zip("ab", LIFT_X):
        for s in "lr":
            z = bail_z(s)
            if scale == "50":
                out.append(sh(f"washer-top-{e}-{s}", "slab", "body", 4, True, 0.25, rect(x - 0.05, z - 0.05, 0.10, 0.10), region="lifting-steel", material="lifting-steel"))
                out.append(sh(f"base-{e}-{s}", "slab", "body", 5, True, 0.22, {"kind": "ellipse", "cx": x, "cy": z, "rx": 0.035, "ry": 0.035}, region="lifting-steel", material="lifting-steel"))
            else:
                x0 = x - 0.05 if e == "a" else x - 0.09
                out.append(sh(f"lifting-block-{e}-{s}", "slab", "body", 4, True, 0.25, rect(x0, z - 0.05, 0.14, 0.10), region="lifting-steel", material="lifting-steel"))
    return out


def grout_elev(scale):
    out = []
    for k, (a, b) in enumerate(PATCH_R):
        if scale == "50":
            top = jag(a, b, 0.064, 0.006, k + 1)
            bot = jag(a, b, -0.034, 0.005, k + 7)
            ring = top + list(reversed(bot))
        else:
            ring = [(a, -0.036), (b, -0.036), (b, 0.066), (a, 0.066)]
        out.append(sh(f"grout-r-{k + 1}", "slab", "detail", 1.4, True, 0.78, poly(ring), region="keyed-edges", material="grout"))
    return out


def grout_front(scale):
    g = GROUT[scale]
    return [
        sh("grout-r-1-section", "slab", "body", 1.4, True, 0.78, poly(front_of([(v, -w) for v, w in g])), region="keyed-edges", material="grout"),
        sh("grout-l-1-beyond", "slab", "body", 0.9, True, 0.78, poly(front_of(g)), region="keyed-edges", material="grout"),
    ]


def grout_plan(scale):
    out = []
    for side, patches, sign in (("l", PATCH_L, 1), ("r", PATCH_R, -1)):
        for k, (a, b) in enumerate(patches):
            if scale == "50":
                outer = jag(a, b, 0.5985, 0.0025, k + 3 * sign + 5, 0.15)
                ring = [(a, 0.5863), (b, 0.5863)] + list(reversed(outer))
            else:
                ring = [(a, 0.5846), (b, 0.5846), (b, 0.602), (a, 0.602)]
            out.append(sh(f"grout-{side}-{k + 1}-plan", "slab", "detail", 1.4, True, 0.78, poly([(x, sign * z) for x, z in ring]), region="keyed-edges", material="grout"))
    return out


def end_face_rings(scale):
    outer = [(r6(w), r6(v + Y0)) for v, w in section(scale)]
    if scale == "500":
        return [outer]
    return [outer] + [[(r6(w), r6(v + Y0)) for v, w in reversed(c)] for c in cores(32)]


def representation(scale):
    pens = {
        "50": {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5},
        "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
        "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25},
        "500": {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18},
    }[scale]
    rng = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}[scale]
    hatches = []
    if scale in ("50", "100"):
        hatches.append({"id": "sawn-concrete", "title": T("Saw-cut concrete", "Gesägter Beton"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.6 if scale == "50" else 0.5, "penMm": 0.1})
    el = []
    fr = []
    pl = []
    side_regions = SLAB_REGIONS if scale != "500" else ALL_SLAB
    el.append(sh("slab-side", "slab", "body", 1, True, 0.62, rect(0, -0.10, L, 0.20), regions=side_regions, material="concrete"))
    if scale == "50":
        for k, v in enumerate([-0.062, -0.012, 0.047]):
            el.append(sh(f"key-line-{k + 1}", "slab", "texture", 1.2, False, 0.3, poly([(0, v), (L, v)], False), region="keyed-edges"))
        el.append(sh("reuse-mark", "slab", "detail", 1.5, False, 0.15, poly([(2.85, -0.045), (3.30, -0.045), (3.30, 0.030), (2.85, 0.030)]), region="keyed-edges"))
        el.append(sh("reuse-mark-stroke", "slab", "texture", 1.6, False, 0.15, poly([(2.90, -0.01), (3.00, 0.02), (3.10, -0.03), (3.25, 0.01)], False), region="keyed-edges"))
    elif scale == "100":
        el.append(sh("key-line-1", "slab", "texture", 1.2, False, 0.3, poly([(0, -0.048), (L, -0.048)], False), region="keyed-edges"))
    if scale != "500":
        for n, x in (("a", 0.0), ("b", L)):
            el.append(sh(f"saw-cut-{n}", "slab", "detail", 1.3, False, 0.3, poly([(x, -0.10), (x, 0.10)], False), region="sawn-ends"))
            el.append(sh(f"core-mouths-{n}", "slab", "texture" if scale != "200" else "detail", 1.35, False, 0.2, poly([(x, r6(CORE_V - CORE_R)), (x, r6(CORE_V + CORE_R))], False), region="cores"))
    if scale in ("50", "100"):
        el += grout_elev(scale)
    el += lift_elev(scale)
    if scale != "500":
        el += bail_shapes(scale, "elevation", 0.02 if scale != "200" else 0.03)
    el += bearer_elev(scale)

    end_regions = ["sawn-ends", "cores"] if scale != "500" else ALL_SLAB
    fr.append(sh("slab-end", "slab", "body", 1, True, 0.62, path(end_face_rings(scale)), regions=end_regions, material="sawn-concrete", hatch="sawn-concrete" if hatches else None))
    if scale == "50":
        for k, w in enumerate(STRAND_W):
            fr.append(sh(f"strand-end-{k + 1}", "slab", "detail", 1.6, True, 0.12, {"kind": "ellipse", "cx": w, "cy": r6(STRAND_V + Y0), "rx": STRAND_R, "ry": STRAND_R}, region="sawn-ends", material="strand"))
    if scale in ("50", "100"):
        fr += grout_front(scale)
    fr += lift_front(scale)
    if scale != "500":
        fr += bail_shapes(scale, "front", 0.02 if scale != "200" else 0.03)
    fr += bearer_front(scale)

    top_regions = ["top-face", "keyed-edges"] if scale != "500" else ALL_SLAB
    wmax = EDGE[scale][0][1]
    pl.append(sh("slab-top", "slab", "body", 1, True, 0.62, rect(0, -wmax, L, 2 * wmax), regions=top_regions, material="concrete"))
    if scale in ("50", "100"):
        wt = EDGE[scale][-1][1]
        pl.append(sh("top-arris-l", "slab", "detail", 1.2, False, 0.3, poly([(0, wt), (L, wt)], False), region="top-face"))
        pl.append(sh("top-arris-r", "slab", "detail", 1.2, False, 0.3, poly([(0, -wt), (L, -wt)], False), region="top-face"))
        pl += grout_plan(scale)
    if scale == "50":
        blobs = [[(1.20, 0.10), (1.55, 0.16), (1.70, 0.05), (1.45, -0.06), (1.22, -0.02)], [(3.80, -0.30), (4.25, -0.24), (4.40, -0.38), (4.00, -0.45)], [(2.10, 0.32), (2.30, 0.40), (2.45, 0.30), (2.25, 0.24)]]
        for k, b in enumerate(blobs):
            pl.append(sh(f"screed-remnant-{k + 1}", "slab", "texture", 1.3, False, 0.4, poly(b), region="top-face"))
        pl.append(sh("reuse-mark-top", "slab", "detail", 1.5, False, 0.15, poly([(5.62, -0.20), (5.86, -0.20), (5.86, 0.20), (5.62, 0.20)]), region="top-face"))
    pl += lift_plan(scale)
    if scale != "500":
        pl += bail_shapes(scale, "plan", 0.02 if scale != "200" else 0.03)
    pl += bearer_plan()
    for s in el + fr + pl:
        if s.get("hatch") is None:
            s.pop("hatch", None)
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches,
            "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": REP_DECISIONS[scale]}


REP_DECISIONS = {
    "50": {
        "silhouette": T("Side elevation of the 6.00 m plank, 200 mm deep (120 x 4 mm on paper), on two 100 x 100 mm timber bearers 0.45 m in from the saw-cut ends, with the four lifting points (washer, swivel base, folded bail, nut under the soffit) at 0.60 m. Front view: the saw-cut end face with its six open cores of 140 mm, the five cut strands, the keyed edges with the grout remnant at the near edge, the through-bolts visible in the two outer cores and the bearer below. Plan: the top face with the top arrises, the grout remnants in the joint profile, the lifting washers and bails and the bearer ends showing beyond the plank.",
                        "Seitenansicht der 6,00 m langen und 200 mm hohen Platte (120 x 4 mm auf Papier) auf zwei Kanthölzern 100 x 100 mm, 0,45 m von den Sägeschnitten, mit den vier Anschlagpunkten (Scheibe, Drehsockel, umgeklappter Bügel, Mutter unter der Untersicht) bei 0,60 m. Vorderansicht: die gesägte Stirnfläche mit sechs offenen Hohlräumen von 140 mm, den fünf geschnittenen Litzen, den profilierten Fugenflanken mit dem Vergussrest an der vorderen Kante, den in den beiden äußeren Hohlräumen sichtbaren Durchsteckbolzen und dem Kantholz darunter. Draufsicht: die Oberseite mit den oberen Kanten, den Vergussresten im Fugenprofil, den Anschlagscheiben und Bügeln und den über die Platte hinausragenden Kantholzenden."),
        "retained": T("The true keyed edge profile (30 mm bottom land, 10 mm bevel, 10 mm grout key, 15 mm narrower top), all six cores at 190 mm pitch, the 12.5 mm strand ends at 40 mm cover, the grout remnants at their places along both edges, the 10 mm washers and the bolts of 24 mm.",
                      "Das wahre Fugenprofil (30 mm Fuß, 10 mm Schräge, 10 mm Vergussnut, Oberseite 15 mm schmaler), alle sechs Hohlräume im Abstand von 190 mm, die Litzenenden von 12,5 mm mit 40 mm Überdeckung, die Vergussreste an ihren Stellen längs beider Kanten, die Scheiben von 10 mm und die Bolzen von 24 mm."),
        "merged": T("Side face, top and soffit are one bar shape; the key flanks are drawn as three texture lines along the side face.", "Seitenfläche, Oberseite und Untersicht sind eine Balkenform; die Fugenflanken sind als drei Texturlinien längs der Seitenfläche gezeichnet."),
        "omitted": T("The far-side grout remnants in side elevation (behind the plank), the bolts inside the plank in side view and plan, and spalls under 10 mm.", "Die Vergussreste der abgewandten Kante in der Seitenansicht (hinter der Platte), die Bolzen im Inneren der Platte in Seitenansicht und Draufsicht und Abplatzungen unter 10 mm."),
        "thinElements": T("Strand ends are filled circles of their true 12.5 mm (0.25 mm); the bails are 20 mm bars (0.4 mm) seen edge-on in side and front view.", "Litzenenden sind gefüllte Kreise in ihren wahren 12,5 mm (0,25 mm); die Bügel sind Stäbe von 20 mm (0,4 mm), in Seiten- und Vorderansicht von der Kante gesehen."),
        "hatching": T("The saw-cut end face carries a stipple of 0.6 mm as the sign of a cut concrete surface; the faces seen in view are tones.", "Die gesägte Stirnfläche trägt eine Punktierung von 0,6 mm als Zeichen einer geschnittenen Betonfläche; die Ansichtsflächen sind Töne."),
        "styles": T("Outline 0.35 mm, steel and grout detail 0.18 mm, key lines, end grain and screed remnants 0.1 mm; grout light, concrete middle, steel and timber dark.", "Kontur 0,35 mm, Stahl- und Vergussdetails 0,18 mm, Fugenlinien, Hirnholz und Estrichreste 0,1 mm; Verguss hell, Beton mittel, Stahl und Holz dunkel."),
        "overlays": T("The plank is one cell, each bearer, lifting part and grout remnant its own cell above it.", "Die Platte ist eine Zelle, jedes Kantholz, Anschlagteil und jeder Vergussrest eine eigene Zelle darüber."),
    },
    "100": {
        "silhouette": T("The plank (60 x 2 mm) on its two bearers with the four lifting points as blocks of washer and swivel base under the folded bails. Front view: the end face with the six cores, the grout remnant at the near edge and the bolts in the outer cores. Plan: the top with the arrises, straight grout strips and the lifting blocks.", "Die Platte (60 x 2 mm) auf ihren zwei Kanthölzern mit den vier Anschlagpunkten als Blöcke aus Scheibe und Drehsockel unter den umgeklappten Bügeln. Vorderansicht: die Stirnfläche mit den sechs Hohlräumen, dem Vergussrest an der vorderen Kante und den Bolzen in den äußeren Hohlräumen. Draufsicht: die Oberseite mit den Kanten, geraden Vergussstreifen und den Anschlagblöcken."),
        "retained": T("Six cores, the edge bevel, the grout remnants at their places, bails of 20 mm, bolts of 24 mm, the 15 mm washer under the soffit.", "Sechs Hohlräume, die Kantenschräge, die Vergussreste an ihren Stellen, Bügel von 20 mm, Bolzen von 24 mm, die 15-mm-Scheibe unter der Untersicht."),
        "merged": T("Top washer, swivel base and the space under the folded bail are one 140 x 50 mm block; nut and bottom washer one 15 mm plate.", "Obere Scheibe, Drehsockel und der Raum unter dem umgeklappten Bügel sind ein Block 140 x 50 mm; Mutter und untere Scheibe eine Platte von 15 mm."),
        "omitted": T("The 10 mm grout key, the strand ends (0.13 mm), end grain, screed remnants and paint marks.", "Die Vergussnut von 10 mm, die Litzenenden (0,13 mm), Hirnholz, Estrichreste und Farbmarkierungen."),
        "thinElements": T("Bails and bolts keep their true 20 and 24 mm (0.2 and 0.24 mm).", "Bügel und Bolzen behalten ihre wahren 20 und 24 mm (0,2 und 0,24 mm)."),
        "hatching": T("Stipple of 0.5 mm on the saw-cut end face.", "Punktierung von 0,5 mm auf der gesägten Stirnfläche."),
        "styles": T("Outline 0.25 mm, details 0.13 mm, one key line 0.1 mm.", "Kontur 0,25 mm, Details 0,13 mm, eine Fugenlinie 0,1 mm."),
        "overlays": T("Plank one cell, bearers, lifting blocks and grout strips their own cells.", "Platte eine Zelle, Kanthölzer, Anschlagblöcke und Vergussstreifen eigene Zellen."),
    },
    "200": {
        "silhouette": T("The plank (30 x 1 mm) on two bearers with four lifting blocks and bails. Front view: the tapered end face (1196 mm at the soffit, 1166 mm at the top) with the six cores (0.7 mm, webs 0.25 mm). Plan: the top with the lifting blocks and the bearer ends.", "Die Platte (30 x 1 mm) auf zwei Kanthölzern mit vier Anschlagblöcken und Bügeln. Vorderansicht: die verjüngte Stirnfläche (1196 mm unten, 1166 mm oben) mit den sechs Hohlräumen (0,7 mm, Stege 0,25 mm). Draufsicht: die Oberseite mit den Anschlagblöcken und den Kantholzenden."),
        "retained": T("Six cores, the taper of the joint flanks, the bearers and the lifting points as blocks of 140 x 45 mm with 30 mm bails.", "Sechs Hohlräume, die Verjüngung der Fugenflanken, die Kanthölzer und die Anschlagpunkte als Blöcke 140 x 45 mm mit Bügeln von 30 mm."),
        "merged": T("Bevel, key and top step of the edge are one straight taper; each lifting point is one block, its bail one 30 mm bar.", "Schräge, Nut und Absatz der Kante sind eine gerade Verjüngung; jeder Anschlagpunkt ist ein Block, sein Bügel ein Stab von 30 mm."),
        "omitted": T("Grout remnants (at most 23 mm thick, 0.12 mm), bolts, bottom washers, strand ends and all texture.", "Vergussreste (höchstens 23 mm dick, 0,12 mm), Bolzen, untere Scheiben, Litzenenden und alle Texturen."),
        "thinElements": T("None below the feature: bails are drawn at the 30 mm feature of the scale.", "Keine unter der Einzelheit: die Bügel sind mit der Einzelheit von 30 mm des Maßstabs gezeichnet."),
        "hatching": T("No hatch: the end face is a tone.", "Keine Schraffur: die Stirnfläche ist ein Ton."),
        "styles": T("Outline 0.18 mm.", "Kontur 0,18 mm."),
        "overlays": T("Plank, bearers, lifting blocks and bails each one cell.", "Platte, Kanthölzer, Anschlagblöcke und Bügel je eine Zelle."),
    },
    "500": {
        "silhouette": T("The plank as a bar of 12 x 0.4 mm on its two bearers (0.2 mm) with the lifting points as four blocks of 140 x 75 mm (0.28 x 0.15 mm) on the top. Front view: the end face as one strip of 2.4 x 0.4 mm over the bearer with the two lifting blocks. Plan: the top as a strip of 12 x 2.4 mm with the lifting blocks and the bearer ends.", "Die Platte als Balken 12 x 0,4 mm auf ihren zwei Kanthölzern (0,2 mm) mit den Anschlagpunkten als vier Blöcke 140 x 75 mm (0,28 x 0,15 mm) auf der Oberseite. Vorderansicht: die Stirnfläche als ein Streifen 2,4 x 0,4 mm über dem Kantholz mit den beiden Anschlagblöcken. Draufsicht: die Oberseite als Streifen 12 x 2,4 mm mit den Anschlagblöcken und den Kantholzenden."),
        "retained": T("Length, depth and width of the plank, the two bearers that tell the storage state and the overall height of 0.37 m to the folded bails.", "Länge, Höhe und Breite der Platte, die beiden Kanthölzer, die den Lagerzustand zeigen, und die Gesamthöhe von 0,37 m bis zu den umgeklappten Bügeln."),
        "merged": T("The cores (0.28 mm with 0.1 mm webs, not printable) are merged into the solid end face; every surface region of the plank is painted as one merged value; washer, swivel base and folded bail of each lifting point are one block on the plank.", "Die Hohlräume (0,28 mm mit Stegen von 0,1 mm, nicht druckbar) sind in die volle Stirnfläche aufgegangen; jede Oberflächenregion der Platte ist als ein zusammengefasster Wert gemalt; Scheibe, Drehsockel und umgeklappter Bügel jedes Anschlagpunkts sind ein Block auf der Platte."),
        "omitted": T("Edge profile, grout, bolts, the bails as moving parts (they stay folded in the block at this scale) and every detail.", "Kantenprofil, Verguss, Bolzen, die Bügel als bewegte Teile (sie bleiben in diesem Maßstab im Block umgeklappt) und jedes Detail."),
        "thinElements": T("None.", "Keine."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": T("Plank, bearers and lifting blocks one cell each.", "Platte, Kanthölzer und Anschlagblöcke je eine Zelle."),
    },
}


def solid_box(id_, bone_, material, tone, size, at, role="body", **kw):
    s = {"id": id_, "kind": "box", "bone": bone_, "material": material, "role": role, "tone": tone, "size": [r6(c) for c in size], "transform": {"translation": [r6(c) for c in at]}}
    s.update(kw)
    return s


def solid_extrude(id_, bone_, material, tone, plane, profile, frm, to, holes=None, role="body", **kw):
    s = {"id": id_, "kind": "extrude", "bone": bone_, "material": material, "role": role, "tone": tone, "profile": pts(profile), "plane": plane, "from": r6(frm), "to": r6(to)}
    if holes:
        s["holes"] = [pts(h) for h in holes]
    s.update(kw)
    return s


def solid_cyl(id_, bone_, material, tone, axis, origin, r, length, **kw):
    s = {"id": id_, "kind": "revolve", "bone": bone_, "material": material, "role": "body", "tone": tone, "profile": [[0, 0], [0, r6(r)], [r6(length), r6(r)], [r6(length), 0]], "axis": axis, "origin": [r6(c) for c in origin]}
    s.update(kw)
    return s


def bail_profile(scale):
    if scale == "50":
        outer = [(-0.035, 0.0), (0.035, 0.0), (0.035, 0.05)] + [(r6(0.035 * math.cos(math.pi * k / 8)), r6(0.05 + 0.035 * math.sin(math.pi * k / 8))) for k in range(1, 8)] + [(-0.035, 0.05)]
        hole = [(-0.015, 0.02), (0.015, 0.02), (0.015, 0.05)] + [(r6(0.015 * math.cos(math.pi * k / 6)), r6(0.05 + 0.015 * math.sin(math.pi * k / 6))) for k in range(1, 6)] + [(-0.015, 0.05)]
        return outer, hole, 0.010
    if scale == "100":
        return [(-0.035, 0.0), (0.035, 0.0), (0.035, 0.085), (-0.035, 0.085)], [(-0.015, 0.02), (0.015, 0.02), (0.015, 0.065), (-0.015, 0.065)], 0.010
    return None, None, 0.015


def body(scale):
    sec = section(scale)
    holes = cores(32) if scale != "500" else None
    out = [solid_extrude("slab", "slab", "concrete", 0.62, "yz", sec, 0, L, holes=holes)]
    if scale == "50":
        for n, u0 in (("a", -0.0015), ("b", L - 0.0025)):
            out.append({"id": f"strand-ends-{n}", "kind": "revolve", "bone": "slab", "material": "strand", "role": "detail", "tone": 0.12, "texture": True,
                        "profile": [[0, 0], [0, STRAND_R], [0.004, STRAND_R], [0.004, 0]], "axis": [1, 0, 0], "origin": [r6(u0), STRAND_V, STRAND_W[0]], "repeat": {"count": 5, "translation": [0, 0, 0.19]}})
    if scale in ("50", "100"):
        g = GROUT[scale]
        for k, (a, b) in enumerate(PATCH_L):
            out.append(solid_extrude(f"grout-l-{k + 1}", "slab", "grout", 0.78, "yz", g, a, b, role="detail"))
        for k, (a, b) in enumerate(PATCH_R):
            out.append(solid_extrude(f"grout-r-{k + 1}", "slab", "grout", 0.78, "yz", [(v, -w) for v, w in reversed(g)], a, b, role="detail"))
    for e, x in zip("ab", LIFT_X):
        rep = {"count": 2, "translation": [0, 0, -0.95]}
        z = LIFT_Z[0]
        if scale == "50":
            out.append(solid_box(f"washer-top-{e}", "slab", "lifting-steel", 0.25, [0.1, 0.01, 0.1], [x, 0.105, z], repeat=rep))
            out.append(solid_box(f"washer-bottom-{e}", "slab", "lifting-steel", 0.25, [0.1, 0.01, 0.1], [x, -0.105, z], repeat=rep))
            out.append(solid_box(f"nut-{e}", "slab", "lifting-steel", 0.22, [0.036, 0.019, 0.036], [x, -0.1195, z], repeat=rep))
            out.append(solid_cyl(f"base-{e}", "slab", "lifting-steel", 0.22, [0, 1, 0], [x, 0.11, z], 0.035, 0.04, repeat=rep))
            out.append(solid_cyl(f"bolt-{e}", "slab", "lifting-steel", 0.25, [0, 1, 0], [x, -0.11, z], 0.012, 0.22, repeat=rep))
        elif scale == "100":
            cx = x + 0.02 if e == "a" else x - 0.02
            out.append(solid_box(f"lifting-block-{e}", "slab", "lifting-steel", 0.25, [0.14, 0.05, 0.1], [cx, 0.125, z], repeat=rep))
            out.append(solid_box(f"washer-bottom-{e}", "slab", "lifting-steel", 0.25, [0.1, 0.015, 0.1], [x, -0.1075, z], repeat=rep))
            out.append(solid_cyl(f"bolt-{e}", "slab", "lifting-steel", 0.25, [0, 1, 0], [x, -0.10, z], 0.012, 0.20, repeat=rep))
        elif scale == "200":
            cx = x + 0.02 if e == "a" else x - 0.02
            out.append(solid_box(f"lifting-block-{e}", "slab", "lifting-steel", 0.25, [0.14, 0.045, 0.1], [cx, 0.1225, z], repeat=rep))
        else:
            cx = x + 0.02 if e == "a" else x - 0.02
            out.append(solid_box(f"lifting-block-{e}", "slab", "lifting-steel", 0.25, [0.14, 0.075, 0.1], [cx, 0.1375, z], repeat=rep))
    if scale != "500":
        outer, hole, half = bail_profile(scale)
        for e, s, _ in BAILS:
            b = bail_id(e, s)
            if outer:
                out.append(solid_extrude(b, b, "lifting-steel", 0.25, "zx", outer, -half, half, holes=[hole]))
            else:
                out.append(solid_box(b, b, "lifting-steel", 0.25, [BAIL_LEN, 2 * half, 0.07], [BAIL_LEN / 2, 0, 0]))
    for n in "ab":
        out.append(solid_box(f"bearer-{n}", f"bearer-{n}", "softwood", 0.38, [0.10, 0.10, 1.40], [0.05, 0, 0]))
    return out


def band(scale, side):
    e = EDGE[scale]
    t = EDGE_BAND[scale]
    outer = [(v, w) for v, w in e]
    inner = [(v, w - t) for v, w in reversed(e)]
    ring = outer + inner
    if side < 0:
        ring = [(v, -w) for v, w in reversed(ring)]
    return ring


def arc(wc, t0, t1, n=12):
    return [(r6(CORE_V + CORE_R * math.sin(math.radians(t0 + (t1 - t0) * k / n))), r6(wc + CORE_R * math.cos(math.radians(t0 + (t1 - t0) * k / n)))) for k in range(n + 1)]


def web(wc):
    t = math.degrees(math.asin((FLANGE_TOP - CORE_V) / CORE_R))
    b = math.degrees(math.asin((FLANGE_BOTTOM - CORE_V) / CORE_R))
    return arc(wc + 0.19, 180 - t, 180 - b) + arc(wc, 360 + b, 360 + t)


def edge_web(sign):
    t = math.degrees(math.asin((FLANGE_TOP - CORE_V) / CORE_R))
    b = math.degrees(math.asin((FLANGE_BOTTOM - CORE_V) / CORE_R))
    ring = [(FLANGE_TOP, 0.598), (FLANGE_BOTTOM, 0.598)] + arc(CORE_W[-1], 360 + b, 360 + t)
    return ring if sign > 0 else [(v, -w) for v, w in reversed(ring)]


def thermal(scale):
    u0, u1 = END_SLICE, L - END_SLICE
    ends = [
        solid_extrude("th-end-a", "slab", "sawn-concrete", 0.6, "yz", section(scale), 0, END_SLICE, holes=cores(32), region="sawn-ends", sheet=True, sectionFaces=["+u", "-v", "+v", "-w", "+w"]),
        solid_extrude("th-end-b", "slab", "sawn-concrete", 0.6, "yz", section(scale), L - END_SLICE, L, holes=cores(32), region="sawn-ends", sheet=True, sectionFaces=["-u", "-v", "+v", "-w", "+w"]),
    ]
    if scale == "500":
        out = [
            solid_extrude("th-top-flange", "slab", "concrete", 0.6, "yz", [(FLANGE_TOP, -0.598), (FLANGE_TOP, 0.598), (0.1, 0.598), (0.1, -0.598)], u0, u1, region="top-face", sheet=True, sectionFaces=["-u", "+u", "-v"]),
            solid_extrude("th-bottom-flange", "slab", "soffit-paint", 0.6, "yz", [(-0.1, -0.598), (-0.1, 0.598), (FLANGE_BOTTOM, 0.598), (FLANGE_BOTTOM, -0.598)], u0, u1, region="soffit", sheet=True, sectionFaces=["-u", "+u", "+v"]),
            solid_extrude("th-edge-web-l", "slab", "concrete", 0.6, "yz", edge_web(1), u0, u1, region="keyed-edges", sectionFaces=["-u", "+u", "-v", "+v"]),
            solid_extrude("th-edge-web-r", "slab", "concrete", 0.6, "yz", edge_web(-1), u0, u1, region="keyed-edges", sectionFaces=["-u", "+u", "-v", "+v"]),
        ] + ends + [
            solid_extrude("th-webs", "slab", "concrete", 0.6, "yz", web(CORE_W[0]), u0, u1, region="cores", sectionFaces=["-u", "+u", "-v", "+v"], repeat={"count": 5, "translation": [0, 0, 0.19]}),
        ]
    else:
        t = SKIN[scale]
        e = EDGE[scale]
        wtop = e[-1][1] - EDGE_BAND[scale]
        wbot = e[0][1] - EDGE_BAND[scale]
        thin = EDGE_BAND[scale] < FEATURE[scale]
        band_kw = {"sheet": True} if thin else {}
        out = [
            solid_extrude("th-top", "slab", "concrete", 0.6, "yz", [(0.1 - t, -wtop), (0.1 - t, wtop), (0.1, wtop), (0.1, -wtop)], u0, u1, region="top-face", sheet=True, sectionFaces=["-u", "+u", "-w", "+w"]),
            solid_extrude("th-soffit", "slab", "soffit-paint", 0.6, "yz", [(-0.1, -wbot), (-0.1, wbot), (-0.1 + t, wbot), (-0.1 + t, -wbot)], u0, u1, region="soffit", sheet=True, sectionFaces=["-u", "+u", "-w", "+w"]),
            solid_extrude("th-edge-l", "slab", "concrete", 0.6, "yz", band(scale, 1), u0, u1, region="keyed-edges", sectionFaces=["-u", "+u", "-v", "+v"], **band_kw),
            solid_extrude("th-edge-r", "slab", "concrete", 0.6, "yz", band(scale, -1), u0, u1, region="keyed-edges", sectionFaces=["-u", "+u", "-v", "+v"], **band_kw),
        ] + ends + [
            solid_extrude("th-cores", "slab", "concrete", 0.6, "yz", circle(CORE_V, CORE_W[0], CORE_R + TUBE[scale], 32), u0, u1, holes=[circle(CORE_V, CORE_W[0], CORE_R, 32)], region="cores", sectionFaces=["-u", "+u"], repeat={"count": 6, "translation": [0, 0, 0.19]}),
        ]
    if scale in ("50", "100"):
        for k in range(3):
            out.append({"id": f"th-grout-l-{k + 1}", "of": f"grout-l-{k + 1}", "region": "keyed-edges"})
            out.append({"id": f"th-grout-r-{k + 1}", "of": f"grout-r-{k + 1}", "region": "keyed-edges"})
    for n in "ab":
        out.append({"id": f"th-bearer-{n}", "of": f"bearer-{n}", "region": "bearers"})
    for e in "ab":
        if scale == "50":
            for p in ("washer-top", "washer-bottom", "nut", "base", "bolt"):
                out.append({"id": f"th-{p}-{e}", "of": f"{p}-{e}", "region": "lifting-steel"})
        elif scale == "100":
            for p in ("lifting-block", "washer-bottom", "bolt"):
                out.append({"id": f"th-{p}-{e}", "of": f"{p}-{e}", "region": "lifting-steel"})
        elif scale == "200":
            out.append({"id": f"th-lifting-block-{e}", "of": f"lifting-block-{e}", "region": "lifting-steel"})
        else:
            out.append({"id": f"th-lifting-block-{e}", "of": f"lifting-block-{e}", "region": "lifting-steel"})
    if scale != "500":
        for e, s, _ in BAILS:
            out.append({"id": f"th-{bail_id(e, s)}", "of": bail_id(e, s), "region": "lifting-steel"})
    return out


def member_section_solid(scale, id_):
    sec = section(scale if scale != "500" else "200")
    return solid_extrude(id_, "slab", "concrete", 0.6, "yz", sec, 0, L, holes=cores(24), member="slab", part="member")


def structure(scale):
    out = []
    if scale == "50":
        out.append({"id": "slab-member", "of": "slab", "member": "slab", "part": "member"})
    else:
        m = member_section_solid(scale, "slab-member")
        if scale == "500":
            m["members"] = [bail_id(e, s) for e, s, _ in BAILS]
        out.append(m)
    for n in "ab":
        out.append({"id": f"bearer-{n}-member", "of": f"bearer-{n}", "member": f"bearer-{n}", "part": "member"})
    if scale != "500":
        for e, s, _ in BAILS:
            b = bail_id(e, s)
            out.append({"id": f"{b}-member", "of": b, "member": b, "part": "member"})
            hgt = PIVOT_V - 0.10 - (0.01 if scale in ("50", "100") else 0.015)
            out.append(solid_box(f"{b}-pivot", "slab", "lifting-steel", 0.3, [0.07, hgt, 0.07], [bail_x(e), 0.10 + hgt / 2, bail_z(s)], member=b, part="joint"))
    th = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    out.append(solid_box("joint-load-carrier", "load-carrier", "lifting-steel", 0.3, [0.1, th, 0.16], [0, 0.10 + th / 2, 0], member="load-carrier", part="joint"))
    return out


VOL_DECISIONS = {
    "50": [
        T("Body: the plank as one extrusion of its true 1:50 section, the 1196 mm soffit, the 30 mm bottom land, the 10 mm bevel and 10 mm grout key of each joint flank and the 1166 mm top, with the six 140 mm cores running open through the full 6.00 m from saw cut to saw cut; the five cut strands are 1.5 mm discs on both end faces (texture); the grout remnants are six extrusions of their broken joint fill along the two flanks; each lifting point is a 10 mm washer on the top and under the soffit, a 24 mm through-bolt in the outer core, a nut, a 70 mm swivel base and a 20 mm D-shaped bail on its own hinged bone; the bearers are 100 x 100 x 1400 mm squared timbers on their own bones.",
          "Körper: die Platte als eine Extrusion ihres wahren Querschnitts 1:50, Untersicht 1196 mm, 30 mm Fuß, 10 mm Schräge und 10 mm Vergussnut je Fugenflanke und Oberseite 1166 mm, mit den sechs Hohlräumen von 140 mm, die offen über die vollen 6,00 m von Sägeschnitt zu Sägeschnitt laufen; die fünf geschnittenen Litzen sind Scheiben von 1,5 mm an beiden Stirnflächen (Textur); die Vergussreste sind sechs Extrusionen ihrer abgebrochenen Fugenfüllung längs der beiden Flanken; jeder Anschlagpunkt ist eine 10-mm-Scheibe oben und unter der Untersicht, ein 24-mm-Durchsteckbolzen im äußeren Hohlraum, eine Mutter, ein Drehsockel von 70 mm und ein D-förmiger Bügel von 20 mm auf seinem eigenen Gelenkknochen; die Auflagerhölzer sind Kanthölzer 100 x 100 x 1400 mm auf eigenen Knochen."),
        T("Thermal: the concrete faces are 10 mm skins of their regions (top, painted soffit, both keyed flanks, both 8 mm saw-cut end slices and six 10 mm tubes around the open cores) that close the hollow webs off as a sealed cavity, so each region counts only its face to the air; skin faces against a neighbouring region are section faces; grout, bearers and lifting steel are referenced from the body.",
          "Thermisch: die Betonflächen sind 10-mm-Schalen ihrer Regionen (Oberseite, gestrichene Untersicht, beide profilierten Flanken, beide gesägten Stirnscheiben von 8 mm und sechs 10-mm-Rohre um die offenen Hohlräume), die das Innere der Stege als dichten Hohlraum abschließen, so dass jede Region nur ihre Fläche zur Luft zählt; Schalenflächen an einer Nachbarregion sind Schnittflächen; Verguss, Kanthölzer und Anschlagstahl sind aus dem Körper referenziert."),
        T("Structure: the plank itself is the member (its true section with the cores); the two bearers are short compression members from the soffit to the ground; each bail is a member carried by a 70 mm pivot joint on the top; the slider of the moving load is a 100 x 160 x 7.5 mm load plate on the top. The ground contacts under the bearers carry the stored plank, the four bail hooks the lifted one and the two wall bearings the reinstalled one.",
          "Tragwerk: die Platte selbst ist das Glied (ihr wahrer Querschnitt mit den Hohlräumen); die beiden Kanthölzer sind kurze Druckglieder von der Untersicht zum Boden; jeder Bügel ist ein Glied, getragen von einem Drehgelenk von 70 mm auf der Oberseite; der Schlitten der wandernden Last ist eine Lastplatte 100 x 160 x 7,5 mm auf der Oberseite. Die Bodenkontakte unter den Kanthölzern tragen die gelagerte Platte, die vier Bügelhaken die angehobene und die beiden Wandauflager die wieder eingebaute."),
    ],
    "100": [
        T("Body: the section simplified to bottom land, bevel and a straight flank to the 1166 mm top (the 10 mm key is below the 15 mm feature), the six cores kept; grout remnants as six straight strips of 17 mm; washer, base and the space under the folded bail are one 140 x 50 mm block per point, nut and bottom washer one 15 mm plate; the bolts in the outer cores and the bails as rectangular rings of 20 mm are kept; strand ends left out.",
          "Körper: der Querschnitt vereinfacht auf Fuß, Schräge und eine gerade Flanke zur 1166 mm breiten Oberseite (die 10-mm-Nut liegt unter der Einzelheit von 15 mm), die sechs Hohlräume bleiben; Vergussreste als sechs gerade Streifen von 17 mm; Scheibe, Sockel und der Raum unter dem umgeklappten Bügel sind je Punkt ein Block 140 x 50 mm, Mutter und untere Scheibe eine Platte von 15 mm; die Bolzen in den äußeren Hohlräumen und die Bügel als rechteckige Ringe von 20 mm bleiben; Litzenenden entfallen."),
        T("Thermal: the same sealed partition on the simplified section with 5 mm skins and 5 mm core tubes (declared sheets and hollow tubes, so the sealed cavity stays wider than the 7.5 mm probe of the scale) and 15 mm flank bands; grout strips, bearers and lifting steel referenced from the body.", "Thermisch: dieselbe dichte Aufteilung auf dem vereinfachten Querschnitt mit 5-mm-Schalen und 5-mm-Hohlraumrohren (erklärte Blätter und hohle Rohre, damit der dichte Hohlraum breiter bleibt als die Prüftiefe von 7,5 mm des Maßstabs) und 15-mm-Flankenbändern; Vergussstreifen, Kanthölzer und Anschlagstahl aus dem Körper referenziert."),
        T("Structure: the member is the simplified section with its cores, bearers and bails referenced, pivot joints of 50 mm, a 15 mm load plate.", "Tragwerk: das Glied ist der vereinfachte Querschnitt mit seinen Hohlräumen, Kanthölzer und Bügel referenziert, Drehgelenke von 50 mm, eine Lastplatte von 15 mm."),
    ],
    "200": [
        T("Body: the section as a straight taper from the 1196 mm soffit to the 1166 mm top with the six cores (140 mm with 50 mm webs, above the 30 mm feature); each lifting point one 140 x 45 mm block, each bail a 30 mm bar; grout, bolts, bottom washers and strand ends left out (all below 30 mm).",
          "Körper: der Querschnitt als gerade Verjüngung von 1196 mm Untersicht auf 1166 mm Oberseite mit den sechs Hohlräumen (140 mm mit 50-mm-Stegen, über der Einzelheit von 30 mm); jeder Anschlagpunkt ein Block 140 x 45 mm, jeder Bügel ein Stab von 30 mm; Verguss, Bolzen, untere Scheiben und Litzenenden entfallen (alle unter 30 mm)."),
        T("Thermal: the sealed partition on the tapered section with 4 mm skins and 4 mm core tubes (sheets and hollow tubes, the cavity stays wider than the 15 mm probe), 30 mm flank bands and the two end slices; the grout belongs to the flanks; bearers, lifting blocks and bails referenced.", "Thermisch: die dichte Aufteilung auf dem verjüngten Querschnitt mit 4-mm-Schalen und 4-mm-Hohlraumrohren (Blätter und hohle Rohre, der Hohlraum bleibt breiter als die Prüftiefe von 15 mm), 30-mm-Flankenbändern und den beiden Stirnscheiben; der Verguss gehört zu den Flanken; Kanthölzer, Anschlagblöcke und Bügel referenziert."),
        T("Structure: the tapered member section with its cores, bearers and bails referenced, pivot joints of 45 mm, a 30 mm load plate.", "Tragwerk: der verjüngte Gliedquerschnitt mit seinen Hohlräumen, Kanthölzer und Bügel referenziert, Drehgelenke von 45 mm, eine Lastplatte von 30 mm."),
    ],
    "500": [
        T("Body: the plank as one solid block of 6000 x 200 x 1196 mm, as the 1:500 drawing (the 0.28 mm cores with 0.1 mm webs cannot print, so the end face is solid); the two bearers of 100 mm stay as the storage state; each lifting point with its folded bail is one 140 x 75 x 100 mm block at the feature of the scale.",
          "Körper: die Platte als ein voller Block 6000 x 200 x 1196 mm wie die Zeichnung 1:500 (die Hohlräume von 0,28 mm mit Stegen von 0,1 mm drucken nicht, daher ist die Stirnfläche voll); die beiden Kanthölzer von 100 mm bleiben als Lagerzustand; jeder Anschlagpunkt mit seinem umgeklappten Bügel ist ein Block 140 x 75 x 100 mm in der Einzelheit des Maßstabs."),
        T("Thermal: the open cores stay in the thermal model (their 15.8 m2 are almost half of the surface the plank exchanges with the air), but a sealed cavity cannot stay wider than the 37.5 mm probe of this scale inside a 200 mm plank, so the section is cut into its real concrete parts: the top flange (32 mm, top face), the bottom flange (38 mm, painted soffit), the two edge webs (flanks and the outer core walls), the five inner webs (the core walls) and the two end slices; faces against a neighbour part are section faces, which also drops the core-wall arcs that face up or down, so the plank shows about three quarters of its true area (only the total is compared at this scale); the lifting blocks are referenced.",
          "Thermisch: die offenen Hohlräume bleiben im thermischen Modell (ihre 15,8 m2 sind fast die Hälfte der Fläche, die die Platte mit der Luft tauscht), doch ein dichter Hohlraum kann in einer 200 mm hohen Platte nicht breiter bleiben als die Prüftiefe von 37,5 mm dieses Maßstabs; daher ist der Querschnitt in seine wirklichen Betonteile zerlegt: der Obergurt (32 mm, Oberseite), der Untergurt (38 mm, gestrichene Untersicht), die beiden Randstege (Flanken und äußere Hohlraumwände), die fünf inneren Stege (die Hohlraumwände) und die beiden Stirnscheiben; Flächen an einem Nachbarteil sind Schnittflächen, was auch die nach oben und unten weisenden Bögen der Hohlraumwände entfernt, so dass die Platte etwa drei Viertel ihrer wahren Fläche zeigt (in diesem Maßstab wird nur die Summe verglichen); die Anschlagblöcke sind referenziert."),
        T("Structure: the true member section with its cores (140 mm above the feature), which also stands for the four bails (members, within 75 mm of its top); the bearers keep their 100 mm members; a 75 mm load plate.", "Tragwerk: der wahre Gliedquerschnitt mit seinen Hohlräumen (140 mm über der Einzelheit), der auch für die vier Bügel steht (members, innerhalb 75 mm seiner Oberseite); die Kanthölzer behalten ihre 100-mm-Glieder; eine Lastplatte von 75 mm."),
    ],
}


def volumes():
    return {s: {"status": "authored", "denominator": int(s), "body": body(s), "thermal": thermal(s), "structure": structure(s), "decisions": VOL_DECISIONS[s]} for s in ("50", "100", "200", "500")}


def region(id_, title, material, area, lc, geometry, svf, hc, ref, source="authored-estimate"):
    return {"id": id_, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": area, "characteristicLengthM": lc, "geometry": geometry, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": hc, "source": source, "reference": ref}


EDGE_LEN = sum(math.dist(EDGE["50"][i], EDGE["50"][i + 1]) for i in range(len(EDGE["50"]) - 1))
AREA = {
    "top-face": round(L * 2 * 0.583, 3),
    "soffit": round(L * 2 * 0.598, 3),
    "keyed-edges": round(2 * L * EDGE_LEN, 3),
    "sawn-ends": round(2 * AREA_NET, 3),
    "cores": round(6 * 2 * math.pi * CORE_R * L, 3),
    "bearers": round(2 * (3 * 0.1 * 1.4 + 2 * 0.01), 3),
    "lifting-steel": 0.0,
}
STEEL_AREA = 4 * (2 * (2 * 0.01 + 4 * 0.001) + (2 * math.pi * 0.035 * 0.04 + 2 * math.pi * 0.035 ** 2) + (2 * math.pi * 0.012 * 0.22) + (2 * 0.036 * 0.036 + 4 * 0.036 * 0.019) + (2 * BAIL_A + 0.02 * (0.07 + 0.1 + math.pi * 0.035 + 0.03 + 0.06 + math.pi * 0.015)))
STEEL_BEDDED = 4 * (2 * 0.01 + math.pi * 0.035 ** 2 + 0.036 * 0.036 + 2 * math.pi * 0.012 * 0.08)
AREA["lifting-steel"] = round(STEEL_AREA - STEEL_BEDDED, 3)


def thermal_model():
    regions = [
        region("top-face", T("Top face (old screed bond face)", "Oberseite (alte Estrich-Haftfläche)"), "concrete", AREA["top-face"], 0.5, "horizontal-plate-up", 0.95, 67500, "en-iso-13786"),
        region("soffit", T("Painted soffit (former ceiling)", "Gestrichene Untersicht (frühere Decke)"), "soffit-paint", AREA["soffit"], 0.5, "horizontal-plate-down", 0.05, 82500, "en-iso-13786"),
        region("keyed-edges", T("Keyed joint flanks with grout remnants", "Profilierte Fugenflanken mit Vergussresten"), "concrete", AREA["keyed-edges"], 0.2, "vertical-plate", 0.5, 100000, "en-iso-13786"),
        region("sawn-ends", T("Saw-cut end faces", "Gesägte Stirnflächen"), "sawn-concrete", AREA["sawn-ends"], 0.2, "vertical-plate", 0.5, 190000, "en-iso-13786"),
        region("cores", T("Open core walls", "Wände der offenen Hohlräume"), "concrete", AREA["cores"], 0.14, "cylinder", 0.0, 62500, "en-iso-13786"),
        region("bearers", T("Timber bearers", "Auflagerhölzer"), "softwood", AREA["bearers"], 0.1, "cylinder", 0.4, 25900, "en-iso-10456"),
        dict(region("lifting-steel", T("Lifting points (washers, bolts, swivel bails)", "Anschlagpunkte (Scheiben, Bolzen, Klappbügel)"), "lifting-steel", AREA["lifting-steel"], 0.07, "vertical-plate", 0.8, 36000, "din-582"), ratedPowerW=0, ratedPowerBasis="passive"),
    ]
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
        "operating": [],
        "regions": regions,
        "emitters": [],
        "assumptions": [
            T("A dismantled plank in an open yard on a mild sunny day; every value is an authored estimate. The plank is an inert passive element without power (0 W).", "Eine rückgebaute Platte auf einem offenen Lagerplatz an einem milden sonnigen Tag; alle Werte sind Schätzungen. Die Platte ist ein träges passives Bauteil ohne Leistung (0 W)."),
            T("Heat capacities per area are the concrete behind each face: the 27 mm top flange (67.5 kJ/m2K), the 33 mm bottom flange (82.5), the 40 mm edge webs (100), half the 24 h penetration depth of 151 mm behind the end faces (190) and half the 50 mm webs behind the core walls (62.5), for 2500 kg/m3 and 1000 J/kgK (EN ISO 13786).", "Die flächenbezogenen Wärmekapazitäten sind der Beton hinter jeder Fläche: der 27-mm-Obergurt (67,5 kJ/m2K), der 33-mm-Untergurt (82,5), die 40-mm-Randstege (100), die halbe 24-h-Eindringtiefe von 151 mm hinter den Stirnflächen (190) und die halben 50-mm-Stege hinter den Hohlraumwänden (62,5), für 2500 kg/m3 und 1000 J/kgK (EN ISO 13786)."),
            T("Surfaces differ by their use: the soffit keeps its white emulsion paint (solar absorptance 0.3), the top is the weathered bond face of the removed screed (0.65), the saw cuts are fresh and lighter (0.55), the galvanised lifting steel is low-emissive (0.3).", "Die Flächen unterscheiden sich durch ihre Nutzung: die Untersicht behält ihre weiße Dispersionsfarbe (Absorptionsgrad 0,3), die Oberseite ist die verwitterte Haftfläche des entfernten Estrichs (0,65), die Sägeschnitte sind frisch und heller (0,55), der verzinkte Anschlagstahl ist niedrig emittierend (0,3)."),
            T("The cores are open at both saw cuts, so their walls exchange heat with the core air and no sun reaches them; in the reinstalled pose the plank lies indoors without sun.", "Die Hohlräume sind an beiden Sägeschnitten offen, ihre Wände tauschen daher Wärme mit der Luft der Hohlräume, und keine Sonne erreicht sie; in der wieder eingebauten Pose liegt die Platte innen ohne Sonne."),
        ],
        "passengers": [],
    }


def binding(state):
    sun = {"stored": {"top-face": 1, "soffit": 0, "keyed-edges": 0.5, "sawn-ends": 0.5, "cores": 0, "bearers": 0.3, "lifting-steel": 0.8},
           "lifted": {"top-face": 1, "soffit": 0, "keyed-edges": 0.5, "sawn-ends": 0.5, "cores": 0, "bearers": 0.5, "lifting-steel": 0.8},
           "reinstalled": {"top-face": 0, "soffit": 0, "keyed-edges": 0, "sawn-ends": 0, "cores": 0, "bearers": 0.5, "lifting-steel": 0}}[state]
    out = {}
    for r in AREA:
        b = {"exposedFraction": 1, "sunlitFraction": sun[r]}
        if r == "bearers":
            b["contact"] = {"fraction": 0.27, "tempC": 18}
        if r == "soffit" and state == "stored":
            b["contact"] = {"fraction": 0.033, "tempC": 20}
        if r == "soffit" and state == "reinstalled":
            b["contact"] = {"fraction": 0.033, "tempC": 20}
        out[r] = b
    return {"regions": out, "emitters": {}}


def bails_up():
    return {bail_id(e, s): {"angleDeg": 90 if e == "a" else -90} for e, s, _ in BAILS}


def bearers_down(dy):
    return {f"bearer-{n}": {"dy": -dy} for n in "ab"}


LOAD_FINISH = {"id": "finishes", "kind": "carried", "title": T("Screed and finishes 1.5 kN/m2, 1.8 kN/m", "Estrich und Belag 1,5 kN/m2, 1,8 kN/m"), "fxN": 0, "fyN": 0, "distributed": {"bone": "slab", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -1800}, "source": "authored-estimate", "reference": "en-1991-1-1"}
LOAD_IMPOSED = {"id": "imposed-floor", "kind": "imposed", "title": T("Imposed floor load 2.0 kN/m2, 2.4 kN/m", "Nutzlast 2,0 kN/m2, 2,4 kN/m"), "fxN": 0, "fyN": 0, "distributed": {"bone": "slab", "fromM": 0, "toM": L, "wxNPerM": 0, "wyNPerM": -2400}, "source": "literature", "reference": "en-1991-1-1"}
LOAD_POINT = {"id": "point-load", "kind": "live", "title": T("Concentrated load 2.0 kN", "Einzellast 2,0 kN"), "anchor": "load-point", "fxN": 0, "fyN": -2000, "source": "literature", "reference": "en-1991-1-1"}
STORED_CONTACTS = ["bearer-a-ground", "bearer-b-ground"]
SLING_CONTACTS = [f"sling-{e}-{s}" for e, s, _ in BAILS]
BEARINGS = ["bearing-a", "bearing-b"]


def poses():
    def pose(id_, title, desc, bones, root, contacts, state, loads, key=False):
        p = {"id": id_, "title": title, "description": desc}
        if key:
            p["key"] = True
        p["bones"] = bones
        if root is not None:
            p["root"] = {"x": 0, "y": root, "angleDeg": 0}
        p.update({"ground": "authored", "contacts": contacts, "thermal": binding(state), "loads": loads})
        return p
    up = bails_up()
    return [
        pose("stored", T("Stored on bearers", "Auf Kanthölzern gelagert"), T("The plank lies in the yard on two squared timbers 0.45 m in from its saw-cut ends, the swivel bails folded flat; its own weight goes through the bearers into the ground.", "Die Platte liegt auf dem Lagerplatz auf zwei Kanthölzern 0,45 m von ihren Sägeschnitten, die Klappbügel flach umgelegt; ihr Eigengewicht geht über die Kanthölzer in den Boden."), {}, None, STORED_CONTACTS, "stored", []),
        pose("bails-raised", T("Bails raised, slings hooked", "Bügel aufgestellt, Anschlag eingehängt"), T("In-between key of the lift: the four bails stand up for the crane hooks while the plank still lies on the bearers.", "Zwischenschlüssel des Hubs: die vier Bügel stehen für die Kranhaken auf, während die Platte noch auf den Kanthölzern liegt."), dict(up), None, STORED_CONTACTS, "stored", [], key=True),
        pose("lifted", T("Lifted by the four lifting points", "An den vier Anschlagpunkten angehoben"), T("The plank hangs 0.90 m above the bearers from four vertical sling legs hooked into the bails; the bearers stay on the ground.", "Die Platte hängt 0,90 m über den Kanthölzern an vier senkrechten Anschlagsträngen in den Bügeln; die Kanthölzer bleiben am Boden."), {**up, **bearers_down(LIFT_DY)}, LIFT_DY, SLING_CONTACTS, "lifted", []),
        pose("reinstalled", T("Reinstalled as a floor slab", "Als Deckenplatte wieder eingebaut"), T("The plank spans 5.90 m between two wall crowns with 100 mm bearing, its soffit at 2.60 m, under screed, finishes and the imposed load of a residential floor; the bearers stay behind in the yard.", "Die Platte spannt 5,90 m zwischen zwei Wandkronen mit 100 mm Auflager, Untersicht auf 2,60 m, unter Estrich, Belag und Nutzlast einer Wohndecke; die Kanthölzer bleiben auf dem Lagerplatz."), bearers_down(REINSTALL_DY), REINSTALL_DY, BEARINGS, "reinstalled", [LOAD_FINISH, LOAD_IMPOSED]),
        pose("reinstalled-point-load", T("Reinstalled, point load at midspan", "Wieder eingebaut, Einzellast in Feldmitte"), T("The reinstalled plank under screed and finishes with the 2.0 kN concentrated load of a category A floor at midspan.", "Die wieder eingebaute Platte unter Estrich und Belag mit der Einzellast 2,0 kN einer Decke der Kategorie A in Feldmitte."), {**bearers_down(REINSTALL_DY), "load-carrier": {"dx": 2.5}}, REINSTALL_DY, BEARINGS, "reinstalled", [LOAD_FINISH, LOAD_POINT]),
    ]


def clips():
    def span(a, f, t):
        return {"anchor": a, "fromS": f, "toS": t}
    lift = {
        "id": "lift", "title": T("Lift off the bearers", "Abheben von den Kanthölzern"),
        "description": T("The bails swing up for the hooks, then the crane takes the 2.2 t plank off its bearers and hoists it 0.90 m; the bearers stay on the ground.", "Die Bügel klappen für die Haken auf, dann nimmt der Kran die 2,2 t schwere Platte von den Kanthölzern und hebt sie 0,90 m an; die Kanthölzer bleiben am Boden."),
        "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.2, "pose": "bails-raised", "ease": "smooth"}, {"t": 4.4, "pose": "lifted", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}],
        "contacts": [span(a, 0, 1.2) for a in STORED_CONTACTS] + [span(a, 1.2, 5) for a in SLING_CONTACTS],
        "phases": [{"id": "raise-bails", "fromS": 0, "toS": 1.2, "title": T("Raise the bails", "Bügel aufstellen")}, {"id": "hoist", "fromS": 1.2, "toS": 4.4, "title": T("Hoist", "Anheben")}, {"id": "hang", "fromS": 4.4, "toS": 5, "title": T("Hang", "Hängen")}],
        "events": [{"t": 1.2, "kind": "contact-off", "title": T("The plank leaves the bearers", "Die Platte verlässt die Kanthölzer")}, {"t": 1.2, "kind": "contact-on", "title": T("The four sling legs take the load", "Die vier Anschlagstränge übernehmen die Last")}],
        "tracks": [],
    }
    moving = {
        "id": "moving-point-load", "title": T("Moving point load on the reinstalled slab", "Wandernde Einzellast auf der wieder eingebauten Platte"),
        "description": T("The 2.0 kN load walks from near bearing A to near bearing B over the reinstalled plank under screed and finishes.", "Die Last von 2,0 kN wandert über die wieder eingebaute Platte unter Estrich und Belag von nahe Auflager A bis nahe Auflager B."),
        "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 0}}}, {"t": 6, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 5.0}}}],
        "contacts": [span(a, 0, 6) for a in BEARINGS],
        "phases": [{"id": "travel", "fromS": 0, "toS": 6, "title": T("Travel", "Wanderung")}],
        "events": [{"t": 0, "kind": "motion", "title": T("Load starts near bearing A", "Last beginnt nahe Auflager A")}],
        "tracks": [],
    }
    ramp = {
        "id": "loading-ramp", "title": T("Floor load comes on", "Nutzlast kommt auf"),
        "description": T("On the reinstalled plank the imposed floor load rises from zero to 2.0 kN/m2 over screed and finishes.", "Auf der wieder eingebauten Platte steigt die Nutzlast über Estrich und Belag von null auf 2,0 kN/m2."),
        "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "reinstalled", "ease": "smooth"}, {"t": 5, "pose": "reinstalled", "ease": "smooth"}],
        "contacts": [span(a, 0, 5) for a in BEARINGS],
        "phases": [{"id": "ramp", "fromS": 0, "toS": 4, "title": T("Ramp", "Aufbau")}, {"id": "hold", "fromS": 4, "toS": 5, "title": T("Hold", "Halten")}],
        "events": [{"t": 4, "kind": "load-on", "title": T("Imposed load at full value", "Nutzlast vollständig")}],
        "tracks": [{"target": "load:imposed-floor", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 1}, {"t": 5, "v": 1}]}],
    }
    return [lift, moving, ramp]


def asset():
    return {
        "id": "reclaimed-hollow-core-slab",
        "category": "components",
        "title": T("Reclaimed Hollow-Core Slab", "Gebrauchte Spannbeton-Hohlplatte"),
        "summary": T("A 6.00 m prestressed hollow-core plank of 1.2 x 0.2 m sawn out of a dismantled floor: saw-cut ends with six open cores and five cut strands, grout remnants on the keyed joint flanks, a painted soffit and four drilled lifting points; stored on two timber bearers at the lifting points, lifted by its swivel bails and reinstalled as a 5.90 m floor span.",
                     "Eine 6,00 m lange Spannbeton-Hohlplatte 1,2 x 0,2 m, aus einer rückgebauten Decke gesägt: gesägte Enden mit sechs offenen Hohlräumen und fünf geschnittenen Litzen, Vergussreste an den profilierten Fugenflanken, eine gestrichene Untersicht und vier gebohrte Anschlagpunkte; auf zwei Kanthölzern an den Anschlagpunkten gelagert, an ihren Klappbügeln angehoben und als Decke mit 5,90 m Spannweite wieder eingebaut."),
        "tags": ["component", "reclaimed", "concrete", "precast", "prestressed", "hollow-core", "slab", "floor", "structural"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFS,
                       "notes": T(f"Section of a standard 200 mm hollow-core unit (EN 1168): 1196 mm soffit, 1166 mm top, six round cores of 140 mm at 190 mm pitch, five strands of 12.5 mm at 40 mm cover; net section {AREA_NET:.4f} m2 for {AREA_NET * L * RHO_C / 1000:.2f} t of concrete at 2500 kg/m3; saw-cut to 6.00 m, reinstalled span 5.90 m on 100 mm bearings. Lifting points of 24 mm through-bolts in the outer cores at 0.60 m from the ends, bearers at 0.45 m.",
                                  f"Querschnitt einer üblichen 200-mm-Hohlplatte (EN 1168): Untersicht 1196 mm, Oberseite 1166 mm, sechs runde Hohlräume 140 mm im Abstand von 190 mm, fünf Litzen 12,5 mm mit 40 mm Überdeckung; Nettoquerschnitt {dn(AREA_NET, 4)} m2 für {dn(AREA_NET * L * RHO_C / 1000, 2)} t Beton bei 2500 kg/m3; auf 6,00 m gesägt, Spannweite nach Wiedereinbau 5,90 m auf 100-mm-Auflagern. Anschlagpunkte als 24-mm-Durchsteckbolzen in den äußeren Hohlräumen 0,60 m von den Enden, Kanthölzer bei 0,45 m.")},
        "dimensions": {"heightM": r6(HEIGHT), "widthM": L, "depthM": 1.40, "massKg": round(TOTAL_KG, 2)},
        "materials": MATERIALS,
        "rig": rig(),
        "anchors": anchors(),
        "representations": {s: representation(s) for s in ("50", "100", "200", "500")},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal_model(),
        "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "clinical": False, "assumptions": [
            T("Stored: the plank rests on two timber bearers 5.10 m apart (unilateral ground contacts with friction); the bearers are compression members of 100 x 100 mm C24.", "Gelagert: die Platte liegt auf zwei Kanthölzern im Abstand von 5,10 m (einseitige Bodenkontakte mit Reibung); die Kanthölzer sind Druckglieder 100 x 100 mm C24."),
            T("Lifted: four vertical sling legs at the bail hooks 4.80 m apart and 0.95 m across; the weight of the bearers left on the ground (0.6 % of the total) rides in the rig and is carried with the plank in this pose.", "Angehoben: vier senkrechte Anschlagstränge an den Bügelhaken im Abstand von 4,80 m und 0,95 m quer; das Gewicht der am Boden verbleibenden Kanthölzer (0,6 % der Gesamtmasse) hängt im Skelett und wird in dieser Pose mit der Platte getragen."),
            T(f"Reinstalled: simply supported over 5.90 m (pin and roller under the 100 mm bearings, EN 1168 and fib Bulletin 6); prestressed concrete C45/55, Ecm 36 GPa, net section {AREA_NET:.4f} m2, I {I_W * 1e4:.2f}e-4 m4 about the horizontal axis, uncracked; the prestress and its camber are not modelled.", f"Wieder eingebaut: Einfeldträger über 5,90 m (Fest- und Rollenlager unter den 100-mm-Auflagern, EN 1168 und fib Bulletin 6); Spannbeton C45/55, Ecm 36 GPa, Nettoquerschnitt {dn(AREA_NET, 4)} m2, I {dn(I_W * 1e4, 2)}e-4 m4 um die waagerechte Achse, ungerissen; Vorspannung und Überhöhung sind nicht modelliert."),
            T(f"Self-weight from the net section with the cores, 2500 kg/m3 (EN 1991-1-1 Table A.1), plus 18 kg of grout remnants; lifting hardware {LIFT_KG:.1f} kg per point.", f"Eigengewicht aus dem Nettoquerschnitt mit den Hohlräumen, 2500 kg/m3 (EN 1991-1-1 Tabelle A.1), zuzüglich 18 kg Vergussreste; Anschlagteile {dn(LIFT_KG, 1)} kg je Punkt."),
        ]},
        "volumes": volumes(),
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


def main():
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print(f"net section {AREA_NET:.5f} m2, centroid v {V_C * 1000:.2f} mm, I {I_W:.4e} m4, extreme fibre {E_FIBRE * 1000:.1f} mm")
    print(f"grout {GROUT_A * 1e4:.2f} cm2 x {GROUT_LEN:.2f} m; slab {SLAB_KG:.1f} kg, bearer {BEARER_KG:.2f} kg, lifting point {LIFT_KG:.2f} kg, total {TOTAL_KG:.1f} kg")
    print(f"self-weight {AREA_NET * RHO_C * 9.81 / 1.2 / 1000:.3f} kN/m2, height {HEIGHT:.3f} m, areas {AREA}")


if __name__ == "__main__":
    main()
