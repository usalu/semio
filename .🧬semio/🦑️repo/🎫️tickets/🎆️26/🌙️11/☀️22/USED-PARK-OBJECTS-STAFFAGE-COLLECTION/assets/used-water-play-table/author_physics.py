"""⚖️ Physics of `used-water-play-table`: masses from section x density, member sections, materials with weathered surface data, thermal regions with their hand-summed areas, and the references they come from.

@see ./author_geometry.py — the hand-chosen numbers
@see ../../brief.md — "Thermal", "Structure", "Provenance"
"""
import math

from author_geometry import (
    APRON_X, APRON_Y, APRON_Z, BEARER, BEARER_HALF_X, BEARER_Z, BS_CX, BS_FL, BS_FLOOR, BS_RIM, BS_T, BS_WATER, BS_X, BS_Y0, BS_Z, CAP, CHUTE_R,
    CHUTE_T, CLAMP, CRANK, DECK, END_FL_X, END_X, FOOT, FOOT_R, FOOT_Y0, GATE_H, GATE_HOLE, GATE_T, GATE_X, GATE_Z, GRIP, GUIDE, HELIX, MUD, PIN, PLATE,
    PLATE_Z, PLINTH, POOL_IDS, POOLS, POST_R, POST_Y0, POST_Y1, SAND_DRIFT, SHAFT, SPOUT, TR_FL, TR_IN_X, TR_IN_Z, TR_RIM, TR_T, TR_X, TR_Y0, TR_Z, TUBE,
    WATER_D, chute_line, foot_centre, local, screw_point,
)
from author_parts import tx

RHO = {"douglas-greyed": 500.0, "douglas-fresh": 470.0, "robinia-weathered": 750.0, "stainless-scaled": 7900.0, "polycarbonate-clouded": 1200.0, "pe-grip": 950.0, "concrete-weathered": 2300.0, "water": 1000.0, "sand-wet": 1900.0}


def shoelace(points):
    """📐 Area and centroid of a simple polygon."""
    a = cx = cy = 0.0
    for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1]):
        c = x0 * y1 - x1 * y0
        a += c
        cx += (x0 + x1) * c
        cy += (y0 + y1) * c
    a /= 2
    return abs(a), cx / (6 * a), cy / (6 * a)


def mud_profile():
    """🟤 The closed (height, radius) profile of the splash rim, as it is revolved."""
    _, ri, ro, h = MUD
    return [(0, ri), (0.012, ri + 0.02), (h, ri + 0.06), (0.02, ri + 0.10), (0.008, ro - 0.01), (0, ro)]


def trough_sheet_area():
    """🛁 Developed sheet area of the trough (m2): floor, long walls, long and end flanges, end plates."""
    length = 2 * TR_X
    return length * 2 * TR_Z + 2 * length * (TR_RIM - TR_Y0) + 2 * length * (TR_FL - TR_Z) + 2 * (2 * TR_IN_Z) * (TR_RIM - TR_Y0) + 2 * (END_FL_X - TR_X) * 2 * TR_FL


def basin_sheet_area():
    """🪣 Developed sheet area of the lower basin (m2)."""
    w = BS_X[1] - BS_X[0]
    return w * 2 * BS_Z + 2 * 2 * BS_Z * (BS_RIM - BS_Y0) + 2 * 2 * BS_Z * BS_FL + 2 * w * (BS_RIM - BS_Y0) + 2 * (w + 2 * BS_FL) * BS_FL


