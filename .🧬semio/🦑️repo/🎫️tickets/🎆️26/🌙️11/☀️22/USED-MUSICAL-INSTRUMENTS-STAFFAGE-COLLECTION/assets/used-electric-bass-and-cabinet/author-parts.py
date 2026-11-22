"""🧱 The parts of the used electric bass and cabinet at each preset scale (1:50, 1:100, 1:200, 1:500): every body solid with its region, material and three drawings, the drawn traces of use, the thermal skins and the structural idealisation, each scale authored for its own smallest feature (7.5, 15, 30, 75 mm).

Imported by `author-bass.py`.
"""
import importlib.util
import math
import pathlib

HERE = pathlib.Path(__file__).parent


def _load(name, file):
    spec = importlib.util.spec_from_file_location(name, HERE / file)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


L = _load("layout", "author-layout.py")
G = L.G

FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
CASTORS = ("castor-front-l", "castor-front-r", "castor-rear-l", "castor-rear-r")


def frames():
    """🦴 Rest frames of every bone, the bass and its carrier included."""
    f = L.frames_rest(L.BASS["delta"])
    fb = L.bass_frame()
    f["bass-slide"] = G.Frame(fb.o[0], fb.o[1], fb.o[2], 0)
    f["bass-lift"] = G.Frame(fb.o[0], fb.o[1], fb.o[2], 0)
    f["bass"] = fb
    sp = L.stand_points()
    hinge = sp["top"]
    foot = sp["rear_foot"]
    deg = math.degrees(math.atan2(foot[1] - hinge[1], foot[0] - hinge[0]))
    f["stand-rear-leg"] = G.Frame(L.STAND_X + hinge[0], hinge[1], L.STAND_Z, deg)
    return f


F = frames()
SP = L.stand_points()
REAR_LEG_LENGTH = math.hypot(SP["rear_foot"][0] - SP["top"][0], SP["rear_foot"][1] - SP["top"][1])


def stand_local_from_world(p):
    """🧷 A world point in the stand frame."""
    return F["stand"].local(*p)


def bass_to_stand(s, v, w):
    """🔁 A bass-local point in the stand frame."""
    return stand_local_from_world(F["bass"].world(s, v, w))


