"""🏗️ Authoritative author script of the staffage asset `reclaimed-precast-stair`: states every hand-chosen number and writes `🔣️.json` next to it.

@see ./author-geometry.py — the flight profile, terrazzo pieces, traces and masses
@see ../../brief.md — the asset contract of the collection
"""
import json
import math
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import importlib.util

_spec = importlib.util.spec_from_file_location("geo", pathlib.Path(__file__).parent / "author-geometry.py")
geo = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(geo)

from shapely.geometry import Polygon
from shapely.ops import unary_union

HERE = pathlib.Path(__file__).parent
SLUG = "reclaimed-precast-stair"


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(v):
    """🎯 Round to 0.1 mm."""
    return round(v + 0.0, 5)


def pts(points):
    """🎯 Rounded point list."""
    return [[r6(x), r6(y)] for x, y in points]


ALPHA_DEG = math.degrees(geo.ALPHA)
U0, V0 = geo.U0, geo.V0
REST_Y = geo.BEARER + V0
FLIGHT_MASS, CU, CV, _ = geo.flight_mass()
BEARER_MASS = geo.BEARER * geo.BEARER * geo.BEARER_LEN * geo.DENSITY["softwood"]
PITCH = math.hypot(geo.G, geo.R)


def bl(p):
    """🦴 Flight frame (u', v') to the bone frame of `flight`."""
    return (p[0] - U0, p[1] - V0)


def inst_bl(points):
    """🦴 Installed points to the bone frame of `flight`."""
    return [bl(geo.local(p)) for p in points]


def world_rest(p):
    """🌍 Installed point to world rest coordinates (x, y)."""
    u, v = geo.local(p)
    return (u, geo.BEARER + v)


def installed_of_local(u, v):
    """🔄 Flight frame (u', v') back to installed (x, y)."""
    return (u * geo.CA - v * geo.SA, u * geo.SA + v * geo.CA - geo.WAIST_V)


#region numbers of the poses
CORE_W = [world_rest(p) for p in geo.CORE_INSTALLED]
NOSING_U, NOSING_V = geo.local((0.0, geo.R))
LOAD_X, LOAD_Y = bl((NOSING_U, NOSING_V))
LOAD_TRAVEL = 7 * PITCH


def placed(theta_deg, dx, dy, p):
    """🌍 World point of a flight-frame point with the flight turned by theta and its origin moved by (dx, dy)."""
    t = math.radians(theta_deg)
    u, v = p[0] - U0, p[1] - V0
    return (U0 + dx + u * math.cos(t) - v * math.sin(t), REST_Y + dy + u * math.sin(t) + v * math.cos(t))


ALL_LOCAL = [geo.local(p) for p in geo.CORE_INSTALLED] + [geo.local(q) for s in geo.STUB_INSTALLED for q in [s]]


def min_y(theta, dx, dy):
    """📉 Lowest world height of the flight outline."""
    return min(placed(theta, dx, dy, p)[1] for p in ALL_LOCAL)


LIFT_CLEAR = 0.55
LIFT_DY = LIFT_CLEAR - min_y(ALPHA_DEG, 0, 0)
INSTALL_D = 1.5
_ox, _oy = installed_of_local(U0, V0)
FFL = 0.20
INSTALL_DX = _ox + INSTALL_D - U0
INSTALL_DY = _oy + FFL - REST_Y
PIVOT = (geo.BEARER_U[0] - 0.06, geo.BEARER)


def pivot_offsets(theta):
    """🔁 Travel and hoist offsets that turn the flight by theta about the down-slope top edge of the lower bearer."""
    t = math.radians(theta)
    pu, pv = PIVOT[0] - U0, PIVOT[1] - REST_Y
    ox = PIVOT[0] - (pu * math.cos(t) - pv * math.sin(t))
    oy = PIVOT[1] - (pu * math.sin(t) + pv * math.cos(t))
    return ox - U0, oy - REST_Y
#endregion


