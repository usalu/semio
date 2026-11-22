#!/usr/bin/env python3
"""🏮️ Authors the staffage asset `used-bollard-light`: a 0.9 m path bollard light with louvre head on its anchor cage in a precast socket footing.

Every number is hand-chosen here once; 2D shapes and 3D solids of every scale are derived from the same part numbers, masses from section x density.
Run: python3 author-bollard-light.py  (writes 🔣️.json next to this file).

See ../../brief.md and the staffage README (🎓️teaching/🏛️architecture/🧍️staffage/README.md).
"""
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
PI = math.pi
SLUG = "used-bollard-light"


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def r6(x):
    """✂️ Rounds to micrometres."""
    return round(x + 0.0, 6)


def pts(seq):
    """📍️ Rounds a point list."""
    return [[r6(a), r6(b)] for a, b in seq]


Y_FOOT = -0.60
Y_CAGE = -0.42
Y_HEAD = 0.70
R_SOCKET_IN, R_SOCKET_OUT, H_SOCKET = 0.20, 0.25, 0.60
R_TUBE, T_TUBE = 0.100, 0.003
R_FLANGE, H_FLANGE = 0.150, 0.012
R_BOLT = 0.125
NUT_R = 0.009 / math.cos(PI / 6)
NUT_TOP = 0.0253
ROD_R, ROD_LEN = 0.006, 0.452
RING_IN, RING_OUT = 0.105, 0.145
LOUVRES = [0.050, 0.080, 0.110, 0.140]
LOUVRE_IN, LOUVRE_OUT = 0.088, 0.108
DIFF_IN, DIFF_OUT, DIFF_U0, DIFF_U1 = 0.080, 0.083, 0.014, 0.160
RIB_IN, RIB_OUT, RIB_T = 0.083, 0.103, 0.008
CAP_OUTLINE = [(0.152, 0.104), (0.152, 0.110), (0.168, 0.110), (0.178, 0.106), (0.188, 0.094), (0.195, 0.072), (0.199, 0.040), (0.200, 0.0)]
CAP_PROFILE = [(0.156, 0.0), (0.156, 0.104)] + CAP_OUTLINE + [(0.195, 0.0), (0.194, 0.036), (0.190, 0.064), (0.183, 0.085), (0.174, 0.097), (0.163, 0.100), (0.160, 0.100), (0.160, 0.0)]
BASE_PROFILE = [(0.0, 0.0), (0.0, 0.104), (0.014, 0.104), (0.014, 0.070), (0.005, 0.070), (0.005, 0.0)]
DENT_SEGMENTS = [(0.100, 0.120, 0.005), (0.120, 0.180, 0.010), (0.180, 0.200, 0.005)]
DENT_HALF = 0.50
LIFT = 0.47

DENS = {"alu-tube": 2700, "alu-cast": 2680, "steel-galv": 7850, "pmma-opal": 1190, "concrete": 2400, "sand-fill": 1700}

