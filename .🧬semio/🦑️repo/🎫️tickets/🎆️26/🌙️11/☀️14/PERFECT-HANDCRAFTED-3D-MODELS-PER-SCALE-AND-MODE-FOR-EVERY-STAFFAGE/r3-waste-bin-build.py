"""🗑️ Round 3: handcrafted volumes of objects/waste-bin at 1:50, 1:100, 1:200 and 1:500 (body, thermal, structure).

Writes `volumes/objects/waste-bin.json`. Every section is stated here by hand: rounded-square rings of the moulded
HDPE tub (3 mm wall with draft), the pedestal foot skirt, the tub floor on the `base-plate` bone, the rim bead and
the lid seat, the hinge lugs and pin, the lid dish notched at the lugs, the skirt, the handle and the liner collar.
See `design.md` (C20, C25, C29, C30, D2, D3) and `round3-brief.md`.
"""
import json
import math
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
OUT = T / "volumes" / "objects" / "waste-bin.json"

DISH = [[0.52, 0.0], [0.52, 0.006], [0.51799, 0.01278], [0.51232, 0.01874], [0.50353, 0.02387], [0.49216, 0.02821], [0.47875, 0.03175], [0.46384, 0.03451], [0.44797, 0.03651], [0.43168, 0.03774], [0.41551, 0.03824], [0.4, 0.038], [0.12, 0.038], [0.09963, 0.03824], [0.08064, 0.03774], [0.06321, 0.03651], [0.04752, 0.03451], [0.03375, 0.03175], [0.02208, 0.02821], [0.01269, 0.02387], [0.00576, 0.01874], [0.00147, 0.01278], [0.0, 0.006], [0.0, 0.0], [0.004, 0.0], [0.004, 0.00554], [0.00511, 0.01113], [0.00857, 0.01589], [0.0147, 0.02041], [0.02348, 0.02446], [0.03471, 0.02787], [0.04816, 0.03056], [0.0636, 0.03252], [0.08083, 0.03375], [0.09966, 0.034], [0.12, 0.034], [0.4, 0.034], [0.41548, 0.034], [0.43147, 0.03375], [0.44757, 0.03253], [0.46323, 0.03056], [0.47788, 0.02785], [0.49095, 0.0244], [0.50183, 0.02025], [0.50989, 0.01556], [0.51456, 0.01072], [0.516, 0.00538], [0.516, 0.0]]
DISH_OUTER = DISH[:24]
HANDLE = [[0.38, 0.036], [0.38, 0.038], [0.38676, 0.04456], [0.39844, 0.04925], [0.4134, 0.05206], [0.43, 0.053], [0.4466, 0.05206], [0.46156, 0.04925], [0.47324, 0.04456], [0.48, 0.038], [0.48, 0.036]]

NOTCH_U = 0.014
LUG_W = (0.16, 0.22)
NOTCH_W = (0.157, 0.223)
PIVOT = (-0.255, 0.98)
WALL_Y0, WALL_S0, WALL_Y1, WALL_S1 = 0.025, 0.2, 0.955, 0.248


def r5(v):
    """🔢 Rounds a coordinate to 0.01 mm."""
    return round(v + 0.0, 5)


def pts(seq):
    """📐 Rounds a point list."""
    return [[r5(a), r5(b)] for a, b in seq]


def rr(hz, hx, r, cz=0.0, cx=0.0, n=6):
    """⬜ Rounded rectangle in profile coordinates [z, x] (half sizes hz, hx, corner radius r), counter-clockwise."""
    out = []
    for qz, qx, a0 in ((1, -1, -90), (1, 1, 0), (-1, 1, 90), (-1, -1, 180)):
        oz, ox = cz + qz * (hz - r), cx + qx * (hx - r)
        for i in range(n + 1):
            a = math.radians(a0 + 90 * i / n)
            out.append([oz + r * math.cos(a), ox + r * math.sin(a)])
    return pts(out)


def ring(h, r, hole_h, hole_r, hx=None, cx=0.0, hole_hx=None):
    """⭕ Outer rounded square with one rounded-square hole."""
    return rr(h, hx if hx is not None else h, r, 0.0, cx), [rr(hole_h, hole_hx if hole_hx is not None else hole_h, hole_r)]


def s_wall(y):
    """📏 Half size of the tub outer wall at height y (straight draft of the moulding, 2D shell of 1:50)."""
    return WALL_S0 + (WALL_S1 - WALL_S0) * (y - WALL_Y0) / (WALL_Y1 - WALL_Y0)


def clip(poly, keep):
    """✂️ Sutherland-Hodgman clip of a simple polygon against the half plane u >= c (keep=('ge', c)) or u <= c."""
    side, c = keep
    inside = (lambda p: p[0] >= c - 1e-12) if side == "ge" else (lambda p: p[0] <= c + 1e-12)
    out = []
    for i, cur in enumerate(poly):
        prev = poly[i - 1]
        if inside(cur):
            if not inside(prev):
                t = (c - prev[0]) / (cur[0] - prev[0])
                out.append([c, prev[1] + t * (cur[1] - prev[1])])
            out.append(cur)
        elif inside(prev):
            t = (c - prev[0]) / (cur[0] - prev[0])
            out.append([c, prev[1] + t * (cur[1] - prev[1])])
    clean = []
    for p in out:
        if not clean or abs(p[0] - clean[-1][0]) > 1e-7 or abs(p[1] - clean[-1][1]) > 1e-7:
            clean.append(p)
    return pts(clean)


