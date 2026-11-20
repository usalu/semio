"""✏️ The drawings of the round 3 city-car: side elevation, front elevation and plan of every scale, drawn shape by shape from the same master lines as the solids (ruling E1), in the frames of the product (elevation: bone-local x along the bone, y across; front: lateral, height; plan: ahead, lateral)."""
import math

from r3_city_car_kit import RA, XF, XR, YA, bonnet_y, screen_point, screen_x

K = 0.5523


def n(v):
    return round(v, 3)


def pt(x, y):
    return f"{n(x)},{n(y)}"


def line(points, close=False):
    return "M " + " L ".join(pt(*p) for p in points) + (" Z" if close else "")


def arch_d(xc, y_from, y_to):
    """⭕ Path piece over one wheel arch from its rear foot to its front foot."""
    a, b, t = xc - RA, xc + RA, YA + RA
    return f"L {pt(a, y_from)} L {pt(a, YA)} C {pt(a, YA + K * RA)} {pt(xc - K * RA, t)} {pt(xc, t)} C {pt(xc + K * RA, t)} {pt(b, YA + K * RA)} {pt(b, YA)} L {pt(b, y_to)}"


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


def path(id, bone, role, z, filled, tone, d, region=None, material=None, **more):
    return shape(id, bone, role, z, filled, tone, "path", region, material, d=d, **more)


def poly(id, bone, role, z, tone, points, region=None, material=None, **more):
    return shape(id, bone, role, z, True, tone, "poly", region, material, points=[[n(x), n(y)] for x, y in points], closed=True, **more)


def ellipse(id, bone, role, z, filled, tone, cx, cy, rx, ry, region=None, material=None, **more):
    return shape(id, bone, role, z, filled, tone, "ellipse", region, material, cx=n(cx), cy=n(cy), rx=n(rx), ry=n(ry), **more)


def capsule(id, bone, role, z, tone, p0, p1, r0, r1=None, region=None, material=None):
    return shape(id, bone, role, z, True, tone, "capsule", region, material, x0=n(p0[0]), y0=n(p0[1]), x1=n(p1[0]), y1=n(p1[1]), r0=r0, r1=r0 if r1 is None else r1)


def rect(id, bone, role, z, tone, x, y, w, h, r=None, region=None, material=None):
    out = shape(id, bone, role, z, True, tone, "rect", region, material, x=n(x), y=n(y), w=n(w), h=n(h))
    if r:
        out["r"] = r
    return out


def mirrored(half):
    """🪞 A closed outline from its half on the positive side, first point and last point on the axis."""
    return half + [(x, -y) for x, y in reversed(half[1:-1])]


