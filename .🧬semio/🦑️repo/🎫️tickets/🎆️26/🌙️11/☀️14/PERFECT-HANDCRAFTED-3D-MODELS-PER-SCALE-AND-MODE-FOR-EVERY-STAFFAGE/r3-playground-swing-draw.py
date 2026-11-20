"""✏️ Round 3 playground swing 2D: side elevation (end view along the beam), front elevation and plan of every scale redrawn as the true orthographic views of the real swing (ruling E1), per bone, from the same hand-stated geometry as the solids. Elevation shapes are bone-local, front shapes are (world z, y), plan shapes (world x, z) at rest; chains and seat carry extruded `solid` hints."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("swing_kit", os.path.join(T, "r3-playground-swing-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

FG, CS, SR = "frame-galv", "chain-steel", "seat-rubber"
ST, RB = "steel-galvanised", "rubber"
A = math.radians(k.LEG_ANGLE)
TUBE, PLATE, DARK, CHAIN, SEATT = 0.45, 0.3, 0.25, 0.3, 0.2
F = {"region": FG, "material": ST}
C = {"region": CS, "material": ST}
S = {"region": SR, "material": RB}


def E(bone, pts):
    return [k.local(bone, p)[:2] for p in pts]


def box_pts(x0, y0, x1, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def rot_rect(c, d, half_len, half_w):
    n = (-d[1], d[0])
    return [(c[0] + d[0] * sl + n[0] * sw, c[1] + d[1] * sl + n[1] * sw) for sl, sw in ((-half_len, -half_w), (half_len, -half_w), (half_len, half_w), (-half_len, half_w))]


def seat_d(top, bottom, half, r):
    """🪑 Rounded rectangle of the seat section (u, v)."""
    return "M %.5f %.5f L %.5f %.5f Q %.5f %.5f %.5f %.5f L %.5f %.5f Q %.5f %.5f %.5f %.5f L %.5f %.5f Q %.5f %.5f %.5f %.5f L %.5f %.5f Q %.5f %.5f %.5f %.5f Z" % (
        -half + r, top, half - r, top, half, top, half, top - r, half, bottom + r, half, bottom, half - r, bottom, -half + r, bottom, -half, bottom, -half, bottom + r, -half, top - r, -half, top, -half + r, top)


def acorn_d(cx, top, half):
    return "M %.5f %.5f L %.5f %.5f L %.5f %.5f Q %.5f %.5f %.5f %.5f Q %.5f %.5f %.5f %.5f L %.5f %.5f Z" % (
        cx - half, top, cx + half, top, cx + half, top - 0.01, cx + half, top - 0.016, cx, top - 0.016, cx - half, top - 0.016, cx - half, top - 0.01, cx - half, top)


def leg_extent_x(name, s_top):
    xs = [p[0] for p in k.leg_outline_world(name, k.PLATE[1], s_top)]
    return min(xs), max(xs)


def socket_world(name):
    d = k.leg_dir(name)
    c = k.apex_point(name, (k.SOCKET_S[0] + k.SOCKET_S[1]) / 2)
    return rot_rect(c, d, (k.SOCKET_S[1] - k.SOCKET_S[0]) / 2, k.R_SOCKET)


def chain_solid(zc, half):
    return {"kind": "extruded", "sweep": "w", "z0": round(zc - half, 5), "z1": round(zc + half, 5)}


def seat_solid(z, half):
    return {"kind": "extruded", "sweep": "w", "z0": round(z - half, 5), "z1": round(z + half, 5)}


def beam_front(half_tube, half, r):
    return [(-half, -0.03), (-half + 0.002, -0.042), (-half_tube, -r), (half_tube, -r), (half - 0.002, -0.042), (half, -0.03), (half, 0.03), (half - 0.002, 0.042), (half_tube, r), (-half_tube, r), (-half + 0.002, 0.042), (-half, 0.03)]


def seat_shapes(scale, front_or_plan, thick_bottom):
    """🪑 Seat with eye bolts as extruded solids (front and plan) or plain elevation shapes."""
    top = k.SEAT_TOP_V
    out = []
    if front_or_plan:
        out.append(k.path2("seat", "seat", "body", 2.0, True, SEATT, seat_d(top, thick_bottom, k.SEAT[0] / 2, 0.01), penMm=0.35 if scale <= 100 else None, solid=seat_solid(0.0, k.SEAT[2] / 2), **S))
        if out[-1]["penMm"] is None:
            del out[-1]["penMm"]
        if scale == 50:
            for side, sg in k.SIDES:
                z = sg * k.SEAT_EYE_Z
                out.append(k.ellipse2("seat-eye-%s" % side, "seat", "detail", 2.2, True, DARK, 0.0, 0.0, k.SEAT_EYE_RC + k.SEAT_EYE_R, k.SEAT_EYE_RC + k.SEAT_EYE_R, solid=seat_solid(z, k.SEAT_EYE_R), **C))
                out.append(k.rect2("seat-collar-%s" % side, "seat", "detail", 2.1, True, DARK, -0.013, top, 0.026, 0.008, solid=seat_solid(z, 0.013), **C))
                out.append(k.path2("seat-nut-%s" % side, "seat", "detail", 2.1, True, DARK, acorn_d(0.0, top - k.SEAT[1], 0.0105), solid=seat_solid(z, 0.0105), **C))
        return out
    out.append(k.path2("seat-plank", "seat", "body", 1.6, True, SEATT, seat_d(top, thick_bottom, k.SEAT[0] / 2, 0.01), **S))
    if scale <= 100:
        out[-1]["penMm"] = 0.35
    if scale == 50:
        out.append(k.ellipse2("seat-eye", "seat", "detail", 1.7, True, DARK, 0.0, 0.0, k.SEAT_EYE_RC + k.SEAT_EYE_R, k.SEAT_EYE_RC + k.SEAT_EYE_R, **C))
        out.append(k.rect2("seat-collar", "seat", "detail", 1.7, True, DARK, -0.013, top, 0.026, 0.008, **C))
        out.append(k.path2("seat-nut", "seat", "detail", 1.7, True, DARK, acorn_d(0.0, top - k.SEAT[1], 0.0105), **C))
    return out


def elevation(scale):
    out = []
    plate_h = {50: k.PLATE[1], 100: 0.015, 200: 0.04, 500: 0.075}[scale]
    for name, bone, xf in k.LEGS:
        if scale == 500:
            out.append(k.poly2("leg-%s-tube" % name, bone, "body", 2.0, False, TUBE, [(0.0, 0.0), (k.LEG_LEN, 0.0)], closed=False, penMm=0.15, **F))
        else:
            s_top = {50: k.SOCKET_S[1] - 0.01, 100: 0.03, 200: 0.03}[scale]
            out.append(k.poly2("leg-%s-tube" % name, bone, "body", 2.0, True, TUBE, E(bone, k.leg_outline_world(name, plate_h if scale >= 200 else k.PLATE[1], s_top)), penMm=0.35 if scale <= 100 else None, **F))
            if out[-1]["penMm"] is None:
                del out[-1]["penMm"]
        if scale <= 100:
            out.append(k.poly2("foot-%s-plate" % name, bone, "body", 2.1, True, PLATE, E(bone, box_pts(xf - 0.09, 0.0, xf + 0.09, plate_h)), penMm=0.3 if scale == 50 else None, **F))
            if out[-1]["penMm"] is None:
                del out[-1]["penMm"]
            outer, inner = k.ribs_world(name)
            out.append(k.poly2("foot-%s-rib-out" % name, bone, "detail", 2.2, True, PLATE, E(bone, outer), **F))
            out.append(k.poly2("foot-%s-rib-in" % name, bone, "detail", 2.2, True, PLATE, E(bone, inner), **F))
        else:
            top = 0.09 if scale == 200 else 0.09
            hx = k.leg_half_x()
            sgn = 1 if name == "left" else -1
            xc = k.leg_centre_x(name, plate_h)
            out.append(k.poly2("foot-%s-plate" % name, bone, "body", 2.1, True, PLATE, E(bone, [(xf - 0.09, 0.0), (xf + 0.09, 0.0), (xf + 0.09, plate_h * 0.5), (xc + hx, plate_h), (xc - hx, plate_h), (xf - 0.09, plate_h * 0.5)]), **F))
        if scale == 50:
            for i, (x, dz) in enumerate(k.nut_spots(name)):
                out.append(k.poly2("foot-%s-nut-%d" % (name, i + 1), bone, "detail", 2.3, True, DARK, E(bone, box_pts(x - k.NUT_AF / 2, k.PLATE[1], x + k.NUT_AF / 2, k.PLATE[1] + 0.0125)), **F))
                out.append(k.poly2("foot-%s-stud-%d" % (name, i + 1), bone, "detail", 2.31, True, DARK, E(bone, [(x - 0.006, k.PLATE[1] + 0.0125), (x + 0.006, k.PLATE[1] + 0.0125), (x + 0.006, k.PLATE[1] + 0.0155), (x + 0.0042, k.PLATE[1] + 0.0197), (x, k.PLATE[1] + 0.0215), (x - 0.0042, k.PLATE[1] + 0.0197), (x - 0.006, k.PLATE[1] + 0.0155)]), **F))
    if scale == 500:
        out.append(k.poly2("brace-tube", "brace", "body", 2.5, False, TUBE, [(0.0, 0.0), (k.BRACE_LEN, 0.0)], closed=False, penMm=0.15, **F))
    else:
        out.append(k.poly2("brace-tube", "brace", "body", 2.5, True, 0.5, box_pts(0.03, -k.R_BRACE, k.BRACE_LEN - 0.03, k.R_BRACE), penMm=0.3 if scale <= 100 else None, **F))
        if out[-1]["penMm"] is None:
            del out[-1]["penMm"]
    if scale == 50:
        for j, (u, name) in enumerate(((0.0, "left"), (k.BRACE_LEN, "right"))):
            d = k.leg_dir(name)
            toward = 1 if j == 0 else -1
            out.append(k.poly2("brace-clamp-%s" % ("l" if j == 0 else "r"), "brace", "detail", 2.65, True, PLATE, rot_rect((u, 0.0), d, 0.035, 0.038), **F))
            out.append(k.poly2("brace-stub-%s" % ("l" if j == 0 else "r"), "brace", "detail", 2.6, True, PLATE, box_pts(min(u + toward * 0.02, u + toward * 0.068), -0.026, max(u + toward * 0.02, u + toward * 0.068), 0.026), **F))
    if scale <= 200:
        out.append(k.poly2("apex-gusset", "beam", "body", 2.75, True, 0.42, k.gusset_world(), **F))
    if scale == 50:
        for name in ("left", "right"):
            p = socket_world(name)
            out.append(k.poly2("apex-socket-%s" % name, "beam", "body", 2.8, True, 0.4, [(x - k.APEX_X, y - k.H_BEAM) for x, y in p], penMm=0.35, **F))
            m = k.apex_point(name, 0.13)
            d = k.leg_dir(name)
            out.append(k.poly2("apex-bolt-%s" % name, "beam", "detail", 2.85, True, DARK, k.hexagon(k.NUT_AF, m[0] - k.APEX_X, m[1] - k.H_BEAM, math.degrees(math.atan2(d[1], d[0]))), **F))
        out.append(k.ellipse2("apex-sleeve", "beam", "body", 2.9, True, 0.4, 0.0, 0.0, k.R_SLEEVE, k.R_SLEEVE, penMm=0.35, **F))
        out.append(k.poly2("sleeve-bolt", "beam", "detail", 2.95, True, DARK, box_pts(-0.011, 0.0555, 0.011, 0.0635), **F))
        out.append(k.poly2("sleeve-nut", "beam", "detail", 2.95, True, DARK, box_pts(-0.011, -0.0655, 0.011, -0.0555), **F))
    out.append(k.ellipse2("beam-end", "beam", "body", 3.0, True, 0.38, 0.0, 0.0, k.R_BEAM, k.R_BEAM, penMm=0.35 if scale <= 100 else None, **F))
    if out[-1]["penMm"] is None:
        del out[-1]["penMm"]
    if scale == 50:
        out.append(k.ellipse2("beam-cap", "beam", "detail", 3.1, False, 0.3, 0.0, 0.0, 0.042, 0.042, **F))
    out += chains_elevation(scale)
    out += seat_shapes(scale, False, k.SEAT_TOP_V - (k.SEAT[1] if scale < 500 else 0.075))
    return out


def chain_outline_d():
    """⛓️ Side-elevation outline (u, v) of one chain at 1:50: the face-on links as stadiums of 27.2 mm joined by the 8 mm edge-on links, one closed ring (the true view, without the 1.6 mm slots beside the bars that no pen can print)."""
    hw = k.LINK_RC + k.WIRE_R
    s = k.LINK_STRAIGHT / 2
    neck = math.degrees(math.asin(k.WIRE_R / hw))
    side = []
    for i in range(0, k.LINKS, 2):
        c = k.link_u(i)
        start = 180.0 if i == 0 else 180.0 - neck
        for t in [start, 150.0, 120.0, 90.0]:
            side.append((c - s + hw * math.cos(math.radians(t)), hw * math.sin(math.radians(t))))
        for t in [90.0, 60.0, 30.0, neck]:
            side.append((c + s + hw * math.cos(math.radians(t)), hw * math.sin(math.radians(t))))
    end = k.link_u(k.LINKS - 1) + s + k.LINK_RC + k.WIRE_R
    side.append((end, k.WIRE_R))
    pts = side + [(u, -v) for u, v in reversed(side)]
    return k.poly_d(pts)


def chains_elevation(scale):
    out = []
    if scale == 50:
        bone = "chain-right"
        out.append(k.ellipse2("chain-top-eye", bone, "detail", 1.45, True, CHAIN, 0.0, 0.0, 0.021, 0.021, penMm=0.13, **C))
        out.append(k.rect2("chain-hook-shank", bone, "detail", 1.44, True, CHAIN, 0.019, -0.0065, 0.027, 0.013, penMm=0.13, **C))
        out.append(k.rect2("chain-lower-eye", bone, "detail", 1.44, True, CHAIN, 0.0395, -0.0055, 0.037, 0.011, penMm=0.13, **C))
        out.append(k.path2("chain-right-links", bone, "body", 1.5, True, CHAIN, chain_outline_d(), penMm=0.13, **C))
        return out
    if scale == 100:
        for name, bone, zc in k.CHAINS:
            out.append(k.capsule2("%s-link" % bone, bone, "body", 1.2 if name == "left" else 1.5, True, CHAIN, 0.0, 0.0, 1.93, 0.0, 0.009, **C))
        return out
    for name, bone, zc in k.CHAINS:
        out.append(k.poly2("%s-link" % bone, bone, "body", 1.2 if name == "left" else 1.5, False, CHAIN, [(-0.075 if scale == 500 else 0.0, 0.0), (1.93, 0.0)], closed=False, penMm=0.13, **C))
    return out


def front(scale):
    out = []
    plate_h = {50: k.PLATE[1], 100: 0.015, 200: 0.04, 500: 0.075}[scale]
    sock_bottom = k.H_BEAM - k.SOCKET_S[1] * math.sin(A) - k.R_SOCKET * math.cos(A)
    for side, sg in k.SIDES:
        z = sg * k.ZA
        top = sock_bottom + 0.01 if scale == 50 else k.H_BEAM
        if scale == 500:
            out.append(k.poly2("leg-%s" % side, "leg-right", "body", 2.0, False, TUBE, [(z, 0.0), (z, k.H_BEAM)], closed=False, penMm=0.15, **F))
        else:
            out.append(k.rect2("leg-%s" % side, "leg-right", "body", 2.0, True, TUBE, z - k.R_LEG, plate_h if scale >= 200 else k.PLATE[1], 2 * k.R_LEG, top - (plate_h if scale >= 200 else k.PLATE[1]), penMm=0.35 if scale <= 100 else None, **F))
            if out[-1]["penMm"] is None:
                del out[-1]["penMm"]
        out.append(k.rect2("foot-%s-plate" % side, "leg-right", "body", 2.1, True, PLATE, z - k.PLATE[2] / 2, 0.0, k.PLATE[2], plate_h, **F))
        if scale == 50:
            for i, (x, dz) in enumerate(k.nut_spots("right")):
                zz = z + dz
                out.append(k.rect2("foot-%s-nut-%d" % (side, i + 1), "leg-right", "detail", 2.3, True, DARK, zz - 0.0095, k.PLATE[1], 0.019, 0.0125, **F))
                out.append(k.rect2("foot-%s-stud-%d" % (side, i + 1), "leg-right", "detail", 2.31, True, DARK, zz - 0.006, k.PLATE[1] + 0.0125, 0.012, 0.009, r=0.004, **F))
            h = 0.035 * math.sin(A) + 0.038 * math.cos(A)
            out.append(k.rect2("brace-clamp-%s" % side, "brace", "detail", 2.6, True, PLATE, z - 0.038, k.BRACE_Y - h, 0.076, 2 * h, **F))
            out.append(k.rect2("apex-socket-%s" % side, "beam", "body", 2.8, True, 0.4, z - k.R_SOCKET, sock_bottom, 2 * k.R_SOCKET, k.H_BEAM - sock_bottom, penMm=0.35, **F))
            m = k.apex_point("right", 0.13)
            out.append(k.rect2("apex-bolt-%s" % side, "beam", "detail", 2.85, True, DARK, min(z + sg * (k.R_SOCKET - 0.0015), z + sg * (k.R_SOCKET + 0.0065)), m[1] - 0.011, 0.008, 0.022, **F))
            out.append(k.rect2("apex-nut-%s" % side, "beam", "detail", 2.85, True, DARK, min(z - sg * (k.R_SOCKET - 0.0015), z - sg * (k.R_SOCKET + 0.0085)), m[1] - 0.011, 0.01, 0.022, **F))
            out.append(k.rect2("apex-sleeve-%s" % side, "beam", "body", 2.9, True, 0.4, z - k.SLEEVE_HALF, k.H_BEAM - k.R_SLEEVE, 2 * k.SLEEVE_HALF, 2 * k.R_SLEEVE, penMm=0.35, **F))
            out.append(k.rect2("sleeve-bolt-%s" % side, "beam", "detail", 2.95, True, DARK, z - 0.0095, k.H_BEAM + 0.0555, 0.019, 0.008, **F))
            out.append(k.rect2("sleeve-nut-%s" % side, "beam", "detail", 2.95, True, DARK, z - 0.0095, k.H_BEAM - 0.0655, 0.019, 0.01, **F))
    half_tube = k.BEAM_TUBE_HALF if scale <= 100 else k.BEAM_HALF - 0.0005
    out.append(k.poly2("beam", "beam", "body", 3.0, True, 0.38, [(z, y + k.H_BEAM) for z, y in beam_front(half_tube, k.BEAM_HALF, k.R_BEAM)] if scale <= 100 else box_pts(-k.BEAM_HALF, k.H_BEAM - k.R_BEAM, k.BEAM_HALF, k.H_BEAM + k.R_BEAM), penMm=0.35 if scale <= 100 else None, **F))
    if out[-1]["penMm"] is None:
        del out[-1]["penMm"]
    if scale == 50:
        for side, sg in k.SIDES:
            out.append(k.poly2("beam-cap-%s" % side, "beam", "detail", 3.05, False, 0.3, [(sg * k.BEAM_TUBE_HALF, k.H_BEAM - 0.046), (sg * k.BEAM_TUBE_HALF, k.H_BEAM + 0.046)], closed=False, **F))
    for name, bone, zc in k.CHAINS:
        if scale == 50:
            out.append(k.rect2("hanger-band-%s" % name, "beam", "detail", 3.2, True, PLATE, zc - 0.025, k.H_BEAM - 0.0585, 0.05, 0.117, **F))
            out.append(k.rect2("hanger-fork-%s" % name, "beam", "detail", 3.15, True, PLATE, zc - 0.021, k.H_BEAM - 0.11, 0.042, 0.062, **F))
            out.append(k.rect2("hanger-bolt-%s" % name, "beam", "detail", 3.25, True, DARK, zc - 0.03, k.H_BEAM - 0.0735, 0.06, 0.019, **F))
            out.append(k.rect2("hanger-pin-%s" % name, "beam", "detail", 3.25, True, DARK, zc - 0.031, k.H_BEAM - k.PIVOT_DROP - 0.012, 0.062, 0.024, **F))
        elif scale <= 200:
            out.append(k.rect2("hanger-%s" % name, "beam", "detail" if scale == 100 else "body", 3.2, True, PLATE, zc - 0.025, k.H_BEAM - 0.11, 0.05, 0.11 + 0.0585, **F))
    out += chains_front_plan(scale, 2.5)
    out += seat_shapes(scale, True, k.SEAT_TOP_V - (k.SEAT[1] if scale < 500 else 0.075))
    return out


def chains_front_plan(scale, z):
    out = []
    for name, bone, zc in k.CHAINS:
        if scale == 50:
            out.append(k.capsule2(bone, bone, "body", z, True, CHAIN, -0.021, 0.0, 1.8975, 0.0, 0.0136, solid=chain_solid(zc, 0.0136), **C))
        else:
            out.append(k.capsule2(bone, bone, "body", z, True, CHAIN, -0.075 if scale == 500 else 0.0, 0.0, 1.93, 0.0, 0.009, solid=chain_solid(zc, 0.009), **C))
    return out


def plan(scale):
    out = []
    plate_h = {50: k.PLATE[1], 100: 0.015, 200: 0.04, 500: 0.075}[scale]
    for name, bone, xf in k.LEGS:
        x0, x1 = leg_extent_x(name, (k.SOCKET_S[1] - 0.01) if scale == 50 else 0.03)
        for side, sg in k.SIDES:
            z = sg * k.ZA
            if scale == 500:
                out.append(k.poly2("leg-%s-%s" % (name, side), bone, "body", 2.0, False, TUBE, [(xf, z), (k.APEX_X, z)], closed=False, penMm=0.15, **F))
            else:
                out.append(k.rect2("leg-%s-%s" % (name, side), bone, "body", 2.0, True, TUBE, x0, z - k.R_LEG, x1 - x0, 2 * k.R_LEG, penMm=0.35 if scale <= 100 else None, **F))
                if out[-1]["penMm"] is None:
                    del out[-1]["penMm"]
            out.append(k.rect2("foot-%s-%s-plate" % (name, side), bone, "body", 1.9, True, PLATE, xf - 0.09, z - k.PLATE[2] / 2, 0.18, k.PLATE[2], **F))
            if scale == 50:
                outer, inner = k.ribs_world(name)
                for rid, tri in (("out", outer), ("in", inner)):
                    xs = [p[0] for p in tri]
                    out.append(k.rect2("foot-%s-%s-rib-%s" % (name, side, rid), bone, "detail", 1.95, True, PLATE, min(xs), z - 0.004, max(xs) - min(xs), 0.008, **F))
                for i, (x, dz) in enumerate(k.nut_spots(name)):
                    hexa = [(x + p[1], z + dz + p[0]) for p in k.hexagon(k.NUT_AF)]
                    out.append(k.poly2("foot-%s-%s-nut-%d" % (name, side, i + 1), bone, "detail", 2.3, True, DARK, hexa, **F))
                    out.append(k.ellipse2("foot-%s-%s-stud-%d" % (name, side, i + 1), bone, "detail", 2.31, False, DARK, x, z + dz, 0.006, 0.006, **F))
        if scale == 50:
            for side, sg in k.SIDES:
                z = sg * k.ZA
                p = socket_world(name)
                xs = [q[0] for q in p]
                out.append(k.rect2("apex-socket-%s-%s" % (name, side), "beam", "body", 2.8, True, 0.4, min(xs), z - k.R_SOCKET, max(xs) - min(xs), 2 * k.R_SOCKET, penMm=0.35, **F))
                m = k.apex_point(name, 0.13)
                out.append(k.rect2("apex-bolt-%s-%s" % (name, side), "beam", "detail", 2.85, True, DARK, m[0] - 0.011, min(z + sg * (k.R_SOCKET - 0.0015), z + sg * (k.R_SOCKET + 0.0065)), 0.022, 0.008, **F))
                out.append(k.rect2("apex-nut-%s-%s" % (name, side), "beam", "detail", 2.85, True, DARK, m[0] - 0.011, min(z - sg * (k.R_SOCKET - 0.0015), z - sg * (k.R_SOCKET + 0.0085)), 0.022, 0.01, **F))
    if scale == 50:
        for side, sg in k.SIDES:
            z = sg * k.ZA
            for j, (xj, name) in enumerate(((k.BRACE_X0, "left"), (k.FEET_X[1] - k.BRACE_X0, "right"))):
                w = 0.035 * math.cos(A) + 0.038 * math.sin(A)
                out.append(k.rect2("brace-clamp-%s-%d" % (side, j + 1), "brace", "detail", 2.1, True, PLATE, xj - w, z - 0.038, 2 * w, 0.076, **F))
    out.append(k.poly2("beam", "beam", "body", 3.0, True, 0.38, [(k.APEX_X + y, z) for z, y in beam_front(k.BEAM_TUBE_HALF, k.BEAM_HALF, k.R_BEAM)] if scale <= 100 else box_pts(k.APEX_X - k.R_BEAM, -k.BEAM_HALF, k.APEX_X + k.R_BEAM, k.BEAM_HALF), penMm=0.35 if scale <= 100 else None, **F))
    if out[-1]["penMm"] is None:
        del out[-1]["penMm"]
    if scale == 50:
        for side, sg in k.SIDES:
            z = sg * k.ZA
            out.append(k.poly2("beam-cap-%s" % side, "beam", "detail", 3.05, False, 0.3, [(k.APEX_X - 0.046, sg * k.BEAM_TUBE_HALF), (k.APEX_X + 0.046, sg * k.BEAM_TUBE_HALF)], closed=False, **F))
            out.append(k.rect2("apex-sleeve-%s" % side, "beam", "body", 3.1, True, 0.4, k.APEX_X - k.R_SLEEVE, z - k.SLEEVE_HALF, 2 * k.R_SLEEVE, 2 * k.SLEEVE_HALF, penMm=0.35, **F))
            out.append(k.poly2("sleeve-bolt-%s" % side, "beam", "detail", 3.15, True, DARK, [(k.APEX_X + p[1], z + p[0]) for p in k.hexagon(k.NUT_AF)], **F))
        for name, bone, zc in k.CHAINS:
            out.append(k.rect2("hanger-band-%s" % name, "beam", "detail", 3.2, True, PLATE, k.APEX_X - 0.0585, zc - 0.025, 0.117, 0.05, **F))
    out += chains_front_plan(scale, 1.2)
    out += [dict(s, z=1.5 + (s["z"] - 2.0)) for s in seat_shapes(scale, True, k.SEAT_TOP_V - (k.SEAT[1] if scale < 500 else 0.075))]
    return out


def T2(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    50: {
        "silhouette": T2("Side elevation along the beam, 43.6 x 50 mm: one A-frame of two splayed o60.3 legs on base plates with gussets and anchor nuts, the o42.4 tie in its tee clamps, the apex bracket (sleeve, sockets with bolts, gusset) round the beam end, the chain of 76 DIN 766 links (rings and edge-on bars alternating) behind the frame and the rubber seat on its eye bolt. Front: both A-frames edge-on with sockets, sleeves, clamps, plates and nuts, the 1.9 m beam with domed ends, two swing hangers, both chains as link envelopes and the seat with its eye bolts; plan: four leg strips, plates with gussets and nuts, clamps, sockets, sleeves, the beam with its hanger bands, the seat under the beam.",
                         "Seitenansicht entlang des Holms, 43,6 x 50 mm: ein A-Bock aus zwei gespreizten Beinen o60,3 auf Fußplatten mit Rippen und Ankermuttern, das Zugrohr o42,4 in seinen T-Klemmen, der Scheitelbeschlag (Hülse, Muffen mit Schrauben, Knotenblech) um das Holmende, die Kette aus 76 Gliedern DIN 766 (Ringe und hochkant gesehene Stege im Wechsel) hinter dem Gestell und der Gummisitz an seiner Ringschraube. Vorderansicht: beide A-Böcke von der Seite mit Muffen, Hülsen, Klemmen, Platten und Muttern, der 1,9 m lange Holm mit gewölbten Enden, zwei Schaukelgelenke, beide Ketten als Gliederhülle und der Sitz mit seinen Ringschrauben; Draufsicht: vier Beinstreifen, Platten mit Rippen und Muttern, Klemmen, Muffen, Hülsen, der Holm mit den Gelenkbändern, der Sitz unter dem Holm."),
        "retained": T2("Every part the 7.5 mm feature can show: tube walls at true width, plates 12 mm, gussets 8 and 10 mm, nuts and bolt heads 19 mm, every chain link, the eye bolts with collar and acorn nut.", "Jedes Teil, das das Merkmal 7,5 mm zeigen kann: Rohre in wahrer Breite, Platten 12 mm, Rippen 8 und 10 mm, Muttern und Schraubenköpfe 19 mm, jedes Kettenglied, die Ringschrauben mit Bund und Hutmutter."),
        "merged": T2("The far A-frame and the far chain lie exactly behind the near ones in the side elevation and are hidden; the far chain is one envelope strip under the near links. Front and plan draw each chain as the 27 mm envelope of its links.", "Der hintere A-Bock und die hintere Kette liegen in der Seitenansicht genau hinter den vorderen und sind verdeckt; die hintere Kette ist ein Hüllstreifen unter den vorderen Gliedern. Vorder- und Draufsicht zeichnen jede Kette als 27-mm-Hülle ihrer Glieder."),
        "omitted": T2("Grub screws of the tee clamps, the swing hangers in the side elevation (hidden behind the apex gusset), the hanger bolts in plan (under the beam), weld seams.", "Madenschrauben der T-Klemmen, die Schaukelgelenke in der Seitenansicht (hinter dem Knotenblech verdeckt), die Gelenkschrauben in der Draufsicht (unter dem Holm), Schweißnähte."),
        "thinElements": T2("The 8 mm link wire is a 0.16 mm edge-on bar or a ring of 0.16 mm wall; the 8 mm gussets and the 12 mm plates are drawn at true size.", "Der 8-mm-Gliederdraht ist ein 0,16 mm breiter Steg oder ein Ring mit 0,16 mm Wand; die 8-mm-Rippen und die 12-mm-Platten sind in wahrer Größe gezeichnet."),
        "hatching": T2("None; galvanised steel stays plain tone, the rubber seat dark.", "Keine; verzinkter Stahl bleibt einfarbig, der Gummisitz dunkel."),
        "styles": T2("Tubes 0.45, beam 0.38, sockets and sleeves 0.4, plates and clamps 0.3, nuts, bolts and chain 0.25-0.3, seat 0.2; body pen 0.35, detail 0.25.", "Rohre 0,45, Holm 0,38, Muffen und Hülsen 0,4, Platten und Klemmen 0,3, Muttern, Schrauben und Kette 0,25-0,3, Sitz 0,2; Körperstift 0,35, Detail 0,25."),
        "overlays": T2("Legs, plates, tie, clamps, beam, apex brackets, hangers = frame-galv; chain links, hooks, eye bolts = chain-steel; seat = seat-rubber (sun-heated, with the child in contact).", "Beine, Platten, Zugrohr, Klemmen, Holm, Scheitelbeschläge, Gelenke = frame-galv; Kettenglieder, Haken, Ringschrauben = chain-steel; Sitz = seat-rubber (sonnenerwärmt, mit dem Kind im Kontakt)."),
    },
    100: {
        "silhouette": T2("Side elevation 21.8 x 25 mm: the A-frame of two splayed legs on base plates with gussets, the tie, the gusset under the beam end, the chain as a strip behind the frame and the seat plank. Front: legs, plates, the beam with domed ends, two hanger blocks, both chains and the seat; plan: leg strips, plates, the beam, the seat under it.", "Seitenansicht 21,8 x 25 mm: der A-Bock aus zwei gespreizten Beinen auf Fußplatten mit Rippen, das Zugrohr, das Knotenblech unter dem Holmende, die Kette als Streifen hinter dem Gestell und das Sitzbrett. Vorderansicht: Beine, Platten, der Holm mit gewölbten Enden, zwei Gelenkblöcke, beide Ketten und der Sitz; Draufsicht: Beinstreifen, Platten, der Holm, der Sitz darunter."),
        "retained": T2("Legs, plates with gussets, tie, beam, gusset, hangers, chains, seat.", "Beine, Platten mit Rippen, Zugrohr, Holm, Knotenblech, Gelenke, Ketten, Sitz."),
        "merged": T2("Sockets and sleeves (8 and 6 mm wider than leg and beam) merge into the legs and the beam; hanger band, fork, bolt and pin are one block; the links are one 18 mm strip; the plates are 15 mm.", "Muffen und Hülsen (8 und 6 mm breiter als Bein und Holm) gehen in Beine und Holm auf; Band, Gabel, Schraube und Bolzen des Gelenks sind ein Block; die Glieder sind ein 18-mm-Streifen; die Platten sind 15 mm."),
        "omitted": T2("Nuts, bolts, tee clamps, eye bolts and hooks (below the 15 mm feature).", "Muttern, Schrauben, T-Klemmen, Ringschrauben und Haken (unter dem Merkmal 15 mm)."),
        "thinElements": T2("Chain 0.18 mm strip, tie 0.42 mm, legs 0.6 mm.", "Kette 0,18-mm-Streifen, Zugrohr 0,42 mm, Beine 0,6 mm."),
        "hatching": T2("None.", "Keine."),
        "styles": T2("Tubes 0.45, beam 0.38, plates 0.3, chain 0.3, seat 0.2; body pen 0.35.", "Rohre 0,45, Holm 0,38, Platten 0,3, Kette 0,3, Sitz 0,2; Körperstift 0,35."),
        "overlays": T2("Frame parts = frame-galv, chains = chain-steel, seat = seat-rubber.", "Gestellteile = frame-galv, Ketten = chain-steel, Sitz = seat-rubber."),
    },
    200: {
        "silhouette": T2("Side elevation 10.9 x 12.5 mm: the A-frame of two 0.3 mm legs on foot blocks, the tie, the gusset and beam end, the chain as a line and the seat. Front: legs, foot blocks, beam, hanger blocks, chains, seat; plan: legs, foot blocks, beam, seat.", "Seitenansicht 10,9 x 12,5 mm: der A-Bock aus zwei 0,3 mm breiten Beinen auf Fußblöcken, das Zugrohr, Knotenblech und Holmende, die Kette als Linie und der Sitz. Vorderansicht: Beine, Fußblöcke, Holm, Gelenkblöcke, Ketten, Sitz; Draufsicht: Beine, Fußblöcke, Holm, Sitz."),
        "retained": T2("Legs, tie, beam, gusset, hangers, chains, seat, feet.", "Beine, Zugrohr, Holm, Knotenblech, Gelenke, Ketten, Sitz, Füße."),
        "merged": T2("Base plate and gussets of each foot are one 40 mm foot block that tapers into the leg; the beam has square ends.", "Fußplatte und Rippen jedes Fußes sind ein 40-mm-Fußblock, der in das Bein übergeht; der Holm hat gerade Enden."),
        "omitted": T2("Clamps, sockets, sleeves, nuts, bolts, eye bolts, links.", "Klemmen, Muffen, Hülsen, Muttern, Schrauben, Ringschrauben, Glieder."),
        "thinElements": T2("The chain is a 0.13 mm line in the side elevation and a strip in front and plan.", "Die Kette ist in der Seitenansicht eine 0,13-mm-Linie und in Vorder- und Draufsicht ein Streifen."),
        "hatching": T2("None.", "Keine."),
        "styles": T2("Tubes 0.45, beam 0.38, feet 0.3, chain 0.3, seat 0.2; body pen 0.25.", "Rohre 0,45, Holm 0,38, Füße 0,3, Kette 0,3, Sitz 0,2; Körperstift 0,25."),
        "overlays": T2("Frame parts = frame-galv, chains = chain-steel, seat = seat-rubber.", "Gestellteile = frame-galv, Ketten = chain-steel, Sitz = seat-rubber."),
    },
    500: {
        "silhouette": T2("Side elevation 4.4 x 5 mm: an A of two 0.15 mm leg lines on foot blocks with the tie line, the beam end, the chain line and the seat bar. Front: two leg lines on foot blocks, the beam, the chains and the seat; plan: four leg lines, foot blocks, the beam and the seat.", "Seitenansicht 4,4 x 5 mm: ein A aus zwei 0,15-mm-Beinlinien auf Fußblöcken mit der Zugrohrlinie, dem Holmende, der Kettenlinie und dem Sitzbalken. Vorderansicht: zwei Beinlinien auf Fußblöcken, der Holm, die Ketten und der Sitz; Draufsicht: vier Beinlinien, Fußblöcke, Holm und Sitz."),
        "retained": T2("Legs, tie, beam, chains, seat, feet.", "Beine, Zugrohr, Holm, Ketten, Sitz, Füße."),
        "merged": T2("Legs and tie are 0.15 mm lines (their 60 and 42 mm are below the 75 mm feature); each foot is a 75 mm block; the seat is generalised to a 75 mm bar.", "Beine und Zugrohr sind 0,15-mm-Linien (ihre 60 und 42 mm liegen unter dem Merkmal 75 mm); jeder Fuß ist ein 75-mm-Block; der Sitz ist zu einem 75-mm-Balken verallgemeinert."),
        "omitted": T2("Gusset, hangers, clamps and all hardware.", "Knotenblech, Gelenke, Klemmen und alle Beschläge."),
        "thinElements": T2("Legs and tie 0.15 mm, chains 0.13 mm lines.", "Beine und Zugrohr 0,15 mm, Ketten 0,13-mm-Linien."),
        "hatching": T2("None.", "Keine."),
        "styles": T2("Tubes 0.45, beam 0.38, feet 0.3, chain 0.3, seat 0.2; body pen 0.18.", "Rohre 0,45, Holm 0,38, Füße 0,3, Kette 0,3, Sitz 0,2; Körperstift 0,18."),
        "overlays": T2("Frame parts = frame-galv, chains = chain-steel, seat = seat-rubber.", "Gestellteile = frame-galv, Ketten = chain-steel, Sitz = seat-rubber."),
    },
}


def apply(asset):
    for key, rep in asset["representations"].items():
        scale = int(key)
        rep["geometry"] = {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = DECISIONS[scale]
    return asset
