"""✏️ The drawings of the round 3 delivery van: side elevation, front elevation and plan of every scale, drawn shape by shape from the same master lines as the solids (ruling E1), in the frames of the product (elevation: bone-local x along the bone, y across; front: lateral, height; plan: ahead, lateral)."""
import math

from r3_delivery_van_kit import LEAN, RA, ROOF, RT, SLIDER, W, XF, XR, X_B, X_BULK, X_HINGE, X_QUARTER, YA, Y_BELT, Y_CANT, Z_LEAF, pillar_x, sweep

MEMBERS = ["representations"]
K = 0.5523
BODY = dict(material="paint")
SHELL = ["body-panels", "engine-bay", "exhaust-system"]


def n(v):
    return round(v, 3)


def pt(x, y):
    return f"{n(x)},{n(y)}"


def line(points, close=False):
    return "M " + " L ".join(pt(*p) for p in points) + (" Z" if close else "")


def arch_d(xc, radius, foot):
    """⭕ Path piece over one wheel arch from its front foot to its rear foot (the outline runs clockwise from the nose back along the sill)."""
    a, b, t = xc - radius, xc + radius, YA + radius
    return f"L {pt(b, foot)} L {pt(b, YA)} C {pt(b, YA + K * radius)} {pt(xc + K * radius, t)} {pt(xc, t)} C {pt(xc - K * radius, t)} {pt(a, YA + K * radius)} {pt(a, YA)} L {pt(a, foot)}"


def shape(id, bone, role, z, filled, tone, kind, region=None, material=None, **geometry):
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    if isinstance(region, list):
        out["regions"] = region
    elif region:
        out["region"] = region
    if material:
        out["material"] = material
    out.update(geometry)
    return out


def path(id, bone, role, z, tone, d, region=None, material=None, **more):
    return shape(id, bone, role, z, True, tone, "path", region, material, d=d, **more)


def poly(id, bone, role, z, tone, points, region=None, material=None, **more):
    return shape(id, bone, role, z, True, tone, "poly", region, material, points=[[n(x), n(y)] for x, y in points], closed=True, **more)


def stroke(id, bone, role, z, tone, points, pen, **more):
    return shape(id, bone, role, z, False, tone, "poly", None, None, points=[[n(x), n(y)] for x, y in points], closed=False, penMm=pen, **more)


def ellipse(id, bone, role, z, tone, cx, cy, rx, ry, region=None, material=None, **more):
    return shape(id, bone, role, z, True, tone, "ellipse", region, material, cx=n(cx), cy=n(cy), rx=n(rx), ry=n(ry), **more)


def capsule(id, bone, role, z, tone, p0, p1, r, region=None, material=None):
    return shape(id, bone, role, z, True, tone, "capsule", region, material, x0=n(p0[0]), y0=n(p0[1]), x1=n(p1[0]), y1=n(p1[1]), r0=r, r1=r)


def rect(id, bone, role, z, tone, x, y, w, h, r=None, region=None, material=None):
    out = shape(id, bone, role, z, True, tone, "rect", region, material, x=n(x), y=n(y), w=n(w), h=n(h))
    if r:
        out["r"] = r
    return out


def mirrored(points):
    return points + [(-x, y) for x, y in reversed(points) if x != 0]


def roof_y(z):
    """🏠 Height of the roof skin over the lateral station z in the cargo part."""
    return 1.90 + 0.09 * (1 - min(1.0, abs(z) / 0.905) ** 5) ** 0.2


def leaves(scale, tone=0.6):
    """🚪 Both rear leaves as their real section in the bone frame (x up the hinge, y back from it), swept across their thickness."""
    t = 0.08 if scale == 500 else 0.045
    out = []
    for side, s, z in (("r", -1, 1.5), ("l", 1, 1.45)):
        span = sorted((s * Z_LEAF, s * (Z_LEAF - t)))
        out.append(poly(f"rear-leaf-{side}", f"rear-door-{side}", "body", z, tone, [(0, 0), (1.35, 0), (1.35, Z_LEAF), (0, Z_LEAF)], "rear-doors", "paint", solid={"kind": "extruded", "sweep": "w", "z0": n(span[0]), "z1": n(span[1])}))
    return out