ROOF_TOP = {
    50: [(0.34, 1.428), (0.275, 1.454), (0.21, 1.468), (0.14, 1.478), (0.0, 1.49), (-0.3, 1.499), (-0.7, 1.5), (-1.1, 1.493), (-1.35, 1.476), (-1.47, 1.456), (-1.525, 1.438)],
    100: [(0.335, 1.432), (0.25, 1.46), (0.14, 1.478), (-0.3, 1.499), (-0.9, 1.498), (-1.35, 1.476), (-1.525, 1.438)],
    200: [(0.335, 1.435), (0.26, 1.458), (0.14, 1.478), (-0.6, 1.5), (-1.35, 1.476), (-1.525, 1.44)],
    500: [(0.335, 1.44), (0.15, 1.48), (-0.6, 1.5), (-1.525, 1.445)],
}
SILL = {50: 0.193, 100: 0.195, 200: 0.235, 500: 0.235}
NOSE = {
    50: [(1.85, 0.26), (1.9, 0.275), (1.93, 0.31), (1.945, 0.34), (1.95, 0.36), (1.95, 0.6), (1.945, 0.66), (1.93, 0.72), (1.9, 0.765), (1.864, 0.799)],
    100: [(1.85, 0.26), (1.9, 0.275), (1.945, 0.34), (1.95, 0.6), (1.93, 0.72), (1.9, 0.765), (1.864, 0.799)],
    200: [(1.85, 0.26), (1.9, 0.275), (1.95, 0.36), (1.95, 0.6), (1.9, 0.765), (1.864, 0.799)],
    500: [(1.864, 0.26), (1.95, 0.36), (1.95, 0.6), (1.864, 0.799)],
}
TAIL = {
    50: [(-1.78, 1.0), (-1.905, 0.825), (-1.912, 0.6), (-1.95, 0.58), (-1.95, 0.4), (-1.93, 0.34), (-1.9, 0.32)],
    100: [(-1.78, 1.0), (-1.905, 0.825), (-1.912, 0.6), (-1.95, 0.58), (-1.95, 0.4), (-1.9, 0.32)],
    200: [(-1.78, 1.0), (-1.905, 0.825), (-1.912, 0.6), (-1.95, 0.58), (-1.95, 0.4), (-1.905, 0.32)],
    500: [(-1.78, 1.0), (-1.905, 0.825), (-1.95, 0.58), (-1.95, 0.4), (-1.905, 0.32)],
}
SCREEN = {50: [(0.94, 0.972), (0.985, 1.005), (0.975, 1.045), (0.905, 1.04), (0.345, 1.408)], 100: [(0.94, 0.972), (0.985, 1.005), (0.975, 1.045), (0.905, 1.04), (0.345, 1.408)], 200: [(0.94, 0.972), (0.985, 0.985), (0.34, 1.41)], 500: [(0.94, 0.972), (0.985, 0.985), (0.34, 1.41)]}
REAR = {50: [(-1.525, 1.406), (-1.5, 1.42), (-1.785, 0.985)], 100: [(-1.525, 1.406), (-1.5, 1.42), (-1.785, 0.985)], 200: [(-1.525, 1.405), (-1.79, 0.96)], 500: [(-1.525, 1.4), (-1.79, 0.96)]}
BONNET = {
    50: [(0.0, 0.016), (0.25, 0.026), (0.6, 0.026), (0.85, 0.018), (0.915, 0.008), (0.915, -0.016), (0.85, -0.026), (0.6, -0.034), (0.25, -0.034), (0.0, -0.024)],
    100: [(0.0, 0.016), (0.5, 0.026), (0.915, 0.01), (0.915, -0.018), (0.5, -0.034), (0.0, -0.024)],
    200: [(0.0, 0.018), (0.915, 0.014), (0.915, -0.022), (0.0, -0.026)],
    500: [(0.0, 0.018), (0.915, 0.018), (0.915, -0.058), (0.0, -0.058)],
}
HATCH = {
    50: [(0.0, 0.0), (0.748, 0.0), (0.924, 0.126), (0.898, 0.162), (0.733, 0.044), (0.0, 0.044)],
    100: [(0.0, 0.0), (0.748, 0.0), (0.924, 0.126), (0.898, 0.162), (0.733, 0.044), (0.49, 0.044), (0.49, 0.04), (0.0, 0.04)],
    200: [(0.02, 0.004), (0.47, 0.004), (0.47, -0.002), (0.75, -0.002), (0.925, 0.124), (0.897, 0.164), (0.732, 0.046), (0.47, 0.046), (0.47, 0.02), (0.02, 0.02)],
    500: [(0.0, -0.008), (0.745, -0.008), (0.745, 0.068), (0.0, 0.068)],
}


def body_shell_d(scale):
    """🚙 The side outline of everything on the chassis bone, clockwise from the rear bumper foot."""
    sill = SILL[scale]
    d = "M " + pt(*TAIL[scale][-1]) + " " + arch_d(XR, 0.32, sill) + " " + arch_d(XF, sill, 0.26)
    for p in NOSE[scale] + SCREEN[scale] + ROOF_TOP[scale] + REAR[scale] + TAIL[scale][:-1]:
        d += " L " + pt(*p)
    return d + " Z"


def well_d(xc):
    a, b, t = xc - RA, xc + RA, YA + RA
    return f"M {pt(a, 0.25)} L {pt(a, YA)} C {pt(a, YA + K * RA)} {pt(xc - K * RA, t)} {pt(xc, t)} C {pt(xc + K * RA, t)} {pt(b, YA + K * RA)} {pt(b, YA)} L {pt(b, 0.25)} Z"


def wheel_elevation(scale, tag, bone):
    out = [ellipse(f"tyre-{tag}", bone, "body", 4, True, 0.1, 0, 0, 0.3, 0.3, ["tyres", "brakes"] if scale == 500 else "tyres", "rubber")]
    if scale <= 100:
        out.append(ellipse(f"rim-barrel-{tag}", bone, "detail", 4.2, True, 0.55, 0, 0, 0.208 if scale == 50 else 0.2, 0.208 if scale == 50 else 0.2, "tyres", "alloy"))
        out.append(ellipse(f"disc-{tag}", bone, "detail", 4.4, True, 0.42, 0, 0, 0.15, 0.15, "brakes", "cast-iron"))
        for k in range(5):
            a = math.radians(90 + 72 * k)
            r0, r1 = (0.045, 0.187) if scale == 50 else (0.04, 0.18)
            out.append(capsule(f"spoke-{tag}-{k + 1}", bone, "detail", 5, 0.8, (r0 * math.cos(a), r0 * math.sin(a)), (r1 * math.cos(a), r1 * math.sin(a)), 0.022 if scale == 50 else 0.024, 0.018, "tyres", "alloy"))
        out.append(ellipse(f"hub-{tag}", bone, "detail", 5.2, True, 0.7, 0, 0, 0.078, 0.078, "tyres", "alloy"))
    if scale == 50:
        out.append(ellipse(f"sidewall-{tag}", bone, "texture", 4.1, False, 0.3, 0, 0, 0.25, 0.25, "tyres", "rubber"))
        marks = " ".join(f"M {pt(0.283 * math.cos(math.radians(15 * k)), 0.283 * math.sin(math.radians(15 * k)))} L {pt(0.298 * math.cos(math.radians(15 * k)), 0.298 * math.sin(math.radians(15 * k)))}" for k in range(24))
        out.append(path(f"tread-{tag}", bone, "texture", 4.1, False, 0.3, marks, "tyres", "rubber"))
    if scale == 200:
        out.append(ellipse(f"disc-{tag}", bone, "detail", 4.4, True, 0.5, 0, 0, 0.2, 0.2, "brakes", "alloy"))
        out.append(ellipse(f"hub-{tag}", bone, "detail", 5.2, True, 0.72, 0, 0, 0.07, 0.07, "tyres", "alloy"))
    return out


