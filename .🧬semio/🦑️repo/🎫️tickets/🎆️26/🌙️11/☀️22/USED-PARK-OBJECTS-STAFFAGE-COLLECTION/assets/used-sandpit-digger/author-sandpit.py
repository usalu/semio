#!/usr/bin/env python3
"""🏖️ Author `used-sandpit-digger`: every hand-chosen number of the used sandpit with its seated play digger in, the asset document out.

World frame of the staffage: x ahead (the digger reaches ahead), y up (lawn at 0), z left. Every part is authored once per scale in world
rest coordinates; the bone-local 3D solids and the three 2D views of that scale are derived from the same numbers, so 2D and 3D agree.
See `🎓️teaching/🏛️architecture/🧍️staffage/README.md` (Front and plan views, Solid placements, Volume).
"""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "🔣️.json")
SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
XP = -0.55
SAND_TOP = 0.12
SAND_SPLIT = 0.07
FRAME_OUT = 2.0
TIMBER = 0.14
COURSE = 0.14
BOARD_W = 0.28
BOARD_T = 0.045
INNER = FRAME_OUT - TIMBER
FRAME_TOP = 2 * COURSE + BOARD_T
BOOM_ANGLE = 22.0
BOOM_L = 0.58
DIPPER_ANGLE = -65.0
DIPPER_L = 0.62
DIPPER_Z = 0.046
LEVER_ANGLE = 105.0
LEVER_L = 0.30
LEVER_Z = 0.10
BOOM_PIVOT = (XP + 0.12, 0.61)
SHELL_SEGMENTS = [(0.035, -0.03), (0.085, -0.09), (0.075, -0.16), (0.015, -0.2), (-0.06, -0.2), (-0.15, -0.165)]
LEVER_PIVOT = (XP - 0.08, 0.597)
EYE_CENTRE = (XP, 0.6215)

T = lambda en, de: {"en": en, "de": de}

BONES = [
    dict(id="ground", parent=None, x=0.0, y=0.0, angleDeg=0.0, lengthM=0.04, joint={"type": "fixed"}, title=T("Ground datum at lawn level", "Bezugspunkt auf Rasenhöhe")),
    dict(id="frame", parent="ground", x=-FRAME_OUT, y=COURSE, angleDeg=0.0, lengthM=2 * FRAME_OUT, joint={"type": "fixed"}, title=T("Timber edging frame with seat boards", "Holzeinfassung mit Sitzbrettern")),
    dict(id="sand", parent="ground", x=0.0, y=0.06, angleDeg=0.0, lengthM=0.04, joint={"type": "fixed"}, title=T("Sand fill, heap and the toys left on it", "Sandfüllung, Haufen und die liegen gebliebenen Spielsachen")),
    dict(id="sleeve", parent="ground", x=XP, y=0.0, angleDeg=90.0, lengthM=0.13, joint={"type": "fixed"}, title=T("Ground sleeve with collar on the point foundation", "Bodenhülse mit Kragen auf dem Punktfundament")),
    dict(id="post", parent="ground", x=XP, y=0.0, angleDeg=90.0, lengthM=0.50, joint={"type": "prismatic", "axisDeg": 90, "minM": 0.0, "maxM": 0.8}, title=T("Removable post with bearing housing", "Herausnehmbarer Pfosten mit Lagergehäuse")),
    dict(id="turret", parent="post", x=0.50, y=0.0, angleDeg=0.0, lengthM=0.04, joint={"type": "revolute", "continuousTwist": True}, title=T("Swivel bearing (turret)", "Drehlager (Drehkranz)")),
    dict(id="beam", parent="turret", x=0.04, y=0.42, angleDeg=-90.0, lengthM=0.56, joint={"type": "fixed"}, title=T("Turret beam with seat and backrest", "Drehträger mit Sitz und Rückenlehne")),
    dict(id="boom", parent="beam", x=0.54, y=0.07, angleDeg=BOOM_ANGLE, lengthM=BOOM_L, joint={"type": "revolute", "minDeg": -30, "maxDeg": 25}, title=T("Boom", "Ausleger")),
    dict(id="dipper", parent="boom", x=BOOM_L, y=0.0, z=DIPPER_Z, angleDeg=DIPPER_ANGLE - BOOM_ANGLE, lengthM=DIPPER_L, joint={"type": "revolute", "minDeg": -40, "maxDeg": 40}, title=T("Dipper arm", "Löffelstiel")),
    dict(id="bucket", parent="dipper", x=DIPPER_L, y=0.0, angleDeg=-90.0 - DIPPER_ANGLE, lengthM=0.16, joint={"type": "revolute", "minDeg": -70, "maxDeg": 50}, title=T("Scoop bucket", "Baggerschaufel")),
    dict(id="lever-boom", parent="beam", x=0.34, y=0.057, z=LEVER_Z, angleDeg=LEVER_ANGLE, lengthM=LEVER_L, joint={"type": "revolute", "minDeg": -25, "maxDeg": 25}, title=T("Left hand lever (boom)", "Linker Handhebel (Ausleger)")),
    dict(id="lever-dipper", parent="beam", x=0.34, y=0.057, z=-LEVER_Z, angleDeg=LEVER_ANGLE, lengthM=LEVER_L, joint={"type": "revolute", "minDeg": -25, "maxDeg": 25}, title=T("Right hand lever (dipper)", "Rechter Handhebel (Löffelstiel)")),
    dict(id="lift-eye", parent="beam", x=0.42, y=0.0815, angleDeg=90.0, lengthM=0.04, joint={"type": "fixed"}, title=T("Lifting eye nut DIN 582 on the turret", "Ringmutter DIN 582 auf dem Drehträger")),
]
SOLID_MODE = {"turret", "beam", "boom", "dipper", "bucket", "lever-boom", "lever-dipper", "lift-eye"}


def frames():
    """🧭 World rest origin, angle and lateral offset of every bone of the planar rest chain."""
    out = {}
    for b in BONES:
        if b["parent"] is None:
            out[b["id"]] = (b["x"], b["y"], b.get("z", 0.0), b["angleDeg"])
            continue
        px, py, pz, pa = out[b["parent"]]
        r = math.radians(pa)
        ox = px + b["x"] * math.cos(r) - b["y"] * math.sin(r)
        oy = py + b["x"] * math.sin(r) + b["y"] * math.cos(r)
        out[b["id"]] = (ox, oy, pz + b.get("z", 0.0), pa + b["angleDeg"])
    return out


FR = frames()


def local(bone, X, Y):
    """📐 World rest point (X, Y) in the (u, v) frame of a bone."""
    ox, oy, _, a = FR[bone]
    r = math.radians(a)
    dx, dy = X - ox, Y - oy
    return (round(dx * math.cos(r) + dy * math.sin(r), 6), round(-dx * math.sin(r) + dy * math.cos(r), 6))


def world(bone, u, v):
    """🌍 Bone-local (u, v) back to world rest (X, Y)."""
    ox, oy, _, a = FR[bone]
    r = math.radians(a)
    return (ox + u * math.cos(r) - v * math.sin(r), oy + u * math.sin(r) + v * math.cos(r))


def bz(bone):
    """↔️ World rest z of a bone origin."""
    return FR[bone][2]


def circle(cx, cy, r, n=24, start=0.0, sweep=2 * math.pi):
    """⭕ Polygon points on a circle (or arc)."""
    full = abs(sweep - 2 * math.pi) < 1e-9
    m = n if full else n + 1
    return [(cx + r * math.cos(start + sweep * k / n), cy + r * math.sin(start + sweep * k / n)) for k in range(m)]


def rectpts(x0, y0, x1, y1):
    """⬛ Counter-clockwise rectangle corners."""
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def stadium(p0, p1, r, n=8):
    """💊 Outline of a bar of half width r between two pin centres with round ends."""
    a = math.atan2(p1[1] - p0[1], p1[0] - p0[0])
    pts = circle(p1[0], p1[1], r, n, a - math.pi / 2, math.pi) + circle(p0[0], p0[1], r, n, a + math.pi / 2, math.pi)
    return pts


def thick_line(pts, t):
    """🪚 A sheet of thickness t along a polyline, offset to its left side."""
    left = []
    for i, p in enumerate(pts):
        a = pts[max(i - 1, 0)]
        b = pts[min(i + 1, len(pts) - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        n = math.hypot(dx, dy)
        left.append((p[0] - dy / n * t, p[1] + dx / n * t))
    return list(pts) + left[::-1]


def area(pts):
    """📏 Signed shoelace area."""
    return 0.5 * sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1] for i in range(len(pts)))


def ccw(pts):
    """🔄 The polygon counter-clockwise."""
    return pts if area(pts) > 0 else pts[::-1]


def centroid(pts):
    """🎯 Centroid of a polygon."""
    a = area(pts)
    cx = sum((pts[i][0] + pts[(i + 1) % len(pts)][0]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts))) / (6 * a)
    cy = sum((pts[i][1] + pts[(i + 1) % len(pts)][1]) * (pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]) for i in range(len(pts))) / (6 * a)
    return cx, cy


def r6(p):
    """✂️ Rounded point."""
    return [round(p[0], 6), round(p[1], 6)]


MATERIALS = [
    dict(id="larch-weathered", title=T("Larch, weathered silver-grey", "Lärche, silbergrau verwittert"), emissivity=0.9, solarAbsorptivity=0.7, densityKgM3=590, conductivityWmK=0.13, specificHeatJKgK=1600, elasticModulusPa=11000000000, strengthPa=24000000, source="authored-estimate", reference="iso-10456-2007"),
    dict(id="larch-fresh", title=T("Larch, fresh-sawn replacement", "Lärche, frisch eingesetzt"), emissivity=0.9, solarAbsorptivity=0.5, densityKgM3=590, conductivityWmK=0.13, specificHeatJKgK=1600, elasticModulusPa=11000000000, strengthPa=24000000, source="authored-estimate", reference="iso-10456-2007"),
    dict(id="play-sand", title=T("Play sand 0/2, dry surface", "Spielsand 0/2, trockene Oberfläche"), emissivity=0.9, solarAbsorptivity=0.6, densityKgM3=1515, conductivityWmK=0.27, specificHeatJKgK=800, source="literature", reference="incropera-2018"),
    dict(id="steel-painted", title=T("Steel, powder-coated, chalked and flaking", "Stahl, pulverbeschichtet, verkreidet und abblätternd"), emissivity=0.9, solarAbsorptivity=0.65, densityKgM3=7850, conductivityWmK=50, specificHeatJKgK=450, elasticModulusPa=210000000000, strengthPa=235000000, source="authored-estimate", reference="en-1993-1-1"),
    dict(id="steel-worn", title=T("Steel sheet, paint worn off, rust film", "Stahlblech, Lack abgerieben, Flugrost"), emissivity=0.7, solarAbsorptivity=0.75, densityKgM3=7850, conductivityWmK=50, specificHeatJKgK=450, elasticModulusPa=210000000000, strengthPa=235000000, source="authored-estimate", reference="incropera-2018"),
    dict(id="steel-galvanised-weathered", title=T("Hot-dip galvanised steel, dull weathered", "Feuerverzinkter Stahl, matt verwittert"), emissivity=0.5, solarAbsorptivity=0.65, densityKgM3=7850, conductivityWmK=50, specificHeatJKgK=450, elasticModulusPa=210000000000, strengthPa=235000000, source="authored-estimate", reference="incropera-2018"),
    dict(id="hdpe-board", title=T("HDPE playground board, UV-faded red", "HDPE-Spielplatzplatte, UV-verblasst rot"), emissivity=0.92, solarAbsorptivity=0.7, densityKgM3=980, conductivityWmK=0.5, specificHeatJKgK=1800, elasticModulusPa=1000000000, source="literature", reference="iso-10456-2007"),
    dict(id="polypropylene-toy", title=T("Polypropylene toys, sun-bleached", "Polypropylen-Spielzeug, ausgebleicht"), emissivity=0.9, solarAbsorptivity=0.5, densityKgM3=910, conductivityWmK=0.22, specificHeatJKgK=1800, source="literature", reference="iso-10456-2007"),
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}


def P(id, bone, mat, region, kind, role="body", tone=0.4, **g):
    """🧩 One authored part in world rest coordinates."""
    return dict(id=id, bone=bone, mat=mat, region=region, kind=kind, role=role, tone=tone, **g)


def frame_parts(s):
    """🪵 Timber edging, seat boards and the replaced front board at scale s."""
    f = FEATURE[s]
    parts = []
    o, i = FRAME_OUT, INNER
    mid = o - TIMBER / 2
    if s in (50, 100):
        for c in range(2):
            y0, y1 = c * COURSE, (c + 1) * COURSE
            longx = c == 0
            for side, zc in (("right", -mid), ("left", mid)):
                x0, x1 = (-o, o) if longx else (-i, i)
                reg = "frame-timber-mossy" if side == "left" else "frame-timber"
                parts.append(P(f"course{c + 1}-{side}", "frame", "larch-weathered", reg, "box", tone=0.6 if side == "right" else 0.5, x0=x0, x1=x1, y0=y0, y1=y1, z0=zc - TIMBER / 2, z1=zc + TIMBER / 2))
            for side, xc in (("front", mid), ("back", -mid)):
                z0, z1 = (-i, i) if longx else (-o, o)
                parts.append(P(f"course{c + 1}-{side}", "frame", "larch-weathered", "frame-timber", "box", tone=0.56, x0=xc - TIMBER / 2, x1=xc + TIMBER / 2, y0=y0, y1=y1, z0=z0, z1=z1))
        if s == 100:
            for p in parts:
                p["id"] = p["id"]
    else:
        h = 2 * COURSE if s == 200 else 0.25
        for side, zc in (("right", -mid), ("left", mid)):
            reg = "frame-timber-mossy" if side == "left" else "frame-timber"
            parts.append(P(f"edge-{side}", "frame", "larch-weathered", reg, "box", tone=0.6 if side == "right" else 0.5, x0=-o, x1=o, y0=0.0, y1=h, z0=zc - TIMBER / 2, z1=zc + TIMBER / 2))
        for side, xc in (("front", mid), ("back", -mid)):
            parts.append(P(f"edge-{side}", "frame", "larch-weathered", "frame-timber", "box", tone=0.56, x0=xc - TIMBER / 2, x1=xc + TIMBER / 2, y0=0.0, y1=h, z0=-i, z1=i))
    bt = BOARD_T if s != 500 else 0.075
    by0 = FRAME_TOP - bt
    q = o - BOARD_W
    boards = {
        "right": [(-o, -o), (o, -o), (q, -q), (-q, -q)],
        "left": [(o, o), (-o, o), (-q, q), (q, q)],
        "front": [(o, -o), (o, o), (q, q), (q, -q)],
        "back": [(-o, o), (-o, -o), (-q, -q), (-q, q)],
    }
    if s == 500:
        boards = {
            "right": [(-o, -o), (o, -o), (o, -q), (-o, -q)],
            "left": [(o, o), (-o, o), (-o, q), (o, q)],
            "front": [(o, -q), (o, q), (q, q), (q, -q)],
            "back": [(-o, q), (-o, -q), (-q, -q), (-q, q)],
        }
    for side, pts in boards.items():
        fresh = side == "front"
        parts.append(P(f"board-{side}", "frame", "larch-fresh" if fresh else "larch-weathered", "seat-board-new" if fresh else "seat-boards", "pprism", tone=0.86 if fresh else 0.66, pts=pts, y0=by0, y1=FRAME_TOP))
    return parts