def skirt_u(ho, hi, length, t, r_o, r_i, u0, n=6):
    """🔲 U-shaped lid skirt in [w, u]: both sides and the front, open at the rear (the hinge rail closes the rear)."""
    outer, inner = [], []
    outer.append([ho, u0])
    for i in range(n + 1):
        a = math.radians(0 + 90 * i / n)
        outer.append([ho - r_o + r_o * math.cos(a), length - r_o + r_o * math.sin(a)])
    for i in range(n + 1):
        a = math.radians(90 + 90 * i / n)
        outer.append([-ho + r_o + r_o * math.cos(a), length - r_o + r_o * math.sin(a)])
    outer.append([-ho, u0])
    inner.append([-hi, u0])
    for i in range(n + 1):
        a = math.radians(180 - 90 * i / n)
        inner.append([-hi + r_i + r_i * math.cos(a), length - t - r_i + r_i * math.sin(a)])
    for i in range(n + 1):
        a = math.radians(90 - 90 * i / n)
        inner.append([hi - r_i + r_i * math.cos(a), length - t - r_i + r_i * math.sin(a)])
    inner.append([hi, u0])
    return pts(outer + inner)


def foot(sz, sx):
    """🦶 Foot pad under one corner of the foot skirt, its outer corner following the skirt radius (30 mm)."""
    base = [[0.15, 0.15], [0.2, 0.15], [0.2, 0.17]]
    for i in range(1, 6):
        a = math.radians(90 * i / 6)
        base.append([0.17 + 0.03 * math.cos(a), 0.17 + 0.03 * math.sin(a)])
    base += [[0.17, 0.2], [0.15, 0.2]]
    p = [[sz * z, sx * x] for z, x in base]
    if sz * sx < 0:
        p.reverse()
    return pts(p)


def ext(id_, bone, prof, plane, f, t, role="body", tone=0.42, material="hdpe-plastic", holes=None, taper=None, **kw):
    """🧱 Extrude solid."""
    s = {"id": id_, "kind": "extrude", "bone": bone, "material": material, "role": role, "tone": tone, "profile": prof, "plane": plane, "from": r5(f), "to": r5(t)}
    if holes:
        s["holes"] = holes
    if taper:
        s["taperTo"] = [1, r5(taper)]
    s.update(kw)
    return s