def elevation(scale):
    c = "chassis"
    shell_regions = ["body-panels", "roof", "engine-bay", "exhaust-system"] if scale == 500 else "body-panels"
    out = []
    steel = "exhaust-steel"
    if scale == 50:
        out.append(capsule("exhaust-pipe", c, "detail", 0.8, 0.35, (0.95, 0.26), (-0.92, 0.215), 0.024, None, "exhaust-system", steel))
        out.append(capsule("resonator", c, "detail", 0.8, 0.4, (0.1, 0.235), (-0.25, 0.235), 0.045, None, "exhaust-system", steel))
        out.append(capsule("axle-pipe", c, "detail", 0.8, 0.35, (-0.92, 0.215), (-1.5, 0.33), 0.024, None, "exhaust-system", steel))
    if scale == 100:
        out.append(capsule("exhaust-pipe", c, "detail", 0.8, 0.35, (0.95, 0.25), (-1.5, 0.3), 0.025, None, "exhaust-system", steel))
    if scale <= 200:
        out.append(ellipse("silencer", c, "detail", 0.8, True, 0.4, -1.66, 0.315, 0.085 if scale == 200 else 0.068, 0.085, "exhaust-system", steel))
        out.append(capsule("tail-pipe", c, "detail", 0.8, 0.45, (-1.7, 0.305), (-1.925 if scale < 200 else -1.92, 0.31), 0.025 if scale < 200 else 0.027, None, "exhaust-system", steel))
    out.append(path("well-rear", c, "body", 1, True, 0.04, well_d(XR), "body-panels", "plastic-trim"))
    out.append(path("well-front", c, "body", 1, True, 0.04, well_d(XF), "body-panels", "plastic-trim"))
    out.append(path("body-shell", c, "body", 2, True, 0.62, body_shell_d(scale), shell_regions, "paint"))
    if scale <= 200:
        roof = ROOF_TOP[scale]
        out.append(poly("roof-band", c, "detail", 2.1, 0.5, roof + [(roof[-1][0], 1.406), (-1.47, 1.4), (0.21, 1.4), (roof[0][0], 1.406)], "roof", "paint"))
        out.append(poly("engine-bay-top", c, "detail", 2.2, 0.12, [(1.02, bonnet_y(1.02) + 0.012), (1.8, bonnet_y(1.8) + 0.012), (1.8, bonnet_y(1.8) - 0.02), (1.02, bonnet_y(1.02) - 0.02)], "engine-bay", "engine-metal"))
    trim = "interior-trim"
    if scale == 50:
        out.append(capsule("seat-front-back", c, "detail", 3, 0.3, (-0.285, 1.05), (-0.355, 1.2), 0.05, 0.045, "interior", trim))
        out.append(capsule("seat-front-headrest", c, "detail", 3, 0.3, (-0.4, 1.265), (-0.4, 1.335), 0.05, None, "interior", trim))
        out.append(capsule("seat-rear-back", c, "detail", 3, 0.3, (-1.075, 1.05), (-1.13, 1.14), 0.05, 0.045, "interior", trim))
        out.append(capsule("seat-rear-headrest", c, "detail", 3, 0.3, (-1.165, 1.19), (-1.165, 1.24), 0.045, None, "interior", trim))
    if scale == 100:
        out.append(capsule("seat-front-back", c, "detail", 3, 0.3, (-0.285, 1.05), (-0.365, 1.22), 0.05, 0.045, "interior", trim))
        out.append(capsule("seat-front-headrest", c, "detail", 3, 0.3, (-0.375, 1.25), (-0.41, 1.34), 0.045, None, "interior", trim))
        out.append(capsule("seat-rear-back", c, "detail", 3, 0.3, (-1.085, 1.05), (-1.15, 1.18), 0.05, 0.045, "interior", trim))
    if scale <= 100:
        out.append(capsule("steering-wheel", c, "detail", 3.1, 0.2, (0.381, 1.05), (0.428, 1.145), 0.014 if scale == 50 else 0.0125, None, "interior", trim))
    if scale <= 200:
        out.append(poly("side-glass-front", c, "glazing", 6, 0.85, [(-0.225, 1.035), (0.7, 1.035), (0.235, 1.4), (-0.225, 1.4)], "glazing", "glass"))
        out.append(poly("side-glass-rear", c, "glazing", 6, 0.85, [(-1.268, 1.035), (-0.375, 1.035), (-0.375, 1.4), (-1.167, 1.4)], "glazing", "glass"))
    else:
        out.append(poly("greenhouse", c, "glazing", 6, 0.85, [(-1.268, 1.0), (0.925, 1.0), (0.34, 1.4), (-1.167, 1.4)], "glazing", "glass"))
    if scale <= 100:
        out.append(path("windscreen-edge", c, "glazing", 6.1, False, 0.85, line([(0.905, 1.04), (0.345, 1.405)]), "glazing", "glass"))
        tone = 0.3 if scale == 50 else 0.5
        out.append(path("door-front-edge", c, "detail", 7, False, tone, line([(0.745, 0.33), (0.745, 0.85), (0.75, 0.99)]), "body-panels"))
        out.append(path("door-gap-b", c, "detail", 7, False, tone, line([(-0.3, 0.33), (-0.3, 1.035)]), "body-panels"))
        out.append(path("door-gap-rear", c, "detail", 7, False, tone, line([(-1.02, 0.7), (-1.0, 0.99)]), "body-panels"))
    if scale == 200:
        out.append(path("door-front-edge", c, "detail", 7, False, 0.5, line([(0.745, 0.33), (0.75, 0.99)]), "body-panels"))
        out.append(path("door-gap-b", c, "detail", 7, False, 0.5, line([(-0.3, 0.33), (-0.3, 1.035)]), "body-panels"))
    if scale == 50:
        out.append(path("belt-crease", c, "detail", 7, False, 0.4, line([(-1.78, 0.9), (0.9, 0.9)]), "body-panels", penMm=0.33))
        out.append(path("sill-line", c, "detail", 7, False, 0.4, line([(-0.93, 0.317), (0.78, 0.317)]), "body-panels"))
        out.append(capsule("door-handle-front", c, "detail", 7, 0.3, (-0.26, 0.9), (-0.14, 0.9), 0.012, None, "body-panels", "plastic-trim"))
        out.append(capsule("door-handle-rear", c, "detail", 7, 0.3, (-0.96, 0.9), (-0.84, 0.9), 0.012, None, "body-panels", "plastic-trim"))
        out.append(ellipse("fuel-flap", c, "detail", 7, False, 0.4, -1.52, 0.78, 0.068, 0.068, "body-panels"))
        out.append(path("bumper-front-line", c, "detail", 7, False, 0.3, line([(1.515, 0.5), (1.95, 0.5)]), "body-panels"))
        out.append(path("bumper-rear-line", c, "detail", 7, False, 0.3, line([(-1.912, 0.6), (-1.665, 0.6)]), "body-panels"))
    if scale <= 200:
        out.append(ellipse("mirror", c, "detail", 7, True, 0.3, 0.76, 1.08, 0.055 if scale < 200 else 0.06, 0.065 if scale == 50 else 0.06, "body-panels", "plastic-trim"))
        out.append(ellipse("headlamp", c, "glazing", 7, True, 0.9, 1.84, 0.7, 0.09, 0.06, "glazing", "glass"))
        out.append(rect("tail-lamp", c, "detail", 7, 0.7, -1.937, 0.63 if scale < 200 else 0.62, 0.05, 0.15 if scale < 200 else 0.16, 0.02, "tailgate", "glass"))
    else:
        out.append(ellipse("mirror", c, "body", 7, True, 0.3, 0.76, 1.08, 0.07, 0.07, "body-panels", "plastic-trim"))
    out.append(poly("bonnet-slab" if scale == 50 else "bonnet-panel", "bonnet", "body", 2.3, 0.62, BONNET[scale], "bonnet", "paint"))
    if scale == 50:
        out.append(path("bonnet-crease", "bonnet", "detail", 7, False, 0.4, line([(0.03, -0.004), (0.9, -0.004)]), "bonnet"))
    out.append(poly("hatch-slab" if scale == 50 else "tailgate-panel", "hatch", "body", 2.3, 0.62, HATCH[scale], "tailgate", "paint"))
    if scale <= 200:
        glass = {50: [(0.03, 0.0), (0.5, 0.0), (0.5, 0.012), (0.03, 0.012)], 100: [(0.04, 0.0), (0.48, 0.0), (0.48, 0.012), (0.04, 0.012)], 200: [(0.02, 0.004), (0.49, 0.004), (0.49, 0.02), (0.02, 0.02)]}[scale]
        out.append(poly("hatch-glass", "hatch", "glazing", 6, 0.85, glass, "glazing", "glass"))
    if scale == 50:
        out.append(path("hatch-seam", "hatch", "detail", 7, False, 0.4, line([(0.47, 0.0), (0.47, 0.044)]), "tailgate"))
        out.append(capsule("well-liner-f", "axle-f", "detail", 3.9, 0.2, (-0.02, 0.2), (-0.035, 0.34), 0.02, None, "body-panels", "alloy"))
        out.append(capsule("well-liner-r", "axle-r", "detail", 3.9, 0.2, (-0.085, 0.2), (-0.105, 0.33), 0.02, None, "body-panels", "alloy"))
        for tag, cx, cy, w, h in (("f", 0.085, 0.118, 0.11, 0.05), ("r", -0.085, -0.118, 0.1, 0.045)):
            ca, sa = math.cos(math.radians(-36)), math.sin(math.radians(-36))
            corners = [(cx + ca * u - sa * v, cy + sa * u + ca * v) for u, v in ((-w / 2, -h / 2), (w / 2, -h / 2), (w / 2, h / 2), (-w / 2, h / 2))]
            out.append(poly(f"caliper-{tag}", f"axle-{tag}", "detail", 4.6, 0.35, corners, "brakes", "cast-iron"))
    return out + wheel_elevation(scale, "f", "wheel-f") + wheel_elevation(scale, "r", "wheel-r")