def shell_elevation(scale):
    """🚐 The side outline of everything on the chassis bone, clockwise from the rear bumper over the roof and the nose and back along the sill with both arches."""
    grow = {50: 0.007, 100: 0.015}.get(scale, 0.0)
    d = "M -2.67,0.37 L -2.67,0.51 L -2.645,0.54 L -2.625,0.54 L -2.625,1.92 L -2.64,1.925 L -2.64,1.965 L -2.61,1.985 L -2.49,1.99 L 0.35,1.99 L 0.6,1.988 L 0.85,1.975 L 1,1.95 L 1.08,1.92 L 1.12,1.885 L 1.125,1.83 L 1.745,1.205 L 1.8,1.19 L 2.2,1.105 L 2.44,1.045 L 2.55,1 L 2.615,0.93 L 2.65,0.84 L 2.665,0.74 "
    d += f"L {pt(2.67 + grow, 0.62)} L {pt(2.67 + grow, 0.42)} L {pt(2.66 + grow, 0.355)} L {pt(2.63 + grow, 0.295)} L {pt(2.55, 0.26 - grow / 2)} L {pt(2.2, 0.27 - grow / 2)} "
    d += arch_d(XF, RA, 0.30) + " " + arch_d(XR, RA, 0.30)
    d += " L -2.12,0.3 L -2.32,0.36 L -2.36,0.34 L -2.62,0.34 L -2.655,0.345 Z"
    return d


def arch_fill(id, xc, radius, tone, z=1.6):
    a, b, t = xc - radius, xc + radius, YA + radius
    d = f"M {pt(b, 0.30)} L {pt(b, YA)} C {pt(b, YA + K * radius)} {pt(xc + K * radius, t)} {pt(xc, t)} C {pt(xc - K * radius, t)} {pt(a, YA + K * radius)} {pt(a, YA)} L {pt(a, 0.30)} Z"
    return path(id, "chassis", "body", z, tone, d, "body-panels", "paint")


def wheels_elevation(scale):
    out = []
    for tag, bone in (("f", "wheel-f"), ("r", "wheel-r")):
        if scale == 500:
            out.append(ellipse(f"wheel-{tag}", bone, "body", 4, 0.1, 0, 0, RT, RT, ["tyres", "brakes"], "rubber"))
            continue
        out.append(ellipse(f"tyre-{tag}", bone, "body", 4, 0.1, 0, 0, RT, RT, "tyres", "rubber"))
        if scale == 200:
            out.append(ellipse(f"disc-{tag}", bone, "body", 4.4, 0.55, 0, 0, 0.21, 0.21, "brakes", "steel-rim"))
            continue
        out.append(ellipse(f"rim-{tag}", bone, "detail", 4.2, 0.6, 0, 0, 0.221, 0.221, None, "steel-rim"))
        out.append(ellipse(f"disc-{tag}", bone, "detail", 4.4, 0.45, 0, 0, 0.15, 0.15, "brakes", "cast-iron"))
        out.append(ellipse(f"hub-{tag}", bone, "detail", 4.8, 0.72, 0, 0, 0.085, 0.085, None, "steel-rim"))
        if scale == 50:
            for k in range(8):
                a = 2 * math.pi * k / 8
                out.append(ellipse(f"hole-{tag}-{k + 1}", bone, "texture", 4.6, 0.2, 0.118 * math.cos(a), 0.118 * math.sin(a), 0.021, 0.021, None, "steel-rim"))
    return out