def sand_parts(s):
    """⏳ Sand fill sunk to 120 mm above the lawn, the dug pit under the scoop, the heap and the toys left behind."""
    i = INNER
    parts = []
    pit_c = (XP + 0.92 - 0.035, DIPPER_Z)
    sq = [(-i, -i), (i, -i), (i, i), (-i, i)]
    if s in (50, 100, 200):
        parts.append(P("sand-bed", "sand", "play-sand", "sand", "pprism", tone=0.84, pts=sq, y0=0.0, y1=SAND_SPLIT))
        pit = circle(pit_c[0], pit_c[1], 0.25, 24 if s == 50 else 16)
        parts.append(P("sand-surface", "sand", "play-sand", "sand", "pprism", tone=0.84, pts=sq, holes=[pit], y0=SAND_SPLIT, y1=SAND_TOP))
    else:
        parts.append(P("sand-fill", "sand", "play-sand", "sand", "pprism", tone=0.84, pts=sq, y0=0.0, y1=SAND_TOP))
    parts.append(P("sand-heap", "sand", "play-sand", "sand", "ellipsoid", tone=0.8, c=(0.78, SAND_TOP, 0.62), r=(0.34, 0.09 if s != 500 else 0.1, 0.27)))
    parts.append(P("toy-bucket", "sand", "polypropylene-toy", "toys", "frustum", tone=0.72, cx=1.05, cz=-0.78, r0=0.085, r1=0.066, y0=SAND_TOP - 0.002, y1=SAND_TOP + 0.14))
    if s in (50, 100):
        blade = [(0.30, -1.26), (0.42, -1.30), (0.42, -1.18), (0.30, -1.22)]
        if s == 50:
            blade = [(0.28, -1.255), (0.30, -1.268), (0.42, -1.300), (0.43, -1.240), (0.42, -1.180), (0.30, -1.212), (0.28, -1.225)]
        parts.append(P("toy-spade-blade", "sand", "polypropylene-toy", "toys", "pprism", tone=0.72, pts=blade, y0=SAND_TOP, y1=SAND_TOP + 0.004, sheet=True))
        hr = 0.011 if s == 50 else 0.0115
        parts.append(P("toy-spade-handle", "sand", "polypropylene-toy", "toys", "capsule3", tone=0.72, p0=(0.29, SAND_TOP + hr, -1.24), p1=(0.07, SAND_TOP + hr, -1.19), r=hr))
    return parts


def digger_parts(s):
    """🚜 Sleeve, post, swivel turret with seat, levers, boom, dipper and scoop at scale s."""
    f = FEATURE[s]
    parts = []
    if s in (50, 100):
        ct = 0.010 if s == 50 else 0.015
        parts.append(P("sleeve-tube", "sleeve", "steel-galvanised-weathered", "sleeve", "vcyl", tone=0.62, cx=XP, cz=0.0, r=0.05715, ri=0.05215, y0=0.0, y1=SAND_TOP + 0.01 - ct))
        parts.append(P("sleeve-collar", "sleeve", "steel-galvanised-weathered", "sleeve", "vcyl", tone=0.62, cx=XP, cz=0.0, r=0.08, ri=0.0522, y0=SAND_TOP + 0.01 - ct, y1=SAND_TOP + 0.01))
        parts.append(P("post", "post", "steel-painted", "digger-paint", "vcyl", tone=0.62, cx=XP, cz=0.0, r=0.0508, y0=0.0, y1=0.42, mkg=0.001226 * 0.42 * 7850))
    else:
        parts.append(P("sleeve-tube", "sleeve", "steel-galvanised-weathered", "sleeve", "vcyl", tone=0.62, cx=XP, cz=0.0, r=0.08, y0=0.0, y1=SAND_TOP + 0.01, mkg=0.001717 * 0.12 * 7850 + 0.9))
        parts.append(P("post", "post", "steel-painted", "digger-paint", "vcyl", tone=0.62, cx=XP, cz=0.0, r=0.0508, y0=SAND_TOP + 0.01, y1=0.42, mkg=0.001226 * 0.42 * 7850))
    parts.append(P("bearing-housing", "post", "steel-painted", "digger-paint", "vcyl", tone=0.55, cx=XP, cz=0.0, r=0.07, y0=0.42, y1=0.50, mkg=0.003311 * 0.08 * 7850 + 0.6))
    bw = 0.03 if s != 500 else 0.0375
    parts.append(P("beam", "beam", "steel-painted", "digger-paint", "sprism", tone=0.6, pts=rectpts(XP - 0.42, 0.50, XP + 0.14, 0.58), z0=-bw, z1=bw, mkg=0.001056 * 0.56 * 7850))
    st = max(0.019, f) if s != 500 else 0.075
    seat_top = 0.599
    parts.append(P("seat-pan", "beam", "hdpe-board", "digger-seat", "sprism", tone=0.42, pts=rectpts(XP - 0.42, seat_top - st, XP - 0.12, seat_top), z0=-0.15, z1=0.15, mkg=0.30 * 0.30 * 0.019 * 980))
    if s in (50, 100):
        parts.append(P("backrest", "beam", "hdpe-board", "digger-seat", "sprism", tone=0.42, pts=[(XP - 0.42, seat_top), (XP - 0.401, seat_top), (XP - 0.401, 0.85), (XP - 0.405, 0.857), (XP - 0.412, 0.86), (XP - 0.42, 0.86)], z0=-0.15, z1=0.15, mkg=0.30 * 0.261 * 0.019 * 980))
        sth = 0.008 if s == 50 else 0.015
        parts.append(P("backrest-strap", "beam", "steel-painted", "digger-paint", "sprism", tone=0.58, pts=rectpts(XP - 0.42 - sth, 0.50, XP - 0.42, 0.84), z0=-0.03, z1=0.03, mkg=0.008 * 0.06 * 0.34 * 7850))
    else:
        d = 0.03 if s == 200 else 0.075
        parts.append(P("backrest", "beam", "hdpe-board", "digger-seat", "sprism", tone=0.42, pts=rectpts(XP - 0.401 - d, 0.50 if s == 500 else seat_top, XP - 0.401, 0.86), z0=-0.15, z1=0.15, mkg=0.30 * 0.261 * 0.019 * 980 + 0.008 * 0.06 * 0.34 * 7850))
    lt_r = 0.01685 if s != 500 else 0.0385
    parts.append(P("lever-tube", "beam", "steel-painted", "digger-paint", "sprism", tone=0.55, pts=circle(LEVER_PIVOT[0], LEVER_PIVOT[1], lt_r / math.cos(math.pi / 16), 16), z0=-0.125 if s != 500 else -0.145, z1=0.125 if s != 500 else 0.145, mkg=0.000393 * 0.25 * 7850))
    lr = 0.0125 if s in (50, 100) else (0.015 if s == 200 else 0.04)
    for lid, side in (("lever-boom", 1), ("lever-dipper", -1)):
        tip = world(lid, LEVER_L, 0.0)
        base = LEVER_PIVOT
        parts.append(P(f"{lid}-rod", lid, "steel-painted", "digger-paint", "capsule2", tone=0.6, p0=base, p1=tip, r=lr, z=side * LEVER_Z, mkg=4.909e-4 * LEVER_L * 7850))
        if s != 500:
            parts.append(P(f"{lid}-grip", lid, "hdpe-board", "digger-seat", "sphere", tone=0.38, c=tip, r=0.0225, z=side * LEVER_Z, mkg=0.045))
    if s in (50, 100):
        t = 0.010 if s == 50 else 0.015
        parts.append(P("eye-nut-base", "lift-eye", "steel-painted", "digger-paint", "sprism", tone=0.5, pts=rectpts(XP - 0.0175, 0.58, XP + 0.0175, 0.59), z0=-0.0175, z1=0.0175, mkg=0.06))
        ring_t = 0.014 if s == 50 else 0.015
        band = 0.014 if s == 50 else 0.015
        parts.append(P("eye-nut-ring", "lift-eye", "steel-painted", "digger-paint", "sprism", tone=0.5, pts=circle(EYE_CENTRE[0], EYE_CENTRE[1], 0.0315, 20), holes=[circle(EYE_CENTRE[0], EYE_CENTRE[1], 0.0315 - band, 16)], z0=-ring_t / 2, z1=ring_t / 2, mkg=0.14))
    elif s == 200:
        parts.append(P("eye-nut", "lift-eye", "steel-painted", "digger-paint", "sprism", tone=0.5, pts=[(XP - 0.0315, 0.58), (XP + 0.0315, 0.58), (XP + 0.0315, 0.62), (XP + 0.02, 0.645), (XP - 0.02, 0.645), (XP - 0.0315, 0.62)], z0=-0.015, z1=0.015, mkg=0.2))
    bp = BOOM_PIVOT
    btip = world("boom", BOOM_L, 0.0)
    dtip = world("dipper", DIPPER_L, 0.0)
    bh = 0.025 if s in (50, 100, 200) else 0.04
    dh = 0.02 if s in (50, 100, 200) else 0.04
    parts.append(P("boom-arm", "boom", "steel-painted", "digger-paint", "sprism", tone=0.66, pts=stadium(bp, btip, bh), z0=-bh, z1=bh, mkg=0.000564 * 0.63 * 7850))
    if s == 500:
        parts.append(P("dipper-arm", "dipper", "steel-painted", "digger-paint", "sprism", tone=0.66, pts=stadium(btip, dtip, dh), z0=-0.04, z1=0.04, mkg=0.000444 * 0.66 * 7850))
    else:
        parts.append(P("dipper-arm", "dipper", "steel-painted", "digger-paint", "sprism", tone=0.66, pts=stadium(btip, dtip, dh), z0=DIPPER_Z - 0.02, z1=DIPPER_Z + 0.02, mkg=0.000444 * 0.66 * 7850))
    if s in (50, 100):
        lt = 0.010 if s == 50 else 0.015
        lug = [(bp[0] - 0.04, 0.58), (bp[0] + 0.02, 0.58)] + circle(bp[0], bp[1], 0.022, 8, 0.0, math.pi)[1:]
        lug = [(bp[0] - 0.04, 0.58), (bp[0] + 0.022, 0.58)] + circle(bp[0], bp[1], 0.022, 8, 0.0, math.pi)[1:] + [(bp[0] - 0.04, bp[1] - 0.005)]
        parts.append(P("boom-lug-left", "beam", "steel-painted", "digger-paint", "sprism", tone=0.55, pts=lug, z0=0.026, z1=0.026 + lt, mkg=0.25))
        parts.append(P("boom-lug-right", "beam", "steel-painted", "digger-paint", "sprism", tone=0.55, pts=lug, z0=-0.026 - lt, z1=-0.026, mkg=0.25))
        parts.append(P("boom-pin", "boom", "steel-painted", "digger-paint", "sprism", tone=0.4, pts=circle(bp[0], bp[1], 0.01, 12), z0=-0.026 - lt - 0.004, z1=0.026 + lt + 0.004, mkg=0.05))
        parts.append(P("dipper-pin", "dipper", "steel-painted", "digger-paint", "sprism", tone=0.4, pts=circle(btip[0], btip[1], 0.01, 12), z0=-0.03, z1=DIPPER_Z + 0.025, mkg=0.05))
    shell_w = [(0.035, -0.03), (0.07, -0.06), (0.085, -0.10), (0.08, -0.14), (0.055, -0.175), (0.015, -0.195), (-0.04, -0.20), (-0.10, -0.19), (-0.15, -0.165)]
    shell_world = [(dtip[0] + a, dtip[1] + b) for a, b in shell_w]
    seg_world = [(dtip[0] + a, dtip[1] + b) for a, b in SHELL_SEGMENTS]
    bz0, bz1 = DIPPER_Z - 0.09, DIPPER_Z + 0.09
    if s in (50, 100):
        for j in range(len(seg_world) - 1):
            parts.append(P(f"bucket-shell-{j}", "bucket", "steel-worn", "digger-bucket", "sheetbox", tone=0.45, a=seg_world[j], b=seg_world[j + 1], t=0.003, pts=thick_line([seg_world[j], seg_world[j + 1]], 0.003), z0=bz0, z1=bz1, sheet=True, mkg=math.dist(seg_world[j], seg_world[j + 1]) * 0.18 * 0.003 * 7850))
        side = seg_world + [(dtip[0] - 0.05, dtip[1] - 0.06)]
        parts.append(P("bucket-side-left", "bucket", "steel-worn", "digger-bucket", "sprism", tone=0.5, pts=side, z0=bz1 - 0.003, z1=bz1, sheet=True, mkg=abs(area(side)) * 0.003 * 7850))
        parts.append(P("bucket-side-right", "bucket", "steel-worn", "digger-bucket", "sprism", tone=0.5, pts=side, z0=bz0, z1=bz0 + 0.003, sheet=True, mkg=abs(area(side)) * 0.003 * 7850))
        lt = 0.010 if s == 50 else 0.015
        blug = [(dtip[0] + a, dtip[1] + b) for a, b in [(-0.022, 0.0), (-0.0156, 0.0156), (0.0, 0.022), (0.0156, 0.0156), (0.022, 0.0), (0.06, -0.055), (0.03, -0.045), (0.0, -0.03)]]
        parts.append(P("bucket-lug-left", "bucket", "steel-worn", "digger-bucket", "sprism", tone=0.45, pts=blug, z0=DIPPER_Z + 0.021, z1=DIPPER_Z + 0.021 + lt, mkg=0.2))
        parts.append(P("bucket-lug-right", "bucket", "steel-worn", "digger-bucket", "sprism", tone=0.45, pts=blug, z0=DIPPER_Z - 0.021 - lt, z1=DIPPER_Z - 0.021, mkg=0.2))
        parts.append(P("bucket-pin", "bucket", "steel-painted", "digger-paint", "sprism", tone=0.4, pts=circle(dtip[0], dtip[1], 0.01, 12), z0=DIPPER_Z - 0.021 - lt - 0.004, z1=DIPPER_Z + 0.021 + lt + 0.004, mkg=0.05))
    else:
        wedge = [(dtip[0] - 0.03, dtip[1] + 0.02), (dtip[0] + 0.03, dtip[1] + 0.02)] + seg_world[1:]
        parts.append(P("bucket-block", "bucket", "steel-worn", "digger-bucket", "sprism", tone=0.48, pts=wedge, z0=bz0, z1=bz1, mkg=2.7 + 0.4))
    return parts


