"""🎠️ Authoritative generator of the used double swing (`used-double-swing`): writes `🔣️.json` next to this file from the hand-chosen numbers of `author_parts.py`.

Run: `python3 author-double-swing.py` (prints masses, areas and solid counts as `[DEBUG] ` lines while the asset is being authored).

@see ./author_parts.py — the numbers and the parts per scale
@see ./author_geometry.py — frames, solids, outlines
@see ./author_text.py — every bilingual text of the document
@see ../../brief.md — the brief of the wave
"""
import json
import math
import pathlib

from author_geometry import Frames, hull, quat_w, rnd, samples, signed_volume, simplify, solid, box, capsule, ellipsoid
from author_parts import (
    ALU, BED, CHAIN, COS, EPDM, EPDM_FADED, EYES, FEATURE, FOOT_X, GRAVEL, HANGERS, HITCH_FLAT, L_CRADLE, L_FLAT, L_LEG, MOUNDS, PIT, PIVOT_Y, R_CHAIN, R_CHAIN_MEMBER,
    RING_CRADLE, SEAT_FLAT, SIN, STEEL, THETA, TIE_X, TIE_Y, TIMBER, TROUGH, YB, ZC, ZF, beam_parts, bed_part, bracket_parts, chain_parts, ground_surface, leg_parts,
    seat_cradle_parts, seat_flat_parts, tie_parts,
)
import author_text as T

HERE = pathlib.Path(__file__).parent
SCALES = (50, 100, 200, 500)
G = 9.80665
T_SWING = 2.61
AMPLITUDE = 30.0
DX_MAINTENANCE = 0.452
CHILD_KG = 25.0
TODDLER_KG = 12.0
CHAIN_KG_M = 0.75
DENSITY = {TIMBER: 530, STEEL: 7850, CHAIN: 7850, EPDM: 1150, EPDM_FADED: 1150, ALU: 2700, GRAVEL: 1600}
AREA = {"timber-frame": 5.62, "frame-steel": 2.26, "chain-steel": 0.53, "seat-flat": 0.19, "seat-cradle": 0.89, "gravel-surface": 7.03}


def bone(bid, parent, title, x, y, angle, length, joint, mass=None, section=None):
    """🦴️ One rig bone in the sagittal plane."""
    b = {"id": bid, "parent": parent, "title": title, "x": rnd(x), "y": rnd(y), "angleDeg": rnd(angle), "lengthM": rnd(length), "joint": joint}
    if mass:
        b["mass"] = mass
    if section:
        b["section"] = section
    return b


def rig_bones():
    """🌳️ The rig: ground, surfacing, back leg → beam (apex) → front leg, tie, four chains and the two seats."""
    tie_local = Frames([bone("ground", None, T.BONES["ground"], 0, 0, 0, 0.1, {"type": "fixed"}), bone("leg-back", "ground", T.BONES["leg-back"], -FOOT_X, 0, THETA, L_LEG, {"type": "fixed"})]).to_local("leg-back", TIE_X[0], TIE_Y)
    return [
        bone("ground", None, T.BONES["ground"], 0, 0, 0, 0.1, {"type": "fixed"}),
        bone("surfacing", "ground", T.BONES["surfacing"], 0, 0, 0, 0.1, {"type": "fixed"}),
        bone("leg-back", "ground", T.BONES["leg-back"], -FOOT_X, 0, THETA, L_LEG, {"type": "fixed"}),
        bone("tie", "leg-back", T.BONES["tie"], tie_local[0], tie_local[1], -THETA, TIE_X[1] - TIE_X[0], {"type": "fixed"}),
        bone("beam", "leg-back", T.BONES["beam"], L_LEG, 0, -THETA, 0.14, {"type": "fixed"}),
        bone("leg-front", "beam", T.BONES["leg-front"], 0, 0, -THETA, L_LEG, {"type": "fixed"}),
        bone("chain-flat-l", "beam", T.BONES["chain-flat-l"], 0, PIVOT_Y - YB, -90, L_FLAT, {"type": "revolute", "minDeg": -70, "maxDeg": 70}),
        bone("chain-flat-r", "beam", T.BONES["chain-flat-r"], 0, PIVOT_Y - YB, -90, L_FLAT, {"type": "revolute", "minDeg": -70, "maxDeg": 70}),
        bone("seat-flat", "chain-flat-l", T.BONES["seat-flat"], L_FLAT, 0, 90, 0.085, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.5}),
        bone("chain-cradle-l", "beam", T.BONES["chain-cradle-l"], 0, PIVOT_Y - YB, -90, L_CRADLE, {"type": "revolute", "minDeg": -70, "maxDeg": 70}),
        bone("chain-cradle-r", "beam", T.BONES["chain-cradle-r"], 0, PIVOT_Y - YB, -90, L_CRADLE, {"type": "revolute", "minDeg": -70, "maxDeg": 70}),
        bone("seat-cradle", "chain-cradle-l", T.BONES["seat-cradle"], L_CRADLE, 0, 90, 0.15, {"type": "fixed"}),
    ]


def polygon_moments(poly):
    """📐️ Area, ∫x dA and ∫y dA, ∫xy dA of a polygon (signed by orientation)."""
    a = sx = sy = sxy = 0.0
    n = len(poly)
    for i in range(n):
        x0, y0 = poly[i]
        x1, y1 = poly[(i + 1) % n]
        c = x0 * y1 - x1 * y0
        a += c / 2
        sx += (x0 + x1) * c / 6
        sy += (y0 + y1) * c / 6
        sxy += (x0 * (2 * y0 + y1) + x1 * (y0 + 2 * y1)) * c / 24
    return a, sx, sy, sxy


