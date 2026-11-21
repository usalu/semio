"""✏️ Round 3 stepladder 2D: side elevation, front elevation and plan of every scale redrawn as the true orthographic views of that scale's body (ruling E1), per bone, from the same hand-stated geometry as the solids. Elevation shapes are bone-local, front shapes are (world z, y), plan shapes (world x, z) at rest. Parts thinner than the printable feature of a scale are strokes of their true width."""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_s = importlib.util.spec_from_file_location("stepladder_kit", os.path.join(T, "r5-stepladder-kit.py"))
k = importlib.util.module_from_spec(_s)
_s.loader.exec_module(k)

F, R = k.FRONT, k.REAR
FR = {"region": "front-rails", "material": "alu-mill"}
TR = {"region": "treads", "material": "alu-mill"}
PL = {"region": "platform", "material": "alu-mill"}
RF = {"region": "rear-frame", "material": "alu-mill"}
SP = {"region": "spreaders", "material": "steel-zinc"}
FT = {"region": "feet", "material": "rubber-black"}
STILE_T, RAIL_T, TREAD_T, DECK_T, STEEL_T, SHOE_T, CAP_T = 0.55, 0.5, 0.6, 0.6, 0.35, 0.12, 0.35
COARSE = {200: (0.030, 0.090, 0.045, 0.045), 500: (0.075, 0.100, 0.075, 0.075)}


def E(bone, pts):
    return [k.local(bone, p)[:2] for p in pts]


def box(x0, y0, x1, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]


def mm(scale, metres):
    """📏 Paper millimetres of a real size at a scale (the pen of a stroke that stands for a part of that thickness)."""
    return round(max(metres * 1000.0 / scale, 0.1), 3)


def stroke(sid, bone, z, pts, pen, binding, role="body", tone=0.3, scale=None):
    """🖊️ An open stroke; with `scale` its two ends are pulled in by half the pen, so that the stroke with its end caps is as long as the part."""
    if scale:
        (x0, y0), (x1, y1) = pts
        length = ((x1 - x0) ** 2 + (y1 - y0) ** 2) ** 0.5
        cut = min(pen * scale / 2000.0, length * 0.45) / length
        pts = [(x0 + (x1 - x0) * cut, y0 + (y1 - y0) * cut), (x1 - (x1 - x0) * cut, y1 - (y1 - y0) * cut)]
    b = {"region": binding["region"]}
    return k.poly2(sid, bone, role, z, False, tone, pts, closed=False, penMm=pen, **b)


def fill(sid, bone, role, z, tone, pts, binding, pen=None):
    return k.poly2(sid, bone, role, z, True, tone, pts, penMm=pen, **binding)


def stile_ends(scale):
    if scale == 50:
        return k.STILE_Y0, k.STILE_Y1
    if scale == 100:
        return 0.03, k.H_TOP - 0.03 * k.COS_F
    return 0.012, k.H_TOP - 0.0115


def rail_ends(scale):
    if scale == 50:
        return 0.028, k.H_HINGE
    if scale == 100:
        return 0.03, k.H_HINGE
    return 0.01, k.H_HINGE - 0.012


def tread_shape(scale, y):
    """👣 (x0, x1, thickness) of a tread at a scale."""
    thick = k.TREAD[1] if scale <= 100 else COARSE[scale][0]
    x1 = k.fx(y - k.TREAD[1] / 2) + k.TREAD_FRONT
    return x1 - k.TREAD[0], x1, thick


def shoe_box(scale, front):
    outline = k.coarse_shoe(front, COARSE[scale][1], COARSE[scale][2])
    return min(p[0] for p in outline), max(p[0] for p in outline), COARSE[scale][2]


def shoe_side(scale, front):
    return k.coarse_shoe(front, COARSE[scale][1], COARSE[scale][2])


