"""🧯 Authoritative generator of the staffage asset `used-underfloor-heating-manifold`.

A used stainless 8-circuit underfloor heating manifold (1" bars, flow meters on the supply bar, thermal actuators on the
return bar, 1" ball valves, two brackets, cut and capped PE-X 17 x 2 stubs) in its surface-mounted sheet-steel cabinet
780 x 705 x 125 mm without its door, lying on its back in an open-top spruce crate; lifted out by hand; reinstalled on a
wall and running at 35/28 degC. Every hand-chosen number is stated here; running the script rewrites `🔣️.json` next to it.

@see ../../brief.md
@see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
"""

import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SLUG = "used-underfloor-heating-manifold"
G = 9.81

CH, CW, CD, TS = 0.705, 0.78, 0.125, 0.001
FL = 0.025
PAD = 0.075
X0 = PAD
SKID, CR_L, CR_W, BOARD, WALL_H = 0.08, CH + 2 * PAD, CW + 2 * PAD, 0.022, 0.16
CHOCK_L, CHOCK_H = 0.08, 0.08
FLOOR_TOP = SKID + BOARD
WALL_TOP = FLOOR_TOP + WALL_H
SKID_Z = (-(CR_W / 2 - SKID / 2), 0.0, CR_W / 2 - SKID / 2)
CHOCK_W = (-0.25, 0.25)
CHOCK_U = (0.18, 0.53)
MAN_U0 = 0.17
SUP_U, SUP_V, RET_U, RET_V, BAR_R = 0.22, 0.080, 0.40, 0.045, 0.0175
BAR_W0, BAR_W1 = -0.29, 0.25
PITCH, N_CIRC = 0.05, 8
CIRC_W = [round(-0.175 + i * PITCH, 4) for i in range(N_CIRC)]
NUT_R, NUT_L = 0.015, 0.03
PEX_R, STUB_END = 0.0085, 0.62
FM_R, FM_TOP = 0.0125, 0.12
ACT_R, ACT_TOP, NECK_U = 0.018, 0.3325, 0.3725
BV_R, BV_W1 = 0.022, 0.31
PIPE_R, PIPE_W, PIPE_END = 0.014, 0.325, 0.64
BRACKET_W = (-0.225, 0.225)
REST_LIFT, REINSTALL_DY, CRATE_DX = 0.30, 1.003, 0.45
RHO_STEEL, RHO_SS, RHO_SPRUCE, RHO_PEX = 7850.0, 7900.0, 450.0, 940.0


def t(en, de):
    """📝 One bilingual text."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪 A number with a decimal comma."""
    return str(x).replace(".", ",")


def r6(x):
    """🎯 Round to micrometres."""
    return round(x, 6)


SKIDS_M3 = 3 * SKID * SKID * CR_L
FLOOR_M3 = (CR_L - 2 * BOARD) * (CR_W - 2 * BOARD) * BOARD
WALLS_M3 = 2 * BOARD * (WALL_TOP - SKID) * CR_W + 2 * BOARD * (WALL_TOP - SKID) * (CR_L - 2 * BOARD) + 8 * (PAD - BOARD) * CHOCK_L * CHOCK_H
CRATE_KG = round((SKIDS_M3 + FLOOR_M3 + WALLS_M3) * RHO_SPRUCE + 0.2, 2)
SHEET_M2 = CH * CW + 2 * CW * (CD - TS) + 2 * (CH - 2 * TS) * (CD - TS) + 2 * CW * FL + 2 * (CH - 2 * TS - 2 * FL) * FL
TERMINAL_KG = 0.35
CABINET_KG = round(SHEET_M2 * TS * RHO_STEEL + TERMINAL_KG, 2)
BAR_KG_M = math.pi * (2 * BAR_R - 0.0015) * 0.0015 * RHO_SS
PEX_KG_M = math.pi * (2 * PEX_R - 0.002) * 0.002 * RHO_PEX
STUB_M = N_CIRC * (STUB_END - (SUP_U + BAR_R + NUT_L)) + N_CIRC * (STUB_END - (RET_U + BAR_R + NUT_L))
PIPE_M = (PIPE_END - SUP_U) + (PIPE_END - RET_U)
MANIFOLD_PARTS = {
    "bars": round(2 * (BAR_W1 - BAR_W0) * BAR_KG_M, 3),
    "eurocone-outlets": round(2 * N_CIRC * 0.06, 3),
    "pex-stubs": round(STUB_M * PEX_KG_M, 3),
    "flow-meters": round(N_CIRC * 0.09, 3),
    "valve-inserts-actuators": round(N_CIRC * 0.14, 3),
    "ball-valves": 0.84,
    "connection-pipes": round(PIPE_M * 1.11 + 4 * 0.05, 3),
    "end-pieces-vents": 0.24,
    "brackets": 0.6,
}
MANIFOLD_KG = round(sum(MANIFOLD_PARTS.values()), 2)
MASS = round(CRATE_KG + CABINET_KG + MANIFOLD_KG, 2)
WATER_L = round((2 * math.pi * (BAR_R - 0.0015) ** 2 * (BAR_W1 - BAR_W0) + math.pi * (PEX_R - 0.002) ** 2 * STUB_M + math.pi * (PIPE_R - 0.0015) ** 2 * PIPE_M) * 1000 + N_CIRC * 0.025, 2)
HEIGHT, WIDTH, DEPTH = r6(WALL_TOP), CR_L, CR_W


def part(id_, bone, kind, material, tone, region, geo, struct=None, **kw):
    """🧩 One part in the frame of its bone: box (centre, size) or cylinder (axis, centre of the other two, a0, a1, r)."""
    p = {"id": id_, "bone": bone, "kind": kind, "material": material, "tone": tone, "region": region, "geo": geo, "struct": struct}
    p.update(kw)
    return p


def cab_box(id_, u0, u1, v0, v1, w0, w1, material, tone, region, bone="cabinet", struct=None, **kw):
    """📦 A box given by its ranges in cabinet coordinates (shifted into the manifold bone when asked)."""
    du = MAN_U0 if bone == "manifold" else 0.0
    return part(id_, bone, "box", material, tone, region, {"c": [r6((u0 + u1) / 2 - du), r6((v0 + v1) / 2), r6((w0 + w1) / 2)], "s": [r6(u1 - u0), r6(v1 - v0), r6(w1 - w0)]}, struct, **kw)


def cab_cyl(id_, axis, a0, a1, c1, c2, r, material, tone, region, bone="manifold", struct=None, **kw):
    """🥫 A cylinder in cabinet coordinates: axis u (c1 = v, c2 = w), v (c1 = u, c2 = w) or w (c1 = u, c2 = v)."""
    du = MAN_U0 if bone == "manifold" else 0.0
    if axis == "u":
        a0, a1 = a0 - du, a1 - du
    else:
        c1 = c1 - du
    return part(id_, bone, "cyl" + axis, material, tone, region, {"a0": r6(a0), "a1": r6(a1), "c1": r6(c1), "c2": r6(c2), "r": r6(r)}, struct, **kw)


def crate_box(id_, x0, x1, y0, y1, z0, z1, tone=0.5, struct=None, **kw):
    """🪵 A crate box in crate (world rest) coordinates."""
    return part(id_, "crate", "box", "spruce-crate", tone, "crate", {"c": [r6((x0 + x1) / 2), r6((y0 + y1) / 2), r6((z0 + z1) / 2)], "s": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)]}, struct, **kw)


def crate_parts(scale):
    """🪵 The crate of one scale: skids, floor, four walls and the timber chocks that hold the cabinet."""
    out = []
    b = {50: BOARD, 100: BOARD, 200: 0.03, 500: PAD}[scale]
    if scale == 500:
        out.append(crate_box("crate-base", 0, CR_L, 0, FLOOR_TOP, -CR_W / 2, CR_W / 2, 0.48, "member"))
    else:
        for k, z in enumerate(SKID_Z):
            out.append(crate_box(f"crate-skid-{k + 1}", 0, CR_L, 0, SKID, z - SKID / 2, z + SKID / 2, 0.45, "member"))
        out.append(crate_box("crate-floor", b, CR_L - b, FLOOR_TOP - b, FLOOR_TOP, -CR_W / 2 + b, CR_W / 2 - b, 0.55, "member"))
    y0 = FLOOR_TOP if scale == 500 else SKID
    out.append(crate_box("crate-wall-head", 0, b, y0, WALL_TOP, -CR_W / 2, CR_W / 2))
    out.append(crate_box("crate-wall-foot", CR_L - b, CR_L, y0, WALL_TOP, -CR_W / 2, CR_W / 2))
    out.append(crate_box("crate-wall-left", b, CR_L - b, y0, WALL_TOP, CR_W / 2 - b, CR_W / 2))
    out.append(crate_box("crate-wall-right", b, CR_L - b, y0, WALL_TOP, -CR_W / 2, -CR_W / 2 + b))
    if scale == 500:
        return out
    top = FLOOR_TOP + CHOCK_H
    for k, w in enumerate(CHOCK_W, 1):
        out.append(crate_box(f"chock-head-{k}", b, X0, FLOOR_TOP, top, w - CHOCK_L / 2, w + CHOCK_L / 2, 0.6))
        out.append(crate_box(f"chock-foot-{k}", X0 + CH, CR_L - b, FLOOR_TOP, top, w - CHOCK_L / 2, w + CHOCK_L / 2, 0.6))
    for k, u in enumerate(CHOCK_U, 1):
        x = X0 + u
        out.append(crate_box(f"chock-left-{k}", x - CHOCK_L / 2, x + CHOCK_L / 2, FLOOR_TOP, top, CW / 2, CR_W / 2 - b, 0.6))
        out.append(crate_box(f"chock-right-{k}", x - CHOCK_L / 2, x + CHOCK_L / 2, FLOOR_TOP, top, -CR_W / 2 + b, -CW / 2, 0.6))
    return out