def cabinet_parts(scale):
    """📦 The 4×10 cabinet: carcass, front frame, grille, ball corners, handles, jack plate, castors and wheels."""
    k = FEATURE[scale]
    P = []
    hw, h, d = L.CAB_W / 2, L.CAB_H, L.CAB_D
    if scale == 50:
        P.append(G.box(F, "carcass", "cabinet", "plywood-tolex", "cabinet-carcass", "body", 0.18, (0, 0, -hw), (0.408, h, hw), 0.010))
        P.append(G.extrude_yz(F, "front-frame", "cabinet", "plywood-tolex", "cabinet-carcass", "body", 0.18, [(0, -hw), (h, -hw), (h, hw), (0, hw)], 0.408, d, holes=[[(0.028, -0.282), (0.632, -0.282), (0.632, 0.282), (0.028, 0.282)]]))
        P.append(G.box(F, "grille", "cabinet", "grille-steel", "cabinet-grille", "body", 0.30, (0.404, 0.028, -0.282), (0.416, 0.632, 0.282)))
    elif scale == 100:
        P.append(G.box(F, "carcass", "cabinet", "plywood-tolex", "cabinet-carcass", "body", 0.18, (0, 0, -hw), (d, h, hw), 0.010))
        P[-1].extra["regions"] = ["cabinet-carcass", "cabinet-grille"]
    else:
        P.append(G.box(F, "carcass", "cabinet", "plywood-tolex", "cabinet-carcass", "body", 0.18, (0, 0, -hw), (d, h, hw)))
        P[-1].extra["regions"] = ["cabinet-carcass", "cabinet-grille"] + (["cabinet-hardware"] if scale == 500 else [])
    if scale in (50, 100, 200):
        size, out = 0.050, 0.003
        n = 0
        for cu in (0, d):
            for cv in (0, h):
                for cw in (-hw, hw):
                    lo = [cu - (size - out) if cu > 0 else -out, cv - (size - out) if cv > 0 else -out, cw - (size - out) if cw > 0 else -out]
                    hi = [lo[0] + size, lo[1] + size, lo[2] + size]
                    n += 1
                    tag = ("front" if cu > 0 else "rear") + "-" + ("top" if cv > 0 else "bottom") + "-" + ("l" if cw > 0 else "r")
                    P.append(G.box(F, f"corner-{tag}", "cabinet", "chrome-steel", "cabinet-hardware", "body", 0.68, lo, hi, 0.012 if scale == 50 else 0.0))
    if scale in (50, 100):
        for side, sg in (("l", 1), ("r", -1)):
            th = 0.008 if scale == 50 else 0.015
            w0, w1 = sorted((sg * hw, sg * (hw + th)))
            P.append(G.box(F, f"handle-{side}", "cabinet", "chrome-steel", "cabinet-hardware", "body", 0.55, (0.13, 0.40, w0), (0.29, 0.48, w1), 0.003))
    if scale == 50:
        P.append(G.box(F, "jack-plate", "cabinet", "chrome-steel", "cabinet-hardware", "body", 0.6, (-0.008, 0.25, -0.08), (0.0, 0.37, 0.08)))
    for c in CASTORS:
        wl = c.replace("castor", "wheel")
        cid = c.replace("castor-", "")
        if scale == 50:
            P.append(G.box(F, f"castor-plate-{cid}", c, "zinc-steel", "castors", "body", 0.62, (0.0, -0.033, -0.033), (0.014, 0.033, 0.033)))
            P.append(G.box(F, f"castor-fork-{cid}", c, "zinc-steel", "castors", "body", 0.6, (0.014, -0.042, -0.0165), (0.0635, 0.008, 0.0165)))
            P.append(G.cylinder(F, f"wheel-{cid}", wl, "pu-wheel", "castors", "body", 0.28, (0, 0, 0), "w", L.WHEEL_R, L.WHEEL_W))
        elif scale == 100:
            P.append(G.box(F, f"castor-fork-{cid}", c, "zinc-steel", "castors", "body", 0.6, (0.0, -0.042, -0.0165), (0.0635, 0.012, 0.0165)))
            P.append(G.cylinder(F, f"wheel-{cid}", wl, "pu-wheel", "castors", "body", 0.28, (0, 0, 0), "w", L.WHEEL_R, L.WHEEL_W))
        elif scale == 200:
            P.append(G.box(F, f"castor-{cid}", c, "pu-wheel", "castors", "body", 0.32, (0.0, -L.TRAIL - L.WHEEL_R, -0.0165), (L.CASTOR_H, -L.TRAIL + L.WHEEL_R, 0.0165)))
        else:
            P.append(G.box(F, f"castor-{cid}", c, "pu-wheel", "castors", "body", 0.32, (0.0, -L.TRAIL - L.WHEEL_R, -0.0375), (L.CASTOR_H, -L.TRAIL + L.WHEEL_R, 0.0375)))
    return P