def elevation(scale):
    out = []
    pen_rail = {50: 0.35, 100: 0.25, 200: 0.18, 500: 0.13}[scale]
    pen_part = {50: 0.25, 100: 0.18, 200: 0.13, 500: 0.13}[scale]
    for i, y in enumerate(k.TREAD_Y):
        x0, x1, thick = tread_shape(scale, y)
        if scale == 50:
            out.append(fill("tread-%d" % (i + 1), F, "body", 2.5, TREAD_T, E(F, k.tread_profile(y, ribs=False)), TR, pen_part))
        else:
            out.append(stroke("tread-%d" % (i + 1), F, 2.5, E(F, [(x0, y - thick / 2), (x1, y - thick / 2)]), mm(scale, thick), TR, scale=scale))
    yt = k.H_PLATFORM
    if scale <= 100:
        yb = yt - k.DECK[2]
        out.append(fill("platform", F, "body", 2.4, DECK_T, E(F, [(k.DECK_X0, yb), (k.DECK_X1 - 0.006, yb), (k.DECK_X1, yb + 0.006), (k.DECK_X1, yt), (k.DECK_X0, yt)]), PL, pen_part))
    else:
        thick = COARSE[scale][0]
        out.append(stroke("platform", F, 2.4, E(F, [(k.DECK_X0, yt - thick / 2), (k.DECK_X1, yt - thick / 2)]), mm(scale, thick), PL, scale=scale))
    ys = k.SPREADER_Y
    if scale == 50:
        h = k.LINK[0] / 2
        out.append(fill("spreader-front", F, "body", 2.7, STEEL_T, E(F, box(k.KNUCKLE_X - 0.012, ys - h, k.SPREADER_X[0] + 0.012, ys + h)), SP, 0.18))
        out.append(fill("spreader-rear", R, "body", 2.9, STEEL_T, E(R, box(k.SPREADER_X[1] - 0.012, ys - h, k.KNUCKLE_X + 0.012, ys + h)), SP, 0.18))
        c = k.local(F, (k.KNUCKLE_X, ys))
        out.append(k.ellipse2("spreader-knuckle", F, "detail", 3.6, True, 0.2, c[0], c[1], 0.007, 0.007, penMm=0.13, **SP))
        c = k.local(R, (k.SPREADER_X[1], ys))
        out.append(k.ellipse2("spreader-pivot-rear", R, "detail", 3.6, True, 0.2, c[0], c[1], 0.006, 0.006, penMm=0.13, **SP))
    else:
        out.append(stroke("spreader", F, 2.7, E(F, [(k.SPREADER_X[1], ys), (k.SPREADER_X[0], ys)]), mm(scale, k.LINK[0]), SP, scale=scale))
    y0, y1 = rail_ends(scale)
    if scale == 500:
        out.append(stroke("rear-rail", R, 2.8, E(R, [(k.rx(y0), y0), (k.rx(y1), y1)]), mm(scale, k.RAIL[0]), RF, tone=RAIL_T))
    else:
        out.append(fill("rear-rail", R, "body", 2.8, RAIL_T, E(R, k.strip((k.rx(y0), y0), (k.rx(y1), y1), k.RAIL[0] / 2)), RF, pen_rail))
    if scale == 50:
        out.append(k.ellipse2("rail-cap", R, "detail", 2.85, True, CAP_T, 0.0, 0.0, k.RAIL[0] / 2, k.RAIL[0] / 2, penMm=0.18, **RF))
    y0, y1 = stile_ends(scale)
    if scale == 500:
        out.append(stroke("front-rail", F, 3.0, E(F, [(k.fx(y0), y0), (k.fx(y1), y1)]), mm(scale, k.STILE[0]), FR, tone=STILE_T))
    else:
        out.append(fill("front-rail", F, "body", 3.0, STILE_T, E(F, k.strip((k.fx(y0), y0), (k.fx(y1), y1), k.STILE[0] / 2)), FR, pen_rail))
    if scale == 50:
        top = (k.fx(y1), y1)
        a = (top[0] - k.F_DIR[0] * 0.008, top[1] - k.F_DIR[1] * 0.008)
        b = (top[0] + k.F_DIR[0] * 0.012, top[1] + k.F_DIR[1] * 0.012)
        out.append(fill("top-cap", F, "detail", 3.3, CAP_T, E(F, k.strip(a, b, k.STILE[0] / 2 + 0.002)), FR, 0.18))
        c = k.local(R, (k.HX, k.H_HINGE))
        out.append(k.ellipse2("hinge", R, "detail", 3.6, True, 0.2, c[0], c[1], 0.009, 0.009, penMm=0.13, **RF))
    if scale == 100:
        c = k.local(R, (k.HX, k.H_HINGE))
        out.append(k.ellipse2("hinge", R, "detail", 3.6, True, 0.2, c[0], c[1], 0.011, 0.011, **RF))
    if scale <= 100:
        sole = None if scale == 50 else 0.015
        out.append(fill("foot-rear", R, "body", 2.95, SHOE_T, E(R, k.shoe_profile(False, sole=sole)), FT, pen_part))
        out.append(fill("foot-front", F, "body", 3.4, SHOE_T, E(F, k.shoe_profile(True, sole=sole)), FT, pen_part))
    elif scale == 200:
        out.append(fill("foot-rear", R, "body", 2.95, SHOE_T, E(R, shoe_side(scale, False)), FT))
        out.append(fill("foot-front", F, "body", 3.4, SHOE_T, E(F, shoe_side(scale, True)), FT))
    else:
        out.append(fill("foot-rear", R, "body", 2.95, SHOE_T, E(R, shoe_side(scale, False)), FT))
        out.append(fill("foot-front", F, "body", 3.4, SHOE_T, E(F, shoe_side(scale, True)), FT))
    return out