def cabinet_parts(scale):
    """🗄️ The sheet-steel cabinet of one scale (door missing, front frame flange kept)."""
    m, tn, rg = "cabinet-steel", 0.86, "cabinet"
    out = [
        cab_box("cabinet-back", 0, CH, 0, TS, -CW / 2, CW / 2, m, 0.8, rg, struct="member", sheet=True),
        cab_box("cabinet-top", 0, TS, TS, CD, -CW / 2, CW / 2, m, tn, rg, sheet=True),
        cab_box("cabinet-bottom", CH - TS, CH, TS, CD, -CW / 2, CW / 2, m, tn, rg, sheet=True),
        cab_box("cabinet-side-left", TS, CH - TS, TS, CD, CW / 2 - TS, CW / 2, m, tn, rg, sheet=True),
        cab_box("cabinet-side-right", TS, CH - TS, TS, CD, -CW / 2, -CW / 2 + TS, m, tn, rg, sheet=True),
    ]
    if scale in (50, 100):
        out += [
            cab_box("flange-top", TS, TS + FL, CD - TS, CD, -CW / 2 + TS, CW / 2 - TS, m, tn, rg, sheet=True),
            cab_box("flange-bottom", CH - TS - FL, CH - TS, CD - TS, CD, -CW / 2 + TS, CW / 2 - TS, m, tn, rg, sheet=True),
            cab_box("flange-left", TS + FL, CH - TS - FL, CD - TS, CD, CW / 2 - TS - FL, CW / 2 - TS, m, tn, rg, sheet=True),
            cab_box("flange-right", TS + FL, CH - TS - FL, CD - TS, CD, -CW / 2 + TS, -CW / 2 + TS + FL, m, tn, rg, sheet=True),
        ]
    if scale != 500:
        out.append(cab_box("terminal-box", 0.25, 0.45, TS, 0.04, -0.37, -0.30, "pa-white", 0.92, "actuators"))
    return out


def manifold_parts(scale):
    """🔧 The manifold of one scale, in cabinet coordinates shifted into the manifold bone."""
    out = []
    if scale == 500:
        out.append(cab_box("meter-slab", FM_TOP, SUP_U - BAR_R, 0.035, 0.11, -0.29, 0.34, "psu-clear", 0.7, "flow-meters", bone="manifold"))
        out.append(cab_box("supply-slab", SUP_U - BAR_R, 0.29, TS, 0.11, -0.29, 0.34, "stainless-steel", 0.62, "supply", bone="manifold", struct="member"))
        out.append(cab_box("actuator-slab", 0.29, RET_U - BAR_R, TS, 0.11, -0.29, 0.34, "pa-white", 0.92, "actuators", bone="manifold"))
        out.append(cab_box("return-slab", RET_U - BAR_R, STUB_END, TS, 0.11, -0.29, 0.34, "stainless-steel", 0.58, "return", bone="manifold", struct="member"))
        return out
    bv = 0.031 if scale == 200 else 0.0275
    for k, w in enumerate(BRACKET_W):
        out.append(cab_box(f"bracket-{k + 1}", MAN_U0, 0.45, TS, bv, w - 0.015, w + 0.015, "galvanised-steel", 0.55, "cabinet", bone="manifold", struct="member"))
        out.append(cab_box(f"standoff-{k + 1}", SUP_U - 0.02, SUP_U + 0.02, bv, SUP_V - BAR_R, w - 0.015, w + 0.015, "galvanised-steel", 0.55, "cabinet", bone="manifold", struct="joint"))
    out.append(cab_cyl("supply-bar", "w", BAR_W0, BAR_W1, SUP_U, SUP_V, BAR_R, "stainless-steel", 0.62, "supply", struct="member"))
    out.append(cab_cyl("return-bar", "w", BAR_W0, BAR_W1, RET_U, RET_V, BAR_R, "stainless-steel", 0.58, "return", struct="member"))
    for name, bu, bvv, reg in (("supply", SUP_U, SUP_V, "supply"), ("return", RET_U, RET_V, "return")):
        out.append(cab_cyl(f"{name}-ball-valve", "w", BAR_W1, BV_W1, bu, bvv, BV_R, "brass-nickel", 0.7, reg))
        pr = 0.015 if scale == 200 else PIPE_R
        if scale == 200:
            out.append(cab_cyl(f"{name}-pipe", "u", bu - pr, PIPE_END + 0.02, bvv, PIPE_W, pr, "copper-oxidised", 0.4, reg))
        else:
            out.append(cab_cyl(f"{name}-elbow", "w", BV_W1, PIPE_W + PIPE_R, bu, bvv, PIPE_R, "copper-oxidised", 0.4, reg))
            out.append(cab_cyl(f"{name}-pipe", "u", bu, PIPE_END, bvv, PIPE_W, PIPE_R, "copper-oxidised", 0.4, reg))
            out.append(cab_cyl(f"{name}-cap", "u", PIPE_END, PIPE_END + 0.02, bvv, PIPE_W, 0.016, "brass-nickel", 0.7, reg))
            hv = 0.008 if scale == 50 else 0.015
            top = bvv + BV_R
            out.append(cab_box(f"{name}-handle", bu - 0.04, bu + 0.04, top, top + hv, 0.27, 0.29, "lever-red" if name == "supply" else "lever-blue", 0.35 if name == "supply" else 0.25, reg, bone="manifold"))
            out.append(cab_cyl(f"{name}-vent", "u", bu - BAR_R - 0.03, bu - BAR_R, bvv, -0.275, 0.008, "brass-nickel", 0.7, reg))
    if scale == 200:
        out.append(cab_box("meter-row", FM_TOP, SUP_U - BAR_R, SUP_V - BAR_R, SUP_V + BAR_R, CIRC_W[0] - FM_R, CIRC_W[-1] + FM_R, "psu-clear", 0.7, "flow-meters", bone="manifold", envelope=True))
        out.append(cab_box("actuator-row", ACT_TOP, RET_U - BAR_R, RET_V - ACT_R, RET_V + ACT_R, CIRC_W[0] - ACT_R, CIRC_W[-1] + ACT_R, "pa-white", 0.92, "actuators", bone="manifold", envelope=True))
        return out
    for i, w in enumerate(CIRC_W, 1):
        for name, bu, bvv, reg in (("supply", SUP_U, SUP_V, "supply"), ("return", RET_U, RET_V, "return")):
            n0 = bu + BAR_R
            out.append(cab_cyl(f"{name}-outlet-{i}", "u", n0, n0 + NUT_L, bvv, w, NUT_R, "brass-nickel", 0.68, reg))
            out.append(cab_cyl(f"{name}-stub-{i}", "u", n0 + NUT_L, STUB_END, bvv, w, PEX_R, "pex-white", 0.95, reg))
        out.append(cab_cyl(f"flow-meter-{i}", "u", FM_TOP, SUP_U - BAR_R, SUP_V, w, FM_R, "psu-clear", 0.7, "flow-meters"))
        if scale == 50:
            out.append(cab_cyl(f"flow-meter-cap-{i}", "u", FM_TOP - 0.01, FM_TOP, SUP_V, w, 0.014, "lever-red", 0.35, "flow-meters"))
            out.append(cab_cyl(f"valve-neck-{i}", "u", NECK_U, RET_U - BAR_R, RET_V, w, 0.011, "brass-nickel", 0.68, "return"))
            out.append(cab_cyl(f"actuator-{i}", "u", ACT_TOP, NECK_U, RET_V, w, ACT_R, "pa-white", 0.92, "actuators"))
        else:
            out.append(cab_cyl(f"actuator-{i}", "u", ACT_TOP, RET_U - BAR_R, RET_V, w, ACT_R, "pa-white", 0.92, "actuators"))
    return out