def volume_centroid(s):
    """⚖️ Volume and bone-local centroid of a solid (revolve by Pappus, extrude by its profile, box, capsule)."""
    k = s["kind"]
    tr = (s.get("transform") or {}).get("translation", [0, 0, 0])
    if k == "box":
        a, b, c = s["size"]
        return a * b * c, tr
    if k == "capsule":
        p0, p1, r = s["p0"], s["p1"], s["r0"]
        length = math.dist(p0, p1)
        return math.pi * r * r * length + 4 / 3 * math.pi * r ** 3, [(p0[i] + p1[i]) / 2 for i in range(3)]
    if k == "ellipsoid":
        a, b, c = s["radii"]
        return 4 / 3 * math.pi * a * b * c, tr
    if k == "revolve":
        prof = s["profile"][:-1] if s["profile"][0] == s["profile"][-1] else s["profile"]
        a, sx, sy, sxy = polygon_moments(prof)
        vol = abs(2 * math.pi * sy)
        ac = sxy / sy if abs(sy) > 1e-15 else 0.0
        return vol, [s["origin"][i] + ac * s["axis"][i] for i in range(3)]
    if k == "extrude":
        a, sx, sy, _ = polygon_moments(s["profile"])
        length = s["to"] - s["from"]
        cu, cv = sx / a, sy / a
        cw = (s["to"] + s["from"]) / 2
        if s["plane"] == "xy":
            c = [cu, cv, cw]
        elif s["plane"] == "yz":
            c = [cw, cu, cv]
        else:
            c = [cv, cw, cu]
        return abs(a) * length, c
    return 0.0, [0, 0, 0]


def parts_of(scale, frames):
    """🧩️ Every body part of one scale."""
    return (
        [bed_part(scale)]
        + leg_parts(scale, "leg-back", +1)
        + leg_parts(scale, "leg-front", -1)
        + tie_parts(scale)
        + beam_parts(scale)
        + bracket_parts(scale, frames)
        + chain_parts(scale)
        + seat_flat_parts(scale)
        + seat_cradle_parts(scale)
    )


def bone_masses(frames):
    """⚖️ Segment masses per bone from the 1:50 parts (section × length × density; chains by their metre weight), with the bone-local centre."""
    acc = {}
    for p in parts_of(50, frames):
        s = p.solid
        if s["bone"] == "surfacing":
            continue
        vol, c = volume_centroid(s)
        if s["material"] == CHAIN and s["kind"] == "capsule" and s.get("slender"):
            kg = CHAIN_KG_M * math.dist(s["p0"], s["p1"])
        elif s["material"] == CHAIN and s["kind"] == "capsule" and s["id"].startswith("bridle"):
            kg = CHAIN_KG_M * math.dist(s["p0"], s["p1"])
        else:
            kg = DENSITY[s["material"]] * vol
        if s["id"] == "seat-flat-body":
            core = 0.16 * 0.008 * 0.40
            kg += core * (DENSITY[ALU] - DENSITY[EPDM])
        m = acc.setdefault(s["bone"], [0.0, 0.0, 0.0])
        m[0] += kg
        m[1] += kg * c[0]
        m[2] += kg * c[1]
    return {b: (m[0], m[1] / m[0], m[2] / m[0]) for b, m in acc.items()}


def sections():
    """📏️ Member sections: round timber C24, half-round tie, two link wires, the aluminium seat core, the cradle pan."""
    circ = lambda d, mat, title: {"areaM2": rnd(math.pi * d * d / 4), "secondMomentM4": float(f"{math.pi * d ** 4 / 64:.4e}"), "extremeFibreM": rnd(d / 2), "material": mat, "title": title}
    return {
        "leg-back": circ(0.12, TIMBER, T.SECTIONS["leg"]),
        "leg-front": circ(0.12, TIMBER, T.SECTIONS["leg"]),
        "beam": circ(0.14, TIMBER, T.SECTIONS["beam"]),
        "tie": {"areaM2": rnd(math.pi * 0.06 ** 2 / 2), "secondMomentM4": float(f"{math.pi * 0.06 ** 4 / 8:.4e}"), "extremeFibreM": 0.06, "material": TIMBER, "title": T.SECTIONS["tie"]},
        "chain-flat-l": {"areaM2": rnd(2 * math.pi * 0.003 ** 2), "secondMomentM4": float(f"{2 * math.pi * 0.006 ** 4 / 64:.4e}"), "extremeFibreM": 0.003, "material": CHAIN, "title": T.SECTIONS["chain"]},
        "seat-flat": {"areaM2": 0.00128, "secondMomentM4": 6.827e-09, "extremeFibreM": 0.004, "material": ALU, "title": T.SECTIONS["seat-flat"]},
        "seat-cradle": {"areaM2": rnd(0.284 * 0.016), "secondMomentM4": float(f"{0.284 * 0.016 ** 3 / 12:.4e}"), "extremeFibreM": 0.008, "material": EPDM_FADED, "title": T.SECTIONS["seat-cradle"]},
    }


def view_hulls(frames, s):
    """👁️ Outline of a solid per view plus its nearness interval (larger is nearer) per view."""
    pts = samples(s, 32 if s["kind"] in ("revolve", "capsule") else 24)
    world = [frames.to_world(s["bone"], p[0], p[1]) + (p[2],) for p in pts]
    out = {
        "elevation": (hull([[p[0], p[1]] for p in pts]), [-p[2] for p in world]),
        "front": (hull([[p[2], p[1]] for p in world]), [p[0] for p in world]),
        "plan": (hull([[p[0], p[2]] for p in world]), [p[1] for p in world]),
    }
    return {v: (h, (min(n), sum(n) / len(n), max(n))) for v, (h, n) in out.items()}


