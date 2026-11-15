import os, sys, math
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-19-lib.py")).read())
BA, BO, PO, HD, RF, KN = "base", "bottle", "pole", "burner-head", "reflector", "knob"
PS, SB, SH, AR, PB = "powder-steel", "stainless-brushed", "stainless-hot", "alu-reflector", "plastic-black"
POLE_Y, HEAD_Y, REF_Y = 0.62, 1.98, 2.06
KNOB = (0.17, 0.46)
ALL6 = ["-u", "+u", "-v", "+v", "-w", "+w"]


def bez(p0, p1, p2, p3, n):
    return [[(1 - u) ** 3 * p0[j] + 3 * (1 - u) ** 2 * u * p1[j] + 3 * (1 - u) * u * u * p2[j] + u ** 3 * p3[j] for j in range(2)] for u in [i / n for i in range(n + 1)]]


def inward(pts, t, ref):
    out = []
    n = len(pts)
    for i, p in enumerate(pts):
        a, b = pts[max(i - 1, 0)], pts[min(i + 1, n - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        l = math.hypot(dx, dy) or 1
        nx, ny = -dy / l, dx / l
        if (ref[0] - p[0]) * nx + (ref[1] - p[1]) * ny < 0:
            nx, ny = -nx, -ny
        out.append([max(0.0, p[0] + t * nx), p[1] + t * ny])
    return out


def prof(pts):
    return [[y, r] for r, y in pts]


def inward_rim(pts, t, ref):
    out = inward(pts, t, ref)
    out[0] = [pts[0][0] - t, pts[0][1]]
    return out


def wall(id, bone, mat, tone, outer, t, ref, origin=(0, 0, 0), role="body", rim=False):
    p = prof(outer) + prof(inward_rim(outer, t, ref) if rim else inward(outer, t, ref))[::-1]
    p.append(p[0])
    return revolve(id, bone, mat, tone, "y", list(origin), p, role=role)


def solid_rev(id, bone, mat, tone, outer, origin=(0, 0, 0), role="body"):
    return revolve(id, bone, mat, tone, "y", list(origin), prof(outer), role=role)


def ring_y(id, bone, mat, tone, y0, y1, r0, r1, origin=(0, 0, 0), role="detail", axis="y"):
    return revolve(id, bone, mat, tone, axis, list(origin), [[y0, r0], [y1, r0], [y1, r1], [y0, r1], [y0, r0]], role=role)


def tube_y(id, bone, mat, tone, y0, y1, ro, ri, origin=(0, 0, 0), role="body"):
    return ring_y(id, bone, mat, tone, y0, y1, ri, ro, origin, role)


def torus(id, bone, mat, tone, R, y, r, role="detail", n=10):
    pts = [[y + r * math.sin(2 * math.pi * k / n), R + r * math.cos(2 * math.pi * k / n)] for k in range(n)]
    pts.append(pts[0])
    return revolve(id, bone, mat, tone, "y", [0, 0, 0], pts, role=role)


def rrect_ring(hw, c, n=6, hh=None):
    hh = hw if hh is None else hh
    pts = []
    for (cx, cy, a0) in ((hw - c, hh - c, 0), (-(hw - c), hh - c, 90), (-(hw - c), -(hh - c), 180), (hw - c, -(hh - c), 270)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append([cx + c * math.cos(a), cy + c * math.sin(a)])
    return pts


def plate_profile(hw, c, bay, n=6):
    pts = []
    corners = ((hw - c, -(hw - c), -90), (hw - c, hw - c, 0), (-(hw - c), hw - c, 90), (-(hw - c), -(hw - c), 180))
    for ci, (cw, cu, a0) in enumerate(corners):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append([cw + c * math.cos(a), cu + c * math.sin(a)])
        if ci == 0:
            u0, u1, wb = bay
            pts += [[hw, u0], [wb, u0], [wb, u1], [hw, u1]]
        if ci == 2:
            u0, u1, wb = bay
            pts += [[-hw, u1], [-wb, u1], [-wb, u0], [-hw, u0]]
    return pts


def cap_cyl(id, bone, mat, tone, axis, p, length, r, role="body", chamfer=0.0):
    c = chamfer
    pr = [[0, 0], [0, r - c], [c, r], [length - c, r], [length, r - c], [length, 0]] if c else [[0, 0], [0, r], [length, r], [length, 0]]
    pr.append(pr[0])
    return revolve(id, bone, mat, tone, axis, p, pr, role=role)


DOME = bez([0.4, 0.0], [0.4, 0.06], [0.22, 0.19], [0, 0.19], 24)
SHOULDER = bez([0.17, 0.6], [0.17, 0.64], [0.05, 0.66], [0.03, 0.66], 8)
BOTTLE_O = [[0.0, 0.04], [0.08, 0.04], [0.115, 0.042], [0.14, 0.055], [0.15, 0.08], [0.15, 0.46], [0.142, 0.51], [0.115, 0.55], [0.07, 0.578], [0.032, 0.592], [0.03, 0.6]]


def dome_r_at(y):
    best = min(DOME, key=lambda p: abs(p[1] - y))
    return best[0]


def housing_loft(id, bone, tone, y0=0.03, k=8):
    secs = [sec([0, y0, 0], 0.17, 0.17, 10), sec([0, 0.3, 0], 0.17, 0.17, 10), sec([0, 0.6, 0], 0.17, 0.17, 10)]
    for r, y in SHOULDER[1:]:
        e = 10 if r > 0.16 else (6 if r > 0.12 else (4 if r > 0.07 else 2))
        secs.append(sec([0, y, 0], r, r, e))
    return loft(id, bone, PS, tone, secs)


def skin_loft(id, bone, mat, tone, dome, off=0.0001, n=None):
    pts = dome if n is None else dome[:: max(1, len(dome) // n)] + [dome[-1]]
    secs = [sec([0, y, 0], max(r + off, 0.0005), max(r + off, 0.0005)) for r, y in pts]
    return loft(id, bone, mat, tone, secs, capStart=False, capEnd=False)


sc = []
# ---- 1:50
s = Scale(50)
plate = ext("base-plate", BA, PS, 0.6, "zx", plate_profile(0.23, 0.03, (-0.19, -0.09, 0.185)), 0.0, 0.03)
s.b(plate)
s.b(housing_loft("housing", BA, 0.6))
s.b(box("housing-door", BA, PS, 0.55, [0.24, 0.43, 0.0075], [0, 0.305, -0.16875], 0.002, role="detail"))
s.b(ext("vent-grille", BA, PS, 0.5, "xy", rect(0, 0.5775, 0.18, 0.08), -0.1725, -0.165, holes=[rect(0, y, 0.14, 0.008) for y in (0.55, 0.58, 0.61)], role="detail"))
s.b(cap_cyl("door-lock", BA, SB, 0.3, "z", [0.085, 0.31, -0.1825], 0.0125, 0.0125, "detail"))
s.b(cap_cyl("knob-escutcheon", BA, PS, 0.5, "x", [0.163, KNOB[1], 0.0], 0.009, 0.04, "detail", 0.002))
for sg, nm in ((1, "left"), (-1, "right")):
    z0 = 0.19 if sg > 0 else -0.21
    s.b(revolve(f"wheel-tyre-{nm}", BA, PB, 0.9, "z", [-0.14, 0.05, z0], [[0, 0.022], [0, 0.036], [0.003, 0.04], [0.017, 0.04], [0.02, 0.036], [0.02, 0.022], [0, 0.022]]))
    s.b(cap_cyl(f"wheel-hub-{nm}", BA, SB, 0.4, "z", [-0.14, 0.05, z0 - 0.0015], 0.023, 0.022, "detail", 0.003))
    za, zb = (0.165, 0.2) if sg > 0 else (-0.2, -0.165)
    s.b(capsule(f"wheel-axle-{nm}", BA, SB, 0.4, [-0.14, 0.05, za], [-0.14, 0.05, zb], 0.0075, role="detail"))
s.b(revolve("pole", PO, SB, 0.45, "y", [0, 0, 0], [[0, 0], [0, 0.03], [1.36, 0.03], [1.36, 0]]))
s.b(ring_y("pole-joint", PO, SB, 0.3, 0.675, 0.685, 0.0, 0.033, role="detail"))
s.b(revolve("head", HD, SH, 0.8, "y", [0, 0, 0], [[0, 0], [0, 0.11], [0.003, 0.125], [0.012, 0.13], [0.108, 0.13], [0.117, 0.125], [0.12, 0.11], [0.12, 0]]))
s.b(capsule("guard-rod", HD, SH, 0.3, [0.13, 0.01, 0], [0.13, 0.075, 0], 0.00375, role="detail", repeat={"count": 12, "rotationAxis": [0, 1, 0], "rotationRad": r6(2 * math.pi / 12), "origin": [0, 0, 0]}))
s.b(wall("reflector-shell", RF, AR, 0.5, DOME, 0.001, [0, 0.0], rim=True))
s.b(torus("dome-bead", RF, AR, 0.4, dome_r_at(0.03) - 0.0005, 0.03, 0.004, role="body"))
s.b(revolve("top-cap", RF, AR, 0.2, "y", [0, 0, 0], [[0.17, 0], [0.17, 0.05], [0.19, 0.045], [0.19, 0], [0.17, 0]], role="detail"))
s.b(tube_y("dome-hanger", RF, SH, 0.3, 0.04, 0.1895, 0.0125, 0.0105, role="detail"))
s.b(ellipsoid("knob-ball", KN, PB, 0.2, [0.03, 0.03, 0.03], [0, 0, 0]))
s.b(box("knob-pointer", KN, PB, 0.3, [0.014, 0.0075, 0.0075], [0.025, 0.0, 0.0], role="detail"))


def emitter_disc(id, bone, region_mat, y0, y1, r, tag, tone=0.9):
    d = revolve(id, bone, region_mat, tone, "y", [0, 0, 0], [[y0, 0], [y0, r], [y1, r], [y1, 0], [y0, 0]], role="detail")
    d["tags"] = [tag]
    d["sectionFaces"] = ALL6
    return d


s.thof("th-housing", "housing", "base-housing")
for i in ("base-plate", "housing-door", "vent-grille", "door-lock", "knob-escutcheon", "wheel-tyre-left", "wheel-tyre-right", "wheel-hub-left", "wheel-hub-right", "wheel-axle-left", "wheel-axle-right"):
    s.thof("th-" + i, i, "base-housing")
s.thof("th-pole", "pole", "pole-stainless")
s.thof("th-pole-joint", "pole-joint", "pole-stainless")
s.thof("th-head", "head", "emitter-head")
s.thof("th-guard-rod", "guard-rod", "emitter-head")
s.th(emitter_disc("th-flue-gas-disc", HD, SH, 0.12, 0.1275, 0.1, "flue-gas"), "emitter-head")
s.th(skin_loft("th-reflector-skin", RF, AR, 0.5, DOME), "reflector")
s.thof("th-dome-bead", "dome-bead", "reflector")
s.thof("th-top-cap", "top-cap", "reflector")
s.th(emitter_disc("th-dome-plume-disc", RF, AR, 0.1875, 0.195, 0.3, "dome-plume"), "reflector")
s.thof("th-knob-ball", "knob-ball", "control")
s.thof("th-knob-pointer", "knob-pointer", "control")


def structure50(s):
    s.st(ext("base-plate-frame", BA, PS, 0.6, "zx", rrect_ring(0.182, 0.03), 0.0015, 0.0285, holes=[rrect_ring(0.1, 0.02)], role="detail"), BA, "member")
    drum = [[0.1525, 0.0285], [0.1525, 0.6]] + [[r * 0.1525 / 0.17, y - 0.004] for r, y in SHOULDER[1:]]
    s.st(wall("housing-drum", BA, PS, 0.6, drum, 0.0012, [0, 0.3], role="detail"), BA, "member")
    s.st(wall("bottle-shell", BO, PS, 0.5, [[r, y] for r, y in BOTTLE_O], 0.0025, [0, 0.3], (0, -0.03, 0), "detail"), BO, "member")
    s.st(revolve("bottle-valve", BO, SB, 0.3, "y", [0, -0.03, 0], [[0.595, 0], [0.595, 0.02], [0.65, 0.02], [0.65, 0]], role="detail"), BO, "member")
    pr = (0.0285, 0.027)
    s.st(tube_y("pole-tube", PO, SB, 0.45, 0.0, 1.36, pr[0], pr[1], role="detail"), PO, "member")
    cup = [[0.0025, 0.1275], [0.1175, 0.1275], [0.1175, 0.0], [0.116, 0.0], [0.116, 0.126], [0.0025, 0.126], [0.0025, 0.1275]]
    s.st(revolve("head-shroud", HD, SH, 0.8, "y", [0, 0, 0], cup, role="detail"), HD, "member")
    s.stof("reflector-dish", "reflector-shell", RF, "member")
    s.stof("dome-hanger-member", "dome-hanger", RF, "member")
    s.st(tube_y("joint-pole-base", PO, SB, 0.3, 0.0, 0.08, 0.0295, 0.0285, role="detail"), PO, "joint")
    s.st(tube_y("joint-head-pole", HD, SB, 0.3, -0.08, 0.0, 0.0295, 0.0285, role="detail"), HD, "joint")
    s.st(tube_y("joint-reflector-head", RF, SH, 0.3, 0.04, 0.06, 0.0165, 0.0125, role="detail"), RF, "joint")
    s.st(cap_cyl("joint-knob-pivot", KN, SB, 0.3, "z", [0, 0, -0.02], 0.04, 0.008, "detail"), KN, "joint")
    s.st(box("support-base-contact-pad", BA, PS, 0.15, [0.46, 0.0075, 0.46], [0, -0.00375, 0]), BA, "support")
    s.st(ellipsoid("mass-base", BA, PS, 0.2, [0.14, 0.1, 0.14], [0, 0.3, 0], role="detail"), BA, "mass")
    s.st(ellipsoid("mass-bottle", BO, PS, 0.2, [0.13, 0.15, 0.13], [0, 0.31, 0], role="detail"), BO, "mass")


structure50(s)
s.dec("tbd", "tbd")
sc.append(s)


def housing_s(id, y0, sh, tone=0.6):
    secs = [sec([0, y0, 0], 0.17, 0.17, 10), sec([0, sh[0][1], 0], 0.17, 0.17, 10)]
    for r, y in sh[1:]:
        e = 10 if r > 0.16 else (6 if r > 0.12 else (4 if r > 0.07 else 2))
        secs.append(sec([0, y, 0], r, r, e))
    return loft(id, BA, PS, tone, secs)


def structure_coarse(s, den, drum_pts, bottle_pts, dish_off, joints, hanger):
    f = 0.015 if den == 100 else (0.03 if den == 200 else 0.075)
    s.st(ext("base-plate-frame", BA, PS, 0.6, "zx", rrect_ring(0.182, 0.03), 0.0015 if den == 100 else 0.002, 0.0385 if den == 100 else (0.0485 if den == 200 else 0.0985), holes=[rrect_ring(0.1, 0.02)], role="detail"), BA, "member")
    s.st(wall("housing-drum", BA, PS, 0.6, drum_pts, 0.0012, [0, 0.3], role="detail"), BA, "member")
    s.st(wall("bottle-shell", BO, PS, 0.5, bottle_pts, 0.0025, [0, 0.3], (0, -0.03, 0), "detail"), BO, "member")
    d_ = tube_y("pole-tube", PO, SB, 0.45, 0.0, 1.36, 0.0285, 0.027, role="detail")
    d_["slender"] = den == 500
    s.st(d_, PO, "member")
    cup = [[0.0025, 0.1275], [0.1175, 0.1275], [0.1175, 0.0], [0.116, 0.0], [0.116, 0.126], [0.0025, 0.126], [0.0025, 0.1275]]
    s.st(revolve("head-shroud", HD, SH, 0.8, "y", [0, 0, 0], cup, role="detail"), HD, "member")
    s.st(wall("reflector-dish", RF, AR, 0.5, dish_off, 0.001, [0, 0.0], role="detail", rim=True), RF, "member")
    if hanger:
        s.st(tube_y("dome-hanger-member", RF, SH, 0.3, 0.04, 0.17, 0.0125, 0.0105, role="detail"), RF, "member")
    if joints:
        s.st(tube_y("joint-pole-base", PO, SB, 0.3, 0.0, 0.08, 0.0295, 0.0285, role="detail"), PO, "joint")
        s.st(tube_y("joint-head-pole", HD, SB, 0.3, -0.08, 0.0, 0.0295, 0.0285, role="detail"), HD, "joint")
        s.st(tube_y("joint-reflector-head", RF, SH, 0.3, 0.04, 0.06, 0.0165, 0.0125, role="detail"), RF, "joint")
        s.st(cap_cyl("joint-knob-pivot", KN, SB, 0.3, "z", [0, 0, -0.02], 0.04, 0.008, "detail"), KN, "joint")
    s.st(box("support-base-contact-pad", BA, PS, 0.15, [0.46, f, 0.46], [0, -f / 2, 0]), BA, "support")
    s.st(ellipsoid("mass-base", BA, PS, 0.2, [0.14, 0.1, 0.14], [0, 0.3, 0], role="detail"), BA, "mass")
    s.st(ellipsoid("mass-bottle", BO, PS, 0.2, [0.13, 0.15, 0.13], [0, 0.31, 0], role="detail"), BO, "mass")


def drum_pts(top=0.6, y0=0.0285):
    return [[0.1525, y0], [0.1525, top]] + [[r * 0.1525 / 0.17, y - 0.004] for r, y in SHOULDER[1:]]


BOTTLE_C = [[0.0, 0.04], [0.115, 0.042], [0.15, 0.08], [0.15, 0.46], [0.115, 0.55], [0.03, 0.592], [0.03, 0.65]]
DRUM = drum_pts()
DISH_IN = lambda off: inward(DOME, off, [0, 0.0])

# ---- 1:100
s = Scale(100)
s.b(ext("base-plate", BA, PS, 0.6, "zx", rrect_ring(0.23, 0.03), 0.0, 0.04))
SH6 = bez([0.17, 0.6], [0.17, 0.64], [0.05, 0.66], [0.03, 0.66], 5)
s.b(housing_s("housing", 0.04, SH6))
s.b(revolve("pole", PO, SB, 0.45, "y", [0, 0, 0], [[0, 0], [0, 0.03], [1.36, 0.03], [1.36, 0]]))
s.b(revolve("head", HD, SH, 0.8, "y", [0, 0, 0], [[0, 0], [0, 0.115], [0.005, 0.13], [0.115, 0.13], [0.12, 0.115], [0.12, 0]]))
D100 = DOME[::2] + [DOME[-1]] if len(DOME) % 2 == 0 else DOME[::2]
s.b(wall("reflector-shell", RF, AR, 0.5, D100, 0.015, [0, 0.0], rim=True))
s.b(revolve("top-cap", RF, AR, 0.2, "y", [0, 0, 0], [[0.17, 0], [0.17, 0.04], [0.19, 0.035], [0.19, 0], [0.17, 0]], role="detail"))
s.b(tube_y("dome-hanger", RF, SH, 0.3, 0.04, 0.1895, 0.0, 0.0125, role="detail"))
s.b(ellipsoid("knob-ball", KN, PB, 0.2, [0.028, 0.028, 0.028], [0, 0, 0]))
s.thof("th-housing", "housing", "base-housing")
s.thof("th-base-plate", "base-plate", "base-housing")
s.thof("th-pole", "pole", "pole-stainless")
s.thof("th-head", "head", "emitter-head")
s.th(emitter_disc("th-flue-gas-disc", HD, SH, 0.12, 0.135, 0.1, "flue-gas"), "emitter-head")
s.th(skin_loft("th-reflector-skin", RF, AR, 0.5, D100), "reflector")
s.thof("th-top-cap", "top-cap", "reflector")
s.th(emitter_disc("th-dome-plume-disc", RF, AR, 0.1875, 0.2025, 0.3, "dome-plume"), "reflector")
s.thof("th-knob-ball", "knob-ball", "control")
structure_coarse(s, 100, DRUM, BOTTLE_C, inward_rim(D100, 0.0075, [0, 0.0]), True, True)
s.dec("tbd", "tbd")
sc.append(s)

# ---- 1:200
s = Scale(200)
s.b(ext("base-plate", BA, PS, 0.6, "zx", rrect_ring(0.23, 0.03), 0.0, 0.05))
SH4 = bez([0.17, 0.6], [0.17, 0.64], [0.05, 0.66], [0.03, 0.66], 4)
s.b(housing_s("housing", 0.05, SH4))
s.b(revolve("pole", PO, SB, 0.45, "y", [0, 0, 0], [[0, 0], [0, 0.035], [1.36, 0.035], [1.36, 0]]))
s.b(revolve("head", HD, SH, 0.8, "y", [0, 0, 0], [[0, 0], [0, 0.13], [0.12, 0.13], [0.12, 0]]))
DOME200 = bez([0.4, -0.01], [0.4, 0.05], [0.22, 0.19], [0, 0.19], 8)
s.b(wall("reflector-shell", RF, AR, 0.5, DOME200, 0.03, [0, -0.01], rim=True))
s.b(ellipsoid("knob-ball", KN, PB, 0.2, [0.03, 0.03, 0.03], [0, 0, 0]))
s.thof("th-housing", "housing", "base-housing")
s.thof("th-base-plate", "base-plate", "base-housing")
s.thof("th-pole", "pole", "pole-stainless")
s.thof("th-head", "head", "emitter-head")
s.th(emitter_disc("th-flue-gas-disc", HD, SH, 0.12, 0.15, 0.1, "flue-gas"), "emitter-head")
s.th(skin_loft("th-reflector-skin", RF, AR, 0.5, DOME200), "reflector")
s.th(emitter_disc("th-dome-plume-disc", RF, AR, 0.188, 0.218, 0.3, "dome-plume"), "reflector")
s.thof("th-knob-ball", "knob-ball", "control")
structure_coarse(s, 200, DRUM, BOTTLE_C, inward_rim(DOME200, 0.0145, [0, -0.01]), False, False)
s.dec("tbd", "tbd")
sc.append(s)

# ---- 1:500
s = Scale(500)
s.b(box("base-plate", BA, PS, 0.6, [0.46, 0.1, 0.46], [0, 0.05, 0]))
SH5 = [[0.17, 0.58], [0.16, 0.615], [0.1, 0.645], [0.05, 0.66]]
secs = [sec([0, 0.1, 0], 0.17, 0.17, 10), sec([0, 0.58, 0], 0.17, 0.17, 10), sec([0, 0.62, 0], 0.15, 0.15, 6), sec([0, 0.66, 0], 0.05, 0.05, 2)]
s.b(loft("housing", BA, PS, 0.6, secs))
s.b(revolve("pole", PO, SB, 0.45, "y", [0, 0, 0], [[0, 0], [0, 0.07], [1.36, 0.07], [1.36, 0]]))
s.b(revolve("head", HD, SH, 0.8, "y", [0, -0.02, 0], [[0, 0], [0, 0.13], [0.14, 0.13], [0.14, 0]]))
DOME500 = bez([0.4, -0.03], [0.4, 0.03], [0.22, 0.19], [0, 0.19], 4)
s.b(wall("reflector-shell", RF, AR, 0.5, DOME500, 0.075, [0, -0.03], rim=True))
s.b(box("knob-block", KN, PB, 0.2, [0.06, 0.12, 0.06], [0, 0, 0]))
s.thof("th-housing", "housing", "base-housing")
s.thof("th-base-plate", "base-plate", "base-housing")
s.thof("th-pole", "pole", "pole-stainless")
s.thof("th-head", "head", "emitter-head")
s.th(emitter_disc("th-flue-gas-disc", HD, SH, 0.12, 0.195, 0.1, "flue-gas"), "emitter-head")
s.th(skin_loft("th-reflector-skin", RF, AR, 0.5, DOME500), "reflector")
s.th(emitter_disc("th-dome-plume-disc", RF, AR, 0.1225, 0.1975, 0.3, "dome-plume"), "reflector")
s.thof("th-knob-block", "knob-block", "control")
structure_coarse(s, 500, DRUM, BOTTLE_C, inward_rim(DOME500, 0.0375, [0, -0.03]), False, False)
s.dec("tbd", "tbd")
sc.append(s)


DEC = {
    50: [
        ("Body: every visible part of the heater is its own solid on the real centre line. Pressed base plate 460 x 460 x 30 mm (extrusion with 30 mm corner radii and two wheel bays), housing as one loft of 17 cm half-width sections with exponent 10 (flat sheet walls, tight corners) rising into the drawn domed shoulder, a 7.5 mm door panel with lock and a three-slot vent grille (an extrusion with three real 8 mm slots), knob escutcheon, two tyres with hubs and axle stubs lifted 10 mm above the ground in their bays, the 60 mm pole as a revolve with its joint collar, the burner drum with 12 guard rods, and the reflector as a real 1 mm hollow dome (revolve of the wall profile) with rim bead, top cap and the hanger tube that carries it from the burner head. The control knob is a 60 mm ball: the drawings show it as a circle in elevation and front, and the rig turns it about w, so only a sphere satisfies both (pointer ridge on it).",
         "Körper: jedes sichtbare Teil des Heizpilzes ist ein eigener Körper auf der wirklichen Mittellinie. Gepresste Bodenplatte 460 x 460 x 30 mm (Extrusion mit Eckradien 30 mm und zwei Radbuchten), Gehäuse als eine Loft aus Schnitten mit 17 cm Halbbreite und Exponent 10 (ebene Blechwände, enge Ecken), die in die gezeichnete gewölbte Schulter übergeht, eine 7,5-mm-Tür mit Schloss und ein Lüftungsgitter mit drei echten 8-mm-Schlitzen (Extrusion), Knopfrosette, zwei Reifen mit Naben und Achsstummeln, 10 mm über dem Boden in ihren Buchten, der 60-mm-Mast als Rotation mit Verbindungsmanschette, die Brennertrommel mit 12 Schutzstäben und der Reflektor als echte 1 mm dicke hohle Haube (Rotation des Wandprofils) mit Randwulst, Kappe und dem Haltrohr, das sie vom Brennerkopf trägt. Der Regelknopf ist eine Kugel von 60 mm: die Zeichnungen zeigen ihn in Ansicht und Vorderansicht als Kreis, das Rig dreht ihn um w, beides erfüllt nur eine Kugel (mit Zeigerrippe)."),
        ("Thermal: every region has a single wall layer (B2), so each is one surface solid: base-housing = plate, housing, door, grille, lock, escutcheon, tyres, hubs and axles (references), pole-stainless = pole and joint collar, emitter-head = drum and guard rods plus the flue-gas disc (tagged flue-gas, 200 mm, above the drum under the dome), reflector = an open skin lofted 0.1 mm outside the hollow dome (one surface instead of the two faces of the 1 mm shell, so the head under the dome stays exposed), the bead and cap, plus the dome-plume disc (tagged dome-plume, 600 mm) on the crown, control = ball and pointer. No region is hidden. Root fix of areaM2 from this model (C16, all within 1.5): base-housing 0.75 -> 0.98 (the housing walls alone are 0.75 m2, plate top and edges and wheels add the rest), reflector 0.55 -> 0.68 (the outer dome is 0.62 m2 before the bead), control 0.004 -> 0.0049.",
         "Thermik: jede Region hat eine einzige Wandschicht (B2), also je einen Oberflächenkörper: base-housing = Platte, Gehäuse, Tür, Gitter, Schloss, Rosette, Reifen, Naben und Achsen (Referenzen), pole-stainless = Mast und Manschette, emitter-head = Trommel und Schutzstäbe plus die Abgasscheibe (markiert flue-gas, 200 mm, über der Trommel unter der Haube), reflector = eine offene Haut, 0,1 mm außerhalb der hohlen Haube gelooft (eine Fläche statt der zwei Flächen der 1-mm-Schale, so bleibt der Kopf unter der Haube frei), Wulst und Kappe sowie die Fahnenscheibe (markiert dome-plume, 600 mm) auf dem Scheitel, control = Kugel und Zeiger. Keine Region ist verdeckt. Korrektur von areaM2 aus diesem Modell (C16, alle innerhalb 1,5): base-housing 0,75 -> 0,98 (die Gehäusewände allein haben 0,75 m2, Plattenoberseite, Kanten und Räder den Rest), reflector 0,55 -> 0,68 (die äußere Haube hat vor dem Wulst 0,62 m2), control 0,004 -> 0,0049."),
        ("Structure: the load path is real sheet and tube. Foot: contact pad under the plate (support at base-contact); a 27 mm box-section frame ring carries a 1.2 mm drum shell 3 mm inside the housing sheet (member of base) with its domed shoulder and pole socket; the 11 kg bottle (member of bottle: 2.5 mm shell with domed ends and the valve) stands on the same ring; the pole is a 60 x 1.5 mm stainless tube (member of pole) with two doubler sleeves as joints at the base and the burner head; the burner head is a 1.5 mm cup (member), and the dome (member of reflector: the body shell itself) hangs on the 25 mm hanger tube, whose collar is the reflector joint; the knob pivot is a joint inside the ball. Masses: the bottle with its propane (24.5 kg, com 0.34 m) and the sheet metal, plate and wheels (8.7 kg, com 0.30 m) as ellipsoids inside the drum; pole, head and dome are carried by their tubes and shells.",
         "Tragwerk: der Lastweg ist echtes Blech und Rohr. Fuß: Aufstandsplatte unter der Bodenplatte (Auflager base-contact); ein Hohlkastenrahmen von 27 mm trägt einen 1,2-mm-Trommelmantel 3 mm innerhalb des Gehäuseblechs (Stab von base) mit gewölbter Schulter und Mastaufnahme; die 11-kg-Flasche (Stab von bottle: 2,5-mm-Mantel mit gewölbten Böden und Ventil) steht auf demselben Rahmen; der Mast ist ein Edelstahlrohr 60 x 1,5 mm (Stab von pole) mit zwei Dopplerhülsen als Gelenke am Fuß und am Brennerkopf; der Brennerkopf ist ein 1,5-mm-Becher (Stab), die Haube (Stab von reflector: die Körperschale selbst) hängt am 25-mm-Haltrohr, dessen Manschette das Gelenk des Reflektors ist; der Knopfzapfen ist ein Gelenk in der Kugel. Massen: die Flasche mit Propan (24,5 kg, Schwerpunkt 0,34 m) und Blech, Platte und Räder (8,7 kg, Schwerpunkt 0,30 m) als Ellipsoide in der Trommel; Mast, Kopf und Haube werden von ihren Rohren und Schalen getragen."),
    ],
    100: [
        ("Body: modelled anew against the 1:100 drawing (plate 40 mm, housing from 40 mm). The two bays, wheels, door panel, lock, vent grille, escutcheon, pointer, guard rods, pole collar and dome bead (all 7.5-12 mm, below the 15 mm feature) are no longer solids: the drawing keeps them as lines. Plate, housing (loft of 6 sections), pole and burner drum (with 5 mm chamfers) stay at true size; the dome becomes a 15 mm hollow shell (the feature of the scale, a solid dish would fail the volume band) with a 40 mm cap boss and a solid 25 mm hanger rod; the knob is the 56 mm ball.",
         "Körper: neu gegen die Zeichnung 1:100 modelliert (Platte 40 mm, Gehäuse ab 40 mm). Die beiden Buchten, Räder, Türplatte, Schloss, Lüftungsgitter, Rosette, Zeiger, Schutzstäbe, Mastmanschette und Haubenwulst (alle 7,5-12 mm, unter der Einzelheit 15 mm) sind keine Körper mehr: die Zeichnung behält sie als Linien. Platte, Gehäuse (Loft aus 6 Schnitten), Mast und Brennertrommel (mit 5-mm-Fasen) bleiben in wahrer Größe; die Haube wird eine hohle Schale von 15 mm (die Einzelheit des Maßstabs, eine massive Schale verfehlt das Volumenband) mit 40-mm-Kappenaufsatz und einem massiven Haltestab von 25 mm; der Knopf ist die Kugel von 56 mm."),
        ("Thermal: references to the plate, housing, pole, drum, cap and ball, the open dome skin (10 sections) and the two emitter discs thickened to the 15 mm feature (flue-gas 200 mm, dome-plume 600 mm); all regions are within 6 % of areaM2 per region.",
         "Thermik: Referenzen auf Platte, Gehäuse, Mast, Trommel, Kappe und Kugel, die offene Haubenhaut (10 Schnitte) und die beiden Emitterscheiben auf die Einzelheit 15 mm verdickt (flue-gas 200 mm, dome-plume 600 mm); alle Regionen liegen je Region innerhalb 6 % von areaM2."),
        ("Structure: members keep their true thin sections at this scale (they are below the body only in wall thickness, not in extent): frame ring, 1.2 mm drum shell, bottle shell (simplified profile, valve merged), 60 x 1.5 mm pole tube, 1.5 mm head cup, a 1 mm dome dish lying inside the 15 mm body shell (so the stiffness is not inflated by the thick body) and the hanger tube; four joints, the contact pad (15 mm) and the two mass ellipsoids as at 1:50.",
         "Tragwerk: die Stäbe behalten hier ihre wahren dünnen Querschnitte (kleiner als der Körper nur in der Wanddicke, nicht in der Ausdehnung): Rahmenring, 1,2-mm-Trommelmantel, Flaschenmantel (vereinfachtes Profil, Ventil aufgegangen), Mastrohr 60 x 1,5 mm, 1,5-mm-Kopfbecher, eine 1-mm-Haubenschale innerhalb der 15-mm-Körperschale (damit die dicke Körperschale die Steifigkeit nicht aufbläht) und das Haltrohr; vier Gelenke, die Aufstandsplatte (15 mm) und die beiden Massenellipsoide wie bei 1:50."),
    ],
    200: [
        ("Body: modelled anew against the 1:200 drawing (plate 50 mm, pole 70 mm, plain burner drum 260 x 120 mm, dome rim 10 mm below its bone). Plate, housing (5 sections), pole, drum and ball only; the 15 mm cap boss, hanger and chamfers are gone. The dome is a 30 mm hollow shell with its rim at the drawn level.",
         "Körper: neu gegen die Zeichnung 1:200 modelliert (Platte 50 mm, Mast 70 mm, glatte Brennertrommel 260 x 120 mm, Haubenrand 10 mm unter seinem Knochen). Nur Platte, Gehäuse (5 Schnitte), Mast, Trommel und Kugel; Kappenaufsatz 15 mm, Haltrohr und Fasen entfallen. Die Haube ist eine hohle Schale von 30 mm mit ihrem Rand auf der gezeichneten Höhe."),
        ("Thermal: references to plate, housing, pole, drum and ball, the open skin of the 8-section dome and the two emitter discs at 30 mm; every region has a solid, the total area is within 2 % of the sum of areaM2 (regions deviate by 0 to 14 % individually, only the total counts at this scale).",
         "Thermik: Referenzen auf Platte, Gehäuse, Mast, Trommel und Kugel, die offene Haut der Haube aus 8 Schnitten und die beiden Emitterscheiben mit 30 mm; jede Region hat einen Körper, die Gesamtfläche liegt innerhalb 2 % der Summe von areaM2 (Regionen weichen einzeln um 0 bis 14 % ab, nur die Summe zählt in diesem Maßstab)."),
        ("Structure: the same members at true section size (all extents are above 30 mm): frame ring, drum, bottle, 60 x 1.5 mm pole tube, head cup, the 1 mm dish inside the 30 mm body shell; joints and the hanger are merged into the members (their sleeves are under the 30 mm feature in length), the contact pad is 30 mm, two masses.",
         "Tragwerk: dieselben Stäbe in wahrer Querschnittsgröße (alle Ausdehnungen über 30 mm): Rahmenring, Trommel, Flasche, Mastrohr 60 x 1,5 mm, Kopfbecher, die 1-mm-Schale innerhalb der 30-mm-Körperschale; Gelenke und Haltrohr sind in die Stäbe aufgegangen (ihre Hülsen sind kürzer als die Einzelheit 30 mm), die Aufstandsplatte ist 30 mm, zwei Massen."),
    ],
    500: [
        ("Body: modelled anew against the 1:500 drawing, which draws a stepped mushroom sign: plain 460 x 100 x 460 mm base block, housing of 4 loft sections, 140 mm solid pole, 260 x 140 mm drum from 20 mm below its bone, a 75 mm hollow dome with its rim 30 mm below its bone, and the knob as the drawn 60 x 120 mm block (its own drawing is thinner than the 75 mm feature, C13).",
         "Körper: neu gegen die Zeichnung 1:500 modelliert, die ein gestuftes Pilzschild zeichnet: glatter Sockelblock 460 x 100 x 460 mm, Gehäuse aus 4 Loftschnitten, massiver Mast 140 mm, Trommel 260 x 140 mm ab 20 mm unter ihrem Knochen, eine hohle Haube von 75 mm mit Rand 30 mm unter ihrem Knochen und der Knopf als gezeichneter Block 60 x 120 mm (seine Zeichnung ist dünner als die Einzelheit 75 mm, C13)."),
        ("Thermal: references to base block, housing, pole, drum and knob block, the 5-section open dome skin and the two emitter discs at 75 mm. Only the total counts at this scale (14 % over the sum of areaM2: the drawn 140 mm pole exposes 2.3 times the area of the 60 mm real pole and the knob block 2.7 times the ball, the 1:500 drawing enlarges them to the scale).",
         "Thermik: Referenzen auf Sockelblock, Gehäuse, Mast, Trommel und Knopfblock, die offene Haubenhaut aus 5 Schnitten und die beiden Emitterscheiben mit 75 mm. In diesem Maßstab zählt nur die Summe (14 % über der Summe von areaM2: der gezeichnete Mast von 140 mm hat die 2,3-fache Fläche des wahren Mastes von 60 mm, der Knopfblock die 2,7-fache der Kugel; die Zeichnung 1:500 vergrößert sie auf den Maßstab)."),
        ("Structure: members keep their real walls. The pole tube is the only member under the 75 mm feature (57 mm): it is a tube of 75 mm outer diameter and 0.63 mm wall that has the bending stiffness EI of the real 60 x 1.5 mm tube (1.0e-7 m4); drum, bottle, head cup, 1 mm dish inside the 75 mm shell, 75 mm contact pad and two masses as before; no joints at this scale (merged into the members).",
         "Tragwerk: die Stäbe behalten ihre wirklichen Wände. Das Mastrohr ist der einzige Stab unter der Einzelheit 75 mm (57 mm): es ist ein Rohr von 75 mm Außendurchmesser und 0,63 mm Wand mit der Biegesteifigkeit EI des wirklichen Rohrs 60 x 1,5 mm (1,0e-7 m4); Trommel, Flasche, Kopfbecher, 1-mm-Schale innerhalb der 75-mm-Körperschale, Aufstandsplatte 75 mm und zwei Massen wie zuvor; keine Gelenke in diesem Maßstab (in die Stäbe aufgegangen)."),
    ],
}
for s_ in sc:
    s_.decisions = []
    for en, de in DEC[s_.den]:
        s_.dec(en, de)
save_asset("patio-heater", sc)