def all_parts(s):
    """📦 Every part of the asset at scale s."""
    return frame_parts(s) + sand_parts(s) + digger_parts(s)


def bbox_world(p):
    """📦 World rest bounding box (x0, x1, y0, y1, z0, z1) of a part."""
    k = p["kind"]
    if k == "box":
        return p["x0"], p["x1"], p["y0"], p["y1"], p["z0"], p["z1"]
    if k in ("sprism", "sheetbox"):
        xs = [q[0] for q in p["pts"]]
        ys = [q[1] for q in p["pts"]]
        return min(xs), max(xs), min(ys), max(ys), p["z0"], p["z1"]
    if k == "pprism":
        xs = [q[0] for q in p["pts"]]
        zs = [q[1] for q in p["pts"]]
        return min(xs), max(xs), p["y0"], p["y1"], min(zs), max(zs)
    if k == "vcyl":
        return p["cx"] - p["r"], p["cx"] + p["r"], p["y0"], p["y1"], p["cz"] - p["r"], p["cz"] + p["r"]
    if k == "frustum":
        r = max(p["r0"], p["r1"])
        return p["cx"] - r, p["cx"] + r, p["y0"], p["y1"], p["cz"] - r, p["cz"] + r
    if k == "ellipsoid":
        (cx, cy, cz), (rx, ry, rz) = p["c"], p["r"]
        return cx - rx, cx + rx, cy - ry, cy + ry, cz - rz, cz + rz
    if k == "sphere":
        (cx, cy), r = p["c"], p["r"]
        return cx - r, cx + r, cy - r, cy + r, p["z"] - r, p["z"] + r
    if k == "capsule2":
        (a, b), (c, d), r = p["p0"], p["p1"], p["r"]
        return min(a, c) - r, max(a, c) + r, min(b, d) - r, max(b, d) + r, p["z"] - r, p["z"] + r
    if k == "capsule3":
        p0, p1, r = p["p0"], p["p1"], p["r"]
        return min(p0[0], p1[0]) - r, max(p0[0], p1[0]) + r, min(p0[1], p1[1]) - r, max(p0[1], p1[1]) + r, min(p0[2], p1[2]) - r, max(p0[2], p1[2]) + r
    raise ValueError(k)


def volume_of(p):
    """⚖️ Solid volume of a part in cubic metres."""
    k = p["kind"]
    if k == "box":
        return (p["x1"] - p["x0"]) * (p["y1"] - p["y0"]) * (p["z1"] - p["z0"])
    if k in ("sprism", "sheetbox"):
        a = abs(area(p["pts"])) - sum(abs(area(h)) for h in p.get("holes", []))
        return a * (p["z1"] - p["z0"])
    if k == "pprism":
        a = abs(area(p["pts"])) - sum(abs(area(h)) for h in p.get("holes", []))
        return a * (p["y1"] - p["y0"])
    if k == "vcyl":
        return math.pi * (p["r"] ** 2 - p.get("ri", 0.0) ** 2) * (p["y1"] - p["y0"])
    if k == "frustum":
        h = p["y1"] - p["y0"]
        return math.pi * h / 3 * (p["r0"] ** 2 + p["r0"] * p["r1"] + p["r1"] ** 2)
    if k == "ellipsoid":
        rx, ry, rz = p["r"]
        return 2.0 / 3.0 * math.pi * rx * ry * rz
    if k == "sphere":
        return 4.0 / 3.0 * math.pi * p["r"] ** 3
    if k == "capsule2":
        (a, b), (c, d), r = p["p0"], p["p1"], p["r"]
        return math.pi * r * r * math.hypot(c - a, d - b) + 4.0 / 3.0 * math.pi * r ** 3
    if k == "capsule3":
        p0, p1, r = p["p0"], p["p1"], p["r"]
        return math.pi * r * r * math.dist(p0, p1) + 4.0 / 3.0 * math.pi * r ** 3
    raise ValueError(k)


def centre_of(p):
    """🎯 World rest centre (x, y) of a part."""
    x0, x1, y0, y1, _, _ = bbox_world(p)
    if p["kind"] == "sprism" and not p.get("holes"):
        return centroid(p["pts"])
    if p["kind"] == "ellipsoid":
        return p["c"][0], p["c"][1] + 3.0 / 8.0 * p["r"][1]
    return (x0 + x1) / 2, (y0 + y1) / 2


def mass_of(p):
    """⚖️ Mass of a part: the section mass where the true section is hollow, else volume times density."""
    return p.get("mkg", volume_of(p) * DENSITY[p["mat"]])


def solid_of(p, s):
    """🧊 The bone-local 3D solid of a part."""
    b = p["bone"]
    k = p["kind"]
    base = {"id": p["id"], "bone": b, "material": p["mat"], "role": p["role"], "tone": p["tone"]}
    ox, oy, oz, a = FR[b]
    if p.get("sheet"):
        base["sheet"] = True
    if k == "box":
        if a == 0.0:
            cx, cy = local(b, (p["x0"] + p["x1"]) / 2, (p["y0"] + p["y1"]) / 2)
            base.update(kind="box", size=[round(p["x1"] - p["x0"], 6), round(p["y1"] - p["y0"], 6), round(p["z1"] - p["z0"], 6)], transform={"translation": [cx, cy, round((p["z0"] + p["z1"]) / 2 - oz, 6)]})
            return base
        raise ValueError("box on rotated bone")
    if k == "sheetbox":
        a0, a1 = local(b, *p["a"]), local(b, *p["b"])
        L = math.dist(a0, a1)
        phi = math.atan2(a1[1] - a0[1], a1[0] - a0[0])
        n = (-math.sin(phi), math.cos(phi))
        c = ((a0[0] + a1[0]) / 2 + n[0] * p["t"] / 2, (a0[1] + a1[1]) / 2 + n[1] * p["t"] / 2)
        base.update(kind="box", size=[round(L, 6), p["t"], round(p["z1"] - p["z0"], 6)], transform={"rotation": [0.0, 0.0, round(math.sin(phi / 2), 7), round(math.cos(phi / 2), 7)], "translation": [round(c[0], 6), round(c[1], 6), round((p["z0"] + p["z1"]) / 2 - oz, 6)]})
        return base
    if k == "sprism":
        prof = ccw([local(b, X, Y) for X, Y in p["pts"]])
        base.update(kind="extrude", plane="xy", profile=[list(q) for q in prof], **{"from": round(p["z0"] - oz, 6), "to": round(p["z1"] - oz, 6)})
        if p.get("holes"):
            base["holes"] = [[list(q) for q in ccw([local(b, X, Y) for X, Y in h])] for h in p["holes"]]
        return base
    if k == "pprism":
        if a == 0.0:
            prof = ccw([(round(Z - oz, 6), round(X - ox, 6)) for X, Z in p["pts"]])
            base.update(kind="extrude", plane="zx", profile=[list(q) for q in prof], **{"from": round(p["y0"] - oy, 6), "to": round(p["y1"] - oy, 6)})
            if p.get("holes"):
                base["holes"] = [[list(q) for q in ccw([(round(Z - oz, 6), round(X - ox, 6)) for X, Z in h])] for h in p["holes"]]
            return base
        raise ValueError("pprism on rotated bone")
    if k in ("vcyl", "frustum"):
        r0 = p["r"] if k == "vcyl" else p["r0"]
        r1 = p["r"] if k == "vcyl" else p["r1"]
        L = round(p["y1"] - p["y0"], 6)
        if a == 90.0:
            u0, v0 = local(b, p["cx"], p["y0"])
            axis, origin = [1, 0, 0], [u0, v0, round(p["cz"] - oz, 6)]
        else:
            u0, v0 = local(b, p["cx"], p["y0"])
            axis, origin = [0, 1, 0], [u0, v0, round(p["cz"] - oz, 6)]
        if p.get("ri"):
            ri = p["ri"]
            prof = [[0.0, ri], [0.0, r0], [L, r1], [L, ri], [0.0, ri]]
        else:
            prof = [[0.0, r0], [L, r1]]
        base.update(kind="revolve", profile=prof, axis=axis, origin=origin)
        return base
    if k == "ellipsoid":
        (cx, cy, cz), (rx, ry, rz) = p["c"], p["r"]
        u, v = local(b, cx, cy)
        base.update(kind="ellipsoid", radii=[rx, ry, rz], transform={"translation": [u, v, round(cz - oz, 6)]})
        return base
    if k == "sphere":
        u, v = local(b, *p["c"])
        base.update(kind="ellipsoid", radii=[p["r"]] * 3, transform={"translation": [u, v, round(p["z"] - oz, 6)]})
        return base
    if k == "capsule2":
        u0, v0 = local(b, *p["p0"])
        u1, v1 = local(b, *p["p1"])
        w = round(p["z"] - oz, 6)
        base.update(kind="capsule", p0=[u0, v0, w], p1=[u1, v1, w], r0=p["r"], r1=p["r"])
        return base
    if k == "capsule3":
        a0 = local(b, p["p0"][0], p["p0"][1])
        a1 = local(b, p["p1"][0], p["p1"][1])
        base.update(kind="capsule", p0=[a0[0], a0[1], round(p["p0"][2] - oz, 6)], p1=[a1[0], a1[1], round(p["p1"][2] - oz, 6)], r0=p["r"], r1=p["r"])
        return base
    raise ValueError(k)


def poly(points, closed=True):
    """🔷 A 2D poly geometry."""
    return {"kind": "poly", "points": [r6(q) for q in points], "closed": closed}


def ring_path(cx, cy, r, ri=None):
    """〰️ A circle, or a ring with its bore as a hole, as a path."""
    def c(rr):
        return f"M {cx + rr:.6f} {cy:.6f} A {rr:.6f} {rr:.6f} 0 1 0 {cx - rr:.6f} {cy:.6f} A {rr:.6f} {rr:.6f} 0 1 0 {cx + rr:.6f} {cy:.6f} Z"
    return {"kind": "path", "d": c(r) + ("" if ri is None else " " + c(ri))}


def poly_path(outer, holes):
    """〰️ A polygon with holes as an even-odd path."""
    def ringd(pts):
        return "M " + " L ".join(f"{q[0]:.6f} {q[1]:.6f}" for q in pts) + " Z"
    return {"kind": "path", "d": " ".join([ringd(outer)] + [ringd(h) for h in holes])}


