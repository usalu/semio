"""🚰️ Geometry chisel of the asset `used-washbasin-vanity`: the hand-chosen numbers of the 60 cm vitreous-china washbasin, its wall-hung vanity unit with one drawer, the mirror cabinet with its light bar and the EUR pallet they stand on, and the part lists of 1:50, 1:100 and 1:200.

World frame of the staffage (x ahead = the fronts, y up, z left), metres; the rest pose is the stored pose on the pallet. Every bone rests at angle 0, so a bone-local point is the world point minus the bone origin (z unchanged).

@see ./author-used-washbasin-vanity.py — the writer of the document
@see ../../brief.md — the brief of the wave
"""
import math

PT = 0.144

ORIGIN = {
    "ground": (0.0, 0.0), "pallet": (-0.6, 0.0),
    "mirror-travel": (0.0, 0.0), "mirror-lift": (0.0, 0.0), "mirror": (-0.40, PT),
    "vanity-travel": (0.0, 0.0), "vanity-lift": (0.0, 0.0), "vanity": (-0.16, PT),
    "drawer": (-0.12, PT + 0.03), "basin": (-0.16, PT + 0.50), "lever": (-0.105, PT + 0.66),
}

XM0, XM1, XD1 = -0.40, -0.252, -0.244
ZM, ZMI = 0.30, 0.284
YM1, YL1 = PT + 0.70, PT + 0.76
XH1, XL1 = -0.29, -0.23
XV0, XV1, XF1 = -0.16, 0.28, 0.299
ZV, ZVI = 0.29, 0.274
YV1 = PT + 0.50
ZF = 0.284
YF0, YF1 = PT + 0.003, PT + 0.4985
XB0, XB1 = -0.12, 0.28
YB0, YB1 = PT + 0.03, PT + 0.22
ZBO, ZBI = 0.255, 0.243
XS0, XS1 = -0.16, 0.32
ZS = 0.30
YD0, YD1 = PT + 0.558, PT + 0.57
BOWL_X, BOWL_Y, BOWL_LAT = 0.11, PT + 0.385, 1.4
BOWL_RX = 0.15
TAP_X, TAP_Y = -0.09, PT + 0.57
HANDLE_Y = PT + 0.44
LEVER_LEN = 0.105

DENS = {"pallet-spruce": 470, "melamine-chipboard": 650, "hardboard": 950, "mdf-gloss": 750, "aluminium-anodised": 2700, "mirror-glass": 2500, "float-glass": 2500, "opal-acrylic": 1190, "abs-endcap": 1050, "vitreous-china": 2350, "chrome-brass": 8500, "zinc-steel": 7850, "water": 993}


def r6(v):
    """🎯️ Rounds a coordinate to the micrometre."""
    return round(v + 0.0, 6)


def arc(cx, cy, r, a0, a1, n):
    """⭕️ Points of a circular arc from angle a0 to a1 (degrees), both ends included."""
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * k / n)), cy + r * math.sin(math.radians(a0 + (a1 - a0) * k / n))) for k in range(n + 1)]


def ellipse_pts(cx, cy, rx, ry, n):
    """🥚️ A closed ellipse ring of n points, counter-clockwise."""
    return [(cx + rx * math.cos(2 * math.pi * k / n), cy + ry * math.sin(2 * math.pi * k / n)) for k in range(n)]


def basin_outline_zx(inset, rf, n, front=None):
    """🛁️ The plan outline of the basin as (z, x) points, counter-clockwise: square back corners at the wall, rounded front corners of radius rf, shrunk by `inset` (by `front` at the front edge when given)."""
    z, x0, x1 = ZS - inset, XS0 + inset, XS1 - (inset if front is None else front)
    r = rf - inset
    pts = [(-z, x0), (z, x0)]
    pts += [(z - r + r * math.cos(math.radians(a)), x1 - r + r * math.sin(math.radians(a))) for a in [90 * k / n for k in range(n + 1)]]
    pts += [(-z + r + r * math.cos(math.radians(a)), x1 - r + r * math.sin(math.radians(a))) for a in [90 + 90 * k / n for k in range(n + 1)]]
    return ccw(pts)


def ccw(pts):
    """🔄️ The ring in counter-clockwise order."""
    a = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1]))
    return pts if a > 0 else list(reversed(pts))