def elevation(scale):
    out = wheels_elevation(scale)
    out.append(path("body-shell", "chassis", "body", 2, 0.65, shell_elevation(scale), SHELL + (["bonnet"] if scale >= 200 else []), "paint"))
    out.append(poly("undertray", "chassis", "body", 1.6, 0.15, [(1.24, 0.24), (2.2, 0.24), (2.2, 0.80), (1.24, 0.80)] if scale >= 200 else [(1.24, 0.24), (2.15, 0.22), (2.30, 0.24), (2.30, 0.60), (2.15, 0.80), (1.24, 0.80)], None, "plastic-trim"))
    out.append(arch_fill("wheelhouse", XR, 0.50 if scale == 500 else 0.465, 0.4))
    out.append(poly("roof", "chassis", "body", 2.2, 0.7, [(-2.64, 1.925), (-2.64, 1.965), (-2.61, 1.985), (-2.49, 1.99), (0.35, 1.99), (0.6, 1.988), (0.85, 1.975), (1.0, 1.95), (1.08, 1.92), (1.12, 1.885), (1.12, 1.845), (1.0, 1.875), (0.6, 1.898), (-2.49, 1.90), (-2.55, 1.945)], "roof", "paint"))
    slider_top = 1.84 if scale <= 100 else Y_CANT
    slider_low = 0.40 if scale <= 100 else 0.30
    ox, oy = SLIDER
    out.append(poly("slider-panel", "slider", "body", 2.5, 0.6, [(X_QUARTER - ox, slider_low - oy), (X_B - ox, slider_low - oy), (X_B - ox, slider_top - oy), (X_QUARTER - ox, slider_top - oy)], "slider-door", "paint"))
    out += leaves(scale)
    glass_top = 1.80 if scale <= 100 else 1.89
    window = [(0.37, Y_BELT), (pillar_x(Y_BELT) - 0.055, Y_BELT), (pillar_x(glass_top) - 0.055, glass_top), (0.37, glass_top)]
    screen = [(1.745, 1.205), (1.125, 1.83), (pillar_x(1.83) + 0.055, 1.83), (pillar_x(1.205) + 0.055, 1.205)]
    if scale == 500:
        out.append(poly("glazing", "chassis", "glazing", 6, 0.88, window, "cab-glazing", "glass"))
        out.append(ellipse("mirror", "chassis", "body", 7, 0.3, 1.415, 1.385, 0.06, 0.135, None, "plastic-trim"))
        out.append(rect("bumper-rear", "chassis", "body", 2.3, 0.2, -2.67, 0.34, 0.16, 0.2, None, None, "plastic-trim"))
        return out
    out.append(poly("windscreen", "chassis", "glazing", 6, 0.88, screen, "cab-glazing", "glass"))
    out.append(poly("door-window", "chassis", "glazing", 6, 0.88, window, "cab-glazing", "glass"))
    out.append(capsule("tail-pipe", "chassis", "detail", 1.7, 0.4, (-2.36 if scale < 200 else -2.30, 0.30), (-2.60, 0.30), 0.03, "exhaust-system", "exhaust-steel"))
    out.append(ellipse("mirror", "chassis", "detail", 7, 0.25, 1.415, 1.385, 0.06, 0.135, None, "plastic-trim"))
    out.append(poly("bumper-rear", "chassis", "body", 2.3, 0.2, [(-2.67, 0.37), (-2.67, 0.51), (-2.645, 0.54), (-2.36, 0.54), (-2.36, 0.34), (-2.62, 0.34), (-2.655, 0.345)], None, "plastic-trim"))
    if scale == 200:
        out.append(ellipse("headlamp", "chassis", "glazing", 7, 0.9, 2.48, 0.86, 0.15, 0.085, None, "glass"))
        return out
    grow = {50: 0.007, 100: 0.015}[scale]
    out.append(path("bumper-front", "chassis", "body", 2.3, 0.2, f"M 2.12,0.3 L 2.12,0.56 L {pt(2.67 + grow, 0.56)} L {pt(2.67 + grow, 0.42)} L {pt(2.66 + grow, 0.355)} L {pt(2.63 + grow, 0.295)} L {pt(2.55, 0.26 - grow / 2)} L {pt(2.2, 0.27 - grow / 2)} Z", None, "plastic-trim"))
    out.append(rect("sill", "chassis", "body", 2.3, 0.25, -1.19, 0.30, 2.475, 0.10, None, None, "plastic-trim"))
    out.append(poly("bonnet", "chassis", "body", 2.2, 0.62, [(1.80, 1.17), (1.80, 1.198), (2.2, 1.113), (2.44, 1.053), (2.54, 1.012), (2.54, 0.985), (2.44, 1.025), (2.2, 1.085)], "bonnet", "paint"))
    out.append(poly("headlamp", "chassis", "glazing", 7, 0.9, [(2.29, 0.86), (2.34, 0.93), (2.42, 0.945), (2.56, 0.94), (2.615, 0.92), (2.64, 0.87), (2.60, 0.80), (2.5, 0.775), (2.34, 0.79)], None, "glass"))
    out.append(rect("tail-lamp", "chassis", "detail", 7, 0.35, -2.64, 1.19, 0.024, 0.42, None, None, "glass"))
    out.append(capsule("mirror-arm", "chassis", "detail", 6.9, 0.25, (1.36, 1.26), (1.40, 1.33), 0.022, None, "plastic-trim"))
    out.append(stroke("door-front", "chassis", "detail", 7, 0.3, [(X_HINGE, 0.40), (X_HINGE, Y_BELT)], 0.18))
    out.append(stroke("door-rear", "chassis", "detail", 7, 0.3, [(X_BULK, 0.40), (X_BULK, Y_CANT)], 0.18))
    out.append(stroke("belt-line", "chassis", "detail", 7, 0.3, [(-2.52, Y_BELT), (X_QUARTER, Y_BELT)], 0.18 if scale == 50 else 0.13))
    out.append(ellipse("spare-wheel", "chassis", "detail", 1.7, 0.1, -2.25, 0.355, 0.33, 0.085, None, "rubber"))
    if scale == 100:
        return out
    out.append(rect("quarter-bar", "chassis", "detail", 6.5, 0.6, 1.19, 1.16, 0.04, 0.31, None, None, "paint"))
    out.append(poly("seat-back", "chassis", "detail", 6.2, 0.35, [(0.44, Y_BELT), (0.56, Y_BELT), (0.44, 1.56), (0.35, 1.56)], None, "plastic-trim"))
    out.append(ellipse("headrest", "chassis", "detail", 6.2, 0.35, 0.385, 1.68, 0.05, 0.095, None, "plastic-trim"))
    out.append(capsule("steering-wheel", "chassis", "detail", 6.2, 0.2, (1.0, 1.185), (1.134, 1.356), 0.015, None, "plastic-trim"))
    out.append(capsule("door-handle", "chassis", "detail", 7, 0.2, (0.42, 1.06), (0.56, 1.06), 0.014, None, "plastic-trim"))
    out.append(stroke("rub-strip", "chassis", "texture", 7, 0.3, [(0.36, 0.72), (1.19, 0.72)], 0.13))
    out.append(stroke("slider-track", "chassis", "texture", 7, 0.25, [(-1.92, 1.10), (-0.80, 1.10)], 0.13))
    out.append(capsule("handle-slider", "slider", "detail", 7, 0.2, (X_B - 0.21 - ox, 1.06 - oy), (X_B - 0.07 - ox, 1.06 - oy), 0.014, None, "plastic-trim"))
    out.append(stroke("rub-strip-slider", "slider", "texture", 7, 0.3, [(0.03, 0.72 - oy), (1.02, 0.72 - oy)], 0.13))
    out.append(stroke("slider-belt", "slider", "texture", 7, 0.3, [(0, Y_BELT - oy), (1.05, Y_BELT - oy)], 0.13))
    return out


