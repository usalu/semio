"""✏️ Vacuum cleaner 2D: the side elevation, front elevation and plan of every scale, drawn from the same measures as the solids (ruling E1), with the representation decisions per scale.

Chain parts (hose, handle, wand, nozzle, wheels, caster) are drawn in the side elevation in their bone frames and carried into front and plan as extruded solids; the canister is drawn in all three views from its plan-section table.

@see ./r3-vacuum-cleaner-kit.py
@see ./r3-vacuum-cleaner-model.py
"""
import importlib.util
from pathlib import Path


def _load(name, file):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(file))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit", "r3-vacuum-cleaner-kit.py")
M = _load("model", "r3-vacuum-cleaner-model.py")
HOSES = [f"hose-{i + 1}" for i in range(K.HOSE_N)]
CAN_KEYS = K.SHELL_KEYS[:-1] + K.LID_KEYS


def shp(sid, bone, role, z, tone, geom, region=None, material=None, filled=True, regions=None, solid=None):
    """🖌️ One 2D shape with the members in the catalog's order."""
    d = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, **geom}
    if regions:
        d["regions"] = regions
    elif region:
        d["region"] = region
    if material:
        d["material"] = material
    if solid:
        d["solid"] = solid
    return K.r5(d)


def poly(points, closed=True):
    return {"kind": "poly", "points": [[x, y] for x, y in points], "closed": closed}


def path(*rings):
    return {"kind": "path", "d": K.path_d(rings)}


def rect(x0, y0, x1, y1, r=0.0):
    g = {"kind": "rect", "x": x0, "y": y0, "w": x1 - x0, "h": y1 - y0}
    if r > 0:
        g["r"] = r
    return g


def ell(cx, cy, rx, ry=None):
    return {"kind": "ellipse", "cx": cx, "cy": cy, "rx": rx, "ry": rx if ry is None else ry}


def cap(x0, x1, r):
    return {"kind": "capsule", "x0": x0, "y0": 0, "x1": x1, "y1": 0, "r0": r, "r1": r}


def ext(z0, z1):
    return {"kind": "extruded", "sweep": "w", "z0": z0, "z1": z1}


def revolve_outline(profile):
    """🌀 Side outline (u, v) of an open revolve profile closed through the axis."""
    pts = [p for p in profile if p[1] > 0]
    return [(a, r) for a, r in pts] + [(a, -r) for a, r in reversed(pts)]


def loft_outline(stations):
    """🧵 Side outline (u, v) of an axial loft: top then bottom."""
    top = [(st[0], (st[3] if len(st) > 3 else 0.0) + st[2]) for st in stations]
    bot = [(st[0], (st[3] if len(st) > 3 else 0.0) - st[2]) for st in reversed(stations)]
    return top + bot


def clean(pts, scale):
    return K.simplify(pts, K.FEATURE[scale] / 12)


GRIP_50 = [(-0.006, 0.010, 0.010), (0.004, 0.019, 0.020), (0.02, 0.0205, 0.022), (0.05, 0.019, 0.0205, -0.002), (0.09, 0.0195, 0.0215, -0.003), (0.13, 0.0195, 0.0215, -0.002), (0.16, 0.0215, 0.023), (0.19, 0.0215, 0.023), (0.20, 0.019, 0.0195), (0.204, 0.012, 0.012)]
GRIP_100 = [(-0.022, 0.011, 0.011), (-0.012, 0.021, 0.021), (0.01, 0.021, 0.022), (0.15, 0.02, 0.0215), (0.195, 0.0215, 0.023), (0.206, 0.012, 0.012)]
NECK_50 = [(0.715, 0.0185, 0.0185), (0.73, 0.0195, 0.0195), (0.78, 0.0195, 0.0195), (0.82, 0.0185, 0.0185), (0.845, 0.016, 0.016), (0.862, 0.011, 0.011)]
NECK_100 = [(0.715, 0.019, 0.019), (0.80, 0.0195, 0.0195), (0.862, 0.012, 0.012)]


def bumper_side():
    """🛡️ Side view of the bumper band round the parting line."""
    a, b, c = M.plan_c(K.PARTING, 0.004)
    y0, y1 = K.BUMPER
    return K.rounded_rect(c - a, y0, c + a, y1, 0.004)


def canister_side(scale):
    if scale >= 200:
        return {"shell": clean(K.side_outline(CAN_KEYS, 0.035, K.LID_TOP), scale)}
    return {"shell": clean(K.side_outline(K.SHELL_KEYS, 0.035, K.PARTING), scale), "lid": clean(K.side_outline(K.LID_KEYS, K.PARTING, K.LID_TOP), scale)}