def front(scale):
    out = []
    pen_rail = {50: 0.35, 100: 0.25, 200: 0.18, 500: 0.13}[scale]
    pen_part = {50: 0.25, 100: 0.18, 200: 0.13, 500: 0.13}[scale]
    ry0, ry1 = rail_ends(scale)
    sy0, sy1 = stile_ends(scale)
    sy1 = k.H_TOP - 0.001 if scale <= 100 else sy1
    for side, sg in k.SIDES:
        if scale == 50:
            hw = k.RAIL[1] / 2
            outer = lambda y: max(k.rz(y) + hw, min(k.fz(y), k.rz(y) + hw + 0.004))
            pts = [(sg * (k.rz(ry0) - hw), ry0), (sg * outer(ry0), ry0), (sg * outer(0.83), 0.83), (sg * outer(ry1), ry1), (sg * (k.rz(ry1) - hw), ry1)]
            out.append(fill("rear-rail-" + side, R, "body", 2.0, RAIL_T, pts, RF, pen_rail))
        else:
            out.append(stroke("rear-rail-" + side, R, 2.0, [(sg * k.rz(ry0), ry0), (sg * k.rz(ry1), ry1)], mm(scale, k.RAIL[1]), RF, tone=RAIL_T))
    braces = {50: k.BRACE_Y, 100: k.BRACE_Y, 200: k.BRACE_Y[:1], 500: ()}[scale]
    for i, y in enumerate(braces):
        half = k.rz(y) - k.RAIL[1] / 2
        name = "brace-%d" % (i + 1) if scale <= 100 else "brace"
        if scale <= 100:
            h = (k.BRACE[0] * k.SIN_R + k.BRACE[1] * k.COS_R) / 2
            out.append(fill(name, R, "body", 2.1, RAIL_T, box(-half, y - h, half, y + h), RF, pen_part))
        else:
            out.append(stroke(name, R, 2.1, [(-half, y), (half, y)], mm(scale, COARSE[scale][0]), RF, tone=RAIL_T, scale=scale))
    for side, sg in k.SIDES:
        zc = sg * (k.rz(0.0) if scale <= 100 else k.coarse_shoe_z(False, COARSE[scale][3]))
        if scale <= 100:
            w, h = k.SHOE_R[1], k.SHOE_R[0]
            out.append(fill("foot-rear-" + side, R, "body", 2.2, SHOE_T, box(zc - w, 0.0, zc + w, h), FT, pen_part))
        elif scale == 200:
            w, h = COARSE[scale][3] / 2, COARSE[scale][2]
            out.append(fill("foot-rear-" + side, R, "body", 2.2, SHOE_T, box(zc - w, 0.0, zc + w, h), FT))
        else:
            w, h = COARSE[scale][3] / 2, COARSE[scale][2]
            out.append(fill("foot-rear-" + side, R, "body", 2.2, SHOE_T, box(zc - w, 0.0, zc + w, h), FT))
    for side, sg in k.SIDES:
        if scale <= 100:
            hw = k.STILE[1] / 2
            pts = [(sg * (k.fz(sy0) - hw), sy0), (sg * (k.fz(sy0) + hw), sy0), (sg * (k.fz(sy1) + hw), sy1), (sg * (k.fz(sy1) - hw), sy1)]
            out.append(fill("front-rail-" + side, F, "body", 3.0, STILE_T, pts, FR, pen_rail))
        else:
            out.append(stroke("front-rail-" + side, F, 3.0, [(sg * k.fz(sy0), sy0), (sg * k.fz(sy1), sy1)], mm(scale, k.STILE[1]), FR, tone=STILE_T))
        if scale == 50:
            zc = sg * k.fz(k.H_TOP - 0.02)
            out.append(fill("top-cap-" + side, F, "detail", 3.35, CAP_T, box(zc - 0.013, k.H_TOP - 0.042, zc + 0.013, k.H_TOP), FR, 0.18))
            zs = sg * (k.LINK_ZF - k.LINK[1] / 2 + 0.002)
            out.append(stroke("spreader-" + side, F, 2.9, [(zs, k.SPREADER_Y - 0.01), (zs, k.SPREADER_Y + 0.01)], 0.16, SP, tone=STEEL_T))
            zh = sg * (k.fz(k.H_HINGE) + k.STILE[1] / 2 + 0.004)
            out.append(stroke("hinge-" + side, R, 3.6, [(zh, k.H_HINGE - 0.009), (zh, k.H_HINGE + 0.009)], 0.16, RF, role="detail", tone=0.2))
            z0, z1 = sorted((sg * k.PLATFORM_HALF, sg * (k.fz(k.AXLE_Y) - k.STILE[1] / 2)))
            out.append(fill("platform-axle-" + side, F, "detail", 3.25, STEEL_T, box(z0, k.AXLE_Y - 0.009, z1, k.AXLE_Y + 0.009), {"region": "platform", "material": "steel-zinc"}, 0.13))
    for i, y in enumerate(k.TREAD_Y):
        _, _, thick = tread_shape(scale, y)
        if scale <= 100:
            half = k.fz(y - thick / 2) - k.STILE[1] / 2
            out.append(fill("tread-%d" % (i + 1), F, "body", 3.1, TREAD_T, box(-half, y - thick, half, y), TR, pen_part))
        else:
            half = k.fz(y - thick / 2)
            out.append(stroke("tread-%d" % (i + 1), F, 3.1, [(-half, y - thick / 2), (half, y - thick / 2)], mm(scale, thick), TR, scale=scale))
    yt = k.H_PLATFORM
    if scale <= 100:
        out.append(fill("platform", F, "body", 3.3, DECK_T, box(-k.PLATFORM_HALF, yt - k.DECK[2], k.PLATFORM_HALF, yt), PL, pen_part))
        half = k.fz(k.BAR_Y) - k.STILE[1] / 2
        out.append(fill("grab-bar", F, "body", 3.1, RAIL_T, box(-half, k.BAR_Y - k.BAR_R, half, k.BAR_Y + k.BAR_R), FR, pen_part))
    else:
        thick = COARSE[scale][0]
        out.append(stroke("platform", F, 3.3, [(-k.PLATFORM_HALF, yt - thick / 2), (k.PLATFORM_HALF, yt - thick / 2)], mm(scale, thick), PL, scale=scale))
        yc = k.H_TOP - thick / 2 - 0.02
        out.append(stroke("grab-bar", F, 3.1, [(-k.fz(yc), yc), (k.fz(yc), yc)], mm(scale, thick), FR, tone=RAIL_T, scale=scale))
    for side, sg in k.SIDES:
        zc = sg * (k.fz(0.0) if scale <= 100 else k.coarse_shoe_z(True, COARSE[scale][3]))
        if scale <= 100:
            w, h = k.SHOE_F[1], k.SHOE_F[0]
            out.append(fill("foot-front-" + side, F, "body", 3.4, SHOE_T, box(zc - w, 0.0, zc + w, h), FT, pen_part))
        elif scale == 200:
            w, h = COARSE[scale][3] / 2, COARSE[scale][2]
            out.append(fill("foot-front-" + side, F, "body", 3.4, SHOE_T, box(zc - w, 0.0, zc + w, h), FT))
        else:
            w, h = COARSE[scale][3] / 2, COARSE[scale][2]
            out.append(fill("foot-front-" + side, F, "body", 3.4, SHOE_T, box(zc - w, 0.0, zc + w, h), FT))
    return out