def head_parts(scale):
    """🔊 The 300 W head: sleeve, recessed front and rear panels, knobs, jack, switch, feet and the strap handle."""
    P = []
    d, h, hw, ft = L.HEAD_D, L.HEAD_H, L.HEAD_W / 2, L.HEAD_FEET
    top = ft + h
    if scale == 50:
        for i, (u, w) in enumerate(((0.035, 0.235), (0.265, 0.235), (0.035, -0.235), (0.265, -0.235))):
            P.append(G.cylinder(F, f"head-foot-{i + 1}", "head", "rubber", "head-casing", "body", 0.1, (u, ft / 2, w), "v", 0.0125, ft))
        P.append(G.extrude_yz(F, "head-sleeve", "head", "plywood-tolex", "head-casing", "body", 0.2, [(ft, -hw), (top, -hw), (top, hw), (ft, hw)], 0.0, d, holes=[[(ft + 0.015, -hw + 0.015), (top - 0.015, -hw + 0.015), (top - 0.015, hw - 0.015), (ft + 0.015, hw - 0.015)]]))
        P.append(G.box(F, "front-panel", "head", "aluminium", "head-front-panel", "body", 0.72, (0.282, ft + 0.015, -hw + 0.015), (0.290, top - 0.015, hw - 0.015)))
        P.append(G.box(F, "rear-panel", "head", "chassis-steel", "head-rear-panel", "body", 0.35, (0.012, ft + 0.015, -hw + 0.015), (0.020, top - 0.015, hw - 0.015)))
        for i in range(9):
            P.append(G.cylinder(F, f"knob-{i + 1}", "head", "knob-abs", "head-front-panel", "body", 0.12, (0.2985, 0.150, -0.20 + 0.05 * i), "u", 0.0105, 0.017))
        P.append(G.cylinder(F, "input-jack", "head", "chrome-steel", "head-front-panel", "body", 0.6, (0.296, 0.085, -0.235), "u", 0.009, 0.012))
        P.append(G.box(F, "power-switch", "head", "knob-abs", "head-front-panel", "body", 0.1, (0.290, 0.075, 0.225), (0.302, 0.098, 0.245)))
        for side, sg in (("l", 1), ("r", -1)):
            w0, w1 = sorted((sg * 0.085, sg * 0.112))
            P.append(G.box(F, f"handle-bracket-{side}", "head", "chrome-steel", "head-casing", "body", 0.62, (0.135, top, w0), (0.165, top + 0.010, w1)))
        P.append(G.extrude_yz(F, "head-handle", "head", "handle-vinyl", "head-casing", "body", 0.12, arch(top + 0.010, 0.095, 0.022, 0.008), 0.138, 0.162))
    elif scale == 100:
        P.append(G.box(F, "head-box", "head", "plywood-tolex", "head-casing", "body", 0.2, (0, 0, -hw), (d, top, hw), 0.008))
        P[-1].extra["regions"] = ["head-casing", "head-front-panel", "head-rear-panel"]
        P.append(G.extrude_yz(F, "head-handle", "head", "handle-vinyl", "head-casing", "body", 0.12, arch(top, 0.10, 0.030, 0.015), 0.135, 0.165))
    elif scale == 200:
        P.append(G.box(F, "head-box", "head", "plywood-tolex", "head-casing", "body", 0.2, (0, 0, -hw), (d, top, hw)))
        P[-1].extra["regions"] = ["head-casing", "head-front-panel", "head-rear-panel"]
        P.append(G.box(F, "head-handle", "head", "handle-vinyl", "head-casing", "body", 0.12, (0.135, top, -0.095), (0.165, top + 0.030, 0.095)))
    else:
        P.append(G.box(F, "head-box", "head", "plywood-tolex", "head-casing", "body", 0.2, (0, 0, -hw), (d, top, hw)))
        P[-1].extra["regions"] = ["head-casing", "head-front-panel", "head-rear-panel"]
    return P


def arch(v0, half, rise, thick, n=10):
    """🌉 The (v, w) profile of a strap handle arching from -half to +half over the base height v0."""
    outer = [(v0 + rise * math.sin(math.pi * k / n), -half + 2 * half * k / n) for k in range(n + 1)]
    ih = half - thick
    inner = [(v0 + (rise - thick) * math.sin(math.pi * k / n), -ih + 2 * ih * k / n) for k in range(n, -1, -1)]
    return outer + inner