def bowl_hole_zx(n, grow=0.0):
    """🥣️ The bowl opening in the deck as (z, x) points: the inner rim of the bowl."""
    return ccw([(z, x) for x, z in ellipse_pts(BOWL_X, 0.0, BOWL_RX + grow, (BOWL_RX + grow) * BOWL_LAT, n)])


def part(id, bone, material, region, tone, geom, rep=None, hatch=None, views=("elevation", "front", "plan"), role="body", **flags):
    """🧩️ One physical part of a scale: its world geometry, region, tone, repeat and drawing hints."""
    return {"id": id, "bone": bone, "material": material, "region": region, "tone": tone, "geom": geom, "rep": rep, "hatch": hatch, "views": views, "role": role, "flags": flags}


def box(x0, x1, y0, y1, z0, z1):
    """📦️ World box bounds."""
    return ("box", x0, x1, y0, y1, z0, z1)


def exy(pts, z0, z1, holes=()):
    """📐️ A side profile (x, y) swept across z0..z1."""
    return ("exy", ccw(pts), z0, z1, [ccw(h) for h in holes])


def eyz(pts, x0, x1, holes=()):
    """🪟️ A front profile (y, z) swept along x0..x1."""
    return ("eyz", ccw(pts), x0, x1, [ccw(h) for h in holes])


def ezx(pts, y0, y1, holes=()):
    """🗺️ A plan profile (z, x) swept up y0..y1."""
    return ("ezx", ccw(pts), y0, y1, [ccw(h) for h in holes])


def rev(xc, y0, zc, profile, closed, lat, outline):
    """🏺️ A turned part about the vertical axis through (xc, y0, zc): profile (height, radius along z), closed loop or open through the axis, `lat` the kernel's `lateralScale` (for a vertical axis it scales the forward x extent), and the outer outline drawn in the views."""
    return ("rev", xc, y0, zc, profile, closed, lat, outline)


def cap(p0, p1, r):
    """💊️ A capsule between two world points."""
    return ("cap", p0, p1, r)


def pallet_parts(scale):
    """🪵️ The EUR pallet (EN 13698-1, 1200 x 800 x 144 mm): every board and block at 1:50 and 1:100, deck plate and nine feet at 1:200."""
    rows = {"rear": (-0.6, -0.455), "middle": (-0.0725, 0.0725), "front": (0.455, 0.6)}
    sides = {"right": (-0.4, -0.3), "centre": (-0.0725, 0.0725), "left": (0.3, 0.4)}
    deck = {"right": (-0.4, -0.255), "inner-right": (-0.21375, -0.11375), "centre": (-0.0725, 0.0725), "inner-left": (0.11375, 0.21375), "left": (0.255, 0.4)}
    out = []
    if scale in (50, 100):
        sfx = "" if scale == 50 else "-100"
        for k, (z0, z1) in sides.items():
            out.append(part(f"pallet-bottom-{k}{sfx}", "pallet", "pallet-spruce", "pallet", 0.5, box(-0.6, 0.6, 0.0, 0.022, z0, z1), hatch="grain" if scale == 50 else None))
        for i, (x0, x1) in rows.items():
            for k, (z0, z1) in sides.items():
                out.append(part(f"pallet-block-{i}-{k}{sfx}", "pallet", "pallet-spruce", "pallet", 0.4, box(x0, x1, 0.022, 0.1, z0, z1)))
        for i, (x0, x1) in rows.items():
            out.append(part(f"pallet-cross-{i}{sfx}", "pallet", "pallet-spruce", "pallet", 0.46, box(x0, x1, 0.1, 0.122, -0.4, 0.4)))
        for k, (z0, z1) in deck.items():
            out.append(part(f"pallet-deck-{k}{sfx}", "pallet", "pallet-spruce", "pallet", 0.55, box(-0.6, 0.6, 0.122, PT, z0, z1), hatch="grain" if scale == 50 else None))
    else:
        out.append(part("pallet-deck-200", "pallet", "pallet-spruce", "pallet", 0.52, box(-0.6, 0.6, 0.1, PT, -0.4, 0.4)))
        for i, (x0, x1) in rows.items():
            for k, (z0, z1) in sides.items():
                out.append(part(f"pallet-foot-{i}-{k}-200", "pallet", "pallet-spruce", "pallet", 0.4, box(x0, x1, 0.0, 0.1, z0, z1)))
    return out