def plan(scale):
    out = []
    pen_rail = {50: 0.35, 100: 0.25, 200: 0.18, 500: 0.13}[scale]
    pen_part = {50: 0.25, 100: 0.18, 200: 0.13, 500: 0.13}[scale]
    for side, sg in k.SIDES:
        for name, bone, front_, z in (("rear", R, False, 1.0), ("front", F, True, 1.1)):
            zc = sg * ((k.fz(0.0) if front_ else k.rz(0.0)) if scale <= 100 else k.coarse_shoe_z(front_, COARSE[scale][3]))
            if scale <= 100:
                w = (k.SHOE_F if front_ else k.SHOE_R)[1]
                x0, x1 = (k.XF - 0.043, k.XF + 0.045) if front_ else (k.XR - 0.040, k.XR + 0.036)
                out.append(fill("foot-%s-%s" % (name, side), bone, "body", z, SHOE_T, box(x0, zc - w, x1, zc + w), FT, pen_part))
            else:
                x0, x1, _ = shoe_box(scale, front_)
                w = COARSE[scale][3] / 2
                out.append(fill("foot-%s-%s" % (name, side), bone, "body", z, SHOE_T, box(x0, zc - w, x1, zc + w), FT))
    ry0, ry1 = rail_ends(scale)
    for side, sg in k.SIDES:
        if scale == 50:
            hw = k.RAIL[1] / 2
            pts = [(k.rx(ry0), sg * (k.rz(ry0) - hw)), (k.rx(ry0), sg * (k.rz(ry0) + hw)), (k.rx(ry1), sg * (k.rz(ry1) + hw)), (k.rx(ry1), sg * (k.rz(ry1) - hw))]
            out.append(fill("rear-rail-" + side, R, "body", 2.0, RAIL_T, pts, RF, pen_rail))
        else:
            out.append(stroke("rear-rail-" + side, R, 2.0, [(k.rx(ry0), sg * k.rz(ry0)), (k.rx(ry1), sg * k.rz(ry1))], mm(scale, k.RAIL[1]), RF, tone=RAIL_T))
    braces = {50: k.BRACE_Y, 100: k.BRACE_Y, 200: k.BRACE_Y[:1], 500: ()}[scale]
    for i, y in enumerate(braces):
        half = k.rz(y) - k.RAIL[1] / 2
        name = "brace-%d" % (i + 1) if scale <= 100 else "brace"
        wide = k.BRACE[1] * k.SIN_R + k.BRACE[0] * k.COS_R if scale <= 100 else COARSE[scale][0]
        if scale == 50:
            out.append(fill(name, R, "body", 2.1, RAIL_T, box(k.rx(y) - wide / 2, -half, k.rx(y) + wide / 2, half), RF, pen_part))
        else:
            out.append(stroke(name, R, 2.1, [(k.rx(y), -half), (k.rx(y), half)], mm(scale, wide), RF, tone=RAIL_T, scale=scale))
    for side, sg in k.SIDES:
        if scale == 50:
            out.append(stroke("spreader-front-" + side, F, 2.2, [(k.KNUCKLE_X - 0.012, sg * k.LINK_ZF), (k.SPREADER_X[0] + 0.012, sg * k.LINK_ZF)], 0.16, SP, tone=STEEL_T))
            out.append(stroke("spreader-rear-" + side, R, 2.25, [(k.SPREADER_X[1] - 0.012, sg * k.LINK_ZR), (k.KNUCKLE_X + 0.012, sg * k.LINK_ZR)], 0.16, SP, tone=STEEL_T))
        else:
            zc = sg * (k.LINK_ZF + k.LINK_ZR) / 2
            out.append(stroke("spreader-" + side, F, 2.2, [(k.SPREADER_X[1], zc), (k.SPREADER_X[0], zc)], mm(scale, k.LINK[1]), SP, tone=STEEL_T, scale=scale))
    step = k.PITCH / k.TAN_F
    for i, y in enumerate(k.TREAD_Y):
        x0, x1, thick = tread_shape(scale, y)
        half = k.fz(y - thick / 2) - k.STILE[1] / 2
        if scale >= 100 and i < k.TREADS - 1:
            x0 = x1 - step
        out.append(fill("tread-%d" % (i + 1), F, "body", 3.0 + 0.01 * i, TREAD_T, box(x0, -half, x1, half), TR, pen_part if scale <= 100 else None))
        if scale == 50:
            for j in range(3):
                x = x1 - 0.02 - 0.02 * j
                out.append(k.poly2("tread-%d-rib-%d" % (i + 1, j + 1), F, "texture", 3.2, False, 0.3, [(x, -half + 0.012), (x, half - 0.012)], closed=False, region="treads"))
    out.append(fill("platform", F, "body", 3.5, DECK_T, box(k.DECK_X0, -k.PLATFORM_HALF, k.DECK_X1, k.PLATFORM_HALF), PL, pen_part if scale <= 100 else None))
    if scale == 50:
        for j in range(8):
            x = k.DECK_X0 + 0.027 + 0.028 * j
            out.append(k.poly2("platform-rib-%d" % (j + 1), F, "texture", 3.6, False, 0.3, [(x, -k.PLATFORM_HALF + 0.015), (x, k.PLATFORM_HALF - 0.015)], closed=False, region="platform"))
    sy0, sy1 = stile_ends(scale)
    for side, sg in k.SIDES:
        if scale <= 100:
            hw = k.STILE[1] / 2
            e = k.STILE[0] / 2 * k.SIN_F
            pts = [(k.fx(sy0) + e, sg * (k.fz(sy0) - hw)), (k.fx(sy0) + e, sg * (k.fz(sy0) + hw)), (k.fx(sy1) - e, sg * (k.fz(sy1) + hw)), (k.fx(sy1) - e, sg * (k.fz(sy1) - hw))]
            out.append(fill("front-rail-" + side, F, "body", 4.0, STILE_T, pts, FR, pen_rail))
        else:
            out.append(stroke("front-rail-" + side, F, 4.0, [(k.fx(sy0), sg * k.fz(sy0)), (k.fx(sy1), sg * k.fz(sy1))], mm(scale, k.STILE[1]), FR, tone=STILE_T))
    if scale <= 100:
        half = k.fz(k.BAR_Y) - k.STILE[1] / 2
        out.append(fill("grab-bar", F, "body", 4.1, RAIL_T, box(k.fx(k.BAR_Y) - k.BAR_R, -half, k.fx(k.BAR_Y) + k.BAR_R, half), FR, pen_part))
    else:
        thick = COARSE[scale][0]
        yc = k.H_TOP - thick / 2 - 0.02
        out.append(stroke("grab-bar", F, 4.1, [(k.fx(yc), -k.fz(yc)), (k.fx(yc), k.fz(yc))], mm(scale, thick), FR, tone=RAIL_T, scale=scale))
    return out