FRONT_SHELL = {
    50: [(0.0, 0.235), (0.62, 0.235), (0.69, 0.193), (0.8, 0.193), (0.832, 0.24), (0.84, 0.4), (0.84, 0.92), (0.83, 0.975), (0.8, 0.998), (0.775, 1.0), (0.625, 1.4), (0.622, 1.45), (0.612, 1.474), (0.57, 1.49), (0.45, 1.497), (0.3, 1.4995), (0.0, 1.5)],
    100: [(0.0, 0.235), (0.62, 0.235), (0.69, 0.195), (0.8, 0.195), (0.835, 0.24), (0.84, 0.92), (0.825, 0.985), (0.775, 1.0), (0.625, 1.4), (0.622, 1.45), (0.6, 1.48), (0.45, 1.497), (0.0, 1.5)],
    200: [(0.0, 0.235), (0.78, 0.235), (0.835, 0.29), (0.84, 0.92), (0.825, 0.985), (0.785, 1.0), (0.625, 1.4), (0.622, 1.45), (0.6, 1.48), (0.45, 1.497), (0.0, 1.5)],
    500: [(0.0, 0.235), (0.78, 0.235), (0.84, 0.3), (0.84, 0.92), (0.785, 1.0), (0.625, 1.4), (0.6, 1.48), (0.0, 1.5)],
}


