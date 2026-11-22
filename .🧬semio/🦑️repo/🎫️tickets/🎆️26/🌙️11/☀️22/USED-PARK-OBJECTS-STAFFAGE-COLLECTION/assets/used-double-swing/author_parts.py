"""🪵️ Hand-chosen numbers and the parts of the used double swing per authored scale: every body solid carries its thermal region and whether its 2D follows a moving bone.

World frame: x ahead (the swing direction), y up, z to the left of the asset (along the beam). The A-frames stand at z = ±1.47 m, the flat seat hangs at z = +0.58 m, the cradle seat at z = −0.58 m.

@see ./author-double-swing.py — assembles the document from these parts
@see ../../reports/used-double-swing.md — the reference table these numbers come from
"""
import math

from author_geometry import box, capsule, cylinder, heightfield, prism, prism_u, solid, torus

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

ZF = 1.47
FOOT_X = 0.85
YB = 2.41
RB = 0.07
ZB = 1.65
RL = 0.06
THETA = math.degrees(math.atan2(YB, FOOT_X))
SIN = math.sin(math.radians(THETA))
COS = math.cos(math.radians(THETA))
L_LEG = math.hypot(FOOT_X, YB)
U_TIMBER0 = 0.12 / SIN
U_TIMBER1 = 2.16 / SIN
U_ANCHOR = 0.45 / SIN
U_SOCKET0 = 1.96 / SIN
U_SOCKET1 = L_LEG - 0.071
R_SOCKET = (0.0605, 0.0645)
R_SLEEVE = (0.0705, 0.0765)
W_SLEEVE = 0.075
BED = {"x": (-1.05, 1.05), "z": (-1.67, 1.67), "y": -0.15}
TIE_Y = 1.00
TIE_X = (-0.62, 0.62)
PIVOT_Y = 2.30
HANGERS = {"flat-l": 0.83, "flat-r": 0.33, "cradle-l": -0.33, "cradle-r": -0.83}
EYES = {"flat-l": 0.78, "flat-r": 0.38, "cradle-l": -0.368, "cradle-r": -0.792}
HITCH_FLAT = 0.465
RING_CRADLE = 0.95
L_FLAT = PIVOT_Y - HITCH_FLAT
L_CRADLE = PIVOT_Y - RING_CRADLE
SEAT_FLAT = {"x": (-0.085, 0.085), "y": (0.40, 0.435), "z": (0.355, 0.805)}
ZC = -0.58
CRADLE_T = 0.016
R_CHAIN = 0.0105
R_CHAIN_MEMBER = math.sqrt(2 * math.pi * 0.003 ** 2 / math.pi)
TROUGH = {"x": 0.10, "z": 0.58, "a": 0.75, "b": 0.40, "d": 0.09}
PIT = {"x": -0.60, "z": -0.58, "a": 0.28, "b": 0.30, "d": 0.04}
MOUNDS = [{"x": 0.90, "z": 0.58, "a": 0.12, "b": 0.30, "h": 0.025}, {"x": -0.70, "z": 0.58, "a": 0.12, "b": 0.30, "h": 0.025}]

TIMBER = "pine-weathered"
STEEL = "steel-galvanised-weathered"
CHAIN = "steel-chain-worn"
EPDM = "epdm-seat-black"
EPDM_FADED = "epdm-cradle-faded"
GRAVEL = "gravel-round-2-8"
ALU = "aluminium-core"

TONE = {"timber": 0.62, "timber-end": 0.55, "steel": 0.45, "nut": 0.32, "chain": 0.36, "seat": 0.12, "cradle": 0.24, "gravel": 0.84}