def section_half(scale):
    """📐 Half of the outline of the body seen from ahead, from the keel up over the side to the roof centre."""
    low = [(0.0, 0.24), (0.66, 0.24)] if scale >= 200 else [(0.0, 0.22), (0.66, 0.22)]
    side = 0.992 if scale <= 100 else 0.985
    low += [(0.66, 0.262), (0.80, 0.262), (0.90, 0.275), (0.955, 0.31), (side, 0.37), (side, 0.56), (W, 0.56), (W, Y_BELT), (0.905, Y_CANT)]
    return low + [(z, roof_y(z)) for z in (0.895, 0.87, 0.82, 0.72, 0.5, 0.25, 0.0)]


def front(scale):
    out = []
    for tag, bone, z in (("rl", "wheel-r", 1.3), ("fl", "wheel-f", 1.4), ("rr", "wheel-r", 1.3), ("fr", "wheel-f", 1.4)):
        s = 1 if tag.endswith("l") else -1
        out.append(rect(f"tyre-{tag}", bone, "body", z, 0.1 if scale <= 100 else 0.2, s * 0.84 - 0.1175, 0, 0.235, 0.712, 0.03, "tyres", "rubber"))
    out.append(poly("body-shell", "chassis", "body", 2, 0.65, mirrored(section_half(scale)), SHELL, "paint"))
    out.append(poly("windscreen", "chassis", "glazing", 6, 0.88, [(-0.90, 1.205), (0.90, 1.205), (0.845, 1.83), (-0.845, 1.83)], "cab-glazing", "glass"))
    for side, s in (("l", 1), ("r", -1)):
        out.append(ellipse(f"mirror-{side}", "chassis", "body" if scale == 500 else "detail", 7, 0.3 if scale == 500 else 0.25, s * 1.04, 1.385, 0.095, 0.135, None, "plastic-trim"))
    t = 0.08 if scale == 500 else 0.045
    proud = {50: 0.008, 100: 0.015, 200: 0.015, 500: 0.0}[scale]
    top = 1.84 if scale <= 100 else Y_CANT
    low = 0.40 if scale <= 100 else 0.30
    lean = LEAN * (top - Y_BELT)
    out.append(poly("slider-panel", "slider", "body", 1.9, 0.6, [(-(W + proud), low), (-(W - t), low), (-(W - t), Y_BELT), (-(W - t - lean), top), (-(W + proud - lean), top), (-(W + proud), Y_BELT)], "slider-door", "paint"))
    out += leaves(scale)
    if scale == 500:
        return out
    roof = [(z, roof_y(z)) for z in (0.905, 0.895, 0.87, 0.82, 0.72, 0.5, 0.25, 0.0)]
    out.append(poly("roof-cap", "chassis", "body", 2.2, 0.7, mirrored(roof), "roof", "paint"))
    if scale == 200:
        for side, s in (("l", 1), ("r", -1)):
            out.append(ellipse(f"headlamp-{side}", "chassis", "glazing", 7, 0.9, s * 0.84, 0.86, 0.13, 0.085, None, "glass"))
        return out
    out.append(poly("bonnet-top", "chassis", "body", 2.3, 0.62, [(-0.77, 1.0), (0.77, 1.0), (0.86, 1.19), (-0.86, 1.19)], "bonnet", "paint"))
    out.append(rect("bumper", "chassis", "body", 2.4, 0.2, -0.992, 0.26, 1.984, 0.30, 0.08, None, "plastic-trim"))
    out.append(rect("grille", "chassis", "detail", 2.5, 0.15, -0.45, 0.70, 0.9, 0.2, 0.015, "engine-bay", "engine-metal"))
    for side, s in (("l", 1), ("r", -1)):
        out.append(rect(f"headlamp-{side}", "chassis", "glazing", 7, 0.9, s * 0.835 - 0.135, 0.775, 0.27, 0.17, 0.04, None, "glass"))
        out.append(capsule(f"mirror-arm-{side}", "chassis", "detail", 6.9, 0.25, (s * 0.975, 1.26), (s * 1.03, 1.33), 0.022, None, "plastic-trim"))
    out.append(rect("cowl", "chassis", "detail", 6.1, 0.15, -0.89, 1.19, 1.78, 0.03, None, None, "plastic-trim"))
    if scale == 100:
        return out
    out.append(rect("intake", "chassis", "detail", 2.5, 0.08, -0.5, 0.32, 1.0, 0.1, 0.014, None, "plastic-trim"))
    out.append(rect("plate-front", "chassis", "detail", 2.6, 0.9, -0.26, 0.445, 0.52, 0.11, 0.004, None, "plastic-trim"))
    for k, y in enumerate((0.75, 0.80, 0.85)):
        out.append(stroke(f"grille-bar-{k + 1}", "chassis", "texture", 2.6, 0.4, [(-0.43, y), (0.43, y)], 0.13))
    out.append(stroke("wiper-driver", "chassis", "detail", 6.5, 0.1, [(0.74, 1.24), (0.12, 1.28)], 0.18))
    out.append(stroke("wiper-passenger", "chassis", "detail", 6.5, 0.1, [(0.02, 1.24), (-0.60, 1.28)], 0.18))
    return out