def shapes_of(p):
    """✏️ The elevation, front and plan geometry of a part."""
    b = p["bone"]
    k = p["kind"]
    x0, x1, y0, y1, z0, z1 = bbox_world(p)
    if b in SOLID_MODE:
        sol = {"kind": "extruded", "sweep": "w", "z0": round(z0, 6), "z1": round(z1, 6)}
        if k in ("sprism", "sheetbox"):
            pts = [local(b, X, Y) for X, Y in p["pts"]]
            g = poly(pts) if not p.get("holes") else poly_path(pts, [[local(b, X, Y) for X, Y in h] for h in p["holes"]])
        elif k == "capsule2":
            a0, a1 = local(b, *p["p0"]), local(b, *p["p1"])
            g = {"kind": "capsule", "x0": a0[0], "y0": a0[1], "x1": a1[0], "y1": a1[1], "r0": p["r"], "r1": p["r"]}
        elif k == "sphere":
            u, v = local(b, *p["c"])
            g = {"kind": "ellipse", "cx": u, "cy": v, "rx": p["r"], "ry": p["r"]}
        else:
            raise ValueError(k)
        return [(view, dict(g, solid=sol)) for view in ("elevation", "front", "plan")]
    ox, oy, oz, a = FR[b]
    if k in ("box", "pprism", "vcyl", "frustum", "ellipsoid", "capsule3"):
        if a == 0.0:
            ex0, ey0 = local(b, x0, y0)
            ew, eh = x1 - x0, y1 - y0
            elev_rect = {"kind": "rect", "x": round(ex0, 6), "y": round(ey0, 6), "w": round(ew, 6), "h": round(eh, 6)}
        else:
            u0, v0 = local(b, x1, y0)
            elev_rect = {"kind": "rect", "x": round(u0, 6), "y": round(v0, 6), "w": round(y1 - y0, 6), "h": round(x1 - x0, 6)}
    out = []
    if k == "box":
        out.append(("elevation", elev_rect))
        out.append(("front", {"kind": "rect", "x": round(z0, 6), "y": round(y0, 6), "w": round(z1 - z0, 6), "h": round(y1 - y0, 6)}))
        out.append(("plan", {"kind": "rect", "x": round(x0, 6), "y": round(z0, 6), "w": round(x1 - x0, 6), "h": round(z1 - z0, 6)}))
    elif k == "pprism":
        out.append(("elevation", elev_rect))
        out.append(("front", {"kind": "rect", "x": round(z0, 6), "y": round(y0, 6), "w": round(z1 - z0, 6), "h": round(y1 - y0, 6)}))
        out.append(("plan", poly(p["pts"]) if not p.get("holes") else poly_path(p["pts"], p["holes"])))
    elif k == "vcyl":
        if a == 90.0:
            u0, _ = local(b, p["cx"], p["y0"])
            r = p["r"]
            elev = {"kind": "rect", "x": round(u0, 6), "y": round(-(p["cx"] + r - ox), 6), "w": round(p["y1"] - p["y0"], 6), "h": round(2 * r, 6)}
        else:
            elev = elev_rect
        out.append(("elevation", elev))
        out.append(("front", {"kind": "rect", "x": round(z0, 6), "y": round(y0, 6), "w": round(z1 - z0, 6), "h": round(y1 - y0, 6)}))
        out.append(("plan", ring_path(p["cx"], p["cz"], p["r"], p.get("ri"))))
    elif k == "frustum":
        cx, cz = p["cx"], p["cz"]
        side = [(cx - p["r0"], p["y0"]), (cx + p["r0"], p["y0"]), (cx + p["r1"], p["y1"]), (cx - p["r1"], p["y1"])]
        out.append(("elevation", poly([local(b, X, Y) for X, Y in side])))
        out.append(("front", poly([(cz - p["r0"], p["y0"]), (cz + p["r0"], p["y0"]), (cz + p["r1"], p["y1"]), (cz - p["r1"], p["y1"])])))
        out.append(("plan", ring_path(cx, cz, max(p["r0"], p["r1"]))))
    elif k == "ellipsoid":
        (cx, cy, cz), (rx, ry, rz) = p["c"], p["r"]
        top = [(cx + rx * math.cos(math.pi * j / 16), cy + ry * math.sin(math.pi * j / 16)) for j in range(17)]
        out.append(("elevation", poly([local(b, X, Y) for X, Y in top])))
        out.append(("front", poly([(cz + rz * math.cos(math.pi * j / 16), cy + ry * math.sin(math.pi * j / 16)) for j in range(17)])))
        out.append(("plan", {"kind": "ellipse", "cx": cx, "cy": cz, "rx": rx, "ry": rz}))
    elif k == "sprism":
        pts = [local(b, X, Y) for X, Y in p["pts"]]
        out.append(("elevation", poly(pts) if not p.get("holes") else poly_path(pts, [[local(b, X, Y) for X, Y in h] for h in p["holes"]])))
        out.append(("front", {"kind": "rect", "x": round(z0, 6), "y": round(y0, 6), "w": round(z1 - z0, 6), "h": round(y1 - y0, 6)}))
        out.append(("plan", {"kind": "rect", "x": round(x0, 6), "y": round(z0, 6), "w": round(x1 - x0, 6), "h": round(z1 - z0, 6)}))
    elif k == "capsule3":
        p0, p1, r = p["p0"], p["p1"], p["r"]
        a0, a1 = local(b, p0[0], p0[1]), local(b, p1[0], p1[1])
        out.append(("elevation", {"kind": "capsule", "x0": a0[0], "y0": a0[1], "x1": a1[0], "y1": a1[1], "r0": r, "r1": r}))
        out.append(("front", {"kind": "capsule", "x0": p0[2], "y0": p0[1], "x1": p1[2], "y1": p1[1], "r0": r, "r1": r}))
        out.append(("plan", {"kind": "capsule", "x0": p0[0], "y0": p0[2], "x1": p1[0], "y1": p1[2], "r0": r, "r1": r}))
    else:
        raise ValueError(k)
    return out


def depth_key(p, view):
    """🎚️ Paint depth of a part in a view: nearer to the viewer paints later."""
    x0, x1, y0, y1, z0, z1 = bbox_world(p)
    if view == "elevation":
        return round(5 - z0, 4)
    if view == "front":
        return round(5 + x1, 4)
    return round(5 + y1, 4)


def surface_of(p):
    """🧮 Plain outer surface of a part, before burial by its neighbours."""
    k = p["kind"]
    if k == "box":
        a, b, c = p["x1"] - p["x0"], p["y1"] - p["y0"], p["z1"] - p["z0"]
        return 2 * (a * b + b * c + c * a)
    if k in ("sprism", "pprism", "sheetbox"):
        pts = p["pts"]
        h = (p["y1"] - p["y0"]) if k == "pprism" else (p["z1"] - p["z0"])
        per = sum(math.dist(pts[i], pts[(i + 1) % len(pts)]) for i in range(len(pts)))
        holes = p.get("holes", [])
        per += sum(sum(math.dist(q[i], q[(i + 1) % len(q)]) for i in range(len(q))) for q in holes)
        a = abs(area(pts)) - sum(abs(area(q)) for q in holes)
        return 2 * a + per * h
    if k == "vcyl":
        L = p["y1"] - p["y0"]
        ri = p.get("ri", 0.0)
        return 2 * math.pi * (p["r"] + ri) * L + 2 * math.pi * (p["r"] ** 2 - ri ** 2)
    if k == "frustum":
        h = p["y1"] - p["y0"]
        sl = math.hypot(h, p["r0"] - p["r1"])
        return math.pi * (p["r0"] + p["r1"]) * sl + math.pi * p["r1"] ** 2
    if k == "ellipsoid":
        rx, ry, rz = p["r"]
        q = 1.6075
        return 0.5 * 4 * math.pi * (((rx * ry) ** q + (rx * rz) ** q + (ry * rz) ** q) / 3) ** (1 / q)
    if k == "sphere":
        return 4 * math.pi * p["r"] ** 2
    if k == "capsule2":
        (a, b), (c, d), r = p["p0"], p["p1"], p["r"]
        return 2 * math.pi * r * math.hypot(c - a, d - b) + 4 * math.pi * r * r
    if k == "capsule3":
        return 2 * math.pi * p["r"] * math.dist(p["p0"], p["p1"]) + 4 * math.pi * p["r"] ** 2
    raise ValueError(k)


def lines(view, bone, sid, pts_list, role="texture", tone=0.3, pen=None, sol=None, filled=False, closed=False, region=None):
    """🖊️ Detail or texture strokes of one view."""
    out = []
    for j, pts in enumerate(pts_list):
        g = dict(id=f"{sid}-{j}", bone=bone, role=role, z=9.0, filled=filled, tone=tone, kind="poly", points=[r6(q) for q in pts], closed=closed)
        if region:
            g["region"] = region
        if pen:
            g["penMm"] = pen
        if sol:
            g["solid"] = sol
        out.append((view, g))
    return out


def wobble(x0, x1, y, amp, n, seed):
    """〰️ A gently wandering crack or grain line."""
    return [(x0 + (x1 - x0) * k / n, y + amp * math.sin(seed + 2.7 * k) * (0.6 + 0.4 * math.cos(seed * 1.3 + k))) for k in range(n + 1)]


def details(s):
    """🔍 Traces of use drawn as strokes where the scale can show them: checks, graffiti, gum, moss, flaking paint, rust, scratches, footprints."""
    out = []
    if s not in (50, 100, 200):
        return out
    fx = lambda X: X + FRAME_OUT
    fy = lambda Y: Y - COURSE
    if s in (50, 100):
        n = 7 if s == 50 else 4
        cracks = [[(fx(x), fy(y)) for x, y in wobble(-1.75 + 0.5 * j, -1.2 + 0.5 * j, 0.2 + 0.025 * (j % 3), 0.006, 6, j)] for j in range(n)]
        out += lines("elevation", "frame", "check-right", cracks, role="texture", region="frame-timber")
        cracks1 = [[(fx(x), fy(y)) for x, y in wobble(-1.7 + 0.55 * j, -1.2 + 0.55 * j, 0.07 + 0.02 * (j % 2), 0.005, 6, j + 3)] for j in range(n - 1)]
        out += lines("elevation", "frame", "check-right-low", cracks1, role="texture", region="frame-timber")
        grain = [wobble(-1.6 + 0.45 * j, -1.1 + 0.45 * j, -1.98 + 0.01 * (j % 3) + 0.0, 0.0, 2, j) for j in range(n)]
        out += lines("plan", "frame", "grain-right", [[(x, -1.93 + 0.05 * math.sin(j)) for x, _ in g] for j, g in enumerate(grain)], role="texture", region="seat-boards")
    tag = [(-0.62, 0.17), (-0.58, 0.25), (-0.54, 0.18), (-0.5, 0.245), (-0.44, 0.19), (-0.4, 0.24), (-0.33, 0.175), (-0.3, 0.23), (-0.24, 0.2)]
    if s in (50, 100):
        out += lines("elevation", "frame", "graffiti-tag", [[(fx(x), fy(y)) for x, y in tag], [(fx(x), fy(y - 0.012)) for x, y in tag[2:7]]], role="detail", tone=0.12, pen=0.25 if s == 50 else 0.18, region="frame-timber")
        sticker = rectpts(fx(0.55), fy(0.19), fx(0.62), fy(0.24))
        out.append(("elevation", dict(id="sticker-residue", bone="frame", role="detail", z=9.1, filled=True, tone=0.92, region="frame-timber", kind="poly", points=[r6(q) for q in sticker], closed=True)))
        front_tag = [(z, y) for y, z in [(0.05, 0.4), (0.11, 0.45), (0.06, 0.52), (0.12, 0.58), (0.05, 0.66), (0.10, 0.7)]]
        out += lines("front", "frame", "graffiti-front", [[(z, y) for z, y in front_tag]], role="detail", tone=0.12, pen=0.25 if s == 50 else 0.18, region="frame-timber")
        gum = [(1.86, -0.5), (1.9, 0.3), (1.79, 1.1), (-1.88, 0.6), (0.4, -1.9), (-0.9, -1.83), (1.2, 1.9)]
        for j, (gx, gz) in enumerate(gum[: 7 if s == 50 else 4]):
            out.append(("plan", dict(id=f"gum-{j}", bone="frame", role="detail", z=9.2, filled=True, tone=0.25, region="seat-board-new" if gx > FRAME_OUT - BOARD_W else "seat-boards", kind="ellipse", cx=gx, cy=gz, rx=0.013 if s == 50 else 0.015, ry=0.011 if s == 50 else 0.015)))
        moss = [(-1.6 + 0.24 * j, 1.94 + 0.03 * math.sin(2.1 * j)) for j in range(14 if s == 50 else 7)]
        for j, (mx, mz) in enumerate(moss):
            out.append(("plan", dict(id=f"moss-{j}", bone="frame", role="texture", z=9.2, filled=True, tone=0.38, region="seat-boards", kind="ellipse", cx=mx, cy=mz, rx=0.03, ry=0.012)))
        prints = [(0.05, -0.35), (0.2, -0.28), (0.95, -0.25), (1.1, -0.36), (-0.2, 0.6), (-0.05, 0.72)]
        for j, (px, pz) in enumerate(prints[: 6 if s == 50 else 4]):
            out.append(("plan", dict(id=f"footprint-{j}", bone="sand", role="texture", z=9.0, filled=False, tone=0.4, region="sand", kind="ellipse", cx=px, cy=pz, rx=0.09, ry=0.035, rotationDeg=20 * (j % 3))))
        boom_sol = {"kind": "extruded", "sweep": "w", "z0": -0.025, "z1": 0.025}
        flakes = [[(0.12, 0.012), (0.17, 0.016), (0.2, 0.004), (0.15, -0.006)], [(0.36, -0.01), (0.4, 0.008), (0.44, 0.0), (0.41, -0.016)]]
        for j, fl in enumerate(flakes):
            out.append(("elevation", dict(id=f"paint-flake-{j}", bone="boom", role="detail", z=9.3, filled=True, tone=0.82, region="digger-paint", kind="poly", points=[r6(q) for q in fl], closed=True, solid=boom_sol)))
        dtip = world("dipper", DIPPER_L, 0.0)
        bsol = {"kind": "extruded", "sweep": "w", "z0": round(DIPPER_Z - 0.09, 6), "z1": round(DIPPER_Z + 0.09, 6)}
        scr = [[local("bucket", dtip[0] + a, dtip[1] + b) for a, b in seg] for seg in ([(-0.09, -0.165), (-0.02, -0.18)], [(-0.06, -0.14), (0.03, -0.165)], [(0.0, -0.1), (0.05, -0.13)])]
        out += lines("elevation", "bucket", "bucket-scratch", scr[: 3 if s == 50 else 2], role="texture" if s == 50 else "detail", tone=0.88, sol=bsol, region="digger-bucket")
        post_rust = [(-(XP + 0.045 - XP), 0.34), (-(XP - 0.03 - XP), 0.34), (-(XP - 0.04 - XP), 0.37), (-(XP + 0.02 - XP), 0.39)]
        out.append(("elevation", dict(id="rust-post", bone="post", role="detail", z=9.3, filled=True, tone=0.3, region="digger-paint", kind="poly", points=[[round(y, 6), round(x, 6)] for x, y in post_rust], closed=True)))
        rust_weld = [(0.40, -0.05), (0.42, -0.045), (0.42, 0.04), (0.40, 0.045)]
        out.append(("elevation", dict(id="rust-weld", bone="post", role="detail", z=9.3, filled=True, tone=0.3, region="digger-paint", kind="poly", points=[r6(q) for q in rust_weld], closed=True)))
    if s == 200:
        out += lines("elevation", "frame", "graffiti-tag", [[(fx(x), fy(y)) for x, y in tag[::2]]], role="detail", tone=0.12, pen=0.13, region="frame-timber")
    return out