def parts(scale):
    """🧱 All parts of one scale."""
    return crate_parts(scale) + cabinet_parts(scale) + manifold_parts(scale)


BONE_REST = {"crate": (0.0, 0.0), "cabinet": (X0, FLOOR_TOP), "manifold": (X0 + MAN_U0, FLOOR_TOP)}


def solid_of(p):
    """🧊 The body solid of a part."""
    s = {"id": p["id"], "kind": "box", "bone": p["bone"], "material": p["material"], "role": "body", "tone": p["tone"]}
    g = p["geo"]
    if p["kind"] == "box":
        s.update(size=g["s"], transform={"translation": g["c"]})
    else:
        ax = p["kind"][-1]
        if ax == "u":
            axis, origin = [1, 0, 0], [0, g["c1"], g["c2"]]
        elif ax == "v":
            axis, origin = [0, 1, 0], [g["c1"], 0, g["c2"]]
        else:
            axis, origin = [0, 0, 1], [g["c1"], g["c2"], 0]
        s.update(kind="revolve", profile=[[g["a0"], g["r"]], [g["a1"], g["r"]]], axis=axis, origin=[r6(x) for x in origin])
    for k in ("sheet", "slender", "envelope"):
        if p.get(k):
            s[k] = True
    return s


def extent(p):
    """📏 Ranges (u, v, w) of a part in its bone frame."""
    g = p["geo"]
    if p["kind"] == "box":
        return [(g["c"][i] - g["s"][i] / 2, g["c"][i] + g["s"][i] / 2) for i in range(3)]
    ax, r = p["kind"][-1], g["r"]
    a = (g["a0"], g["a1"])
    if ax == "u":
        return [a, (g["c1"] - r, g["c1"] + r), (g["c2"] - r, g["c2"] + r)]
    if ax == "v":
        return [(g["c1"] - r, g["c1"] + r), a, (g["c2"] - r, g["c2"] + r)]
    return [(g["c1"] - r, g["c1"] + r), (g["c2"] - r, g["c2"] + r), a]