def diffuser_profile(n):
    """💡️ Side profile of the opal diffuser: a quarter round of 60 mm in front of the housing."""
    return [(XH1, YM1)] + arc(XH1, YM1, XL1 - XH1, 0, 90, n)[:-1] + [(XH1, YL1)]


def light_outline(n):
    """💡️ Side outline of the light bar: the aluminium housing with the quarter-round opal diffuser clipped to its front."""
    return [(XM0, YM1)] + diffuser_profile(n)[1:] + [(XM0, YL1)]


def light_chamber(n, t):
    """🕳️ The one lamp chamber inside housing and diffuser, behind a shell of wall `t`."""
    r = XL1 - XH1 - t
    return [(XM0 + t, YM1 + t)] + arc(XH1, YM1, r, math.degrees(math.asin(t / r)), 90, n) + [(XM0 + t, YL1 - t)]


def mirror_parts(scale):
    """🪞️ The mirror cabinet: 16 mm melamine carcass, 3 mm back, glass shelves, two 8 mm mirror doors and the light bar on top."""
    m = "mirror"
    if scale == 50:
        return [
            part("mc-back", m, "hardboard", "mirror-carcass", 0.35, box(XM0, XM0 + 0.003, PT, YM1, -ZMI, ZMI), sheet=True),
            part("mc-sides", m, "melamine-chipboard", "mirror-carcass", 0.82, box(XM0, XM1, PT, YM1, ZMI, ZM), rep=(2, (0, 0, -(ZM + ZMI)))),
            part("mc-bottom", m, "melamine-chipboard", "mirror-carcass", 0.78, box(XM0 + 0.003, XM1, PT, PT + 0.016, -ZMI, ZMI)),
            part("mc-top", m, "melamine-chipboard", "mirror-carcass", 0.78, box(XM0 + 0.003, XM1, YM1 - 0.016, YM1, -ZMI, ZMI)),
            part("mc-shelf-low", m, "float-glass", "mirror-carcass", 0.6, box(XM0 + 0.003, XM1 - 0.012, PT + 0.24, PT + 0.246, -ZMI, ZMI), sheet=True),
            part("mc-shelf-high", m, "float-glass", "mirror-carcass", 0.6, box(XM0 + 0.003, XM1 - 0.012, PT + 0.46, PT + 0.466, -ZMI, ZMI), sheet=True),
            part("mc-door-left", m, "mirror-glass", "mirror-doors", 0.68, box(XM1, XD1, PT, YM1, 0.00075, ZM), hatch="mirror"),
            part("mc-door-right", m, "mirror-glass", "mirror-doors", 0.68, box(XM1, XD1, PT, YM1, -ZM, -0.00075), hatch="mirror"),
            part("mc-light-bar", m, "opal-acrylic", "light-bar", 0.85, exy(light_outline(8), -ZM + 0.008, ZM - 0.008, [light_chamber(8, 0.0025)])),
            part("mc-light-caps", m, "abs-endcap", "light-bar", 0.5, exy(light_outline(8), ZM - 0.008, ZM), rep=(2, (0, 0, -(2 * ZM - 0.008)))),
        ]
    if scale == 100:
        frame = [(PT, -ZM), (PT, ZM), (YM1, ZM), (YM1, -ZM)]
        inner = [(PT + 0.016, -ZMI), (PT + 0.016, ZMI), (YM1 - 0.016, ZMI), (YM1 - 0.016, -ZMI)]
        return [
            part("mc-back-100", m, "hardboard", "mirror-carcass", 0.35, box(XM0, XM0 + 0.003, PT, YM1, -ZMI, ZMI), sheet=True),
            part("mc-carcass-100", m, "melamine-chipboard", "mirror-carcass", 0.82, eyz(frame, XM0 + 0.003, XM1, [inner])),
            part("mc-doors-100", m, "mirror-glass", "mirror-doors", 0.68, box(XM1, XD1, PT, YM1, -ZM, ZM), sheet=True),
            part("mc-light-100", m, "opal-acrylic", "light-bar", 0.85, exy(light_outline(4), -ZM, ZM)),
        ]
    frame = [(PT, -ZM), (PT, ZM), (YM1, ZM), (YM1, -ZM)]
    inner = [(PT + 0.03, -ZM + 0.03), (PT + 0.03, ZM - 0.03), (YM1 - 0.03, ZM - 0.03), (YM1 - 0.03, -ZM + 0.03)]
    return [
        part("mc-carcass-200", m, "melamine-chipboard", "mirror-carcass", 0.82, eyz(frame, XM0, XM1, [inner])),
        part("mc-doors-200", m, "mirror-glass", "mirror-doors", 0.68, box(XM1, XD1, PT, YM1, -ZM, ZM), sheet=True),
        part("mc-light-200", m, "opal-acrylic", "light-bar", 0.85, box(XM0, XL1, YM1, YL1, -ZM, ZM)),
    ]