SECTIONS = {
    "frame": dict(areaM2=0.0784, secondMomentM4=0.000512, extremeFibreM=0.14, material="larch-weathered", title=T("Two courses of larch 140 x 140 mm, near and far side (EN 338 C24)", "Zwei Lagen Lärche 140 x 140 mm, vordere und hintere Seite (EN 338 C24)")),
    "sleeve": dict(areaM2=0.001717, secondMomentM4=2.53e-06, extremeFibreM=0.05715, material="steel-galvanised-weathered", title=T("Ground sleeve CHS 114.3 x 5.0 (EN 10219-2)", "Bodenhülse Rundrohr 114,3 x 5,0 (EN 10219-2)")),
    "post": dict(areaM2=0.001226, secondMomentM4=1.47e-06, extremeFibreM=0.0508, material="steel-painted", title=T("Post CHS 101.6 x 4.0 (EN 10219-2)", "Pfosten Rundrohr 101,6 x 4,0 (EN 10219-2)")),
    "beam": dict(areaM2=0.001056, secondMomentM4=9.43e-07, extremeFibreM=0.04, material="steel-painted", title=T("Turret beam RHS 80 x 60 x 4 (EN 10219-2)", "Drehträger Rechteckrohr 80 x 60 x 4 (EN 10219-2)")),
    "boom": dict(areaM2=0.000564, secondMomentM4=2.085e-07, extremeFibreM=0.025, material="steel-painted", title=T("Boom SHS 50 x 50 x 3 (EN 10219-2)", "Ausleger Quadratrohr 50 x 50 x 3 (EN 10219-2)")),
    "dipper": dict(areaM2=0.000444, secondMomentM4=1.02e-07, extremeFibreM=0.02, material="steel-painted", title=T("Dipper SHS 40 x 40 x 3 (EN 10219-2)", "Löffelstiel Quadratrohr 40 x 40 x 3 (EN 10219-2)")),
    "bucket": dict(areaM2=0.00108, secondMomentM4=4.0e-07, extremeFibreM=0.045, material="steel-worn", title=T("Scoop shell 3 mm sheet with two side plates", "Schaufelschale 3-mm-Blech mit zwei Seitenblechen")),
    "lever-boom": dict(areaM2=0.000491, secondMomentM4=1.917e-08, extremeFibreM=0.0125, material="steel-painted", title=T("Lever round bar 25 mm", "Hebel Rundstahl 25 mm")),
    "lever-dipper": dict(areaM2=0.000491, secondMomentM4=1.917e-08, extremeFibreM=0.0125, material="steel-painted", title=T("Lever round bar 25 mm", "Hebel Rundstahl 25 mm")),
}
MASS_REF = {"frame": "en-338-2016", "sand": "incropera-2018", "sleeve": "en-10219-2", "post": "en-10219-2", "beam": "en-10219-2", "boom": "en-10219-2", "dipper": "en-10219-2", "bucket": "en-10219-2", "lever-boom": "en-10219-2", "lever-dipper": "en-10219-2"}


def bone_masses():
    """⚖️ Mass and local centre of every bone from the 1:50 parts (section or volume times density)."""
    acc = {}
    for p in all_parts(50):
        m = mass_of(p)
        cx, cy = centre_of(p)
        a = acc.setdefault("beam" if p["bone"] == "lift-eye" else p["bone"], [0.0, 0.0, 0.0])
        a[0] += m
        a[1] += m * cx
        a[2] += m * cy
    out = {}
    for b, (m, sx, sy) in acc.items():
        u, v = local(b, sx / m, sy / m)
        out[b] = (round(m, 3), round(u, 4), round(v, 4))
    return out


MASSES = bone_masses()


def rig():
    """🦴 The rig with masses and sections on the bones."""
    bones = []
    for b in BONES:
        d = {k: b[k] for k in ("id", "parent", "title", "x", "y")}
        if b.get("z"):
            d["z"] = b["z"]
        d.update(angleDeg=round(b["angleDeg"], 6), lengthM=b["lengthM"], joint=b["joint"])
        if b["id"] in MASSES:
            m, u, v = MASSES[b["id"]]
            d["mass"] = {"kg": m, "comX": u, "comY": v, "distribution": "point", "source": "authored-estimate" if b["id"] in ("frame", "sand") else "literature", "reference": MASS_REF.get(b["id"], "en-10219-2")}
        if b["id"] in SECTIONS:
            d["section"] = SECTIONS[b["id"]]
        bones.append(d)
    return {"kind": "mechanical", "root": "ground", "bones": bones}


def anchors():
    """📍 Footings, sockets, the lifting eye and the load points."""
    out = []
    m = FRAME_OUT - TIMBER / 2
    for name, sx, sz, en, de in (("corner-front-left", 1, 1, "front left", "vorn links"), ("corner-front-right", 1, -1, "front right", "vorn rechts"), ("corner-back-left", -1, 1, "back left", "hinten links"), ("corner-back-right", -1, -1, "back right", "hinten rechts")):
        u, v = local("frame", sx * m, 0.0)
        out.append({"id": name, "bone": "frame", "x": u, "y": v, "z": sz * m, "kind": "contact", "title": T(f"Timber corner on the gravel bed, {en}", f"Holzecke auf dem Schotterbett, {de}"), "support": "contact", "normalDeg": 90, "footprintLengthM": TIMBER, "footprintWidthM": TIMBER, "frictionCoefficient": 0.6})
    u, v = local("sand", 0.0, 0.0)
    out.append({"id": "sand-bed", "bone": "sand", "x": u, "y": v, "kind": "contact", "title": T("Sand fill on the subsoil and fleece", "Sandfüllung auf Untergrund und Vlies"), "support": "contact", "normalDeg": 90, "footprintLengthM": 2 * INNER, "footprintWidthM": 2 * INNER, "frictionCoefficient": 0.6})
    out.append({"id": "sleeve-foundation", "bone": "sleeve", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Sleeve cast into the concrete point foundation", "Hülse im Beton-Punktfundament einbetoniert"), "support": "fixed", "footprintLengthM": 0.16, "footprintWidthM": 0.16})
    out.append({"id": "post-socket", "bone": "post", "x": 0.0, "y": 0.0, "kind": "support", "title": T("Post foot clamped in the sleeve", "Pfostenfuß in der Hülse verklemmt"), "support": "fixed", "footprintLengthM": 0.1, "footprintWidthM": 0.1})
    out.append({"id": "lift-eye", "bone": "lift-eye", "x": 0.0, "y": 0.0, "kind": "load", "title": T("Crane hook in the eye nut (lifting point)", "Kranhaken in der Ringmutter (Anschlagpunkt)")})
    u, v = local("beam", XP - 0.27, 0.599)
    out.append({"id": "seat-point", "bone": "beam", "x": u, "y": v, "kind": "load", "title": T("Seat point of the child", "Sitzpunkt des Kindes")})
    dt = world("dipper", DIPPER_L, 0.0)
    u, v = local("bucket", dt[0] - 0.15, dt[1] - 0.165)
    out.append({"id": "bucket-teeth", "bone": "bucket", "x": u, "y": v, "kind": "load", "title": T("Cutting edge of the scoop", "Schneide der Schaufel")})
    return out


REGION_DEF = [
    ("frame-timber", T("Weathered larch edging, sunny faces", "Verwitterte Lärcheneinfassung, besonnte Flächen"), "wall", "larch-weathered", 0.14, "vertical-plate", 0.5, 0.06, "timber", "authored-estimate"),
    ("frame-timber-mossy", T("Larch edging on the shaded side, moss and algae", "Lärcheneinfassung auf der Schattenseite, Moos und Algen"), "wall", "larch-weathered", 0.14, "vertical-plate", 0.4, 0.06, "timber", "authored-estimate"),
    ("seat-boards", T("Silver-grey seat boards, checked and gum-spotted", "Silbergraue Sitzbretter, rissig und mit Kaugummiflecken"), "surface", "larch-weathered", 0.28, "horizontal-plate-up", 0.95, 0.045, "timber", "authored-estimate"),
    ("seat-board-new", T("Replaced front seat board, fresh larch", "Ersetztes vorderes Sitzbrett, frische Lärche"), "surface", "larch-fresh", 0.28, "horizontal-plate-up", 0.95, 0.045, "timber", "authored-estimate"),
    ("sand", T("Play sand surface, sunk and trodden", "Spielsandoberfläche, abgesackt und zertreten"), "surface", "play-sand", 3.72, "horizontal-plate-up", 0.95, 0.12, "masonry", "literature"),
    ("digger-paint", T("Powder-coated steel of the digger, flaking at the welds", "Pulverbeschichteter Stahl des Baggers, an den Schweißnähten abblätternd"), "surface", "steel-painted", 0.08, "cylinder", 0.7, 0.004, "metal", "authored-estimate"),
    ("digger-bucket", T("Scoop sheet worn to bare steel", "Schaufelblech bis auf blanken Stahl abgerieben"), "surface", "steel-worn", 0.18, "vertical-plate", 0.6, 0.003, "metal", "authored-estimate"),
    ("digger-seat", T("HDPE seat, backrest and lever grips", "HDPE-Sitz, Rückenlehne und Hebelgriffe"), "surface", "hdpe-board", 0.3, "horizontal-plate-up", 0.9, 0.019, "polymer", "literature"),
    ("sleeve", T("Galvanised ground sleeve and collar", "Verzinkte Bodenhülse mit Kragen"), "surface", "steel-galvanised-weathered", 0.114, "horizontal-plate-up", 0.9, 0.005, "metal", "authored-estimate"),
    ("toys", T("Toys left behind: upturned bucket and spade", "Liegen gebliebene Spielsachen: umgestülpter Eimer und Schaufel"), "surface", "polypropylene-toy", 0.15, "cylinder", 0.8, 0.002, "polymer", "literature"),
]
MAT = {m["id"]: m for m in MATERIALS}
AREA_OVERRIDE = {"frame-timber": 9.837, "frame-timber-mossy": 3.367, "sand": 29.583, "toys": 0.146}


def region_areas():
    """🧮 Plain area per region from the 1:50 parts (replaced by the measured outer area where known)."""
    acc = {}
    for p in all_parts(50):
        acc[p["region"]] = acc.get(p["region"], 0.0) + surface_of(p)
    return {k: round(AREA_OVERRIDE.get(k, v), 4) for k, v in acc.items()}


def thermal():
    """🌡️ Passive regions with weathered surface data; the sun on dark steel, red HDPE and dry sand is the thermal story."""
    areas = region_areas()
    regions = []
    for rid, title, kind, mat, lc, geom, svf, t, lkind, src in REGION_DEF:
        m = MAT[mat]
        alpha = m["conductivityWmK"] / (m["densityKgM3"] * m["specificHeatJKgK"])
        depth = min(t, math.sqrt(alpha * 3600))
        c = round(m["densityKgM3"] * m["specificHeatJKgK"] * depth)
        regions.append({"id": rid, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": areas[rid], "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf,
                        "layers": [{"id": f"{rid}-layer", "title": T(f"{m['title']['en']}, {t * 1000:.0f} mm", f"{m['title']['de']}, {t * 1000:.0f} mm".replace(".", ",")), "kind": lkind, "material": mat, "thicknessM": t, "resistanceM2KW": round(t / m["conductivityWmK"], 6), "heatCapacityJM2K": c, "source": "literature", "reference": m.get("reference", "iso-10456-2007")}],
                        "heatCapacityJM2K": c, "source": src, "reference": m.get("reference", "iso-10456-2007")})
    return {
        "environment": {"airTempC": 27, "meanRadiantTempC": 32, "relativeHumidity": 0.4, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 750, "diffuseFraction": 0.15, "pressurePa": 101325},
        "passengers": [], "operating": [], "emitters": [], "regions": regions,
        "assumptions": [
            T("Summer afternoon in a municipal playground: 750 W/m2 sun, 27 degC air, 1.5 m/s wind. Every region is passive; the asset has no electrical or fuel input and therefore no rated power (0 W).", "Sommernachmittag auf einem städtischen Spielplatz: 750 W/m2 Sonne, 27 degC Luft, 1,5 m/s Wind. Alle Regionen sind passiv; das Objekt hat keine elektrische oder Brennstoffzufuhr und daher keine Nennleistung (0 W)."),
            T("Weathered surfaces differ from new ones: silver-grey larch absorbs about 0.70 of the sun against about 0.50 for the fresh replacement board, the chalked orange powder coat about 0.65, the scoop worn to bare rusty steel about 0.75 with an emissivity of 0.7 instead of 0.9 (authored estimates after Incropera Table A.12 and ISO 10456 for the bulk values).", "Verwitterte Oberflächen unterscheiden sich von neuen: silbergraue Lärche absorbiert etwa 0,70 der Sonne, das frische Ersatzbrett etwa 0,50, die verkreidete orange Pulverbeschichtung etwa 0,65, die bis auf rostigen Stahl abgeriebene Schaufel etwa 0,75 bei einem Emissionsgrad von 0,7 statt 0,9 (Schätzwerte nach Incropera Tabelle A.12 und ISO 10456 für die Stoffwerte)."),
            T("The red HDPE seat (absorptance 0.70, 19 mm, low conductivity) and the dry sand surface (Incropera: 1515 kg/m3, 0.27 W/mK, 800 J/kgK) are the hot surfaces of a summer afternoon; the shaded mossy side of the edging stays cool and damp.", "Der rote HDPE-Sitz (Absorptionsgrad 0,70, 19 mm, geringe Leitfähigkeit) und die trockene Sandoberfläche (Incropera: 1515 kg/m3, 0,27 W/mK, 800 J/kgK) sind die heißen Flächen eines Sommernachmittags; die beschattete, bemooste Seite der Einfassung bleibt kühl und feucht."),
            T("Heat capacities rho*cp*min(thickness, sqrt(alpha*3600 s)); areas are the outer surfaces of the 1:50 model, including the faces the sand covers.", "Wärmekapazitäten rho*cp*min(Dicke, sqrt(alpha*3600 s)); Flächen sind die Außenflächen des Modells 1:50, einschließlich der vom Sand bedeckten Flächen."),
        ],
    }


def bindings(pose):
    """🌞 Exposure and sun share of every region in a pose."""
    base = {
        "frame-timber": (1.0, 0.6), "frame-timber-mossy": (1.0, 0.15), "seat-boards": (0.9, 0.75), "seat-board-new": (0.9, 0.85),
        "sand": (1.0, 0.85), "digger-paint": (1.0, 0.65), "digger-bucket": (1.0, 0.5), "digger-seat": (1.0, 0.9), "sleeve": (0.1, 0.1), "toys": (1.0, 0.9),
    }
    if pose == "maintenance":
        base.update({"sleeve": (1.0, 0.8), "digger-bucket": (1.0, 0.7), "digger-paint": (1.0, 0.7)})
    out = {}
    for k, (e, sun) in base.items():
        out[k] = {"exposedFraction": e, "sunlitFraction": sun}
    if pose == "in-use":
        out["digger-seat"] = {"exposedFraction": 0.6, "sunlitFraction": 0.35, "contact": {"fraction": 0.4, "tempC": 32}}
        out["digger-bucket"] = {"exposedFraction": 0.8, "sunlitFraction": 0.5, "wettedness": 0.0}
    return {"regions": out, "emitters": {}}


CONTACT_SERVICE = ["corner-front-left", "corner-front-right", "corner-back-left", "corner-back-right", "sand-bed", "sleeve-foundation", "post-socket"]
CONTACT_LIFT = ["corner-front-left", "corner-front-right", "corner-back-left", "corner-back-right", "sand-bed", "sleeve-foundation"]
DIGGER_BONES = ("post", "beam", "boom", "dipper", "bucket", "lever-boom", "lever-dipper")
HOOK = {"id": "crane-hook", "kind": "equipment", "title": T("Crane hook carrying the whole digger in the eye nut", "Kranhaken trägt den ganzen Bagger in der Ringmutter"), "anchor": "lift-eye", "fxN": 0, "fyN": round(sum(MASSES[b][0] for b in DIGGER_BONES) * 9.80665, 1), "source": "authored-estimate", "reference": "din-582"}
FOLDED = {"boom": {"angleDeg": 20}, "dipper": {"angleDeg": -30}, "bucket": {"angleDeg": -40}}
OCCUPANT = {"id": "child-occupant", "kind": "occupant", "title": T("Seated child, 30 kg (EN 1176-1 user up to 8 years)", "Sitzendes Kind, 30 kg (EN 1176-1 Nutzer bis 8 Jahre)"), "anchor": "seat-point", "fxN": 0, "fyN": -294.2, "source": "authored-estimate", "reference": "en-1176-1"}
DIG = {"id": "digging-force", "kind": "live", "title": T("Sand resisting the scoop, 150 N", "Sandwiderstand an der Schaufel, 150 N"), "anchor": "bucket-teeth", "fxN": 150, "fyN": 60, "source": "authored-estimate", "reference": "en-1176-1"}
LIFT_BONES = {"post": {"dy": 0.30}, "boom": {"angleDeg": 20}, "dipper": {"angleDeg": -30}, "bucket": {"angleDeg": -40}}
USE_BONES = {"boom": {"angleDeg": -10}, "dipper": {"angleDeg": 22}, "bucket": {"angleDeg": -35}, "lever-boom": {"angleDeg": 12}, "lever-dipper": {"angleDeg": -14}}


def poses():
    """🧍 In service, lifted out by the depot, and in use by a child."""
    return [
        {"id": "in-service", "title": T("In service", "In Betrieb"), "description": T("The digger stands idle in its sleeve, scoop resting in the pit it dug, the sand 205 mm below the seat boards after years of sinking and carrying out; an upturned toy bucket and a spade lie where the children left them.", "Der Bagger steht unbenutzt in seiner Hülse, die Schaufel ruht in der selbst gegrabenen Grube, der Sand liegt nach Jahren des Absackens und Heraustragens 205 mm unter den Sitzbrettern; ein umgestülpter Spielzeugeimer und eine Schaufel liegen, wo die Kinder sie ließen."),
         "bones": {}, "ground": "auto", "contacts": CONTACT_SERVICE, "thermal": bindings("in-service"), "loads": []},
        {"id": "maintenance", "title": T("Lifted out by the depot", "Vom Bauhof ausgehoben"), "description": T("The depot crew has loosened the clamping bolt, folded the arm and hooked the truck crane into the DIN 582 eye nut: the 30 kg digger hangs 0.30 m above its sleeve, which stays behind flush in the sand for topping up and inspection.", "Der Bauhoftrupp hat die Klemmschraube gelöst, den Arm angelegt und den Ladekran in die Ringmutter DIN 582 eingehängt: der 30 kg schwere Bagger hängt 0,30 m über seiner Hülse, die bündig im Sand zum Auffüllen und Prüfen zurückbleibt."),
         "bones": LIFT_BONES, "ground": "auto", "contacts": CONTACT_LIFT, "thermal": bindings("maintenance"), "loads": [HOOK]},
        {"id": "in-use", "title": T("Child digging", "Kind beim Baggern"), "description": T("A child of about six sits on the seat and pulls both levers: the boom is lowered, the dipper curled in and the scoop drags through the sand with 150 N.", "Ein etwa sechsjähriges Kind sitzt auf dem Sitz und zieht beide Hebel: der Ausleger ist abgesenkt, der Stiel eingezogen und die Schaufel zieht mit 150 N durch den Sand."),
         "bones": USE_BONES, "ground": "auto", "contacts": CONTACT_SERVICE, "thermal": bindings("in-use"), "loads": [OCCUPANT, DIG]},
        {"id": "hook-taut", "key": True, "title": T("Hook taut", "Haken gespannt"), "description": T("Key of the lifting clip: the arm is folded and the crane hook carries the digger while its post still stands in the sleeve.", "Schlüssel des Aushebe-Clips: der Arm ist angelegt und der Kranhaken trägt den Bagger, solange sein Pfosten noch in der Hülse steht."),
         "bones": FOLDED, "ground": "auto", "contacts": CONTACT_SERVICE, "thermal": bindings("in-service"), "loads": [HOOK]},
    ]


def dig_key(t, boom, dipper, bucket, twist=0.0):
    """⛏️ One key of the digging cycle; the levers follow the arm."""
    b = {"boom": {"angleDeg": boom}, "dipper": {"angleDeg": dipper}, "bucket": {"angleDeg": bucket}, "lever-boom": {"angleDeg": round(-0.6 * boom, 3)}, "lever-dipper": {"angleDeg": round(-0.5 * dipper, 3)}}
    if twist:
        b["turret"] = {"twistDeg": twist}
    return {"t": t, "pose": "in-use", "ease": "smooth", "bones": b}


def clips():
    """🎞️ The depot lift and the child's digging cycle."""
    lift = {"id": "maintenance-lift", "title": T("Lifting the digger out", "Bagger ausheben"), "description": T("The crew folds the arm, hooks the crane in, releases the clamp and lifts the digger 0.30 m out of its sleeve within 8 s.", "Der Trupp legt den Arm an, hängt den Kran ein, löst die Klemmung und hebt den Bagger in 8 s 0,30 m aus seiner Hülse."),
            "durationS": 8.0, "fps": 25, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0.0, "pose": "in-service", "ease": "smooth"},
                     {"t": 3.0, "pose": "in-service", "ease": "smooth", "bones": FOLDED},
                     {"t": 3.5, "pose": "hook-taut", "ease": "smooth"},
                     {"t": 4.0, "pose": "hook-taut", "ease": "in"},
                     {"t": 8.0, "pose": "maintenance", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0.0, "toS": 8.0} for a in CONTACT_SERVICE if a != "post-socket"] + [{"anchor": "post-socket", "fromS": 0.0, "toS": 4.0}],
            "phases": [{"id": "fold-arm", "fromS": 0.0, "toS": 3.0, "title": T("Fold the arm", "Arm anlegen")}, {"id": "lift", "fromS": 4.0, "toS": 8.0, "title": T("Lift out of the sleeve", "Aus der Hülse heben")}],
            "events": [{"t": 3.5, "kind": "load-on", "title": T("Crane hook takes the load", "Kranhaken übernimmt die Last"), "target": "crane-hook"}, {"t": 4.0, "kind": "contact-off", "title": T("Post foot leaves the sleeve clamp", "Pfostenfuß verlässt die Klemmung"), "target": "post-socket"}],
            "tracks": []}
    keys = [dig_key(0.0, 8, -12, 20), dig_key(1.2, -12, 18, 15), dig_key(2.2, -10, 26, -45), dig_key(3.0, 16, 6, -45), dig_key(3.9, 16, 6, -45, 70), dig_key(4.6, 16, 6, 35, 70), dig_key(5.3, 10, -6, 10, 15), dig_key(6.0, 8, -12, 20)]
    dig = {"id": "digging", "title": T("Digging and dumping", "Baggern und Auskippen"), "description": T("One 6 s cycle of a child at play: lower the boom, drag the scoop through the pit, curl and lift, swivel the turret 70 degrees, dump the sand on the heap and swivel back.", "Ein 6-s-Zyklus eines spielenden Kindes: Ausleger absenken, Schaufel durch die Grube ziehen, einrollen und anheben, Drehkranz um 70 Grad schwenken, Sand auf den Haufen kippen und zurückschwenken."),
           "durationS": 6.0, "fps": 25, "loop": True, "rootMotion": "none", "keys": keys,
           "contacts": [{"anchor": a, "fromS": 0.0, "toS": 6.0} for a in CONTACT_SERVICE],
           "phases": [{"id": "dig", "fromS": 0.0, "toS": 2.2, "title": T("Dig", "Graben")}, {"id": "lift-swivel", "fromS": 2.2, "toS": 3.9, "title": T("Lift and swivel", "Heben und schwenken")}, {"id": "dump-return", "fromS": 3.9, "toS": 6.0, "title": T("Dump and return", "Auskippen und zurück")}],
           "events": [{"t": 1.2, "kind": "load-on", "title": T("Scoop bites into the sand", "Schaufel greift in den Sand"), "target": "digging-force"}, {"t": 2.2, "kind": "load-off", "title": T("Scoop leaves the sand", "Schaufel verlässt den Sand"), "target": "digging-force"}, {"t": 4.6, "kind": "motion", "title": T("Sand dumped", "Sand ausgekippt")}],
           "tracks": [{"target": "load:digging-force", "ease": "smooth", "keys": [{"t": 0.0, "v": 0.0}, {"t": 1.0, "v": 0.0}, {"t": 1.5, "v": 1.0}, {"t": 2.0, "v": 1.0}, {"t": 2.4, "v": 0.0}, {"t": 6.0, "v": 0.0}]}]}
    return [dig, lift]