def inside(poly, pts, tol=1e-4):
    """🎯️ True when every point lies inside or on the convex counter-clockwise polygon."""
    for q in pts:
        for i in range(len(poly)):
            a, b = poly[i], poly[(i + 1) % len(poly)]
            if (b[0] - a[0]) * (q[1] - a[1]) - (b[1] - a[1]) * (q[0] - a[0]) < -tol * math.hypot(b[0] - a[0], b[1] - a[1]):
                return False
    return True


def lateral_span(s):
    """↔️ The lateral span of a solid at the joint and, when it leans, at the bone tip, for a `solid` placement."""
    pts = samples(s, 16)
    w0, w1 = min(p[2] for p in pts), max(p[2] for p in pts)
    if s["kind"] == "capsule" and abs(s["p0"][2] - s["p1"][2]) > 1e-6:
        return None
    return rnd(w0), rnd(w1)


def chain_solid_hint(s, length):
    """⛓️ The extruded placement of a leaning chain: span at the joint and at the bone tip."""
    p0, p1, r = s["p0"], s["p1"], s["r0"]
    slope = (p1[2] - p0[2]) / (p1[0] - p0[0])
    w_at = lambda u: p0[2] + slope * (u - p0[0])
    return {"kind": "extruded", "sweep": "w", "z0": rnd(w_at(0) - r), "z1": rnd(w_at(0) + r), "zTip": [rnd(w_at(length) - r), rnd(w_at(length) + r)]}


def drawing(scale, frames, parts, lengths):
    """✏️ The three authored views of one scale: one filled outline per visible part, the moving parts placed as extruded solids, hidden fixed parts left out."""
    info = [(p, view_hulls(frames, p.solid)) for p in parts]
    geometry = {"elevation": [], "front": [], "plan": []}
    tol = FEATURE[scale] / 8
    for view in geometry:
        for i, (p, h) in enumerate(info):
            ring, (nmin, nmean, nmax) = h[view]
            if not p.moving:
                hidden = False
                for j, (q, g) in enumerate(info):
                    if j == i or q.moving:
                        continue
                    ring2, (mmin, mmean, mmax) = g[view]
                    if inside(ring2, ring) and (mmean > nmax + 1e-6 or (mmean >= nmean and mmin >= nmax - 1e-6 and j < i)):
                        hidden = True
                        break
                if hidden:
                    continue
            s = p.solid
            elev_ring = simplify(h["elevation"][0], tol / 4)
            if s["id"] == "gravel-bed":
                elev_ring, ring = bed_outline(s, view)
            shape = {"id": s["id"], "bone": s["bone"], "role": s["role"], "z": 0.0, "filled": True, "tone": s["tone"], "region": p.region, "material": s["material"]}
            if p.hatch and scale != 500:
                shape["hatch"] = p.hatch
            if view == "elevation":
                shape["z"] = rnd(5 + nmax / 10 + (0.001 if s["role"] == "detail" else 0))
                pts = elev_ring
            elif p.moving:
                span = lateral_span(s)
                hint = chain_solid_hint(s, lengths[s["bone"]]) if span is None else {"kind": "extruded", "sweep": "w", "z0": span[0], "z1": span[1]}
                shape["solid"] = hint
                shape["z"] = rnd(5 + (nmax if view == "front" else nmax / 10) / 10 + (0.001 if s["role"] == "detail" else 0))
                pts = elev_ring
            else:
                shape["z"] = rnd(5 + nmax / 10 + (0.001 if s["role"] == "detail" else 0))
                pts = simplify(ring, tol / 4)
            shape.update({"kind": "poly", "points": rnd(pts), "closed": True})
            geometry[view].append(shape)
    return geometry


def bed_outline(s, view):
    """🪨️ The true outline of the gravel bed in a view: the highest surface point along the line of sight (the ridges stand above the original level), the cut floor below."""
    v = s["vertices"]
    top = [p for p in v if p[1] > BED["y"] + 1e-9]
    if view == "plan":
        ring = [[BED["x"][0], BED["z"][0]], [BED["x"][1], BED["z"][0]], [BED["x"][1], BED["z"][1]], [BED["x"][0], BED["z"][1]]]
        return ring, ring
    k = 0 if view == "elevation" else 2
    lo, hi = (BED["x"] if k == 0 else BED["z"])
    peaks = {}
    for p in top:
        key = round(p[k], 5)
        peaks[key] = max(peaks.get(key, -1.0), p[1])
    prof = [[key, max(0.0, y)] for key, y in sorted(peaks.items())]
    prof = [q for i, q in enumerate(prof) if i in (0, len(prof) - 1) or not (q[1] == prof[i - 1][1] == prof[i + 1][1])]
    ring = [[lo, BED["y"]], [hi, BED["y"]]] + list(reversed(prof))
    return ring, ring