def canister_front(scale):
    if scale >= 200:
        return {"shell": clean(K.front_outline(CAN_KEYS, 0.035, K.LID_TOP), scale)}
    return {"shell": clean(K.front_outline(K.SHELL_KEYS, 0.035, K.PARTING), scale), "lid": clean(K.front_outline(K.LID_KEYS, K.PARTING, K.LID_TOP), scale)}


def canister_plan(scale):
    if scale >= 200:
        return {"shell": clean(K.plan_outline(CAN_KEYS, 0.035, K.LID_TOP), scale)}
    return {"shell": clean(K.plan_outline(K.SHELL_KEYS, 0.035, K.PARTING), scale), "lid": clean(K.plan_outline(K.LID_KEYS, K.PARTING, K.PARTING + 0.002), scale)}


def chain(scale):
    """⛓️ Side-elevation shapes of the chain parts with the lateral span each one takes in front and plan."""
    out = []
    hr = 0.03 if scale == 500 else K.HOSE_R
    for h in HOSES:
        out.append((shp(f"{h}-tube", h, "body", 2.2, 0.35, cap(0, K.HOSE_SEG, hr) if scale < 500 else rect(-0.012, -hr, K.HOSE_SEG + 0.012, hr), "hose", "hose-pvc"), ext(-hr, hr)))
        if scale == 50:
            for k in range(7):
                a = 0.015 + 0.02 * k
                out.append((shp(f"{h}-rib-{k}", h, "texture", 2.3, 0.15, poly([(a, -0.0195), (a, 0.0195)], False), "hose", filled=False), None))
    if scale == 50:
        out.append((shp("hose-end", "hose-10", "body", 2.35, 0.3, path(revolve_outline([(0.105, 0.0), (0.105, 0.0195), (0.112, 0.021), (0.153, 0.021), (0.153, 0.0)])), "hose", "abs-grey"), ext(-0.021, 0.021)))
    if scale == 50:
        out.append((shp("grip-swivel", "grip", "body", 2.7, 0.3, ell(0, 0, 0.02), "grip", "abs-grey"), ext(-0.02, 0.02)))
        out.append((shp("grip-sleeve", "grip", "body", 2.75, 0.3, path(clean(loft_outline(GRIP_50), 50)), "grip", "abs-grey"), ext(-0.0215, 0.0215)))
        out.append((shp("air-slide", "grip", "body", 2.8, 0.7, rect(0.058, 0.017, 0.092, 0.025, 0.003), "grip", "abs-grey"), ext(-0.008, 0.008)))
    elif scale == 100:
        out.append((shp("grip-sleeve", "grip", "body", 2.75, 0.3, path(clean(loft_outline(GRIP_100), 100)), "grip", "abs-grey"), ext(-0.0215, 0.0215)))
    elif scale == 200:
        out.append((shp("grip-sleeve", "grip", "body", 2.75, 0.3, rect(-0.02, -0.021, 0.19, 0.021), "grip", "abs-grey"), ext(-0.021, 0.021)))
    else:
        out.append((shp("grip-sleeve", "grip", "body", 2.75, 0.3, rect(-0.012, -0.03, 0.18, 0.03), "grip", "abs-grey"), ext(-0.03, 0.03)))
    if scale == 50:
        out.append((shp("wand-tube", "wand", "body", 2.5, 0.8, rect(0.0, -0.0175, 0.30, 0.0175), "wand", "tube-steel"), ext(-0.0175, 0.0175)))
        out.append((shp("wand-inner", "wand", "body", 2.5, 0.85, rect(0.30, -0.016, 0.74, 0.016), "wand", "tube-steel"), ext(-0.016, 0.016)))
        out.append((shp("wand-collar", "wand", "body", 2.55, 0.3, path(revolve_outline([(0.28, 0.0), (0.28, 0.0205), (0.286, 0.0215), (0.312, 0.0215), (0.318, 0.0195), (0.318, 0.0)])), "wand", "abs-grey"), ext(-0.0215, 0.0215)))
        out.append((shp("wand-lock", "wand", "body", 2.56, 0.6, rect(0.291, 0.0185, 0.307, 0.0265, 0.003), "wand", "abs-grey"), ext(-0.006, 0.006)))
        out.append((shp("wand-shine", "wand", "detail", 2.57, 0.95, poly([(0.03, 0.008), (0.27, 0.008)], False), "wand", filled=False), None))
        out.append((shp("wand-shine-inner", "wand", "detail", 2.57, 0.95, poly([(0.33, 0.007), (0.70, 0.007)], False), "wand", filled=False), None))
        out.append((shp("nozzle-neck", "wand", "body", 2.52, 0.45, path(clean(loft_outline(NECK_50), 50)), "nozzle", "abs-grey"), ext(-0.0195, 0.0195)))
    elif scale == 100:
        out.append((shp("wand-tube", "wand", "body", 2.5, 0.8, rect(0.0, -0.0175, 0.74, 0.0175), "wand", "tube-steel"), ext(-0.0175, 0.0175)))
        out.append((shp("nozzle-neck", "wand", "body", 2.52, 0.45, path(clean(loft_outline(NECK_100), 100)), "nozzle", "abs-grey"), ext(-0.0195, 0.0195)))
    elif scale == 200:
        out.append((shp("wand-tube", "wand", "body", 2.5, 0.8, rect(0.0, -0.0175, 0.85, 0.0175), "wand", "tube-steel"), ext(-0.0175, 0.0175)))
    else:
        out.append((shp("wand-tube", "wand", "body", 2.5, 0.8, rect(0.0, -0.03, 0.83, 0.03), "wand", "tube-steel"), ext(-0.03, 0.03)))
    head = {50: M.HEAD, 100: M.HEAD_100, 200: M.HEAD_200, 500: M.HEAD_500}[scale]
    if scale < 500:
        out.append((shp("swivel", "nozzle", "body", 2.55, 0.5, ell(0, 0, 0.02), "nozzle", "abs-grey"), ext(-0.02, 0.02)))
    if scale <= 100:
        out.append((shp("brush", "nozzle", "body", 2.58, 0.15, rect(0.035, -0.047, 0.041, -0.039), "brush", "nylon-bristle"), ext(-0.125, 0.125)))
        out.append((shp("brush-rear", "nozzle", "body", 2.58, 0.15, rect(-0.032, -0.047, -0.026, -0.039), "brush", "nylon-bristle"), ext(-0.125, 0.125)))
    half = 0.1325 if scale == 50 else K.NOZZLE_HALF
    out.append((shp("nozzle", "nozzle", "body", 2.6, 0.45, path(head), "nozzle", "abs-grey", regions=["nozzle", "brush"] if scale >= 200 else None), ext(-half, half)))
    if scale == 50:
        capd = path([(round(x, 5), round(y, 5)) for x, y in K.offset(M.HEAD, 0.0015)])
        out.append((shp("nozzle-cap", "nozzle", "body", 2.62, 0.2, capd, "nozzle", "rubber-wheel"), ext(0.1315, K.NOZZLE_HALF)))
        out.append((shp("nozzle-cap-right", "nozzle", "body", 2.61, 0.2, capd, "nozzle", "rubber-wheel"), ext(-K.NOZZLE_HALF, -0.1315)))
        out.append((shp("nozzle-yoke", "nozzle", "body", 2.65, 0.45, path([(round(x, 5), round(y, 5)) for x, y in M.YOKE]), "nozzle", "abs-grey"), ext(0.021, 0.031)))
        out.append((shp("nozzle-yoke-right", "nozzle", "body", 2.64, 0.45, path([(round(x, 5), round(y, 5)) for x, y in M.YOKE]), "nozzle", "abs-grey"), ext(-0.031, -0.021)))
    return out