def ground_surface(x, z, mounds=True, pit=True):
    """⛰️ Height of the worn gravel surface: 0 at the original level, the dragged trough under the flat seat, the shallow pit where the parent stands behind the cradle, and the ridges the feet pushed out of the trough."""
    y = 0.0
    r2 = ((x - TROUGH["x"]) / TROUGH["a"]) ** 2 + ((z - TROUGH["z"]) / TROUGH["b"]) ** 2
    if r2 < 1:
        y -= TROUGH["d"] * (1 - r2)
    if pit:
        q2 = ((x - PIT["x"]) / PIT["a"]) ** 2 + ((z - PIT["z"]) / PIT["b"]) ** 2
        if q2 < 1:
            y -= PIT["d"] * (1 - q2)
    if mounds:
        for m in MOUNDS:
            m2 = ((x - m["x"]) / m["a"]) ** 2 + ((z - m["z"]) / m["b"]) ** 2
            if m2 < 1:
                y += m["h"] * (1 - m2)
    return y


def grid(lo, hi, step, extra=()):
    """📏️ Grid nodes from lo to hi with about `step` spacing plus the given extra nodes."""
    n = max(1, round((hi - lo) / step))
    nodes = {round(lo + (hi - lo) * k / n, 5) for k in range(n + 1)} | {round(e, 5) for e in extra if lo < e < hi}
    return sorted(nodes)


class Part:
    """🧩️ One body solid with what its drawing needs: its thermal region, whether its 2D follows a moving bone, and a hatch."""

    def __init__(self, s, region, moving=False, hatch=None):
        self.solid = s
        self.region = region
        self.moving = moving
        self.hatch = hatch


def hex_nut_vw(cu, cv, af):
    """🔩️ A hexagon nut with its axis along w: profile in (u, v) about (cu, cv), across flats `af`."""
    r = af / math.sqrt(3)
    return [[cu + r * math.cos(math.radians(30 + 60 * k)), cv + r * math.sin(math.radians(30 + 60 * k))] for k in range(6)]


def hex_nut_up(cw, cu, af):
    """🔩️ A hexagon nut with its axis along v: profile in (w, u) about (cw, cu)."""
    r = af / math.sqrt(3)
    return [[cw + r * math.cos(math.radians(60 * k)), cu + r * math.sin(math.radians(60 * k))] for k in range(6)]


def leg_parts(scale, bone, sign):
    """🦵️ The two legs of one bone (one per A-frame) with their post anchors, bolts and nuts at `scale`; `sign` is +1 for the back leg (origin at the foot) and −1 for the front leg (origin at the apex)."""
    parts = []
    for side, w in (("l", ZF), ("r", -ZF)):
        foot = 0.0 if sign > 0 else L_LEG
        at = lambda u: u if sign > 0 else L_LEG - u
        if scale == 500:
            parts.append(Part(cylinder(f"{bone}-{side}", bone, TIMBER, "body", TONE["timber"], [at(0.0), 0, w], [at(U_TIMBER1), 0, w], RL), "timber-frame"))
            continue
        parts.append(Part(cylinder(f"{bone}-{side}", bone, TIMBER, "body", TONE["timber"], [at(U_TIMBER0), 0, w], [at(U_TIMBER1), 0, w], RL), "timber-frame"))
        ua = at(U_ANCHOR)
        g = 0.03 * COS / SIN
        prof = [[g, -0.03], [ua, -0.03], [ua, 0.03], [-g, 0.03]] if sign > 0 else [[foot - g, 0.03], [ua, 0.03], [ua, -0.03], [foot + g, -0.03]]
        o = math.copysign(1, w)
        if scale == 50:
            for tag, a, b in (("out", w + o * 0.060, w + o * 0.068), ("in", w - o * 0.068, w - o * 0.060)):
                parts.append(Part(prism(f"{bone}-{side}-bar-{tag}", bone, STEEL, "detail", TONE["steel"], prof, min(a, b), max(a, b)), "frame-steel"))
            for k, yb in enumerate((0.20, 0.36)):
                for tag, a, b in (("out", w + o * 0.068, w + o * 0.081), ("in", w - o * 0.081, w - o * 0.068)):
                    parts.append(Part(prism(f"{bone}-{side}-nut-{tag}-{k + 1}", bone, STEEL, "detail", TONE["nut"], hex_nut_vw(at(yb / SIN), 0.0, 0.024), min(a, b), max(a, b)), "frame-steel"))
        else:
            parts.append(Part(prism(f"{bone}-{side}-anchor", bone, STEEL, "detail", TONE["steel"], prof, w - 0.068, w + 0.068), "frame-steel"))
    return parts