def stroke(sid, bone_id, role, pts, tone=0.3, closed=False, region=None, pen=None, filled=False, hatch=None, z=6.0):
    """〰️ A hand-drawn detail or texture stroke (open or closed) in the coordinates of its view."""
    s = {"id": sid, "bone": bone_id, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if pen:
        s["penMm"] = pen
    if hatch:
        s["hatch"] = hatch
    s.update({"kind": "poly", "points": rnd(pts), "closed": closed})
    return s


def ellipse_ring(cx, cy, a, b, n=28):
    """⭕️ Points of an ellipse."""
    return [[cx + a * math.cos(2 * math.pi * k / n), cy + b * math.sin(2 * math.pi * k / n)] for k in range(n)]


def strokes(scale, frames):
    """🖋️ Traces of use and weather drawn as detail and texture strokes: drying checks, algae, rust, graffiti, sticker residue, chain wear, the hollows and ridges in the gravel."""
    e, f, p = [], [], []
    detail = scale != 500
    texture = scale in (50, 100)
    if detail:
        xs = [TROUGH["x"] - TROUGH["a"] + k * (2 * TROUGH["a"]) / 30 for k in range(31)]
        e.append(stroke("trough-profile", "surfacing", "detail", [[x, ground_surface(x, TROUGH["z"], scale in (50, 100))] for x in xs], z=5.05))
        zs = [TROUGH["z"] - TROUGH["b"] + k * (2 * TROUGH["b"]) / 24 for k in range(25)]
        f.append(stroke("trough-profile", "surfacing", "detail", [[z, ground_surface(TROUGH["x"], z, scale in (50, 100))] for z in zs], z=5.05))
        p.append(stroke("trough-rim", "surfacing", "detail", ellipse_ring(TROUGH["x"], TROUGH["z"], TROUGH["a"], TROUGH["b"]), closed=True, z=5.05))
        if scale in (50, 100, 200):
            p.append(stroke("trough-floor", "surfacing", "detail", ellipse_ring(TROUGH["x"], TROUGH["z"], TROUGH["a"] * 0.55, TROUGH["b"] * 0.55, 20), closed=True, z=5.05))
            xs = [PIT["x"] - PIT["a"] + k * (2 * PIT["a"]) / 16 for k in range(17)]
            e.append(stroke("pit-profile", "surfacing", "detail", [[x, ground_surface(x, PIT["z"], False)] for x in xs], z=5.05))
            p.append(stroke("pit-rim", "surfacing", "detail", ellipse_ring(PIT["x"], PIT["z"], PIT["a"], PIT["b"], 20), closed=True, z=5.05))
        if scale in (50, 100):
            for k, m in enumerate(MOUNDS):
                p.append(stroke(f"ridge-{k + 1}", "surfacing", "detail", ellipse_ring(m["x"], m["z"], m["a"], m["b"], 16), closed=True, z=5.05))
    if texture:
        for leg, sign in (("leg-back", 1), ("leg-front", -1)):
            at = lambda u: u if sign > 0 else L_LEG - u
            e.append(stroke(f"{leg}-check-1", leg, "texture", [[at(0.55), 0.014], [at(1.05), 0.017], [at(1.45), 0.013]], z=5.2))
            if scale == 50:
                e.append(stroke(f"{leg}-check-2", leg, "texture", [[at(1.05), -0.022], [at(1.62), -0.026], [at(2.0), -0.021]], z=5.2))
                e.append(stroke(f"{leg}-rust", leg, "texture", [[at(0.02), -0.026], [at(0.07), -0.012], [at(0.05), 0.004], [at(0.10), 0.02]], z=5.25))
        if scale == 50:
            e.append(stroke("leg-back-algae", "leg-back", "detail", [[0.14, 0.036], [1.05, 0.036], [1.05, 0.058], [0.14, 0.058]], tone=0.48, closed=True, filled=True, region="timber-frame", hatch="algae", z=5.21))
            e.append(stroke("beam-end-check", "beam", "texture", [[0.0, 0.0], [0.03, 0.04], [0.045, 0.052]], z=5.3))
            e.append(stroke("seat-flat-scuff", "seat-flat", "texture", [[0.06, -0.064], [0.083, -0.06], [0.084, -0.05]], z=5.3))
            e.append(stroke("shackle-wear", "chain-flat-r", "texture", [[PIVOT_Y - 0.512, -0.006], [PIVOT_Y - 0.505, 0.006]], z=5.3))
        f.append(stroke("beam-check-1", "beam", "texture", [[-1.30, 2.425], [-0.55, 2.43], [0.10, 2.427]], z=5.6))
        if scale == 50:
            f.append(stroke("beam-check-2", "beam", "texture", [[0.45, 2.395], [1.20, 2.392]], z=5.6))
            f.append(stroke("graffiti-tag", "beam", "texture", [[-0.22, 2.39], [-0.18, 2.44], [-0.15, 2.39], [-0.11, 2.45], [-0.08, 2.39], [-0.03, 2.43], [0.02, 2.40]], z=5.65))
            f.append(stroke("sticker-residue", "leg-front", "texture", [[-1.495, 1.20], [-1.445, 1.20], [-1.445, 1.262], [-1.495, 1.262]], closed=True, z=5.65))
            p.append(stroke("beam-top-check", "beam", "texture", [[0.01, -1.20], [0.012, -0.40], [0.008, 0.30]], z=6.6))
    return e, f, p


def hatches(scale):
    """▤️ The hatches of a scale: gravel dots in the bed, algae stipple on the shaded leg."""
    out = []
    if scale != 500:
        out.append({"id": "gravel", "title": T.HATCH["gravel"], "pattern": "dots", "angleDeg": 0, "spacingMm": {50: 1.2, 100: 1.6, 200: 2.0}[scale], "penMm": 0.13})
    if scale == 50:
        out.append({"id": "algae", "title": T.HATCH["algae"], "pattern": "stipple", "angleDeg": 0, "spacingMm": 0.5, "penMm": 0.13})
    return out


PENS = {
    50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}


def thin_strokes(scale, geometry, frames):
    """➖️ At 1:200 and 1:500 the chains are drawn as strokes on their centre lines (their 21 mm envelope is under the printable feature)."""
    for view in geometry:
        keep = []
        for s in geometry[view]:
            if s["bone"].startswith("chain-") and s["id"].endswith("-links"):
                key = s["bone"][len("chain-"):]
                length = L_FLAT if key.startswith("flat") else L_CRADLE
                pts = s["points"]
                u0, u1 = min(q[0] for q in pts), max(q[0] for q in pts)
                if view == "elevation":
                    line = [[u0, 0.0], [u1, 0.0]]
                    keep.append({**{k: v for k, v in s.items() if k not in ("points", "solid", "filled")}, "filled": False, "penMm": 0.13, "points": rnd(line), "closed": False})
                else:
                    wt, wb = HANGERS[key], EYES[key]
                    ya, yb = PIVOT_Y - u0, PIVOT_Y - u1
                    wa, wbb = wt + (wb - wt) * u0 / length, wt + (wb - wt) * u1 / length
                    line = [[wa, ya], [wbb, yb]] if view == "front" else [[0.0, wa], [0.0, wbb]]
                    keep.append({**{k: v for k, v in s.items() if k not in ("points", "solid", "filled")}, "filled": False, "penMm": 0.13, "points": rnd(line), "closed": False})
            else:
                keep.append(s)
        geometry[view] = keep
    return geometry


def representation(scale, frames, parts, lengths):
    """🗺️ The authored representation of one scale."""
    geometry = drawing(scale, frames, parts, lengths)
    if scale in (200, 500):
        geometry = thin_strokes(scale, geometry, frames)
    e, f, p = strokes(scale, frames)
    geometry["elevation"] += e
    geometry["front"] += f
    geometry["plan"] += p
    lo, hi = RANGES[scale]
    return {
        "status": "authored",
        "denominator": scale,
        "validRange": {"minDenominator": lo, "maxDenominator": hi},
        "pens": PENS[scale],
        "hatches": hatches(scale),
        "geometry": geometry,
        "decisions": T.REPRESENTATION[scale],
    }


def foot_support(scale, frames, bone_id, w, anchor):
    """🧷️ The fixed support block under one foot, level with the ground, at its anchor."""
    f = FEATURE[scale]
    size = [0.085 + 2 * f, 0.03 + f, 0.085 + 2 * f]
    x = -FOOT_X if bone_id == "leg-back" else FOOT_X
    cu, cv = frames.to_local(bone_id, x, -size[1] / 2)
    angle = -frames.world[bone_id][2]
    s = box(f"support-{anchor}", bone_id, STEEL, "body", 0.15, [cu, cv, w], size, angle)
    s.update({"member": bone_id, "part": "support", "anchor": anchor})
    return s


def mass_marker(scale, bid, centre, ident, thin=False):
    """🎯️ A marker at a centre of mass, at least one feature across; on a chain a slender spindle inside the 21 mm link envelope."""
    f = FEATURE[scale]
    r = [max(0.25, 0.012 * 25), 0.008, 0.008] if thin else [0.035 + 0.5 * f, 0.02 + 0.5 * f, 0.02 + 0.5 * f]
    s = ellipsoid(f"mass-{ident}", bid, STEEL, "detail", 0.2, centre, r, **({"slender": True} if thin else {}))
    s.update({"member": bid, "part": "mass"})
    return s


def ref(sid, of, member, part):
    """🔗️ A structural reference to a body solid."""
    return {"id": sid, "of": of, "member": member, "part": part}


def structure_list(scale, frames, parts, masses):
    """🏗️ The load path of one scale: members along the bones with their real sections, joints, the four fixed supports and the mass markers."""
    ids = {p.solid["id"]: p.solid for p in parts}
    out = []
    for leg in ("leg-back", "leg-front"):
        for side, w in (("l", ZF), ("r", -ZF)):
            out.append(ref(f"member-{leg}-{side}", f"{leg}-{side}", leg, "member"))
            joint = f"{leg}-{side}-bar-out" if scale == 50 else f"{leg}-{side}-anchor"
            if joint in ids:
                out.append(ref(f"joint-foot-{leg}-{side}", joint, leg, "joint"))
            name = leg.split("-")[1]
            out.append(ref(f"joint-socket-{name}-{side}", f"socket-{name}-{side}", leg, "joint"))
            out.append(foot_support(scale, frames, leg, w, f"foot-{name}-{side}"))
            kg, cu, cv = masses[leg]
            out.append(mass_marker(scale, leg, [cu, cv, w], f"{leg}-{side}"))
    for side, w in (("l", ZF), ("r", -ZF)):
        out.append(ref(f"member-tie-{side}", f"tie-{side}", "tie", "member"))
        for k in (1, 2):
            if f"tie-bolt-{side}-{k}" in ids:
                out.append(ref(f"joint-tie-bolt-{side}-{k}", f"tie-bolt-{side}-{k}", "tie", "joint"))
        kg, cu, cv = masses["tie"]
        out.append(mass_marker(scale, "tie", [cu, cv, w], f"tie-{side}"))
        out.append(ref(f"joint-apex-{side}", f"sleeve-{side}", "beam", "joint"))
    out.append(ref("member-beam", "beam", "beam", "member"))
    kg, cu, cv = masses["beam"]
    for side, w in (("l", ZF), ("r", -ZF)):
        out.append(mass_marker(scale, "beam", [cu, cv, w], f"beam-{side}"))
    for key in HANGERS:
        bid = f"chain-{key}"
        length = L_FLAT if key.startswith("flat") else L_CRADLE
        wt, wb = HANGERS[key], EYES[key]
        if key.startswith("flat"):
            u_end = length if scale in (50, 100) else PIVOT_Y - 0.4175
        else:
            u_end = {50: length, 100: length + 0.02}.get(scale, PIVOT_Y - 0.672)
        wat = lambda u: wt + (wb - wt) * u / length
        u_start = -0.06 if scale == 500 else 0.0
        s = capsule(f"member-{bid}", bid, CHAIN, "body", 0.5, [u_start, 0.0, wat(u_start)], [u_end, 0.0, wat(u_end)], R_CHAIN_MEMBER, slender=True)
        s.update({"member": bid, "part": "member"})
        out.append(s)
        if f"hanger-{key}" in ids:
            out.append(ref(f"joint-pivot-{key}", f"hanger-{key}", bid, "joint"))
        if f"{bid}-shackle" in ids:
            out.append(ref(f"joint-shackle-{key}", f"{bid}-shackle", bid, "joint"))
        kg, cu, cv = masses[bid]
        out.append(mass_marker(scale, bid, [cu, cv, wat(cu)], bid, thin=True))
    core = box("member-seat-core", "seat-flat", ALU, "body", 0.6, [0, -0.0475, (SEAT_FLAT["z"][0] + SEAT_FLAT["z"][1]) / 2], [0.16, 0.008, 0.40], sheet=True)
    core.update({"member": "seat-flat", "part": "member"})
    out.append(core)
    for side in ("l", "r"):
        if f"seat-eye-{side}" in ids:
            out.append(ref(f"joint-hitch-{side}", f"seat-eye-{side}", "seat-flat", "joint"))
            out.append(ref(f"member-eye-shank-{side}", f"seat-eye-shank-{side}", "seat-flat", "member"))
        if f"seat-hitch-{side}" in ids:
            out.append(ref(f"joint-hitch-{side}", f"seat-hitch-{side}", "seat-flat", "joint"))
    kg, cu, cv = masses["seat-flat"]
    out.append(mass_marker(scale, "seat-flat", [cu, cv, (SEAT_FLAT["z"][0] + SEAT_FLAT["z"][1]) / 2], "seat-flat"))
    pan = "cradle-pan" if "cradle-pan" in ids else "cradle-body"
    out.append(ref("member-cradle-pan", pan, "seat-cradle", "member"))
    for sid in ("cradle-side-l", "cradle-side-r"):
        if sid in ids:
            out.append(ref(f"member-{sid}", sid, "seat-cradle", "member"))
    for sd in ("l", "r"):
        if f"cradle-ring-{sd}" in ids:
            out.append(ref(f"joint-ring-{sd}", f"cradle-ring-{sd}", "seat-cradle", "joint"))
        for name in ("back", "front"):
            if f"bridle-{name}-{sd}" in ids:
                out.append(ref(f"member-bridle-{name}-{sd}", f"bridle-{name}-{sd}", "seat-cradle", "member"))
            if f"cradle-eye-{name}-{sd}" in ids:
                out.append(ref(f"joint-eye-{name}-{sd}", f"cradle-eye-{name}-{sd}", "seat-cradle", "joint"))
    kg, cu, cv = masses["seat-cradle"]
    out.append(mass_marker(scale, "seat-cradle", [cu, cv, ZC], "seat-cradle"))
    return out


def thermal_list(parts):
    """🌡️ Every body solid as the surface of its region; the gravel bed without its cut faces."""
    out = []
    for p in parts:
        s = {"id": f"th-{p.solid['id']}", "of": p.solid["id"], "region": p.region}
        if p.solid["id"] == "gravel-bed":
            s["sectionFaces"] = ["-v", "+u", "-u", "+w", "-w"]
        out.append(s)
    return out


def anchors():
    """📍️ Four fixed feet, the two seat load points and the rest point of the flat seat when it is set down."""
    feet = []
    for name, b, x in (("back", "leg-back", 0.0), ("front", "leg-front", L_LEG)):
        for side, z in (("l", ZF), ("r", -ZF)):
            feet.append({"id": f"foot-{name}-{side}", "bone": b, "x": rnd(x), "y": 0, "z": z, "kind": "support", "title": T.ANCHORS[f"foot-{name}-{side}"], "support": "fixed", "footprintLengthM": 0.4, "footprintWidthM": 0.4, "frictionCoefficient": 0.6})
    return feet + [
        {"id": "seat-flat-point", "bone": "seat-flat", "x": 0, "y": rnd(SEAT_FLAT["y"][1] - HITCH_FLAT), "z": 0.58, "kind": "load", "title": T.ANCHORS["seat-flat-point"]},
        {"id": "seat-cradle-point", "bone": "seat-cradle", "x": 0, "y": rnd(0.436 - RING_CRADLE), "z": ZC, "kind": "load", "title": T.ANCHORS["seat-cradle-point"]},
        {"id": "seat-flat-rest", "bone": "seat-flat", "x": 0, "y": rnd(SEAT_FLAT["y"][0] - HITCH_FLAT), "z": 0.58, "kind": "load", "title": T.ANCHORS["seat-flat-rest"]},
    ]


FEET = ["foot-back-l", "foot-back-r", "foot-front-l", "foot-front-r"]


def binding(sun, seat_flat=None, seat_cradle=None):
    """🌬️ The thermal binding of every region for one pose."""
    regions = {
        "timber-frame": {"exposedFraction": 1, "sunlitFraction": sun["timber"]},
        "frame-steel": {"exposedFraction": 1, "sunlitFraction": sun["steel"]},
        "chain-steel": {"exposedFraction": 1, "sunlitFraction": sun["chain"]},
        "seat-flat": {"exposedFraction": 1, "sunlitFraction": sun["flat"]},
        "seat-cradle": {"exposedFraction": 1, "sunlitFraction": sun["cradle"]},
        "gravel-surface": {"exposedFraction": 1, "sunlitFraction": sun["gravel"]},
    }
    if seat_flat:
        regions["seat-flat"]["contact"] = seat_flat
    if seat_cradle:
        regions["seat-cradle"]["contact"] = seat_cradle
    return {"regions": regions, "emitters": {}}


SUN = {"timber": 0.5, "steel": 0.5, "chain": 0.6, "flat": 0.85, "cradle": 0.6, "gravel": 0.8}


def loads_in_use():
    """🧒️ The child on the flat seat and the toddler in the cradle."""
    return [
        {"id": "child-flat", "kind": "occupant", "title": T.LOADS["child-flat"], "anchor": "seat-flat-point", "fxN": 0, "fyN": rnd(-CHILD_KG * G), "source": "authored-estimate", "reference": "en-1176-1-2017"},
        {"id": "toddler-cradle", "kind": "occupant", "title": T.LOADS["toddler-cradle"], "anchor": "seat-cradle-point", "fxN": 0, "fyN": rnd(-TODDLER_KG * G), "source": "authored-estimate", "reference": "en-1176-1-2017"},
    ]


def poses(seat_kg):
    """🧍️ In service (default), maintenance (flat seat unshackled and set down for replacement, carried by the fitter and then by the gravel as an applied force of its own weight) and in use (child at the forward turning point, toddler in the cradle)."""
    carried = [{"id": "seat-carried", "kind": "equipment", "title": T.LOADS["seat-carried"], "anchor": "seat-flat-rest", "fxN": 0, "fyN": rnd(round(seat_kg, 3) * G), "source": "authored-estimate", "reference": "en-1176-7-2020"}]
    return [
        {"id": "in-service", "title": T.POSES["in-service"]["title"], "description": T.POSES["in-service"]["description"], "bones": {}, "ground": "authored", "contacts": FEET, "thermal": binding(SUN), "loads": []},
        {"id": "maintenance", "title": T.POSES["maintenance"]["title"], "description": T.POSES["maintenance"]["description"], "bones": {"seat-flat": {"dx": DX_MAINTENANCE}}, "ground": "authored", "contacts": FEET, "thermal": binding({**SUN, "flat": 0.55}, seat_flat={"fraction": 0.45, "tempC": 24}), "loads": carried},
        {"id": "in-use", "title": T.POSES["in-use"]["title"], "description": T.POSES["in-use"]["description"], "bones": {"chain-flat-l": {"angleDeg": AMPLITUDE}, "chain-flat-r": {"angleDeg": AMPLITUDE}}, "ground": "authored", "contacts": FEET, "thermal": binding({**SUN, "flat": 0.4, "cradle": 0.45}, seat_flat={"fraction": 0.55, "tempC": 33}, seat_cradle={"fraction": 0.5, "tempC": 33}), "loads": loads_in_use()},
    ]


def clips():
    """🎞️ The maintenance clip (shackles opened, seat lowered into its hollow) and the swinging clip (one period of the 30-degree pendulum with the child)."""
    keys = []
    n = 12
    for k in range(n + 1):
        t = T_SWING * k / n
        a = AMPLITUDE * math.cos(2 * math.pi * k / n)
        keys.append({"t": rnd(t), "pose": "in-use", "ease": "smooth", "bones": {"chain-flat-l": {"angleDeg": rnd(a)}, "chain-flat-r": {"angleDeg": rnd(a)}}})
    swing = {
        "id": "swinging", "title": T.CLIPS["swinging"]["title"], "description": T.CLIPS["swinging"]["description"], "durationS": T_SWING, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
        "contacts": [{"anchor": a, "fromS": 0, "toS": T_SWING} for a in FEET],
        "phases": [{"id": "forward-half", "fromS": 0, "toS": rnd(T_SWING / 2), "title": T.CLIPS["swinging"]["phases"][0]}, {"id": "back-half", "fromS": rnd(T_SWING / 2), "toS": T_SWING, "title": T.CLIPS["swinging"]["phases"][1]}],
        "events": [{"t": 0, "kind": "motion", "title": T.CLIPS["swinging"]["events"][0]}, {"t": rnd(T_SWING / 2), "kind": "motion", "title": T.CLIPS["swinging"]["events"][1]}],
        "tracks": [],
    }
    maint = {
        "id": "maintenance", "title": T.CLIPS["maintenance"]["title"], "description": T.CLIPS["maintenance"]["description"], "durationS": 6.0, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 1.0, "pose": "in-service", "ease": "smooth"}, {"t": 5.0, "pose": "maintenance", "ease": "smooth"}, {"t": 6.0, "pose": "maintenance", "ease": "smooth"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 6.0} for a in FEET],
        "phases": [{"id": "unshackle", "fromS": 0, "toS": 1.0, "title": T.CLIPS["maintenance"]["phases"][0]}, {"id": "lower", "fromS": 1.0, "toS": 5.0, "title": T.CLIPS["maintenance"]["phases"][1]}, {"id": "set-down", "fromS": 5.0, "toS": 6.0, "title": T.CLIPS["maintenance"]["phases"][2]}],
        "events": [{"t": 1.0, "kind": "motion", "title": T.CLIPS["maintenance"]["events"][0]}, {"t": 5.0, "kind": "motion", "title": T.CLIPS["maintenance"]["events"][1]}],
        "tracks": [],
    }
    return [maint, swing]


