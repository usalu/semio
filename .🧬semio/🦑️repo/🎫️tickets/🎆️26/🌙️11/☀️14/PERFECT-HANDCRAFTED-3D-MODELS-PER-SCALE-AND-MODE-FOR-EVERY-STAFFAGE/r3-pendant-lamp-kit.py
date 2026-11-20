"""💡 Round 3 pendant lamp: the real measures of an enamelled dome pendant (ceiling rose, textile cord, brass cord-grip collar on an M10 nipple, 0.6 mm enamelled steel dome, E27 brass lampholder inside the dome, A60 60 W bulb) and the profile and solid writers every scale states its parts with.

Frames: every bone hangs straight down (rose angleDeg -90, children 0), so u runs down along the bone from its joint and v is world ahead; world height = joint height - u. Joints at rest (world y): rose 0.8 (ceiling), cord 0.776 (under the rose), holder 0.176 (top of the collar), shade 0.146 (outer apex of the dome), bulb 0.105 (top contact of the E27 cap); the bulb tip is the origin. Every number is metres.
"""
import math

Y_ROSE = 0.8
ROSE_H = 0.024
CORD_L = 0.6
Y_HOLDER = Y_ROSE - ROSE_H - CORD_L
COLLAR_H = 0.030
Y_SHADE = Y_HOLDER - COLLAR_H
SHEET = 0.0006
SOCKET_TOP = COLLAR_H + SHEET
SOCKET_L = 0.065
SOCKET_MOUTH = SOCKET_TOP + SOCKET_L
SOCKET_R = 0.020
BORE_R = 0.0142
BULB_L = 0.105
Y_BULB = BULB_L
BULB_AT = Y_HOLDER - Y_BULB
CAP_L = 0.027
CAP_R = 0.0135
CORD_R = 0.0033
SHADE_H = 0.1385
DOME = ((0.017, 0.0), (0.06, 0.0), (0.11, 0.03), (0.13, 0.133))
BEAD_C = (0.1355, 0.130)
BEAD_R = 0.003
NIPPLE_R = 0.0055

GLASS = [(0.0805, 0.0128), (0.078, 0.0138), (0.07439, 0.01439), (0.07087, 0.0154), (0.06717, 0.01678), (0.063, 0.0185), (0.05855, 0.02091), (0.05407, 0.02348), (0.04938, 0.02594), (0.04426, 0.02802), (0.03853, 0.02946), (0.032, 0.03), (0.02546, 0.02929), (0.01941, 0.02728), (0.01396, 0.0242), (0.00925, 0.02025), (0.00538, 0.01564), (0.00247, 0.01059), (0.00064, 0.00531), (0.0, 0.0)]


def r5(x):
    return round(x, 5) + 0.0


def bez(t, p=DOME):
    """〰️ Point (r, u) of the dome's cubic Bezier at parameter t."""
    a = (1 - t) ** 3
    b = 3 * (1 - t) ** 2 * t
    c = 3 * (1 - t) * t ** 2
    d = t ** 3
    return (a * p[0][0] + b * p[1][0] + c * p[2][0] + d * p[3][0], a * p[0][1] + b * p[1][1] + c * p[2][1] + d * p[3][1])


def bez_d(t, p=DOME):
    a = 3 * (1 - t) ** 2
    b = 6 * (1 - t) * t
    c = 3 * t ** 2
    return (a * (p[1][0] - p[0][0]) + b * (p[2][0] - p[1][0]) + c * (p[3][0] - p[2][0]), a * (p[1][1] - p[0][1]) + b * (p[2][1] - p[1][1]) + c * (p[3][1] - p[2][1]))


def dome_points(n, t_end=1.0, offset=0.0):
    """🔔 Dome surface (u, r) from the apex landing to t_end in n steps; offset > 0 moves it inward (towards the inside of the shade) along the normal."""
    out = []
    for k in range(n + 1):
        t = t_end * k / n
        r, u = bez(t)
        dr, du = bez_d(t)
        L = math.hypot(dr, du)
        out.append((u + offset * dr / L, r - offset * du / L))
    return out


def bead_entry():
    """⭕ Parameter where the outer dome surface enters the rolled rim bead."""
    lo, hi = 0.8, 1.0
    for _ in range(60):
        m = (lo + hi) / 2
        r, u = bez(m)
        if math.hypot(u - BEAD_C[0], r - BEAD_C[1]) > BEAD_R:
            lo = m
        else:
            hi = m
    return lo