def flip(half):
    return half + [(-x, y) for x, y in reversed(half[1:-1])]


def swept(id, bone, role, z, tone, profile, half, region, material):
    return poly(id, bone, role, z, tone, profile, region, material, solid={"kind": "extruded", "sweep": "w", "z0": -half, "z1": half})


def front(scale):
    c = "chassis"
    tone = {50: 0.1, 100: 0.15, 200: 0.2, 500: 0.25}[scale]
    out = [rect(f"tyre-{tag}", bone, "body", z, tone, x, 0, 0.185, 0.6, 0.03, "tyres", "rubber") for tag, bone, z, x in (("rl", "wheel-r", 3.9, 0.6275), ("fl", "wheel-f", 4, 0.6275), ("rr", "wheel-r", 3.9, -0.8125), ("fr", "wheel-f", 4, -0.8125))]
    if scale <= 200:
        out.append(capsule("silencer", c, "detail", 0.8, 0.4, (-0.44, 0.315), (0.0, 0.315), 0.085, None, "exhaust-system", "exhaust-steel"))
    if scale <= 100:
        out.append(ellipse("tail-pipe", c, "detail", 0.8, True, 0.4, -0.3, 0.31, 0.025, 0.025, "exhaust-system", "exhaust-steel"))
    if scale <= 100:
        out.append(rect("subframe", c, "detail", 0.9, 0.3, -0.35, 0.205, 0.7, 0.05, 0.015, "body-panels", "cast-iron"))
        for side, s in (("l", 1), ("r", -1)):
            out.append(capsule(f"lower-arm-{side}", c, "detail", 0.9, 0.3, (s * 0.33, 0.24), (s * 0.64, 0.215), 0.02, None, "body-panels", "cast-iron"))
    out.append(swept("hatch-panel", "hatch", "body", 1.5, 0.62, HATCH[scale], 0.6, "tailgate" if scale <= 200 else ["tailgate", "glazing"], "paint"))
    if scale <= 200:
        glass = {50: [(0.03, 0.0), (0.5, 0.0), (0.5, 0.012), (0.03, 0.012)], 100: [(0.04, 0.0), (0.48, 0.0), (0.48, 0.012), (0.04, 0.012)], 200: [(0.02, 0.004), (0.49, 0.004), (0.49, 0.02), (0.02, 0.02)]}[scale]
        out.append(swept("hatch-glass", "hatch", "glazing", 1.6, 0.85, glass, 0.58, "glazing", "glass"))
    if scale <= 100:
        out.append(rect("engine-bay", c, "detail", 1.8, 0.12 if scale == 50 else 0.15, -0.62, 0.79, 1.24, 0.17, 0.03, "engine-bay", "engine-metal"))
    regions = {50: "body-panels", 100: ["body-panels", "interior"], 200: ["body-panels", "engine-bay", "exhaust-system", "interior"], 500: ["body-panels", "roof", "engine-bay", "exhaust-system"]}[scale]
    out.append(poly("body-shell", c, "body", 2, 0.62, flip(FRONT_SHELL[scale]), regions, "paint"))
    if scale <= 200:
        out.append(poly("roof-band", c, "detail", 2.1, 0.5, flip([(0.0, 1.4), (0.6, 1.4), (0.622, 1.45), (0.6, 1.48), (0.45, 1.497), (0.0, 1.5)]), "roof", "paint"))
    out.append(swept("bonnet-panel", "bonnet", "body", 2.3, 0.62 if scale <= 100 else 0.55, BONNET[scale], 0.7, "bonnet", "paint"))
    if scale == 50:
        out.append(rect("bumper", c, "detail", 2.4, 0.15, -0.45, 0.36, 0.9, 0.1, 0.014, "body-panels", "plastic-trim"))
        out.append(rect("grille", c, "detail", 2.5, 0.15, -0.31, 0.64, 0.62, 0.05, 0.012, "engine-bay", "engine-metal"))
        out.append(rect("plate-front", c, "detail", 2.6, 0.9, -0.26, 0.475, 0.52, 0.11, 0.004, "body-panels", "plastic-trim"))
    if scale == 100:
        out.append(rect("grille", c, "detail", 2.5, 0.15, -0.45, 0.355, 0.9, 0.13, 0.014, "engine-bay", "engine-metal"))
    if scale <= 100:
        for side, s in (("l", 1), ("r", -1)):
            if scale == 50:
                out.append(rect(f"seat-front-headrest-{side}", c, "detail", 3, 0.3, s * 0.36 - 0.12, 1.215, 0.24, 0.17, 0.06, "interior", "interior-trim"))
            else:
                out.append(rect(f"seat-front-headrest-{side}", c, "detail", 3, 0.3, s * 0.36 - 0.11, 1.22, 0.22, 0.18, 0.06, "interior", "interior-trim"))
        out.append(path("steering-wheel", c, "detail", 3.1, False, 0.2, f"M {pt(0.196, 1.05)} C {pt(0.22, 1.19)} {pt(0.5, 1.19)} {pt(0.524, 1.05)}", "interior", "interior-trim"))
    out.append(poly("windscreen" if scale <= 200 else "greenhouse", c, "glazing", 6, 0.85, [(-0.745, 1.035), (0.745, 1.035), (0.6, 1.4), (-0.6, 1.4)] if scale <= 100 else [(-0.76, 1.0), (0.76, 1.0), (0.61, 1.4), (-0.61, 1.4)], "glazing", "glass"))
    if scale == 50:
        for name, z0, z1 in (("driver", 0.64, 0.08), ("passenger", -0.02, -0.56)):
            a, b = screen_point(0.085, z0, 0.012), screen_point(0.125, z1, 0.012)
            out.append(path(f"wiper-{name}", c, "detail", 6.5, False, 0.1, line([(a[2], a[1]), (b[2], b[1])]), "glazing"))
    for side, s in (("l", 1), ("r", -1)):
        if scale <= 200:
            out.append(ellipse(f"headlamp-{side}", c, "glazing", 7, True, 0.85, s * 0.63, 0.7, 0.1, 0.06, "glazing", "glass"))
        radii = {50: (0.095, 0.065), 100: (0.105, 0.06), 200: (0.11, 0.06), 500: (0.115, 0.07)}[scale]
        centre = {50: 0.855, 100: 0.845, 200: 0.84, 500: 0.835}[scale]
        out.append(ellipse(f"mirror-{side}", c, "detail" if scale <= 200 else "body", 7, True, 0.3, s * centre, 1.08 if scale > 50 else 1.085, radii[0], radii[1], "body-panels", "plastic-trim"))
    return out