def bracket_parts(scale, frames):
    """🔗️ The galvanised apex brackets (sleeve round the beam, two leg sockets, gusset, bolts) on the beam bone; tubes keep their walls at 1:50 and 1:100 and are solid at 1:200 and 1:500."""
    parts = []
    hollow = scale in (50, 100)
    s0, s1 = L_LEG - U_SOCKET0, L_LEG - U_SOCKET1
    dirs = {"back": frames.local_dir("beam", -COS, -SIN), "front": frames.local_dir("beam", COS, -SIN)}
    for side, w in (("l", ZF), ("r", -ZF)):
        parts.append(Part(cylinder(f"sleeve-{side}", "beam", STEEL, "body", TONE["steel"], [0, 0, w - W_SLEEVE], [0, 0, w + W_SLEEVE], R_SLEEVE[1], R_SLEEVE[0] if hollow else None), "frame-steel"))
        for name, d in dirs.items():
            parts.append(Part(cylinder(f"socket-{name}-{side}", "beam", STEEL, "body", TONE["steel"], [d[0] * s0, d[1] * s0, w], [d[0] * s1, d[1] * s1, w], R_SOCKET[1], R_SOCKET[0] if hollow else None), "frame-steel"))
        if scale == 50:
            a_half = FOOT_X * (1 - 2.06 / YB) - R_SOCKET[1] / SIN + 0.004
            apex_y = YB * (1 - (R_SOCKET[1] / SIN) / FOOT_X) + 0.004
            prof = [[-a_half, 2.06 - YB], [a_half, 2.06 - YB], [0.0, apex_y - YB]]
            parts.append(Part(prism(f"gusset-{side}", "beam", STEEL, "detail", TONE["steel"], prof, w - 0.004, w + 0.004), "frame-steel"))
        if scale == 50:
            o = math.copysign(1, w)
            for name, d in dirs.items():
                sm = L_LEG - (U_SOCKET0 + U_TIMBER1) / 2
                for tag, a, b in (("out", w + o * (R_SOCKET[1] - 0.002), w + o * (R_SOCKET[1] + 0.010)), ("in", w - o * (R_SOCKET[1] + 0.010), w - o * (R_SOCKET[1] - 0.002))):
                    parts.append(Part(prism(f"socket-nut-{name}-{side}-{tag}", "beam", STEEL, "detail", TONE["nut"], hex_nut_vw(d[0] * sm, d[1] * sm, 0.019), min(a, b), max(a, b)), "frame-steel"))
            for k, dw in enumerate((-0.035, 0.035)):
                parts.append(Part(solid(f"sleeve-nut-{side}-{k + 1}", "extrude", "beam", STEEL, "detail", TONE["nut"], profile=hex_nut_up(w + dw, 0.0, 0.019), plane="zx", **{"from": R_SLEEVE[1] - 0.004, "to": R_SLEEVE[1] + 0.006}), "frame-steel"))
    return parts