def bead_arc(phi0, phi1, steps):
    return [(BEAD_C[0] + BEAD_R * math.cos(math.radians(phi0 + (phi1 - phi0) * k / steps)), BEAD_C[1] + BEAD_R * math.sin(math.radians(phi0 + (phi1 - phi0) * k / steps))) for k in range(steps + 1)]


def shade50():
    """🔔 1:50 shade: 0.6 mm enamelled sheet from the nipple hole over the dome into an outward rolled rim bead of 6 mm, as one closed loop (u, r)."""
    te = bead_entry()
    outer = [(0.0, NIPPLE_R)] + dome_points(40, te)
    u, r = outer[-1]
    phi_in = math.degrees(math.atan2(r - BEAD_C[1], u - BEAD_C[0])) % 360.0
    arc = bead_arc(phi_in, -75.0, 16)[1:]
    inner = dome_points(40, te * 0.985, SHEET)
    inner.reverse()
    return outer + arc + inner + [(SHEET, NIPPLE_R), (0.0, NIPPLE_R)]


def shade100():
    """🔔 1:100 shade: the same 0.6 mm sheet on a ten-step dome; the bead becomes a short flared lip to r 0.133 at the rim."""
    outer = [(0.0, NIPPLE_R)] + dome_points(10, 0.97)
    outer.append((SHADE_H, 0.133))
    inner = dome_points(10, 0.97, SHEET)
    inner.reverse()
    return outer + [(SHADE_H, 0.133 - 0.0015)] + inner + [(SHEET, NIPPLE_R), (0.0, NIPPLE_R)]


def shade200():
    """🔔 1:200 shade with the collar merged as its neck: one closed turned body, 3 mm wall."""
    wall = 0.003
    outer = [(-COLLAR_H, 0.0), (-COLLAR_H, 0.0155), (0.0, 0.0155)] + dome_points(6, 0.97)
    outer.append((SHADE_H, 0.133))
    inner = dome_points(6, 0.97, wall)
    inner.reverse()
    return outer + [(SHADE_H, 0.133 - wall)] + inner + [(wall, 0.0)]


ROSE = {
    "50": [(0.0, 0.0), (0.0, 0.05), (0.016, 0.05)] + [(0.016 + 0.008 * math.sin(math.radians(a)), 0.042 + 0.008 * math.cos(math.radians(a))) for a in (22.5, 45, 67.5, 90)] + [(ROSE_H, 0.0)],
    "100": [(0.0, 0.0), (0.0, 0.05), (0.015, 0.05), (0.0215, 0.047), (ROSE_H, 0.04), (ROSE_H, 0.0)],
    "200": [(0.0, 0.0), (0.0, 0.05), (0.03, 0.05), (0.03, 0.0)],
}
ROSE_GRIP = [(0.02, 0.0), (0.02, 0.007), (0.031, 0.007), (0.034, 0.0055), (0.034, 0.0)]
COLLAR = {
    "50": [(-0.0005, 0.0), (-0.0005, 0.0055), (0.0015, 0.0065), (0.0025, 0.0095), (0.0045, 0.0128), (0.0075, 0.0150), (0.0105, 0.0155), (0.0290, 0.0155), (0.0302, 0.0145), (0.0302, 0.0)],
    "100": [(0.0, 0.0), (0.0, 0.0105), (0.007, 0.0155), (0.0302, 0.0155), (0.0302, 0.0)],
}
SOCKET = {
    "50": [(SOCKET_TOP, 0.0), (SOCKET_TOP, 0.016), (SOCKET_TOP + 0.0012, 0.0185), (SOCKET_TOP + 0.004, SOCKET_R), (SOCKET_MOUTH - 0.0015, SOCKET_R), (SOCKET_MOUTH, SOCKET_R - 0.0015), (SOCKET_MOUTH, BORE_R), (BULB_AT + 0.003, BORE_R), (BULB_AT + 0.003, 0.0)],
    "100": [(SOCKET_TOP, 0.0), (SOCKET_TOP, SOCKET_R), (SOCKET_MOUTH, SOCKET_R), (SOCKET_MOUTH, BORE_R), (BULB_AT + 0.003, BORE_R), (BULB_AT + 0.003, 0.0)],
}
CAP = [(0.0, 0.0), (0.0, 0.0045), (0.0015, 0.0047), (0.0032, 0.0105), (0.0045, CAP_R), (CAP_L, CAP_R), (CAP_L, 0.0)]