def vanity_parts(scale):
    """🗄️ The wall-hung vanity unit: 16 mm melamine sides and bottom, 3 mm back with the pipe cut-out, two top rails."""
    v = "vanity"
    if scale == 50:
        return [
            part("v-sides", v, "melamine-chipboard", "vanity-carcass", 0.8, box(XV0, XV1, PT, YV1, ZVI, ZV), rep=(2, (0, 0, -(ZV + ZVI))), hatch=None),
            part("v-bottom", v, "melamine-chipboard", "vanity-carcass", 0.74, box(XV0 + 0.003, XV1, PT, PT + 0.016, -ZVI, ZVI)),
            part("v-back", v, "hardboard", "vanity-carcass", 0.35, box(XV0, XV0 + 0.003, PT + 0.016, YV1, -ZVI, ZVI), sheet=True),
            part("v-rail-front", v, "melamine-chipboard", "vanity-carcass", 0.74, box(XV1 - 0.016, XV1, YV1 - 0.08, YV1, -ZVI, ZVI)),
            part("v-rail-back", v, "melamine-chipboard", "vanity-carcass", 0.74, box(XV0 + 0.003, XV0 + 0.103, YV1 - 0.016, YV1, -ZVI, ZVI)),
        ]
    w = 0.016 if scale == 100 else 0.03
    u = [(PT, -ZV), (PT, ZV), (YV1, ZV), (YV1, ZV - w), (PT + w, ZV - w), (PT + w, -ZV + w), (YV1, -ZV + w), (YV1, -ZV)]
    sfx = f"-{scale}"
    out = [part("v-carcass" + sfx, v, "melamine-chipboard", "vanity-carcass", 0.8, eyz(u, XV0 + 0.003, XV1))]
    out.append(part("v-back" + sfx, v, "hardboard", "vanity-carcass", 0.35, box(XV0, XV0 + 0.003, PT, YV1, -ZV, ZV), sheet=True))
    if scale == 100:
        out.append(part("v-rail-front-100", v, "melamine-chipboard", "vanity-carcass", 0.74, box(XV1 - 0.016, XV1, YV1 - 0.08, YV1, -ZVI, ZVI)))
        out.append(part("v-rail-back-100", v, "melamine-chipboard", "vanity-carcass", 0.74, box(XV0 + 0.003, XV0 + 0.103, YV1 - 0.016, YV1, -ZVI, ZVI)))
    return out