def layer(lid, kind, material, thickness, conductivity, density, cp, reference, source="literature"):
    """🧶️ One thermal layer with resistance and areal heat capacity from its thickness."""
    return {"id": lid, "title": T.LAYERS[lid], "kind": kind, "material": material, "thicknessM": thickness, "resistanceM2KW": rnd(thickness / conductivity), "heatCapacityJM2K": round(density * cp * thickness), "source": source, "reference": reference}


def thermal_model():
    """🌡️ The summer reference case and the six passive regions with their weathered surfaces."""
    regions = [
        ("timber-frame", "surface", TIMBER, "cylinder", 0.12, 0.55, layer("timber-skin", "timber", TIMBER, 0.0235, 0.13, 530, 1600, "iso-10456-2007")),
        ("frame-steel", "surface", STEEL, "cylinder", 0.06, 0.55, layer("steel-wall", "metal", STEEL, 0.006, 50, 7850, 470, "iso-10456-2007")),
        ("chain-steel", "surface", CHAIN, "cylinder", 0.021, 0.6, layer("link-wire", "metal", CHAIN, 0.003, 50, 7850, 470, "din-766")),
        ("seat-flat", "surface", EPDM, "horizontal-plate-up", 0.17, 0.85, layer("epdm-flat", "polymer", EPDM, 0.0175, 0.25, 1150, 1000, "iso-10456-2007")),
        ("seat-cradle", "surface", EPDM_FADED, "vertical-plate", 0.28, 0.6, layer("epdm-cradle", "polymer", EPDM_FADED, 0.008, 0.25, 1150, 1000, "iso-10456-2007")),
        ("gravel-surface", "surface", GRAVEL, "horizontal-plate-up", 2.1, 0.85, layer("gravel-top", "masonry", GRAVEL, 0.043, 0.7, 1600, 840, "iso-10456-2007", "authored-estimate")),
    ]
    out = []
    for rid, kind, mat, geom, lc, sky, lay in regions:
        r = {"id": rid, "title": T.REGIONS[rid], "kind": kind, "model": "passive", "material": mat, "areaM2": AREA[rid], "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
        if rid == "timber-frame":
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        out.append(r)
    return {
        "environment": {"airTempC": 28, "meanRadiantTempC": 32, "skyTempC": 16, "groundTempC": 42, "relativeHumidity": 0.45, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 800, "diffuseFraction": 0.15, "pressurePa": 101325},
        "referenceEnvironment": "summer-hot-sun",
        "passengers": [],
        "operating": [],
        "regions": out,
        "emitters": [],
        "assumptions": T.THERMAL_ASSUMPTIONS,
    }


def materials():
    """🧱️ Weathered pine, weathered galvanised steel, worn chain steel, black and faded EPDM, the aluminium core and the round gravel."""
    m = lambda mid, e, a, rho, k, cp, E=None, f=None, source="literature", reference="iso-10456-2007": {k2: v for k2, v in {"id": mid, "title": T.MATERIALS[mid], "emissivity": e, "solarAbsorptivity": a, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp, "elasticModulusPa": E, "strengthPa": f, "source": source, "reference": reference}.items() if v is not None}
    return [
        m(TIMBER, 0.90, 0.65, 530, 0.13, 1600, 11e9, 24e6, "authored-estimate", "en-338-2016"),
        m(STEEL, 0.45, 0.62, 7850, 50, 470, 210e9, 235e6, "authored-estimate", "en-iso-1461-2022"),
        m(CHAIN, 0.35, 0.55, 7850, 50, 470, 210e9, 400e6, "authored-estimate", "din-766"),
        m(EPDM, 0.92, 0.93, 1150, 0.25, 1000, 6e6, None, "literature", "iso-10456-2007"),
        m(EPDM_FADED, 0.92, 0.85, 1150, 0.25, 1000, 6e6, None, "authored-estimate", "iso-10456-2007"),
        m(ALU, 0.25, 0.40, 2700, 160, 880, 70e9, 160e6, "literature", "iso-10456-2007"),
        m(GRAVEL, 0.92, 0.70, 1600, 0.70, 840, None, None, "authored-estimate", "iso-10456-2007"),
    ]


def main():
    bones = rig_bones()
    frames = Frames(bones)
    masses = bone_masses(frames)
    secs = sections()
    secs["chain-flat-r"] = secs["chain-flat-l"]
    secs["chain-cradle-l"] = secs["chain-flat-l"]
    secs["chain-cradle-r"] = secs["chain-flat-l"]
    refs = {"leg-back": "en-338-2016", "leg-front": "en-338-2016", "beam": "en-338-2016", "tie": "en-338-2016", "seat-flat": "en-1176-2-2017", "seat-cradle": "en-1176-2-2017"}
    for b in bones:
        if b["id"] in masses:
            kg, cu, cv = masses[b["id"]]
            b["mass"] = {"kg": round(kg, 3), "comX": rnd(cu), "comY": rnd(cv), "distribution": "point", "source": "literature" if b["id"].startswith("chain") else "authored-estimate", "reference": "din-766" if b["id"].startswith("chain") else refs[b["id"]]}
        if b["id"] in secs:
            b["section"] = secs[b["id"]]
    total = sum(v[0] for v in masses.values())
    lengths = {b["id"]: b["lengthM"] for b in bones}
    reps, vols = {}, {}
    for scale in SCALES:
        parts = parts_of(scale, frames)
        bed = parts[0].solid
        print(f"[DEBUG] 1:{scale} bed volume {signed_volume(bed):.4f} m3, body {len(parts)}")
        reps[str(scale)] = representation(scale, frames, parts, lengths)
        vols[str(scale)] = {
            "status": "authored",
            "denominator": scale,
            "body": [p.solid for p in parts],
            "thermal": thermal_list(parts),
            "structure": structure_list(scale, frames, parts, masses),
            "decisions": T.VOLUME[scale],
        }
    print(f"[DEBUG] masses {[(k, round(v[0], 2)) for k, v in masses.items()]} total {total:.2f}")
    xs = [BED["x"][0], BED["x"][1]]
    asset = {
        "id": "used-double-swing",
        "category": "objects",
        "title": T.TITLE,
        "summary": T.SUMMARY,
        "tags": ["used", "park", "public-space", "play", "swing", "double-swing", "timber", "round-timber", "a-frame", "playground", "gravel", "weathered"],
        "provenance": T.PROVENANCE,
        "dimensions": {"heightM": round(2.50 - BED["y"], 3), "widthM": round(xs[1] - xs[0], 3), "depthM": round(BED["z"][1] - BED["z"][0], 3), "massKg": round(total, 2)},
        "materials": materials(),
        "rig": {"kind": "mechanical", "root": "ground", "bones": bones},
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(masses["seat-flat"][0]),
        "clips": clips(),
        "thermal": thermal_model(),
        "structure": {"gravityMS2": G, "dynamics": "inertial", "supports": FEET, "assumptions": T.STRUCTURE_ASSUMPTIONS, "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "swinging", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print(f"[DEBUG] wrote {HERE / '🔣️.json'}")


if __name__ == "__main__":
    main()