def structure():
    """🏗️ Footings, sleeve and the cantilevered digger."""
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": ["corner-front-left", "corner-front-right", "corner-back-left", "corner-back-right", "sand-bed", "sleeve-foundation"],
            "assumptions": [
                T("The edging is two courses of 140 x 140 mm larch (EN 338 C24) bolted at the corners and resting on a compacted gravel bed; its four corners are unilateral contacts. The sand (Incropera: 1515 kg/m3) rests on the subsoil through its own contact; it does not load the edging.", "Die Einfassung besteht aus zwei Lagen Lärche 140 x 140 mm (EN 338 C24), an den Ecken verschraubt, auf einem verdichteten Schotterbett; ihre vier Ecken sind einseitige Kontakte. Der Sand (Incropera: 1515 kg/m3) liegt mit eigenem Kontakt auf dem Untergrund und belastet die Einfassung nicht."),
                T("The digger is a cantilever: the CHS 101.6 x 4.0 post is clamped in a CHS 114.3 x 5.0 sleeve cast into a concrete point foundation whose top lies at lawn level (120 mm below the sunk sand, 250 mm below the original fill: DIN EN 1176-1 asks for foundations to stay covered in loose fill, so topping up the sand is due). Turret, boom, dipper and scoop are pinned; the levers are linked to the arm.", "Der Bagger ist ein Kragarm: der Pfosten Rundrohr 101,6 x 4,0 ist in einer Hülse 114,3 x 5,0 verklemmt, die in einem Beton-Punktfundament mit Oberkante auf Rasenhöhe sitzt (120 mm unter dem abgesackten Sand, 250 mm unter der ursprünglichen Füllung: DIN EN 1176-1 verlangt, dass Fundamente in losem Material bedeckt bleiben, das Auffüllen ist also fällig). Drehkranz, Ausleger, Stiel und Schaufel sind gelenkig; die Hebel sind mit dem Arm gekoppelt."),
                T("In use a 30 kg child sits 0.48 m above the sand and drags the scoop with 150 N (authored estimates for the DIN EN 1176-1 user group); in maintenance the post foot is free and the crane hook in the eye nut is applied as its lifting force, the 0.28 kN weight of the digger, because supports of the archive are springs of equal stiffness that would share the weight of sand and edging with the hook; the eye nut stands 0.12 m behind the centre of mass of the folded digger, so the crew steadies it by hand.", "Im Gebrauch sitzt ein 30 kg schweres Kind 0,48 m über dem Sand und zieht die Schaufel mit 150 N (Schätzwerte für die Nutzergruppe der DIN EN 1176-1); bei der Wartung ist der Pfostenfuß frei und der Kranhaken in der Ringmutter als seine Hubkraft angesetzt, die 0,28 kN Gewicht des Baggers, weil Auflager des Archivs Federn gleicher Steifigkeit sind, die das Gewicht von Sand und Einfassung mit dem Haken teilen würden; die Ringmutter steht 0,12 m hinter dem Schwerpunkt des angelegten Baggers, der Trupp führt ihn daher von Hand."),
            ], "clinical": False}