#region materials and references
REFS = [
    ("din-18065", "DIN 18065:2015-03 Building stairs, terms, measuring rules, main dimensions: step formula 2 r + g = 590 to 650 mm, rise 140 to 200 mm and going 230 to 370 mm for residential buildings, usable flight width of at least 1.00 m in buildings with more than two dwellings."),
    ("din-1045-1972", "DIN 1045:1972-01 Concrete and reinforced concrete, design and construction: concrete grade B 25 (nominal cube strength 25 N/mm2), modulus of elasticity of B 25 = 30 000 N/mm2 (Table 11)."),
    ("din-488-1984", "DIN 488-1:1984-09 Reinforcing steels, grades and properties: BSt 420 S ribbed bar, yield strength 420 N/mm2."),
    ("din-18500", "DIN 18500:2006-12 Cast stone (Betonwerkstein), requirements and testing: terrazzo face mix of cement and crushed marble, ground and polished, face layer of at least 15 mm on precast stair treads."),
    ("vdi-bv-bs-6205", "VDI/BV-BS 6205:2012-04 Lifting inserts and lifting insert systems for precast concrete elements: threaded lifting sockets cast flush into the element, lifted with screwed-in loops or ring clutches."),
    ("din-4070-1", "DIN 4070-1:1958-01 Softwood, cross-section dimensions and static values of sawn timber, stock squared timber and battens: stock squared timber 12/12 cm."),
    ("en-338", "EN 338:2016 Structural timber, strength classes: C24 mean density 420 kg/m3 at 12 % moisture, E0,mean = 11 GPa."),
    ("en-1991-1-1", "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Table A.1 (reinforced concrete 25 kN/m3) and Tables 6.1 and 6.2 (category C5, areas susceptible to large crowds: q_k = 5.0 kN/m2, Q_k = 4.5 kN)."),
    ("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties: reinforced concrete (1 % steel) 2.3 W/mK and 1000 J/kgK, softwood 0.13 W/mK and 1600 J/kgK, cement mortar 1.0 W/mK, steel 50 W/mK and 450 J/kgK."),
    ("emissivity-literature", "ASHRAE Handbook Fundamentals (2021), thermal radiation properties of building materials: weathered concrete, ground stone, weathered timber and rusted steel (typical rounded emissivity and solar absorptance, not table look-ups)."),
    ("en-iso-13786", "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); about half of it is taken as the effective storage depth of an exposed face."),
    ("bauteilboersen-practice", "Bauteilnetz Deutschland and bauteilbörse basel, practice notes on the dismantling, storage and resale of used building components: precast stair flights are sawn or broken out along the landing joints and stored lying on squared-timber bearers."),
    ("din-1356-1", "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups, stairs)."),
]
MATERIALS = [
    {"id": "reinforced-concrete", "title": T("Reinforced concrete B 25, weathered formed faces", "Stahlbeton B 25, verwitterte Schalflächen"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "densityKgM3": 2500, "conductivityWmK": 2.3, "specificHeatJKgK": 1000, "elasticModulusPa": 30000000000, "strengthPa": 25000000, "source": "literature", "reference": "din-1045-1972"},
    {"id": "terrazzo", "title": T("Terrazzo face layer, ground, worn and soiled", "Terrazzovorsatz, geschliffen, abgetreten und verschmutzt"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 2400, "conductivityWmK": 2.0, "specificHeatJKgK": 1000, "source": "authored-estimate", "reference": "din-18500"},
    {"id": "softwood", "title": T("Squared spruce bearer C24, weathered grey", "Kantholz Fichte C24, vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    {"id": "cement-mortar", "title": T("Bedding mortar remnant", "Mörtelbettreste"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 1900, "conductivityWmK": 1.0, "specificHeatJKgK": 1000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "steel", "title": T("Reinforcing and socket steel, rusted", "Betonstahl und Hülsenstahl, verrostet"), "emissivity": 0.8, "solarAbsorptivity": 0.8, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 200000000000, "strengthPa": 420000000, "source": "literature", "reference": "din-488-1984"},
]
#endregion


#region rig and anchors
SECTION_W = geo.WIDTH * geo.WAIST
RIG = {"kind": "structural", "root": "yard", "bones": [
    {"id": "yard", "parent": None, "title": T("Yard datum (ground, massless)", "Lagerplatz-Bezug (Boden, masselos)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    *[{"id": f"bearer-{k}", "parent": "yard", "title": T(f"{e} squared-timber bearer 12/12", f"{d} Kantholz 12/12"), "x": r6(geo.BEARER_U[i] - 0.06), "y": 0, "angleDeg": 0, "lengthM": 0.12, "joint": {"type": "fixed"},
       "mass": {"kg": r6(BEARER_MASS / 2), "comX": 0.06, "comY": 0.06, "distribution": "point", "source": "authored-estimate", "reference": "en-338"},
       "section": {"areaM2": 0.0144, "secondMomentM4": 1.728e-05, "extremeFibreM": 0.06, "material": "softwood", "title": T("Squared timber 120 x 120 mm", "Kantholz 120 x 120 mm")}}
      for i, (k, e, d) in enumerate((("lower", "Lower", "Unteres"), ("upper", "Upper", "Oberes")))],
    {"id": "travel", "parent": "yard", "title": T("Crane travel (massless sling carrier, horizontal)", "Kranfahrt (masseloser Anschlagträger, waagerecht)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.5, "maxM": 2.0}},
    {"id": "hoist", "parent": "travel", "title": T("Crane hoist (massless sling carrier, vertical)", "Kranhub (masseloser Anschlagträger, senkrecht)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -0.5, "maxM": 1.5}},
    {"id": "flight", "parent": "hoist", "title": T("Flight (waist axis along the soffit)", "Lauf (Achse der Laufplatte entlang der Untersicht)"), "x": r6(U0), "y": r6(REST_Y), "angleDeg": 0, "lengthM": r6(geo.LENGTH), "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 35},
     "mass": {"kg": round(FLIGHT_MASS, 2), "comX": r6(geo.LENGTH / 2), "comY": r6(CV - V0), "distribution": "uniform", "source": "literature", "reference": "en-1991-1-1"},
     "section": {"areaM2": r6(SECTION_W), "secondMomentM4": float(f"{geo.WIDTH * geo.WAIST ** 3 / 12:.6e}"), "extremeFibreM": r6(geo.WAIST / 2), "material": "reinforced-concrete", "title": T("Waist slab 1100 x 160 mm (steps not counted)", "Laufplatte 1100 x 160 mm (Stufen nicht angesetzt)")}},
    {"id": "load-carrier", "parent": "flight", "title": T("Point-load carrier on the nosing line (massless)", "Lastträger auf der Stufenkantenlinie (masselos)"), "x": r6(LOAD_X), "y": r6(LOAD_Y), "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": r6(LOAD_TRAVEL)}},
]}


def fl_anchor(aid, inst, kind, title, **extra):
    """📍 An anchor on the flight bone at an installed point."""
    u, v = bl(geo.local(inst))
    return {"id": aid, "bone": "flight", "x": r6(u), "y": r6(v), "kind": kind, "title": title, **extra}


def soffit_anchor(aid, u_prime, title):
    """📍 A bearer contact on the soffit at u'."""
    u, v = bl((u_prime, 0.0))
    return {"id": aid, "bone": "flight", "x": r6(u), "y": r6(v), "kind": "contact", "title": title, "support": "contact", "normalDeg": 90, "footprintLengthM": 0.12, "footprintWidthM": geo.WIDTH, "frictionCoefficient": 0.5}


def socket_point(i, z):
    """🔩 Installed point and lateral position of a lifting socket on tread i."""
    x0, x1, top, _ = geo.tread_installed(i)
    return (x0 + geo.SOCKET_X, top), z


ANCHORS = [
    soffit_anchor("rest-lower", geo.BEARER_U[0], T("Soffit on the lower bearer", "Untersicht auf dem unteren Kantholz")),
    soffit_anchor("rest-upper", geo.BEARER_U[1], T("Soffit on the upper bearer", "Untersicht auf dem oberen Kantholz")),
    *[dict(fl_anchor(f"sling-{n + 1}", socket_point(i, z)[0], "support", T(f"Lifting socket {n + 1}, tread {i} ({'left' if z > 0 else 'right'})", f"Transportankerhülse {n + 1}, Stufe {i} ({'links' if z > 0 else 'rechts'})"), support="pin"), z=z) for n, (i, z) in enumerate(geo.SOCKETS)],
    fl_anchor("bearing-bottom", (-0.084, -geo.BLOCK), "support", T("Lower end bedded on the ground-floor slab (pin)", "Unteres Ende im Mörtelbett auf der Bodenplatte (Festlager)"), support="pin", footprintLengthM=0.13, footprintWidthM=geo.WIDTH),
    fl_anchor("bearing-top", (geo.TOP_END - 0.05, geo.TOP_NIB_UNDER), "support", T("Sawn upper nib on the landing corbel, elastomer strip (roller)", "Gesägte obere Nase auf der Podestkonsole, Elastomerstreifen (Rollenlager)"), support="roller", normalDeg=90, footprintLengthM=0.08, footprintWidthM=geo.WIDTH),
    {"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": T("Concentrated load on the nosing line", "Einzellast auf der Stufenkantenlinie")},
]
#endregion


#region 2D helpers
def shape(sid, bone, role, z, filled, tone, geom, region=None, regions=None, material=None, hatch=None, pen=None):
    """✏️ One authored 2D shape."""
    out = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        out["region"] = region
    if regions:
        out["regions"] = regions
    if material:
        out["material"] = material
    if hatch:
        out["hatch"] = hatch
    if pen is not None:
        out["penMm"] = pen
    out.update(geom)
    return out


def poly(points, closed=True):
    """🔷 Poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def rect(x0, y0, x1, y1):
    """⬛ Rect geometry from two corners."""
    return {"kind": "rect", "x": r6(min(x0, x1)), "y": r6(min(y0, y1)), "w": r6(abs(x1 - x0)), "h": r6(abs(y1 - y0))}


def ellipse(cx, cy, rx, ry=None):
    """⭕ Ellipse geometry."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rx), "ry": r6(ry if ry is not None else rx)}


CONCRETE, TERRAZZO, TIMBER = "concrete-faces", "terrazzo-treads", "timber-bearers"
TONE_CONCRETE, TONE_TERRAZZO, TONE_TIMBER, TONE_STEEL, TONE_MORTAR, TONE_SAWN = 0.58, 0.84, 0.46, 0.22, 0.7, 0.66


def core_profile(scale):
    """🧱 Installed concrete profile at a scale: 1:50 and 1:100 keep the broken bottom nib end, 1:200 straightens it, 1:500 also fills the dapped notches."""
    c = geo.CORE_INSTALLED
    if scale in (50, 100):
        return c
    straight = [c[0], c[5]] + c[6:]
    straight[0] = (-geo.BOTTOM_STRIP, -geo.TERRAZZO)
    straight[1] = (-geo.BOTTOM_STRIP, -geo.NIB)
    return straight


def union_profile(scale):
    """🧱 Installed outline of concrete and terrazzo together (1:200, 1:500 bodies)."""
    if scale == 500:
        x_low = geo.X_SOFFIT_LOW
        return ([(-geo.BOTTOM_STRIP, 0.0), (-geo.BOTTOM_STRIP, -geo.BLOCK), (x_low, -geo.BLOCK), (geo.TOP_END, geo.soffit_y(geo.TOP_END)), (geo.TOP_END, 9 * geo.R), (8 * geo.G, 9 * geo.R)]
                + [pt for i in range(8, 0, -1) for pt in ((i * geo.G, i * geo.R), ((i - 1) * geo.G, i * geo.R))] + [(0.0, 0.0)])
    poly_ = unary_union([Polygon(core_profile(scale))] + [Polygon(geo.tread_profile(i, "sharp")) for i in range(10)])
    return list(poly_.exterior.coords)[:-1]


def plan_strip(inst_pts):
    """🗺️ World rest x-range (plan) of an installed outline."""
    xs = [world_rest(p)[0] for p in inst_pts]
    return min(xs), max(xs)


def front_band(inst_pts):
    """🪟 World rest y-range (front) of an installed outline."""
    ys = [world_rest(p)[1] for p in inst_pts]
    return min(ys), max(ys)
#endregion


#region representations
def elevation(scale):
    """📐 Side elevation of one scale: flight in its bone frame, bearers in theirs."""
    out = []
    if scale in (50, 100):
        out.append(shape("core", "flight", "body", 1, True, TONE_CONCRETE, poly(inst_bl(core_profile(scale))), region=CONCRETE, material="reinforced-concrete"))
        for i in range(10):
            out.append(shape(f"terrazzo-{i:02d}", "flight", "body", 2, True, TONE_TERRAZZO, poly(inst_bl(geo.tread_profile(i, "sharp"))), region=TERRAZZO, material="terrazzo"))
        x0, _, top, _ = geo.tread_installed(geo.CHIP_STEP)
        chip = [(x0, top - geo.CHIP_DY), (x0 + 0.018, top - 0.012), (x0 + geo.CHIP_DX, top)]
        out.append(shape("spall-nosing-03", "flight", "detail", 3, False, 0.3, poly(inst_bl(chip), closed=False), region=TERRAZZO))
        hole_r = 0.009 if scale == 50 else 0.012
        for n, (xh, hh) in enumerate([(0.30, 0.10), (1.00, 0.10), (1.70, 0.10), (2.40, 0.10)]):
            u, v = bl(geo.local((xh, geo.soffit_y(xh) + hh / geo.CA)))
            out.append(shape(f"dowel-hole-{n + 1}", "flight", "texture", 3, False, 0.25, ellipse(u, v, hole_r), region=CONCRETE))
        sx0, sx1 = geo.TOP_END, geo.TOP_END
        kerf = [(geo.TOP_END - 0.012, geo.TOP_NIB_UNDER + 0.012), (geo.TOP_END - 0.012, 9 * geo.R - geo.TERRAZZO - 0.012)]
        out.append(shape("saw-kerf-top", "flight", "texture", 3, False, 0.3, poly(inst_bl(kerf), closed=False), region=CONCRETE))
    else:
        out.append(shape("flight", "flight", "body", 1, True, TONE_CONCRETE if scale == 500 else 0.66, poly(inst_bl(union_profile(scale))), regions=[CONCRETE, TERRAZZO], material="reinforced-concrete"))
        if scale == 200:
            for i in range(1, 10):
                x0, x1, top, _ = geo.tread_installed(i)
                out.append(shape(f"tread-line-{i:02d}", "flight", "detail", 2, False, 0.3, poly(inst_bl([(x0, top - geo.TERRAZZO), (x1, top - geo.TERRAZZO)]), closed=False), region=TERRAZZO))
    if scale == 50:
        for n, z in enumerate(geo.STUB_Z):
            a, b = geo.STUB_INSTALLED
            ux, uy = b[0] - a[0], b[1] - a[1]
            ln = math.hypot(ux, uy)
            nx, ny = -uy / ln * 0.005, ux / ln * 0.005
            if n == 0:
                out.append(shape("bar-stub", "flight", "body", 2.5, True, TONE_STEEL, poly(inst_bl([(a[0] + 0.03 * ux / ln + nx, a[1] + 0.03 * uy / ln + ny), (b[0] + nx, b[1] + ny), (b[0] - nx, b[1] - ny), (a[0] + 0.03 * ux / ln - nx, a[1] + 0.03 * uy / ln - ny)])), material="steel"))
        out.append(shape("mortar-top", "flight", "body", 2.5, True, TONE_MORTAR, poly(inst_bl(geo.MORTAR_TOP)), material="cement-mortar"))
        out.append(shape("mortar-bottom", "flight", "body", 2.5, True, TONE_MORTAR, poly(inst_bl(geo.MORTAR_BOTTOM)), material="cement-mortar"))
    for k in ("lower", "upper"):
        out.append(shape(f"bearer-{k}", f"bearer-{k}", "body", 0, True, TONE_TIMBER, rect(0, 0, 0.12, 0.12), region=TIMBER, material="softwood"))
        if scale in (50, 100):
            out.append(shape(f"bearer-{k}-check", f"bearer-{k}", "detail" if scale == 50 else "texture", 0.5, False, 0.3, poly([(0.06, 0.06), (0.02, 0.105)] if k == "lower" else [(0.06, 0.06), (0.105, 0.025)], closed=False), region=TIMBER))
        if scale == 50:
            out.append(shape(f"bearer-{k}-rings", f"bearer-{k}", "texture", 0.5, False, 0.3, ellipse(0.055, 0.065, 0.03, 0.028), region=TIMBER))
    return out


def front(scale):
    """🪟 Front elevation from ahead (seen from the upper end): flight bands at world rest heights, bearers below."""
    out = []
    lo = 0.12
    v_end_low = world_rest((geo.TOP_END, geo.TOP_NIB_UNDER))[1]
    v_nib_top = world_rest((geo.TOP_END, 9 * geo.R - geo.TERRAZZO))[1]
    top_sharp = max(world_rest(p)[1] for p in geo.tread_profile(9, "sharp"))
    top_worn = max(world_rest(p)[1] for p in geo.tread_profile(9, "worn"))
    h, wz = geo.HALF, geo.WORN_HALF
    if scale in (50, 100):
        out.append(shape("core", "flight", "body", 1, True, TONE_CONCRETE, rect(-h, lo, h, v_end_low), region=CONCRETE, material="reinforced-concrete"))
        out.append(shape("nib-saw-cut", "flight", "body", 1.2, True, TONE_SAWN, rect(-h, v_end_low, h, v_nib_top), region=CONCRETE, material="reinforced-concrete"))
        top = [(-h, v_nib_top), (h, v_nib_top), (h, top_sharp), (wz, top_sharp)]
        top += [(wz, top_worn), (-wz, top_worn)] if scale == 50 else []
        top += [(-wz, top_sharp), (-h, top_sharp)]
        out.append(shape("terrazzo-09", "flight", "body", 2, True, TONE_TERRAZZO, poly(top), region=TERRAZZO, material="terrazzo"))
        if scale == 50:
            yb = (v_end_low + v_nib_top) / 2
            for n, z in enumerate(geo.CUT_BARS_Z):
                out.append(shape(f"cut-bar-{n + 1}", "flight", "texture", 3, False, 0.2, ellipse(z, yb, 0.005), region=CONCRETE))
            ym = world_rest((geo.TOP_END - 0.006, geo.TOP_NIB_UNDER - 0.012))[1]
            out.append(shape("mortar-top", "flight", "body", 1.5, True, TONE_MORTAR, rect(geo.MORTAR_TOP_Z[0], ym, geo.MORTAR_TOP_Z[1], v_end_low), material="cement-mortar"))
    else:
        out.append(shape("flight", "flight", "body", 1, True, TONE_CONCRETE if scale == 500 else 0.66, rect(-h, lo, h, top_sharp), regions=[CONCRETE, TERRAZZO], material="reinforced-concrete"))
        if scale == 200:
            out.append(shape("nib-line", "flight", "detail", 2, False, 0.3, poly([(-h, v_nib_top), (h, v_nib_top)], closed=False), region=CONCRETE))
    for k in ("lower", "upper"):
        out.append(shape(f"bearer-{k}", f"bearer-{k}", "body", 0 if k == "lower" else 0.2, True, TONE_TIMBER, rect(-geo.BEARER_LEN / 2, 0, geo.BEARER_LEN / 2, 0.12), region=TIMBER, material="softwood"))
    return out


def plan(scale):
    """🗺️ Plan from above at rest: the tilted treads and risers as strips, bearers under the flight."""
    out = []
    h = geo.HALF
    full_lo = min(world_rest(p)[0] for p in core_profile(scale if scale != 500 else 200))
    full_hi = max(world_rest(p)[0] for p in geo.tread_profile(9, "sharp") + [(geo.TOP_END, geo.TOP_NIB_UNDER)])
    if scale == 500:
        full_lo = min(world_rest(p)[0] for p in union_profile(500))
        full_hi = max(world_rest(p)[0] for p in union_profile(500))
    for k in ("lower", "upper"):
        u = geo.BEARER_U[0 if k == "lower" else 1]
        out.append(shape(f"bearer-{k}", f"bearer-{k}", "body", 0, True, TONE_TIMBER, rect(u - 0.06, -geo.BEARER_LEN / 2, u + 0.06, geo.BEARER_LEN / 2), region=TIMBER, material="softwood"))
    if scale in (200, 500):
        out.append(shape("flight", "flight", "body", 1, True, TONE_CONCRETE if scale == 500 else 0.66, rect(full_lo, -h, full_hi, h), regions=[CONCRETE, TERRAZZO], material="reinforced-concrete"))
        if scale == 200:
            for i in range(1, 10):
                x0, _, top, _ = geo.tread_installed(i)
                xa = world_rest((x0, top))[0]
                out.append(shape(f"nosing-{i:02d}", "flight", "detail", 2, False, 0.3, poly([(xa, -h), (xa, h)], closed=False), region=TERRAZZO))
        return out
    out.append(shape("core", "flight", "body", 1, True, TONE_CONCRETE, rect(full_lo, -h, full_hi, h), region=CONCRETE, material="reinforced-concrete"))
    for i in range(10):
        a, b = plan_strip(geo.tread_profile(i, "sharp"))
        x0, x1, top, nos = geo.tread_installed(i)
        b = world_rest((x1, top))[0]
        a = world_rest((x0, top))[0] if i == 0 else world_rest((x0, top - geo.TERRAZZO))[0]
        out.append(shape(f"terrazzo-{i:02d}", "flight", "body", 2, True, TONE_TERRAZZO, rect(a, -h, b, h), region=TERRAZZO, material="terrazzo"))
        if scale == 50 and nos:
            xr = world_rest((x0 + geo.WEAR_R, top - geo.WEAR_DIP))[0]
            out.append(shape(f"wear-{i:02d}", "flight", "texture", 3, False, 0.3, poly([(xr, -geo.WORN_HALF), (xr, geo.WORN_HALF)], closed=False), region=TERRAZZO))
    x0, _, top, _ = geo.tread_installed(geo.CHIP_STEP)
    xa, xb = world_rest((x0, top - geo.CHIP_DY))[0], world_rest((x0 + geo.CHIP_DX, top))[0]
    out.append(shape("spall-nosing-03", "flight", "detail", 3, False, 0.3, poly([(xa, -h), (xb, -h + 0.02), (xb, -0.47), (xa, -0.47)], closed=False), region=TERRAZZO))
    if scale == 50:
        for n, (i, z) in enumerate(geo.SOCKETS):
            p, _ = socket_point(i, z)
            out.append(shape(f"socket-{n + 1}", "flight", "texture", 3, False, 0.2, ellipse(world_rest(p)[0], z, geo.SOCKET_D / 2 * geo.CA, geo.SOCKET_D / 2), region=CONCRETE))
        a, b = geo.STUB_INSTALLED
        xa, xb = world_rest(b)[0], world_rest(a)[0]
        for n, z in enumerate(geo.STUB_Z):
            out.append(shape(f"bar-stub-{n + 1}", "flight", "body", 1.5, True, TONE_STEEL, rect(xa - 0.004, z - 0.005, xb, z + 0.005), material="steel"))
    return out


PENS = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35},
        200: {"body": 0.18, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def rep_decisions(scale):
    """📝 The design decisions of one representation."""
    if scale == 50:
        return {
            "silhouette": T("Side elevation of the flight lying on its soffit on two 12/12 squared-timber bearers, treads tilted by the 32 degree pitch: the concrete side face with its nine risers of 175 mm and goings of 280 mm, the 30 mm terrazzo face layer of the eight treads and the two landing strips, the dapped upper nib sawn flush, the dapped lower nib broken out with a ragged end and a bent cut bar, and the mortar remnants under both bearings. Front: the flight seen from its upper end, the sawn nib face with its seven cut bars over the formed end face, the top landing strip with the worn walking zone of its nosing, the bearers below. Plan: the tilted treads and risers as alternating strips, worn nosings, the spalled nosing of tread 3, four lifting sockets and the four bar stubs at the lower end.", "Seitenansicht des auf der Untersicht auf zwei Kanthölzern 12/12 liegenden Laufs, die Stufen um die Steigung von 32 Grad geneigt: die Betonseitenfläche mit neun Steigungen zu 175 mm und Auftritten zu 280 mm, der 30-mm-Terrazzovorsatz der acht Trittstufen und der beiden Podeststreifen, die bündig gesägte obere Ausklinkung, die herausgebrochene untere Ausklinkung mit unregelmäßigem Ende und abgebogenem Stabende sowie die Mörtelreste unter beiden Auflagern. Vorderansicht: der Lauf vom oberen Ende gesehen, die gesägte Nasenfläche mit sieben angeschnittenen Stäben über der geschalten Stirnfläche, der obere Podeststreifen mit der abgetretenen Gehzone seiner Kante, darunter die Kanthölzer. Draufsicht: die geneigten Tritt- und Setzstufen als abwechselnde Streifen, abgetretene Kanten, die abgeplatzte Kante der Stufe 3, vier Transportankerhülsen und die vier Stabenden am unteren Ende."),
            "retained": T("Every riser and tread at its true size, the 30 mm terrazzo layer (0.6 mm), the 100 mm dapped nibs, the 12 mm mortar beds, the 10 mm bar stubs, the 40 mm lifting sockets, the four 18 mm dowel holes of the removed handrail on the right side face and the spall of 50 x 22 mm at the right end of nosing 3.", "Jede Steigung und jeder Auftritt in wahrer Größe, der 30-mm-Terrazzovorsatz (0,6 mm), die 100-mm-Ausklinkungen, die 12-mm-Mörtelbetten, die 10-mm-Stabenden, die 40-mm-Transportankerhülsen, die vier 18-mm-Dübellöcher des entfernten Handlaufs an der rechten Seitenfläche und die Abplatzung von 50 x 22 mm am rechten Ende der Kante 3."),
            "merged": T("Concrete side face, risers, soffit and end faces are one concrete shape per view; each terrazzo piece is one shape.", "Betonseitenfläche, Setzstufen, Untersicht und Stirnflächen sind je Ansicht eine Betonform; jedes Terrazzostück ist eine Form."),
            "omitted": T("The 4 mm dishing of the treads in the walking line (0.08 mm), the reinforcement inside the concrete and the plastic plugs of the sockets.", "Die 4-mm-Mulde der Trittflächen in der Gehlinie (0,08 mm), die Bewehrung im Beton und die Kunststoffstopfen der Hülsen."),
            "thinElements": T("Dowel holes, cut bars, sockets, the saw kerf and the tangent line of each rounded nosing in the walking zone are texture strokes at 0.1 mm; the spall and the timber checks are detail strokes at 0.18 mm.", "Dübellöcher, angeschnittene Stäbe, Hülsen, der Sägeschnitt und die Tangentenlinie jeder gerundeten Kante in der Gehzone sind Texturstriche zu 0,1 mm; Abplatzung und Holzrisse sind Detailstriche zu 0,18 mm."),
            "hatching": T("No hatch: no view is a section; materials read by tone (terrazzo light, concrete mid grey, sawn face a shade lighter, timber dark, steel darkest).", "Keine Schraffur: keine Ansicht ist ein Schnitt; die Werkstoffe lesen sich über den Ton (Terrazzo hell, Beton mittelgrau, Sägefläche eine Nuance heller, Holz dunkel, Stahl am dunkelsten)."),
            "styles": T("Body outline 0.35 mm, details 0.18 mm, texture 0.1 mm (DIN 1356-1 line group 0.35).", "Körperkontur 0,35 mm, Details 0,18 mm, Textur 0,1 mm (DIN 1356-1 Liniengruppe 0,35)."),
            "overlays": T("Thermal regions per shape (concrete, terrazzo, timber); structural views draw the waist member, the bearer contacts, the sling points and the landing bearings of the pose.", "Thermische Regionen je Form (Beton, Terrazzo, Holz); die Tragwerksansichten zeichnen das Laufplattenglied, die Kantholzkontakte, die Anschlagpunkte und die Podestauflager der Pose."),
        }
    if scale == 100:
        return {
            "silhouette": T("The same side, front and plan views with the true stepped profile, the terrazzo layer (0.3 mm), the dapped nibs, the ragged lower end and the bearers.", "Dieselben Seiten-, Vorder- und Draufsichten mit dem wahren Stufenprofil, dem Terrazzovorsatz (0,3 mm), den Ausklinkungen, dem unregelmäßigen unteren Ende und den Kanthölzern."),
            "retained": T("Risers, treads, terrazzo pieces, nibs, bearers, the spalled nosing as a stroke and the dowel holes as texture.", "Steigungen, Auftritte, Terrazzostücke, Ausklinkungen, Kanthölzer, die abgeplatzte Kante als Strich und die Dübellöcher als Textur."),
            "merged": T("The worn walking zone of the top strip is merged into its straight outline; the sawn nib face is a band of its own tone.", "Die abgetretene Gehzone des oberen Streifens geht in seinem geraden Umriss auf; die gesägte Nasenfläche ist ein Band in eigenem Ton."),
            "omitted": T("Bar stubs (10 mm), mortar beds (12 mm), lifting sockets, cut-bar dots and nosing wear lines: below the 15 mm smallest feature.", "Stabenden (10 mm), Mörtelbetten (12 mm), Transportankerhülsen, Stabpunkte und Kantenabnutzungslinien: unter der kleinsten Einzelheit von 15 mm."),
            "thinElements": T("The spall and timber checks as 0.1 mm texture or 0.13 mm detail strokes.", "Abplatzung und Holzrisse als Textur- zu 0,1 mm oder Detailstriche zu 0,13 mm."),
            "hatching": T("No hatch; tones as at 1:50.", "Keine Schraffur; Töne wie bei 1:50."),
            "styles": T("Body 0.25 mm, detail 0.13 mm, texture 0.1 mm.", "Körper 0,25 mm, Detail 0,13 mm, Textur 0,1 mm."),
            "overlays": T("As at 1:50.", "Wie bei 1:50."),
        }
    if scale == 200:
        return {
            "silhouette": T("The flight as one stepped outline with the dapped nibs, lying on the two bearers; front as one band with the line of the sawn nib, plan as one rectangle with the nine nosings as lines.", "Der Lauf als ein gestufter Umriss mit den Ausklinkungen, auf den beiden Kanthölzern liegend; Vorderansicht als ein Band mit der Linie der gesägten Nase, Draufsicht als ein Rechteck mit den neun Kanten als Linien."),
            "retained": T("All nine risers (0.88 mm), the nibs (0.5 mm), the bearers (0.6 mm) and the terrazzo layer as a detail line under each tread.", "Alle neun Steigungen (0,88 mm), die Ausklinkungen (0,5 mm), die Kanthölzer (0,6 mm) und der Terrazzovorsatz als Detaillinie unter jedem Auftritt."),
            "merged": T("Terrazzo and concrete are one shape in the side and front views (the 30 mm layer would be 0.15 mm, under the 0.2 mm printable feature); the shape lists both regions.", "Terrazzo und Beton sind in Seiten- und Vorderansicht eine Form (die 30-mm-Schicht wäre 0,15 mm, unter der druckbaren Einzelheit von 0,2 mm); die Form führt beide Regionen."),
            "omitted": T("The ragged end of the lower nib, the spall, holes, sockets and timber checks.", "Das unregelmäßige Ende der unteren Nase, die Abplatzung, Löcher, Hülsen und Holzrisse."),
            "thinElements": T("Tread lines at 0.1 mm detail pen.", "Trittlinien mit der Detailfeder 0,1 mm."),
            "hatching": T("No hatch.", "Keine Schraffur."),
            "styles": T("Body 0.18 mm, detail 0.1 mm, no texture pen.", "Körper 0,18 mm, Detail 0,1 mm, keine Texturfeder."),
            "overlays": T("Merged regions are painted area-weighted.", "Zusammengefasste Regionen werden flächengewichtet eingefärbt."),
        }
    return {
        "silhouette": T("The flight as one stepped outline with square ends lying on the two bearers (6.5 x 0.6 mm on paper); front and plan each one rectangle over the bearers.", "Der Lauf als ein gestufter Umriss mit rechtwinkligen Enden auf den beiden Kanthölzern (6,5 x 0,6 mm auf Papier); Vorder- und Draufsicht je ein Rechteck über den Kanthölzern."),
        "retained": T("The nine steps (0.35 mm risers), the flight width and the bearer positions.", "Die neun Stufen (Steigungen 0,35 mm), die Laufbreite und die Lage der Kanthölzer."),
        "merged": T("Terrazzo and concrete one shape listing both regions; the dapped nibs are filled to square ends (0.2 mm notches).", "Terrazzo und Beton eine Form mit beiden Regionen; die Ausklinkungen sind zu rechtwinkligen Enden gefüllt (Kerben von 0,2 mm)."),
        "omitted": T("Every trace of use; the tread strips of the plan (0.47 mm and 0.19 mm alternating would merge in print).", "Jede Gebrauchsspur; die Trittstreifen der Draufsicht (abwechselnd 0,47 mm und 0,19 mm verliefen im Druck)."),
        "thinElements": T("None.", "Keine."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Body 0.13 mm only.", "Nur Körper 0,13 mm."),
        "overlays": T("Merged regions are painted area-weighted.", "Zusammengefasste Regionen werden flächengewichtet eingefärbt."),
    }


def representation(scale):
    """🗺️ One authored representation."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [],
            "geometry": {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}, "decisions": rep_decisions(scale)}
#endregion


#region volumes
def extrude(sid, bone, material, tone, profile, w0, w1, role="body", **extra):
    """🧊 A prism of a bone-frame profile along w."""
    return {"id": sid, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts(profile), "plane": "xy", "from": r6(w0), "to": r6(w1), **extra}


def bearer_boxes(scale, ids=("bearer-lower", "bearer-upper")):
    """🪵 The two bearers as boxes in their bone frames."""
    cr = {50: 0.006, 100: 0.0}[scale] if scale in (50, 100) else 0.0
    out = []
    for sid, k in zip(ids, ("lower", "upper")):
        s = {"id": sid, "kind": "box", "bone": f"bearer-{k}", "material": "softwood", "role": "body", "tone": TONE_TIMBER, "size": [0.12, 0.12, geo.BEARER_LEN], "transform": {"translation": [0.06, 0.06, 0.0]}}
        if cr:
            s["cornerRadius"] = cr
        out.append(s)
    return out


def body(scale):
    """🧊 The body list of one scale."""
    out = []
    if scale in (50, 100):
        out.append(extrude("core", "flight", "reinforced-concrete", TONE_CONCRETE, inst_bl(core_profile(scale)), -geo.HALF, geo.HALF))
        for i in range(10):
            zones = geo.tread_zones(i) if scale == 50 else ([(-geo.HALF, geo.CHIP_Z[1], "chip"), (geo.CHIP_Z[1], geo.HALF, "sharp")] if i == geo.CHIP_STEP else [(-geo.HALF, geo.HALF, "sharp")])
            for w0, w1, kind in zones:
                part = {"chip": "spall", "worn": "worn"}.get(kind, "right" if w1 <= -geo.WORN_HALF + 1e-9 else ("left" if w0 >= geo.WORN_HALF - 1e-9 else "rest"))
                sid = f"terrazzo-{i:02d}" + ("" if len(zones) == 1 else f"-{part}")
                tone = TONE_TERRAZZO - (0.04 if kind == "worn" else 0.0) - 0.01 * (i % 2)
                out.append(extrude(sid, "flight", "terrazzo", tone, inst_bl(geo.tread_profile(i, kind)), w0, w1))
    else:
        out.append(extrude("flight", "flight", "reinforced-concrete", TONE_CONCRETE, inst_bl(union_profile(scale)), -geo.HALF, geo.HALF))
    if scale == 50:
        a, b = geo.STUB_INSTALLED
        pa, pb = bl(geo.local(a)), bl(geo.local(b))
        for n, z in enumerate(geo.STUB_Z):
            out.append({"id": f"bar-stub-{n + 1}", "kind": "capsule", "bone": "flight", "material": "steel", "role": "body", "tone": TONE_STEEL, "p0": [r6(pa[0] + 0.02), r6(pa[1] + 0.004), z], "p1": [r6(pb[0]), r6(pb[1]), z], "r0": 0.005, "r1": 0.005})
        out.append(extrude("mortar-top", "flight", "cement-mortar", TONE_MORTAR, inst_bl(geo.MORTAR_TOP), *geo.MORTAR_TOP_Z))
        out.append(extrude("mortar-bottom", "flight", "cement-mortar", TONE_MORTAR, inst_bl(geo.MORTAR_BOTTOM), *geo.MORTAR_BOTTOM_Z))
        nrm = (geo.SA, geo.CA, 0.0)
        for n, (i, z) in enumerate(geo.SOCKETS):
            p, _ = socket_point(i, z)
            u, v = bl(geo.local(p))
            out.append({"id": f"socket-{n + 1}", "kind": "revolve", "bone": "flight", "material": "steel", "role": "texture", "tone": TONE_STEEL, "texture": True, "profile": [[-0.002, 0.0], [-0.002, geo.SOCKET_D / 2], [0.002, geo.SOCKET_D / 2], [0.002, 0.0]], "axis": [r6(nrm[0]), r6(nrm[1]), 0.0], "origin": [r6(u), r6(v), z]})
    out += bearer_boxes(scale)
    return out


def lowered_core(scale, depth):
    """🧱 Installed concrete profile with every tread and strip lowered by a terrazzo depth (thermal lists of 1:200 and 1:500)."""
    if scale == 200:
        return core_profile(200)
    top = 9 * geo.R - depth
    return ([(-geo.BOTTOM_STRIP, -depth), (-geo.BOTTOM_STRIP, -geo.BLOCK), (geo.X_SOFFIT_LOW, -geo.BLOCK), (geo.TOP_END, geo.soffit_y(geo.TOP_END)), (geo.TOP_END, top), (8 * geo.G, top)]
            + [pt for i in range(8, 0, -1) for pt in ((i * geo.G, i * geo.R - depth), ((i - 1) * geo.G, i * geo.R - depth))] + [(0.0, -depth)])


def thermal(scale):
    """🌡️ The thermal list of one scale."""
    out = []
    if scale in (50, 100):
        for s in body(scale):
            if s["material"] == "reinforced-concrete":
                out.append({"id": f"th-{s['id']}", "of": s["id"], "region": CONCRETE})
            elif s["material"] == "terrazzo":
                out.append({"id": f"th-{s['id']}", "of": s["id"], "region": TERRAZZO})
            elif s["material"] == "softwood":
                out.append({"id": f"th-{s['id']}", "of": s["id"], "region": TIMBER})
        return out
    depth = geo.TERRAZZO if scale == 200 else 0.075
    out.append(dict(extrude("th-concrete", "flight", "reinforced-concrete", TONE_CONCRETE, inst_bl(lowered_core(scale, depth)), -geo.HALF, geo.HALF), region=CONCRETE))
    for i in range(10):
        x0, x1, top, _ = geo.tread_installed(i)
        prof = [(x0, top - depth), (x1, top - depth), (x1, top), (x0, top)]
        if scale == 500 and i == 0:
            prof = [(-geo.BOTTOM_STRIP, -depth), (0.0, -depth), (0.0, 0.0), (-geo.BOTTOM_STRIP, 0.0)]
        out.append(dict(extrude(f"th-terrazzo-{i:02d}", "flight", "terrazzo", TONE_TERRAZZO, inst_bl(prof), -geo.HALF, geo.HALF), region=TERRAZZO))
    for s in bearer_boxes(scale):
        out.append({"id": f"th-{s['id']}", "of": s["id"], "region": TIMBER})
    return out


FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def structure(scale):
    """🏗️ The structure list of one scale: waist member along the flight bone, bearer members, the load-carrier joint down to the waist."""
    f = FEATURE[scale]
    jh = NOSING_V - geo.WAIST
    hb = {50: 0.05, 100: 0.06, 200: 0.07, 500: 0.08}[scale]
    riser_du, tread_du = geo.R * geo.SA, geo.G * geo.CA
    ul, ur = -riser_du * (1 - hb / jh) + 0.005, tread_du * (1 - hb / jh) - 0.005
    return [
        {"id": "flight-member", "kind": "box", "bone": "flight", "material": "reinforced-concrete", "role": "body", "tone": 0.6, "size": [r6(geo.LENGTH), geo.WAIST, geo.WIDTH], "transform": {"translation": [r6(geo.LENGTH / 2), 0.0, 0.0]}, "member": "flight", "part": "member"},
        *[{"id": f"bearer-{k}-member", "kind": "box", "bone": f"bearer-{k}", "material": "softwood", "role": "body", "tone": 0.5, "size": [0.12, 0.12, geo.BEARER_LEN], "transform": {"translation": [0.06, 0.06, 0.0]}, "member": f"bearer-{k}", "part": "member"} for k in ("lower", "upper")],
        {"id": "joint-load-carrier", "kind": "box", "bone": "load-carrier", "material": "steel", "role": "body", "tone": 0.3, "size": [r6(ur - ul), r6(hb), r6(min(geo.WIDTH, 20 * f))], "transform": {"translation": [r6((ul + ur) / 2), r6(-jh + hb / 2), 0.0]}, "member": "load-carrier", "part": "joint"},
    ]


def vol_decisions(scale):
    """📝 The decisions of one authored volume."""
    if scale == 50:
        return [
            T("Body: the concrete flight as one prism of its true side profile (nine risers 175 mm, goings 280 mm, waist 160 mm, dapped nibs of 100 mm, the upper one sawn flush, the lower one broken out with a ragged end); the terrazzo face layer as 30 mm pieces on the eight treads and the two landing strips, each nosing split into the worn walking zone of 600 mm (rounded by 25 mm and dished by 4 mm) and two sharp edge zones; tread 3 carries a spall of 50 x 22 mm at its right end. Four bent bar stubs of 10 mm leave the broken lower nib, 12 mm mortar beds cling under both bearings, and four 40 mm lifting sockets sit flush in treads 2 and 7 (texture). The two 12/12 bearers carry the soffit.", "Körper: der Betonlauf als ein Prisma seines wahren Seitenprofils (neun Steigungen 175 mm, Auftritte 280 mm, Laufplatte 160 mm, Ausklinkungen von 100 mm, die obere bündig gesägt, die untere mit unregelmäßigem Ende herausgebrochen); der Terrazzovorsatz als 30-mm-Stücke auf den acht Auftritten und den beiden Podeststreifen, jede Kante geteilt in die abgetretene Gehzone von 600 mm (um 25 mm gerundet und 4 mm ausgemuldet) und zwei scharfe Randzonen; Stufe 3 trägt an ihrem rechten Ende eine Abplatzung von 50 x 22 mm. Vier abgebogene 10-mm-Stabenden treten aus der gebrochenen unteren Nase, 12-mm-Mörtelbetten haften unter beiden Auflagern, vier 40-mm-Transportankerhülsen sitzen bündig in den Stufen 2 und 7 (Textur). Die beiden Kanthölzer 12/12 tragen die Untersicht."),
            T("Thermal: every concrete, terrazzo and timber body solid is its own thermal surface (references); bar stubs, mortar remnants and sockets are below 0.01 m2 and merged into the concrete region.", "Thermisch: jeder Beton-, Terrazzo- und Holzkörper ist seine eigene thermische Oberfläche (Verweise); Stabenden, Mörtelreste und Hülsen liegen unter 0,01 m2 und gehen in der Betonregion auf."),
            T("Structure: the waist slab 1100 x 160 mm along the flight bone, the two bearers as members, and the load carrier joined to the waist by a 50 mm block inside the step under its nosing.", "Tragwerk: die Laufplatte 1100 x 160 mm entlang des Laufknochens, die beiden Kanthölzer als Glieder und der Lastträger, mit einem 50-mm-Block in der Stufe unter ihrer Kante an die Laufplatte angeschlossen."),
        ]
    if scale == 100:
        return [
            T("Body: concrete prism with the ragged lower end and both dapped nibs, ten terrazzo pieces at their sharp profile (the 25 mm nosing wear and the 4 mm dishing merged), tread 3 split into its 80 mm spalled end and the rest, bearers as plain boxes. Bar stubs (10 mm), mortar beds (12 mm) and sockets are below the 15 mm feature and left out.", "Körper: Betonprisma mit dem unregelmäßigen unteren Ende und beiden Ausklinkungen, zehn Terrazzostücke in ihrem scharfen Profil (die 25-mm-Kantenabnutzung und die 4-mm-Mulde aufgegangen), Stufe 3 geteilt in ihr 80-mm-Ende mit Abplatzung und den Rest, Kanthölzer als einfache Quader. Stabenden (10 mm), Mörtelbetten (12 mm) und Hülsen liegen unter der Einzelheit von 15 mm und entfallen."),
            T("Thermal: references to the body solids by region.", "Thermisch: Verweise auf die Körper nach Region."),
            T("Minimal: the structure list keeps waist member, two bearer members and the load-carrier joint, the least that carries the flight; only the joint is resized to the 15 mm feature.", "Minimal: die Tragwerksliste behält Laufplattenglied, zwei Kantholzglieder und das Lastträgergelenk, das Wenigste, das den Lauf trägt; nur das Gelenk wird auf die Einzelheit von 15 mm angepasst."),
        ]
    if scale == 200:
        return [
            T("Body: the flight as one prism of the union of concrete and terrazzo with the dapped nibs and a straight lower end (the ragged end of 12 to 20 mm is below the 30 mm feature); the bearers as boxes.", "Körper: der Lauf als ein Prisma der Vereinigung aus Beton und Terrazzo mit den Ausklinkungen und einem geraden unteren Ende (das unregelmäßige Ende von 12 bis 20 mm liegt unter der Einzelheit von 30 mm); die Kanthölzer als Quader."),
            T("Thermal: the concrete as the body profile less the 30 mm terrazzo layer, the terrazzo as ten full-width 30 mm slabs, so both regions keep their faces; bearers by reference.", "Thermisch: der Beton als Körperprofil ohne den 30-mm-Terrazzovorsatz, der Terrazzo als zehn durchgehende 30-mm-Platten, sodass beide Regionen ihre Flächen behalten; Kanthölzer per Verweis."),
            T("Minimal: the structure list is the same least load path as at 1:100 with the joint at the 30 mm feature.", "Minimal: die Tragwerksliste ist derselbe geringste Lastweg wie bei 1:100 mit dem Gelenk auf der Einzelheit von 30 mm."),
        ]
    return [
        T("Body: one prism of the stepped outline with square ends (the 100 mm dapped notches are filled, they would be single features at 75 mm) and the two bearers.", "Körper: ein Prisma des gestuften Umrisses mit rechtwinkligen Enden (die 100-mm-Ausklinkungen sind gefüllt, sie wären bei 75 mm einzelne Einzelheiten) und die beiden Kanthölzer."),
        T("Thermal: the concrete prism lowered by 75 mm under every tread and strip and ten 75 mm terrazzo slabs on it, the smallest feature of the scale standing for the 30 mm layer; bearers by reference.", "Thermisch: das Betonprisma unter jedem Auftritt und Streifen um 75 mm abgesenkt und darauf zehn 75-mm-Terrazzoplatten, die kleinste Einzelheit des Maßstabs steht für die 30-mm-Schicht; Kanthölzer per Verweis."),
        T("Minimal: the structure list is the least load path, waist member, two bearer members and the 75 mm load-carrier joint.", "Minimal: die Tragwerksliste ist der geringste Lastweg, Laufplattenglied, zwei Kantholzglieder und das 75-mm-Lastträgergelenk."),
    ]


def volume(scale):
    """🧊 One authored volume."""
    return {"status": "authored", "denominator": scale, "body": body(scale), "thermal": thermal(scale), "structure": structure(scale), "decisions": vol_decisions(scale)}
#endregion


#region thermal model and areas
def prism_area_local(profile_inst, w0, w1):
    """📏 Outer area of a prism of an installed profile."""
    p = Polygon(profile_inst)
    return 2 * p.area + p.length * (w1 - w0)


AREA_CONCRETE = prism_area_local(geo.CORE_INSTALLED, -geo.HALF, geo.HALF)
AREA_TERRAZZO = sum(geo.zone_union_area(i) for i in range(10))
AREA_TIMBER = 2 * (2 * 0.12 * 0.12 + 4 * 0.12 * geo.BEARER_LEN)


def region(rid, title, material, area, geometry, lc, sky, hc):
    """🔥 One passive thermal region."""
    return {"id": rid, "title": title, "kind": "surface", "model": "passive", "material": material, "areaM2": round(area, 3), "characteristicLengthM": lc, "geometry": geometry, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "en-iso-13786"}


THERMAL = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
    "operating": [],
    "regions": [
        region(CONCRETE, T("Weathered concrete faces (sides, risers, soffit, nib faces)", "Verwitterte Betonflächen (Seiten, Setzstufen, Untersicht, Nasenflächen)"), "reinforced-concrete", AREA_CONCRETE, "vertical-plate", 0.3, 0.5, 170000),
        region(TERRAZZO, T("Terrazzo treads and landing strips, worn", "Terrazzo-Trittstufen und Podeststreifen, abgetreten"), "terrazzo", AREA_TERRAZZO, "horizontal-plate-up", 0.28, 0.9, 72000),
        region(TIMBER, T("Squared-timber bearers", "Kanthölzer"), "softwood", AREA_TIMBER, "horizontal-plate-up", 0.12, 0.6, 26000),
    ],
    "emitters": [],
    "assumptions": [
        T("Passive faces store heat in an effective depth of half the 24 h penetration depth (EN ISO 13786): concrete about 68 mm (delta 136 mm for lambda 2.3 W/mK, rho 2500 kg/m3, c 1000 J/kgK, 170 kJ/m2K), the terrazzo face layer its own 30 mm (72 kJ/m2K), timber about 35 mm (26 kJ/m2K).", "Passive Flächen speichern Wärme in einer wirksamen Tiefe der halben 24-h-Eindringtiefe (EN ISO 13786): Beton etwa 68 mm (Delta 136 mm für Lambda 2,3 W/mK, Rho 2500 kg/m³, c 1000 J/kgK, 170 kJ/m²K), der Terrazzovorsatz seine eigenen 30 mm (72 kJ/m²K), Holz etwa 35 mm (26 kJ/m²K)."),
        T("Weathered faces differ from new ones: weathered grey concrete absorbs about 0.70 of the sun (new formed concrete 0.55 to 0.65), the soiled and worn terrazzo about 0.50 (freshly polished white terrazzo 0.30 to 0.40), weathered grey spruce about 0.70 (fresh 0.40 to 0.50); emissivities 0.92, 0.90 and 0.90, authored estimates guided by the ASHRAE Handbook.", "Verwitterte Flächen unterscheiden sich von neuen: vergrauter Beton absorbiert etwa 0,70 der Sonne (neuer Schalbeton 0,55 bis 0,65), der verschmutzte und abgetretene Terrazzo etwa 0,50 (frisch polierter weißer Terrazzo 0,30 bis 0,40), vergraute Fichte etwa 0,70 (frisch 0,40 bis 0,50); Emissionsgrade 0,92, 0,90 und 0,90, Schätzungen nach dem ASHRAE Handbook."),
        T("The rusted bar stubs, the flush socket rims and the mortar remnants are below 0.01 m2 together and counted with the concrete faces.", "Die verrosteten Stabenden, die bündigen Hülsenränder und die Mörtelreste haben zusammen unter 0,01 m² und zählen zu den Betonflächen."),
        T("The asset is a passive element: it has no nameplate power (0 W).", "Das Asset ist ein passives Bauteil: es hat keine Nennleistung (0 W)."),
        T("Reference case: a mild sunny day in the component yard; reinstalled in a stair hall the flight receives little sun (pose bindings).", "Referenzfall: ein milder sonniger Tag auf dem Bauteillager; im Treppenhaus wieder eingebaut erhält der Lauf wenig Sonne (Posenbindungen)."),
    ],
    "passengers": [],
}


def tb(concrete_sun, terrazzo_sun, timber_sun, stored=True):
    """🌬️ The thermal binding of a pose."""
    regions = {CONCRETE: {"exposedFraction": 1, "sunlitFraction": concrete_sun}, TERRAZZO: {"exposedFraction": 1, "sunlitFraction": terrazzo_sun}, TIMBER: {"exposedFraction": 1, "sunlitFraction": timber_sun, "contact": {"fraction": 0.24, "tempC": 18}}}
    if stored:
        regions[CONCRETE]["contact"] = {"fraction": 0.03, "tempC": 20}
    return {"regions": regions, "emitters": {}}
#endregion


#region poses and clips
CROWD_TOTAL = 5000 * geo.WIDTH * (geo.TOP_END + geo.BOTTOM_STRIP)
CROWD_W = CROWD_TOTAL / geo.LENGTH
POINT_TREAD = 5
LIFTED = {"travel": {"dx": 0}, "hoist": {"dy": r6(LIFT_DY)}, "flight": {"angleDeg": r6(ALPHA_DEG)}}
INSTALLED = {"travel": {"dx": r6(INSTALL_DX)}, "hoist": {"dy": r6(INSTALL_DY)}, "flight": {"angleDeg": r6(ALPHA_DEG)}}
SLINGS = [f"sling-{n}" for n in range(1, 5)]
BEARINGS = ["bearing-bottom", "bearing-top"]
CROWD_LOAD = {"id": "crowd", "kind": "imposed", "title": T("Crowd load 5.0 kN/m2 (category C5)", "Menschengedränge 5,0 kN/m² (Kategorie C5)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "flight", "fromM": 0, "toM": r6(geo.LENGTH), "wxNPerM": 0, "wyNPerM": -round(CROWD_W, 1)}, "source": "literature", "reference": "en-1991-1-1"}
POINT_LOAD = {"id": "point-tread", "kind": "live", "title": T("Concentrated load 4.5 kN (category C5)", "Einzellast 4,5 kN (Kategorie C5)"), "anchor": "load-point", "fxN": 0, "fyN": -4500, "source": "literature", "reference": "en-1991-1-1"}
POSES = [
    {"id": "stored", "title": T("Stored on bearers", "Auf Kanthölzern gelagert"), "description": T("As the component yard keeps it: lying on its soffit on two 12/12 squared-timber bearers, treads tilted by the 32 degree pitch, self-weight only.", "Wie das Bauteillager ihn aufbewahrt: auf der Untersicht auf zwei Kanthölzern 12/12 liegend, die Stufen um die Steigung von 32 Grad geneigt, nur Eigengewicht."), "bones": {}, "ground": "authored", "contacts": ["rest-lower", "rest-upper"], "thermal": tb(0.3, 0.8, 0.4), "loads": []},
    {"id": "lifted", "title": T("Lifted on the four sockets", "An den vier Hülsen angehoben"), "description": T(f"Hanging from a four-leg chain sling screwed into the four threaded lifting sockets, legs set so the flight hangs at its 32 degree installation pitch {LIFT_CLEAR:.2f} m clear of the ground; the bearers stay behind on the ground.", f"Hängend an einem viersträngigen Kettengehänge, eingeschraubt in die vier Gewindehülsen, Stränge so eingestellt, dass der Lauf in seiner Einbausteigung von 32 Grad {LIFT_CLEAR:.2f} m über dem Boden hängt; die Kanthölzer bleiben am Boden liegen."), "bones": LIFTED, "ground": "authored", "contacts": SLINGS, "thermal": tb(0.3, 0.8, 0.4, False), "loads": []},
    {"id": "reinstalled", "title": T("Reinstalled between landings", "Zwischen Podesten wieder eingebaut"), "description": T("Back in its role: the lower end bedded in mortar on the ground-floor slab (pin), the sawn upper nib on an elastomer strip on the landing corbel 1.775 m above (roller); self-weight only.", "Wieder in seiner Rolle: das untere Ende im Mörtelbett auf der Bodenplatte (Festlager), die gesägte obere Nase auf einem Elastomerstreifen auf der Podestkonsole 1,775 m darüber (Rollenlager); nur Eigengewicht."), "bones": INSTALLED, "ground": "authored", "contacts": BEARINGS, "thermal": tb(0.1, 0.1, 0.4, False), "loads": []},
    {"id": "reinstalled-crowd", "title": T("Reinstalled, crowd load", "Wieder eingebaut, Menschengedränge"), "description": T(f"Category C5 crowd load 5.0 kN/m2 over the 1.10 x 2.79 m plan of the flight ({CROWD_TOTAL / 1000:.1f} kN), spread along the waist.", f"Menschengedränge der Kategorie C5 mit 5,0 kN/m² über den Grundriss des Laufs von 1,10 x 2,79 m ({CROWD_TOTAL / 1000:.1f} kN), entlang der Laufplatte verteilt."), "bones": INSTALLED, "ground": "authored", "contacts": BEARINGS, "thermal": tb(0.1, 0.1, 0.4, False), "loads": [CROWD_LOAD]},
    {"id": "reinstalled-point", "title": T("Reinstalled, point load on tread 5", "Wieder eingebaut, Einzellast auf Stufe 5"), "description": T("Category C5 concentrated load of 4.5 kN on the nosing of the middle tread.", "Einzellast der Kategorie C5 von 4,5 kN auf der Kante der mittleren Stufe."), "bones": {**INSTALLED, "load-carrier": {"dx": r6((POINT_TREAD - 1) * PITCH)}}, "ground": "authored", "contacts": BEARINGS, "thermal": tb(0.1, 0.1, 0.4, False), "loads": [POINT_LOAD]},
]


def lift_keys():
    """🎞️ Keys of the lift clip: slack taken up, the upper end rising about the lower bearer edge to the hanging pitch, then the hoist."""
    keys = [{"t": 0, "pose": "stored", "ease": "linear"}, {"t": 1.0, "pose": "stored", "ease": "linear"}]
    for k, theta in enumerate((6.4, 12.8, 19.2, 25.6, ALPHA_DEG)):
        dx, dy = pivot_offsets(theta)
        keys.append({"t": round(1.0 + 0.6 * (k + 1), 3), "pose": "stored", "ease": "linear", "bones": {"travel": {"dx": r6(dx)}, "hoist": {"dy": r6(dy)}, "flight": {"angleDeg": r6(theta)}}})
    keys[-1]["ease"] = "smooth"
    keys.append({"t": 7.0, "pose": "lifted", "ease": "smooth"})
    return keys


CLIPS = [
    {"id": "lift", "title": T("Lift from the bearers", "Von den Kanthölzern abheben"), "description": T("The crane takes up the slack, the upper end rises first about the edge of the lower bearer until the flight hangs at its 32 degree pitch, then the hoist lifts it clear; the bearers stay on the ground.", "Der Kran strafft das Gehänge, das obere Ende hebt sich zuerst um die Kante des unteren Kantholzes, bis der Lauf in seiner Steigung von 32 Grad hängt, dann hebt der Hub ihn frei; die Kanthölzer bleiben am Boden."),
     "durationS": 7, "fps": 30, "loop": False, "rootMotion": "none", "keys": lift_keys(),
     "contacts": [{"anchor": "rest-upper", "fromS": 0, "toS": 1.1}, {"anchor": "rest-lower", "fromS": 0, "toS": 4.0}] + [{"anchor": s, "fromS": 0.8, "toS": 7} for s in SLINGS],
     "phases": [{"id": "slack", "fromS": 0, "toS": 1.0, "title": T("Taking up the slack", "Gehänge straffen")}, {"id": "tilt", "fromS": 1.0, "toS": 4.0, "title": T("Upper end rising", "Oberes Ende hebt sich")}, {"id": "hoist", "fromS": 4.0, "toS": 7, "title": T("Hoisting clear", "Freiheben")}],
     "events": [{"t": 1.1, "kind": "contact-off", "title": T("Soffit leaves the upper bearer", "Untersicht verlässt das obere Kantholz"), "target": "rest-upper"}, {"t": 4.0, "kind": "contact-off", "title": T("Flight hangs free", "Lauf hängt frei"), "target": "rest-lower"}], "tracks": []},
    {"id": "set-in-place", "title": T("Set in place between landings", "Zwischen die Podeste einsetzen"), "description": T("The hanging flight travels 1.5 m and is lowered onto the ground-floor slab and the landing corbel.", "Der hängende Lauf fährt 1,5 m und wird auf die Bodenplatte und die Podestkonsole abgesenkt."),
     "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "lifted", "ease": "smooth"}, {"t": 3.0, "pose": "lifted", "ease": "smooth", "bones": {"travel": {"dx": r6(INSTALL_DX)}, "hoist": {"dy": r6(LIFT_DY)}, "flight": {"angleDeg": r6(ALPHA_DEG)}}}, {"t": 6.0, "pose": "reinstalled", "ease": "smooth"}],
     "contacts": [{"anchor": s, "fromS": 0, "toS": 6} for s in SLINGS] + [{"anchor": b, "fromS": 5.8, "toS": 6} for b in BEARINGS],
     "phases": [{"id": "travel", "fromS": 0, "toS": 3.0, "title": T("Crane travel", "Kranfahrt")}, {"id": "lower", "fromS": 3.0, "toS": 6.0, "title": T("Lowering onto the bearings", "Absenken auf die Auflager")}],
     "events": [{"t": 5.8, "kind": "contact-on", "title": T("Flight bears on slab and corbel", "Lauf liegt auf Bodenplatte und Konsole auf"), "target": "bearing-top"}], "tracks": []},
    {"id": "moving-point-load", "title": T("Point load climbing the flight", "Einzellast steigt den Lauf hinauf"), "description": T("The 4.5 kN concentrated load moves from the nosing of tread 1 to the nosing of tread 8.", "Die Einzellast von 4,5 kN wandert von der Kante der Stufe 1 zur Kante der Stufe 8."),
     "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-point", "ease": "linear", "bones": {**INSTALLED, "load-carrier": {"dx": 0}}}, {"t": 8, "pose": "reinstalled-point", "ease": "linear", "bones": {**INSTALLED, "load-carrier": {"dx": r6(LOAD_TRAVEL)}}}],
     "contacts": [{"anchor": b, "fromS": 0, "toS": 8} for b in BEARINGS],
     "phases": [{"id": "climb", "fromS": 0, "toS": 8, "title": T("Climb", "Aufstieg")}],
     "events": [{"t": 0, "kind": "motion", "title": T("Load on tread 1", "Last auf Stufe 1")}, {"t": 8, "kind": "motion", "title": T("Load on tread 8", "Last auf Stufe 8")}], "tracks": []},
]
#endregion


#region dimensions and document
def rest_extent():
    """📏 Extents of the rest pose from the drawn outlines: flight with stubs, bearers."""
    xs = [world_rest(p)[0] for p in geo.CORE_INSTALLED] + [world_rest(p)[0] for i in range(10) for p in geo.tread_profile(i, "sharp")] + [world_rest(geo.STUB_INSTALLED[1])[0] - 0.005]
    ys = [world_rest(p)[1] for p in geo.CORE_INSTALLED] + [world_rest(p)[1] for i in range(10) for p in geo.tread_profile(i, "sharp")] + [0.0]
    return min(xs), max(xs), min(ys), max(ys)


X0, X1, Y0, Y1 = rest_extent()
DIMENSIONS = {"heightM": round(Y1 - Y0, 4), "widthM": round(X1 - X0, 4), "depthM": geo.BEARER_LEN, "massKg": round(FLIGHT_MASS + BEARER_MASS, 2)}

ASSET = {
    "id": SLUG,
    "category": "components",
    "title": T("Reclaimed Precast Stair Flight", "Gebrauchter Betonfertigteil-Treppenlauf"),
    "summary": T(f"A dismantled precast reinforced-concrete stair flight of nine risers of 175 mm and goings of 280 mm (2 r + g = 630 mm, 32 degrees, DIN 18065), 1.10 m wide, waist 160 mm, {FLIGHT_MASS / 1000:.2f} t, with a 30 mm terrazzo face on its treads whose nosings are worn round in the walking line, a spalled nosing on tread 3, an upper bearing nib sawn flush with seven cut bars, a lower nib broken out with bent bar stubs and mortar remnants under both bearings. It lies on its soffit on two 12/12 squared-timber bearers, hangs from four threaded lifting sockets at its 32 degree pitch, and is reinstalled between the ground-floor slab and a landing corbel under a C5 crowd load and a moving 4.5 kN point load.",
               f"Ein ausgebauter Stahlbeton-Fertigteil-Treppenlauf mit neun Steigungen zu 175 mm und Auftritten zu 280 mm (2 s + a = 630 mm, 32 Grad, DIN 18065), 1,10 m breit, Laufplatte 160 mm, {FLIGHT_MASS / 1000:.2f} t, mit 30 mm Terrazzovorsatz auf den Trittstufen, deren Kanten in der Gehlinie rund abgetreten sind, einer abgeplatzten Kante an Stufe 3, einer bündig gesägten oberen Auflagernase mit sieben angeschnittenen Stäben, einer herausgebrochenen unteren Nase mit abgebogenen Stabenden und Mörtelresten unter beiden Auflagern. Er liegt auf der Untersicht auf zwei Kanthölzern 12/12, hängt an vier Gewindehülsen in seiner Steigung von 32 Grad und wird zwischen Bodenplatte und Podestkonsole unter Menschengedränge C5 und einer wandernden Einzellast von 4,5 kN wieder eingebaut."),
    "tags": ["component", "reclaimed", "structural", "concrete", "precast", "terrazzo", "stair", "timber"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFS],
                   "notes": T(f"Installed geometry: lower landing floor at 0, first riser face at 0, nine risers to 1.575 m, top end at 2.54 m; bottom landing strip 250 mm and top landing strip 300 mm with dapped nibs of 100 x 100 mm; soffit length {geo.LENGTH:.3f} m. Masses from the drawn profile: concrete {Polygon(geo.CORE_INSTALLED).area:.4f} m2 x 1.10 m x 2500 kg/m3, terrazzo pieces x 2400 kg/m3, bearers 0.12 x 0.12 x 1.40 m x 450 kg/m3.",
                              f"Einbaugeometrie: unteres Podest auf 0, erste Setzstufe bei 0, neun Steigungen bis 1,575 m, oberes Ende bei 2,54 m; unterer Podeststreifen 250 mm und oberer 300 mm mit Ausklinkungen von 100 x 100 mm; Länge der Untersicht {geo.LENGTH:.3f} m. Massen aus dem gezeichneten Profil: Beton {Polygon(geo.CORE_INSTALLED).area:.4f} m² x 1,10 m x 2500 kg/m³, Terrazzostücke x 2400 kg/m³, Kanthölzer 0,12 x 0,12 x 1,40 m x 450 kg/m³.")},
    "dimensions": DIMENSIONS,
    "materials": MATERIALS,
    "rig": RIG,
    "anchors": ANCHORS,
    "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T("The supports belong to the pose: the soffit on the two bearers when stored (unilateral contacts with friction 0.5), the four sockets as pins when lifted, the bedded lower end (pin) and the sawn upper nib (roller) when reinstalled.", "Die Auflager gehören zur Pose: die Untersicht auf den beiden Kanthölzern im Lager (einseitige Kontakte mit Reibung 0,5), die vier Hülsen als Gelenke beim Heben, das gebettete untere Ende (Festlager) und die gesägte obere Nase (Rollenlager) im Einbau."),
        T("The flight is idealised as its waist slab 1100 x 160 mm (A = 0.176 m2, I = 3.75e-4 m4) along the soffit with E = 30 GPa (B 25, DIN 1045:1972); the steps add stiffness and mass but no counted section; uniform self-weight along the waist.", "Der Lauf ist als Laufplatte 1100 x 160 mm (A = 0,176 m², I = 3,75e-4 m⁴) entlang der Untersicht mit E = 30 GPa (B 25, DIN 1045:1972) idealisiert; die Stufen tragen Steifigkeit und Masse bei, aber keinen angesetzten Querschnitt; gleichmäßiges Eigengewicht entlang der Laufplatte."),
        T("The solver treats the asset as one rigid body: the 18 kg of the two bearers left on the ground are carried by the active supports of every pose (0.9 % of the weight).", "Der Löser behandelt das Asset als einen starren Körper: die 18 kg der beiden am Boden verbleibenden Kanthölzer werden von den aktiven Auflagern jeder Pose getragen (0,9 % des Gewichts)."),
        T("The reduced bearing of the sawn nib (80 mm of the original 100 mm) and the broken lower nib are why the lower end is bedded on the slab when reinstalled; the cut bars of the nibs carry nothing.", "Die verkürzte Auflagertiefe der gesägten Nase (80 mm der ursprünglichen 100 mm) und die gebrochene untere Nase sind der Grund, warum das untere Ende im Einbau auf der Bodenplatte gebettet wird; die angeschnittenen Stäbe der Nasen tragen nichts."),
    ], "clinical": False},
    "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}
#endregion

if __name__ == "__main__":
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print("[DEBUG] written", DIMENSIONS, "areas", round(AREA_CONCRETE, 3), round(AREA_TERRAZZO, 3), round(AREA_TIMBER, 3), "lift dy", LIFT_DY, "install", INSTALL_DX, INSTALL_DY)
    for th in (0, 8, 16, 24, ALPHA_DEG):
        dx, dy = pivot_offsets(th)
        print("[DEBUG] pivot", th, round(dx, 4), round(dy, 4), "min y", round(min_y(th, dx, dy), 4))