MATERIALS = [
    {"id": "alu-tube", "title": T("Extruded aluminium tube EN AW-6060, weathered anthracite powder coat", "Strangpressrohr EN AW-6060, verwitterte anthrazitfarbene Pulverbeschichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.88, "densityKgM3": 2700, "conductivityWmK": 200, "specificHeatJKgK": 900, "elasticModulusPa": 69e9, "strengthPa": 150e6, "source": "literature", "reference": "en-755-2"},
    {"id": "alu-cast", "title": T("Cast aluminium EN AC-AlSi10Mg, chalked powder coat", "Aluminiumguss EN AC-AlSi10Mg, gekreidete Pulverbeschichtung"), "emissivity": 0.9, "solarAbsorptivity": 0.86, "densityKgM3": 2680, "conductivityWmK": 150, "specificHeatJKgK": 900, "elasticModulusPa": 72e9, "strengthPa": 140e6, "source": "literature", "reference": "en-1706"},
    {"id": "steel-galv", "title": T("Hot-dip galvanised steel with dull zinc patina and red rust spots", "Feuerverzinkter Stahl mit stumpfer Zinkpatina und Rostpunkten"), "emissivity": 0.6, "solarAbsorptivity": 0.75, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 400e6, "source": "authored-estimate", "reference": "incropera-2018"},
    {"id": "pmma-opal", "title": T("Opal PMMA diffuser, yellowed by UV", "Opale PMMA-Abdeckung, durch UV vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": 1190, "conductivityWmK": 0.19, "specificHeatJKgK": 1470, "source": "authored-estimate", "reference": "iso-10456-2007"},
    {"id": "concrete", "title": T("Precast concrete pipe DN 400 (socket)", "Betonfertigrohr DN 400 (Köcher)"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "densityKgM3": 2400, "conductivityWmK": 1.65, "specificHeatJKgK": 1000, "elasticModulusPa": 30e9, "strengthPa": 30e6, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "sand-fill", "title": T("Compacted sand-gravel fill, top dressed with path gravel", "Verdichtete Sand-Kies-Füllung, oben mit Wegedecke abgestreut"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "densityKgM3": 1700, "conductivityWmK": 2.0, "specificHeatJKgK": 910, "source": "literature", "reference": "iso-10456-2007"},
]


def area_centroid(poly):
    """📐️ Signed area and centroid of a polygon."""
    a = cx = cy = 0.0
    n = len(poly)
    for i in range(n):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return a, cx / (6 * a), cy / (6 * a)


def closed_profile(profile):
    """🔁️ The polygon a revolve profile encloses (closed loop, or closed through the axis)."""
    if profile[0] == profile[-1]:
        return [tuple(p) for p in profile[:-1]]
    p = [tuple(q) for q in profile]
    if p[-1][1] != 0:
        p.append((p[-1][0], 0.0))
    if p[0][1] != 0:
        p.append((p[0][0], 0.0))
    return p


def revolve_volume(profile, sweep=2 * PI):
    """🌀️ Volume and axial centroid of a revolved profile (Pappus)."""
    a, ca, cr = area_centroid(closed_profile(profile))
    return abs(a) * cr * sweep, ca


def hexagon(cv, cw, rad, phase=0.0):
    """⬡️ A hexagon in the (v, w) plane."""
    return [(cv + rad * math.cos(phase + k * PI / 3), cw + rad * math.sin(phase + k * PI / 3)) for k in range(6)]


def rot(p, ang):
    """🔄️ Rotates a (v, w) point about the bone axis."""
    v, w = p
    return (v * math.cos(ang) - w * math.sin(ang), v * math.sin(ang) + w * math.cos(ang))


def dent_ring(radius, depth, n=96):
    """🥊️ A tube cross-section in (v, w) with the mower dent centred towards world +x (v negative)."""
    out = []
    for k in range(n):
        th = 2 * PI * k / n
        d = abs(((th - PI) + PI) % (2 * PI) - PI)
        r = radius - (depth * math.cos(PI * d / (2 * DENT_HALF)) ** 2 if d < DENT_HALF else 0.0)
        out.append((r * math.cos(th), r * math.sin(th)))
    return out


def dent_reach(depth):
    """📏️ How far the dented section reaches towards world +x (the silhouette of the elevation)."""
    return max(-v for v, w in dent_ring(R_TUBE, depth, 720))


def rev(id, bone, material, role, tone, profile, origin=(0, 0, 0), **extra):
    """🌀️ A revolve solid about the bone axis u."""
    s = {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts(profile), "axis": [1, 0, 0], "origin": [r6(c) for c in origin]}
    s.update(extra)
    return s


def ring_loop(u0, u1, ri, ro):
    """⭕️ A closed-loop profile of a ring or tube."""
    return [(u0, ri), (u1, ri), (u1, ro), (u0, ro), (u0, ri)]


def ext_yz(id, bone, material, role, tone, profile, u0, u1, holes=None, **extra):
    """🧱️ A prism of a (v, w) profile along u."""
    s = {"id": id, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": pts(profile), "plane": "yz", "from": r6(u0), "to": r6(u1)}
    if holes:
        s["holes"] = [pts(h) for h in holes]
    s.update(extra)
    return s


def box(id, bone, material, role, tone, size, at, **extra):
    """📦️ A box."""
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r6(c) for c in size], "transform": {"translation": [r6(c) for c in at]}}
    s.update(extra)
    return s


def ell(id, bone, material, radius, at, **extra):
    """⚫️ A mass marker."""
    s = {"id": id, "kind": "ellipsoid", "bone": bone, "material": material, "role": "body", "tone": 0.5, "radii": [radius] * 3, "transform": {"translation": [r6(c) for c in at]}}
    s.update(extra)
    return s


def repeat_about_u(count):
    """♻️ Copies about the bone axis."""
    return {"count": count, "rotationAxis": [1, 0, 0], "rotationRad": r6(2 * PI / count), "origin": [0, 0, 0]}


def solid_volume(s):
    """⚖️ Volume and axial centroid of a body solid (all copies)."""
    n = s.get("repeat", {}).get("count", 1)
    if s["kind"] == "revolve":
        v, c = revolve_volume(s["profile"], s.get("sweepRad", 2 * PI))
        return v * n, c + s["origin"][0]
    if s["kind"] == "extrude":
        a = abs(area_centroid(s["profile"])[0]) - sum(abs(area_centroid(h)[0]) for h in s.get("holes", []))
        return a * (s["to"] - s["from"]) * n, (s["to"] + s["from"]) / 2
    if s["kind"] == "box":
        x, y, z = s["size"]
        return x * y * z * n, s["transform"]["translation"][0]
    raise ValueError(s["kind"])


NUT_PHASE = PI / 12
NUT0 = (R_BOLT * math.cos(PI / 4), R_BOLT * math.sin(PI / 4))
NUT_HEX = hexagon(NUT0[0], NUT0[1], NUT_R, NUT_PHASE)
ROD_O = NUT0
RIB_AT = (0.5 * (DIFF_U0 + DIFF_U1), 0.0, 0.5 * (RIB_IN + RIB_OUT))


def nut_copies():
    """⬡️ The four nut hexagons in (v, w)."""
    return [[rot(p, k * PI / 2) for p in NUT_HEX] for k in range(4)]


def rib_copies():
    """🦴️ The rectangles of the three ribs in (v, w)."""
    base = [(-RIB_T / 2, RIB_IN), (RIB_T / 2, RIB_IN), (RIB_T / 2, RIB_OUT), (-RIB_T / 2, RIB_OUT)]
    return [[rot(p, k * 2 * PI / 3) for p in base] for k in range(3)]


def louvre_loop(uc, inner=LOUVRE_IN):
    """🪟️ The section of one conical louvre ring."""
    return [(uc - 0.015, LOUVRE_OUT), (uc - 0.012, LOUVRE_OUT), (uc, inner), (uc - 0.003, inner), (uc - 0.015, LOUVRE_OUT)]


def body50():
    """🔍️ The 1:50 body: every assembly of the real bollard, cage and socket."""
    b = []
    b.append(rev("socket", "footing", "concrete", "body", 0.65, ring_loop(0.0, H_SOCKET, R_SOCKET_IN, R_SOCKET_OUT)))
    b.append(rev("sand-fill", "footing", "sand-fill", "body", 0.75, [(0.0, 0.0), (0.0, R_SOCKET_IN), (H_SOCKET, R_SOCKET_IN), (H_SOCKET, 0.0)]))
    b.append(rev("cage-rods", "cage", "steel-galv", "body", 0.3, [(0.0, 0.0), (0.0, ROD_R), (ROD_LEN, ROD_R), (ROD_LEN, 0.0)], (0, ROD_O[0], ROD_O[1]), slender=True, repeat=repeat_about_u(4)))
    b.append(rev("cage-ring-bottom", "cage", "steel-galv", "body", 0.3, ring_loop(0.020, 0.028, RING_IN, RING_OUT)))
    b.append(rev("cage-ring-top", "cage", "steel-galv", "body", 0.3, ring_loop(0.352, 0.360, RING_IN, RING_OUT)))
    b.append(rev("flange", "bollard", "alu-cast", "body", 0.3, [(0.0, 0.0), (0.0, R_FLANGE), (0.009, R_FLANGE), (H_FLANGE, R_FLANGE - 0.003), (H_FLANGE, 0.0)]))
    b.append(rev("collar", "bollard", "alu-cast", "body", 0.3, [(H_FLANGE, 0.100), (H_FLANGE, 0.110), (0.032, 0.110), (0.040, 0.102), (0.040, 0.100), (H_FLANGE, 0.100)]))
    b.append(ext_yz("anchor-nut", "bollard", "steel-galv", "detail", 0.2, NUT_HEX, H_FLANGE, NUT_TOP, repeat=repeat_about_u(4)))
    b.append(rev("tube-foot", "bollard", "alu-tube", "body", 0.35, ring_loop(H_FLANGE, 0.100, R_TUBE - T_TUBE, R_TUBE)))
    for k, (u0, u1, d) in enumerate(DENT_SEGMENTS):
        b.append(ext_yz(f"tube-dent-{k}", "bollard", "alu-tube", "body", 0.35, dent_ring(R_TUBE, d), u0, u1, holes=[dent_ring(R_TUBE - T_TUBE, d)]))
    b.append(rev("tube", "bollard", "alu-tube", "body", 0.35, ring_loop(0.200, 0.700, R_TUBE - T_TUBE, R_TUBE)))
    b.append(rev("base-ring", "head", "alu-cast", "body", 0.3, BASE_PROFILE))
    b.append(rev("diffuser", "head", "pmma-opal", "glazing", 0.85, ring_loop(DIFF_U0, DIFF_U1, DIFF_IN, DIFF_OUT)))
    b.append(box("rib", "head", "alu-cast", "detail", 0.3, (DIFF_U1 - DIFF_U0, RIB_T, RIB_OUT - RIB_IN), RIB_AT, repeat=repeat_about_u(3)))
    b.append(rev("louvre", "head", "alu-cast", "body", 0.3, louvre_loop(LOUVRES[0]), slender=True, repeat={"count": 4, "translation": [0.03, 0, 0]}))
    b.append(rev("cap", "head", "alu-cast", "body", 0.3, CAP_PROFILE))
    b.append(rev("led-board", "head", "alu-cast", "detail", 0.2, [(0.146, 0.0), (0.146, 0.070), (0.156, 0.070), (0.156, 0.0)]))
    return b


def masses():
    """⚖️ Bone masses and centres from the 1:50 body solids, section x density (footing excluded: it stays in the ground)."""
    out = {}
    for s in body50():
        if s["bone"] == "footing":
            continue
        v, c = solid_volume(s)
        m = v * DENS[s["material"]]
        kg, mom = out.get(s["bone"], (0.0, 0.0))
        out[s["bone"]] = (kg + m, mom + m * c)
    return {b: (round(kg, 3), round(mom / kg, 4)) for b, (kg, mom) in out.items()}


def footing_mass():
    """🪨️ Mass of socket and fill left in the ground."""
    return sum(solid_volume(s)[0] * DENS[s["material"]] for s in body50() if s["bone"] == "footing")


MASS = masses()
SCALES = ["50", "100", "200", "500"]
REF = {
    "en-60598-2-3": "EN 60598-1:2015 and EN 60598-2-3:2003+A1:2011 Luminaires - general requirements and particular requirements for road and street lighting luminaires (bollard luminaires for paths, IP 65, class I or II).",
    "en-62262": "EN 62262:2002 Degrees of protection provided by enclosures for electrical equipment against external mechanical impacts (IK code; IK08 = 5 J, IK10 = 20 J impact energy, the class of public bollard luminaires).",
    "en-13201-2": "EN 13201-2:2015 Road lighting, performance requirements (classes P for footpaths and park paths; bollard heights 0.6 to 1.1 m serve P4 to P6).",
    "en-755-2": "EN 755-2:2016 Aluminium and aluminium alloys - extruded rod/bar, tube and profiles - mechanical properties (EN AW-6060 T66: Rp0.2 150 MPa; density 2700 kg/m3, E 69 GPa, conductivity about 200 W/mK).",
    "en-1706": "EN 1706:2020 Aluminium and aluminium alloys - castings - chemical composition and mechanical properties (EN AC-AlSi10Mg: density 2680 kg/m3, E about 72 GPa).",
    "en-1916": "EN 1916:2002 Concrete pipes and fittings, unreinforced, steel fibre and reinforced (DN 400: wall about 50 mm, outer diameter about 0.50 m).",
    "iso-4032": "ISO 4032:2012 Hexagon regular nuts, style 1 (M12: width across flats 18 mm, height 10.8 mm); ISO 7089 plain washers (M12: 24 x 2.5 mm); ISO 898-1 property class 5.6 rods, stress area 84.3 mm2.",
    "en-1991-1-4": "EN 1991-1-4:2005 + NA Germany: wind actions (peak velocity pressure about 0.5 kN/m2 at 1 m in a sheltered park, terrain category III to IV, force coefficient of a cylinder about 1.2).",
    "en-1991-1-1": "EN 1991-1-1:2002 Densities, self-weight, imposed loads (concentrated and line loads of persons on street furniture, used here as the basis of the leaning load).",
    "iso-10456-2007": "ISO 10456:2007 Building materials and products - hygrothermal properties - tabulated design values (concrete 2400 kg/m3, 1.65 W/mK, 1000 J/kgK; sand and gravel 1700 to 2200 kg/m3, 2.0 W/mK, 910 J/kgK; PMMA 1190 kg/m3, 0.19 W/mK).",
    "incropera-2018": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity and solar absorptance of painted, weathered galvanised and oxidised metal surfaces).",
    "schubert-2006": "Schubert, E. F. (2006). Light-Emitting Diodes, 2nd ed., Cambridge University Press (the electrical input of a white LED module not emitted as light, about 70 percent at 2000s retrofit efficacies, leaves as heat through the heat spreader and the housing).",
    "brandt-2008": "Brandt, J., Kuhn, B., et al. (2008). Witterungsbedingte Alterung von Pulverlacken auf Aluminium (chalking of polyester powder coats after 10 to 20 years of outdoor exposure: gloss loss above 50 percent, solar absorptance of dark tones about 0.02 to 0.05 lower than new); used as a qualitative source for the weathered coat.",
    "bollard-catalogue-typical": "Catalogue-typical data of German municipal bollard luminaires with louvre head of the 1990s and 2000s (no brand): extruded aluminium tube 200 mm x 3 mm, 0.8 to 1.0 m above ground, louvre head with four conical anti-glare rings around an opal diffuser, cast cap, base flange on an anchor cage of 4 x M12 on a 250 mm bolt circle, 400 to 450 mm long, luminaire mass 8 to 12 kg, originally 18 to 26 W compact fluorescent, retrofitted with LED modules of 10 to 16 W.",
}


def shape(kind, id, bone, role, z, tone, region=None, material=None, filled=True, **geom):
    """✏️ A 2D shape."""
    s = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    if isinstance(region, list):
        s["regions"] = region
    elif region:
        s["region"] = region
    if material:
        s["material"] = material
    s["kind"] = kind
    for k, v in geom.items():
        if k == "points":
            s[k] = pts(v)
        elif isinstance(v, float):
            s[k] = r6(v)
        else:
            s[k] = v
    return s


def P(id, bone, role, z, tone, region, material, points, closed=True, filled=True, **kw):
    """🔺️ A polygon shape."""
    return shape("poly", id, bone, role, z, tone, region, material, filled, points=points, closed=closed, **kw)


def R(id, bone, role, z, tone, region, material, x, y, w, h, filled=True, **kw):
    """▭️ A rectangle shape."""
    return shape("rect", id, bone, role, z, tone, region, material, filled, x=x, y=y, w=w, h=h, **kw)


def E(id, bone, role, z, tone, region, material, cx, cy, rad, filled=True, **kw):
    """⚪️ A circle shape."""
    return shape("ellipse", id, bone, role, z, tone, region, material, filled, cx=cx, cy=cy, rx=rad, ry=rad, **kw)


ORIGIN = {"footing": Y_FOOT, "cage": Y_CAGE, "bollard": 0.0, "head": Y_HEAD}


def sil(outline):
    """🪞️ A mirrored side silhouette from an outer edge (u, r) bottom to top."""
    return [(u, -r) for u, r in outline] + [(u, r) for u, r in reversed(outline)]


class Views:
    """🖼️ Collects the elevation, front and plan shapes of one scale."""

    def __init__(self):
        self.e, self.f, self.p = [], [], []

    def axisym(self, id, bone, role, z, tone, region, material, outline, plan=True, plan_z=None, hatch=None, pen=None):
        """🌀️ An axisymmetric part in all three views from its outer edge."""
        extra = {}
        if hatch:
            extra["hatch"] = hatch
        if pen:
            extra["penMm"] = pen
        y0 = ORIGIN[bone]
        self.e.append(P(id, bone, role, z, tone, region, material, sil(outline), **extra))
        self.f.append(P(id, bone, role, z, tone, region, material, [(-r, y0 + u) for u, r in outline] + [(r, y0 + u) for u, r in reversed(outline)], **extra))
        if plan:
            self.p.append(E(id, bone, role, plan_z if plan_z is not None else z, tone, region, material, 0.0, 0.0, max(r for _, r in outline), **extra))


def nut_extents(axis):
    """📐️ Union of the projected nut extents along v (axis 0) or w (axis 1), per side."""
    lo, hi = [], []
    for hexp in nut_copies():
        vals = [p[axis] for p in hexp]
        (hi if sum(vals) > 0 else lo).append((min(vals), max(vals)))
    return (min(a for a, _ in lo), max(b for _, b in lo)), (min(a for a, _ in hi), max(b for _, b in hi))


def rib_extent_v():
    """📐️ The v range of the two ribs nearer to the elevation viewer (w < 0)."""
    out = []
    for rib in rib_copies():
        if sum(p[1] for p in rib) < 0:
            vs = [p[0] for p in rib]
            out.append((min(vs), max(vs)))
    return sorted(out)


def rib_front():
    """📐️ The w ranges of the rib at the side (v = 0) and of the rib facing ahead (v < 0, world x > 0)."""
    side = front = None
    for rib in rib_copies():
        vs = [p[0] for p in rib]
        ws = [p[1] for p in rib]
        if abs(sum(vs)) < 1e-9:
            side = (min(ws), max(ws))
        elif sum(vs) < 0:
            front = (min(ws), max(ws))
    return side, front


def views50():
    """🔍️ The 2D drawings of 1:50."""
    v = Views()
    v.axisym("socket", "footing", "body", 1.0, 0.65, ["socket-concrete", "sand-fill"], "concrete", [(0.0, R_SOCKET_OUT), (H_SOCKET, R_SOCKET_OUT)], plan_z=1.0, hatch="concrete-stipple")
    v.p.append(E("sand-fill", "footing", "body", 1.1, 0.75, "sand-fill", "sand-fill", 0.0, 0.0, R_SOCKET_IN, hatch="sand-stipple"))
    for side, (a, b) in zip(("r", "l"), nut_extents(0)):
        v.e.append(R(f"cage-rod-{side}", "cage", "body", 0.5, 0.3, "anchor-cage", "steel-galv", 0.0, (a + b) / 2 - ROD_R, ROD_LEN, 2 * ROD_R))
    for side, (a, b) in zip(("r", "l"), nut_extents(1)):
        v.f.append(R(f"cage-rod-{side}", "cage", "body", 0.5, 0.3, "anchor-cage", "steel-galv", (a + b) / 2 - ROD_R, Y_CAGE, 2 * ROD_R, ROD_LEN))
    for nm, u0, u1 in (("cage-ring-bottom", 0.020, 0.028), ("cage-ring-top", 0.352, 0.360)):
        v.e.append(R(nm, "cage", "body", 0.6, 0.3, "anchor-cage", "steel-galv", u0, -RING_OUT, u1 - u0, 2 * RING_OUT))
        v.f.append(R(nm, "cage", "body", 0.6, 0.3, "anchor-cage", "steel-galv", -RING_OUT, Y_CAGE + u0, 2 * RING_OUT, u1 - u0))
    v.axisym("flange", "bollard", "body", 2.0, 0.3, "tube-coat", "alu-cast", [(0.0, R_FLANGE), (0.009, R_FLANGE), (H_FLANGE, R_FLANGE - 0.003)], plan_z=2.0)
    v.axisym("collar", "bollard", "body", 2.2, 0.3, "tube-coat", "alu-cast", [(H_FLANGE, 0.110), (0.032, 0.110), (0.040, 0.102)], plan=False)
    for side, (a, b) in zip(("r", "l"), nut_extents(0)):
        v.e.append(R(f"anchor-nut-{side}", "bollard", "detail", 2.3, 0.2, "anchor-cage", "steel-galv", H_FLANGE, a, NUT_TOP - H_FLANGE, b - a, penMm=0.13))
    for side, (a, b) in zip(("r", "l"), nut_extents(1)):
        v.f.append(R(f"anchor-nut-{side}", "bollard", "detail", 2.3, 0.2, "anchor-cage", "steel-galv", a, H_FLANGE, b - a, NUT_TOP - H_FLANGE, penMm=0.13))
    for k, hexp in enumerate(nut_copies()):
        v.p.append(P(f"anchor-nut-{k}", "bollard", "detail", 2.3, 0.2, "anchor-cage", "steel-galv", [(-a, b) for a, b in hexp], penMm=0.13))
        c = (-sum(p[0] for p in hexp) / 6, sum(p[1] for p in hexp) / 6)
        v.p.append(E(f"rod-end-{k}", "cage", "detail", 2.4, 0.35, "anchor-cage", "steel-galv", c[0], c[1], ROD_R, penMm=0.1))
    reach = [(u0, u1, dent_reach(d)) for u0, u1, d in DENT_SEGMENTS]
    tube_e = [(H_FLANGE, -R_TUBE)] + [q for u0, u1, x in reach for q in ((u0, -x), (u1, -x))] + [(0.700, -R_TUBE), (0.700, R_TUBE), (H_FLANGE, R_TUBE)]
    v.e.append(P("tube", "bollard", "body", 3.0, 0.35, "tube-coat", "alu-tube", tube_e))
    v.f.append(P("tube", "bollard", "body", 3.0, 0.35, "tube-coat", "alu-tube", [(-R_TUBE, H_FLANGE), (-R_TUBE, 0.700), (R_TUBE, 0.700), (R_TUBE, H_FLANGE)]))
    v.e.append(P("dent-crease", "bollard", "detail", 3.2, 0.25, "tube-coat", "alu-tube", [(0.104, -0.094), (0.15, -0.088), (0.196, -0.094)], closed=False, filled=False, penMm=0.13))
    v.f.append(P("dent-crease-top", "bollard", "detail", 3.2, 0.25, "tube-coat", "alu-tube", [(-0.047, 0.12), (-0.02, 0.124), (0.02, 0.124), (0.047, 0.12)], closed=False, filled=False, penMm=0.13))
    v.f.append(P("dent-crease-bottom", "bollard", "detail", 3.2, 0.25, "tube-coat", "alu-tube", [(-0.047, 0.18), (-0.02, 0.176), (0.02, 0.176), (0.047, 0.18)], closed=False, filled=False, penMm=0.13))
    v.f.append(P("dent-floor", "bollard", "texture", 3.3, 0.25, "tube-coat", "alu-tube", [(-0.03, 0.15), (0.03, 0.15)], closed=False, filled=False, penMm=0.13))
    v.e.append(P("sticker-residue", "bollard", "texture", 3.4, 0.6, "tube-coat", "alu-tube", [(0.45, -0.035), (0.53, -0.03), (0.528, 0.05), (0.452, 0.046)], filled=False, penMm=0.13))
    v.e.append(P("sticker-tear", "bollard", "texture", 3.4, 0.6, "tube-coat", "alu-tube", [(0.47, -0.02), (0.49, 0.0), (0.475, 0.025)], closed=False, filled=False, penMm=0.13))
    for k, (a, b) in enumerate([((0.30, -0.06), (0.34, -0.03)), ((0.36, 0.02), (0.41, 0.05)), ((0.58, -0.07), (0.6, -0.04))]):
        v.e.append(P(f"scratch-{k}", "bollard", "texture", 3.5, 0.5, "tube-coat", "alu-tube", [a, b], closed=False, filled=False, penMm=0.13))
    for k, (a, b) in enumerate([((-0.06, 0.32), (-0.02, 0.35)), ((0.04, 0.55), (0.07, 0.58))]):
        v.f.append(P(f"scratch-{k}", "bollard", "texture", 3.5, 0.5, "tube-coat", "alu-tube", [a, b], closed=False, filled=False, penMm=0.13))
    v.e.append(P("coat-flaking", "bollard", "texture", 3.6, 0.6, "tube-coat", "alu-tube", [(0.045, -0.1), (0.052, -0.07), (0.047, -0.03), (0.055, 0.02), (0.049, 0.06), (0.054, 0.1)], closed=False, filled=False, penMm=0.13))
    v.f.append(P("coat-flaking", "bollard", "texture", 3.6, 0.6, "tube-coat", "alu-tube", [(-0.1, 0.05), (-0.07, 0.057), (-0.03, 0.05), (0.02, 0.058), (0.06, 0.051), (0.1, 0.056)], closed=False, filled=False, penMm=0.13))
    v.axisym("base-ring", "head", "body", 4.0, 0.3, "head-housing", "alu-cast", [(0.0, 0.104), (0.014, 0.104)], plan=False)
    v.axisym("diffuser", "head", "glazing", 4.1, 0.85, "diffuser", "pmma-opal", [(DIFF_U0, DIFF_OUT), (DIFF_U1, DIFF_OUT)], plan=False)
    for k, (a, b) in enumerate(rib_extent_v()):
        v.e.append(R(f"rib-{k}", "head", "detail", 4.2, 0.3, "head-housing", "alu-cast", DIFF_U0, a, DIFF_U1 - DIFF_U0, b - a))
    side, front = rib_front()
    v.f.append(R("rib-side", "head", "detail", 4.2, 0.3, "head-housing", "alu-cast", side[0], Y_HEAD + DIFF_U0, side[1] - side[0], DIFF_U1 - DIFF_U0))
    v.f.append(R("rib-front", "head", "detail", 4.25, 0.3, "head-housing", "alu-cast", front[0], Y_HEAD + DIFF_U0, front[1] - front[0], DIFF_U1 - DIFF_U0))
    for k, uc in enumerate(LOUVRES):
        o = [(uc - 0.015, LOUVRE_OUT), (uc - 0.012, LOUVRE_OUT), (uc, LOUVRE_IN)]
        v.axisym(f"louvre-{k}", "head", "body", 4.3, 0.3, "head-housing", "alu-cast", o, plan=False)
    v.axisym("cap", "head", "body", 4.5, 0.3, "head-housing", "alu-cast", CAP_OUTLINE, plan_z=5.0)
    v.e.append(P("cap-chalking", "head", "texture", 4.6, 0.6, "head-housing", "alu-cast", [(0.19, -0.07), (0.196, -0.04), (0.199, 0.0)], closed=False, filled=False, penMm=0.13))
    v.p.append(E("cap-crown", "head", "detail", 5.1, 0.4, "head-housing", "alu-cast", 0.0, 0.0, 0.06, filled=False, penMm=0.13))
    v.p.append(P("cap-lichen", "head", "texture", 5.2, 0.5, "head-housing", "alu-cast", [(-0.07, 0.03), (-0.05, 0.06), (-0.02, 0.07)], closed=False, filled=False, penMm=0.13))
    return v


def views100():
    """🔍️ The 2D drawings of 1:100."""
    v = Views()
    v.axisym("socket", "footing", "body", 1.0, 0.65, ["socket-concrete", "sand-fill"], "concrete", [(0.0, R_SOCKET_OUT), (H_SOCKET, R_SOCKET_OUT)], hatch="concrete-stipple")
    v.p.append(E("sand-fill", "footing", "body", 1.1, 0.75, "sand-fill", "sand-fill", 0.0, 0.0, R_SOCKET_IN))
    for side, (a, b) in zip(("r", "l"), nut_extents(0)):
        v.e.append(R(f"cage-rod-{side}", "cage", "body", 0.5, 0.3, "anchor-cage", "steel-galv", 0.0, (a + b) / 2 - ROD_R, ROD_LEN, 2 * ROD_R))
    for side, (a, b) in zip(("r", "l"), nut_extents(1)):
        v.f.append(R(f"cage-rod-{side}", "cage", "body", 0.5, 0.3, "anchor-cage", "steel-galv", (a + b) / 2 - ROD_R, Y_CAGE, 2 * ROD_R, ROD_LEN))
    for nm, u0, u1 in (("cage-ring-bottom", 0.016, 0.031), ("cage-ring-top", 0.345, 0.360)):
        v.e.append(R(nm, "cage", "body", 0.6, 0.3, "anchor-cage", "steel-galv", u0, -RING_OUT, u1 - u0, 2 * RING_OUT))
        v.f.append(R(nm, "cage", "body", 0.6, 0.3, "anchor-cage", "steel-galv", -RING_OUT, Y_CAGE + u0, 2 * RING_OUT, u1 - u0))
    v.axisym("flange", "bollard", "body", 2.0, 0.3, "tube-coat", "alu-cast", [(0.0, R_FLANGE), (H_FLANGE, R_FLANGE)])
    for side, (a, b) in zip(("r", "l"), nut_extents(0)):
        v.e.append(R(f"anchor-nut-{side}", "bollard", "detail", 2.3, 0.2, "anchor-cage", "steel-galv", H_FLANGE, a, 0.032 - H_FLANGE, b - a))
    for side, (a, b) in zip(("r", "l"), nut_extents(1)):
        v.f.append(R(f"anchor-nut-{side}", "bollard", "detail", 2.3, 0.2, "anchor-cage", "steel-galv", a, H_FLANGE, b - a, 0.032 - H_FLANGE))
    for k, hexp in enumerate(nut_copies()):
        v.p.append(P(f"anchor-nut-{k}", "bollard", "detail", 2.3, 0.2, "anchor-cage", "steel-galv", [(-a, b) for a, b in hexp]))
    v.axisym("tube", "bollard", "body", 3.0, 0.35, "tube-coat", "alu-tube", [(H_FLANGE, 0.110), (0.040, 0.110), (0.040, R_TUBE), (0.700, R_TUBE)], plan=False)
    v.e.append(P("dent", "bollard", "texture", 3.2, 0.25, "tube-coat", "alu-tube", [(0.11, -0.092), (0.15, -0.086), (0.19, -0.092)], closed=False, filled=False))
    v.f.append(P("dent", "bollard", "texture", 3.2, 0.25, "tube-coat", "alu-tube", [(-0.045, 0.12), (0.0, 0.126), (0.045, 0.12)], closed=False, filled=False))
    v.e.append(P("scratch", "bollard", "texture", 3.5, 0.5, "tube-coat", "alu-tube", [(0.30, -0.06), (0.36, -0.02)], closed=False, filled=False))
    v.axisym("base-ring", "head", "body", 4.0, 0.3, "head-housing", "alu-cast", [(0.0, 0.104), (0.015, 0.104)], plan=False)
    v.axisym("diffuser", "head", "glazing", 4.1, 0.85, "diffuser", "pmma-opal", [(0.015, DIFF_OUT), (DIFF_U1, DIFF_OUT)], plan=False)
    for k, uc in enumerate(LOUVRES):
        v.axisym(f"louvre-{k}", "head", "body", 4.3, 0.3, "head-housing", "alu-cast", [(uc - 0.015, LOUVRE_OUT), (uc - 0.012, LOUVRE_OUT), (uc, DIFF_OUT)], plan=False)
    v.axisym("cap", "head", "body", 4.5, 0.3, "head-housing", "alu-cast", CAP100[1:], plan_z=5.0)
    return v


CAP100 = [(0.160, 0.0), (0.160, 0.110), (0.172, 0.110), (0.186, 0.098), (0.196, 0.070), (0.200, 0.0)]
CAP200 = [(0.152, 0.0), (0.152, 0.110), (0.176, 0.108), (0.192, 0.086), (0.200, 0.0)]


def views200():
    """🔍️ The 2D drawings of 1:200."""
    v = Views()
    v.axisym("socket", "footing", "body", 1.0, 0.65, ["socket-concrete", "sand-fill"], "concrete", [(0.0, R_SOCKET_OUT), (H_SOCKET, R_SOCKET_OUT)])
    v.p.append(E("sand-fill", "footing", "body", 1.1, 0.75, "sand-fill", "sand-fill", 0.0, 0.0, R_SOCKET_IN))
    for side, (a, b) in zip(("r", "l"), nut_extents(0)):
        c = (a + b) / 2
        v.e.append(P(f"cage-rod-{side}", "cage", "detail", 0.5, 0.3, "anchor-cage", "steel-galv", [(0.0, c), (0.432, c)], closed=False, filled=False))
    for side, (a, b) in zip(("r", "l"), nut_extents(1)):
        c = (a + b) / 2
        v.f.append(P(f"cage-rod-{side}", "cage", "detail", 0.5, 0.3, "anchor-cage", "steel-galv", [(c, Y_CAGE), (c, Y_CAGE + 0.432)], closed=False, filled=False))
    v.axisym("flange", "bollard", "body", 2.0, 0.3, "tube-coat", "alu-cast", [(0.0, R_FLANGE), (H_FLANGE, R_FLANGE)])
    v.axisym("tube", "bollard", "body", 3.0, 0.35, "tube-coat", "alu-tube", [(H_FLANGE, R_TUBE), (0.700, R_TUBE)], plan=False)
    v.axisym("louvre-band", "head", "body", 4.0, 0.4, ["head-housing", "diffuser"], "alu-cast", [(0.0, LOUVRE_OUT), (0.152, LOUVRE_OUT)], plan=False)
    for k, uc in enumerate((0.065, 0.125)):
        v.e.append(P(f"louvre-line-{k}", "head", "detail", 4.1, 0.3, "head-housing", "alu-cast", [(uc, -LOUVRE_OUT), (uc, LOUVRE_OUT)], closed=False, filled=False))
        v.f.append(P(f"louvre-line-{k}", "head", "detail", 4.1, 0.3, "head-housing", "alu-cast", [(-LOUVRE_OUT, Y_HEAD + uc), (LOUVRE_OUT, Y_HEAD + uc)], closed=False, filled=False))
    v.axisym("cap", "head", "body", 4.5, 0.3, "head-housing", "alu-cast", CAP200[1:], plan_z=5.0)
    return v


def views500():
    """🔍️ The 2D drawings of 1:500."""
    v = Views()
    v.axisym("socket", "footing", "body", 1.0, 0.65, ["socket-concrete", "sand-fill"], "concrete", [(0.0, R_SOCKET_OUT), (H_SOCKET, R_SOCKET_OUT)])
    v.p.append(E("sand-fill", "footing", "body", 1.1, 0.75, "sand-fill", "sand-fill", 0.0, 0.0, R_SOCKET_IN))
    for side, (a, b) in zip(("r", "l"), nut_extents(0)):
        c = (a + b) / 2
        v.e.append(P(f"cage-rod-{side}", "cage", "body", 0.5, 0.3, "anchor-cage", "steel-galv", [(0.0, c), (0.432, c)], closed=False, filled=False, penMm=0.1))
    for side, (a, b) in zip(("r", "l"), nut_extents(1)):
        c = (a + b) / 2
        v.f.append(P(f"cage-rod-{side}", "cage", "body", 0.5, 0.3, "anchor-cage", "steel-galv", [(c, Y_CAGE), (c, Y_CAGE + 0.432)], closed=False, filled=False, penMm=0.1))
    v.axisym("flange", "bollard", "body", 2.0, 0.3, "tube-coat", "alu-cast", [(0.0, R_FLANGE), (H_FLANGE, R_FLANGE)])
    v.axisym("tube", "bollard", "body", 3.0, 0.35, "tube-coat", "alu-tube", [(H_FLANGE, R_TUBE), (0.700, R_TUBE)], plan=False)
    v.axisym("head", "head", "body", 4.0, 0.35, ["diffuser", "head-housing"], "alu-cast", [(0.0, 0.110), (0.200, 0.110)], plan_z=5.0)
    return v


def body100():
    """🔍️ The 1:100 body."""
    b = []
    b.append(rev("socket", "footing", "concrete", "body", 0.65, ring_loop(0.0, H_SOCKET, R_SOCKET_IN, R_SOCKET_OUT)))
    b.append(rev("sand-fill", "footing", "sand-fill", "body", 0.75, [(0.0, 0.0), (0.0, R_SOCKET_IN), (H_SOCKET, R_SOCKET_IN), (H_SOCKET, 0.0)]))
    b.append(rev("cage-rods", "cage", "steel-galv", "body", 0.3, [(0.0, 0.0), (0.0, ROD_R), (ROD_LEN, ROD_R), (ROD_LEN, 0.0)], (0, ROD_O[0], ROD_O[1]), slender=True, repeat=repeat_about_u(4)))
    b.append(rev("cage-rings", "cage", "steel-galv", "body", 0.3, ring_loop(0.016, 0.031, RING_IN, RING_OUT), repeat={"count": 2, "translation": [0.329, 0, 0]}))
    b.append(rev("flange", "bollard", "alu-cast", "body", 0.3, [(0.0, 0.0), (0.0, R_FLANGE), (H_FLANGE, R_FLANGE), (H_FLANGE, 0.0)], sheet=True))
    b.append(ext_yz("anchor-nut", "bollard", "steel-galv", "detail", 0.2, NUT_HEX, H_FLANGE, 0.032, repeat=repeat_about_u(4)))
    b.append(rev("tube", "bollard", "alu-tube", "body", 0.35, [(H_FLANGE, R_TUBE - T_TUBE), (0.700, R_TUBE - T_TUBE), (0.700, R_TUBE), (0.040, R_TUBE), (0.040, 0.110), (H_FLANGE, 0.110), (H_FLANGE, R_TUBE - T_TUBE)]))
    b.append(rev("base-ring", "head", "alu-cast", "body", 0.3, [(0.0, 0.0), (0.0, 0.104), (0.015, 0.104), (0.015, 0.0)]))
    b.append(rev("diffuser", "head", "pmma-opal", "glazing", 0.85, ring_loop(0.015, DIFF_U1, DIFF_IN, DIFF_OUT)))
    b.append(rev("louvre", "head", "alu-cast", "body", 0.3, louvre_loop(LOUVRES[0], DIFF_OUT), slender=True, repeat={"count": 4, "translation": [0.03, 0, 0]}))
    b.append(rev("cap", "head", "alu-cast", "body", 0.3, CAP100))
    return b


def body200():
    """🔍️ The 1:200 body."""
    b = []
    b.append(rev("socket", "footing", "concrete", "body", 0.65, ring_loop(0.0, H_SOCKET, R_SOCKET_IN, R_SOCKET_OUT)))
    b.append(rev("sand-fill", "footing", "sand-fill", "body", 0.75, [(0.0, 0.0), (0.0, R_SOCKET_IN), (H_SOCKET, R_SOCKET_IN), (H_SOCKET, 0.0)]))
    b.append(rev("cage-rods", "cage", "steel-galv", "body", 0.3, [(0.0, 0.0), (0.0, ROD_R), (0.432, ROD_R), (0.432, 0.0)], (0, ROD_O[0], ROD_O[1]), slender=True, repeat=repeat_about_u(4)))
    b.append(rev("flange", "bollard", "alu-cast", "body", 0.3, [(0.0, 0.0), (0.0, R_FLANGE), (H_FLANGE, R_FLANGE), (H_FLANGE, 0.0)], sheet=True))
    b.append(rev("tube", "bollard", "alu-tube", "body", 0.35, ring_loop(H_FLANGE, 0.700, R_TUBE - T_TUBE, R_TUBE)))
    b.append(rev("louvre-band", "head", "alu-cast", "body", 0.4, [(0.0, 0.0), (0.0, LOUVRE_OUT), (0.152, LOUVRE_OUT), (0.152, 0.0)]))
    b.append(rev("cap", "head", "alu-cast", "body", 0.3, CAP200))
    return b


def body500():
    """🔍️ The 1:500 body."""
    b = []
    b.append(rev("socket", "footing", "concrete", "body", 0.65, ring_loop(0.0, H_SOCKET, R_SOCKET_IN, R_SOCKET_OUT)))
    b.append(rev("sand-fill", "footing", "sand-fill", "body", 0.75, [(0.0, 0.0), (0.0, R_SOCKET_IN), (H_SOCKET, R_SOCKET_IN), (H_SOCKET, 0.0)]))
    b.append(rev("cage-rods", "cage", "steel-galv", "body", 0.3, [(0.0, 0.0), (0.0, ROD_R), (0.432, ROD_R), (0.432, 0.0)], (0, ROD_O[0], ROD_O[1]), slender=True, repeat=repeat_about_u(4)))
    b.append(rev("flange", "bollard", "alu-cast", "body", 0.3, [(0.0, 0.0), (0.0, R_FLANGE), (H_FLANGE, R_FLANGE), (H_FLANGE, 0.0)], sheet=True))
    b.append(rev("tube", "bollard", "alu-tube", "body", 0.35, ring_loop(H_FLANGE, 0.700, R_TUBE - T_TUBE, R_TUBE)))
    b.append(rev("louvre-band", "head", "pmma-opal", "glazing", 0.4, [(0.0, 0.0), (0.0, LOUVRE_OUT), (0.120, LOUVRE_OUT), (0.120, 0.0)]))
    b.append(rev("cap", "head", "alu-cast", "body", 0.3, [(0.120, 0.0), (0.120, 0.110), (0.200, 0.110), (0.200, 0.0)]))
    return b


REGION_OF = {"socket": "socket-concrete", "sand-fill": "sand-fill", "cage-rods": "anchor-cage", "cage-ring-bottom": "anchor-cage", "cage-ring-top": "anchor-cage", "cage-rings": "anchor-cage", "anchor-nut": "anchor-cage", "flange": "tube-coat", "collar": "tube-coat", "tube-foot": "tube-coat", "tube-dent-0": "tube-coat", "tube-dent-1": "tube-coat", "tube-dent-2": "tube-coat", "tube": "tube-coat", "base-ring": "head-housing", "diffuser": "diffuser", "rib": "head-housing", "louvre": "head-housing", "cap": "head-housing", "led-board": "head-housing", "louvre-band": "head-housing"}


def emitter_patch(scale):
    """♨️ The outlet patch of the plume on the cap crown."""
    t = {"50": 0.008, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    rad = {"50": 0.05, "100": 0.05, "200": 0.06, "500": 0.08}[scale]
    return rev("th-head-plume", "head", "alu-cast", "detail", 0.9, [(0.2015 - t, 0.0), (0.2015 - t, rad), (0.2015, rad), (0.2015, 0.0)], region="head-housing", tags=["head-plume"], texture=True)


def thermal_list(body, scale):
    """🌡️ The thermal list: every body solid on its region, the plume outlet, and at the coarse scales the diffuser shell inside the band."""
    out = []
    for s in body:
        region = REGION_OF[s["id"]]
        if scale == "500" and s["id"] == "louvre-band":
            region = "diffuser"
        out.append({"id": "th-" + s["id"], "of": s["id"], "region": region})
    if scale == "200":
        out.append(rev("th-diffuser", "head", "pmma-opal", "glazing", 0.85, ring_loop(0.014, 0.150, DIFF_IN, DIFF_OUT), region="diffuser"))
    out.append(emitter_patch(scale))
    return out


def structure_list(scale, body):
    """🏗️ The load path of one scale: footing block, rods, flange joint, tube, head ribs, cap, masses."""
    feat = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}[scale]
    mr = {"50": 0.015, "100": 0.02, "200": 0.03, "500": 0.0375}[scale]
    ids = {s["id"] for s in body}
    if scale != "50":
        st = [rev("member-footing", "footing", "concrete", "body", 0.65, [(0.0, 0.0), (0.0, R_SOCKET_OUT), (H_SOCKET, R_SOCKET_OUT), (H_SOCKET, 0.0)], member="footing", part="member")]
    else:
        st = [{"id": "member-footing", "of": "socket", "member": "footing", "part": "member"}, {"id": "joint-fill", "of": "sand-fill", "member": "footing", "part": "joint"}]
    st.append({"id": "member-cage", "of": "cage-rods", "member": "cage", "part": "member"})
    if "cage-ring-top" in ids:
        st.append({"id": "joint-cage-ring-top", "of": "cage-ring-top", "member": "cage", "part": "joint"})
        st.append({"id": "joint-cage-ring-bottom", "of": "cage-ring-bottom", "member": "cage", "part": "joint"})
    if "cage-rings" in ids:
        st.append({"id": "joint-cage-rings", "of": "cage-rings", "member": "cage", "part": "joint"})
    if "flange" in ids:
        st.append({"id": "joint-flange", "of": "flange", "member": "bollard", "part": "joint"})
    else:
        st.append(rev("joint-base", "bollard", "alu-cast", "body", 0.3, [(-feat, 0.0), (-feat, R_FLANGE), (0.0, R_FLANGE), (0.0, 0.0)], member="bollard", part="joint"))
    if "anchor-nut" in ids:
        st.append({"id": "joint-anchor-nut", "of": "anchor-nut", "member": "cage", "part": "joint"})
    if scale == "500":
        st.append({"id": "member-bollard", "of": "tube", "member": "bollard", "part": "member"})
    else:
        u0 = 0.0 if scale == "200" else H_FLANGE
        st.append(rev("member-bollard", "bollard", "alu-tube", "body", 0.35, ring_loop(u0, 0.700, R_TUBE - T_TUBE, R_TUBE), member="bollard", part="member"))
    if scale in ("50", "100"):
        st.append({"id": "joint-base-ring", "of": "base-ring", "member": "head", "part": "joint"})
        if scale == "50":
            st.append({"id": "member-head", "of": "rib", "member": "head", "part": "member"})
        else:
            st.append(box("member-head", "head", "alu-cast", "body", 0.3, (DIFF_U1 - 0.015, RIB_T, RIB_OUT - RIB_IN), (0.5 * (0.015 + DIFF_U1), 0, RIB_AT[2]), repeat=repeat_about_u(3), slender=True, member="head", part="member"))
    elif scale == "200":
        st.append({"id": "member-head", "of": "louvre-band", "member": "head", "part": "member"})
    else:
        st.append(rev("member-head", "head", "alu-cast", "body", 0.35, [(0.0, 0.0), (0.0, 0.110), (0.200, 0.110), (0.200, 0.0)], member="head", part="member"))
    if scale != "500":
        st.append({"id": "joint-cap", "of": "cap", "member": "head", "part": "joint"})
    for bone in ("cage", "bollard", "head"):
        st.append(ell(f"mass-{bone}", bone, "steel-galv" if bone == "cage" else "alu-cast", round(mr, 4), (MASS[bone][1], 0.0, 0.0), member=bone, part="mass"))
    return st


def tube_section():
    """🧮️ Section values of the 200 x 3 tube."""
    ro, ri = R_TUBE, R_TUBE - T_TUBE
    return PI * (ro ** 2 - ri ** 2), PI / 4 * (ro ** 4 - ri ** 4)


def build():
    """🏗️ Assembles the asset document."""
    a_t, i_t = tube_section()
    a_s = PI * (R_SOCKET_OUT ** 2 - R_SOCKET_IN ** 2)
    i_s = PI / 4 * (R_SOCKET_OUT ** 4 - R_SOCKET_IN ** 4)
    a_rod = 84.3e-6
    rig = {"kind": "mechanical", "root": "footing", "bones": [
        {"id": "footing", "parent": None, "title": T("Precast socket footing with sand-gravel fill (stays in the ground)", "Köcherfundament aus Betonfertigrohr mit Sand-Kies-Füllung (bleibt im Boden)"), "x": 0, "y": Y_FOOT, "angleDeg": 90, "lengthM": H_SOCKET, "joint": {"type": "fixed"}, "section": {"areaM2": r6(a_s), "secondMomentM4": r6(i_s), "extremeFibreM": R_SOCKET_OUT, "material": "concrete", "title": T("Concrete pipe DN 400, 500 x 50 mm", "Betonrohr DN 400, 500 x 50 mm")}},
        {"id": "bollard", "parent": "footing", "title": T("Bollard tube with base flange (lifts out)", "Pollerrohr mit Fußflansch (wird herausgehoben)"), "x": H_SOCKET, "y": 0, "angleDeg": 0, "lengthM": 0.70, "joint": {"type": "prismatic", "minM": 0, "maxM": 0.5, "axisDeg": 0}, "mass": {"kg": MASS["bollard"][0], "comX": MASS["bollard"][1], "comY": 0, "distribution": "point", "source": "literature", "reference": "bollard-catalogue-typical"}, "section": {"areaM2": r6(a_t), "secondMomentM4": r6(i_t), "extremeFibreM": R_TUBE, "material": "alu-tube", "title": T("Aluminium tube 200 x 3 mm", "Aluminiumrohr 200 x 3 mm")}},
        {"id": "cage", "parent": "bollard", "title": T("Anchor cage 4 x M12 with two template rings", "Ankerkorb 4 x M12 mit zwei Schablonenringen"), "x": Y_CAGE, "y": 0, "angleDeg": 0, "lengthM": ROD_LEN, "joint": {"type": "fixed"}, "mass": {"kg": MASS["cage"][0], "comX": MASS["cage"][1], "comY": 0, "distribution": "point", "source": "literature", "reference": "iso-4032"}, "section": {"areaM2": r6(4 * a_rod), "secondMomentM4": r6(4 * a_rod * NUT0[0] ** 2), "extremeFibreM": r6(NUT0[0] + ROD_R), "material": "steel-galv", "title": T("Four rods M12, stress area 84.3 mm2 each", "Vier Gewindestangen M12, je 84,3 mm2 Spannungsquerschnitt")}},
        {"id": "head", "parent": "bollard", "title": T("Louvre head: base ring, opal diffuser, three ribs, four louvres, cap", "Lamellenkopf: Grundring, opale Abdeckung, drei Stege, vier Lamellen, Kappe"), "x": Y_HEAD, "y": 0, "angleDeg": 0, "lengthM": 0.20, "joint": {"type": "fixed"}, "mass": {"kg": MASS["head"][0], "comX": MASS["head"][1], "comY": 0, "distribution": "point", "source": "literature", "reference": "bollard-catalogue-typical"}, "section": {"areaM2": r6(3 * RIB_T * (RIB_OUT - RIB_IN)), "secondMomentM4": r6(RIB_T * (RIB_OUT - RIB_IN) * 2 * (0.866 * RIB_AT[2]) ** 2), "extremeFibreM": r6(0.866 * RIB_OUT), "material": "alu-cast", "title": T("Three cast ribs 8 x 20 mm", "Drei Gussstege 8 x 20 mm")}},
    ]}
    total = round(sum(m for m, _ in MASS.values()), 2)
    anchors = [
        {"id": "footing-soil", "bone": "footing", "x": H_SOCKET, "y": 0, "kind": "support", "title": T("Socket footing held by the soil, equivalent fixity at ground level", "Köcherfundament im Boden gehalten, gleichwertige Einspannung in Geländehöhe"), "support": "fixed", "footprintLengthM": 0.5, "footprintWidthM": 0.5},
        {"id": "grip-right", "bone": "bollard", "x": 0.64, "y": 0, "z": -R_TUBE, "kind": "support", "title": T("Right hand grip of the depot worker under the head", "Rechter Handgriff des Bauhofmitarbeiters unter dem Kopf"), "support": "pin"},
        {"id": "grip-left", "bone": "bollard", "x": 0.64, "y": 0, "z": R_TUBE, "kind": "support", "title": T("Left hand grip of the second depot worker under the head", "Linker Handgriff des zweiten Bauhofmitarbeiters unter dem Kopf"), "support": "pin"},
        {"id": "lean-point", "bone": "head", "x": 0.20, "y": 0, "kind": "load", "title": T("Forearms of a leaning visitor on the cap", "Unterarme eines angelehnten Besuchers auf der Kappe")},
        {"id": "mower-hit", "bone": "bollard", "x": 0.15, "y": 0, "kind": "load", "title": T("Deck edge of a ride-on mower at the dent", "Mähdeckkante eines Aufsitzmähers an der Delle")},
        {"id": "head-plume", "bone": "head", "x": 0.2015, "y": 0, "kind": "emitter", "title": T("Warm air rising from the cap above the LED module", "Warme Luft über der Kappe oberhalb des LED-Moduls")},
    ]
    sun = {"tube-coat": 0.5, "head-housing": 0.6, "diffuser": 0.4, "anchor-cage": 0.0, "socket-concrete": 0.03, "sand-fill": 0.1}
    def regions(sunlit, lifted=False):
        out = {}
        for rid in ("tube-coat", "head-housing", "diffuser", "anchor-cage", "socket-concrete", "sand-fill"):
            b = {"exposedFraction": 1.0, "sunlitFraction": round(sun[rid] if sunlit else 0.0, 3)}
            if rid == "anchor-cage" and not lifted:
                b["contact"] = {"fraction": 0.95, "tempC": 16}
            if rid == "anchor-cage" and lifted:
                b["sunlitFraction"] = 0.3 if sunlit else 0.0
            if rid == "socket-concrete":
                b["contact"] = {"fraction": 0.96, "tempC": 16}
            if rid == "sand-fill":
                b["contact"] = {"fraction": 0.85 if not lifted else 0.8, "tempC": 16}
            if rid == "tube-coat" and not lifted:
                b["contact"] = {"fraction": 0.12, "tempC": 22}
            out[rid] = b
        return out
    def pose(id, en, de, den, dde, sunlit, lamp, contacts, bones=None, loads=None, lifted=False):
        return {"id": id, "title": T(en, de), "description": T(den, dde), "bones": bones or {}, "ground": "authored", "contacts": contacts, "thermal": {"regions": regions(sunlit, lifted), "emitters": {"head-plume": lamp}, "operating": {"lamp": lamp}}, "loads": loads or []}
    poses = [
        pose("in-service", "In service, summer afternoon", "Im Betrieb, Sommernachmittag", "Standing at the edge of a water-bound gravel path on its anchor cage in the sand-filled socket, lamp off; the low sun heats the anthracite tube and cap, the dent from the mower faces the path.", "Steht am Rand eines wassergebundenen Weges auf dem Ankerkorb im sandverfüllten Köcher, Leuchte aus; die tiefe Sonne erwärmt das anthrazitfarbene Rohr und die Kappe, die Mäherdelle zeigt zum Weg.", True, 0, ["footing-soil"]),
        pose("dusk-off", "Dusk, off", "Dämmerung, aus", "After sunset, the light sensor has not yet switched; the coat gives back the heat of the afternoon.", "Nach Sonnenuntergang, der Dämmerungsschalter hat noch nicht geschaltet; die Beschichtung gibt die Wärme des Nachmittags ab.", False, 0, ["footing-soil"]),
        pose("lit-night", "Lit at night", "Nachts eingeschaltet", "The 15 W LED retrofit module under the cap is on; the opal diffuser glows between the louvres and the cap releases its heat into a small plume.", "Das LED-Nachrüstmodul mit 15 W unter der Kappe ist eingeschaltet; die opale Abdeckung leuchtet zwischen den Lamellen und die Kappe gibt ihre Wärme in eine kleine Fahne ab.", False, 1, ["footing-soil"]),
        pose("in-use", "In use, a visitor leaning on the head", "In Benutzung, ein Besucher lehnt auf dem Kopf", "Lit at night while a visitor waiting on the path leans with both forearms on the cap: 0.25 kN down and 0.08 kN pushed away from the path.", "Nachts eingeschaltet, während ein wartender Besucher sich mit beiden Unterarmen auf die Kappe lehnt: 0,25 kN nach unten und 0,08 kN vom Weg weg.", False, 1, ["footing-soil"], loads=[{"id": "lean", "kind": "occupant", "title": T("Leaning forearms on the cap", "Aufgelehnte Unterarme auf der Kappe"), "anchor": "lean-point", "fxN": -80, "fyN": -250, "source": "authored-estimate", "reference": "en-1991-1-1"}]),
        pose("mower-strike", "Mower strike", "Mäheranprall", "The moment the deck edge of a ride-on mower hits the tube 0.15 m above ground with 1.5 kN: the strike that left the dent; the tube and the cage carry it into the socket.", "Der Augenblick, in dem die Mähdeckkante eines Aufsitzmähers mit 1,5 kN 0,15 m über dem Boden auf das Rohr trifft: der Stoß, der die Delle hinterließ; Rohr und Ankerkorb leiten ihn in den Köcher.", True, 0, ["footing-soil"], loads=[{"id": "mower", "kind": "equipment", "title": T("Mower deck edge", "Mähdeckkante"), "anchor": "mower-hit", "fxN": -1500, "fyN": 0, "source": "authored-estimate", "reference": "en-62262"}]),
        pose("maintenance", "Maintenance, lifted out with its cage", "Wartung, mit Ankerkorb herausgehoben", "After the cable is disconnected and the sand loosened, two depot workers pull the bollard with its bolted anchor cage 0.47 m out of the socket by hand, gripping the tube under the head; the socket and its fill stay in the ground.", "Nach dem Abklemmen des Kabels und dem Lockern des Sandes ziehen zwei Bauhofmitarbeiter den Poller mit dem angeschraubten Ankerkorb von Hand 0,47 m aus dem Köcher und greifen dabei das Rohr unter dem Kopf; Köcher und Füllung bleiben im Boden.", True, 0, ["grip-right", "grip-left"], bones={"bollard": {"dx": LIFT}}, lifted=True),
    ]
    clips = [
        {"id": "maintenance-lift", "title": T("Lifting out for maintenance", "Herausheben zur Wartung"), "description": T("The sand around the cage is loosened, then the two workers take the weight at the grips and pull the bollard with its cage 0.47 m out of the socket.", "Der Sand um den Ankerkorb wird gelockert, dann übernehmen die beiden Mitarbeiter das Gewicht an den Griffen und ziehen den Poller mit dem Ankerkorb 0,47 m aus dem Köcher."), "durationS": 10, "fps": 10, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 2.5, "pose": "in-service", "ease": "smooth"}, {"t": 10, "pose": "maintenance", "ease": "smooth"}],
         "contacts": [{"anchor": "footing-soil", "fromS": 0, "toS": 2.5}, {"anchor": "grip-right", "fromS": 2, "toS": 10}, {"anchor": "grip-left", "fromS": 2, "toS": 10}],
         "phases": [{"id": "loosen", "fromS": 0, "toS": 2.5, "title": T("Sand loosened, grips taken", "Sand gelockert, Griffe gefasst")}, {"id": "lift", "fromS": 2.5, "toS": 10, "title": T("Pulled out of the socket", "Aus dem Köcher gezogen")}],
         "events": [{"t": 2, "kind": "contact-on", "title": T("The workers take the weight", "Die Mitarbeiter übernehmen das Gewicht"), "target": "grip-right"}, {"t": 2.5, "kind": "contact-off", "title": T("The cage leaves its bed", "Der Ankerkorb verlässt sein Bett"), "target": "footing-soil"}, {"t": 10, "kind": "motion", "title": T("Cage clear of the socket rim", "Ankerkorb über dem Köcherrand")}],
         "tracks": []},
        {"id": "dusk-switching", "title": T("Switching on at dusk", "Einschalten in der Dämmerung"), "description": T("The light sensor switches the LED module on two minutes into dusk; the cap warms up over the next quarter of an hour and a small warm plume rises above it.", "Der Dämmerungsschalter schaltet das LED-Modul zwei Minuten nach Beginn der Dämmerung ein; die Kappe erwärmt sich in der folgenden Viertelstunde und darüber steigt eine kleine warme Fahne auf."), "durationS": 900, "fps": 2, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "dusk-off", "ease": "step"}, {"t": 120, "pose": "lit-night", "ease": "step"}, {"t": 900, "pose": "lit-night", "ease": "step"}],
         "contacts": [{"anchor": "footing-soil", "fromS": 0, "toS": 900}],
         "phases": [{"id": "dark", "fromS": 0, "toS": 120, "title": T("Dusk, off", "Dämmerung, aus")}, {"id": "warm-up", "fromS": 120, "toS": 900, "title": T("Lit, cap warming up", "Eingeschaltet, Kappe erwärmt sich")}],
         "events": [{"t": 120, "kind": "ignition", "title": T("Light sensor switches on", "Dämmerungsschalter schaltet ein"), "target": "lamp"}, {"t": 180, "kind": "thermal", "title": T("Cap starts to warm up", "Kappe beginnt sich zu erwärmen")}],
         "tracks": [{"target": "operating:lamp", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 120, "v": 1}]}, {"target": "emitter:head-plume", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 120, "v": 1}]}]},
    ]
    areas = AREAS
    def region(id, en, de, kind, model, material, area, length, geom, sky, layer, extra=None):
        lid, len_, lde, lk, lm, t, cap = layer
        k = next(m["conductivityWmK"] for m in MATERIALS if m["id"] == lm)
        r = {"id": id, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geom, "skyViewFactor": sky,
             "layers": [{"id": lid, "title": T(len_, lde), "kind": lk, "material": lm, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": cap, "source": "literature", "reference": "iso-10456-2007"}],
             "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}
        if extra:
            r.update(extra)
        return r
    thermal = {
        "environment": {"airTempC": 24, "meanRadiantTempC": 27, "skyTempC": 10, "groundTempC": 30, "relativeHumidity": 0.5, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 650, "diffuseFraction": 0.25, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "lamp", "title": T("LED retrofit module", "LED-Nachrüstmodul"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("On (15 W input, 11 W of heat)", "Ein (15 W Aufnahme, 11 W Wärme)")}]}],
        "regions": [
            region("tube-coat", "Powder-coated tube, collar and flange (weathered)", "Pulverbeschichtetes Rohr, Kragen und Flansch (verwittert)", "surface", "passive", "alu-tube", areas["tube-coat"], 0.2, "cylinder", 0.5, ("coat-wall", "Aluminium wall 3 mm under a chalked 80 µm powder coat", "Aluminiumwand 3 mm unter gekreideter Pulverbeschichtung 80 µm", "metal", "alu-tube", 0.003, 7290)),
            region("head-housing", "Cast cap, louvres, ribs and base ring with the LED module", "Gusskappe, Lamellen, Stege und Grundring mit LED-Modul", "lamp", "operating", "alu-cast", areas["head-housing"], 0.21, "horizontal-plate-up", 0.7, ("cast-wall", "Cast aluminium about 6 mm", "Aluminiumguss etwa 6 mm", "metal", "alu-cast", 0.006, 14472), {"operating": {"key": "lamp", "heatW": 11}, "ratedPowerW": 15, "ratedPowerBasis": "electrical-input"}),
            region("diffuser", "Opal PMMA diffuser, yellowed", "Opale PMMA-Abdeckung, vergilbt", "glazing", "passive", "pmma-opal", areas["diffuser"], 0.166, "cylinder", 0.4, ("pmma-wall", "PMMA 3 mm", "PMMA 3 mm", "polymer", "pmma-opal", 0.003, 5248)),
            region("anchor-cage", "Galvanised anchor cage with nuts", "Verzinkter Ankerkorb mit Muttern", "surface", "passive", "steel-galv", areas["anchor-cage"], 0.012, "cylinder", 0.0, ("rod-steel", "Steel rod and ring 8 mm", "Stahlstange und -ring 8 mm", "metal", "steel-galv", 0.008, 28260)),
            region("socket-concrete", "Concrete pipe socket", "Betonrohr-Köcher", "surface", "passive", "concrete", areas["socket-concrete"], 0.5, "vertical-plate", 0.0, ("concrete", "Concrete 50 mm wall", "Beton 50 mm Wand", "masonry", "concrete", 0.05, 120000)),
            region("sand-fill", "Sand-gravel fill of the socket", "Sand-Kies-Füllung des Köchers", "surface", "passive", "sand-fill", areas["sand-fill"], 0.4, "horizontal-plate-up", 0.9, ("sand", "Thermally active sand 100 mm", "Thermisch aktiver Sand 100 mm", "masonry", "sand-fill", 0.1, 154700)),
        ],
        "emitters": [{"id": "head-plume", "kind": "plume", "title": T("Warm plume above the cap", "Warme Fahne über der Kappe"), "anchor": "head-plume", "directionDeg": 90, "diameterM": 0.16, "region": "head-housing", "source": "authored-estimate"}],
        "assumptions": [
            T("Reference case: a clear summer afternoon in a German city park, 24 degC air, 650 W/m2 of sun at a low angle; the night poses switch the sun off (sunlit fraction 0) in the same air. No temperature is authored; surfaces follow their heat balance.", "Referenzfall: ein klarer Sommernachmittag in einem deutschen Stadtpark, Luft 24 degC, 650 W/m2 Sonne unter flachem Winkel; die Nachtposen schalten die Sonne ab (besonnter Anteil 0) bei gleicher Luft. Es wird keine Temperatur festgelegt; die Oberflächen folgen ihrer Wärmebilanz."),
            T("Weathering: the anthracite polyester powder coat has chalked over about 20 years, so its solar absorptance (0.86 to 0.88) is a few hundredths below the new value of about 0.92 and its emissivity stays 0.9; the opal PMMA has yellowed (solar absorptance 0.35 instead of about 0.2 new); the zinc of the cage has a dull patina (emissivity 0.6 instead of 0.25 new). Authored estimates after Incropera 2018 and Brandt 2008.", "Verwitterung: die anthrazitfarbene Polyester-Pulverbeschichtung ist in etwa 20 Jahren gekreidet, ihr solarer Absorptionsgrad (0,86 bis 0,88) liegt einige Hundertstel unter dem Neuwert von etwa 0,92, der Emissionsgrad bleibt 0,9; das opale PMMA ist vergilbt (solarer Absorptionsgrad 0,35 statt etwa 0,2 neu); das Zink des Ankerkorbs hat eine stumpfe Patina (Emissionsgrad 0,6 statt 0,25 neu). Schätzwerte nach Incropera 2018 und Brandt 2008."),
            T("Lamp: the original 26 W compact fluorescent lamp was replaced by a 15 W LED module on a heat spreader under the cap (catalogue-typical retrofit); about 4 W leave as light, 11 W as heat through the cap (Schubert 2006). The nameplate of the asset is these 15 W electrical input; everything else is passive.", "Leuchte: die ursprüngliche Kompaktleuchtstofflampe mit 26 W wurde durch ein LED-Modul mit 15 W auf einem Wärmeverteiler unter der Kappe ersetzt (katalogtypische Nachrüstung); etwa 4 W verlassen sie als Licht, 11 W als Wärme über die Kappe (Schubert 2006). Die Nennleistung des Objekts sind diese 15 W elektrische Aufnahme; alles andere ist passiv."),
            T("Ground: socket, fill and the buried cage exchange with the soil at 16 degC through their contact fraction; only the top of the fill and of the socket see the sky. When the bollard is lifted out the cage is in the air.", "Boden: Köcher, Füllung und der eingegrabene Ankerkorb tauschen über ihren Kontaktanteil mit dem Erdreich bei 16 degC; nur die Oberseiten von Füllung und Köcher sehen den Himmel. Ist der Poller herausgehoben, steht der Ankerkorb in der Luft."),
        ],
    }
    structure = {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T("In service the socket is held by the soil; its fixity is idealised at ground level (support footing-soil). The bollard is one rigid body with its bolted anchor cage; the cage rods carry bending and shear from the flange into the compacted fill.", "Im Betrieb wird der Köcher vom Boden gehalten; seine Einspannung ist in Geländehöhe idealisiert (Auflager footing-soil). Der Poller ist mit dem angeschraubten Ankerkorb ein starrer Körper; die Gewindestangen leiten Biegung und Querkraft vom Flansch in die verdichtete Füllung."),
        T("In maintenance the two hand grips under the head are pins that carry the whole weight of bollard and cage; the socket and the fill are ground and have no mass in this asset (about 102 kg of concrete and 128 kg of fill stay in place).", "Bei der Wartung tragen die beiden Handgriffe unter dem Kopf als Gelenke das ganze Gewicht von Poller und Ankerkorb; Köcher und Füllung gehören zum Boden und haben in diesem Objekt keine Masse (etwa 102 kg Beton und 128 kg Füllung bleiben an Ort)."),
        T("Loads: the leaning visitor (0.25 kN down, 0.08 kN horizontal) is an authored estimate on the basis of EN 1991-1-1 for persons on street furniture; the mower strike of 1.5 kN at 0.15 m is a quasi-static stand-in for an impact of about 20 J (IK10, EN 62262) that dented the 3 mm wall by 10 mm. Wind (about 0.1 kN on the 0.18 m2 of the bollard after EN 1991-1-4) is far below both and is not posed.", "Lasten: der angelehnte Besucher (0,25 kN nach unten, 0,08 kN waagerecht) ist eine Schätzung auf Grundlage von EN 1991-1-1 für Personen an Stadtmobiliar; der Mäheranprall von 1,5 kN in 0,15 m ist ein quasistatischer Ersatz für einen Stoß von etwa 20 J (IK10, EN 62262), der die 3-mm-Wand um 10 mm eingedellt hat. Wind (etwa 0,1 kN auf die 0,18 m2 des Pollers nach EN 1991-1-4) liegt weit darunter und wird nicht als Pose geführt."),
    ], "clinical": False}
    reps, vols = {}, {}
    pens = {"50": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.18, "ground": 0.5}, "100": {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.13, "ground": 0.35}, "200": {"body": 0.18, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.25}, "500": {"body": 0.14, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}
    hatches = {"50": [{"id": "concrete-stipple", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13}, {"id": "sand-stipple", "title": T("Sand stipple", "Sandpunktierung"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.1}], "100": [{"id": "concrete-stipple", "title": T("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.1}], "200": [], "500": []}
    ranges = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}
    views = {"50": views50(), "100": views100(), "200": views200(), "500": views500()}
    bodies = {"50": body50(), "100": body100(), "200": body200(), "500": body500()}
    for s in SCALES:
        v = views[s]
        reps[s] = {"status": "authored", "denominator": int(s), "validRange": {"minDenominator": ranges[s][0], "maxDenominator": ranges[s][1]}, "pens": pens[s], "hatches": hatches[s], "geometry": {"elevation": v.e, "front": v.f, "plan": v.p}, "decisions": REP_DECISIONS[s]}
        vols[s] = {"status": "authored", "denominator": int(s), "body": bodies[s], "thermal": thermal_list(bodies[s], s), "structure": structure_list(s, bodies[s]), "decisions": VOL_DECISIONS[s]}
    asset = {
        "id": SLUG, "category": "objects",
        "title": T("Used Path Bollard Light", "Gebrauchte Pollerleuchte"),
        "summary": T("A municipal path bollard light of the 1990s, 0.90 m above ground: a 200 mm aluminium tube with a louvre head of four conical rings around a yellowed opal diffuser, retrofitted with a 15 W LED module, bolted to a 4 x M12 anchor cage in a sand-filled concrete socket 0.60 m deep; chalked anthracite coat, scratches, sticker residue and a 10 mm mower dent at 0.15 m. Standing in sun, lit at dusk, leaned on, struck by a mower and lifted out with its cage by two depot workers.", "Eine kommunale Wegepollerleuchte der 1990er-Jahre, 0,90 m über dem Boden: ein Aluminiumrohr von 200 mm mit Lamellenkopf aus vier kegeligen Ringen um eine vergilbte opale Abdeckung, nachgerüstet mit einem LED-Modul von 15 W, verschraubt auf einem Ankerkorb 4 x M12 in einem sandverfüllten Betonköcher von 0,60 m Tiefe; gekreidete anthrazitfarbene Beschichtung, Kratzer, Aufkleberreste und eine Mäherdelle von 10 mm in 0,15 m Höhe. In der Sonne stehend, in der Dämmerung eingeschaltet, angelehnt, vom Mäher getroffen und von zwei Bauhofmitarbeitern mit dem Ankerkorb herausgehoben."),
        "tags": ["used", "park", "public-space", "lighting", "bollard-light", "louvre", "aluminium", "powder-coat", "led-retrofit", "anchor-cage", "path"],
        "provenance": {"author": "Semio staffage content (used park objects wave, row 13)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": k, "citation": v} for k, v in REF.items()],
                       "notes": T("Heights: 0.90 m above ground to the top of the cap, the socket footing 0.60 m below ground, so the drawn height is 1.50 m. dimensions.massKg is bollard, head and cage (what is lifted out); the socket and its fill are ground. Every mass is section x density of the modelled 1:50 parts. Catalogue data are typical of the type and tagged literature where a standard fixes them (tube, nuts, pipe), authored-estimate where they are judged (loads, weathering, lamp share); nothing is measured.", "Höhen: 0,90 m über dem Boden bis Oberkante Kappe, das Köcherfundament 0,60 m unter dem Boden, die gezeichnete Höhe ist daher 1,50 m. dimensions.massKg umfasst Poller, Kopf und Ankerkorb (was herausgehoben wird); Köcher und Füllung gehören zum Boden. Jede Masse ist Querschnitt mal Dichte der modellierten Teile 1:50. Katalogwerte sind typenüblich und als literature markiert, wo eine Norm sie festlegt (Rohr, Muttern, Betonrohr), als authored-estimate, wo sie geschätzt sind (Lasten, Verwitterung, Lampenanteil); nichts ist gemessen.")},
        "dimensions": {"heightM": 1.5, "widthM": 2 * R_SOCKET_OUT, "depthM": 2 * R_SOCKET_OUT, "massKg": total},
        "materials": MATERIALS, "rig": rig, "anchors": anchors, "representations": reps, "symbols": [], "poses": poses, "clips": clips,
        "thermal": thermal, "structure": structure, "volumes": vols,
        "presentation": {"defaultPose": "in-service", "defaultClip": "maintenance-lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


AREAS = {"tube-coat": 0.56, "head-housing": 0.24, "diffuser": 0.079, "anchor-cage": 0.21, "socket-concrete": 1.838, "sand-fill": 1.005}

D = T
REP_DECISIONS = {
    "50": {
        "silhouette": D("Side elevation 30 x 10 mm above a 12 mm socket: the concrete socket below the ground line, the chamfered flange with two of the four nuts, the collar, the tube with the 10 mm mower dent stepped into its path-side edge at 0.10 to 0.20 m, the base ring, the opal diffuser between the two near ribs, four conical louvres and the domed cap with its drip lip. Front: the same in width with the dent face-on as its two creases and the side rib. Plan: socket ring, sand fill, flange, four hexagon nuts with rod ends, the cap with its crown.", "Seitenansicht 30 x 10 mm über einem Köcher von 12 mm: der Betonköcher unter der Geländelinie, der gefaste Flansch mit zwei der vier Muttern, der Kragen, das Rohr mit der 10-mm-Mäherdelle in der wegseitigen Kante bei 0,10 bis 0,20 m, der Grundring, die opale Abdeckung zwischen den beiden vorderen Stegen, vier kegelige Lamellen und die gewölbte Kappe mit Tropfkante. Vorderansicht: dasselbe in der Breite, die Delle von vorn als zwei Knickfalten und der seitliche Steg. Draufsicht: Köcherring, Sandfüllung, Flansch, vier Sechskantmuttern mit Stangenenden, die Kappe mit ihrem Scheitel."),
        "retained": D("Every assembly: socket, cage rods and both rings (behind the socket in service, free when lifted), flange, collar, nuts, tube, dent, base ring, diffuser, ribs, louvres, cap; as traces the sticker residue, scratches, the flaking ring above the collar, chalking on the cap and lichen on its crown.", "Jede Baugruppe: Köcher, Gewindestangen und beide Ringe des Ankerkorbs (im Betrieb hinter dem Köcher, herausgehoben frei), Flansch, Kragen, Muttern, Rohr, Delle, Grundring, Abdeckung, Stege, Lamellen, Kappe; als Spuren die Aufkleberreste, Kratzer, der Abplatzring über dem Kragen, Kreidung auf der Kappe und Flechten auf ihrem Scheitel."),
        "merged": D("Washer and nut are one hexagon; the sand fill is drawn in plan only (it is inside the socket in the views).", "Scheibe und Mutter sind ein Sechskant; die Sandfüllung ist nur im Grundriss gezeichnet (in den Ansichten liegt sie im Köcher)."),
        "omitted": D("The LED module under the cap and the cable inside the tube (hidden), the levelling nuts under the flange (in the fill), the third rib behind the diffuser.", "Das LED-Modul unter der Kappe und das Kabel im Rohr (verdeckt), die Kontermuttern unter dem Flansch (in der Füllung), der dritte Steg hinter der Abdeckung."),
        "thinElements": D("Louvres are 3 mm cones drawn as their true trapezoids (0.06 mm thick on paper); creases, scratches, sticker outline, flaking and chalking are 0.13 mm strokes; the 0.6 mm sticker film itself is below the smallest feature and is only drawn.", "Lamellen sind Kegel von 3 mm, als wahre Trapeze gezeichnet (0,06 mm dick auf dem Papier); Knickfalten, Kratzer, Aufkleberumriss, Abplatzungen und Kreidung sind Striche von 0,13 mm; die Aufkleberfolie von 0,6 mm liegt unter dem kleinsten Merkmal und ist nur gezeichnet."),
        "hatching": D("Concrete dots in the socket, sand stipple in the fill (plan); the metal parts stay plain tone.", "Betonpunkte im Köcher, Sandpunktierung in der Füllung (Grundriss); die Metallteile bleiben ohne Schraffur."),
        "styles": D("Socket light (0.65), fill lighter (0.75), anthracite tube 0.35, cast parts 0.3, nuts 0.2, diffuser light 0.85; body 0.35, detail 0.18, texture 0.13.", "Köcher hell (0,65), Füllung heller (0,75), anthrazitfarbenes Rohr 0,35, Gussteile 0,3, Muttern 0,2, Abdeckung hell 0,85; Körper 0,35, Detail 0,18, Textur 0,13."),
        "overlays": D("Socket = socket-concrete, fill = sand-fill, rods, rings and nuts = anchor-cage, flange, collar and tube = tube-coat, base ring, ribs, louvres and cap = head-housing (operating, the LED heat), diffuser = diffuser.", "Köcher = socket-concrete, Füllung = sand-fill, Stangen, Ringe und Muttern = anchor-cage, Flansch, Kragen und Rohr = tube-coat, Grundring, Stege, Lamellen und Kappe = head-housing (Betrieb, LED-Wärme), Abdeckung = diffuser."),
    },
    "100": {
        "silhouette": D("15 x 5 mm above the 6 mm socket: socket, flange, nut blocks, the tube with its collar step, base ring, diffuser, four louvre trapezoids and the cap without drip lip. Front likewise; plan: socket, fill, flange, four nuts, cap.", "15 x 5 mm über dem Köcher von 6 mm: Köcher, Flansch, Mutternblöcke, das Rohr mit Kragenabsatz, Grundring, Abdeckung, vier Lamellentrapeze und die Kappe ohne Tropfkante. Vorderansicht ebenso; Grundriss: Köcher, Füllung, Flansch, vier Muttern, Kappe."),
        "retained": D("Socket, cage rods and both rings, flange, nuts, tube, collar, louvres, diffuser, base ring, cap; the dent and one scratch as texture strokes.", "Köcher, Gewindestangen und beide Ringe, Flansch, Muttern, Rohr, Kragen, Lamellen, Abdeckung, Grundring, Kappe; die Delle und ein Kratzer als Texturstriche."),
        "merged": D("Collar and tube are one outline; nut, washer and rod end are one block; the 8 mm cage rings are drawn 15 mm; the louvres reach in to the diffuser, closing the 5 mm gap the ribs bridge at 1:50.", "Kragen und Rohr sind ein Umriss; Mutter, Scheibe und Stangenende sind ein Block; die Ankerkorbringe von 8 mm sind mit 15 mm gezeichnet; die Lamellen reichen bis an die Abdeckung und schließen den Spalt von 5 mm, den bei 1:50 die Stege überbrücken."),
        "omitted": D("Ribs (8 mm), drip lip, sticker, flaking and chalking; the dent (10 mm) is below the 15 mm feature and is a stroke.", "Stege (8 mm), Tropfkante, Aufkleber, Abplatzungen und Kreidung; die Delle (10 mm) liegt unter dem Merkmal von 15 mm und ist ein Strich."),
        "thinElements": D("Louvres and flange stay at their true 3 mm and 12 mm; rods are 0.12 mm filled strips.", "Lamellen und Flansch bleiben bei ihren wahren 3 mm und 12 mm; Stangen sind gefüllte Streifen von 0,12 mm."),
        "hatching": D("Concrete dots in the socket.", "Betonpunkte im Köcher."),
        "styles": D("Same tones as 1:50; body 0.25, detail 0.13, texture 0.1.", "Gleiche Tonwerte wie 1:50; Körper 0,25, Detail 0,13, Textur 0,1."),
        "overlays": D("As 1:50.", "Wie 1:50."),
    },
    "200": {
        "silhouette": D("7.5 x 2.5 mm above the 3 mm socket: socket, flange line, tube, the louvre band as one mass with two louvre lines and the cap.", "7,5 x 2,5 mm über dem Köcher von 3 mm: Köcher, Flanschlinie, Rohr, das Lamellenband als eine Masse mit zwei Lamellenlinien und die Kappe."),
        "retained": D("Socket, cage rods (lines), flange, tube, louvre band, cap; plan: socket, fill, flange, cap.", "Köcher, Gewindestangen (Linien), Flansch, Rohr, Lamellenband, Kappe; Grundriss: Köcher, Füllung, Flansch, Kappe."),
        "merged": D("Base ring, diffuser and four louvres are one band of the louvre diameter; two lines typify the four louvres.", "Grundring, Abdeckung und vier Lamellen sind ein Band im Lamellendurchmesser; zwei Linien stehen für die vier Lamellen."),
        "omitted": D("Collar, nuts, cage rings, every trace of use (all below 30 mm).", "Kragen, Muttern, Ankerkorbringe, alle Gebrauchsspuren (alle unter 30 mm)."),
        "thinElements": D("The cage rods are 0.1 mm detail lines; the flange is its true 12 mm.", "Die Gewindestangen sind Detaillinien von 0,1 mm; der Flansch hat seine wahren 12 mm."),
        "hatching": D("None: a hatch does not print at this size.", "Keine: eine Schraffur druckt in dieser Größe nicht."),
        "styles": D("Tones as 1:50; body 0.18, detail 0.1.", "Tonwerte wie 1:50; Körper 0,18, Detail 0,1."),
        "overlays": D("Band = head-housing and diffuser; otherwise as 1:50.", "Band = head-housing und diffuser; sonst wie 1:50."),
    },
    "500": {
        "silhouette": D("3 x 1 mm: the socket block below the ground line, the tube and the head as one 0.20 m block (light band and cap; drawn apart they would not print at 1:1000).", "3 x 1 mm: der Köcherblock unter der Geländelinie, das Rohr und der Kopf als ein Block von 0,20 m (Leuchtband und Kappe; getrennt gezeichnet drucken sie bei 1:1000 nicht)."),
        "retained": D("Socket, cage rods (thin lines, seen when lifted), tube, light band, cap; plan: socket, fill, cap.", "Köcher, Gewindestangen (dünne Linien, herausgehoben sichtbar), Rohr, Leuchtband, Kappe; Grundriss: Köcher, Füllung, Kappe."),
        "merged": D("The rods end in the flange, kept as a declared 12 mm sheet so the lifted cage hangs on it; louvres, diffuser and base ring into one band of 120 mm; the cap is an 80 mm block.", "Die Stangen enden im Flansch, der als erklärte Platte von 12 mm bleibt, damit der herausgehobene Ankerkorb an ihm hängt; Lamellen, Abdeckung und Grundring in ein Band von 120 mm; die Kappe ist ein Block von 80 mm."),
        "omitted": D("Nuts, rings, collar and all traces of use.", "Muttern, Ringe, Kragen und alle Gebrauchsspuren."),
        "thinElements": D("Rods are 0.1 mm lines.", "Stangen sind Linien von 0,1 mm."),
        "hatching": D("None.", "Keine."),
        "styles": D("Tones as 1:50; body 0.14.", "Tonwerte wie 1:50; Körper 0,14."),
        "overlays": D("Head = diffuser and head-housing; otherwise as 1:50.", "Kopf = diffuser und head-housing; sonst wie 1:50."),
    },
}

VOL_DECISIONS = {
    "50": [
        D("Body at 1:50 (feature 7.5 mm): the precast socket DN 400 as a hollow 50 mm wall with its sand-gravel fill; the anchor cage of four M12 rods (slender) and two 40 x 8 mm template rings; the cast flange with chamfer, the collar, four hexagon nuts with washers; the 200 x 3 mm tube in five pieces, three of them the stepped 5 / 10 / 5 mm mower dent as dented sections facing the path; the head: base ring, opal diffuser tube, three 8 x 20 mm ribs, four 3 mm conical louvres (hoops, slender), the domed cap with drip lip and the LED heat spreader under it.", "Körper 1:50 (Merkmal 7,5 mm): der Fertigteilköcher DN 400 als hohle Wand von 50 mm mit Sand-Kies-Füllung; der Ankerkorb aus vier Stangen M12 (schlank) und zwei Schablonenringen 40 x 8 mm; der gegossene Flansch mit Fase, der Kragen, vier Sechskantmuttern mit Scheiben; das Rohr 200 x 3 mm in fünf Stücken, drei davon die gestufte Mäherdelle 5 / 10 / 5 mm als eingedellte Querschnitte zum Weg; der Kopf: Grundring, opales Abdeckrohr, drei Stege 8 x 20 mm, vier kegelige Lamellen von 3 mm (Reifen, schlank), die gewölbte Kappe mit Tropfkante und der LED-Wärmeverteiler darunter."),
        D("Thermal at 1:50: every body solid on its region; the plume outlet is a thin patch on the cap crown tagged with its emitter. The tube and diffuser cavities are closed by flange, base ring and cap, so only outer faces count.", "Thermik 1:50: jeder Körper auf seiner Region; der Fahnenaustritt ist ein dünnes Feld auf dem Kappenscheitel, mit seinem Emitter markiert. Rohr- und Abdeckungshohlraum sind durch Flansch, Grundring und Kappe geschlossen, es zählen nur Außenflächen."),
        D("Structure at 1:50: the concrete pipe wall as member footing with the compacted fill as its joint around the rods, the four rods (member cage) with their rings and nuts as joints, the flange as joint to the tube (member bollard), the base ring as joint to the three ribs (member head) and the cap as joint; three masses at the centres of bollard, cage and head.", "Tragwerk 1:50: die Betonrohrwand als Stab footing mit der verdichteten Füllung als Knoten um die Stangen, die vier Stangen (Stab cage) mit Ringen und Muttern als Knoten, der Flansch als Knoten zum Rohr (Stab bollard), der Grundring als Knoten zu den drei Stegen (Stab head) und die Kappe als Knoten; drei Massen in den Schwerpunkten von Poller, Ankerkorb und Kopf."),
    ],
    "100": [
        D("Body at 1:100 (feature 15 mm): socket and fill, rods, the two cage rings at 15 mm, the flange as a declared 12 mm sheet, four nut blocks of 20 mm, one tube with its collar step, base ring of 15 mm, diffuser, four louvres (slender), the cap without drip lip. The dent, the ribs, the LED spreader and the chamfer are below the feature and are left out or drawn as strokes.", "Körper 1:100 (Merkmal 15 mm): Köcher und Füllung, Stangen, die beiden Ankerkorbringe mit 15 mm, der Flansch als erklärte Platte von 12 mm, vier Mutternblöcke von 20 mm, ein Rohr mit Kragenabsatz, Grundring von 15 mm, Abdeckung, vier Lamellen (schlank), die Kappe ohne Tropfkante. Delle, Stege, LED-Wärmeverteiler und Fase liegen unter dem Merkmal und entfallen oder sind Striche."),
        D("Thermal at 1:100: the body solids on their regions and the plume patch.", "Thermik 1:100: die Körper auf ihren Regionen und das Fahnenfeld."),
        D("Structure at 1:100: socket and fill as one embedded footing block, rods with rings and nuts, flange, tube, base ring, the three ribs as slender members (kept in the structure although the body merges them), cap, three masses.", "Tragwerk 1:100: Köcher und Füllung als ein eingebetteter Fundamentblock, Stangen mit Ringen und Muttern, Flansch, Rohr, Grundring, die drei Stege als schlanke Stäbe (im Tragwerk behalten, obwohl der Körper sie verschmilzt), Kappe, drei Massen."),
    ],
    "200": [
        D("Body at 1:200 (feature 30 mm): socket and fill, the four rods (slender), the flange sheet, the plain tube, the head as one louvre band of 108 mm radius and a simplified cap; collar, nuts and rings are merged or left out.", "Körper 1:200 (Merkmal 30 mm): Köcher und Füllung, die vier Stangen (schlank), die Flanschplatte, das glatte Rohr, der Kopf als ein Lamellenband mit 108 mm Radius und eine vereinfachte Kappe; Kragen, Muttern und Ringe sind verschmolzen oder entfallen."),
        D("Thermal at 1:200: band and cap on head-housing, the diffuser kept as its own shell inside the band so the region keeps its solid; only the total area is compared.", "Thermik 1:200: Band und Kappe auf head-housing, die Abdeckung als eigene Schale im Band, damit die Region ihren Körper behält; verglichen wird nur die Gesamtfläche."),
        D("Structure at 1:200: footing block, rods, flange joint, tube, the band as head member, the cap as joint, three masses.", "Tragwerk 1:200: Fundamentblock, Stangen, Flanschknoten, Rohr, das Band als Kopfstab, die Kappe als Knoten, drei Massen."),
    ],
    "500": [
        D("Body at 1:500 (feature 75 mm): socket and fill, the four rods (slender, the cage when lifted), the flange as a declared 12 mm sheet that holds the rods, the tube, a light band of 120 mm and the cap as an 80 mm block.", "Körper 1:500 (Merkmal 75 mm): Köcher und Füllung, die vier Stangen (schlank, der Ankerkorb beim Herausheben), der Flansch als erklärte Platte von 12 mm, die die Stangen hält, das Rohr, ein Leuchtband von 120 mm und die Kappe als Block von 80 mm."),
        D("Thermal at 1:500: the band carries the diffuser region, the cap the head-housing region with the plume patch.", "Thermik 1:500: das Band trägt die Region diffuser, die Kappe die Region head-housing mit dem Fahnenfeld."),
        D("Structure at 1:500: footing block, rods, the flange as joint of rods and tube, tube, the head (band and cap) as one member block, three masses.", "Tragwerk 1:500: Fundamentblock, Stangen, der Flansch als Knoten von Stangen und Rohr, Rohr, der Kopf (Band und Kappe) als ein Stabblock, drei Massen."),
    ],
}


if __name__ == "__main__":
    doc = build()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print("[DEBUG] masses", MASS, "total", doc["asset"]["dimensions"]["massKg"], "footing kg", round(footing_mass(), 1))