def masses():
    """⚖️ Bone masses (kg) and centres in world rest metres from section x density; one source for the rig, the mass markers, the report and `dimensions.massKg`."""
    st, dg, df, rb, cc, wt, sd = (RHO[k] for k in ("stainless-scaled", "douglas-greyed", "douglas-fresh", "robinia-weathered", "concrete-weathered", "water", "sand-wet"))
    out = {}
    apron = 2 * APRON_X * (APRON_Y[1] - APRON_Y[0]) * (APRON_Z[1] - APRON_Z[0]) * dg
    board = (END_X[1] - END_X[0]) * (APRON_Y[1] - APRON_Y[0]) * 2 * APRON_Z[0]
    deck = (DECK[1] - DECK[0]) * (DECK[3] - DECK[2]) * (DECK[5] - DECK[4]) * dg
    bearer = 2 * BEARER_HALF_X * (BEARER[1] - BEARER[0]) * 2 * BEARER_Z * dg
    sand = 0.5 * (SAND_DRIFT[1] - SAND_DRIFT[0]) * SAND_DRIFT[2] * 2 * TR_IN_Z * sd
    pa, pr, ph = shoelace([(r, h) for h, r in mud_profile()])
    mud = pa * 2 * math.pi * pr * sd
    pl = PLINTH
    plinth = (pl[1] - pl[0]) * (pl[3] - pl[2]) * 2 * pl[4] * cc
    plate_a, plate_cx, plate_cy = shoelace(PLATE)
    (xa, ya), (xb, yb) = chute_line()
    chute = math.pi * CHUTE_R * math.hypot(xa - xb, ya - yb) * CHUTE_T * st
    out["frame"] = [
        (apron, 0.0, sum(APRON_Y) / 2), (apron, 0.0, sum(APRON_Y) / 2),
        (board * dg, (END_X[0] + END_X[1]) / 2, sum(APRON_Y) / 2), (board * df, -(END_X[0] + END_X[1]) / 2, sum(APRON_Y) / 2),
        (deck, 0.0, (DECK[2] + DECK[3]) / 2), (bearer, 0.66, sum(BEARER) / 2), (bearer, -0.66, sum(BEARER) / 2),
        (trough_sheet_area() * TR_T * st, 0.0, 0.585),
        (4 * 0.03 * 0.0015 * (TR_RIM - TR_Y0) * st, 0.0, 0.61),
        (math.pi * (SPOUT[3] ** 2 - SPOUT[4] ** 2) * (SPOUT[1] - SPOUT[0]) * st, (SPOUT[0] + SPOUT[1]) / 2, SPOUT[2]),
        (sand, (2 * SAND_DRIFT[0] + SAND_DRIFT[1]) / 3, 0.5415 + SAND_DRIFT[2] / 3),
        (mud, MUD[0], ph),
        (2 * plate_a * (PLATE_Z[1] - PLATE_Z[0]) * st, plate_cx, plate_cy),
        (2 * math.pi * PIN[0] ** 2 * (PIN[2] - PIN[1]) * st, 0.92, 0.84),
        (0.03 * 0.008 * 0.192 * st, 0.85, 0.69),
        (chute, (xa + xb) / 2, (ya + yb) / 2 - CHUTE_R * 0.6),
    ]
    out["basin"] = [(plinth, BS_CX, (pl[2] + pl[3]) / 2), (basin_sheet_area() * BS_T * st, BS_CX, 0.225)]
    for bone in ("post-rl", "post-rr", "post-fl", "post-fr"):
        x = -0.66 if bone in ("post-rl", "post-rr") else 0.66
        post = math.pi * POST_R ** 2 * (POST_Y1 - POST_Y0) * rb
        footing = (math.pi * FOOT_R ** 2 * -FOOT_Y0 - math.pi * POST_R ** 2 * (0 - POST_Y0)) * cc
        out[bone] = [(post, x, (POST_Y0 + POST_Y1) / 2), (footing, x, FOOT_Y0 / 2)]
    h0, h1, hw = GATE_HOLE
    for gid, xg in GATE_X.items():
        out[gid] = [((2 * GATE_Z * GATE_H - 2 * hw * (h1 - h0)) * GATE_T * st, xg, 0.5415 + GATE_H / 2)]
    for pid in POOL_IDS:
        x0, x1 = POOLS[pid]
        out[pid] = [((x1 - x0) * WATER_D * 2 * TR_IN_Z * wt, (x0 + x1) / 2, 0.5415 + WATER_D / 2)]
    out["basin-water"] = [((BS_X[1] - BS_X[0] - 2 * BS_T) * BS_WATER * 2 * (BS_Z - BS_T) * wt, BS_CX, BS_FLOOR + BS_WATER / 2)]
    t0, t1, ri, ro = TUBE
    fx, fy0, fy1 = foot_centre()
    out["screw-mount"] = [
        (math.pi * (ro ** 2 - ri ** 2) * (t1 - t0) * RHO["polycarbonate-clouded"], *screw_point((t0 + t1) / 2, 0)),
        (math.pi * (CLAMP[2] ** 2 - ro ** 2) * (CLAMP[1] - CLAMP[0]) * st, *screw_point(0, 0)),
        (math.pi * (CAP[3] ** 2 - CAP[2] ** 2) * (CAP[1] - CAP[0]) * st, *screw_point((CAP[0] + CAP[1]) / 2, 0)),
        (0.25 * FOOT[0] * FOOT[1] * (fy1 - fy0) * st, fx, (fy0 + fy1) / 2),
    ]
    u0, u1, rs = SHAFT
    hh0, hh1, hr0, hr1, pitch = HELIX
    turns = (hh1 - hh0) / pitch
    flight = turns * math.pi * (hr1 ** 2 - hr0 ** 2) * 1.12 * 0.002 * st
    c0, c1, v0, v1, cw = CRANK
    out["screw"] = [
        (math.pi * (rs ** 2 - (rs - 0.002) ** 2) * (u1 - u0) * st, *screw_point((u0 + u1) / 2, 0)),
        (flight, *screw_point((hh0 + hh1) / 2, 0)),
        ((c1 - c0) * (v1 - v0) * 2 * cw * st, *screw_point((c0 + c1) / 2, (v0 + v1) / 2)),
    ]
    g0, g1, gr, g2, kr = GRIP
    out["grip"] = [(math.pi * gr ** 2 * (g1 - g0) * RHO["pe-grip"] + math.pi * kr ** 2 * (g2 - g1) * RHO["pe-grip"], *screw_point(-0.09 - 0.06, 0.135))]
    total = {}
    for bone, parts in out.items():
        m = sum(p[0] for p in parts)
        total[bone] = (m, sum(p[0] * p[1] for p in parts) / m, sum(p[0] * p[2] for p in parts) / m)
    return out, total