def wheels(scale):
    """🛞 Side shapes of the rear wheel pair and the caster, with their lateral spans."""
    out = []
    if scale <= 100:
        out.append((shp("rear-wheel", "wheel", "body", 3.6, 0.2, ell(0, 0, K.WHEEL_R), "wheels", "rubber-wheel"), None))
        out.append((shp("rear-rim" if scale == 50 else "rear-hub", "wheel", "body", 3.7, 0.6, ell(0, 0, 0.0445 if scale == 50 else 0.02), "wheels", "abs-grey"), None))
        if scale == 50:
            out.append((shp("rear-hub", "wheel", "body", 3.8, 0.8, ell(0, 0, 0.014), "wheels", "abs-grey"), None))
            out.append((shp("caster-fork", "caster", "body", 2.85, 0.35, rect(-0.018, -0.009, 0.018, 0.017, 0.005), "wheels", "abs-grey"), None))
        out.append((shp("caster-wheel", "caster", "body", 2.9, 0.2, ell(0, 0, K.CASTER_R), "wheels", "rubber-wheel"), None))
        if scale == 50:
            out.append((shp("caster-hub", "caster", "body", 2.95, 0.7, ell(0, 0, 0.006), "wheels", "abs-grey"), None))
    else:
        out.append((shp("rear-wheel", "wheel", "body", 3.6, 0.2, ell(0, 0, K.WHEEL_R), "wheels", "rubber-wheel"), None))
    return out