def box(id_, bone, size, at, role="body", tone=0.42, cr=None, material="hdpe-plastic", **kw):
    """📦 Box solid."""
    s = {"id": id_, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": [r5(v) for v in size]}
    if cr:
        s["cornerRadius"] = cr
    s["transform"] = {"translation": [r5(v) for v in at]}
    s.update(kw)
    return s


def tube(id_, bone, origin, length, radius, role="detail", tone=0.3):
    """🔩 Cylinder along +w (revolve of a line about the lateral axis)."""
    return {"id": id_, "kind": "revolve", "bone": bone, "material": "hdpe-plastic", "role": role, "tone": tone, "profile": [[0, r5(radius)], [r5(length), r5(radius)]], "axis": [0, 0, 1], "origin": [r5(v) for v in origin]}


def ell(id_, bone, radii, at, member, tone=0.15):
    """⚫ Mass ellipsoid."""
    return {"id": id_, "kind": "ellipsoid", "bone": bone, "material": "hdpe-plastic", "role": "detail", "tone": tone, "member": member, "part": "mass", "radii": [r5(v) for v in radii], "transform": {"translation": [r5(v) for v in at]}}


def ref(id_, of, **kw):
    """🔗 Reference to a body solid."""
    d = {"id": id_, "of": of}
    d.update(kw)
    return d


def tub_wall(id_, y0, y1, t=0.003):
    """🪣 Hollow tub wall with draft: rounded square 2 x s(y0), corner 30 mm scaled, tapered to s(y1)."""
    s0 = s_wall(y0)
    r0 = 0.03 * s0 / 0.2
    prof, holes = ring(s0, r0, s0 - t, r0 - t)
    return ext(id_, "body", prof, "zx", y0, y1, holes=holes, taper=s_wall(y1) / s0)


def band(id_, y0, h, proud=0.002, sink=0.0015, region_texture=True):
    """〰️ Moulded reinforcing band: ring proud of the wall by 2 mm, sunk 1.5 mm into it."""
    s0 = s_wall(y0)
    r0 = 0.03 * s0 / 0.2
    prof, holes = ring(s0 + proud, r0 + proud, s0 - sink, r0 - sink)
    s = ext(id_, "body", prof, "zx", y0, y0 + h, holes=holes, taper=s_wall(y0 + h) / s0)
    if region_texture:
        s["texture"] = True
    return s


def film_ring(id_, y0, y1, hz, hx, r, cx=0.0, t=0.00003, inward=False):
    """🛍️ Liner film shell of 30 micrometres lying on a rounded-square face (outside it, or inside when inward)."""
    if inward:
        prof = rr(hz, hx, r, 0.0, cx)
        hole = rr(hz - t, hx - t, r - t, 0.0, cx)
    else:
        prof = rr(hz + t, hx + t, r + t, 0.0, cx)
        hole = rr(hz, hx, r, 0.0, cx)
    return ext(id_, "body", prof, "zx", y0, y1, role="detail", tone=0.3, material="ldpe-film", holes=[hole], region="liner-film", layer="film")


def film_flat(id_, y, outer, inner, t=0.00003):
    """🛍️ Horizontal annulus of film: outer = (hz, hx, r, cx), inner = (hz, hx, r, cx)."""
    return ext(id_, "body", rr(outer[0], outer[1], outer[2], 0.0, outer[3]), "zx", y, y + t, role="detail", tone=0.3, material="ldpe-film", holes=[rr(inner[0], inner[1], inner[2], 0.0, inner[3])], region="liner-film", layer="film")


def lid_set(scale):
    """🎩 Lid of 1:50 and 1:100: dish clipped at the hinge rail, rear rail pieces beside the lug notches, end walls, U skirt, handle."""
    cap_w = 0.242 if scale == 50 else 0.224
    rear = clip(DISH, ("le", NOTCH_U))
    out = [
        ext("lid-cap", "lid", clip(DISH, ("ge", NOTCH_U)), "xy", -cap_w, cap_w, tone=0.55),
        ext("lid-rail", "lid", rear, "xy", -NOTCH_W[0], NOTCH_W[0], tone=0.55),
    ]
    if scale == 50:
        out += [
            ext("lid-rail-l", "lid", rear, "xy", NOTCH_W[1], 0.242, tone=0.55),
            ext("lid-rail-r", "lid", rear, "xy", -0.242, -NOTCH_W[1], tone=0.55),
            ext("lid-end-l", "lid", pts(DISH_OUTER), "xy", 0.241, 0.25, tone=0.55),
            ext("lid-end-r", "lid", pts(DISH_OUTER), "xy", -0.25, -0.241, tone=0.55),
        ]
    else:
        out += [
            ext("lid-end-l", "lid", pts(DISH_OUTER), "xy", NOTCH_W[1], 0.25, tone=0.55),
            ext("lid-end-r", "lid", pts(DISH_OUTER), "xy", -0.25, -NOTCH_W[1], tone=0.55),
        ]
    out += [
        ext("lid-skirt", "lid", skirt_u(0.25, 0.246, 0.52, 0.004, 0.012, 0.008, 0.012), "zx", -0.028, 0.002, tone=0.45),
        ext("lid-handle", "lid", pts(HANDLE), "xy", -0.05, 0.05, role="detail", tone=0.35),
    ]
    return out


SEAT = dict(hz=0.245, hx=0.2475, cx=0.0025, r=0.035)
BEAD = dict(hz=0.25, hx=0.255, r=0.04)


def supports(h, shape="foot"):
    """🟩 Contact pads of the four foot anchors."""
    out = []
    for name, sx, sz in (("front-left", 1, 1), ("front-right", 1, -1), ("rear-left", -1, 1), ("rear-right", -1, -1)):
        if shape == "foot":
            s = ext("support-foot-" + name, "body", foot(sz, sx), "zx", 0.0, h, role="detail", tone=0.1)
        else:
            c = min(0.175, 0.199 - h / 2)
            s = box("support-foot-" + name, "body", [h, h, h], [c * sx, h / 2, c * sz], role="detail", tone=0.1)
        s.update({"member": "body", "part": "support", "tags": ["foot-" + name], "anchor": "foot-" + name})
        out.append(s)
    return out


def s50():
    """🔬 1:50: every assembly of the real moulding down to 7.5 mm."""
    body = []
    for name, sx, sz in (("front-left", 1, 1), ("front-right", 1, -1), ("rear-left", -1, 1), ("rear-right", -1, -1)):
        body.append(ext("foot-" + name, "body", foot(sz, sx), "zx", 0.0, 0.0085, role="detail", tone=0.15))
    prof, holes = ring(0.2, 0.03, 0.196, 0.026)
    body.append(ext("pedestal", "body", prof, "zx", 0.008, 0.017, holes=holes, tone=0.3))
    body.append(ext("pedestal-shoulder", "body", prof, "zx", 0.017, 0.025, holes=holes, taper=0.95, tone=0.3))
    body.append(ext("floor", "base-plate", rr(0.198, 0.198, 0.028, 0.0, 0.2), "zx", 0.0, 0.0075, tone=0.3))
    body.append(tub_wall("wall", 0.025, 0.952))
    for i, y in enumerate((0.15, 0.45, 0.72)):
        body.append(band(f"rib-{i}", y, 0.018))
    body.append(band("liner-ring", 0.895, 0.012, proud=0.0015, sink=0.0015))
    p, h = ring(BEAD["hz"], BEAD["r"], 0.2445, 0.0365, hx=BEAD["hx"])
    body.append(ext("rim-bead", "body", p, "zx", 0.94, 0.952, holes=h, tone=0.5))
    body.append(ext("rim-seat", "body", rr(SEAT["hz"], SEAT["hx"], SEAT["r"], 0.0, SEAT["cx"]), "zx", 0.948, 0.98, holes=[rr(0.235, 0.235, 0.03)], tone=0.5))
    for side, sz in (("l", 1), ("r", -1)):
        body.append(box(f"hinge-bracket-{side}", "body", [0.021, 0.062, 0.06], [-0.2575, 0.956, sz * 0.19], tone=0.3, cr=0.004))
    body.append(tube("hinge-pin", "body", [PIVOT[0], PIVOT[1], -0.22], 0.44, 0.0045))
    body.append(ext("liner-collar", "body", rr(0.2509, 0.2559, 0.0409), "zx", 0.944, 0.952, role="detail", tone=0.25, material="ldpe-film", holes=[rr(0.2502, 0.2552, 0.0402)], texture=True))
    body += lid_set(50)
    body.append(tube("hinge-barrel", "lid", [0.0, 0.0, -0.155], 0.31, 0.009, tone=0.25))

    th = []
    for s in body:
        sid = s["id"]
        if sid.startswith("rib-") or sid == "liner-ring":
            th.append(ref("th-" + sid, sid, region="bin-ribs", layer="hdpe-wall"))
        elif sid == "liner-collar":
            continue
        elif s["bone"] == "lid":
            sf = {"lid-cap": ["-v", "+u", "-u"], "lid-rail": ["-v"], "lid-rail-l": ["-v"], "lid-rail-r": ["-v"], "lid-end-l": ["-w"], "lid-end-r": ["+w"]}.get(sid)
            th.append(ref("th-" + sid, sid, region="lid-hdpe", layer="hdpe-wall", **({"sectionFaces": sf} if sf else {})))
        else:
            th.append(ref("th-" + sid, sid, region="bin-hdpe", layer="hdpe-wall"))
    th += film_parts(50)

    st = [
        ref("member-wall", "wall", member="body", part="member"),
        ref("member-pedestal", "pedestal", member="body", part="member"),
        ref("member-pedestal-shoulder", "pedestal-shoulder", member="body", part="member"),
        ref("member-rim-bead", "rim-bead", member="body", part="member"),
        ref("member-rim-seat", "rim-seat", member="body", part="member"),
        ref("member-floor", "floor", member="base-plate", part="member"),
        ref("member-lid", "lid-cap", member="lid", part="member"),
        ref("member-lid-rail", "lid-rail", member="lid", part="member"),
        ref("member-lid-end-l", "lid-end-l", member="lid", part="member"),
        ref("member-lid-end-r", "lid-end-r", member="lid", part="member"),
        ref("member-lid-skirt", "lid-skirt", member="lid", part="member"),
        ref("joint-pin", "hinge-pin", member="body", part="joint"),
        ref("joint-barrel", "hinge-barrel", member="lid", part="joint"),
        ref("joint-lug-l", "hinge-bracket-l", member="body", part="joint"),
        ref("joint-lug-r", "hinge-bracket-r", member="body", part="joint"),
        ell("mass-body", "body", [0.04, 0.04, 0.04], [0.0, 0.5, 0.0], "body"),
        ell("mass-lid", "lid", [0.05, 0.012, 0.05], [0.26, 0.019, 0.0], "lid"),
    ] + supports(0.0075)
    return body, th, st


def film_parts(scale):
    """🛍️ Liner bag collar (region liner-film, 30 micrometres): over the seat, down between seat and lid skirt, under the skirt edge, hanging over the bead."""
    seat = (SEAT["hz"], SEAT["hx"], SEAT["r"], SEAT["cx"])
    bead_o = (BEAD["hz"], BEAD["hx"], BEAD["r"], 0.0)
    hem = 0.944
    if scale in (50, 100):
        return [
            film_flat("th-film-top", 0.98, (seat[0] + 3e-5, seat[1] + 3e-5, seat[2] + 3e-5, seat[3]), (0.235, 0.235, 0.03, 0.0)),
            film_ring("th-film-sleeve", 0.952, 0.98, seat[0], seat[1], seat[2], cx=seat[3]),
            film_flat("th-film-step", 0.952, (bead_o[0] + 3e-5, bead_o[1] + 3e-5, bead_o[2] + 3e-5, 0.0), (seat[0], seat[1], seat[2], seat[3])),
            film_ring("th-film-drape", hem, 0.952, bead_o[0], bead_o[1], bead_o[2]),
        ]
    return [film_ring("th-film-rim", 0.94, 0.98, bead_o[0], bead_o[1], bead_o[2])]


def s100():
    """🔎 1:100: feet merged into the foot skirt, 15 mm floor, one wall, bands, bead and seat at 15 mm, lugs, lid without hinge barrel."""
    prof, holes = ring(0.2, 0.03, 0.196, 0.026)
    body = [
        ext("pedestal", "body", prof, "zx", 0.0, 0.025, holes=holes, tone=0.3),
        ext("floor", "base-plate", rr(0.198, 0.198, 0.028, 0.0, 0.2), "zx", 0.0, 0.015, tone=0.3),
    ]
    s0 = 0.2
    p, h = ring(s0, 0.03, s0 - 0.003, 0.027)
    body.append(ext("wall", "body", p, "zx", 0.025, 0.94, holes=h, taper=0.248 / 0.2))
    for i, y in enumerate((0.15, 0.45, 0.72)):
        sy = 0.2 + 0.048 * (y - 0.025) / 0.915
        p, h = ring(sy + 0.002, 0.03 * sy / 0.2 + 0.002, sy - 0.0015, 0.03 * sy / 0.2 - 0.0015)
        body.append(ext(f"rib-{i}", "body", p, "zx", y, y + 0.018, holes=h, taper=(sy + 0.048 * 0.018 / 0.915) / sy, texture=True))
    p, h = ring(BEAD["hz"], BEAD["r"], 0.2445, 0.0365, hx=BEAD["hx"])
    body.append(ext("rim-bead", "body", p, "zx", 0.937, 0.952, holes=h, tone=0.5))
    body.append(ext("rim-seat", "body", rr(SEAT["hz"], SEAT["hx"], SEAT["r"], 0.0, SEAT["cx"]), "zx", 0.948, 0.98, holes=[rr(0.23, 0.23, 0.025)], tone=0.5))
    for side, sz in (("l", 1), ("r", -1)):
        body.append(box(f"hinge-bracket-{side}", "body", [0.021, 0.062, 0.06], [-0.2575, 0.956, sz * 0.19], tone=0.3, cr=0.004))
    body += lid_set(100)

    th = []
    for s in body:
        sid = s["id"]
        if sid.startswith("rib-"):
            th.append(ref("th-" + sid, sid, region="bin-ribs", layer="hdpe-wall"))
        elif s["bone"] == "lid":
            sf = {"lid-cap": ["-v", "+u", "-u"], "lid-rail": ["-v"], "lid-end-l": ["-w"], "lid-end-r": ["+w"]}.get(sid)
            th.append(ref("th-" + sid, sid, region="lid-hdpe", layer="hdpe-wall", **({"sectionFaces": sf} if sf else {})))
        else:
            th.append(ref("th-" + sid, sid, region="bin-hdpe", layer="hdpe-wall"))
    th.append(band("th-liner-ring", 0.895, 0.012, proud=0.0015, sink=0.0015, region_texture=False) | {"region": "bin-ribs", "layer": "hdpe-wall"})
    th += film_parts(100)

    st = [
        ref("member-wall", "wall", member="body", part="member"),
        ref("member-pedestal", "pedestal", member="body", part="member"),
        ref("member-rim-bead", "rim-bead", member="body", part="member"),
        ref("member-rim-seat", "rim-seat", member="body", part="member"),
        ref("member-floor", "floor", member="base-plate", part="member"),
        ref("member-lid", "lid-cap", member="lid", part="member"),
        ref("member-lid-rail", "lid-rail", member="lid", part="member"),
        ref("member-lid-end-l", "lid-end-l", member="lid", part="member"),
        ref("member-lid-end-r", "lid-end-r", member="lid", part="member"),
        ref("member-lid-skirt", "lid-skirt", member="lid", part="member"),
        ref("joint-lug-l", "hinge-bracket-l", member="body", part="joint"),
        ref("joint-lug-r", "hinge-bracket-r", member="body", part="joint"),
        ell("mass-body", "body", [0.04, 0.04, 0.04], [0.0, 0.5, 0.0], "body"),
        ell("mass-lid", "lid", [0.05, 0.012, 0.05], [0.26, 0.019, 0.0], "lid"),
    ] + supports(0.015)
    return body, th, st


def s200():
    """🗺️ 1:200: pedestal and floor one 30 mm slab, one wall, the rim as one 40 mm ring, the lid dish seated on it."""
    body = [
        box("slab", "body", [0.4, 0.03, 0.4], [0.0, 0.015, 0.0], cr=0.0085, tone=0.42),
    ]
    p, h = ring(0.2, 0.03, 0.197, 0.027)
    body.append(ext("wall", "body", p, "zx", 0.03, 0.94, holes=h, taper=0.248 / 0.2))
    p, h = ring(BEAD["hz"], BEAD["r"], 0.235, 0.03, hx=BEAD["hx"])
    body.append(ext("rim", "body", p, "zx", 0.94, 0.98, holes=h, tone=0.5))
    body += [
        ext("lid-cap", "lid", pts(DISH), "xy", -0.235, 0.235, tone=0.55),
        ext("lid-end-l", "lid", pts(DISH_OUTER), "xy", 0.234, 0.25, tone=0.55),
        ext("lid-end-r", "lid", pts(DISH_OUTER), "xy", -0.25, -0.234, tone=0.55),
    ]
    th = [
        ref("th-slab", "slab", region="bin-hdpe", layer="hdpe-wall"),
        ref("th-wall", "wall", region="bin-hdpe", layer="hdpe-wall"),
        ref("th-rim", "rim", region="bin-hdpe", layer="hdpe-wall"),
        ref("th-lid-cap", "lid-cap", region="lid-hdpe", layer="hdpe-wall", sectionFaces=["-v", "+u", "-u"]),
        ref("th-lid-end-l", "lid-end-l", region="lid-hdpe", layer="hdpe-wall", sectionFaces=["-w"]),
        ref("th-lid-end-r", "lid-end-r", region="lid-hdpe", layer="hdpe-wall", sectionFaces=["+w"]),
    ]
    for i, y in enumerate((0.15, 0.45, 0.72)):
        sy = 0.2 + 0.048 * (y - 0.03) / 0.91
        p, h = ring(sy + 0.002, 0.03 * sy / 0.2 + 0.002, sy - 0.0015, 0.03 * sy / 0.2 - 0.0015)
        th.append(ext(f"th-rib-{i}", "body", p, "zx", y, y + 0.018, holes=h, taper=(sy + 0.048 * 0.018 / 0.91) / sy, region="bin-ribs", layer="hdpe-wall"))
    th += film_parts(200)
    st = [
        ref("member-wall", "wall", member="body", part="member"),
        ref("member-slab", "slab", member="body", part="member"),
        ref("member-rim", "rim", member="body", part="member"),
        ext("member-floor", "base-plate", rr(0.19, 0.19, 0.025, 0.0, 0.2), "zx", -0.024, 0.006, role="detail", tone=0.3, member="base-plate", part="member"),
        ref("member-lid", "lid-cap", member="lid", part="member"),
        ref("member-lid-end-l", "lid-end-l", member="lid", part="member"),
        ref("member-lid-end-r", "lid-end-r", member="lid", part="member"),
        ell("mass-body", "body", [0.04, 0.04, 0.04], [0.0, 0.5, 0.0], "body"),
        ell("mass-lid", "lid", [0.05, 0.015, 0.05], [0.26, 0.019, 0.0], "lid"),
    ] + supports(0.03, shape="box")
    return body, th, st


def s500():
    """🌍 1:500: a 75 mm plinth, one tapered wall up to the rim, the lid as a plate."""
    body = [
        box("slab", "body", [0.4, 0.075, 0.4], [0.0, 0.0375, 0.0], cr=0.012),
    ]
    s0 = 0.2 + 0.055 * 0.05 / 0.915
    p, h = ring(s0, 0.03 * s0 / 0.2, s0 - 0.003, 0.03 * s0 / 0.2 - 0.003)
    body.append(ext("wall", "body", p, "zx", 0.075, 0.98, holes=h, taper=0.2555 / s0))
    body.append(box("lid", "lid", [0.52, 0.04, 0.5], [0.26, 0.02, 0.0], tone=0.55))
    th = [
        ref("th-slab", "slab", region="bin-hdpe", layer="hdpe-wall"),
        ref("th-wall", "wall", region="bin-hdpe", layer="hdpe-wall"),
        ref("th-lid", "lid", region="lid-hdpe", layer="hdpe-wall"),
    ]
    for i, y in enumerate((0.15, 0.45, 0.72)):
        sy = s0 + (0.2555 - s0) * (y - 0.075) / 0.905
        p, h = ring(sy + 0.002, 0.03 * sy / 0.2 + 0.002, sy - 0.0015, 0.03 * sy / 0.2 - 0.0015)
        th.append(ext(f"th-rib-{i}", "body", p, "zx", y, y + 0.018, holes=h, taper=(sy + 0.0555 * 0.018 / 0.905) / sy, region="bin-ribs", layer="hdpe-wall"))
    sa = s0 + (0.2555 - s0) * (0.94 - 0.075) / 0.905
    ra = 0.03 * sa / 0.2
    th.append(ext("th-film-rim", "body", rr(sa + 3e-5, sa + 3e-5, ra + 3e-5), "zx", 0.94, 0.98, role="detail", tone=0.3, material="ldpe-film", holes=[rr(sa, sa, ra)], taper=0.2555 / sa, region="liner-film", layer="film"))
    st = [
        ref("member-wall", "wall", member="body", part="member"),
        ref("member-slab", "slab", member="body", part="member"),
        ref("member-lid", "lid", member="lid", part="member"),
        ext("member-floor", "base-plate", rr(0.19, 0.19, 0.025, 0.0, 0.2), "zx", -0.024, 0.051, role="detail", tone=0.3, member="base-plate", part="member"),
        ell("mass-body", "body", [0.04, 0.04, 0.04], [0.0, 0.5, 0.0], "body"),
        ell("mass-lid", "lid", [0.06, 0.019, 0.06], [0.26, 0.02, 0.0], "lid"),
    ] + supports(0.075, shape="box")
    return body, th, st


def txt(en, de):
    """🌐 Bilingual sentence."""
    return {"en": en, "de": de}


DEC = {
    50: [
        txt("Body 1:50 after a moulded 0.5 m square HDPE litter bin (EN 840-type construction): four 50 x 8.5 mm foot pads whose outer corners follow the 30 mm skirt radius, a 4 mm foot skirt with a chamfered shoulder (the drawn pedestal), the 7.5 mm tub floor on the base-plate bone at 25 mm, one 3 mm tub wall with straight draft from 0.400 to 0.496 m (no segments), three reinforcing bands 2 mm proud and the bag-retaining bead, the 12 mm rim bead flush with the lid skirt and the lid seat inside the skirt, two hinge lugs and the 9 mm pin, the liner collar draped over the bead (texture), and the lid: a 4 mm dish notched 14 mm deep at the lugs so that it swings to 110 degrees without touching them, its rear hinge rail, 9 mm end walls, a U skirt open at the hinge side, the handle and the 18 mm hinge barrel on the pivot axis.",
            "Körper 1:50 nach einem gespritzten 0,5 m quadratischen HDPE-Abfallbehälter (Bauweise wie EN 840): vier Fußpolster 50 x 8,5 mm, deren Außenecken dem 30-mm-Radius der Fußschürze folgen, eine 4-mm-Fußschürze mit gefaster Schulter (der gezeichnete Sockel), der 7,5-mm-Behälterboden am Knochen base-plate auf 25 mm, eine einzige 3-mm-Wand mit gerader Formschräge von 0,400 auf 0,496 m (keine Segmente), drei 2 mm erhabene Verstärkungsbänder und der Sackhaltewulst, der 12-mm-Randwulst bündig mit der Deckelschürze und der Deckelsitz innerhalb der Schürze, zwei Scharnierlaschen und der 9-mm-Stift, der über den Wulst hängende Sackkragen (Textur) und der Deckel: eine 4-mm-Schale, an den Laschen 14 mm tief ausgespart, damit er ohne Berührung bis 110 Grad aufschwenkt, seine hintere Scharnierleiste, 9-mm-Stirnwände, eine zur Scharnierseite offene U-Schürze, der Griff und die 18-mm-Scharnierwalze auf der Drehachse."),
        txt("Thermal 1:50: shell, foot skirt, floor, rim, lugs and pin reference the body (bin-hdpe), bands and bead (bin-ribs), the lid pieces and barrel (lid-hdpe, the faces looking into the closed bin are section faces as decided by the area reviewers); the liner collar is a real 30 micrometre film over the seat, down between seat and skirt, under the skirt edge and over the bead (liner-film). No hidden regions, no emitters.",
            "Thermik 1:50: Schale, Fußschürze, Boden, Rand, Laschen und Stift verweisen auf den Körper (bin-hdpe), Bänder und Wulst (bin-ribs), die Deckelteile und die Walze (lid-hdpe, die Flächen zum geschlossenen Behälterinneren sind Schnittflächen nach Entscheid der Flächenprüfer); der Sackkragen ist eine echte 30-Mikrometer-Folie über den Sitz, zwischen Sitz und Schürze hinab, unter der Schürzenkante durch und über den Wulst (liner-film). Keine verdeckten Regionen, keine Emitter."),
        txt("Structure 1:50: the bin is a monocoque, so its load path is the moulding itself: the tub wall, foot skirt with shoulder, rim bead and seat are the members of body, the floor that carries the waste is the member of base-plate, the lid dish, rail, end walls and skirt the members of lid; pin, lugs and barrel are the hinge joints, a 4.5 kg body mass at the rig centre of mass (0.5 m) and the 1.2 kg lid mass, and the four foot pads are the contact supports of their anchors. Bands, bead, handle, feet and the bag are non-bearing and absent.",
            "Tragwerk 1:50: Der Behälter ist eine Schalenkonstruktion, sein Lastpfad ist das Formteil selbst: Wand, Fußschürze mit Schulter, Randwulst und Sitz sind die Stäbe von body, der Boden, der den Abfall trägt, der Stab von base-plate, Deckelschale, Leiste, Stirnwände und Schürze die Stäbe von lid; Stift, Laschen und Walze sind die Scharniergelenke, eine 4,5-kg-Körpermasse im Rig-Schwerpunkt (0,5 m) und die 1,2-kg-Deckelmasse, die vier Fußpolster sind die Kontaktlager ihrer Anker. Bänder, Wulst, Griff, Füße und Sack tragen nicht und fehlen."),
    ],
    100: [
        txt("Body 1:100 authored for 15 mm: the feet merge into a 25 mm hollow foot skirt (the drawn pedestal), a 15 mm floor, one straight 3 mm wall to 0.94 m, the three bands as texture, a 15 mm rim bead and a 15 mm wide lid seat, the two lugs, and the lid as dish, hinge rail, 27 mm end walls that take in the rail corners, U skirt and handle, still notched at the lugs; hinge pin and barrel (9 and 18 mm) are not drawn at this scale and the bag hangs inside the thermal model only.",
            "Körper 1:100 für 15 mm erstellt: die Füße gehen in eine 25 mm hohe hohle Fußschürze auf (der gezeichnete Sockel), ein 15-mm-Boden, eine gerade 3-mm-Wand bis 0,94 m, die drei Bänder als Textur, ein 15-mm-Randwulst und ein 15 mm breiter Deckelsitz, die zwei Laschen und der Deckel als Schale, Scharnierleiste, 27-mm-Stirnwände, die die Leistenecken aufnehmen, U-Schürze und Griff, weiterhin an den Laschen ausgespart; Scharnierstift und -walze (9 und 18 mm) sind in diesem Maßstab nicht gezeichnet, der Sack hängt nur im thermischen Modell."),
        txt("Thermal 1:100: body references per region as at 1:50, the bag-retaining bead as a thermal-only ring of bin-ribs, and the liner collar as the same 30 micrometre film path over seat and bead.",
            "Thermik 1:100: Körperverweise je Region wie in 1:50, der Sackhaltewulst als rein thermischer Ring von bin-ribs und der Sackkragen als derselbe 30-Mikrometer-Folienweg über Sitz und Wulst."),
        txt("Structure 1:100: wall, foot skirt, rim bead and seat (body), floor (base-plate), lid dish, rail, end walls and skirt (lid) as members, the lugs as hinge joints, the two masses and four 15 mm foot pads as supports.",
            "Tragwerk 1:100: Wand, Fußschürze, Randwulst und Sitz (body), Boden (base-plate), Deckelschale, Leiste, Stirnwände und Schürze (lid) als Stäbe, die Laschen als Scharniergelenke, die zwei Massen und vier 15-mm-Fußpolster als Lager."),
    ],
    200: [
        txt("Body 1:200 authored for 30 mm: pedestal, feet and floor merge into one 30 mm slab, one wall, bead and seat into one 40 mm rim ring, and the lid is the domed dish with 16 mm end walls seated on that ring (its 28 mm skirt is below the feature and merged into the rim, so the drawn lid starts at the seat); bands, lugs, hinge, handle and bag are not drawn.",
            "Körper 1:200 für 30 mm erstellt: Sockel, Füße und Boden werden eine 30-mm-Platte, eine Wand, Wulst und Sitz ein 40-mm-Randring, und der Deckel ist die gewölbte Schale mit 16-mm-Stirnwänden auf diesem Ring (seine 28-mm-Schürze liegt unter dem Merkmal und geht im Rand auf, die gezeichnete Deckelkontur beginnt daher am Sitz); Bänder, Laschen, Scharnier, Griff und Sack sind nicht gezeichnet."),
        txt("Thermal 1:200: slab, wall, rim and lid by reference, the three bands as thermal-only rings (bin-ribs) and the bag collar as one film ring over the rim (liner-film); only the total area is compared here.",
            "Thermik 1:200: Platte, Wand, Rand und Deckel als Verweis, die drei Bänder als rein thermische Ringe (bin-ribs) und der Sackkragen als ein Folienring über dem Rand (liner-film); verglichen wird hier nur die Gesamtfläche."),
        txt("Structure 1:200: wall, slab and rim (body), the floor plate inside the slab (base-plate), lid dish and end walls (lid) as members, the two masses and four 30 mm contact blocks.",
            "Tragwerk 1:200: Wand, Platte und Rand (body), die Bodenplatte in der Platte (base-plate), Deckelschale und Stirnwände (lid) als Stäbe, die zwei Massen und vier 30-mm-Kontaktblöcke."),
    ],
    500: [
        txt("Body 1:500 authored for 75 mm: a 75 mm plinth (the print feature, thicker than the 25 mm pedestal it stands for), one tapered wall that includes the rim and flares to 0.511 m, and the lid as a 0.52 x 0.04 x 0.5 m plate; the dome (38 mm) and everything smaller are below the feature.",
            "Körper 1:500 für 75 mm erstellt: ein 75-mm-Sockel (das Druckmerkmal, dicker als der 25-mm-Sockel, für den er steht), eine konische Wand bis 0,511 m einschließlich Rand und der Deckel als Platte 0,52 x 0,04 x 0,5 m; die Wölbung (38 mm) und alles Kleinere liegt unter dem Merkmal."),
        txt("Thermal 1:500: plinth, wall and lid by reference, the bands as thermal-only rings and the bag collar as a film ring at the top of the wall; only the total is compared.",
            "Thermik 1:500: Sockel, Wand und Deckel als Verweis, die Bänder als rein thermische Ringe und der Sackkragen als Folienring am oberen Wandende; verglichen wird nur die Summe."),
        txt("Structure 1:500: wall, plinth and lid as members, the floor plate inside the plinth for base-plate, the two masses and four 75 mm contact blocks.",
            "Tragwerk 1:500: Wand, Sockel und Deckel als Stäbe, die Bodenplatte im Sockel für base-plate, die zwei Massen und vier 75-mm-Kontaktblöcke."),
    ],
}


def main():
    """🚀 Builds and writes the volumes source."""
    vols = {}
    for scale, fn in ((50, s50), (100, s100), (200, s200), (500, s500)):
        body, th, st = fn()
        vols[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": th, "structure": st, "decisions": DEC[scale]}
    OUT.write_text(json.dumps({"volumes": vols}, ensure_ascii=False, indent=2) + "\n")
    for k, v in vols.items():
        print(k, len(v["body"]), len(v["thermal"]), len(v["structure"]), file=sys.stderr)


if __name__ == "__main__":
    main()