def drawer_parts(scale):
    """🗃️ The drawer: 19 mm high-gloss MDF front, chipboard box on ball-bearing runners, the replaced aluminium bar handle."""
    d = "drawer"
    if scale == 50:
        return [
            part("d-front", d, "mdf-gloss", "drawer", 0.9, box(XV1, XF1, YF0, YF1, -ZF, ZF), hatch=None),
            part("d-sides", d, "melamine-chipboard", "drawer", 0.75, box(XB0, XB1, YB0, YB1, ZBI, ZBO), rep=(2, (0, 0, -(ZBO + ZBI)))),
            part("d-back", d, "melamine-chipboard", "drawer", 0.72, box(XB0, XB0 + 0.012, YB0 + 0.008, YB1 - 0.04, -ZBI, ZBI)),
            part("d-bottom", d, "hardboard", "drawer", 0.4, box(XB0 + 0.012, XB1, YB0, YB0 + 0.008, -ZBI, ZBI)),
            part("d-handle-posts", d, "aluminium-anodised", "drawer", 0.45, box(XF1, XF1 + 0.018, HANDLE_Y - 0.005, HANDLE_Y + 0.005, 0.075, 0.085), rep=(2, (0, 0, -0.16))),
            part("d-handle-bar", d, "aluminium-anodised", "drawer", 0.45, box(XF1 + 0.018, XF1 + 0.03, HANDLE_Y - 0.006, HANDLE_Y + 0.006, -0.09, 0.09)),
        ]
    if scale == 100:
        return [
            part("d-front-100", d, "mdf-gloss", "drawer", 0.9, box(XV1, XF1, YF0, YF1, -ZF, ZF)),
            part("d-sides-100", d, "melamine-chipboard", "drawer", 0.75, box(XB0, XB1, YB0, YB1, ZBO - 0.015, ZBO), rep=(2, (0, 0, -(2 * ZBO - 0.015)))),
            part("d-back-100", d, "melamine-chipboard", "drawer", 0.72, box(XB0, XB0 + 0.015, YB0 + 0.008, YB1 - 0.04, -ZBO + 0.015, ZBO - 0.015)),
            part("d-bottom-100", d, "hardboard", "drawer", 0.4, box(XB0 + 0.015, XB1, YB0, YB0 + 0.008, -ZBO + 0.015, ZBO - 0.015), sheet=True),
            part("d-handle-100", d, "aluminium-anodised", "drawer", 0.45, box(XF1, XF1 + 0.03, HANDLE_Y - 0.0075, HANDLE_Y + 0.0075, -0.09, 0.09)),
        ]
    u = [(YB0, -ZBO), (YB0, ZBO), (YB1, ZBO), (YB1, ZBO - 0.03), (YB0 + 0.03, ZBO - 0.03), (YB0 + 0.03, -ZBO + 0.03), (YB1, -ZBO + 0.03), (YB1, -ZBO)]
    return [part("d-front-200", d, "mdf-gloss", "drawer", 0.9, box(XV1 - 0.011, XF1, YF0, YF1, -ZF, ZF)), part("d-box-200", d, "melamine-chipboard", "drawer", 0.75, eyz(u, XB0, XV1 - 0.011))]


BOWL_OUTER = [(0.0, 0.055), (0.03, 0.095), (0.07, 0.128), (0.12, 0.150), (0.173, 0.162), (0.185, 0.162)]


def offset_chain(chain, d):
    """↘️ The polyline `chain` (height, radius) offset by `d` to its inner side (higher, smaller radius) with mitred corners."""
    lines = []
    for (a0, r0), (a1, r1) in zip(chain, chain[1:]):
        L = math.hypot(a1 - a0, r1 - r0)
        na, nr = (r1 - r0) / L, -(a1 - a0) / L
        lines.append(((a0 + d * na, r0 + d * nr), (a1 - a0, r1 - r0)))
    pts = [lines[0][0]]
    for (p, u), (q, w) in zip(lines, lines[1:]):
        det = u[0] * -w[1] - u[1] * -w[0]
        t = ((q[0] - p[0]) * -w[1] - (q[1] - p[1]) * -w[0]) / det
        pts.append((p[0] + t * u[0], p[1] + t * u[1]))
    p, u = lines[-1]
    pts.append((p[0] + u[0], p[1] + u[1]))
    return pts


def bowl_profile(wall):
    """🥣️ The open section of the bowl wall of thickness `wall` (height above the outer bottom, radius in x), closed through the axis: the bottom is solid ceramic under the pop-up waste that stays in the basin."""
    chain = [(0.0, 0.0225)] + BOWL_OUTER
    inner = offset_chain(chain, wall)
    inner[0] = (wall, 0.0225)
    top = inner[-1]
    inner[-1] = (0.185, top[1])
    prof = BOWL_OUTER + list(reversed(inner))
    return [(r6(a), r6(r)) for a, r in prof]


def bowl_rim(wall):
    """⭕️ Inner radius of the bowl rim in x for a wall of thickness `wall`."""
    return offset_chain([(0.0, 0.0225)] + BOWL_OUTER, wall)[-1][1]


TAP_PROFILE = [(0.0, 0.028), (0.006, 0.028), (0.008, 0.0225), (0.085, 0.0225), (0.09, 0.02)]