def glass(points=GLASS):
    """💡 A60 glass envelope in the bulb frame (u from the cap top down), closed on the axis at both ends."""
    return [(BULB_L - points[0][0], 0.0)] + [(BULB_L - y, r) for y, r in points]


def bulb100():
    """💡 1:100 bulb: cap and glass as one turned body (the cap shows 2.4 mm under the socket)."""
    g = [GLASS[1]] + GLASS[3::2]
    return [(0.0, 0.0), (0.0, 0.009), (0.004, CAP_R), (CAP_L, CAP_R)] + [(BULB_L - y, r) for y, r in g]


def bulb200():
    """💡 1:200 bulb with the lampholder merged: socket body from the shade's inner apex, cap and a six-point glass pear."""
    top = SOCKET_TOP + 0.0024 - BULB_AT
    g = [GLASS[1], GLASS[5], GLASS[9], GLASS[11], GLASS[14], GLASS[17], GLASS[-1]]
    return [(top, 0.0), (top, SOCKET_R), (SOCKET_MOUTH - BULB_AT, SOCKET_R), (SOCKET_MOUTH - BULB_AT, CAP_R)] + [(BULB_L - y, r) for y, r in g]


def glass200():
    """💡 1:200 glass of the thermal list: the bulb body below the lampholder mouth (cap stub and pear), without the merged lampholder."""
    return [(SOCKET_MOUTH - BULB_AT, 0.0)] + bulb200()[3:]


def solid(sid, kind, bone, material, role, tone, **extra):
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(extra)
    return s


def revolve(sid, bone, material, role, tone, profile, **extra):
    return solid(sid, "revolve", bone, material, role, tone, profile=[[r5(a), r5(r)] for a, r in profile], axis=[1, 0, 0], origin=[0, 0, 0], **extra)


def rod(sid, bone, material, role, tone, u0, u1, r, **extra):
    return revolve(sid, bone, material, role, tone, [(u0, 0.0), (u0, r), (u1, r), (u1, 0.0)], **extra)


def ellipsoid(sid, bone, material, role, tone, radii, at, **extra):
    return solid(sid, "ellipsoid", bone, material, role, tone, radii=[r5(c) for c in radii], transform={"translation": [r5(at[0]), r5(at[1]), r5(at[2])]}, **extra)


def capsule(sid, bone, material, role, tone, p0, p1, r, **extra):
    return solid(sid, "capsule", bone, material, role, tone, p0=[r5(c) for c in p0], p1=[r5(c) for c in p1], r0=r5(r), r1=r5(r), **extra)


def box(sid, bone, material, role, tone, size, at, corner=0.0, **extra):
    s = solid(sid, "box", bone, material, role, tone, size=[r5(c) for c in size], transform={"translation": [r5(at[0]), r5(at[1]), r5(at[2])]}, **extra)
    if corner:
        s["cornerRadius"] = r5(corner)
    return s


def plume_patch(r1, r0=0.0155, proud=0.0015, n=8):
    """🔥 Outlet patch of the shade plume (D2): the crown of the dome from the collar out to r1, a skin 0.5 mm into and proud by `proud` over the outer surface, so it lies flush on the crown; r1 is chosen per scale so that the patch's drop over the crown reaches the smallest feature."""
    t1 = 0.0
    while bez(t1)[0] < r1:
        t1 += 0.0005
    t0 = 0.0
    while bez(t0)[0] < r0:
        t0 += 0.0005
    outer = []
    inner = []
    for k in range(n + 1):
        t = t0 + (t1 - t0) * k / n
        r, u = bez(t)
        dr, du = bez_d(t)
        L = math.hypot(dr, du)
        outer.append((u - proud * dr / L, r + proud * du / L))
        inner.append((u + 0.0005 * dr / L, r - 0.0005 * du / L))
    outer.insert(0, (-proud, r0))
    inner.insert(0, (0.0005, r0))
    inner.reverse()
    return outer + inner