def plan_half(scale):
    """🗺️ Half of the outline of the body seen from above, from the tail along the left side to the nose."""
    grow = {50: 0.007, 100: 0.015}.get(scale, 0.0)
    bumper = 0.992 if scale <= 100 else 0.985
    rear = [(-2.67, 0.0), (-2.67, 0.93), (-2.655, 0.965), (-2.62, 0.99), (-2.36, 0.992), (-2.36, W)] if scale <= 200 else [(-2.67, 0.0), (-2.67, W)]
    nose = [(2.115, W), (2.115, bumper), (2.20, 0.98 + grow), (2.32, 0.965 + grow), (2.44, 0.935 + grow), (2.54, 0.885 + grow), (2.61, 0.82 + grow), (2.65 + grow, 0.74), (2.67 + grow, 0.66), (2.67 + grow, 0.0)]
    return rear + nose


def plan(scale):
    out = []
    if scale == 500:
        out.append(rect("tyres-r", "wheel-r", "body", 1, 0.25, XR - RT, -0.9575, 2 * RT, 1.915, None, ["tyres", "brakes"], "rubber"))
        out.append(rect("tyres-f", "wheel-f", "body", 1, 0.25, XF - RT, -0.9575, 2 * RT, 1.915, None, ["tyres", "brakes"], "rubber"))
    else:
        for tag, bone, xc in (("rl", "wheel-r", XR), ("fl", "wheel-f", XF), ("rr", "wheel-r", XR), ("fr", "wheel-f", XF)):
            s = 1 if tag.endswith("l") else -1
            out.append(rect(f"tyre-{tag}", bone, "body", 1, 0.1 if scale <= 100 else 0.2, xc - RT, s * 0.84 - 0.1175, 2 * RT, 0.235, 0.03, ["tyres"] if scale <= 100 else ["tyres", "brakes"], "rubber"))
    half = plan_half(scale)
    out.append(poly("body-shell", "chassis", "body", 2, 0.65, half + [(x, -z) for x, z in reversed(half)][1:-1], SHELL if scale >= 200 else "body-panels", "paint"))
    rows = ROOF if scale <= 100 else [(-2.64, 0, 0, 0.89), (-2.55, 0, 0, 0.905), (0.60, 0, 0, 0.90), (1.0, 0, 0, 0.875), (1.12, 0, 0, 0.845)]
    edge = [(x, h) for x, _, _, h in rows]
    out.append(poly("roof", "chassis", "body", 2.2, 0.7, edge + [(x, -h) for x, h in reversed(edge)], "roof", "paint"))
    zs = (-0.90, -0.6, -0.3, 0.0, 0.3, 0.6, 0.90)
    top = (0.845, 0.5, 0.0, -0.5, -0.845)
    out.append(poly("windscreen", "chassis", "glazing", 2.1, 0.88, [(1.745 - sweep(z), z) for z in zs] + [(1.105 - sweep(z), z) for z in top], "cab-glazing", "glass"))
    glass_top = 1.805 if scale <= 100 else 1.89
    for side, s in (("l", 1), ("r", -1)):
        z0, z1 = W - 0.012, W - LEAN * (glass_top - Y_BELT) - 0.012
        out.append(poly(f"side-glass-{side}", "chassis", "glazing", 2.1, 0.88, [(0.365, s * z0), (1.535, s * z0), (pillar_x(glass_top) - 0.055, s * z1), (0.365, s * z1)], "cab-glazing", "glass"))
        out.append(ellipse(f"mirror-{side}", "chassis", "body" if scale == 500 else "detail", 7, 0.3 if scale == 500 else 0.25, 1.415, s * 1.04, 0.06, 0.095, None, "plastic-trim"))
    t = 0.08 if scale == 500 else 0.045
    proud = {50: 0.008, 100: 0.015, 200: 0.015, 500: 0.0}[scale]
    out.append(rect("slider-panel", "slider", "body", 2.5, 0.6 if scale <= 100 else 0.55, X_QUARTER, -(W + proud), X_B - X_QUARTER, t + proud, None, "slider-door", "paint"))
    out += leaves(scale, 0.6 if scale <= 100 else 0.55)
    if scale == 500:
        return out
    out.append(rect("exhaust-tip", "chassis", "detail", 1.2, 0.4, -2.63, -0.34, 0.27 if scale < 200 else 0.33, 0.06, 0.03, "exhaust-system", "exhaust-steel"))
    if scale == 200:
        return out
    out.append(poly("bonnet", "chassis", "body", 2.3, 0.62, [(1.80, -0.86), (2.2, -0.855), (2.44, -0.82), (2.54, -0.77), (2.54, 0.77), (2.44, 0.82), (2.2, 0.855), (1.80, 0.86)], ["bonnet", "engine-bay"], "paint"))
    out.append(poly("cowl", "chassis", "detail", 2.4, 0.15, [(1.725 - sweep(z), z) for z in (-0.89, -0.45, 0.0, 0.45, 0.89)] + [(1.825 - sweep(z), z) for z in (0.89, 0.45, 0.0, -0.45, -0.89)], None, "plastic-trim"))
    for side, s in (("l", 1), ("r", -1)):
        out.append(poly(f"headlamp-{side}", "chassis", "glazing", 7, 0.9, [(2.30, s * 0.973), (2.44, s * 0.943), (2.54, s * 0.893), (2.61, s * 0.83), (2.63, s * 0.80), (2.60, s * 0.79), (2.52, s * 0.86), (2.42, s * 0.915), (2.30, s * 0.945)], None, "glass"))
        out.append(rect(f"tail-lamp-{side}", "chassis", "detail", 7, 0.35, -2.64, s * 0.895 - 0.0425, 0.024, 0.085, None, None, "glass"))
        out.append(capsule(f"mirror-arm-{side}", "chassis", "detail", 6.9, 0.25, (1.36, s * 0.975), (1.40, s * 1.03), 0.022, None, "plastic-trim"))
    if scale == 100:
        return out
    out.append(stroke("wiper-driver", "chassis", "detail", 6.5, 0.1, [(1.69 - sweep(0.74), 0.74), (1.65 - sweep(0.12), 0.12)], 0.18))
    out.append(stroke("wiper-passenger", "chassis", "detail", 6.5, 0.1, [(1.69 - sweep(0.02), 0.02), (1.65 - sweep(0.6), -0.60)], 0.18))
    return out