def basin_parts(scale):
    """🚰️ The vitreous-china basin: deck with the bowl opening, apron ring, bowl with its waste hole, and the single-lever mixer tap."""
    b = "basin"
    if scale == 50:
        n, t, wall = 6, 0.012, 0.012
    elif scale == 100:
        n, t, wall = 4, 0.015, 0.015
    else:
        n, t, wall = 3, 0.03, 0.03
    sfx = "" if scale == 50 else f"-{scale}"
    hole_n = {50: 40, 100: 28, 200: 20}[scale]
    out = [
        part("b-bowl" + sfx, b, "vitreous-china", "basin", 0.86, rev(BOWL_X, BOWL_Y, 0.0, [(a, r6(r * BOWL_LAT)) for a, r in bowl_profile(wall)], False, r6(1 / BOWL_LAT), [(a, r * BOWL_LAT) for a, r in BOWL_OUTER])),
        part("b-deck" + sfx, b, "vitreous-china", "basin", 0.92, ezx(basin_outline_zx(0.0, 0.06, n), YD1 - t, YD1, [bowl_hole_zx(hole_n, bowl_rim(wall) - BOWL_RX)])),
        part("b-apron" + sfx, b, "vitreous-china", "basin", 0.88, ezx(basin_outline_zx(0.0, 0.06, n), PT + 0.50, YD1 - t, [basin_outline_zx(0.015 if scale < 200 else 0.03, 0.06, n, 0.025 if scale < 200 else 0.03)])),
    ]
    tap = TAP_PROFILE if scale < 200 else [(0.0, 0.028), (0.09, 0.0225)]
    out.append(part("b-tap" + sfx, b, "chrome-brass", "tap", 0.55, rev(TAP_X, TAP_Y, 0.0, tap, False, 1.0, tap)))
    sr = 0.011 if scale < 200 else 0.015
    out.append(part("b-spout" + sfx, b, "chrome-brass", "tap", 0.55, cap((-0.075, PT + 0.63, 0.0), (0.025, PT + 0.622, 0.0), sr)))
    lt = {50: 0.012, 100: 0.015, 200: 0.03}[scale]
    lw = 0.02 if scale < 200 else 0.03
    out.append(part("lever" + sfx, "lever", "chrome-brass", "tap", 0.6, box(ORIGIN["lever"][0], ORIGIN["lever"][0] + LEVER_LEN, ORIGIN["lever"][1], ORIGIN["lever"][1] + lt, -lw / 2, lw / 2)))
    return out


def parts_of(scale):
    """🧱️ Every physical part of a scale."""
    return pallet_parts(scale) + mirror_parts(scale) + vanity_parts(scale) + drawer_parts(scale) + basin_parts(scale)


def loc(bone, x, y):
    """📍️ World rest point to the bone-local (u, v)."""
    ox, oy = ORIGIN[bone]
    return (r6(x - ox), r6(y - oy))