def bass_parts(scale):
    """🎸 The four-string bass: body, pickguard, neck, fretboard, headstock, tuners, posts, bridge, control plate, knobs, pickups, strap buttons and strings."""
    P = []
    per = {50: 6, 100: 4, 200: 3, 500: 2}[scale]
    ring = L.body_outline(per)
    half_t = {50: 0.0225, 100: 0.0225, 200: 0.0225, 500: 0.0375}[scale]
    P.append(G.prism_v(F, "bass-body", "bass", "alder-finish", "bass-body", "body", 0.42, ring, -half_t, half_t))
    if scale in (200, 500):
        P[-1].extra["regions"] = ["bass-body", "bass-hardware"] + (["bass-pickguard"] if scale == 500 else [])
    if scale in (50, 100, 200):
        P.append(G.prism_v(F, "pickguard", "bass", "pickguard-pvc", "bass-pickguard", "body", 0.28, L.PICKGUARD, -0.0248, -0.0225))
        P[-1].flags["sheet"] = True
    if scale == 50:
        P.append(G.prism_v(F, "neck", "bass", "maple", "bass-neck", "body", 0.78, L.neck_outline(L.BOARD_END_S, L.NUT_S), L.V_BOARD_BOTTOM, L.V_NECK_BACK))
        P.append(G.prism_v(F, "fretboard", "bass", "rosewood", "bass-neck", "body", 0.22, L.neck_outline(L.BOARD_END_S, L.NUT_S + 0.003), L.V_BOARD_TOP, L.V_BOARD_BOTTOM))
        P.append(G.prism_v(F, "headstock", "bass", "maple", "bass-neck", "body", 0.78, L.HEADSTOCK, L.V_HEAD_FACE, L.V_HEAD_BACK))
        for i, s in enumerate(L.TUNER_S):
            P.append(G.prism_v(F, f"tuner-{i + 1}", "bass", "chrome-hardware", "bass-hardware", "body", 0.75, rrect(s - 0.013, s + 0.013, -0.095, -0.022, 0.006), L.V_HEAD_BACK, 0.0005))
            P.append(G.prism_v(F, f"post-{i + 1}", "bass", "chrome-hardware", "bass-hardware", "body", 0.8, G.circle_points(s, L.POST_W, 0.005, 12), -0.0395, L.V_HEAD_FACE))
        P.append(G.prism_v(F, "bridge", "bass", "chrome-hardware", "bass-hardware", "body", 0.75, rrect(0.055, 0.100, -0.037, 0.037, 0.004), -0.0370, -0.0225))
        P.append(G.prism_v(F, "control-plate", "bass", "chrome-hardware", "bass-hardware", "body", 0.75, rrect(0.052, 0.128, 0.082, 0.138, 0.02), -0.0240, -0.0225))
        P[-1].flags["sheet"] = True
        for i, (s, w) in enumerate(((0.074, 0.111), (0.108, 0.112))):
            P.append(G.prism_v(F, f"bass-knob-{i + 1}", "bass", "chrome-hardware", "bass-hardware", "body", 0.7, G.circle_points(s, w, 0.010, 14), -0.0418, -0.0240))
        for i, (s0, s1, w0, w1) in enumerate(((0.200, 0.222, -0.036, -0.002), (0.178, 0.200, 0.002, 0.036))):
            P.append(G.prism_v(F, f"pickup-{i + 1}", "bass", "pickup-abs", "bass-pickguard", "body", 0.1, rrect(s0, s1, w0, w1, 0.004), -0.0373, -0.0248))
        tip = L.horn_tip(ring)
        P.append(G.prism_v(F, "strap-button-tail", "bass", "chrome-hardware", "bass-hardware", "body", 0.7, rrect(-0.013, 0.004, -0.006, 0.006, 0.002), -0.006, 0.006))
        P.append(G.prism_v(F, "strap-button-horn", "bass", "chrome-hardware", "bass-hardware", "body", 0.7, rrect(tip[0] - 0.004, tip[0] + 0.013, tip[1] - 0.006, tip[1] + 0.006, 0.002), -0.006, 0.006))
        for i, (wb, wn, r) in enumerate(((-0.0285, -0.0172, 0.00133), (-0.0095, -0.0057, 0.00108), (0.0095, 0.0057, 0.00083), (0.0285, 0.0172, 0.00057))):
            post_s = L.TUNER_S[3 - i]
            vb, vn = -0.0372, -0.0350
            part = G.Part(f"string-{i + 1}", "bass", "nickel-string", "bass-hardware", "texture", 0.6, {"kind": "capsule", "p0": [L.BRIDGE_S, vb, wb], "p1": [L.NUT_S + 0.003, vn, wn], "r0": r, "r1": r}, {}, [])
            part.flags["slender"] = True
            P.append(part)
            part = G.Part(f"string-{i + 1}-head", "bass", "nickel-string", "bass-hardware", "texture", 0.6, {"kind": "capsule", "p0": [L.NUT_S + 0.003, vn, wn], "p1": [post_s, -0.0365, L.POST_W], "r0": r, "r1": r}, {}, [])
            part.flags["slender"] = True
            P.append(part)
    elif scale == 100:
        P.append(G.prism_v(F, "neck", "bass", "maple", "bass-neck", "body", 0.78, L.neck_outline(L.BOARD_END_S, L.NUT_S + 0.003), L.V_BOARD_TOP, L.V_NECK_BACK))
        P.append(G.prism_v(F, "headstock", "bass", "maple", "bass-neck", "body", 0.78, L.HEADSTOCK, L.V_HEAD_FACE - 0.001, L.V_HEAD_BACK + 0.003))
        for i, s in enumerate(L.TUNER_S):
            P.append(G.prism_v(F, f"tuner-{i + 1}", "bass", "chrome-hardware", "bass-hardware", "body", 0.75, rrect(s - 0.013, s + 0.013, -0.095, -0.022, 0.006), L.V_HEAD_BACK + 0.003, 0.0185))
        P.append(G.prism_v(F, "bridge", "bass", "chrome-hardware", "bass-hardware", "body", 0.75, rrect(0.055, 0.100, -0.037, 0.037, 0.004), -0.0375, -0.0225))
        P.append(G.prism_v(F, "control-plate", "bass", "chrome-hardware", "bass-hardware", "body", 0.75, rrect(0.052, 0.128, 0.082, 0.138, 0.02), -0.0240, -0.0225))
        P[-1].flags["sheet"] = True
        for i, (s, w) in enumerate(((0.074, 0.111), (0.108, 0.112))):
            P.append(G.prism_v(F, f"bass-knob-{i + 1}", "bass", "chrome-hardware", "bass-hardware", "body", 0.7, G.circle_points(s, w, 0.010, 10), -0.0418, -0.0240))
        P.append(G.prism_v(F, "pickup", "bass", "pickup-abs", "bass-pickguard", "body", 0.1, rrect(0.178, 0.222, -0.036, 0.036, 0.004), -0.0398, -0.0248))
    elif scale == 200:
        planform = neck_and_head(0.0, 1)
        P.append(G.prism_v(F, "neck", "bass", "maple", "bass-neck", "body", 0.7, planform, -0.0375, -0.0075))
    else:
        planform = neck_and_head(0.0375, 2)
        P.append(G.prism_v(F, "neck", "bass", "maple", "bass-neck", "body", 0.7, planform, -0.0505, 0.0245))
    return P