def beam_parts(scale):
    """🪵️ The round-timber beam with the four swing hangers (bearing housing below, top plate and nuts above)."""
    parts = [Part(cylinder("beam", "beam", TIMBER, "body", TONE["timber"], [0, 0, -ZB], [0, 0, ZB], RB), "timber-frame")]
    for key, w in HANGERS.items():
        if scale == 500:
            continue
        parts.append(Part(box(f"hanger-{key}", "beam", STEEL, "detail", TONE["steel"], [0, PIVOT_Y + 0.015 - YB, w], [0.06, 0.05, 0.045]), "frame-steel"))
        if scale == 50:
            parts.append(Part(box(f"hanger-plate-{key}", "beam", STEEL, "detail", TONE["steel"], [0, RB + 0.004, w], [0.08, 0.008, 0.05]), "frame-steel"))
            for k, du in enumerate((-0.025, 0.025)):
                parts.append(Part(solid(f"hanger-nut-{key}-{k + 1}", "extrude", "beam", STEEL, "detail", TONE["nut"], profile=hex_nut_up(w, du, 0.019), plane="zx", **{"from": RB + 0.008, "to": RB + 0.020}), "frame-steel"))
    return parts


def tie_parts(scale):
    """➖️ The half-round ties bolted to the outer faces of each A-frame at 1.0 m with carriage bolts; at 1:500 a 75 mm band on the same face."""
    parts = []
    length = TIE_X[1] - TIE_X[0]
    n = {50: 12, 100: 8, 200: 6}.get(scale)
    for side, w in (("l", ZF), ("r", -ZF)):
        o = math.copysign(1, w)
        if n is None:
            t = FEATURE[500]
            prof = [[0.06, w + o * 0.06], [0.06, w + o * (0.06 + t)], [-0.06, w + o * (0.06 + t)], [-0.06, w + o * 0.06]]
        else:
            prof = [[0.06 * math.cos(math.pi * k / n), w + o * (0.06 + 0.06 * math.sin(math.pi * k / n))] for k in range(n + 1)]
        if o < 0:
            prof = list(reversed(prof))
        parts.append(Part(prism_u(f"tie-{side}", "tie", TIMBER, "body", TONE["timber"], prof, 0.0, length), "timber-frame"))
        if scale == 50:
            for k, xl in enumerate((-FOOT_X * (1 - TIE_Y / YB), FOOT_X * (1 - TIE_Y / YB))):
                u = xl - TIE_X[0]
                parts.append(Part(capsule(f"tie-bolt-{side}-{k + 1}", "tie", STEEL, "detail", TONE["nut"], [u, 0, w + o * 0.117], [u, 0, w + o * 0.121], 0.012), "frame-steel"))
    return parts


def bed_part(scale):
    """🪨️ The safety gravel bed under the swing, cut 0.15 m below the surface and at its edges, with the hollows worn into it."""
    step = {50: 0.05, 100: 0.10, 200: 0.15, 500: 0.30}[scale]
    xs = grid(BED["x"][0], BED["x"][1], step, extra=(TROUGH["x"], PIT["x"]))
    fine = [z for z in grid(BED["z"][0], BED["z"][1], step) if TROUGH["z"] - TROUGH["b"] - step <= z <= TROUGH["z"] + TROUGH["b"] + step or PIT["z"] - PIT["b"] - step <= z <= PIT["z"] + PIT["b"] + step]
    zs = grid(BED["z"][0], BED["z"][1], max(step, 0.30), extra=fine + [TROUGH["z"], PIT["z"]])
    mounds = scale in (50, 100)
    pit = scale != 500
    top = lambda x, z: round(ground_surface(x, z, mounds, pit), 5)
    s = heightfield("gravel-bed", "surfacing", GRAVEL, "body", TONE["gravel"], xs, zs, top, BED["y"])
    return Part(s, "gravel-surface", hatch="gravel" if scale != 500 else None)