def T2(en, de):
    return {"en": en, "de": de}


DECISIONS = {
    50: {
        "silhouette": T2("Side elevation 20 mm wide and 34 mm high: the A of the 60 mm stile at 67.5 degrees and the 45 mm rear rail at 74 degrees hinged 0.27 m above the platform, the noses of four treads in front of the stile, the platform between the frames, the two-link spreader at 0.53 m and the shoes. Front view: the two stiles converging upwards with end caps, the grab bar, four treads and the platform between them, the axle sleeves, and behind them the rear rails with two braces and their shoes. Plan: the stiles running back over four grooved treads and the ribbed platform, the rear rails with two braces, the spreader links and the four shoes.",
                         "Seitenansicht 20 mm breit und 34 mm hoch: das A aus dem 60-mm-Holm unter 67,5 Grad und dem 45-mm-Hinterholm unter 74 Grad, 0,27 m über der Plattform angelenkt, die Nasen der vier Stufen vor dem Holm, die Plattform zwischen den Rahmen, die zweigliedrige Spreize auf 0,53 m und die Schuhe. Vorderansicht: die zwei nach oben zusammenlaufenden Holme mit Endkappen, der Haltebügel, vier Stufen und die Plattform dazwischen, die Achshülsen und dahinter die Hinterholme mit zwei Traversen und ihren Schuhen. Draufsicht: die Holme laufen über vier gerillte Stufen und die geriffelte Plattform zurück, die Hinterholme mit zwei Traversen, die Spreizenglieder und die vier Schuhe."),
        "retained": T2("Every part of the 1:50 body at its true size and place: stiles, caps, grab bar, treads, platform, axle sleeves, rear rails with caps, braces, hinge bolt heads, spreader links with knuckle and pivot heads, shoes.",
                       "Jedes Teil des Körpers 1:50 in wahrer Größe und Lage: Holme, Kappen, Haltebügel, Stufen, Plattform, Achshülsen, Hinterholme mit Kappen, Traversen, Gelenkbolzenköpfe, Spreizenglieder mit Knie- und Drehnietköpfen, Schuhe."),
        "merged": T2("The near and the far frame coincide in the side elevation; the platform rims and cheeks are one outline; in the front view and the plan the 1.8 mm play between rear rail and stile is closed.",
                     "Naher und ferner Rahmen fallen in der Seitenansicht zusammen; Ränder und Wangen der Plattform sind ein Umriss; in Vorderansicht und Draufsicht ist das Spiel von 1,8 mm zwischen Hinterholm und Holm geschlossen."),
        "omitted": T2("Parts hidden in a view are not drawn there: grab bar, axle, rest tube and braces in the side elevation, the rest tube in the front view and the plan. The 10 mm rivet heads of the treads, the grooves of the treads in the elevations and labels are below the print profile.",
                      "In einer Ansicht verdeckte Teile sind dort nicht gezeichnet: Haltebügel, Achse, Auflagerohr und Traversen in der Seitenansicht, das Auflagerohr in Vorderansicht und Draufsicht. Die 10-mm-Nietköpfe der Stufen, die Rillen der Stufen in den Ansichten und Beschriftungen liegen unter dem Druckprofil."),
        "thinElements": T2("The 8 mm spreader links are 0.16 mm strokes in the plan and in the front view; the hinge bolt heads are 0.16 mm strokes in the front view; tread grooves and platform ribs are 0.13 mm lines at a spacing of 0.4 and 0.56 mm.",
                           "Die 8 mm starken Spreizenglieder sind in Draufsicht und Vorderansicht 0,16-mm-Striche; die Gelenkbolzenköpfe sind in der Vorderansicht 0,16-mm-Striche; Stufenrillen und Plattformrippen sind 0,13-mm-Linien im Abstand von 0,4 und 0,56 mm."),
        "hatching": T2("None; grooves and ribs are parallel texture lines.", "Keine; Rillen und Rippen sind parallele Texturlinien."),
        "styles": T2("Aluminium 0.5 to 0.6, steel 0.35, caps 0.35, rubber 0.12; stiles and rails 0.35 mm, treads, platform and shoes 0.25 mm, steel parts and caps 0.18 mm, texture 0.13 mm.",
                     "Aluminium 0,5 bis 0,6, Stahl 0,35, Kappen 0,35, Gummi 0,12; Holme und Hinterholme 0,35 mm, Stufen, Plattform und Schuhe 0,25 mm, Stahlteile und Kappen 0,18 mm, Textur 0,13 mm."),
        "overlays": T2("Stiles, caps and grab bar carry front-rails; treads carry treads; platform and axle sleeves carry platform; rear rails, caps, braces and hinge carry rear-frame; links and rivets carry spreaders; shoes carry feet.",
                       "Holme, Kappen und Haltebügel tragen front-rails; Stufen tragen treads; Plattform und Achshülsen tragen platform; Hinterholme, Kappen, Traversen und Gelenk tragen rear-frame; Glieder und Nieten tragen spreaders; Schuhe tragen feet."),
    },
    100: {
        "silhouette": T2("Side elevation 10 mm wide and 17 mm high: stile and rear rail as bands of true depth, the tread noses as 0.25 mm strokes, the platform slab, the spreader as one 0.2 mm stroke, the hinge boss and the shoes. Front view: two stiles, grab bar, four treads, the platform, the rear rails as 0.2 mm strokes with two braces and the shoes. Plan: stiles over four treads and the platform, rear rails, braces and spreaders as strokes, four shoes.",
                         "Seitenansicht 10 mm breit und 17 mm hoch: Holm und Hinterholm als Bänder in wahrer Tiefe, die Stufennasen als 0,25-mm-Striche, die Plattformplatte, die Spreize als ein 0,2-mm-Strich, der Gelenkzapfen und die Schuhe. Vorderansicht: zwei Holme, Haltebügel, vier Stufen, die Plattform, die Hinterholme als 0,2-mm-Striche mit zwei Traversen und die Schuhe. Draufsicht: Holme über vier Stufen und der Plattform, Hinterholme, Traversen und Spreizen als Striche, vier Schuhe."),
        "retained": T2("Stiles, grab bar, treads, platform, rear rails, braces, hinge boss, spreaders and shoes at their true sizes.", "Holme, Haltebügel, Stufen, Plattform, Hinterholme, Traversen, Gelenkzapfen, Spreizen und Schuhe in wahrer Größe."),
        "merged": T2("Caps merge into the stiles and rails, the two links of a spreader into one strap, rims, cheeks and axle into the platform slab; in the plan each tread is drawn up to the next one (the 8 mm gap is not printable).",
                     "Kappen gehen in Holmen und Hinterholmen auf, die zwei Glieder einer Spreize in einem Band, Ränder, Wangen und Achse in der Plattformplatte; in der Draufsicht ist jede Stufe bis zur nächsten gezeichnet (die 8-mm-Fuge ist nicht druckbar)."),
        "omitted": T2("Rivets, grooves, ribs, axle sleeves; parts hidden in a view (grab bar, rest tube and braces in the side elevation; rest tube, hinge and spreaders in the front view).",
                      "Nieten, Rillen, Rippen, Achshülsen; in einer Ansicht verdeckte Teile (Haltebügel, Auflagerohr und Traversen in der Seitenansicht; Auflagerohr, Gelenk und Spreizen in der Vorderansicht)."),
        "thinElements": T2("Treads in the side elevation are 0.25 mm strokes (25 mm), the spreader a 0.2 mm stroke (20 mm) and in the plan a 0.1 mm stroke; the 20 mm rear rails are 0.2 mm strokes in front view and plan, the braces 0.22 mm strokes in the plan.",
                           "Stufen in der Seitenansicht sind 0,25-mm-Striche (25 mm), die Spreize ein 0,2-mm-Strich (20 mm) und in der Draufsicht ein 0,1-mm-Strich; die 20 mm breiten Hinterholme sind in Vorderansicht und Draufsicht 0,2-mm-Striche, die Traversen in der Draufsicht 0,22-mm-Striche."),
        "hatching": T2("None.", "Keine."),
        "styles": T2("Aluminium 0.5 to 0.6, steel 0.35, rubber 0.12; stiles and rails 0.25 mm, other parts 0.18 mm.", "Aluminium 0,5 bis 0,6, Stahl 0,35, Gummi 0,12; Holme und Hinterholme 0,25 mm, übrige Teile 0,18 mm."),
        "overlays": T2("Every region is drawn in the side elevation and the plan; the front view omits the hidden spreaders.", "Jede Region ist in Seitenansicht und Draufsicht gezeichnet; die Vorderansicht lässt die verdeckten Spreizen weg."),
    },
    200: {
        "silhouette": T2("Side elevation 5 mm wide and 8.4 mm high: stile and rear rail as bands of true depth, the platform and the tread noses as 0.15 mm strokes, the spreader as a 0.1 mm stroke and two shoe blocks. Front view: stiles and rear rails as strokes of their true width, four tread strokes, platform, grab bar and one brace as 0.15 mm strokes, four shoe blocks. Plan: rails as strokes over the four treads drawn as one stair of abutting slabs, the platform, the brace and spreader strokes and four shoes.",
                         "Seitenansicht 5 mm breit und 8,4 mm hoch: Holm und Hinterholm als Bänder in wahrer Tiefe, Plattform und Stufennasen als 0,15-mm-Striche, die Spreize als 0,1-mm-Strich und zwei Schuhblöcke. Vorderansicht: Holme und Hinterholme als Striche in wahrer Breite, vier Stufenstriche, Plattform, Haltebügel und eine Traverse als 0,15-mm-Striche, vier Schuhblöcke. Draufsicht: Holme als Striche über den vier Stufen, die als Treppe aneinanderstoßender Platten gezeichnet sind, die Plattform, Traversen- und Spreizenstriche und vier Schuhe."),
        "retained": T2("Both frames at their true inclination and size, four treads and the platform at their true heights, grab bar, one brace, spreaders, four shoes.", "Beide Rahmen in wahrer Neigung und Größe, vier Stufen und die Plattform in wahrer Höhe, Haltebügel, eine Traverse, Spreizen, vier Schuhe."),
        "merged": T2("Caps into the rails, spreader links into one strap, rest tube and axle into the platform.", "Kappen in den Holmen, Spreizenglieder in einem Band, Auflagerohr und Achse in der Plattform."),
        "omitted": T2("Hinge, upper brace, rivets, grooves and ribs.", "Gelenk, obere Traverse, Nieten, Rillen und Rippen."),
        "thinElements": T2("Treads, platform, grab bar and brace are thickened from 25 to 28 mm to 30 mm (0.15 mm strokes); stiles (22 mm) and rear rails (20 mm) are 0.11 and 0.1 mm strokes in front view and plan, the spreader a 0.1 mm stroke.",
                           "Stufen, Plattform, Haltebügel und Traverse sind von 25 bis 28 mm auf 30 mm verdickt (0,15-mm-Striche); Holme (22 mm) und Hinterholme (20 mm) sind in Vorderansicht und Draufsicht 0,11- und 0,1-mm-Striche, die Spreize ein 0,1-mm-Strich."),
        "hatching": T2("None.", "Keine."),
        "styles": T2("Aluminium 0.5 to 0.6, rubber 0.12; bands with a 0.18 mm outline.", "Aluminium 0,5 bis 0,6, Gummi 0,12; Bänder mit 0,18-mm-Kontur."),
        "overlays": T2("Every region is drawn in the side elevation and the plan; the front view omits the spreaders behind the stiles.", "Jede Region ist in Seitenansicht und Draufsicht gezeichnet; die Vorderansicht lässt die Spreizen hinter den Holmen weg."),
    },
    500: {
        "silhouette": T2("Side elevation 2 mm wide and 3.4 mm high: an A of two strokes (stile 0.12 mm, rear rail 0.1 mm) with the platform stroke between them, the spreader stroke and two foot blocks. Front view: a narrow ladder of two converging strokes with four tread strokes, the platform and the grab bar. Plan: two stile strokes over the stair of treads and the platform, two rear rail strokes and four foot blocks.",
                         "Seitenansicht 2 mm breit und 3,4 mm hoch: ein A aus zwei Strichen (Holm 0,12 mm, Hinterholm 0,1 mm) mit dem Plattformstrich dazwischen, dem Spreizenstrich und zwei Fußblöcken. Vorderansicht: eine schmale Leiter aus zwei zusammenlaufenden Strichen mit vier Stufenstrichen, Plattform und Haltebügel. Draufsicht: zwei Holmstriche über der Treppe der Stufen und der Plattform, zwei Hinterholmstriche und vier Fußblöcke."),
        "retained": T2("Both frames at their true inclination, four treads and the platform at their true heights, grab bar, spreaders, four feet, the 1.02 m footprint.", "Beide Rahmen in wahrer Neigung, vier Stufen und die Plattform in wahrer Höhe, Haltebügel, Spreizen, vier Füße, die Standlänge 1,02 m."),
        "merged": T2("The near and far frame coincide; in the front view the rear rails merge into the stile strokes.", "Naher und ferner Rahmen fallen zusammen; in der Vorderansicht gehen die Hinterholme in den Holmstrichen auf."),
        "omitted": T2("Hinge, braces, rest tube, caps, rivets, grooves.", "Gelenk, Traversen, Auflagerohr, Kappen, Nieten, Rillen."),
        "thinElements": T2("Treads, platform and grab bar are thickened to 75 mm (0.15 mm strokes), the shoes to 100 x 75 mm blocks leaning with their stile or rail; stiles and rear rails are strokes of 0.12 mm in the side elevation and 0.1 mm otherwise, the spreader a 0.1 mm stroke: pen inflation up to 6 times.",
                           "Stufen, Plattform und Haltebügel sind auf 75 mm verdickt (0,15-mm-Striche), die Schuhe zu Blöcken 100 x 75 mm, geneigt mit ihrem Holm; Holme und Hinterholme sind Striche von 0,12 mm in der Seitenansicht und sonst 0,1 mm, die Spreize ein 0,1-mm-Strich: Stiftaufblähung bis 6-fach."),
        "hatching": T2("None.", "Keine."),
        "styles": T2("Strokes in the tone of their part; treads and platform filled 0.6 in the plan.", "Striche im Ton ihres Teils; Stufen und Plattform in der Draufsicht mit 0,6 gefüllt."),
        "overlays": T2("Every region is drawn in the side elevation and the plan; the front view omits the spreaders behind the stiles.", "Jede Region ist in Seitenansicht und Draufsicht gezeichnet; die Vorderansicht lässt die Spreizen hinter den Holmen weg."),
    },
}


def apply(asset):
    for key, rep in asset["representations"].items():
        scale = int(key)
        rep["geometry"] = {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
        rep["decisions"] = DECISIONS[scale]
    return asset