REFERENCES = [
    ("din-en-1176-1", "DIN EN 1176-1:2017-12 Playground equipment and surfacing, Part 1: General safety requirements and test methods (loads from users, finger and head entrapment, protruding parts, the 1.0 m free space around equipment without forced movement)."),
    ("din-en-1176-7", "DIN EN 1176-7:2020-07 Playground equipment and surfacing, Part 7: Guidance on installation, inspection, maintenance and operation (routine visual, operational and annual main inspection; winter draining of water play)."),
    ("din-18034", "DIN 18034:2020-10 Playgrounds and outdoor play areas: requirements for planning, construction and operation (water play areas with sand-water play tables for small children, drinking-water quality of the feed, drainage into sand)."),
    ("din-en-1177", "DIN EN 1177:2018-03 Impact attenuating playground surfacing: playground sand 0.2-2 mm as loose fill around low equipment without a fall height."),
    ("en-338", "EN 338:2016 Structural timber, strength classes (C24: fm,k 24 MPa, E0,mean 11.0 GPa, rho_mean 420 kg/m3; D40: fm,k 40 MPa, E0,mean 13.0 GPa, rho_mean 660 kg/m3)."),
    ("en-350", "EN 350:2016 Durability of wood and wood-based products: Robinia pseudoacacia durability class 1-2 against fungi in ground contact, Douglas fir class 3-4 (heartwood), hence Robinia for the posts."),
    ("en-10088-2", "EN 10088-2:2014 Stainless steels, Part 2: sheet and strip for general purposes (1.4301 / X5CrNi18-10: Rp0.2 >= 230 MPa, E 200 GPa, density 7900 kg/m3)."),
    ("en-iso-1127", "EN ISO 1127:1996 Stainless steel tubes, dimensions and tolerances (42.4 x 2.0 for the screw shaft)."),
    ("en-1991-1-1", "DIN EN 1991-1-1:2010-12 with NA: densities and self-weight (water 10 kN/m3, wet sand 19 kN/m3, concrete 24 kN/m3 nominal)."),
    ("en-1991-1-3", "DIN EN 1991-1-3:2010-12 with DIN EN 1991-1-3/NA:2019-04 Snow loads (zone 2, sk 0.85 kN/m2 below 285 m; flat surface mu 0.8)."),
    ("din-33402-2", "DIN 33402-2:2020-12 Ergonomics, human body dimensions, Part 2: values (children 3-6 years: stature 0.95-1.20 m, body mass 14-23 kg, shoulder height 0.75-0.95 m)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products, hygrothermal properties: design values of conductivity, specific heat and density of timber, concrete, steel and plastics."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of stainless steel, water, wet sand, plastics and weathered wood)."),
    ("duffie-beckman-2013", "Duffie, J. A., Beckman, W. A. (2013). Solar Engineering of Thermal Processes, 4th ed., Wiley (solar absorptance of greyed and fresh wood, stainless steel, concrete, wet sand and shallow water over a bright bottom)."),
    ("playground-catalogue", "Catalogue-typical data of German playground equipment manufacturers, 2000-2024, for sand-water play tables (Matschtische) and water play channels with an Archimedes screw: stainless 1.4301 channel 1.2-1.6 m long at 0.55-0.70 m working height on Robinia or Douglas frames, slide sluices in welded guides, screw tube 150-200 mm at 30-50 degrees with a hand crank; a product type, no brand."),
    ("galabau-water-play", "GaLaBau practice for water play in sand areas: posts set 0.5 m into augered C20/25 point footings 0.30 x 0.60 m, low basins bedded on a precast plinth in the sand, overflow into a sand hollow; seasonal draining from October to April."),
]