def wheel_views(scale):
    """🛞 Front and plan shapes of wheels and caster (extruded discs)."""
    zl = (K.WHEEL_Z - K.WHEEL_W / 2, K.WHEEL_Z + K.WHEEL_W / 2) if scale <= 100 else (0.12, 0.15)
    s = [shp("wheel-left", "wheel", "body", 0.9, 0.2, ell(0, 0, K.WHEEL_R), "wheels", "rubber-wheel", solid=ext(*zl)), shp("wheel-right", "wheel", "body", 0.9, 0.2, ell(0, 0, K.WHEEL_R), "wheels", "rubber-wheel", solid=ext(-zl[1], -zl[0]))]
    if scale == 500:
        s = [shp("wheel-pair", "wheel", "body", 0.9, 0.2, ell(0, 0, K.WHEEL_R), "wheels", "rubber-wheel", solid=ext(-0.15, 0.15))]
    caster = []
    if scale == 50:
        caster = [shp("caster-left", "caster", "body", 0.9, 0.2, ell(0, 0, K.CASTER_R), "wheels", "rubber-wheel", solid=ext(0.012, 0.024)), shp("caster-right", "caster", "body", 0.9, 0.2, ell(0, 0, K.CASTER_R), "wheels", "rubber-wheel", solid=ext(-0.024, -0.012)), shp("caster-fork", "caster", "body", 0.85, 0.35, rect(-0.018, -0.009, 0.018, 0.017, 0.005), "wheels", "abs-grey", solid=ext(-0.013, 0.013))]
    elif scale == 100:
        caster = [shp("caster-wheel", "caster", "body", 0.9, 0.2, ell(0, 0, K.CASTER_R), "wheels", "rubber-wheel", solid=ext(-0.0195, 0.0195))]
    return s, caster


def body_side(scale):
    """🧺 Canister shapes in the side elevation."""
    o = canister_side(scale)
    s = []
    if scale >= 200:
        s.append(shp("shell", "body", "body", 3.0, 0.5, path(o["shell"]), regions=["body-shell", "lid", "exhaust"], material="abs-red"))
    else:
        s.append(shp("shell", "body", "body", 3.0, 0.5, path(o["shell"]), "body-shell", "abs-red"))
        s.append(shp("lid", "body", "body", 3.1, 0.62, path(o["lid"]), "lid", "abs-red"))
    if scale == 50:
        s.append(shp("handle", "body", "body", 3.05, 0.62, path(K.rounded_rect(-0.102, 0.236, 0.042, 0.282, 0.012), list(reversed(K.rounded_rect(-0.078, 0.2445, 0.018, 0.2645, 0.006)))), "lid", "abs-red"))
        s.append(shp("rewind-button", "body", "body", 3.08, 0.8, rect(-0.192, 0.236, -0.158, 0.249, 0.003), "lid", "abs-grey"))
        s.append(shp("bumper", "body", "body", 3.2, 0.25, path(bumper_side()), "body-shell", "abs-grey"))
        s.append(shp("hose-socket", "body", "body", 3.3, 0.25, path([(0.238, 0.093), (0.258, 0.093), (0.262, 0.0965), (0.262, 0.1535), (0.258, 0.157), (0.238, 0.157)]), "body-shell", "abs-grey"))
        s.append(shp("cuff", "body", "body", 3.35, 0.3, path([(0.262, 0.1015), (0.268, 0.1015), (0.274, 0.1035), (0.282, 0.1045), (0.282, 0.1455), (0.274, 0.1465), (0.268, 0.1485), (0.262, 0.1485)]), "hose", "abs-grey"))
        s.append(shp("exhaust-grille", "body", "body", 3.3, 0.25, rect(-0.256, 0.074, -0.245, 0.168), "exhaust", "abs-grey"))
    elif scale == 100:
        s.append(shp("handle", "body", "body", 3.05, 0.62, rect(-0.102, 0.236, 0.042, 0.282, 0.014), "lid", "abs-red"))
        s.append(shp("hose-socket", "body", "body", 3.3, 0.25, rect(0.238, 0.093, 0.262, 0.157), "body-shell", "abs-grey"))
        s.append(shp("exhaust-grille", "body", "body", 3.3, 0.25, rect(-0.26, 0.074, -0.245, 0.168), "exhaust", "abs-grey"))
    elif scale == 200:
        s.append(shp("handle", "body", "body", 2.95, 0.62, rect(-0.102, 0.236, 0.042, 0.282, 0.015), "lid", "abs-red"))
        s.append(shp("hose-socket", "body", "body", 3.3, 0.25, rect(0.232, 0.093, 0.262, 0.157), "body-shell", "abs-grey"))
    return s