def neck_and_head(min_half, n):
    """🪵 The planform of neck and headstock as one part (the tuner keys taken into the headstock), never narrower than `min_half` on each side, for the coarse scales."""
    s0, s1 = L.BOARD_END_S, L.NUT_S
    side = [(s0 + (s1 - s0) * k / n, max(min_half, L.neck_half(s0 + (s1 - s0) * k / n))) for k in range(n + 1)]
    if min_half:
        head = [(1.165, min_half), (1.165, -0.095), (0.965, -0.095)]
    else:
        head = [(1.02, 0.019), (1.13, 0.032), (1.165, 0.010), (1.150, -0.095), (0.972, -0.095), (0.960, -0.030)]
    back = [(s, -h) for s, h in reversed(side)]
    return G.ccw(side + head + back)


def rrect(s0, s1, w0, w1, r, n=3):
    """▢ A rounded rectangle in (s, w)."""
    r = min(r, (s1 - s0) / 2, (w1 - w0) / 2)
    pts = []
    for cs, cw, a0 in ((s1 - r, w1 - r, 0), (s0 + r, w1 - r, 90), (s0 + r, w0 + r, 180), (s1 - r, w0 + r, 270)):
        for k in range(n + 1):
            a = math.radians(a0 + 90 * k / n)
            pts.append((cs + r * math.cos(a), cw + r * math.sin(a)))
    return pts