def solid_of(p):
    """🧊️ The volume solid of a part, in the rest frame of its bone."""
    g = p["geom"]
    bone = p["bone"]
    ox, oy = ORIGIN[bone]
    s = {"id": p["id"], "bone": bone, "material": p["material"], "role": "body", "tone": p["tone"]}
    if g[0] == "box":
        _, x0, x1, y0, y1, z0, z1 = g
        s.update({"kind": "box", "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]}})
    elif g[0] == "exy":
        _, pts, z0, z1, holes = g
        s.update({"kind": "extrude", "plane": "xy", "profile": [[r6(x - ox), r6(y - oy)] for x, y in pts], "from": r6(z0), "to": r6(z1)})
        if holes:
            s["holes"] = [[[r6(x - ox), r6(y - oy)] for x, y in h] for h in holes]
    elif g[0] == "eyz":
        _, pts, x0, x1, holes = g
        s.update({"kind": "extrude", "plane": "yz", "profile": [[r6(y - oy), r6(z)] for y, z in pts], "from": r6(x0 - ox), "to": r6(x1 - ox)})
        if holes:
            s["holes"] = [[[r6(y - oy), r6(z)] for y, z in h] for h in holes]
    elif g[0] == "ezx":
        _, pts, y0, y1, holes = g
        s.update({"kind": "extrude", "plane": "zx", "profile": [[r6(z), r6(x - ox)] for z, x in pts], "from": r6(y0 - oy), "to": r6(y1 - oy)})
        if holes:
            s["holes"] = [[[r6(z), r6(x - ox)] for z, x in h] for h in holes]
    elif g[0] == "rev":
        _, xc, y0, zc, prof, closed, lat, _o = g
        pts = [[r6(a), r6(r)] for a, r in prof]
        if not closed:
            pts = [[0.0, 0.0]] + pts + [[pts[-1][0], 0.0]]
        s.update({"kind": "revolve", "axis": [0, 1, 0], "origin": [r6(xc - ox), r6(y0 - oy), r6(zc)], "profile": pts})
        if lat != 1.0:
            s["lateralScale"] = lat
    elif g[0] == "cap":
        _, p0, p1, r = g
        s.update({"kind": "capsule", "p0": [r6(p0[0] - ox), r6(p0[1] - oy), r6(p0[2])], "p1": [r6(p1[0] - ox), r6(p1[1] - oy), r6(p1[2])], "r0": r, "r1": r})
    if p["rep"]:
        n, t = p["rep"]
        s["repeat"] = {"count": n, "translation": [r6(c) for c in t]}
    for k, v in p["flags"].items():
        s[k] = v
    return s


def poly_area(pts):
    """🧮️ Signed shoelace area of a ring."""
    return sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / 2


def ring_centroid(pts):
    """🎯️ Area and centroid of a ring."""
    a = poly_area(pts)
    cx = sum((x0 + x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / (6 * a)
    cy = sum((y0 + y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(pts, pts[1:] + pts[:1])) / (6 * a)
    return abs(a), cx, cy


def volume_centre(p):
    """⚖️ World volume (all copies) and world centre of a part."""
    g = p["geom"]
    if g[0] == "box":
        _, x0, x1, y0, y1, z0, z1 = g
        v, c = (x1 - x0) * (y1 - y0) * (z1 - z0), [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2]
    elif g[0] in ("exy", "eyz", "ezx"):
        _, pts, s0, s1, holes = g
        a, c0, c1 = ring_centroid(pts)
        m0, m1 = a * c0, a * c1
        for h in holes:
            ah, h0, h1 = ring_centroid(h)
            a -= ah
            m0 -= ah * h0
            m1 -= ah * h1
        c0, c1 = m0 / a, m1 / a
        v = a * (s1 - s0)
        c = {"exy": [c0, c1, (s0 + s1) / 2], "eyz": [(s0 + s1) / 2, c0, c1], "ezx": [c1, (s0 + s1) / 2, c0]}[g[0]]
    elif g[0] == "rev":
        _, xc, y0, zc, prof, closed, lat, _o = g
        ring = list(prof[:-1]) if closed else [(0.0, 0.0)] + list(prof) + [(prof[-1][0], 0.0)]
        s1 = s2 = 0.0
        for (a0, r0), (a1, r1) in zip(ring, ring[1:] + ring[:1]):
            cr = a0 * r1 - a1 * r0
            s1 += cr * (r0 + r1) / 6
            s2 += cr * (2 * a0 * r0 + a0 * r1 + a1 * r0 + 2 * a1 * r1) / 24
        v = abs(2 * math.pi * s1) * lat
        c = [xc, y0 + s2 / s1, zc]
    else:
        _, p0, p1, r = g
        L = math.dist(p0, p1)
        v, c = math.pi * r * r * L + 4 / 3 * math.pi * r ** 3, [(p0[i] + p1[i]) / 2 for i in range(3)]
    if p["rep"]:
        n, t = p["rep"]
        v *= n
        c = [c[i] + t[i] * (n - 1) / 2 for i in range(3)]
    return v, c


def masses(parts):
    """⚖️ Mass, world centre and parts per bone from section x density."""
    out = {}
    for p in parts:
        v, c = volume_centre(p)
        kg = v * DENS[p["material"]]
        a = out.setdefault(p["bone"], [0.0, 0.0, 0.0, 0.0, {}])
        a[0] += kg
        for i in range(3):
            a[1 + i] += kg * c[i]
        a[4][p["id"]] = round(kg, 3)
    return {b: {"kg": a[0], "x": a[1] / a[0], "y": a[2] / a[0], "z": a[3] / a[0], "parts": a[4]} for b, a in out.items()}