def body_front(scale):
    """🧺 Canister shapes in the front elevation (x lateral, y height)."""
    o = canister_front(scale)
    s = []
    if scale >= 200:
        s.append(shp("shell", "body", "body", 1.0, 0.5, path(o["shell"]), regions=["body-shell", "lid", "exhaust"], material="abs-red"))
    else:
        s.append(shp("shell", "body", "body", 1.0, 0.5, path(o["shell"]), "body-shell", "abs-red"))
        s.append(shp("lid", "body", "body", 1.1, 0.62, path(o["lid"]), "lid", "abs-red"))
    if scale <= 200:
        s.append(shp("handle", "body", "body", 1.05 if scale < 200 else 0.95, 0.62, rect(-0.016, 0.236, 0.016, 0.282, 0.004), "lid", "abs-red"))
    if scale == 50:
        s.append(shp("rewind-button", "body", "body", 1.08, 0.8, rect(0.038, 0.236, 0.072, 0.249, 0.003), "lid", "abs-grey"))
        s.append(shp("power-button", "body", "body", 1.08, 0.8, rect(-0.072, 0.236, -0.038, 0.249, 0.003), "lid", "abs-grey"))
        a0, b0, c = M.plan_c(K.PARTING, 0.004)
        s.append(shp("bumper", "body", "body", 1.2, 0.25, rect(-b0, K.BUMPER[0], b0, K.BUMPER[1], 0.004), "body-shell", "abs-grey"))
        s.append(shp("hose-socket", "body", "body", 1.3, 0.25, ell(0, K.SOCKET[1], 0.032), "body-shell", "abs-grey"))
        s.append(shp("cuff", "body", "body", 1.35, 0.3, ell(0, K.SOCKET[1], 0.0235), "hose", "abs-grey"))
    elif scale <= 200:
        s.append(shp("hose-socket", "body", "body", 1.3, 0.25, ell(0, K.SOCKET[1], 0.032), "body-shell", "abs-grey"))
    return s


def body_plan(scale):
    """🧺 Canister shapes in the plan (x ahead, y lateral)."""
    o = canister_plan(scale)
    s = []
    if scale >= 200:
        s.append(shp("shell", "body", "body", 1.0, 0.5, path(o["shell"]), regions=["body-shell", "lid", "exhaust"], material="abs-red"))
    else:
        s.append(shp("shell", "body", "body", 1.0, 0.5, path(o["shell"]), "body-shell", "abs-red"))
        s.append(shp("lid", "body", "body", 1.1, 0.62, path(o["lid"]), "lid", "abs-red"))
    if scale <= 200:
        s.append(shp("handle", "body", "body", 1.2, 0.62, rect(-0.102, -0.016, 0.042, 0.016, 0.006), "lid", "abs-red"))
    if scale == 50:
        s.append(shp("rewind-button", "body", "body", 1.25, 0.8, ell(-0.175, 0.055, 0.017), "lid", "abs-grey"))
        s.append(shp("power-button", "body", "body", 1.25, 0.8, ell(-0.175, -0.055, 0.017), "lid", "abs-grey"))
        s.append(shp("bumper", "body", "body", 0.95, 0.25, path([(x, z) for z, x in M.bumper_ring(0.004)]), "body-shell", "abs-grey"))
        s.append(shp("cuff", "body", "body", 1.06, 0.3, rect(0.262, -0.0235, 0.282, 0.0235), "hose", "abs-grey"))
    if scale <= 100:
        s.append(shp("hose-socket", "body", "body", 1.05, 0.25, rect(0.238, -0.032, 0.262, 0.032), "body-shell", "abs-grey"))
        s.append(shp("exhaust-grille", "body", "body", 1.05, 0.25, rect(-0.256 if scale == 50 else -0.26, -0.078, -0.245, 0.078), "exhaust", "abs-grey"))
    elif scale == 200:
        s.append(shp("hose-socket", "body", "body", 1.05, 0.25, rect(0.232, -0.032, 0.262, 0.032), "body-shell", "abs-grey"))
    return s