PLAN_HALF = {
    50: [(-1.95, 0.0), (-1.95, 0.62), (-1.93, 0.7), (-1.905, 0.745), (-1.85, 0.785), (-1.78, 0.81), (-1.665, 0.83), (-1.3, 0.84), (-0.935, 0.84), (-0.6, 0.83), (0.0, 0.825), (0.5, 0.83), (0.785, 0.84), (1.15, 0.84), (1.515, 0.825), (1.7, 0.79), (1.8, 0.735), (1.864, 0.675), (1.9, 0.62), (1.93, 0.54), (1.945, 0.46), (1.95, 0.4), (1.95, 0.0)],
    100: [(-1.95, 0.0), (-1.95, 0.62), (-1.93, 0.7), (-1.85, 0.785), (-1.665, 0.83), (-1.3, 0.84), (-0.935, 0.84), (0.0, 0.825), (0.785, 0.84), (1.15, 0.84), (1.515, 0.825), (1.7, 0.79), (1.8, 0.735), (1.9, 0.62), (1.945, 0.46), (1.95, 0.4), (1.95, 0.0)],
    200: [(-1.95, 0.0), (-1.95, 0.62), (-1.905, 0.745), (-1.78, 0.81), (-1.665, 0.83), (-1.3, 0.84), (-0.935, 0.84), (0.0, 0.825), (0.785, 0.84), (1.15, 0.84), (1.515, 0.825), (1.7, 0.79), (1.864, 0.675), (1.95, 0.4), (1.95, 0.0)],
    500: [(-1.95, 0.0), (-1.95, 0.62), (-1.905, 0.745), (-1.78, 0.81), (-1.3, 0.84), (1.15, 0.84), (1.515, 0.825), (1.864, 0.675), (1.95, 0.4), (1.95, 0.0)],
}
PLAN_ROOF = {
    50: [(0.34, 0.0), (0.34, 0.12), (0.33, 0.2), (0.317, 0.3), (0.299, 0.4), (0.275, 0.5), (0.25, 0.575), (0.21, 0.61), (0.14, 0.622), (-1.1, 0.622), (-1.35, 0.612), (-1.47, 0.598), (-1.525, 0.575), (-1.525, 0.0)],
    100: [(0.335, 0.0), (0.335, 0.2), (0.3, 0.4), (0.25, 0.575), (0.14, 0.622), (-0.9, 0.622), (-1.35, 0.612), (-1.525, 0.575), (-1.525, 0.0)],
    200: [(0.335, 0.0), (0.335, 0.25), (0.26, 0.56), (0.14, 0.622), (-0.6, 0.622), (-1.35, 0.612), (-1.525, 0.575), (-1.525, 0.0)],
    500: [(0.335, 0.0), (0.335, 0.4), (0.15, 0.622), (-0.6, 0.622), (-1.525, 0.59), (-1.525, 0.0)],
}