def drawing(scale):
    return {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}


TEXTS = {
    50: {
        "silhouette": ("True orthographic views of the 1:50 body. Side (seen from the right): the panel van with both wheel arches as circles of 0.42 m around the 0.356 m tyres, sill, bumpers, the raked windscreen in front of the A-pillar, door glass with the quarter-light bar, sliding door, belt line, door joints, roof band, bonnet band, lamps, mirror on its arm, tail pipe and spare wheel under the tail. Front: sill and bumper, body side to the belt at 0.985 m half width, 80 mm lean to the roof line, crowned roof, windscreen, bonnet, grille with three bars, intake, plate, wrapped headlamps and both mirrors to the overall width of 2.27 m. Plan: body outline, roof, swept windscreen with cowl and wipers, leaning side glass, bonnet, lamps, mirrors, the slider standing 8 mm proud and both rear leaves.", "Wahre orthogonale Ansichten des Körpers 1:50. Seite (von rechts): der Kastenwagen mit beiden Radausschnitten als Kreise von 0,42 m um die 0,356-m-Reifen, Schweller, Stoßfängern, der geneigten Windschutzscheibe vor der A-Säule, Türscheibe mit Dreiecksfenstersteg, Schiebetür, Brüstungslinie, Türfugen, Dachband, Haubenband, Leuchten, Spiegel am Arm, Endrohr und Reserverad unter dem Heck. Vorn: Schweller und Stoßfänger, Flanke bis zur Brüstung mit 0,985 m halber Breite, 80 mm Einzug zur Dachlinie, gewölbtes Dach, Windschutzscheibe, Haube, Grill mit drei Stäben, Lufteinlass, Kennzeichen, umlaufende Scheinwerfer und beide Spiegel bis zur Gesamtbreite von 2,27 m. Grundriss: Aufbaukontur, Dach, gepfeilte Windschutzscheibe mit Windlauf und Wischern, geneigte Seitenscheiben, Haube, Leuchten, Spiegel, die 8 mm vorstehende Schiebetür und beide Heckflügel."),
        "retained": ("All printable parts: wheels with rim, eight holes, brake disc and hub, seat back, headrest and steering wheel behind the door glass, handles, rub strips and the slider track as fine lines.", "Alle druckbaren Teile: Räder mit Felge, acht Löchern, Bremsscheibe und Nabe, Sitzlehne, Kopfstütze und Lenkrad hinter der Türscheibe, Griffe, Scheuerleisten und die Schiebetürschiene als feine Linien."),
        "merged": ("Engine bay and exhaust run lie behind the body shell fill; pillars, posts and the cab door are the shell fill between their joints.", "Motorraum und Abgasstrang liegen hinter der Aufbaufläche; Säulen, Pfosten und die Fahrerhaustür sind die Aufbaufläche zwischen ihren Fugen."),
        "omitted": ("Seals, tread, badges, rear hinges, wheel nuts and the front number plate in the side view.", "Dichtungen, Profil, Embleme, Heckscharniere, Radmuttern und das vordere Kennzeichen in der Seitenansicht."),
        "thinElements": ("Door leaves are drawn at their real thickness of 45 mm (0.9 mm); the tail lamp is 24 mm deep (0.48 mm); no shape is thinner than the pen.", "Türflügel sind in ihrer wahren Dicke von 45 mm (0,9 mm) gezeichnet; die Rückleuchte ist 24 mm tief (0,48 mm); keine Form ist dünner als der Stift."),
    },
    100: {
        "silhouette": ("Panel van profile with circular wheel arches 64 mm above each tyre (0.64 mm), sill, bumpers, roof band, bonnet band, windscreen in front of the A-pillar, door glass, sliding door, belt line and both door joints. Front: body side, lean, roof, windscreen, bonnet, bumper, grille, headlamps and both mirrors (2.27 m overall). Plan: body outline, roof, swept windscreen with cowl, side glass, bonnet, lamps, mirrors, slider and both rear leaves.", "Kastenwagenprofil mit kreisförmigen Radausschnitten 64 mm über jedem Reifen (0,64 mm), Schweller, Stoßfängern, Dachband, Haubenband, Windschutzscheibe vor der A-Säule, Türscheibe, Schiebetür, Brüstungslinie und beiden Türfugen. Vorn: Flanke, Einzug, Dach, Windschutzscheibe, Haube, Stoßfänger, Grill, Scheinwerfer und beide Spiegel (2,27 m gesamt). Grundriss: Aufbaukontur, Dach, gepfeilte Windschutzscheibe mit Windlauf, Seitenscheiben, Haube, Leuchten, Spiegel, Schiebetür und beide Heckflügel."),
        "retained": ("Everything above 0.05 m: lamps, mirror with arm, tail pipe, spare wheel, wheels with rim, brake disc and hub.", "Alles über 0,05 m: Leuchten, Spiegel mit Arm, Endrohr, Reserverad, Räder mit Felge, Bremsscheibe und Nabe."),
        "merged": ("Engine bay and exhaust share the body shell; grille bars, intake and plate merge into grille and bumper.", "Motorraum und Abgasanlage teilen die Aufbaufläche; Grillstäbe, Lufteinlass und Kennzeichen gehen in Grill und Stoßfänger auf."),
        "omitted": ("Handles, rub strips, slider track, wipers, wheel holes, the quarter-light bar, seats and steering wheel.", "Griffe, Scheuerleisten, Schiebetürschiene, Wischer, Felgenlöcher, der Dreiecksfenstersteg, Sitze und Lenkrad."),
        "thinElements": ("Door leaves are drawn at their real thickness of 45 mm (0.45 mm); no shape is thinner than the pen.", "Türflügel sind in ihrer wahren Dicke von 45 mm (0,45 mm) gezeichnet; keine Form ist dünner als der Stift."),
    },
    200: {
        "silhouette": ("Panel van profile with circular wheel arches 64 mm (0.32 mm) clear of the tyres, roof band, windscreen and door glass up to the roof, sliding door over the full body height, rear bumper, wheel discs. Front: body with lean and roof, windscreen, headlamps, mirrors (2.27 m overall). Plan: body outline, roof, windscreen, side glass, mirrors, slider and both rear leaves.", "Kastenwagenprofil mit kreisförmigen Radausschnitten im Abstand von 64 mm (0,32 mm) zu den Reifen, Dachband, Windschutz- und Türscheibe bis zum Dach, Schiebetür über die ganze Aufbauhöhe, hinterem Stoßfänger, Radscheiben. Vorn: Aufbau mit Einzug und Dach, Windschutzscheibe, Scheinwerfer, Spiegel (2,27 m gesamt). Grundriss: Aufbaukontur, Dach, Windschutzscheibe, Seitenscheiben, Spiegel, Schiebetür und beide Heckflügel."),
        "retained": ("All main masses, both door bones, mirror (0.19 m = 0.95 mm), headlamp, tail pipe.", "Alle Hauptmassen, beide Türknochen, Spiegel (0,19 m = 0,95 mm), Scheinwerfer, Endrohr."),
        "merged": ("Bonnet, front bumper, grille, sill and engine bay are one body shape; rim and brake disc are one wheel disc (brakes region).", "Haube, vorderer Stoßfänger, Grill, Schweller und Motorraum sind eine Aufbaufläche; Felge und Bremsscheibe sind eine Radscheibe (Region Bremsen)."),
        "omitted": ("Handles, tail lamps, door joints, belt line, hub, spare wheel, interior.", "Griffe, Rückleuchten, Türfugen, Brüstungslinie, Nabe, Reserverad, Innenraum."),
        "thinElements": ("Door leaves are drawn at their real thickness of 45 mm (0.23 mm); the tail pipe is 60 mm (0.3 mm).", "Türflügel sind in ihrer wahren Dicke von 45 mm (0,23 mm) gezeichnet; das Endrohr misst 60 mm (0,3 mm)."),
    },
    500: {
        "silhouette": ("One body silhouette of the panel van with both wheel arches open by 64 mm (0.13 mm) over two solid wheel discs, roof band, door glass, sliding door, rear bumper and the mirror. Front: body with lean, windscreen and both mirrors (2.27 m overall). Plan: body outline, roof, windscreen, side glass, mirrors, slider and both rear leaves as strips.", "Eine Aufbausilhouette des Kastenwagens mit beiden Radausschnitten, 64 mm (0,13 mm) offen über zwei vollen Radscheiben, Dachband, Türscheibe, Schiebetür, hinterem Stoßfänger und dem Spiegel. Vorn: Aufbau mit Einzug, Windschutzscheibe und beide Spiegel (2,27 m gesamt). Grundriss: Aufbaukontur, Dach, Windschutzscheibe, Seitenscheiben, Spiegel, Schiebetür und beide Heckflügel als Streifen."),
        "retained": ("Body, roof, glass, the three doors, wheels and mirrors.", "Aufbau, Dach, Glas, die drei Türen, Räder und Spiegel."),
        "merged": ("Engine, exhaust, bonnet and panels are one body fill; tyre and disc are one disc.", "Motor, Abgasanlage, Haube und Bleche sind eine Aufbaufläche; Reifen und Scheibe sind eine Scheibe."),
        "omitted": ("Handles, lamps, door joints, tail pipe, rim, front bumper line, interior.", "Griffe, Leuchten, Türfugen, Endrohr, Felge, vordere Stoßfängerlinie, Innenraum."),
        "thinElements": ("Door leaves are drawn 80 mm thick (0.16 mm, real 45 mm), just above the smallest printable feature; they lie inside the body outline, so the closed outline equals the real one.", "Türflügel sind 80 mm dick gezeichnet (0,16 mm, real 45 mm), knapp über dem kleinsten druckbaren Merkmal; sie liegen innerhalb der Aufbaukontur, sodass der geschlossene Umriss dem wahren entspricht."),
    },
}


def apply(asset):
    for scale in (50, 100, 200, 500):
        representation = asset["representations"][str(scale)]
        representation["geometry"] = drawing(scale)
        for key, (en, de) in TEXTS[scale].items():
            representation["decisions"][key] = {"en": en, "de": de}
    return asset