def structure_list(s, parts):
    """🏗️ Members, joints, supports and masses of scale s."""
    f = FEATURE[s]
    ids = {p["id"] for p in parts}
    out = []
    member_of = {"frame": [i for i in ids if (i.startswith("course") and s == 50) or (i.startswith("edge-") and (s == 200 or i in ("edge-right", "edge-left")))], "sleeve": ["sleeve-tube"], "beam": ["beam", "boom-lug-left", "boom-lug-right", "lever-tube"], "boom": ["boom-arm"], "dipper": ["dipper-arm"],
                 "bucket": ["bucket-shell-0", "bucket-shell-1", "bucket-shell-2", "bucket-shell-3", "bucket-shell-4", "bucket-block", "bucket-lug-left", "bucket-lug-right"], "lever-boom": ["lever-boom-rod"], "lever-dipper": ["lever-dipper-rod"]}
    for bone, lst in member_of.items():
        for i in sorted(lst):
            if i in ids:
                out.append({"id": f"st-{i}", "of": i, "member": bone, "part": "member"})
    own = []
    if s == 100:
        o, i, m = FRAME_OUT, INNER, FRAME_OUT - TIMBER / 2
        for side, box in (("right", (-o, o, -o, -i)), ("left", (-o, o, i, o)), ("front", (i, o, -i, i)), ("back", (-o, -i, -i, i))):
            own.append(("member", "frame", P(f"member-frame-{side}", "frame", "larch-weathered", None, "box", role="detail", tone=0.5, x0=box[0], x1=box[1], y0=0.0, y1=2 * COURSE, z0=box[2], z1=box[3])))
    own.append(P("member-post", "post", "steel-painted", None, "vcyl", role="detail", tone=0.5, cx=XP, cz=0.0, r=0.0508, y0=0.0, y1=0.50))
    jt = max(0.02, f)
    own.append(("joint", "turret", P("joint-turret", "turret", "steel-painted", None, "vcyl", role="detail", tone=0.3, cx=XP, cz=0.0, r=max(0.06, f / 2), y0=0.50 - jt, y1=0.50)))
    own.append(("joint", "post", P("joint-sleeve", "post", "steel-painted", None, "vcyl", role="detail", tone=0.3, cx=XP, cz=0.0, r=0.056, y0=SAND_TOP - max(0.02, f), y1=SAND_TOP)))
    pin_r = max(0.01, f / 2 / math.cos(math.pi / 12) + 0.0005)
    dt = world("dipper", DIPPER_L, 0.0)
    bt = world("boom", BOOM_L, 0.0)
    for name, bone, c, z0, z1, ref in (("joint-boom", "boom", BOOM_PIVOT, -max(0.04, f / 2), max(0.04, f / 2), "boom-pin"), ("joint-dipper", "dipper", bt, -0.03, DIPPER_Z + max(0.025, f / 2), "dipper-pin"), ("joint-bucket", "bucket", dt, DIPPER_Z - max(0.035, f / 2), DIPPER_Z + max(0.035, f / 2), "bucket-pin")):
        if ref in ids:
            out.append({"id": f"st-{ref}", "of": ref, "member": bone, "part": "joint"})
        else:
            own.append(("joint", bone, P(name, bone, "steel-painted", None, "sprism", role="detail", tone=0.3, pts=circle(c[0], c[1], pin_r, 12), z0=z0, z1=z1)))
    for lid, side in (("lever-boom", 1), ("lever-dipper", -1)):
        hz = max(0.015, f / 2)
        own.append(("joint", lid, P(f"joint-{lid}", lid, "steel-painted", None, "sprism", role="detail", tone=0.3, pts=circle(LEVER_PIVOT[0], LEVER_PIVOT[1], max(0.017, f / 2 / math.cos(math.pi / 12) + 0.0005), 12), z0=side * LEVER_Z - hz, z1=side * LEVER_Z + hz)))
    sup = []
    m = FRAME_OUT - TIMBER / 2
    h = max(f, 0.0075)
    for name, sx, sz in (("corner-front-left", 1, 1), ("corner-front-right", 1, -1), ("corner-back-left", -1, 1), ("corner-back-right", -1, -1)):
        sup.append((name, "frame", P(f"support-{name}", "frame", "larch-weathered", None, "box", role="detail", tone=0.1, x0=sx * m - TIMBER / 2, x1=sx * m + TIMBER / 2, y0=0.0, y1=h, z0=sz * m - TIMBER / 2, z1=sz * m + TIMBER / 2)))
    sup.append(("sand-bed", "sand", P("support-sand-bed", "sand", "play-sand", None, "pprism", role="detail", tone=0.1, pts=rectpts(-0.3, -0.3, 0.3, 0.3), y0=0.0, y1=h)))
    sup.append(("sleeve-foundation", "sleeve", P("support-sleeve-foundation", "sleeve", "steel-galvanised-weathered", None, "vcyl", role="detail", tone=0.1, cx=XP, cz=0.0, r=max(0.06, f), y0=0.0, y1=h)))
    for anchor, bone, p in sup:
        d = solid_of(p, s)
        d.update(member=bone, part="support", anchor=anchor)
        out.append(d)
    for item in own:
        if isinstance(item, tuple):
            kind, bone, p = item
            d = solid_of(p, s)
            d.update(member=bone, part=kind)
        else:
            d = solid_of(item, s)
            d.update(member="post", part="member")
        out.append(d)
    for bone, (kg, u, v) in MASSES.items():
        X, Y = world(bone, u, v)
        r = f / 2 + 0.0005
        p = P(f"mass-{bone}", bone, "steel-painted", None, "sphere", role="detail", tone=0.15, c=(X, Y), r=r, z=bz(bone))
        d = solid_of(p, s)
        d.update(member=bone, part="mass")
        out.append(d)
    return out


PENS = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, 500: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}
HATCH = {50: {"id": "sand-stipple", "title": T("Sand", "Sand"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 1.0, "penMm": 0.13}, 100: {"id": "sand-stipple", "title": T("Sand", "Sand"), "pattern": "stipple", "angleDeg": 0, "spacingMm": 1.2, "penMm": 0.13}, 200: {"id": "sand-dots", "title": T("Sand", "Sand"), "pattern": "dots", "angleDeg": 0, "spacingMm": 1.5, "penMm": 0.13}}
RANGE = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
DECISIONS = {
    50: dict(
        silhouette=T("Side elevation 80 x 18 mm: the near edging of two larch courses and its seat board, the sand line 120 mm above the lawn, and above it the digger: post with bearing housing, the turret beam with the red seat and backrest, both levers, the eye nut, the boom on its lugs, the offset dipper and the scoop hanging in its pit. Front: edging edge-on with the fresh front board, the sand, the digger seen from ahead; plan: the 4 x 4 m square of mitred boards, the sand with the pit, the heap and the toys, the digger from above.", "Seitenansicht 80 x 18 mm: die vordere Einfassung aus zwei Lärchenlagen mit Sitzbrett, die Sandlinie 120 mm über dem Rasen und darüber der Bagger: Pfosten mit Lagergehäuse, der Drehträger mit rotem Sitz und Lehne, beide Hebel, die Ringmutter, der Ausleger an seinen Laschen, der versetzte Stiel und die in ihrer Grube hängende Schaufel. Vorderansicht: Einfassung von vorn mit dem frischen Vorderbrett, der Sand, der Bagger von vorn; Draufsicht: das Quadrat 4 x 4 m aus Gehrungsbrettern, der Sand mit Grube, Haufen und Spielsachen, der Bagger von oben."),
        retained=T("Every course timber, the four mitred seat boards with the replaced front board, sand bed and surface layer with the dug pit, the heap, upturned toy bucket and spade (blade and handle), sleeve with collar, post, housing, beam, seat pan, backrest and strap, lever tube, levers with grips, eye nut base and ring, boom lugs and pins, boom, dipper, scoop shell, side plates and lugs.", "Jedes Lagerholz, die vier Gehrungsbretter mit dem ersetzten Vorderbrett, Sandbett und Oberschicht mit der gegrabenen Grube, der Haufen, umgestülpter Spielzeugeimer und Schaufel (Blatt und Stiel), Hülse mit Kragen, Pfosten, Gehäuse, Träger, Sitzschale, Lehne und Lasche, Hebelrohr, Hebel mit Griffen, Ringmuttersockel und Ring, Auslegerlaschen und Bolzen, Ausleger, Stiel, Schaufelschale, Seitenbleche und Laschen."),
        merged=T("Nothing merged; corner bolts and the hydraulic-free lever linkage rods inside the beam are not visible.", "Nichts zusammengefasst; Eckschrauben und die Hebelgestänge im Träger sind nicht sichtbar."),
        omitted=T("The buried third course, the gravel bed, the fleece, the concrete point foundation and the sleeve below lawn level belong to the ground and are not drawn.", "Die eingegrabene dritte Lage, das Schotterbett, das Vlies, das Beton-Punktfundament und die Hülse unter Rasenhöhe gehören zum Boden und werden nicht gezeichnet."),
        thinElements=T("Scoop sheet 3 mm and spade blade 4 mm drawn at true thickness as declared sheets; the checks of the timber, the scoop scratches and footprints are 0.13 mm texture strokes.", "Schaufelblech 3 mm und Spatenblatt 4 mm in wahrer Dicke als erklärte Bleche; Risse im Holz, Kratzer der Schaufel und Fußabdrücke sind 0,13-mm-Texturstriche."),
        hatching=T("Sand stipple at 1.0 mm.", "Sand als Punktierung im Abstand 1,0 mm."),
        styles=T("Weathered larch 0.47 to 0.55, fresh board 0.22, sand 0.2, painted steel 0.55 to 0.66, scoop 0.48, red HDPE 0.3, toys 0.6; graffiti, gum, rust and sticker residue as detail fills.", "Verwitterte Lärche 0,47 bis 0,55, frisches Brett 0,22, Sand 0,2, lackierter Stahl 0,55 bis 0,66, Schaufel 0,48, rotes HDPE 0,3, Spielsachen 0,6; Graffiti, Kaugummi, Rost und Aufkleberreste als Detailflächen."),
        overlays=T("Regions frame-timber, frame-timber-mossy, seat-boards, seat-board-new, sand, digger-paint, digger-bucket, digger-seat, sleeve and toys are bound shape by shape.", "Die Regionen frame-timber, frame-timber-mossy, seat-boards, seat-board-new, sand, digger-paint, digger-bucket, digger-seat, sleeve und toys sind Form für Form gebunden.")),
    100: dict(
        silhouette=T("Side elevation 40 x 9 mm with the same parts as 1:50: courses, boards, sand, the digger with its arm hanging in the pit.", "Seitenansicht 40 x 9 mm mit denselben Teilen wie 1:50: Lagen, Bretter, Sand, der Bagger mit dem in der Grube hängenden Arm."),
        retained=T("Course timbers, mitred boards with the new front board, sand with pit and heap, toy bucket and spade, sleeve, post, housing, beam, seat, backrest, strap, lever tube, levers and grips, eye nut, lugs, pins, boom, dipper, scoop sheets and lugs.", "Lagerhölzer, Gehrungsbretter mit dem neuen Vorderbrett, Sand mit Grube und Haufen, Spielzeugeimer und Schaufel, Hülse, Pfosten, Gehäuse, Träger, Sitz, Lehne, Lasche, Hebelrohr, Hebel und Griffe, Ringmutter, Laschen, Bolzen, Ausleger, Stiel, Schaufelbleche und Laschen."),
        merged=T("Strap, lugs, collar and eye ring are drawn and modelled at the 15 mm feature instead of 8 to 14 mm; the pit is a 16-sided polygon.", "Lasche, Laschenbleche, Kragen und Ösenring sind im Merkmal 15 mm statt 8 bis 14 mm gezeichnet und modelliert; die Grube ist ein 16-Eck."),
        omitted=T("Buried course, gravel bed, foundation and the sleeve below the lawn; fewer checks, gum spots, moss flecks and footprints than at 1:50.", "Eingegrabene Lage, Schotterbett, Fundament und Hülse unter dem Rasen; weniger Risse, Kaugummiflecken, Moosflecken und Fußabdrücke als bei 1:50."),
        thinElements=T("Scoop and spade sheets keep their true thickness as declared sheets; texture strokes 0.13 mm.", "Schaufel- und Spatenbleche behalten als erklärte Bleche ihre wahre Dicke; Texturstriche 0,13 mm."),
        hatching=T("Sand stipple at 1.2 mm.", "Sand als Punktierung im Abstand 1,2 mm."),
        styles=T("Tones as at 1:50; body pen 0.35.", "Tonwerte wie bei 1:50; Körperstift 0,35."),
        overlays=T("All ten regions bound shape by shape.", "Alle zehn Regionen Form für Form gebunden.")),
    200: dict(
        silhouette=T("Side elevation 20 x 4.5 mm: each edging side one 280 mm block under its seat board, the sand slab with the heap, and the digger generalised to post, sleeve, beam, a 30 mm seat and backrest, 30 mm levers, eye nut block, boom, dipper and a solid scoop.", "Seitenansicht 20 x 4,5 mm: jede Einfassungsseite ein 280-mm-Block unter ihrem Sitzbrett, die Sandplatte mit Haufen und der Bagger verallgemeinert zu Pfosten, Hülse, Träger, 30-mm-Sitz und Lehne, 30-mm-Hebeln, Ringmutterblock, Ausleger, Stiel und massiver Schaufel."),
        retained=T("Edging blocks, four boards with the fresh one, sand with pit and heap, toy bucket, the digger parts named above.", "Einfassungsblöcke, vier Bretter mit dem frischen, Sand mit Grube und Haufen, Spielzeugeimer, die genannten Baggerteile."),
        merged=T("Both courses of a side into one block; sleeve and collar into one 160 mm cylinder; seat and backrest at 30 mm with the strap merged into the backrest; the scoop sheets and lugs into one solid wedge; the eye nut into one block; pins left out.", "Beide Lagen einer Seite zu einem Block; Hülse und Kragen zu einem 160-mm-Zylinder; Sitz und Lehne mit 30 mm, die Lasche in die Lehne übernommen; Schaufelbleche und Laschen zu einem massiven Keil; Ringmutter zu einem Block; Bolzen entfallen."),
        omitted=T("Spade (4 mm blade, 22 mm handle below the 30 mm feature), gum, moss, checks and footprints.", "Spaten (4-mm-Blatt, 22-mm-Stiel unter dem Merkmal 30 mm), Kaugummi, Moos, Risse und Fußabdrücke."),
        thinElements=T("None below 30 mm.", "Keine unter 30 mm."),
        hatching=T("Sand dots at 1.5 mm.", "Sand als Punkte im Abstand 1,5 mm."),
        styles=T("Tones as at 1:50; body pen 0.25.", "Tonwerte wie bei 1:50; Körperstift 0,25."),
        overlays=T("All ten regions bound.", "Alle zehn Regionen gebunden.")),
    500: dict(
        silhouette=T("Side elevation 8 x 1.8 mm: edging blocks under 75 mm boards, the sand slab with its heap, toy bucket and the digger as post, beam, seat block, lever bars, boom, dipper and scoop at the 75 mm feature.", "Seitenansicht 8 x 1,8 mm: Einfassungsblöcke unter 75-mm-Brettern, die Sandplatte mit Haufen, Spielzeugeimer und der Bagger als Pfosten, Träger, Sitzblock, Hebelstäbe, Ausleger, Stiel und Schaufel im Merkmal 75 mm."),
        retained=T("Edging, boards with the fresh front board, sand and heap, toy bucket, sleeve, post, housing, beam, seat, backrest, levers, boom, dipper, scoop.", "Einfassung, Bretter mit dem frischen Vorderbrett, Sand und Haufen, Spielzeugeimer, Hülse, Pfosten, Gehäuse, Träger, Sitz, Lehne, Hebel, Ausleger, Stiel, Schaufel."),
        merged=T("Boards thickened to 75 mm over 250 mm edging blocks and laid as butt-jointed rectangles; sand as one 120 mm slab without the pit; beam, boom, dipper and levers at 75 mm; seat pan and backrest as 75 mm blocks.", "Bretter auf 75 mm verdickt über 250-mm-Einfassungsblöcken und stumpf gestoßen; Sand als eine 120-mm-Platte ohne Grube; Träger, Ausleger, Stiel und Hebel mit 75 mm; Sitzschale und Lehne als 75-mm-Blöcke."),
        omitted=T("Pit, spade, grips, lever tube, eye nut, lugs, pins and every trace stroke.", "Grube, Spaten, Griffe, Hebelrohr, Ringmutter, Laschen, Bolzen und alle Gebrauchsspuren."),
        thinElements=T("None.", "Keine."),
        hatching=T("None.", "Keine."),
        styles=T("Tones as at 1:50; body pen 0.18.", "Tonwerte wie bei 1:50; Körperstift 0,18."),
        overlays=T("All ten regions bound.", "Alle zehn Regionen gebunden.")),
}
VOLUME_DECISIONS = {
    50: [T("Body 1:50 (feature 7.5 mm): eight larch course timbers 140 x 140 mm, four mitred 280 x 45 mm seat boards (the front one fresh larch), the sand as a 70 mm bed and a 50 mm surface layer with the 0.5 m pit dug by the scoop, a heap of 0.34 x 0.27 m, the upturned toy bucket and the spade with its 4 mm blade (declared sheet); the hollow sleeve with its collar, the post and bearing housing, turret beam RHS 80 x 60, seat pan and backrest 19 mm HDPE, backrest strap, lever tube, two lever bars with grips, the DIN 582 eye nut, boom lugs and the three 20 mm pins, boom SHS 50, dipper SHS 40 offset 46 mm to the left, the scoop of 3 mm sheet (declared sheets) with its lugs.", "Körper 1:50 (Merkmal 7,5 mm): acht Lärchenlagerhölzer 140 x 140 mm, vier Gehrungs-Sitzbretter 280 x 45 mm (das vordere aus frischer Lärche), der Sand als 70-mm-Bett und 50-mm-Oberschicht mit der 0,5-m-Grube der Schaufel, ein Haufen 0,34 x 0,27 m, der umgestülpte Spielzeugeimer und der Spaten mit 4-mm-Blatt (erklärtes Blech); die hohle Hülse mit Kragen, Pfosten und Lagergehäuse, Drehträger 80 x 60, Sitzschale und Lehne aus 19-mm-HDPE, Lehnenlasche, Hebelrohr, zwei Hebel mit Griffen, die Ringmutter DIN 582, Auslegerlaschen und die drei 20-mm-Bolzen, Ausleger 50 x 50, Stiel 40 x 40 um 46 mm nach links versetzt, die Schaufel aus 3-mm-Blech (erklärte Bleche) mit Laschen."),
         T("Thermal: every body solid is its own thermal surface by reference; structure: the course timbers, sleeve, beam with lugs, boom, dipper, scoop shell with lugs and lever bars are members by reference, the post is its own CHS member from the socket to the turret, pins and the swivel are joints, eight supports sit at their anchors and eleven mass markers at the bone centres.", "Thermik: jeder Körper ist per Verweis seine eigene thermische Fläche; Struktur: Lagerhölzer, Hülse, Träger mit Laschen, Ausleger, Stiel, Schaufelschale mit Laschen und Hebelstäbe sind Glieder per Verweis, der Pfosten ist ein eigenes Rundrohrglied vom Fuß bis zum Drehkranz, Bolzen und Drehlager sind Gelenke, acht Lager sitzen an ihren Ankern und elf Massenmarken in den Knochenschwerpunkten.")],
    100: [T("Body 1:100 (feature 15 mm), authored anew: course timbers and mitred boards kept (all above 15 mm); the backrest strap, boom and scoop lugs, sleeve collar and eye ring generalised to 15 mm; pit as a 16-gon; spade blade a declared 4 mm sheet with a 23 mm handle; scoop sheets kept as declared sheets.", "Körper 1:100 (Merkmal 15 mm), neu erstellt: Lagerhölzer und Gehrungsbretter beibehalten (alle über 15 mm); Lehnenlasche, Ausleger- und Schaufellaschen, Hülsenkragen und Ösenring auf 15 mm verallgemeinert; Grube als 16-Eck; Spatenblatt als erklärtes 4-mm-Blech mit 23-mm-Stiel; Schaufelbleche als erklärte Bleche beibehalten."),
          T("Thermal and structure as at 1:50, the joints and supports sized to the 15 mm feature.", "Thermik und Struktur wie bei 1:50, Gelenke und Lager auf das Merkmal 15 mm bemessen.")],
    200: [T("Body 1:200 (feature 30 mm), authored anew: each edging side one 140 x 280 mm block, boards kept at 45 mm; sleeve and collar one 160 mm cylinder with the post standing on it; seat pan and backrest 30 mm with the strap merged; levers 30 mm bars with 45 mm grips; eye nut one 63 x 65 x 30 mm block; lugs and pins merged into the arms; scoop one solid wedge; spade left out (below the feature).", "Körper 1:200 (Merkmal 30 mm), neu erstellt: jede Einfassungsseite ein Block 140 x 280 mm, Bretter mit 45 mm beibehalten; Hülse und Kragen ein 160-mm-Zylinder, auf dem der Pfosten steht; Sitzschale und Lehne 30 mm mit eingeschlossener Lasche; Hebel 30-mm-Stäbe mit 45-mm-Griffen; Ringmutter ein Block 63 x 65 x 30 mm; Laschen und Bolzen in den Armen aufgegangen; Schaufel ein massiver Keil; Spaten entfällt (unter dem Merkmal)."),
          T("Structure: edging blocks, sleeve, beam, boom, dipper, scoop and levers are members by reference, the post its own member, pins and swivel own 30 mm joint solids.", "Struktur: Einfassungsblöcke, Hülse, Träger, Ausleger, Stiel, Schaufel und Hebel sind Glieder per Verweis, der Pfosten ein eigenes Glied, Bolzen und Drehlager eigene 30-mm-Gelenkkörper.")],
    500: [T("Body 1:500 (feature 75 mm), authored anew: edging blocks 140 x 250 mm under 75 mm butt-jointed boards (front board still the fresh one), sand one 120 mm slab without the pit, heap, toy bucket; sleeve cylinder, post, housing, beam 75 mm wide, seat and backrest 75 mm blocks, levers as 75 mm bars, boom and dipper 75 mm, scoop a solid wedge; grips, lever tube, eye nut, lugs, pins and spade left out.", "Körper 1:500 (Merkmal 75 mm), neu erstellt: Einfassungsblöcke 140 x 250 mm unter stumpf gestoßenen 75-mm-Brettern (das Vorderbrett weiterhin das frische), Sand eine 120-mm-Platte ohne Grube, Haufen, Spielzeugeimer; Hülsenzylinder, Pfosten, Gehäuse, Träger 75 mm breit, Sitz und Lehne als 75-mm-Blöcke, Hebel als 75-mm-Stäbe, Ausleger und Stiel 75 mm, Schaufel ein massiver Keil; Griffe, Hebelrohr, Ringmutter, Laschen, Bolzen und Spaten entfallen."),
          T("Structure: the edging idealised as its two side blocks along the bone (the short front and back blocks only span between the corner footings), the other members by reference to the generalised parts, the post its own member, 75 mm joint, support and mass solids.", "Struktur: die Einfassung als ihre beiden Seitenblöcke entlang des Knochens idealisiert (die kurzen Blöcke vorn und hinten spannen nur zwischen den Eckauflagern), die übrigen Glieder per Verweis auf die verallgemeinerten Teile, der Pfosten ein eigenes Glied, 75-mm-Gelenk-, Lager- und Massenkörper.")],
}


def representation(s, parts):
    """🖨️ The authored drawings of scale s, derived part by part from the same numbers as the solids."""
    geo = {"elevation": [], "front": [], "plan": []}
    for idx, p in enumerate(parts):
        for view, g in shapes_of(p):
            d = {"id": p["id"], "bone": p["bone"], "role": "body", "z": round(depth_key(p, view) + 0.0001 * idx, 5), "filled": True, "tone": p["tone"], "region": p["region"], "material": p["mat"]}
            d.update(g)
            if p["region"] == "sand" and s in HATCH:
                d["hatch"] = HATCH[s]["id"]
            geo[view].append(d)
    for view, g in details(s):
        geo[view].append(g)
    lo, hi = RANGE[s]
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[s], "hatches": [HATCH[s]] if s in HATCH else [], "geometry": geo, "decisions": DECISIONS[s]}