def plan(scale):
    c = "chassis"
    tone = {50: 0.1, 100: 0.15, 200: 0.2, 500: 0.25}[scale]
    tyre_regions = ["tyres"] if scale <= 100 else ["tyres", "brakes"]
    out = [rect(f"tyre-{tag}", bone, "body", 1, tone, x, y, 0.6, 0.185, 0.03, tyre_regions, "rubber") for tag, bone, x, y in (("rl", "wheel-r", XR - 0.3, 0.6275), ("fl", "wheel-f", XF - 0.3, 0.6275), ("rr", "wheel-r", XR - 0.3, -0.8125), ("fr", "wheel-f", XF - 0.3, -0.8125))]
    if scale <= 100:
        out.append(rect("engine-bay", c, "detail", 1.8, 0.12 if scale == 50 else 0.15, 1.02, -0.62, 0.78, 1.24, 0.05, "engine-bay", "engine-metal"))
    regions = {50: "body-panels", 100: ["body-panels", "exhaust-system"], 200: ["body-panels", "engine-bay", "exhaust-system"], 500: ["body-panels", "roof", "engine-bay", "exhaust-system"]}[scale]
    out.append(poly("body-shell", c, "body", 2, 0.62, mirrored(PLAN_HALF[scale]), regions, "paint"))
    if scale <= 100:
        zs = (-0.735, -0.4, 0.0, 0.4, 0.735)
        out.append(poly("cowl", c, "detail", 2.2, 0.15, [(0.985 - screen_x(z), z) for z in zs] + [(0.875 - screen_x(z), z) for z in reversed(zs)], "body-panels", "plastic-trim"))
    out.append(swept("bonnet-0", "bonnet", "body", 2.3, 0.62, BONNET[scale], 0.7, "bonnet", "paint"))
    out.append(swept("hatch-0", "hatch", "body", 2.3, 0.62, HATCH[scale], 0.6, "tailgate", "paint"))
    out.append(poly("roof", c, "body", 2.4, 0.5, mirrored(PLAN_ROOF[scale]), "roof", "paint"))
    if scale == 50:
        out.append(ellipse("binnacle", c, "detail", 3, True, 0.2, 0.6, 0.36, 0.11, 0.17, "interior", "interior-trim"))
    if scale <= 100:
        out.append(path("steering-wheel", c, "detail", 3.1, False, 0.2, f"M {pt(0.35, 0.2)} C {pt(0.46, 0.2)} {pt(0.46, 0.52)} {pt(0.35, 0.52)}", "interior", "interior-trim"))
    if scale <= 200:
        glass = {50: [(0.03, 0.0), (0.5, 0.0), (0.5, 0.012), (0.03, 0.012)], 100: [(0.04, 0.0), (0.48, 0.0), (0.48, 0.012), (0.04, 0.012)], 200: [(0.02, 0.004), (0.49, 0.004), (0.49, 0.02), (0.02, 0.02)]}[scale]
        out.append(swept("hatch-glass", "hatch", "glazing", 5.9, 0.85, glass, 0.58, "glazing", "glass"))
    screen = [(0.757, -0.745), (0.863, -0.4), (0.905, 0.0), (0.863, 0.4), (0.757, 0.745), (0.25, 0.6), (0.32, 0.3), (0.345, 0.0), (0.32, -0.3), (0.25, -0.6)] if scale <= 100 else [(0.78, -0.76), (0.925, -0.3), (0.925, 0.3), (0.78, 0.76), (0.26, 0.6), (0.335, 0.0), (0.26, -0.6)]
    out.append(poly("windscreen" if scale <= 200 else "greenhouse", c, "glazing", 6, 0.85, screen, "glazing", "glass"))
    for side, s in (("l", 1), ("r", -1)):
        if scale <= 100:
            out.append(poly(f"side-glass-front-{side}", c, "glazing", 6, 0.85, [(-0.225, s * 0.762), (0.7, s * 0.762), (0.235, s * 0.624), (-0.225, s * 0.624)], "glazing", "glass"))
            out.append(poly(f"side-glass-rear-{side}", c, "glazing", 6, 0.85, [(-1.268, s * 0.762), (-0.375, s * 0.762), (-0.375, s * 0.624), (-1.167, s * 0.624)], "glazing", "glass"))
        else:
            out.append(poly(f"side-glass-{side}", c, "glazing", 6, 0.85, [(-1.268, s * 0.77), (0.72, s * 0.77), (0.235, s * 0.624), (-1.167, s * 0.624)], "glazing", "glass"))
        radii = {50: (0.055, 0.095), 100: (0.055, 0.105), 200: (0.06, 0.11), 500: (0.07, 0.115)}[scale]
        centre = {50: 0.855, 100: 0.845, 200: 0.84, 500: 0.835}[scale]
        out.append(ellipse(f"mirror-{side}", c, "detail" if scale <= 200 else "body", 7, True, 0.3, 0.76, s * centre, radii[0], radii[1], "body-panels", "plastic-trim"))
        if scale <= 200:
            out.append(ellipse(f"headlamp-{side}", c, "glazing", 7, True, 0.85, 1.85, s * 0.63, 0.06, 0.1, "glazing", "glass"))
            out.append(rect(f"tail-lamp-{side}", c, "detail", 7, 0.7, -1.937, s * 0.65 - 0.05, 0.05, 0.1, 0.01, "tailgate", "glass"))
    if scale <= 200:
        out.append(capsule("tail-pipe", c, "detail", 7, 0.45, (-1.9, -0.3), (-1.925 if scale < 200 else -1.92, -0.3), 0.025 if scale < 200 else 0.027, None, "exhaust-system", "exhaust-steel"))
    if scale == 50:
        for name, z0, z1 in (("driver", 0.64, 0.08), ("passenger", -0.02, -0.56)):
            a, b = screen_point(0.085, z0, 0.012), screen_point(0.125, z1, 0.012)
            out.append(path(f"wiper-{name}", c, "detail", 6.5, False, 0.1, line([(a[0], a[2]), (b[0], b[2])]), "glazing"))
    return out


def drawing(scale):
    return {"elevation": elevation(scale), "front": front(scale), "plan": plan(scale)}