def shape(id_, bone, role, z, filled, tone, geom, region=None, material=None, solid=None, regions=None):
    """✏️ One 2D shape."""
    s = {"id": id_, "bone": bone, "role": role, "z": r6(z), "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    s.update(geom)
    if solid is not None:
        s["solid"] = {"kind": "extruded", "sweep": "w", "z0": r6(solid[0]), "z1": r6(solid[1])}
    return s


def rect(x, y, w, h):
    """▭ A rect geometry."""
    return {"kind": "rect", "x": r6(x), "y": r6(y), "w": r6(w), "h": r6(h)}


def circ(cx, cy, rad):
    """⚪ An ellipse geometry of equal radii."""
    return {"kind": "ellipse", "cx": r6(cx), "cy": r6(cy), "rx": r6(rad), "ry": r6(rad)}


def poly(points):
    """〰️ An open polyline geometry."""
    return {"kind": "poly", "points": [[r6(a), r6(b)] for a, b in points], "closed": False}


def profile_geom(p):
    """🔲 The (u, v) profile of a part as a 2D geometry."""
    (u0, u1), (v0, v1), _ = extent(p)
    if p["kind"] == "cylw":
        return circ(p["geo"]["c1"], p["geo"]["c2"], p["geo"]["r"])
    return rect(u0, v0, u1 - u0, v1 - v0)


def body_shapes(scale, view):
    """📐 The filled body shapes of one view, one per part, from the same numbers as the solids."""
    out = []
    if scale == 500:
        g = rect(FM_TOP - MAN_U0, TS, STUB_END - FM_TOP, 0.11 - TS)
        z = 5 - BAR_W0 if view == "elevation" else 5 + (BONE_REST["manifold"][0] + STUB_END - MAN_U0 if view == "front" else FLOOR_TOP + 0.11)
        out.append(shape("manifold-block", "manifold", "body", z, True, 0.7, g, material="stainless-steel", solid=None if view == "elevation" else (BAR_W0, 0.34), regions=["flow-meters", "supply", "actuators", "return"]))
    for p in parts(scale):
        if scale == 500 and p["bone"] == "manifold":
            continue
        (u0, u1), (v0, v1), (w0, w1) = extent(p)
        bx, by = BONE_REST[p["bone"]]
        if view == "elevation":
            z = 5 - w0
            out.append(shape(p["id"], p["bone"], "body", z, True, p["tone"], profile_geom(p), p["region"], p["material"]))
        else:
            z = 5 + (bx + u1 if view == "front" else by + v1)
            out.append(shape(p["id"], p["bone"], "body", z, True, p["tone"], profile_geom(p), p["region"], p["material"], solid=(w0, w1)))
    return out


def details(scale, view):
    """🖋️ Traces of use and dismantling as detail and texture strokes (1:50 and 1:100)."""
    out = []
    if scale not in (50, 100):
        return out
    mid = FLOOR_TOP + WALL_H / 2
    if view == "elevation":
        out.append(shape("crate-wall-joint", "crate", "texture", 5.44, False, 0.3, poly([(0.0, mid), (CR_L, mid)]), "crate"))
        out.append(shape("crate-lot-tag", "crate", "detail", 5.45, True, 0.95, rect(0.30, FLOOR_TOP + 0.095, 0.148, 0.05), "crate"))
        out.append(shape("crate-ispm-stamp", "crate", "detail", 5.45, False, 0.25, rect(0.56, FLOOR_TOP + 0.02, 0.09, 0.05), "crate"))
        out.append(shape("cabinet-type-plate", "cabinet", "detail", 5.395, True, 0.95, rect(0.55, 0.03, 0.08, 0.05), "cabinet"))
        out.append(shape("cabinet-scuff", "cabinet", "texture", 5.395, False, 0.6, poly([(0.08, 0.10), (0.16, 0.095), (0.22, 0.102)]), "cabinet"))
        if scale == 50:
            out.append(shape("crate-nail-row", "crate", "texture", 5.44, False, 0.2, poly([(0.011, FLOOR_TOP + 0.01), (0.011, WALL_TOP - 0.01)]), "crate"))
            out.append(shape("crate-nail-row-2", "crate", "texture", 5.44, False, 0.2, poly([(CR_L - 0.011, FLOOR_TOP + 0.01), (CR_L - 0.011, WALL_TOP - 0.01)]), "crate"))
    if view == "front":
        out.append(shape("crate-head-joint", "crate", "texture", 6.0, False, 0.3, poly([(-CR_W / 2, mid), (CR_W / 2, mid)]), "crate"))
    if view == "plan":
        boards = 6
        for k in range(1, boards):
            x = CR_L * k / boards
            out.append(shape(f"floor-board-joint-{k}", "crate", "texture", 5.1025, False, 0.3, poly([(x, -CR_W / 2 + BOARD), (x, CR_W / 2 - BOARD)]), "crate"))
        for name, bu in (("supply", SUP_U), ("return", RET_U)):
            out.append(shape(f"{name}-bar-limescale", "manifold", "texture", 5.2, False, 0.75, poly([(bu - MAN_U0 - 0.004, BAR_W0 + 0.02), (bu - MAN_U0 - 0.006, -0.05), (bu - MAN_U0 - 0.004, BAR_W1 - 0.02)]), name))
        if scale == 50:
            for i, w in enumerate(CIRC_W, 1):
                out.append(shape(f"stub-cut-{i}", "manifold", "detail", 5.21, False, 0.1, poly([(STUB_END - MAN_U0, w - PEX_R), (STUB_END - MAN_U0, w + PEX_R)]), "supply"))
    return out


def view_shapes(scale, view):
    """🗺️ One view of one scale."""
    return body_shapes(scale, view) + details(scale, view)


PENS = {50: {"body": 0.25, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.2, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.2, "detail": 0.1, "texture": None, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}

DECISIONS = {
    50: {
        "silhouette": t("Side elevation from the right: the open-top spruce crate 855 x 262 mm (17.1 x 5.2 mm on paper) on its three 80 x 80 mm skids, the near side wall hiding the cabinet that lies on its back inside. Plan: the crate seen from above with the open cabinet 705 x 780 mm and the whole manifold in it: two 1 inch stainless bars, eight flow meters, eight actuators, the red and blue ball valves, the copper connection pipes and sixteen cut PE-X stubs. Front: the head wall of the crate with the skids below.", "Seitenansicht von rechts: die oben offene Fichtenkiste 855 x 262 mm (17,1 x 5,2 mm auf Papier) auf ihren drei Kufen 80 x 80 mm; die nahe Seitenwand verdeckt den Schrank, der auf dem Rücken darin liegt. Draufsicht: die Kiste von oben mit dem offenen Schrank 705 x 780 mm und dem ganzen Verteiler darin: zwei Edelstahlbalken 1 Zoll, acht Durchflussanzeiger, acht Stellantriebe, die roten und blauen Kugelhähne, die Kupfer-Anschlussrohre und sechzehn abgeschnittene PE-X-Stummel. Vorderansicht: die Kopfwand der Kiste mit den Kufen darunter."),
        "retained": t("Every part at its true size: the 25 mm front flange of the cabinet whose door is missing, eurocone nuts, flow meter caps, valve necks under the actuators, 8 mm lever handles, air vents, the terminal box of the actuator wiring, the brackets with their stand-offs, the eight timber chocks that hold the cabinet in the crate; traces of use: limescale streaks on the bars, cut faces of the PE-X stubs, a scuff on the cabinet side, the type plate, the lot tag and the ISPM 15 stamp of the crate, nail rows and board joints.", "Jedes Teil in wahrer Größe: der 25-mm-Frontflansch des Schranks, dessen Tür fehlt, Eurokonus-Muttern, Kappen der Durchflussanzeiger, Ventilhälse unter den Stellantrieben, Hebelgriffe 8 mm, Entlüfter, die Klemmleiste der Stellantriebsverdrahtung, die Halter mit ihren Abstandsstücken, die acht Holzklötze, die den Schrank in der Kiste halten; Gebrauchsspuren: Kalkschlieren auf den Balken, Schnittflächen der PE-X-Stummel, eine Schramme an der Schrankseite, das Typenschild, der Losanhänger und der ISPM-15-Stempel der Kiste, Nagelreihen und Brettfugen."),
        "merged": t("Nothing is merged; each solid has its own shape in all three views.", "Nichts ist zusammengefasst; jeder Körper hat in allen drei Ansichten seine eigene Form."),
        "omitted": t("Threads, hexagon flats, the scale of the flow meters, the actuator cables (cut at the terminal box) and the screws of the brackets.", "Gewinde, Sechskantflächen, die Skala der Durchflussanzeiger, die Kabel der Stellantriebe (an der Klemmleiste abgeschnitten) und die Schrauben der Halter."),
        "thinElements": t("The 1 mm cabinet sheets are drawn at their true section; in the views where they stand edge-on the outline pen closes them.", "Die Schrankbleche von 1 mm sind in wahrem Querschnitt gezeichnet; in den Ansichten, in denen sie hochkant stehen, schließt sie die Konturfeder."),
        "hatching": t("No hatch; materials read by tone: grey-white powder coat, light stainless, white PE-X and actuators, red and blue levers, warm weathered spruce.", "Keine Schraffur; Materialien über den Ton: grauweiße Pulverbeschichtung, helles Edelstahl, weißes PE-X und weiße Stellantriebe, rote und blaue Hebel, warmes vergrautes Fichtenholz."),
        "styles": t("Body 0.25 mm, details 0.18 mm, textures 0.1 mm.", "Körper 0,25 mm, Details 0,18 mm, Texturen 0,1 mm."),
        "overlays": t("Regions crate, cabinet, supply, return, flow meters and actuators carry the thermal and structural overlays.", "Die Regionen Kiste, Schrank, Vorlauf, Rücklauf, Durchflussanzeiger und Stellantriebe tragen die thermischen und statischen Überlagerungen."),
    },
    100: {
        "silhouette": t("The same crate, cabinet and manifold at 8.6 x 2.6 mm; in plan the bars, the rows of flow meters and actuators and the sixteen stubs stay separate parts.", "Dieselbe Kiste, derselbe Schrank und Verteiler mit 8,6 x 2,6 mm; in der Draufsicht bleiben Balken, die Reihen der Durchflussanzeiger und Stellantriebe und die sechzehn Stummel getrennte Teile."),
        "retained": t("Front flange, eurocone nuts, PE-X stubs, flow meters, actuators, ball valves with their levers, vents, terminal box, brackets; the board joints, the lot tag, the type plate and the limescale streaks.", "Frontflansch, Eurokonus-Muttern, PE-X-Stummel, Durchflussanzeiger, Stellantriebe, Kugelhähne mit ihren Hebeln, Entlüfter, Klemmleiste, Halter; die Brettfugen, der Losanhänger, das Typenschild und die Kalkschlieren."),
        "merged": t("Actuator and valve neck are one cylinder; the flow meter caps belong to the flow meters.", "Stellantrieb und Ventilhals sind ein Zylinder; die Kappen gehören zu den Durchflussanzeigern."),
        "omitted": t("The cut faces of the stubs and the nail rows.", "Die Schnittflächen der Stummel und die Nagelreihen."),
        "thinElements": t("Lever handles are drawn 15 mm thick (8 mm in reality) so they stay one feature; the cabinet sheets keep their true 1 mm.", "Die Hebelgriffe sind 15 mm dick gezeichnet (8 mm in Wirklichkeit), damit sie ein Merkmal bleiben; die Schrankbleche behalten ihre wahren 1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Body 0.2 mm, details 0.13 mm, textures 0.1 mm.", "Körper 0,2 mm, Details 0,13 mm, Texturen 0,1 mm."),
        "overlays": t("Same regions as at 1:50.", "Dieselben Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": t("Crate 4.3 x 1.3 mm with skids, floor and walls at 30 mm; in plan the cabinet with the two bars, a flow meter band and an actuator band, the ball valves and the two connection pipes.", "Kiste 4,3 x 1,3 mm mit Kufen, Boden und Wänden zu 30 mm; in der Draufsicht der Schrank mit den beiden Balken, einem Band der Durchflussanzeiger und einem Band der Stellantriebe, den Kugelhähnen und den beiden Anschlussrohren."),
        "retained": t("The two bars at their true 35 mm, ball valves, connection pipes, brackets with stand-offs and the terminal box.", "Die beiden Balken mit ihren wahren 35 mm, Kugelhähne, Anschlussrohre, Halter mit Abstandsstücken und die Klemmleiste."),
        "merged": t("The eight flow meters and the eight actuators are each one typified band (envelope); elbow, pipe and cap are one pipe of 30 mm; crate boards are 30 mm thick inwards.", "Die acht Durchflussanzeiger und die acht Stellantriebe sind je ein typisiertes Band (Hülle); Bogen, Rohr und Kappe sind ein Rohr von 30 mm; die Kistenbretter sind nach innen 30 mm dick."),
        "omitted": t("PE-X stubs and eurocone nuts (17 and 30 mm, 0.09 and 0.15 mm on paper), front flange, levers, vents and every trace of use.", "PE-X-Stummel und Eurokonus-Muttern (17 und 30 mm, 0,09 und 0,15 mm auf Papier), Frontflansch, Hebel, Entlüfter und alle Gebrauchsspuren."),
        "thinElements": t("Cabinet sheets stay 1 mm sheets.", "Die Schrankbleche bleiben Bleche von 1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Body 0.2 mm, details 0.1 mm.", "Körper 0,2 mm, Details 0,1 mm."),
        "overlays": t("Same regions; the stubs are part of the supply and return regions through the bars.", "Dieselben Regionen; die Stummel sind über die Balken Teil der Vorlauf- und Rücklaufregion."),
    },
    500: {
        "silhouette": t("A crate 1.7 x 0.5 mm whose 75 mm walls take in the boards and the chocks around the cabinet; in plan the cabinet with four bands of the manifold: flow meters, supply bar, actuators, return bar with its stubs.", "Eine Kiste 1,7 x 0,5 mm, deren Wände von 75 mm die Bretter und die Klötze um den Schrank aufnehmen; in der Draufsicht der Schrank mit vier Bändern des Verteilers: Durchflussanzeiger, Vorlaufbalken, Stellantriebe, Rücklaufbalken mit seinen Stummeln."),
        "retained": t("Overall size, the open crate and the open cabinet.", "Gesamtgröße, die offene Kiste und der offene Schrank."),
        "merged": t("Skids and floor are one base block; each crate wall and its chocks are one 75 mm wall that touches the cabinet; the manifold is four slabs, one per thermal region, drawn in the views as one block across the bars.", "Kufen und Boden sind ein Sockelblock; jede Kistenwand und ihre Klötze sind eine Wand von 75 mm, die den Schrank berührt; der Verteiler sind vier Scheiben, eine je thermischer Region, in den Ansichten als ein Block über die Balken gezeichnet."),
        "omitted": t("All single parts of the manifold, the terminal box and every trace of use.", "Alle Einzelteile des Verteilers, die Klemmleiste und alle Gebrauchsspuren."),
        "thinElements": t("Cabinet sheets stay 1 mm sheets.", "Die Schrankbleche bleiben Bleche von 1 mm."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Body 0.2 mm.", "Körper 0,2 mm."),
        "overlays": t("Each manifold slab is one region cell.", "Jede Verteilerscheibe ist eine Regionszelle."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t("Body: the crate as three skids, floor, four 22 mm walls and eight timber chocks 53 x 80 x 80 mm between walls and cabinet; the cabinet as five 1 mm sheets (sheet) with the four strips of its 25 mm front flange; the manifold part by part: bars as cylinders 35 mm, brackets and stand-offs as boxes, every outlet as eurocone nut and PE-X stub, flow meters with caps, valve necks with actuators, ball valves with levers, elbows, pipes, caps and vents.", "Körper: die Kiste aus drei Kufen, Boden, vier Wänden zu 22 mm und acht Holzklötzen 53 x 80 x 80 mm zwischen Wänden und Schrank; der Schrank aus fünf Blechen von 1 mm (sheet) mit den vier Streifen seines 25-mm-Frontflanschs; der Verteiler Teil für Teil: Balken als Zylinder 35 mm, Halter und Abstandsstücke als Quader, jeder Abgang als Eurokonus-Mutter und PE-X-Stummel, Durchflussanzeiger mit Kappen, Ventilhälse mit Stellantrieben, Kugelhähne mit Hebeln, Bögen, Rohre, Kappen und Entlüfter."),
        t("Thermal: every body solid is a cell of its region (crate, cabinet, supply, return, flow meters, actuators).", "Thermisch: jeder Körper ist eine Zelle seiner Region (Kiste, Schrank, Vorlauf, Rücklauf, Durchflussanzeiger, Stellantriebe)."),
        t("Structure: skids and floor are the crate member, the back sheet the cabinet member, brackets and bars the manifold members, the stand-offs its joints.", "Tragwerk: Kufen und Boden sind das Kistenglied, das Rückblech das Schrankglied, Halter und Balken die Verteilerglieder, die Abstandsstücke seine Knoten."),
    ],
    100: [
        t("Body: as at 1:50 without the flow meter caps and valve necks; levers 15 mm thick; actuator one cylinder from bar to top.", "Körper: wie bei 1:50 ohne die Kappen der Durchflussanzeiger und die Ventilhälse; Hebel 15 mm dick; Stellantrieb ein Zylinder vom Balken bis oben."),
        t("Thermal: every body solid is a cell of its region.", "Thermisch: jeder Körper ist eine Zelle seiner Region."),
        t("Minimal: the structure keeps the same eleven members and joints as at 1:50, because each of them is the load path of its bone at this scale.", "Minimal: das Tragwerk behält dieselben elf Glieder und Knoten wie bei 1:50, weil jedes von ihnen bei diesem Maßstab der Lastweg seines Knochens ist."),
    ],
    200: [
        t("Body: crate boards 30 mm (inwards, the chocks 45 mm), no front flange; manifold with bars, ball valves, one 30 mm pipe per connection, brackets 30 mm deep, terminal box, and the flow meters and actuators as one typified band each (envelope); stubs, nuts, levers and vents left out below 30 mm.", "Körper: Kistenbretter 30 mm (nach innen, die Klötze 45 mm), kein Frontflansch; Verteiler mit Balken, Kugelhähnen, einem 30-mm-Rohr je Anschluss, Haltern 30 mm tief, Klemmleiste und den Durchflussanzeigern und Stellantrieben als je ein typisiertes Band (Hülle); Stummel, Muttern, Hebel und Entlüfter unter 30 mm entfallen."),
        t("Thermal: every body solid is a cell of its region.", "Thermisch: jeder Körper ist eine Zelle seiner Region."),
        t("Minimal: skids and floor, back sheet, brackets and bars stay the members and the stand-offs the joints, the least that still carries every bone.", "Minimal: Kufen und Boden, Rückblech, Halter und Balken bleiben die Glieder und die Abstandsstücke die Knoten, das Wenigste, das noch jeden Knochen trägt."),
    ],
    500: [
        t("Body: crate as base block and four 75 mm walls that take in the chocks and touch the cabinet; cabinet as five 1 mm sheets; manifold as four slabs across its width, one per region.", "Körper: Kiste als Sockelblock und vier Wände von 75 mm, die die Klötze aufnehmen und den Schrank berühren; Schrank als fünf Bleche von 1 mm; Verteiler als vier Scheiben über seine Breite, eine je Region."),
        t("Thermal: every body solid is a cell of its region.", "Thermisch: jeder Körper ist eine Zelle seiner Region."),
        t("Structure: base block, back sheet and the two bar slabs as members.", "Tragwerk: Sockelblock, Rückblech und die beiden Balkenscheiben als Glieder."),
    ],
}


def volume(scale):
    """🧊 The authored volume of one scale."""
    ps = parts(scale)
    body = [solid_of(p) for p in ps]
    thermal = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in ps]
    f = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}[scale]
    thermal.append({"id": "th-plume-source", "kind": "box", "bone": "cabinet", "material": "stainless-steel", "role": "body", "tone": 0.6, "size": [r6(f), r6(f), 0.4], "transform": {"translation": [r6(TS + f / 2), r6(0.11 - f / 2), 0]}, "region": "supply", "tags": ["manifold-plume"]})
    structure = [{"id": f"st-{p['id']}", "of": p["id"], "member": p["bone"], "part": p["struct"]} for p in ps if p["struct"]]
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}


def representation(scale):
    """🖨️ The authored representation of one scale."""
    lo, hi = RANGES[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[scale], "hatches": [], "geometry": {v: view_shapes(scale, v) for v in ("elevation", "front", "plan")}, "decisions": DECISIONS[scale]}


MATERIALS = [
    {"id": "cabinet-steel", "title": t("Galvanised sheet steel DX51D 1 mm, powder coated RAL 9016, scuffed", "Verzinktes Stahlblech DX51D 1 mm, pulverbeschichtet RAL 9016, verschrammt"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 140e6, "source": "literature", "reference": "en-10346"},
    {"id": "stainless-steel", "title": t("Stainless steel 1.4301, dull with limescale streaks", "Edelstahl 1.4301, matt mit Kalkschlieren"), "emissivity": 0.35, "solarAbsorptivity": 0.5, "densityKgM3": RHO_SS, "conductivityWmK": 15, "specificHeatJKgK": 500, "elasticModulusPa": 200e9, "strengthPa": 230e6, "source": "literature", "reference": "en-10088-2"},
    {"id": "brass-nickel", "title": t("Nickel-plated brass CW617N, tarnished", "Messing CW617N vernickelt, angelaufen"), "emissivity": 0.3, "solarAbsorptivity": 0.45, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "elasticModulusPa": 100e9, "source": "literature", "reference": "ashrae-2021"},
    {"id": "copper-oxidised", "title": t("Copper tube EN 1057 28 x 1.5 mm, oxidised", "Kupferrohr EN 1057 28 x 1,5 mm, oxidiert"), "emissivity": 0.6, "solarAbsorptivity": 0.65, "densityKgM3": 8900, "conductivityWmK": 380, "specificHeatJKgK": 385, "elasticModulusPa": 120e9, "source": "literature", "reference": "en-1057"},
    {"id": "pex-white", "title": t("PE-X tube 17 x 2 mm with oxygen barrier, yellowed", "PE-X-Rohr 17 x 2 mm mit Sauerstoffsperre, vergilbt"), "emissivity": 0.92, "solarAbsorptivity": 0.4, "densityKgM3": RHO_PEX, "conductivityWmK": 0.35, "specificHeatJKgK": 2300, "elasticModulusPa": 0.6e9, "source": "literature", "reference": "en-iso-15875"},
    {"id": "psu-clear", "title": t("Polysulfone sight glass of the flow meters, clouded", "Schauglas der Durchflussanzeiger aus Polysulfon, eingetrübt"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 1240, "conductivityWmK": 0.26, "specificHeatJKgK": 1100, "source": "literature", "reference": "en-iso-10456"},
    {"id": "pa-white", "title": t("White polyamide housing of actuators and terminal box, yellowed", "Weißes Polyamidgehäuse der Stellantriebe und der Klemmleiste, vergilbt"), "emissivity": 0.9, "solarAbsorptivity": 0.35, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1600, "source": "literature", "reference": "en-iso-10456"},
    {"id": "lever-red", "title": t("Red coated lever and cap (supply)", "Roter beschichteter Hebel und Kappe (Vorlauf)"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "source": "literature", "reference": "ashrae-2021"},
    {"id": "lever-blue", "title": t("Blue coated lever (return)", "Blauer beschichteter Hebel (Rücklauf)"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "source": "literature", "reference": "ashrae-2021"},
    {"id": "galvanised-steel", "title": t("Galvanised steel bracket, dull", "Verzinkter Stahlhalter, matt"), "emissivity": 0.28, "solarAbsorptivity": 0.65, "densityKgM3": RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210e9, "strengthPa": 220e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "spruce-crate", "title": t("Rough-sawn spruce, heat treated (ISPM 15), greyed", "Sägeraue Fichte, hitzebehandelt (ISPM 15), vergraut"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": RHO_SPRUCE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "strengthPa": 24e6, "source": "literature", "reference": "en-338"},
]

REFERENCES = [
    {"id": "en-1264", "citation": "EN 1264-1 to -5:2021 Water based surface embedded heating and cooling systems (design flow temperatures of screed heating, typically 35 degC flow at 7 K spread; circuit lengths up to about 100 m of 16 or 17 mm pipe; hydraulic balancing at the manifold)."},
    {"id": "din-18380", "citation": "DIN 18380:2019 VOB Part C, heating and central water heating systems (manifold installation, pressure test and flushing, handing over drained)."},
    {"id": "en-iso-15875", "citation": "EN ISO 15875-2:2004 Plastics piping systems for hot and cold water installations, crosslinked polyethylene PE-X, pipes (17 x 2.0 mm, density about 940 kg/m3)."},
    {"id": "en-10088-2", "citation": "EN 10088-2:2014 Stainless steels, sheet and strip (1.4301: Rp0.2 at least 230 MPa, density 7900 kg/m3, conductivity 15 W/mK)."},
    {"id": "en-10346", "citation": "EN 10346:2015 Continuously hot-dip coated steel flat products (DX51D+Z cabinet sheet; no specified yield strength, about 140 MPa taken)."},
    {"id": "en-1057", "citation": "EN 1057:2006+A1:2010 Seamless round copper tubes for water and gas (28 x 1.5 mm, 1.11 kg/m)."},
    {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes (C24: f_m,k 24 MPa, E0,mean 11 GPa; 450 kg/m3 taken for air-dry crate spruce)."},
    {"id": "ispm-15", "citation": "ISPM 15:2019 Regulation of wood packaging material in international trade (heat treatment HT and the stamp on two opposite sides of a crate)."},
    {"id": "manifold-datasheets", "citation": "Manufacturer data sheets of stainless underfloor heating manifolds 1 inch with flow meters and thermostatic valve inserts, 2 to 12 circuits (for example Oventrop Multidis SF, Uponor Vario S, HKV class): bar spacing about 200 mm, outlet pitch 50 mm, 3/4 inch eurocone outlets, overall length of an 8-circuit manifold about 540 mm without ball valves; values here are typical of the class, not of one product (authored-estimate)."},
    {"id": "cabinet-datasheets", "citation": "Manufacturer data sheets of surface-mounted manifold cabinets (Aufputz-Verteilerschrank) of galvanised powder-coated sheet steel: size for 8 to 9 circuits with flow meters about 780 x 705 x 125 mm, sheet 1 mm, removable door (authored-estimate)."},
    {"id": "actuator-datasheets", "citation": "Manufacturer data sheets of thermal actuators for manifold valve inserts, 230 V normally closed (for example the Moehlenhoff Alpha 5 class): about 1 W operating power, 4 mm stroke, housing about 44 x 50 mm (authored-estimate)."},
    {"id": "vdi-2035", "citation": "VDI 2035 Part 1:2021 Prevention of damage in water heating installations, scale formation (source of the limescale streaks on used manifolds)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design conductivity and heat capacity of plastics and timber)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities: painted surfaces 0.9, oxidised copper about 0.6, tarnished nickel about 0.3, dull stainless about 0.35, dull galvanised steel about 0.28; new bright metal is much lower (0.05 to 0.15)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."},
]

BONES = [
    {"id": "yard", "parent": None, "title": t("Storage floor of the component exchange", "Lagerboden der Bauteilbörse"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
    {"id": "crate", "parent": "yard", "title": t("Open-top spruce crate (slides aside when the manifold is reinstalled)", "Oben offene Fichtenkiste (wird beim Wiedereinbau beiseitegeschoben)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": CR_L, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.6}, "mass": {"kg": CRATE_KG, "comX": r6(CR_L / 2), "comY": 0.095, "distribution": "point", "source": "literature", "reference": "en-338"}, "section": {"areaM2": r6(SKID * SKID), "secondMomentM4": round(SKID ** 4 / 12, 12), "extremeFibreM": SKID / 2, "material": "spruce-crate", "title": t("Skid 80 x 80 mm spruce", "Kufe Fichte 80 x 80 mm")}},
    {"id": "carrier", "parent": "yard", "title": t("Lift carrier (massless: the hands of two people, then the wall height)", "Hubträger (masselos: die Hände zweier Personen, dann die Wandhöhe)"), "x": X0, "y": FLOOR_TOP, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 1.05}},
    {"id": "cabinet", "parent": "carrier", "title": t("Manifold cabinet (pivot at the top back edge; lies on its back, hangs upright on the wall)", "Verteilerschrank (Drehpunkt an der oberen Rückkante; liegt auf dem Rücken, hängt aufrecht an der Wand)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": CH, "joint": {"type": "revolute", "minDeg": -90, "maxDeg": 0}, "mass": {"kg": CABINET_KG, "comX": r6(CH / 2), "comY": 0.03, "distribution": "uniform", "source": "authored-estimate", "reference": "cabinet-datasheets"}, "section": {"areaM2": r6(2 * (CW + CD) * TS), "secondMomentM4": r6(TS * CD ** 3 / 6 + CW * TS * (CD / 2) ** 2 * 2), "extremeFibreM": CD / 2, "material": "cabinet-steel", "title": t("Cabinet as thin-walled box 780 x 125 x 1 mm", "Schrank als dünnwandiger Kasten 780 x 125 x 1 mm")}},
    {"id": "manifold", "parent": "cabinet", "title": t("Manifold on its two brackets", "Verteiler auf seinen beiden Haltern"), "x": MAN_U0, "y": 0, "angleDeg": 0, "lengthM": r6(0.45 - MAN_U0), "joint": {"type": "fixed"}, "mass": {"kg": MANIFOLD_KG, "comX": 0.15, "comY": 0.065, "distribution": "point", "source": "authored-estimate", "reference": "manifold-datasheets"}, "section": {"areaM2": r6(BAR_KG_M / RHO_SS), "secondMomentM4": round(math.pi / 64 * ((2 * BAR_R) ** 4 - (2 * BAR_R - 0.003) ** 4), 12), "extremeFibreM": BAR_R, "material": "stainless-steel", "title": t("Manifold bar 1 inch stainless tube 35 x 1.5 mm", "Verteilerbalken Edelstahlrohr 1 Zoll 35 x 1,5 mm")}},
]


def anchor(id_, bone, x, y, kind, en, de, z=None, **kw):
    """📍 One anchor."""
    a = {"id": id_, "bone": bone, "x": r6(x), "y": r6(y)}
    if z is not None:
        a["z"] = r6(z)
    a.update({"kind": kind, "title": t(en, de)})
    a.update(kw)
    return a


SUP = {"support": "contact", "normalDeg": 90, "frictionCoefficient": 0.5}
ANCHORS = []
for k, z in enumerate(SKID_Z, 1):
    for end, x, en, de in (("head", 0.04, "head", "Kopf"), ("foot", CR_L - 0.04, "foot", "Fuß")):
        ANCHORS.append(anchor(f"skid-{k}-{end}", "crate", x, 0, "support", f"Ground under skid {k}, {en} end", f"Boden unter Kufe {k}, {de}ende", z, footprintLengthM=0.08, footprintWidthM=0.08, **SUP))
for side, z, en, de in (("l", CW / 2, "left", "links"), ("r", -CW / 2, "right", "rechts")):
    ANCHORS.append(anchor(f"grip-{side}", "cabinet", 0.35, 0.06, "support", f"Hand grip on the {en} cabinet side", f"Handgriff an der Schrankseite {de}", z, footprintLengthM=0.1, footprintWidthM=0.05, **SUP))
for pos, u, en, de in (("top", 0.05, "top", "obere"), ("bottom", CH - 0.05, "bottom", "untere")):
    for side, z, sen, sde in (("l", 0.34, "left", "links"), ("r", -0.34, "right", "rechts")):
        extra = {"support": "pin", "normalDeg": 0} if pos == "top" else {"support": "contact", "normalDeg": 0, "frictionCoefficient": 0.3}
        ANCHORS.append(anchor(f"wall-{pos}-{side}", "cabinet", u, 0, "support", f"Wall plug and screw, {en} {sen}", f"Dübel und Schraube, {de} {sde}", z, footprintLengthM=0.02, footprintWidthM=0.02, **extra))
ANCHORS.append(anchor("water-fill", "manifold", 0.14, 0.065, "load", "Centre of the water filling of bars, stubs and flow meters", "Schwerpunkt der Wasserfüllung von Balken, Stummeln und Durchflussanzeigern"))
ANCHORS.append(anchor("plume-top", "cabinet", 0.02, 0.08, "emitter", "Top of the cabinet where the warm air of the running manifold rises", "Oberkante des Schranks, an der die warme Luft des laufenden Verteilers aufsteigt"))

REGION_IDS = ["crate", "cabinet", "supply", "return", "flow-meters", "actuators"]
SKID_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("skid")]
GRIP_CONTACTS = ["grip-l", "grip-r"]
WALL_CONTACTS = [a["id"] for a in ANCHORS if a["id"].startswith("wall")]
INSTALLED = {"carrier": {"dy": REINSTALL_DY}, "cabinet": {"angleDeg": -90}, "crate": {"dx": CRATE_DX}}


def bind(installed):
    """🌬️ Region bindings of one pose (indoors, no sun)."""
    out = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGION_IDS}
    out["crate"]["contact"] = {"fraction": 0.15, "tempC": 12 if not installed else 18}
    if installed:
        out["cabinet"]["contact"] = {"fraction": 0.28, "tempC": 19}
    return out


def water_load():
    """💧 The weight of the water filling when reinstalled."""
    return [{"id": "water-fill", "kind": "carried", "title": t(f"Heating water in the manifold, {WATER_L} l", f"Heizwasser im Verteiler, {dez(WATER_L)} l"), "anchor": "water-fill", "fxN": 0, "fyN": r6(-WATER_L * G), "source": "authored-estimate", "reference": "manifold-datasheets"}]


POSES = [
    {"id": "stored", "title": t("Stored in its crate", "In seiner Kiste gelagert"), "description": t("Drained and dismantled, the manifold lies in its cabinet on its back in an open-top spruce crate on the floor of the component exchange; the door of the cabinet is missing.", "Entleert und ausgebaut liegt der Verteiler in seinem Schrank auf dem Rücken in einer oben offenen Fichtenkiste auf dem Boden der Bauteilbörse; die Tür des Schranks fehlt."), "bones": {}, "ground": "authored", "contacts": SKID_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(False), "emitters": {"manifold-plume": 0}}, "loads": []},
    {"id": "lifted", "title": t("Lifted out by hand", "Von Hand herausgehoben"), "description": t("Two people grip the cabinet at its sides and lift it 300 mm out of the crate, still lying flat; the crate stays on the floor.", "Zwei Personen fassen den Schrank an den Seiten und heben ihn flach liegend 300 mm aus der Kiste; die Kiste bleibt am Boden."), "bones": {"carrier": {"dy": REST_LIFT}}, "ground": "authored", "contacts": GRIP_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(False), "emitters": {"manifold-plume": 0}}, "loads": []},
    {"id": "reinstalled", "title": t("Reinstalled and running", "Wieder eingebaut und in Betrieb"), "description": t("Screwed back to a wall with its bottom 400 mm above the floor, the eight circuits reconnected: 35 degC flow, 28 degC return, the actuators open, the crate set aside.", "Wieder an eine Wand geschraubt, Unterkante 400 mm über dem Boden, die acht Kreise wieder angeschlossen: 35 °C Vorlauf, 28 °C Rücklauf, die Stellantriebe offen, die Kiste beiseitegestellt."), "bones": INSTALLED, "ground": "authored", "contacts": WALL_CONTACTS, "thermal": {"operating": {"heating": 1}, "regions": bind(True), "emitters": {"manifold-plume": 1}}, "loads": water_load()},
    {"id": "reinstalled-off", "title": t("Reinstalled, filled and off", "Wieder eingebaut, befüllt und aus"), "description": t("On the wall and filled, but the circulation is off and the actuators closed: the manifold is at room temperature.", "An der Wand und befüllt, aber die Umwälzung ist aus und die Stellantriebe zu: der Verteiler hat Raumtemperatur."), "bones": INSTALLED, "ground": "authored", "contacts": WALL_CONTACTS, "thermal": {"operating": {"heating": 0}, "regions": bind(True), "emitters": {"manifold-plume": 0}}, "loads": water_load()},
]

CLIPS = [
    {"id": "lift", "title": t("Lift out of the crate", "Aus der Kiste heben"), "description": t("Two people take the cabinet at its sides and lift it 300 mm out of the crate in three seconds; the crate stays on the floor.", "Zwei Personen fassen den Schrank an den Seiten und heben ihn in drei Sekunden 300 mm aus der Kiste; die Kiste bleibt am Boden."), "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1, "pose": "stored", "ease": "smooth"}, {"t": 4, "pose": "lifted", "ease": "smooth"}, {"t": 5, "pose": "lifted", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 1} for a in SKID_CONTACTS] + [{"anchor": a, "fromS": 1, "toS": 5} for a in GRIP_CONTACTS],
     "phases": [{"id": "grip", "fromS": 0, "toS": 1, "title": t("Grip", "Greifen")}, {"id": "raise", "fromS": 1, "toS": 4, "title": t("Raise", "Anheben")}, {"id": "hold", "fromS": 4, "toS": 5, "title": t("Hold", "Halten")}],
     "events": [{"t": 1, "kind": "contact-off", "title": t("The cabinet leaves the crate floor", "Der Schrank verlässt den Kistenboden")}], "tracks": []},
    {"id": "heat-up", "title": t("Heating up after recommissioning", "Aufheizen nach der Wiederinbetriebnahme"), "description": t("The circulation pump starts and the actuators open within about three minutes: the bars and the flow meters go from room temperature to 35/28 degC and a weak warm plume rises from the cabinet.", "Die Umwälzpumpe läuft an und die Stellantriebe öffnen innerhalb von etwa drei Minuten: Balken und Durchflussanzeiger gehen von Raumtemperatur auf 35/28 °C, und eine schwache warme Fahne steigt aus dem Schrank."), "durationS": 30, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "reinstalled-off", "ease": "linear"}, {"t": 30, "pose": "reinstalled", "ease": "linear"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 30} for a in WALL_CONTACTS],
     "phases": [{"id": "open", "fromS": 0, "toS": 18, "title": t("Actuators opening", "Stellantriebe öffnen")}, {"id": "run", "fromS": 18, "toS": 30, "title": t("Running", "Laufen")}],
     "events": [{"t": 0, "kind": "operating", "title": t("Room thermostat calls for heat", "Raumthermostat fordert Wärme"), "target": "heating"}],
     "tracks": [{"target": "operating:heating", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 18, "v": 1}, {"t": 30, "v": 1}]}, {"target": "emitter:manifold-plume", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 18, "v": 1}, {"t": 30, "v": 1}]}]},
]

AREAS = json.loads((HERE / "author-areas.json").read_text(encoding="utf-8")) if (HERE / "author-areas.json").exists() else {"crate": 2.6, "cabinet": 2.2, "supply": 0.55, "return": 0.4, "flow-meters": 0.07, "actuators": 0.12}
HOT = [{"v": 0.5, "supplyTempC": 30, "returnTempC": 26}, {"v": 1, "supplyTempC": 35, "returnTempC": 28}]


def hydronic(pos, film=0.002):
    """♨️ The hydronic binding of a water-carrying region."""
    return {"key": "heating", "rows": HOT, "flowPosition": pos, "filmM2KW": film}


THERMAL = {
    "environment": {"airTempC": 14, "meanRadiantTempC": 14, "skyTempC": 12, "groundTempC": 12, "relativeHumidity": 0.55, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [],
    "operating": [{"id": "heating", "title": t("Underfloor heating circulation", "Umwälzung der Fußbodenheizung"), "levels": [{"v": 0, "title": t("Off or drained", "Aus oder entleert")}, {"v": 0.5, "title": t("Part load, 30/26 degC", "Teillast, 30/26 °C")}, {"v": 1, "title": t("Design point 35/28 degC, about 1.5 l/min per circuit", "Auslegungspunkt 35/28 °C, etwa 1,5 l/min je Kreis")}]}],
    "regions": [
        {"id": "crate", "title": t("Spruce crate", "Fichtenkiste"), "kind": "surface", "model": "passive", "material": "spruce-crate", "areaM2": AREAS["crate"], "characteristicLengthM": 0.8, "geometry": "horizontal-plate-up", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": "cabinet", "title": t("Sheet-steel cabinet with brackets", "Stahlblechschrank mit Haltern"), "kind": "surface", "model": "passive", "material": "cabinet-steel", "areaM2": AREAS["cabinet"], "characteristicLengthM": CH, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(TS * RHO_STEEL * 450), "ratedPowerW": 0, "ratedPowerBasis": "passive", "source": "authored-estimate", "reference": "cabinet-datasheets"},
        {"id": "supply", "title": t("Supply bar with outlets, PE-X stubs, ball valve and connection pipe", "Vorlaufbalken mit Abgängen, PE-X-Stummeln, Kugelhahn und Anschlussrohr"), "kind": "heater", "model": "hydronic", "material": "stainless-steel", "areaM2": AREAS["supply"], "characteristicLengthM": 2 * BAR_R, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(0.0015 * RHO_SS * 500), "ratedPowerW": 0, "ratedPowerBasis": "passive", "hydronic": hydronic(0), "source": "authored-estimate", "reference": "en-1264"},
        {"id": "return", "title": t("Return bar with valve inserts, outlets, PE-X stubs, ball valve and connection pipe", "Rücklaufbalken mit Ventileinsätzen, Abgängen, PE-X-Stummeln, Kugelhahn und Anschlussrohr"), "kind": "heater", "model": "hydronic", "material": "stainless-steel", "areaM2": AREAS["return"], "characteristicLengthM": 2 * BAR_R, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(0.0015 * RHO_SS * 500), "hydronic": hydronic(1), "source": "authored-estimate", "reference": "en-1264"},
        {"id": "flow-meters", "title": t("Flow meters on the supply bar", "Durchflussanzeiger auf dem Vorlaufbalken"), "kind": "heater", "model": "hydronic", "material": "psu-clear", "areaM2": AREAS["flow-meters"], "characteristicLengthM": 2 * FM_R, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(0.002 * 1240 * 1100), "hydronic": hydronic(0, 0.004), "source": "authored-estimate", "reference": "manifold-datasheets"},
        {"id": "actuators", "title": t("Thermal actuators and terminal box", "Thermische Stellantriebe und Klemmleiste"), "kind": "surface", "model": "operating", "material": "pa-white", "areaM2": AREAS["actuators"], "characteristicLengthM": 2 * ACT_R, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": round(0.002 * 1140 * 1600), "operating": {"key": "heating", "heatW": 8}, "ratedPowerW": 8, "ratedPowerBasis": "electrical-input", "source": "authored-estimate", "reference": "actuator-datasheets"},
    ],
    "emitters": [{"id": "manifold-plume", "kind": "plume", "title": t("Weak warm plume rising from the running manifold", "Schwache warme Fahne über dem laufenden Verteiler"), "anchor": "plume-top", "directionDeg": 90, "diameterM": 0.3, "operatingKey": "heating", "region": "supply", "source": "authored-estimate", "reference": "en-1264"}],
    "assumptions": [
        t("Power: the manifold is fed from a heat source elsewhere and is passive (0 W); the eight 230 V thermal actuators draw about 1 W each while open, 8 W on the actuator region.", "Leistung: der Verteiler wird von einem Wärmeerzeuger an anderer Stelle gespeist und ist passiv (0 W); die acht thermischen Stellantriebe 230 V nehmen geöffnet je etwa 1 W auf, 8 W an der Region der Stellantriebe."),
        t("Reinstalled the supply side runs at 35 degC and the return at 28 degC (EN 1264 screed heating, 7 K spread); part load 30/26 degC; stored and lifted it is drained and at hall temperature.", "Eingebaut läuft der Vorlauf mit 35 °C und der Rücklauf mit 28 °C (Estrichheizung nach EN 1264, 7 K Spreizung); Teillast 30/26 °C; gelagert und angehoben ist er entleert und hat Hallentemperatur."),
        t("Used surfaces: the stainless bars are dull and streaked with limescale (emissivity 0.35 instead of about 0.15 when polished), the brass is tarnished, the powder coat scuffed, PE-X and actuator housings yellowed, the spruce greyed.", "Gebrauchte Oberflächen: die Edelstahlbalken sind matt und kalkschlierig (Emissionsgrad 0,35 statt etwa 0,15 poliert), das Messing ist angelaufen, die Pulverbeschichtung verschrammt, PE-X und Stellantriebsgehäuse vergilbt, die Fichte vergraut."),
        t("Reference case: an unheated storage hall at 14 degC without sun; reinstalled, the back of the cabinet touches a wall at 19 degC.", "Referenzfall: eine ungeheizte Lagerhalle mit 14 °C ohne Sonne; eingebaut berührt die Schrankrückwand eine Wand mit 19 °C."),
    ],
}

STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": [], "assumptions": [
    t("Supports per pose: stored, six ground contacts under the ends of the three skids; lifted, the two hand grips on the cabinet sides; reinstalled, two wall screws at the top (pins) and two contact points at the bottom against the wall. The whole asset is one rigid body per pose, so lifted the crate left on the floor is counted with the cabinet.", "Auflager je Pose: gelagert sechs Bodenkontakte unter den Enden der drei Kufen; angehoben die beiden Handgriffe an den Schrankseiten; eingebaut zwei Wandschrauben oben (Gelenke) und zwei Kontaktpunkte unten an der Wand. Das ganze Asset ist je Pose ein starrer Körper; angehoben wird die am Boden verbliebene Kiste daher mit dem Schrank gezählt."),
    t(f"Masses from volumes and densities: crate {CRATE_KG} kg (spruce 450 kg/m3), cabinet {CABINET_KG} kg ({SHEET_M2:.3f} m2 of 1 mm sheet plus terminal box), manifold {MANIFOLD_KG} kg (bars {MANIFOLD_PARTS['bars']} kg from 35 x 1.5 mm stainless tube, the rest catalogue-typical); total {MASS} kg drained.", f"Massen aus Volumen und Dichten: Kiste {dez(CRATE_KG)} kg (Fichte 450 kg/m³), Schrank {dez(CABINET_KG)} kg ({dez(f'{SHEET_M2:.3f}')} m² Blech 1 mm und Klemmleiste), Verteiler {dez(MANIFOLD_KG)} kg (Balken {dez(MANIFOLD_PARTS['bars'])} kg aus Edelstahlrohr 35 x 1,5 mm, der Rest katalogtypisch); zusammen {dez(MASS)} kg entleert."),
    t(f"Reinstalled and filled, {WATER_L} l of heating water in bars, stubs, pipes and flow meters hang on the manifold as a point load at its centre.", f"Eingebaut und befüllt hängen {dez(WATER_L)} l Heizwasser in Balken, Stummeln, Rohren und Durchflussanzeigern als Einzellast in seinem Schwerpunkt am Verteiler."),
], "clinical": False}

ASSET = {
    "id": SLUG,
    "category": "components",
    "title": t("Used Underfloor Heating Manifold", "Gebrauchter Fußbodenheizungsverteiler"),
    "summary": t("A used stainless 8-circuit underfloor heating manifold with flow meters on the supply bar, thermal actuators on the return bar, red and blue 1 inch ball valves, two brackets and sixteen cut PE-X stubs, still in its surface-mounted sheet-steel cabinet 780 x 705 x 125 mm whose door is missing; drained, it lies on its back in an open-top spruce crate, is lifted out by hand and, reinstalled on a wall, runs at 35/28 degC.", "Ein gebrauchter Edelstahl-Fußbodenheizungsverteiler für acht Kreise mit Durchflussanzeigern auf dem Vorlaufbalken, thermischen Stellantrieben auf dem Rücklaufbalken, roten und blauen Kugelhähnen 1 Zoll, zwei Haltern und sechzehn abgeschnittenen PE-X-Stummeln, noch in seinem Aufputz-Stahlblechschrank 780 x 705 x 125 mm, dessen Tür fehlt; entleert liegt er auf dem Rücken in einer oben offenen Fichtenkiste, wird von Hand herausgehoben und läuft, wieder an eine Wand montiert, mit 35/28 °C."),
    "tags": ["component", "reclaimed", "used", "hvac", "heating", "plumbing", "controls", "manifold", "underfloor-heating", "stainless-steel", "sheet-steel", "pe-x", "crate"],
    "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t(
        f"Class-typical stainless 1 inch manifold for eight circuits: bars 540 mm long at 180 mm spacing, outlets at 50 mm pitch, in a surface-mounted cabinet 780 x 705 x 125 mm; crate 855 x 930 x 262 mm with 53 mm timber chocks; drained mass {MASS} kg. All dimensions and masses are catalogue-typical (authored-estimate), nothing is measured.",
        f"Klassentypischer Edelstahlverteiler 1 Zoll für acht Kreise: Balken 540 mm lang im Abstand von 180 mm, Abgänge im Raster 50 mm, in einem Aufputzschrank 780 x 705 x 125 mm; Kiste 855 x 930 x 262 mm mit Holzklötzen 53 mm; Masse entleert {dez(MASS)} kg. Alle Maße und Massen sind katalogtypisch (authored-estimate), nichts ist gemessen.")},
    "dimensions": {"heightM": HEIGHT, "widthM": WIDTH, "depthM": DEPTH, "massKg": MASS},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "yard", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {str(s): representation(s) for s in (50, 100, 200, 500)},
    "symbols": [],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": THERMAL,
    "structure": STRUCTURE,
    "volumes": {str(s): volume(s) for s in (50, 100, 200, 500)},
    "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"[DEBUG] wrote {SLUG}: mass {MASS} kg (crate {CRATE_KG}, cabinet {CABINET_KG}, manifold {MANIFOLD_KG}), water {WATER_L} l, h {HEIGHT} w {WIDTH} d {DEPTH}")
    print("[DEBUG] solids", {s: len(volume(s)["body"]) for s in (50, 100, 200, 500)})