def chain_parts(scale):
    """⛓️ The four chains with their shackles (on the chain bones) and, at the cradle, the spreader rings, bridle chains and eyes."""
    parts = []
    for key in HANGERS:
        bone = f"chain-{key}"
        length = L_FLAT if key.startswith("flat") else L_CRADLE
        wt, wb = HANGERS[key], EYES[key]
        wat = lambda u: wt + (wb - wt) * u / length
        if key.startswith("flat"):
            u_end = {50: PIVOT_Y - 0.508, 100: PIVOT_Y - 0.508, 200: PIVOT_Y - SEAT_FLAT["y"][1] + 0.003, 500: PIVOT_Y - SEAT_FLAT["y"][0] - FEATURE[500] + 0.005}[scale]
        else:
            u_end = {50: L_CRADLE - 0.018, 100: L_CRADLE + 0.021, 200: PIVOT_Y - 0.672, 500: PIVOT_Y - 0.672}[scale]
        u0 = 0.0045 + R_CHAIN if scale != 500 else -0.069 + R_CHAIN
        u1 = u_end - R_CHAIN
        parts.append(Part(capsule(f"{bone}-links", bone, CHAIN, "body", TONE["chain"], [u0, 0, wat(u0)], [u1, 0, wat(u1)], R_CHAIN, slender=True), "chain-steel", moving=True))
        if key.startswith("flat") and scale in (50, 100):
            parts.append(Part(capsule(f"{bone}-shackle", bone, CHAIN, "detail", TONE["chain"], [PIVOT_Y - 0.504, 0, wb], [PIVOT_Y - 0.486, 0, wb], 0.008), "chain-steel", moving=True))
    return parts


def seat_flat_parts(scale):
    """🟫️ The flat seat (EPDM over an aluminium core) with its eye bolts, in the seat frame whose origin is the hitch."""
    parts = []
    hy = HITCH_FLAT
    y0, y1 = SEAT_FLAT["y"]
    if scale == 500:
        y1 = y0 + FEATURE[500]
    x0, x1 = SEAT_FLAT["x"]
    c = 0.005 if scale in (50, 100) else 0.0
    if c:
        prof = [[x0, y0 + c], [x0 + c, y0], [x1 - c, y0], [x1, y0 + c], [x1, y1 - c], [x1 - c, y1], [x0 + c, y1], [x0, y1 - c]]
    else:
        prof = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
    prof = [[p[0], p[1] - hy] for p in prof]
    parts.append(Part(prism("seat-flat-body", "seat-flat", EPDM, "body", TONE["seat"], prof, SEAT_FLAT["z"][0], SEAT_FLAT["z"][1]), "seat-flat", moving=True))
    if scale in (50, 100):
        for side, w in (("l", EYES["flat-l"]), ("r", EYES["flat-r"])):
            if scale == 50:
                parts.append(Part(torus(f"seat-eye-{side}", "seat-flat", CHAIN, "detail", TONE["chain"], [0, 0, w], [0, 0, 1], 0.013, 0.004), "chain-steel", moving=True))
                parts.append(Part(capsule(f"seat-eye-shank-{side}", "seat-flat", CHAIN, "detail", TONE["chain"], [0, -0.019, w], [0, -0.044, w], 0.005), "chain-steel", moving=True))
            else:
                parts.append(Part(box(f"seat-hitch-{side}", "seat-flat", CHAIN, "detail", TONE["chain"], [0, (0.418 + 0.482) / 2 - hy, w], [0.024, 0.482 - 0.418, 0.016]), "chain-steel", moving=True))
    return parts