def geometry(scale):
    """🗺️ The three viewpoints of one scale."""
    ch = chain(scale)
    elevation = [s for s, _ in ch] + body_side(scale) + [s for s, _ in wheels(scale)]
    front_chain = [dict(s, solid=sol) for s, sol in ch if sol is not None]
    wv, caster = wheel_views(scale)
    front = front_chain + wv + caster + body_front(scale)
    plan = [dict(s, z=s["z"]) for s in front_chain] + wv + body_plan(scale)
    elevation.sort(key=lambda s: s["z"])
    front.sort(key=lambda s: s["z"])
    plan.sort(key=lambda s: s["z"])
    return {"elevation": elevation, "plan": plan, "front": front}


T = K.text
DECISIONS = {
    50: {
        "silhouette": T("A canister vacuum cleaner 1.35 m long and 0.99 m high in use, 27 mm by 20 mm on the sheet: a red canister 0.50 x 0.25 m with a domed lid, carry handle and grey bumper band at the lid joint on two 0.12 m rear wheels and a twin swivel caster, a 1.5 m corrugated hose from the front socket to the handle, a telescopic wand of outer and inner tube with its locking collar, and a 0.285 m floor nozzle on an elbow and ball pivot. Front view: the canister section with lid and handle between the wheels, the socket ring with the hose cuff, and in front of it the hose, wand and the full width of the nozzle. Plan: the rounded canister with lid, handle, foot buttons, bumper rim, socket and rear grille beside the wheel strips, the hose, handle and wand on the centre line and the nozzle across.", "Ein Bodenstaubsauger, im Gebrauch 1,35 m lang und 0,99 m hoch, 27 mm mal 20 mm auf dem Blatt: ein roter Kessel von 0,50 x 0,25 m mit gewölbtem Deckel, Tragegriff und grauem Stoßband an der Deckelfuge auf zwei Hinterrädern von 0,12 m und einer Doppel-Lenkrolle, ein gewellter Schlauch von 1,5 m vom Stutzen vorne zum Handgriff, ein Teleskoprohr aus Außen- und Innenrohr mit Verriegelungsring und eine 0,285 m breite Bodendüse an Krümmer und Kugelgelenk. Vorderansicht: der Kesselquerschnitt mit Deckel und Griff zwischen den Rädern, der Stutzenring mit der Schlauchmuffe und davor Schlauch, Rohr und die ganze Düsenbreite. Draufsicht: der gerundete Kessel mit Deckel, Griff, Fußtasten, Stoßbandrand, Stutzen und Abluftgitter neben den Radstreifen, Schlauch, Griff und Rohr auf der Mittellinie und die Düse quer."),
        "retained": T("The canister outline from its plan-section table, the parting line between shell and lid, handle with finger opening, foot buttons, bumper band, socket and cuff, rear exhaust grille, tyres with dished rims and hub caps, twin caster in its fork, hose corrugation every 20 mm, the hose end fitting, handle swivel, sleeve and air slide, both wand tubes with collar, lock and highlights, the nozzle elbow, ball pivot, yoke, rubber end caps and both bristle strips.", "Der Kesselumriss aus seiner Grundriss-Schnitttabelle, die Fuge zwischen Schale und Deckel, Griff mit Grifföffnung, Fußtasten, Stoßband, Stutzen und Muffe, Abluftgitter hinten, Reifen mit Schüsselfelgen und Nabenkappen, Doppelrolle in ihrer Gabel, Schlauchwellen alle 20 mm, das Schlauchendstück, Griffgelenk, Griffhülse und Nebenluftschieber, beide Rohre mit Ring, Verriegelung und Lichtkanten, Düsenkrümmer, Kugelgelenk, Gabel, Gummi-Endkappen und beide Borstenleisten."),
        "merged": T("The two rear wheels and the two foot buttons coincide in side view; the two caster wheels are one disc there.", "Die zwei Hinterräder und die zwei Fußtasten fallen in der Seitenansicht zusammen; die zwei Lenkrollen sind dort eine Scheibe."),
        "omitted": T("Mains cable and plug, filter flap, parking hook, labels and the grille slots in the side view (they lie in the rear face). Front and plan leave out the corrugation strokes and highlights (they cannot follow the bending hose across views); the hose runs in the symmetry plane.", "Netzkabel und Stecker, Filterklappe, Parkhaken, Beschriftungen und die Gitterschlitze in der Seitenansicht (sie liegen in der Rückfläche). Vorderansicht und Draufsicht lassen die Wellenstriche und Lichtkanten weg (sie könnten dem gebogenen Schlauch nicht über die Ansichten folgen); der Schlauch verläuft in der Symmetrieebene."),
        "thinElements": T("Hose 36 mm = 0.72 mm, wand 35 and 32 mm = 0.70 and 0.64 mm and bristle strips 6 mm = 0.12 mm are drawn true; the corrugation is 0.1 mm texture strokes 0.4 mm apart.", "Schlauch 36 mm = 0,72 mm, Rohr 35 und 32 mm = 0,70 und 0,64 mm und Borstenleisten 6 mm = 0,12 mm sind wahr gezeichnet; die Wellung sind 0,1-mm-Texturstriche im Abstand 0,4 mm."),
        "hatching": T("None; the corrugation is texture.", "Keine; die Wellung ist Textur."),
        "styles": T("Canister 0.5, lid and handle 0.62, bumper, socket and grille 0.25, hose 0.35, chromed tubes 0.8 and 0.85, handle 0.3, nozzle 0.45, rubber caps and tyres 0.2, rims 0.6, brush 0.15; body pen 0.18 mm, detail 0.13 mm, texture 0.1 mm.", "Kessel 0,5, Deckel und Griff 0,62, Stoßband, Stutzen und Gitter 0,25, Schlauch 0,35, verchromte Rohre 0,8 und 0,85, Handgriff 0,3, Düse 0,45, Gummikappen und Reifen 0,2, Felgen 0,6, Bürste 0,15; Körperstift 0,18 mm, Detail 0,13 mm, Textur 0,1 mm."),
        "overlays": T("Nine regions: the thermal view paints the warm canister shell, lid and exhaust grille above cool hose, wand, handle and nozzle.", "Neun Regionen: Die Thermalansicht färbt Kesselschale, Deckel und Abluftgitter warm über kühlem Schlauch, Rohr, Griff und Düse."),
    },
    100: {
        "silhouette": T("The canister cleaner 13.5 mm long: canister with lid, handle block, socket and grille on its rear wheel and caster, ten hose tubes, handle, one wand tube with the nozzle elbow and the 0.285 m nozzle on its two bristle strips. Front and plan as at 1:50 without the small parts.", "Der Bodenstaubsauger, 13,5 mm lang: Kessel mit Deckel, Griffblock, Stutzen und Gitter auf Hinterrad und Lenkrolle, zehn Schlauchrohre, Handgriff, ein Rohr mit Düsenkrümmer und die 0,285 m breite Düse auf ihren zwei Borstenleisten. Vorderansicht und Draufsicht wie bei 1:50 ohne die kleinen Teile."),
        "retained": T("Shell and lid parting, handle, socket, grille plate, tyre with hub, caster, hose tubes, handle sleeve, wand, nozzle elbow, pivot ball, head and bristles.", "Fuge zwischen Schale und Deckel, Griff, Stutzen, Gitterplatte, Reifen mit Nabe, Lenkrolle, Schlauchrohre, Griffhülse, Rohr, Düsenkrümmer, Gelenkkugel, Kopf und Borsten."),
        "merged": T("Socket and cuff are one collar, the two wand tubes one tube, the twin caster one roller, end caps join the head.", "Stutzen und Muffe sind ein Ring, die zwei Rohre ein Rohr, die Doppelrolle eine Rolle, die Endkappen gehen im Kopf auf."),
        "omitted": T("Bumper, foot buttons, finger opening, rims, corrugation, end fitting, air slide, locking collar and yoke (all under 15 mm).", "Stoßband, Fußtasten, Grifföffnung, Felgen, Wellung, Endstück, Nebenluftschieber, Verriegelungsring und Gabel (alle unter 15 mm)."),
        "thinElements": T("Hose 36 mm and wand 35 mm are drawn true (0.36 and 0.35 mm); the 6 mm bristle strips are the thinnest filled parts.", "Schlauch 36 mm und Rohr 35 mm sind wahr gezeichnet (0,36 und 0,35 mm); die 6-mm-Borstenleisten sind die dünnsten gefüllten Teile."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:50; body pen 0.18 mm, detail 0.13 mm.", "Tonwerte wie bei 1:50; Körperstift 0,18 mm, Detail 0,13 mm."),
        "overlays": T("Nine regions as at 1:50.", "Neun Regionen wie bei 1:50."),
    },
    200: {
        "silhouette": T("The canister cleaner 6.8 mm long: one canister outline with handle and socket on the rear wheel, the hose as ten tubes, handle and wand as tubes and a nozzle block with its pivot ball.", "Der Bodenstaubsauger, 6,8 mm lang: ein Kesselumriss mit Griff und Stutzen auf dem Hinterrad, der Schlauch als zehn Rohre, Griff und Rohr als Rohre und ein Düsenblock mit Gelenkkugel."),
        "retained": T("Canister with handle and socket, rear wheels, hose, handle, wand, nozzle with pivot.", "Kessel mit Griff und Stutzen, Hinterräder, Schlauch, Griff, Rohr, Düse mit Gelenk."),
        "merged": T("Shell, lid and grille are one outline (regions body shell, lid and exhaust), nozzle head and bristles one block (regions nozzle and brush), the nozzle elbow joins the wand.", "Schale, Deckel und Gitter sind ein Umriss (Regionen Kesselschale, Deckel und Abluft), Düsenkopf und Borsten ein Block (Regionen Düse und Bürste), der Düsenkrümmer geht im Rohr auf."),
        "omitted": T("Caster (25 mm), hub, grille plate (11 mm) and all details under 30 mm.", "Lenkrolle (25 mm), Nabe, Gitterplatte (11 mm) und alle Details unter 30 mm."),
        "thinElements": T("Hose 36 mm, wand 35 mm and handle 42 mm are drawn true (0.18 to 0.21 mm); wheels 30 mm wide in front and plan.", "Schlauch 36 mm, Rohr 35 mm und Griff 42 mm sind wahr gezeichnet (0,18 bis 0,21 mm); Räder in Vorderansicht und Draufsicht 30 mm breit."),
        "hatching": T("None.", "Keine."),
        "styles": T("Canister 0.5, handle 0.62, socket 0.25, hose 0.35, wand 0.8, handle sleeve 0.3, nozzle 0.45, tyres 0.2; body pen 0.18 mm.", "Kessel 0,5, Griff 0,62, Stutzen 0,25, Schlauch 0,35, Rohr 0,8, Griffhülse 0,3, Düse 0,45, Reifen 0,2; Körperstift 0,18 mm."),
        "overlays": T("The merged outlines carry all their regions for the thermal view.", "Die zusammengelegten Umrisse tragen alle ihre Regionen für die Thermalansicht."),
    },
    500: {
        "silhouette": T("The canister cleaner 2.7 mm long: canister outline on its rear wheel, hose, handle and wand as 60 mm tubes (0.12 mm on paper inside the 0.18 mm body pen) and the nozzle block.", "Der Bodenstaubsauger, 2,7 mm lang: Kesselumriss auf dem Hinterrad, Schlauch, Griff und Rohr als 60-mm-Rohre (0,12 mm auf dem Papier im 0,18-mm-Körperstift) und der Düsenblock."),
        "retained": T("Canister, rear wheels, hose, handle, wand and nozzle.", "Kessel, Hinterräder, Schlauch, Griff, Rohr und Düse."),
        "merged": T("Shell, lid and grille are one outline, the handle sits in line with the wand, the two rear wheels are one roller, nozzle head and bristles one 75 mm block.", "Schale, Deckel und Gitter sind ein Umriss, der Griff liegt in der Flucht des Rohrs, die zwei Hinterräder sind eine Walze, Düsenkopf und Borsten ein 75-mm-Block."),
        "omitted": T("Carry handle, socket, caster, pivot ball and everything under 75 mm.", "Tragegriff, Stutzen, Lenkrolle, Gelenkkugel und alles unter 75 mm."),
        "thinElements": T("Hose, handle and wand are widened from 35 to 42 mm to 60 mm; with the 0.18 mm body pen they print as a 0.3 mm line; the wheel pair is one 0.30 m roller in front and plan.", "Schlauch, Griff und Rohr sind von 35 bis 42 mm auf 60 mm verbreitert; mit dem 0,18-mm-Körperstift drucken sie als 0,3-mm-Linie; das Radpaar ist in Vorderansicht und Draufsicht eine 0,30 m breite Walze."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:200; body pen 0.18 mm.", "Tonwerte wie bei 1:200; Körperstift 0,18 mm."),
        "overlays": T("The merged outlines carry all their regions for the thermal view.", "Die zusammengelegten Umrisse tragen alle ihre Regionen für die Thermalansicht."),
    },
}