def materials():
    """🧱 The materials with weathered surface data; the replaced board carries the values of fresh timber."""
    def m(mid, en, de, eps, alpha, rho, lam, cp, e, f, src, ref):
        return {"id": mid, "title": tx(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": cp, "elasticModulusPa": e, "strengthPa": f, "source": src, "reference": ref}
    return [
        m("douglas-greyed", "Douglas fir C24, greyed and checked", "Douglasie C24, vergraut und rissig", 0.9, 0.7, 500, 0.13, 1600, 1.1e10, 2.4e7, "literature", "en-338"),
        m("douglas-fresh", "Douglas fir C24, freshly planed replacement", "Douglasie C24, frisch gehobelter Ersatz", 0.9, 0.55, 470, 0.12, 1600, 1.1e10, 2.4e7, "literature", "en-338"),
        m("robinia-weathered", "Robinia D40, weathered round posts", "Robinie D40, verwitterte Rundpfosten", 0.9, 0.72, 750, 0.17, 1600, 1.3e10, 4.0e7, "literature", "en-350"),
        m("stainless-scaled", "Stainless steel 1.4301, scratched with a limescale film", "Edelstahl 1.4301, verkratzt mit Kalkschleier", 0.35, 0.48, 7900, 15, 500, 2.0e11, 2.3e8, "literature", "en-10088-2"),
        m("polycarbonate-clouded", "Polycarbonate tube, scratched and clouded by limescale", "Polycarbonatrohr, verkratzt und kalkgetrübt", 0.9, 0.2, 1200, 0.2, 1200, 2.3e9, 6.0e7, "authored-estimate", "iso-10456-2007"),
        m("pe-grip", "Polyethylene crank grip, sun-faded", "Kurbelgriff aus Polyethylen, sonnengebleicht", 0.92, 0.7, 950, 0.4, 1900, 1.0e9, 2.5e7, "authored-estimate", "iso-10456-2007"),
        m("concrete-weathered", "Concrete C20/25, footings and basin plinth", "Beton C20/25, Fundamente und Beckensockel", 0.92, 0.68, 2300, 1.65, 1000, 3.0e10, 2.0e7, "literature", "iso-10456-2007"),
        {"id": "water", "title": tx("Water from the playground pump", "Wasser aus der Spielplatzpumpe"), "emissivity": 0.96, "solarAbsorptivity": 0.15, "densityKgM3": 1000, "conductivityWmK": 0.6, "specificHeatJKgK": 4186, "source": "literature", "reference": "incropera-2018"},
        {"id": "sand-wet", "title": tx("Playground sand, wet", "Spielsand, nass"), "emissivity": 0.95, "solarAbsorptivity": 0.8, "densityKgM3": 1900, "conductivityWmK": 2.0, "specificHeatJKgK": 1380, "source": "literature", "reference": "duffie-beckman-2013"},
    ]


def sections():
    """📏 Member sections of the load-bearing bones."""
    sec = lambda a, i, e, mat, en, de: {"areaM2": a, "secondMomentM4": i, "extremeFibreM": e, "material": mat, "title": tx(en, de)}
    d = 2 * POST_R
    ro, ri = TUBE[3], TUBE[2]
    rs = SHAFT[2]
    return {
        "frame": sec(round(2 * 0.04 * 0.24, 5), float(f"{2 * 0.04 * 0.24 ** 3 / 12:.4g}"), 0.12, "douglas-greyed", "Two aprons 40 x 240 mm, Douglas fir C24 (EN 338)", "Zwei Zargen 40 x 240 mm, Douglasie C24 (EN 338)"),
        "post": sec(round(math.pi * d * d / 4, 6), float(f"{math.pi * d ** 4 / 64:.4g}"), POST_R, "robinia-weathered", "Round post 120 mm, Robinia D40 (EN 338, EN 350)", "Rundpfosten 120 mm, Robinie D40 (EN 338, EN 350)"),
        "screw-mount": sec(float(f"{math.pi * (ro ** 2 - ri ** 2):.4g}"), float(f"{math.pi / 4 * (ro ** 4 - ri ** 4):.4g}"), ro, "polycarbonate-clouded", "Polycarbonate tube 160 x 5 mm", "Polycarbonatrohr 160 x 5 mm"),
        "screw": sec(float(f"{math.pi * (rs ** 2 - (rs - 0.002) ** 2):.4g}"), float(f"{math.pi / 4 * (rs ** 4 - (rs - 0.002) ** 4):.4g}"), rs, "stainless-scaled", "Stainless tube 42.4 x 2.0 (EN ISO 1127)", "Edelstahlrohr 42,4 x 2,0 (EN ISO 1127)"),
    }