def seat_cradle_parts(scale):
    """🧺️ The cradle seat: back wall, pan, front posts with the leg openings, top bar, side walls, eyes, bridle chains and spreader rings, in the frame whose origin is the ring."""
    parts = []
    oy = RING_CRADLE
    t = CRADLE_T if scale in (50, 100) else FEATURE[200] if scale == 200 else None
    zl, zr = ZC + 0.22, ZC - 0.22
    loc = lambda pts: [[p[0], p[1] - oy] for p in pts]
    if t is None:
        prof = loc([[-0.15, 0.42], [0.15, 0.42], [0.15, 0.65], [-0.15, 0.70]])
        parts.append(Part(prism("cradle-body", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], prof, zr, zl), "seat-cradle", moving=True))
        return parts
    top = lambda x: 0.70 + (0.65 - 0.70) * (x + 0.15) / 0.30
    parts.append(Part(prism("cradle-back", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], loc([[-0.15, 0.42], [-0.15 + t, 0.42], [-0.15 + t, top(-0.15 + t)], [-0.15, 0.70]]), zr, zl), "seat-cradle", moving=True))
    parts.append(Part(prism("cradle-pan", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], loc([[-0.15 + t, 0.42], [0.15, 0.42], [0.15, 0.42 + t], [-0.15 + t, 0.42 + t]]), zr, zl), "seat-cradle", moving=True))
    fx0, fx1 = 0.15 - t, 0.15
    if scale in (50, 100):
        front = loc([[fx0, 0.42 + t], [fx1, 0.42 + t], [fx1, 0.65], [fx0, top(fx0)]])
        parts.append(Part(prism("cradle-post-l", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], front, ZC + 0.17, zl), "seat-cradle", moving=True))
        parts.append(Part(prism("cradle-post-r", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], front, zr, ZC - 0.17), "seat-cradle", moving=True))
        parts.append(Part(prism("cradle-crotch", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], loc([[fx0, 0.42 + t], [fx1, 0.42 + t], [fx1, 0.61], [fx0, 0.61]]), ZC - 0.04, ZC + 0.04), "seat-cradle", moving=True))
        parts.append(Part(prism("cradle-bar", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], loc([[fx0, 0.61], [fx1, 0.61], [fx1, 0.65], [fx0, top(fx0)]]), ZC - 0.17, ZC + 0.17), "seat-cradle", moving=True))
    else:
        parts.append(Part(prism("cradle-front", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], loc([[fx0, 0.42 + t], [fx1, 0.42 + t], [fx1, 0.65], [fx0, top(fx0)]]), zr, zl), "seat-cradle", moving=True))
    sx0, sx1 = -0.15 + t, 0.15 - t
    side = loc([[sx0, 0.42 + t], [sx1, 0.42 + t], [sx1, top(sx1)], [sx0, top(sx0)]])
    parts.append(Part(prism("cradle-side-l", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], side, zl - t, zl), "seat-cradle", moving=True))
    parts.append(Part(prism("cradle-side-r", "seat-cradle", EPDM_FADED, "body", TONE["cradle"], side, zr, zr + t), "seat-cradle", moving=True))
    if scale in (50, 100):
        for sd, w in (("l", EYES["cradle-l"]), ("r", EYES["cradle-r"])):
            eyes = {"back": (-0.105, top(-0.105) + 0.012), "front": (0.105, top(0.105) + 0.012)}
            if scale == 50:
                parts.append(Part(torus(f"cradle-ring-{sd}", "seat-cradle", CHAIN, "detail", TONE["chain"], [0, 0, w], [0, 0, 1], 0.018, 0.005), "chain-steel", moving=True))
            for name, (ex, ey) in eyes.items():
                if scale == 50:
                    parts.append(Part(torus(f"cradle-eye-{name}-{sd}", "seat-cradle", CHAIN, "detail", TONE["chain"], [ex, ey - oy, w], [0, 0, 1], 0.010, 0.004), "chain-steel", moving=True))
                p0 = [0.0, -0.023 + R_CHAIN * 0.3, w]
                d = [ex - p0[0], ey - oy - p0[1]]
                n = math.hypot(*d)
                p1 = [ex - d[0] / n * 0.002, ey - oy - d[1] / n * 0.002, w] if scale == 50 else [ex, ey - 0.012 - oy + R_CHAIN - 0.002, w]
                parts.append(Part(capsule(f"bridle-{name}-{sd}", "seat-cradle", CHAIN, "detail", TONE["chain"], p0, p1, R_CHAIN), "chain-steel", moving=True))
    return parts