def stand_parts(scale):
    """🪜 The folding A-frame bass stand: frame tubes, cross tube, post, cradle arms with foam and lips, the foam yoke, the folding rear leg and rubber caps."""
    P = []
    sp = SP
    foot, top, root = sp["foot"], sp["top"], sp["arm_root"]
    yk = sp["yoke"]
    tr = L.TUBE_R
    thick = {50: tr, 100: tr, 200: tr, 500: tr}[scale]
    for side, sg in (("l", 1), ("r", -1)):
        part = G.tube(F, f"frame-tube-{side}", "stand", "stand-steel", "stand", "body", 0.15, (foot[0], foot[1], sg * L.ARM_W), (top[0], top[1], sg * L.ARM_W), thick)
        if scale in (200, 500):
            part.flags["slender"] = True
        P.append(part)
    cross_r = {50: tr, 100: tr, 200: 0.015, 500: 0.0375}[scale]
    P.append(G.tube(F, "cross-tube", "stand", "stand-steel", "stand", "body", 0.15, (top[0], top[1], -L.ARM_W), (top[0], top[1], L.ARM_W), cross_r))
    leg_end = (REAR_LEG_LENGTH, 0, 0)
    part = G.tube(F, "rear-leg", "stand-rear-leg", "stand-steel", "stand", "body", 0.15, (0, 0, 0), leg_end, tr)
    if scale in (200, 500):
        part.flags["slender"] = True
    P.append(part)
    if scale in (50, 100, 200):
        post_r = {50: tr, 100: tr, 200: 0.015}[scale]
        P.append(G.tube(F, "post", "stand", "stand-steel", "stand", "body", 0.15, (top[0], top[1], yk[2]), (yk[0], yk[1] - 0.004, yk[2]), post_r))
        arm_r = {50: tr, 100: L.FOAM_R, 200: 0.015}[scale]
        foam_r = {50: L.FOAM_R, 100: L.FOAM_R, 200: 0.015}[scale]
        ay = L.ARM_Y + L.FOAM_R - foam_r
        for side, sg in (("l", 1), ("r", -1)):
            z = sg * L.ARM_W
            if scale == 50:
                P.append(G.tube(F, f"arm-{side}", "stand", "stand-steel", "stand", "body", 0.15, (root[0], ay, z), (sp["lip_x"], ay, z), tr))
                P.append(G.tube(F, f"arm-foam-{side}", "stand", "pe-foam", "stand", "body", 0.22, (sp["contact_x"][0] - 0.030, ay, z), (sp["lip_x"] - 0.009, ay, z), foam_r))
                P.append(G.tube(F, f"lip-{side}", "stand", "pe-foam", "stand", "body", 0.22, (sp["lip_x"], ay, z), (sp["lip_x"], ay + 0.040, z), 0.009))
            else:
                P.append(G.tube(F, f"arm-{side}", "stand", "pe-foam", "stand", "body", 0.22, (root[0], ay, z), (sp["lip_x"], ay, z), arm_r))
                if scale == 100:
                    P.append(G.tube(F, f"lip-{side}", "stand", "pe-foam", "stand", "body", 0.22, (sp["lip_x"], ay, z), (sp["lip_x"], ay + 0.040, z), 0.009))
        yoke_r = {50: L.FOAM_R, 100: L.FOAM_R, 200: 0.015}[scale]
        yh = sp["yoke_half"] + L.FOAM_R
        P.append(G.tube(F, "yoke", "stand", "pe-foam", "stand", "body", 0.22, (yk[0], yk[1], yk[2] - yh), (yk[0], yk[1], yk[2] + yh), yoke_r))
        if scale in (50, 100):
            tip = sp["yoke_tip"]
            for side, sg in (("l", 1), ("r", -1)):
                P.append(G.tube(F, f"yoke-arm-{side}", "stand", "pe-foam", "stand", "body", 0.22, (yk[0], yk[1], yk[2] + sg * yh), (tip[0], tip[1], tip[2] + sg * yh), 0.010))
    if scale in (50, 100):
        dx, dy = top[0] - foot[0], top[1] - foot[1]
        n = math.hypot(dx, dy)
        for side, sg in (("l", 1), ("r", -1)):
            a = (foot[0] + dx / n * (L.CAP_R - tr), L.CAP_R, sg * L.ARM_W)
            b = (a[0] + dx / n * 0.025, a[1] + dy / n * 0.025, a[2])
            P.append(G.tube(F, f"foot-cap-{side}", "stand", "rubber", "stand", "body", 0.1, a, b, L.CAP_R))
        fr = F["stand-rear-leg"]
        end = fr.world(REAR_LEG_LENGTH, 0, 0)
        ux, uy = fr.c, fr.s
        a_w = (end[0] - ux * 0.0, L.CAP_R, end[2])
        a_l = fr.local(*a_w)
        b_l = (a_l[0] - 0.025, a_l[1], 0)
        P.append(G.tube(F, "rear-cap", "stand-rear-leg", "rubber", "stand", "body", 0.1, a_l, b_l, L.CAP_R))
    return P


def all_parts(scale):
    """🧩 Every body part of a scale, in drawing order."""
    return cabinet_parts(scale) + head_parts(scale) + stand_parts(scale) + bass_parts(scale)