def volume(s, parts):
    """🧊 The authored 3D model of scale s."""
    body = [solid_of(p, s) for p in parts]
    therm = [{"id": f"th-{p['id']}", "of": p["id"], "region": p["region"]} for p in parts]
    return {"status": "authored", "denominator": s, "body": body, "thermal": therm, "structure": structure_list(s, parts), "decisions": VOLUME_DECISIONS[s]}


REFERENCES = [
    {"id": "din-en-1176-1", "citation": "DIN EN 1176-1:2017-12 Playground equipment and surfacing - Part 1: General safety requirements and test methods (user groups, foundations in loose fill, entrapment, free space)."},
    {"id": "en-1176-1", "citation": "EN 1176-1:2017 Playground equipment and surfacing - Part 1: General safety requirements and test methods (loads of users; 30 kg child of the 3 to 8 year group taken as authored estimate)."},
    {"id": "din-en-1176-7", "citation": "DIN EN 1176-7:2020-07 Playground equipment and surfacing - Part 7: Guidance on installation, inspection, maintenance and operation (routine visual, operational and annual main inspection; topping up loose fill)."},
    {"id": "din-en-1177", "citation": "DIN EN 1177:2018-03 Impact attenuating playground surfacing - Methods of test for determination of impact attenuation (sand 0.2 to 2 mm as loose surfacing)."},
    {"id": "din-18034", "citation": "DIN 18034:2020-10 Spielplätze und Freiräume zum Spielen - Anforderungen für Planung, Bau und Betrieb (sand play areas in municipal playgrounds, edgings as seats)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24: mean density 420 kg/m3, characteristic 350 kg/m3, E0,mean 11 GPa, fm,k 24 MPa; larch air-dry about 590 kg/m3)."},
    {"id": "en-350-2016", "citation": "EN 350:2016 Durability of wood and wood-based products - Testing and classification of the durability to biological agents (European larch heartwood durability class 3 to 4: greying, checking and moss on shaded faces in ground-near use)."},
    {"id": "en-10219-2", "citation": "EN 10219-2:2019 Cold formed welded steel structural hollow sections - Part 2: Tolerances, dimensions and sectional properties (CHS 114.3 x 5.0, 101.6 x 4.0, 139.7 x 8.0, 33.7 x 4.0; RHS 80 x 60 x 4; SHS 50 x 50 x 3, 40 x 40 x 3)."},
    {"id": "din-582", "citation": "DIN 582:2018-04 Lifting eye nuts (M16: ring outer diameter about 63 mm, working load 0.7 t in axial pull)."},
    {"id": "en-1993-1-1", "citation": "EN 1993-1-1:2005 Eurocode 3: Design of steel structures - Part 1-1: General rules (E = 210 GPa, density 7850 kg/m3, S235)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: Densities, self-weight, imposed loads for buildings (Annex A: sand 14 to 19 kN/m3, timber)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (softwood 500 kg/m3: 0.13 W/mK, 1600 J/kgK; HD polyethylene 980 kg/m3: 0.50 W/mK, 1800 J/kgK; polypropylene 910 kg/m3: 0.22 W/mK, 1800 J/kgK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (Table A.3 sand 1515 kg/m3, 0.27 W/mK, 800 J/kgK; Tables A.11 and A.12 emissivities and solar absorptivities of paints, weathered and oxidised metals, wood and soil)."},
]


def asset():
    """🏺 The whole asset."""
    reps, vols = {}, {}
    for s in SCALES:
        parts = all_parts(s)
        reps[str(s)] = representation(s, parts)
        vols[str(s)] = volume(s, parts)
    total = round(sum(m for m, _, _ in MASSES.values()), 1)
    top = max(bbox_world(p)[3] for p in all_parts(50))
    return {
        "id": "used-sandpit-digger", "category": "objects",
        "title": T("Used Sandpit with Play Digger", "Gebrauchter Sandkasten mit Sandbagger"),
        "summary": T("A 4 x 4 m municipal sandpit edged with two courses of silver-grey larch and mitred seat boards, one of them replaced in fresh wood; the sand has sunk to 120 mm above the lawn, a pit lies under the scoop of the seated play digger and a toy bucket and spade are left behind. The digger swivels on a removable post in a ground sleeve; its powder coat flakes at the welds and its scoop is worn to bare steel.", "Ein städtischer Sandkasten von 4 x 4 m, eingefasst mit zwei Lagen silbergrauer Lärche und Gehrungs-Sitzbrettern, von denen eines in frischem Holz ersetzt ist; der Sand ist bis 120 mm über dem Rasen abgesackt, unter der Schaufel des Sitzbaggers liegt eine Grube, ein Spielzeugeimer und ein Spaten sind liegen geblieben. Der Bagger dreht auf einem herausnehmbaren Pfosten in einer Bodenhülse; seine Pulverbeschichtung blättert an den Schweißnähten, die Schaufel ist bis auf blanken Stahl abgerieben."),
        "tags": ["used", "park", "public-space", "play", "sandpit", "digger", "larch", "timber", "steel", "sand", "hdpe", "playground"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T("Dimensions after municipal street-furniture and playground catalogue types (no manufacturer); every parameter is literature or an authored estimate, nothing is measured.", "Maße nach Katalogtypen städtischer Ausstattung und Spielgeräte (kein Hersteller); jeder Parameter ist Literatur oder Schätzung, nichts ist gemessen.")},
        "dimensions": {"heightM": round(top, 4), "widthM": 2 * FRAME_OUT, "depthM": 2 * FRAME_OUT, "massKg": total},
        "materials": MATERIALS, "rig": rig(), "volumes": vols, "anchors": anchors(), "representations": reps, "symbols": [],
        "poses": poses(), "clips": clips(), "thermal": thermal(), "structure": structure(),
        "presentation": {"defaultPose": "in-service", "defaultClip": "digging", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }


def main():
    """🚀 Write the document and print the measurements against the reference table."""
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset()}
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
    a = doc["asset"]
    print("[DEBUG] written", OUT)
    print("[DEBUG] dimensions", a["dimensions"])
    for s in SCALES:
        ps = all_parts(s)
        bb = [bbox_world(p) for p in ps]
        print(f"[DEBUG] 1:{s} parts {len(ps)} structure {len(a['volumes'][str(s)]['structure'])} bbox x {min(b[0] for b in bb):.4f}..{max(b[1] for b in bb):.4f} y {min(b[2] for b in bb):.4f}..{max(b[3] for b in bb):.4f} z {min(b[4] for b in bb):.4f}..{max(b[5] for b in bb):.4f}")


if __name__ == "__main__":
    main()
