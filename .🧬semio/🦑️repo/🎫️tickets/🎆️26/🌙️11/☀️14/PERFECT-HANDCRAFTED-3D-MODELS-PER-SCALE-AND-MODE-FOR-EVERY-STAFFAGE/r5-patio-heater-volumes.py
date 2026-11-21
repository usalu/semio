"""🔥 Round 3 propane patio heater: body, thermal and structure of every authored scale, every solid stated by hand from the kit's real geometry.

@see r5-patio-heater-kit.py — dimensions, profiles and writers
"""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-patio-heater-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
_spec2 = importlib.util.spec_from_file_location("texts", os.path.join(T, "r5-patio-heater-texts.py"))
TX = importlib.util.module_from_spec(_spec2)
_spec2.loader.exec_module(TX)

PI = math.pi
THREE = {"count": 3, "rotationAxis": [1, 0, 0], "rotationRad": K.r6(2 * PI / 3), "origin": [0, 0, 0]}
EIGHT = {"count": 8, "rotationAxis": [1, 0, 0], "rotationRad": K.r6(PI / 4), "origin": [0, 0, 0]}


def wheel_parts(scale, side):
    """🛞 Tyre and hub of one rear wheel about world Z (one solid tyre at 1:100)."""
    sign = 1 if side == "left" else -1
    origin = K.local("base", K.WHEEL_X, K.WHEEL_Y, sign * K.WHEEL_Z)
    hw = K.WHEEL_W / 2
    if scale == 100:
        tyre = [[-hw, 0.0], [-hw, 0.040], [-hw + 0.004, K.WHEEL_R], [hw - 0.004, K.WHEEL_R], [hw, 0.040], [hw, 0.0]]
        return [K.solid("wheel-%s" % side, "revolve", "base", "plastic-black", "body", 0.9, profile=tyre, axis=[0, 0, 1], origin=origin)]
    tyre = K.loop([[-hw, 0.029], [-hw, 0.040], [-hw + 0.004, K.WHEEL_R], [hw - 0.004, K.WHEEL_R], [hw, 0.040], [hw, 0.029]])
    hub = [[-0.014, 0.0], [-0.014, 0.012], [-0.011, 0.014], [-0.011, 0.0295], [0.011, 0.0295], [0.011, 0.014], [0.014, 0.012], [0.014, 0.0]]
    return [
        K.solid("tyre-%s" % side, "revolve", "base", "plastic-black", "body", 0.9, profile=tyre, axis=[0, 0, 1], origin=origin),
        K.solid("hub-%s" % side, "revolve", "base", "stainless-brushed", "detail", 0.4, profile=hub, axis=[0, 0, 1], origin=origin),
    ]


def axle(scale):
    """➖ The rear wheel axle across the heater (12 mm; slender 15 mm at 1:100)."""
    p0 = K.local("base", K.WHEEL_X, K.WHEEL_Y, -(K.WHEEL_Z + 0.012))
    p1 = K.local("base", K.WHEEL_X, K.WHEEL_Y, K.WHEEL_Z + 0.012)
    r = 0.006 if scale == 50 else 0.0075
    s = K.solid("axle", "capsule", "base", "stainless-brushed", "detail", 0.4, p0=p0, p1=p1, r0=r, r1=r)
    if scale == 100:
        s["slender"] = True
    return s


def bracket(side):
    """📎 Pressed 8 mm axle bracket welded to the rear rim of the base pan."""
    sign = 1 if side == "left" else -1
    cu, cv = K.WHEEL_Y, -K.WHEEL_X
    prof = [[0.016, 0.205], [0.016, cv - 0.004]]
    prof += [[K.r6(cu + 0.022 * math.sin(math.radians(a))), K.r6(cv + 0.022 * math.cos(math.radians(a)))] for a in (-60, -20, 20, 60, 100)]
    prof += [[0.074, 0.205]]
    lo, hi = sorted([sign * 0.09, sign * 0.098])
    return K.solid("bracket-%s" % side, "extrude", "base", "powder-steel", "detail", 0.55, profile=prof, plane="xy", **{"from": K.r6(lo), "to": K.r6(hi)})


def door_parts(scale):
    """🚪 Access panel over the bottle opening (facing -Z) with louvre rows and latch at 1:50; a revolve sweep starts at +w and turns towards +v."""
    start = PI - K.DOOR_SWEEP / 2
    out = [K.solid("door", "revolve", "base", "powder-steel", "detail", 0.55,
                   profile=K.ring(K.DOOR_Y0, K.DOOR_Y1, K.HOUSING_R - 0.0005, K.HOUSING_R + K.DOOR_PROUD), axis=[1, 0, 0], origin=[0.0, 0.0, 0.0],
                   sweepRad=K.r6(K.DOOR_SWEEP), transform={"rotation": K.quat_u(start)}, texture=True)]
    if scale == 50:
        n = 0
        for y0 in (0.135, 0.152, 0.598, 0.615):
            for k in (-1, 0, 1):
                a = PI + k * 0.27 - 0.1
                out.append(K.solid("louvre-%d" % n, "revolve", "base", "powder-steel", "texture", 0.15,
                                   profile=K.ring(y0, y0 + 0.008, K.HOUSING_R + K.DOOR_PROUD - 0.0003, K.HOUSING_R + K.DOOR_PROUD + 0.0015), axis=[1, 0, 0], origin=[0.0, 0.0, 0.0],
                                   sweepRad=0.2, transform={"rotation": K.quat_u(a)}, texture=True))
                n += 1
        z = -math.sqrt((K.HOUSING_R + K.DOOR_PROUD) ** 2 - 0.11 ** 2)
        out.append(K.solid("door-latch", "revolve", "base", "stainless-brushed", "detail", 0.3,
                           profile=[[-0.002, 0.0], [-0.002, 0.011], [0.008, 0.011], [0.012, 0.007], [0.012, 0.0]], axis=[0, 0, -1],
                           origin=K.local("base", 0.11, 0.38, z)))
    return out


def housing_loop(wall, lid_n):
    """🛢️ One closed profile of pan, wall, lid and socket with a sheet `wall` thick (1:200)."""
    outer = [[K.DISH_TOP, K.HOUSING_R], [K.WALL_TOP, K.HOUSING_R]] + K.lid_outer(lid_n)[1:]
    inner = K.offset_in(outer, wall)
    inner[0] = [K.DISH_TOP, K.r6(K.HOUSING_R - wall)]
    pts = [[0.0, 0.0], [0.0, K.DISH_R], [K.DISH_TOP, K.DISH_R]] + outer[:-1] + [[0.7299, K.SOCKET_R], [0.755, K.SOCKET_R], [0.755, K.POLE_R], [K.r6(outer[-1][0] - wall), K.POLE_R]]
    pts += list(reversed(inner[:-1])) + [[K.DISH_TOP, 0.0]]
    return K.loop(pts)


def housing_parts(scale):
    """🧱 Base pan, cylinder wall, pressed lid and pole socket of the bottle housing."""
    if scale <= 100:
        return [
            K.revolve_up("dish", "base", "powder-steel", "body", 0.6, K.dish_profile(0.009 if scale == 50 else 0.015)),
            K.revolve_up("housing-wall", "base", "powder-steel", "body", 0.6, K.ring(K.DISH_TOP, K.WALL_TOP, K.HOUSING_R - K.SHEET, K.HOUSING_R)),
            K.revolve_up("housing-lid", "base", "powder-steel", "body", 0.6, K.lid_shell(K.SHEET, 12 if scale == 50 else 8)),
            K.revolve_up("socket", "base", "powder-steel", "body", 0.5,
                         K.loop([[0.712, K.POLE_R], [0.712, 0.044], [0.722, K.SOCKET_R], [0.750, K.SOCKET_R], [0.755, 0.046], [0.755, K.POLE_R]]) if scale == 50
                         else K.ring(0.712, 0.755, K.POLE_R, K.SOCKET_R)),
        ]
    if scale == 200:
        return [K.revolve_up("housing", "base", "powder-steel", "body", 0.6, housing_loop(K.SHEET, 6))]
    return [K.revolve_up("housing", "base", "powder-steel", "body", 0.6,
                         K.loop([[0.0, 0.0], [0.0, 0.225], [0.695, 0.225], [0.718, 0.21], [0.73, 0.18], [0.735, 0.075], [0.755, 0.0375], [0.66, 0.0375], [0.655, 0.15], [0.075, 0.15], [0.075, 0.0]]))]


def pole_top():
    """📏 Top of the post inside the head sleeve, in pole coordinates."""
    return K.r6(K.HEAD_Y - K.POLE_Y + 0.03)


def pole_parts(scale):
    """🦯 The 60 x 1.5 mm stainless post (three sections at 1:50) between the lid socket and the head sleeve."""
    top = pole_top()
    if scale <= 100:
        out = [K.revolve_up("pole", "pole", "stainless-brushed", "body", 0.45, K.ring(-0.018, top, K.POLE_R - K.POLE_T, K.POLE_R))]
        if scale == 50:
            for i, u in enumerate((0.30, 0.61)):
                out.append(K.revolve_up("pole-coupling-%d" % i, "pole", "stainless-brushed", "detail", 0.3, K.ring(u, u + 0.03, 0.0298, 0.0325)))
        return out
    radius = K.POLE_R if scale == 200 else 0.0375
    return [K.revolve_up("pole", "pole", "stainless-brushed", "body", 0.45, [[-0.018, 0.0], [-0.018, radius], [top, radius], [top, 0.0]])]


def head_parts(scale):
    """🔥 The head assembly: valve housing, emitter screen with bands, guard rods and cap, knob boss and igniter."""
    p0, p1 = K.rod_points()
    if scale == 50:
        return [
            K.revolve_up("valve-housing", "burner-head", "stainless-brushed", "body", 0.45, K.head_profile(50)),
            K.revolve_up("emitter-screen", "burner-head", "stainless-hot", "body", 0.8, K.ring(K.DECK_U, K.SCREEN_TOP_U, K.SCREEN_R - 0.0015, K.SCREEN_R)),
            K.revolve_up("screen-band-low", "burner-head", "stainless-hot", "detail", 0.7, K.ring(K.DECK_U, K.DECK_U + 0.012, K.SCREEN_R - 0.0005, K.BAND_R)),
            K.revolve_up("screen-band-high", "burner-head", "stainless-hot", "detail", 0.7, K.ring(K.SCREEN_TOP_U - 0.012, K.SCREEN_TOP_U, K.SCREEN_R - 0.0005, K.BAND_R)),
            K.revolve_up("emitter-cap", "burner-head", "stainless-hot", "body", 0.7, K.cap_profile()),
            K.solid("guard-rod", "capsule", "burner-head", "stainless-hot", "detail", 0.6, p0=p0, p1=p1, r0=0.004, r1=0.004, repeat=EIGHT),
            K.solid("knob-boss", "revolve", "burner-head", "stainless-brushed", "detail", 0.35,
                    profile=[[0.074, 0.0], [0.074, 0.026], [0.086, 0.026], [0.089, 0.023], [0.089, 0.0]], axis=[0, -1, 0], origin=[0.082, 0.0, 0.0]),
            K.solid("igniter", "revolve", "burner-head", "plastic-black", "detail", 0.2,
                    profile=[[0.076, 0.0], [0.076, 0.012], [0.089, 0.012], [0.089, 0.0075], [0.097, 0.0075], [0.098, 0.0]],
                    axis=[0, K.r6(-math.cos(math.radians(40))), K.r6(math.sin(math.radians(40)))], origin=[0.082, 0.0, 0.0]),
        ]
    if scale == 100:
        emitter = K.loop([[K.DECK_U, K.SCREEN_R - 0.0015], [K.DECK_U, K.SCREEN_R], [K.r6(K.CAP_TOP_U - 0.006), K.SCREEN_R], [K.r6(K.CAP_TOP_U - 0.006), K.CAP_R],
                          [K.CAP_TOP_U, K.r6(K.CAP_R - 0.004)], [K.CAP_TOP_U, 0.0], [K.r6(K.CAP_TOP_U - 0.015), 0.0], [K.r6(K.CAP_TOP_U - 0.015), K.SCREEN_R - 0.0015]])
        return [
            K.revolve_up("valve-housing", "burner-head", "stainless-brushed", "body", 0.45, K.head_profile(100)),
            K.solid("knob-boss", "revolve", "burner-head", "stainless-brushed", "detail", 0.35,
                    profile=[[0.074, 0.0], [0.074, 0.026], [0.089, 0.026], [0.089, 0.0]], axis=[0, -1, 0], origin=[0.082, 0.0, 0.0]),
            K.revolve_up("emitter", "burner-head", "stainless-hot", "body", 0.8, emitter),
            K.solid("guard-rod", "capsule", "burner-head", "stainless-hot", "detail", 0.6, p0=p0, p1=p1, r0=0.004, r1=0.004, slender=True, repeat=EIGHT),
        ]
    if scale == 200:
        prof = [[0.0, 0.0], [0.0, 0.036], [0.040, 0.085], [0.118, 0.085], [0.136, K.DECK_R], [K.DECK_U, K.SCREEN_R], [K.r6(K.CAP_TOP_U - 0.015), K.SCREEN_R],
                [K.r6(K.CAP_TOP_U - 0.015), K.CAP_R], [K.CAP_TOP_U, K.CAP_R], [K.CAP_TOP_U, 0.0], [K.r6(K.CAP_TOP_U - 0.03), 0.0],
                [K.r6(K.CAP_TOP_U - 0.03), K.r6(K.SCREEN_R - 0.015)], [K.r6(K.DECK_U + 0.03), K.r6(K.SCREEN_R - 0.015)], [K.r6(K.DECK_U + 0.03), 0.0]]
        return [K.revolve_up("head", "burner-head", "stainless-hot", "body", 0.8, K.loop(prof))]
    prof = [[0.0, 0.0], [0.0, 0.0375], [0.040, 0.085], [0.12, 0.085], [0.14, K.SCREEN_R], [K.CAP_TOP_U, K.SCREEN_R], [K.CAP_TOP_U, 0.10], [K.r6(K.CAP_TOP_U + 0.15), 0.10], [K.r6(K.CAP_TOP_U + 0.15), 0.0],
            [K.r6(K.CAP_TOP_U - 0.075), 0.0], [K.r6(K.CAP_TOP_U - 0.075), K.r6(K.SCREEN_R - 0.0375)], [K.r6(K.DECK_U + 0.04), K.r6(K.SCREEN_R - 0.0375)], [K.r6(K.DECK_U + 0.04), 0.0]]
    return [K.revolve_up("head", "burner-head", "stainless-hot", "body", 0.8, K.loop(prof))]


def knob_parts(scale):
    """🎛️ Control knob on the valve stem (with its grip bar at 1:50; the 75 mm size floor at 1:500)."""
    half = K.KNOB_HALF[scale]
    prof = [[half[0][0], 0.0]] + half
    out = [K.solid("knob", "revolve", "knob", "plastic-black", "body", 0.2, profile=prof, axis=[1, 0, 0], origin=[0.0, 0.0, 0.0])]
    if scale == 50:
        out.append(K.solid("knob-grip", "box", "knob", "plastic-black", "detail", 0.3, size=[0.014, 0.042, 0.0085], cornerRadius=0.003, transform={"translation": [0.027, 0.0, 0.0]}))
    return out


def reflector_parts(scale):
    """☂️ The one-piece aluminised dome on three spacer studs, with rolled rim, finial and dome nuts."""
    spacer_top = K.r6(K.dome_outer(K.SPACER_R) - K.DISH_T - 0.004)
    if scale == 50:
        return [
            K.revolve_up("dome", "reflector", "alu-reflector", "body", 0.5, K.dome_shell(K.DISH_T, 28)),
            K.revolve_up("dome-bead", "reflector", "alu-reflector", "body", 0.45, K.loop(K.arc(K.RIM_U - 0.003, K.DOME_R, 0.006, 0, 350, 12)[:-1])),
            K.revolve_up("finial", "reflector", "alu-reflector", "detail", 0.25, K.finial_profile(6)),
            K.revolve_up("dome-rib", "reflector", "alu-reflector", "texture", 0.4,
                         K.loop([[K.r6(K.dome_outer(K.RIB_R - 0.006) - 0.0005), K.RIB_R - 0.006], [K.r6(K.dome_outer(K.RIB_R - 0.003) + 0.003), K.RIB_R - 0.003],
                                 [K.r6(K.dome_outer(K.RIB_R + 0.003) + 0.003), K.RIB_R + 0.003], [K.r6(K.dome_outer(K.RIB_R + 0.006) - 0.0005), K.RIB_R + 0.006]]), texture=True),
            K.solid("spacer", "capsule", "reflector", "stainless-hot", "detail", 0.4, p0=[0.004, -K.SPACER_R, 0.0], p1=[spacer_top, -K.SPACER_R, 0.0], r0=0.006, r1=0.006, repeat=THREE),
            K.solid("dome-nut", "revolve", "reflector", "stainless-brushed", "detail", 0.3,
                    profile=[[-0.004, 0.0], [-0.004, 0.011], [0.003, 0.011], [0.004, 0.008], [0.012, 0.008], [0.013, 0.0]], axis=[1, 0, 0],
                    origin=[K.r6(K.dome_outer(K.SPACER_R)), -K.SPACER_R, 0.0], repeat=THREE),
        ]
    if scale == 100:
        return [
            K.revolve_up("dome", "reflector", "alu-reflector", "body", 0.5, K.dome_shell(K.DISH_T, 18, 0.400)),
            K.revolve_up("finial", "reflector", "alu-reflector", "detail", 0.25, K.finial_profile(3)),
            K.solid("spacer", "capsule", "reflector", "stainless-hot", "detail", 0.4, p0=[0.0075, -K.SPACER_R, 0.0], p1=[K.r6(spacer_top - 0.002), -K.SPACER_R, 0.0], r0=0.0075, r1=0.0075, repeat=THREE),
        ]
    if scale == 200:
        outer = [[K.r6(K.RIM_U - 0.004), 0.400]] + K.dome_curve(0.400, 0.05, 10) + [[u, r] for u, r in K.finial_profile(3)[2:]]
        inner = K.dome_curve(0.0, 0.400, 10, -K.DISH_T)
        inner[0][1] = 0.0
        inside = lambda r: K.r6(K.dome_outer(r) - K.DISH_T + 0.0003)
        studs = K.loop([[-0.002, 0.089], [-0.002, 0.101], [inside(0.101), 0.101], [inside(0.089), 0.089]])
        return [K.revolve_up("dome", "reflector", "alu-reflector", "body", 0.5, K.loop(outer + inner[:-1] + [[K.r6(K.RIM_U - 0.004), 0.394]])),
                K.revolve_up("stud-ring", "reflector", "stainless-hot", "detail", 0.4, studs)]
    top = K.FINIAL_TOP_U
    prof = [[K.r6(K.RIM_U - 0.004), 0.400], [K.r6(K.RIM_U + 0.045), 0.33], [K.r6(K.RIM_U + 0.10), 0.20], [K.r6(K.RIM_U + 0.14), 0.08], [top, 0.0],
            [K.r6(top - 0.04), 0.0], [K.r6(K.RIM_U + 0.10), 0.12], [K.r6(K.RIM_U + 0.05), 0.25], [K.r6(K.RIM_U - 0.004), 0.37]]
    return [K.revolve_up("dome", "reflector", "alu-reflector", "body", 0.5, K.loop(prof))]


def bottle_profile():
    """🧪 Outer profile (u, r) of the 11 kg EN 1442 propane cylinder from the bottom head to the valve boss."""
    return [[0.030, 0.0], [0.031, 0.040], [0.036, 0.075], [0.045, 0.102], [0.058, 0.124], [0.075, 0.140], [0.095, K.BOTTLE_R], [0.430, K.BOTTLE_R],
            [0.455, 0.146], [0.478, 0.132], [0.497, 0.110], [0.511, 0.082], [0.519, 0.055], [0.523, 0.030], [0.525, 0.0]]


def bottle_parts(scale):
    """🧯 The propane bottle: foot ring, welded body, valve guard collar and valve (hidden in the housing, thermal only)."""
    if scale == 50:
        return [
            K.revolve_up("bottle-foot", "bottle", "bottle-steel", "detail", 0.4, K.ring(0.0, 0.058, 0.117, 0.123)),
            K.revolve_up("bottle-body", "bottle", "bottle-steel", "body", 0.45, bottle_profile()),
            K.revolve_up("bottle-collar", "bottle", "bottle-steel", "detail", 0.4, K.ring(0.49, 0.58, 0.087, 0.092)),
            K.revolve_up("bottle-valve", "bottle", "stainless-brushed", "detail", 0.3, [[0.515, 0.0], [0.515, 0.016], [0.552, 0.016], [0.552, 0.028], [0.562, 0.028], [0.562, 0.0]]),
        ]
    if scale == 100:
        return [
            K.revolve_up("bottle-body", "bottle", "bottle-steel", "body", 0.45, [[0.0, 0.0], [0.0, 0.123], [0.058, 0.124], [0.075, 0.140], [0.095, K.BOTTLE_R], [0.430, K.BOTTLE_R], [0.478, 0.132], [0.511, 0.082], [0.523, 0.030], [0.525, 0.0]]),
            K.revolve_up("bottle-collar", "bottle", "bottle-steel", "detail", 0.4, K.ring(0.49, 0.58, 0.077, 0.092)),
        ]
    if scale == 200:
        return [K.revolve_up("bottle-body", "bottle", "bottle-steel", "body", 0.45, [[0.0, 0.0], [0.0, 0.125], [0.07, K.BOTTLE_R], [0.44, K.BOTTLE_R], [0.51, 0.09], [0.58, 0.09], [0.58, 0.0]])]
    return [K.revolve_up("bottle-body", "bottle", "bottle-steel", "body", 0.45, [[0.0, 0.0], [0.0, K.BOTTLE_R], [0.50, K.BOTTLE_R], [0.58, 0.075], [0.58, 0.0]])]


def emitters(scale):
    """💨 The flue outlet patch on the emitter cap and the plume patch on the dome crown (D2), one feature thick."""
    f = K.FEATURE[scale]
    flue = K.revolve_up("th-flue-gas-outlet", "burner-head", "stainless-hot", "detail", 0.9, [[K.CAP_TOP_U, 0.0], [K.CAP_TOP_U, 0.10], [K.r6(K.CAP_TOP_U + f), 0.10], [K.r6(K.CAP_TOP_U + f), 0.0]],
                        tags=["flue-gas"], region="emitter-head")
    outer = K.dome_curve(0.30, 0.0, 10, f)
    base = K.dome_curve(0.0, 0.30, 10)
    outer[-1][1] = 0.0
    base[0][1] = 0.0
    plume = K.revolve_up("th-dome-plume-outlet", "reflector", "alu-reflector", "detail", 0.9, K.loop(outer + base), tags=["dome-plume"], region="reflector")
    return [flue, plume]


def body(scale):
    """🧍 The body list of a scale."""
    out = housing_parts(scale)
    if scale <= 100:
        out += wheel_parts(scale, "left") + wheel_parts(scale, "right") + [axle(scale)]
        if scale == 50:
            out += [bracket("left"), bracket("right")]
        out += door_parts(scale)
    return out + pole_parts(scale) + head_parts(scale) + knob_parts(scale) + reflector_parts(scale)


def envelope(sid, bone, material, region, profile):
    """🫙 A closed outer envelope of a hollow body part: the outer area the air reaches (C31)."""
    return K.revolve_up(sid, bone, material, "body", 0.5, profile, region=region)


def thermal(scale, solids):
    """🌡️ The thermal list: outer envelopes of the closed housings, references to every open or solid body surface, the hidden bottle and the two outlets."""
    ids = {s["id"] for s in solids}
    out = []
    lid = K.lid_outer(8)
    top = pole_top()
    if scale <= 200:
        housing = [[K.DISH_TOP if scale <= 100 else 0.0, 0.0]] + ([] if scale <= 100 else [[0.0, K.DISH_R], [K.DISH_TOP, K.DISH_R]]) + [[K.DISH_TOP, K.HOUSING_R], [K.WALL_TOP, K.HOUSING_R]] + lid[1:-1]
        housing += [[0.7299, K.SOCKET_R], [0.755, K.SOCKET_R], [0.755, 0.0]]
        out.append(envelope("th-housing", "base", "powder-steel", "base-housing", housing))
        if scale <= 100:
            out.append(K.ref("th-dish", "dish", region="base-housing"))
    else:
        out.append(envelope("th-housing", "base", "powder-steel", "base-housing", [[0.0, 0.0], [0.0, 0.225], [0.70, 0.225], [0.73, 0.15], [0.755, 0.0375], [0.755, 0.0]]))
    for sid in ("tyre-left", "tyre-right", "hub-left", "hub-right", "wheel-left", "wheel-right", "axle", "bracket-left", "bracket-right", "door-latch"):
        if sid in ids:
            out.append(K.ref("th-" + sid, sid, region="base-housing"))
    for s in bottle_parts(scale):
        out.append(K.tag(s, id="th-" + s["id"], region="gas-bottle"))
    if scale <= 100:
        out.append(envelope("th-pole", "pole", "stainless-brushed", "pole-stainless", [[-0.018, 0.0], [-0.018, K.POLE_R], [top, K.POLE_R], [top, 0.0]]))
        out += [K.ref("th-" + sid, sid, region="pole-stainless") for sid in ("pole-coupling-0", "pole-coupling-1", "valve-housing") if sid in ids]
        out.append(envelope("th-emitter", "burner-head", "stainless-hot", "emitter-head", [[K.DECK_U, 0.0], [K.DECK_U, K.SCREEN_R], [K.SCREEN_TOP_U, K.SCREEN_R], [K.SCREEN_TOP_U, 0.0]]))
        out += [K.ref("th-" + sid, sid, region="emitter-head") for sid in ("screen-band-low", "screen-band-high", "emitter-cap", "guard-rod") if sid in ids]
        if scale == 100:
            out.append(envelope("th-emitter-cap", "burner-head", "stainless-hot", "emitter-head", [[K.SCREEN_TOP_U, 0.0], [K.SCREEN_TOP_U, K.SCREEN_R], [K.r6(K.CAP_TOP_U - 0.006), K.SCREEN_R], [K.r6(K.CAP_TOP_U - 0.006), K.CAP_R], [K.CAP_TOP_U, K.r6(K.CAP_R - 0.004)], [K.CAP_TOP_U, 0.0]]))
    else:
        out.append(K.ref("th-pole", "pole", region="pole-stainless"))
        if scale == 200:
            head = [[0.0, 0.0], [0.0, 0.036], [0.040, 0.085], [0.118, 0.085], [0.136, K.DECK_R], [K.DECK_U, K.SCREEN_R], [K.r6(K.CAP_TOP_U - 0.015), K.SCREEN_R],
                    [K.r6(K.CAP_TOP_U - 0.015), K.CAP_R], [K.CAP_TOP_U, K.CAP_R], [K.CAP_TOP_U, 0.0]]
        else:
            head = [[0.0, 0.0], [0.0, 0.0375], [0.040, 0.085], [0.12, 0.085], [0.14, K.SCREEN_R], [K.CAP_TOP_U, K.SCREEN_R], [K.CAP_TOP_U, 0.0]]
        out.append(envelope("th-head", "burner-head", "stainless-hot", "emitter-head", head))
    out += [K.ref("th-" + sid, sid, region="reflector") for sid in ("dome", "dome-bead", "finial", "spacer", "dome-nut", "stud-ring") if sid in ids]
    out += [K.ref("th-" + sid, sid, region="control") for sid in ("knob", "knob-grip", "knob-boss", "igniter") if sid in ids]
    return out + emitters(scale)


def mass_solid(bone, radius, at):
    """⚖️ A compact mass solid at a bone's centre of mass."""
    return K.solid("mass-" + bone, "ellipsoid", bone, "powder-steel", "detail", 0.2, radii=[radius, radius, radius], transform={"translation": at}, member=bone, part="mass")


def structure(scale, solids):
    """🏗️ The structural list: sheet, tube and rod load path, joints at the rig joints, the ground pad and the masses."""
    f = K.FEATURE[scale]
    out = []
    top = pole_top()
    if scale <= 100:
        out += [K.ref("st-dish", "dish", member="base", part="member"), K.ref("st-housing-wall", "housing-wall", member="base", part="member"),
                K.ref("st-housing-lid", "housing-lid", member="base", part="member"), K.ref("st-socket", "socket", member="pole", part="joint")]
    elif scale == 200:
        out += [K.ref("st-housing", "housing", member="base", part="member"),
                K.revolve_up("st-socket", "base", "powder-steel", "detail", 0.5, K.ring(0.715, 0.755, K.POLE_R, 0.06), member="pole", part="joint")]
    else:
        out += [K.ref("st-housing", "housing", member="base", part="member")]
    out.append(K.revolve_up("st-support-base-contact", "base", "powder-steel", "body", 0.15, [[K.r6(-f), 0.0], [K.r6(-f), K.DISH_R if scale < 500 else 0.225], [0.0, K.DISH_R if scale < 500 else 0.225], [0.0, 0.0]],
                            member="base", part="support", anchor="base-contact"))
    if scale <= 100:
        out.append(K.revolve_up("st-bottle-shell", "bottle", "bottle-steel", "detail", 0.45, K.shell_loop(bottle_profile(), 0.0025), member="bottle", part="member"))
        out.append(K.revolve_up("st-bottle-foot", "bottle", "bottle-steel", "detail", 0.4, K.ring(0.0, 0.058, 0.117 if scale == 50 else 0.108, 0.123), member="bottle", part="member"))
    else:
        out.append(K.revolve_up("st-bottle-shell", "bottle", "bottle-steel", "detail", 0.45,
                                K.loop([[0.0, 0.0], [0.0, K.BOTTLE_R], [0.50, K.BOTTLE_R], [0.58, 0.075], [0.58, 0.0], [K.r6(0.58 - f), 0.0], [K.r6(0.58 - f), 0.06], [K.r6(0.50 - f / 2), K.r6(K.BOTTLE_R - f)], [f, K.r6(K.BOTTLE_R - f)], [f, 0.0]]),
                                member="bottle", part="member"))
    if scale <= 100:
        out.append(K.ref("st-pole", "pole", member="pole", part="member"))
        if scale == 50:
            out += [K.ref("st-pole-coupling-0", "pole-coupling-0", member="pole", part="joint"), K.ref("st-pole-coupling-1", "pole-coupling-1", member="pole", part="joint")]
    elif scale == 200:
        out.append(K.revolve_up("st-pole", "pole", "stainless-brushed", "detail", 0.45, K.ring(-0.018, top, K.POLE_R - K.POLE_T, K.POLE_R), member="pole", part="member"))
    else:
        d_in = (0.075 ** 4 - 64 * 1.18e-7 / PI) ** 0.25
        out.append(K.revolve_up("st-pole", "pole", "stainless-brushed", "detail", 0.45, K.ring(-0.018, top, K.r6(d_in / 2), 0.0375), member="pole", part="member"))
    if scale <= 100:
        out.append(K.revolve_up("st-valve-housing", "burner-head", "stainless-brushed", "detail", 0.45, K.shell_loop(K.head_profile(scale), 0.0015), member="burner-head", part="member"))
        out.append(K.revolve_up("st-head-sleeve", "burner-head", "stainless-brushed", "detail", 0.3, K.ring(-0.004, 0.04, K.POLE_R, 0.0375), member="burner-head", part="joint"))
        out.append(K.ref("st-guard-rod", "guard-rod", member="burner-head", part="member"))
        out.append(K.ref("st-emitter-cap", "emitter-cap" if scale == 50 else "emitter", member="burner-head", part="member"))
        out.append(K.ref("st-dome", "dome", member="reflector", part="member"))
        out.append(K.ref("st-spacer", "spacer", member="reflector", part="member"))
        out.append(K.solid("st-reflector-studs", "ellipsoid", "reflector", "stainless-brushed", "detail", 0.3, radii=[f, 0.012, 0.012], transform={"translation": [0.0, -K.SPACER_R, 0.0]},
                           member="reflector", part="joint", repeat=THREE))
        out.append(K.solid("st-knob-stem", "capsule", "knob", "stainless-brushed", "detail", 0.3, p0=[-0.012, 0.0, 0.0], p1=[0.004, 0.0, 0.0], r0=K.r6(max(0.004, f / 2)), r1=K.r6(max(0.004, f / 2)), member="knob", part="joint"))
    else:
        t = f / 2
        out.append(K.revolve_up("st-valve-housing", "burner-head", "stainless-brushed", "detail", 0.45,
                                K.shell_loop([[0.0, 0.0], [0.0, 0.036], [0.040, 0.085], [K.DECK_U, 0.085], [K.DECK_U, 0.0]] if scale == 200 else [[0.0, 0.0], [0.0, 0.085], [K.DECK_U, 0.085], [K.DECK_U, 0.0]], t), member="burner-head", part="member"))
        out.append(K.revolve_up("st-screen-frame", "burner-head", "stainless-hot", "detail", 0.6, K.ring(K.DECK_U, K.SCREEN_TOP_U, K.r6(K.SCREEN_R - t), K.SCREEN_R), member="burner-head", part="member"))
        out.append(K.revolve_up("st-cap-plate", "burner-head", "stainless-hot", "detail", 0.6, [[K.r6(K.CAP_TOP_U - f), 0.0], [K.r6(K.CAP_TOP_U - f), K.SCREEN_R], [K.CAP_TOP_U, K.SCREEN_R], [K.CAP_TOP_U, 0.0]], member="burner-head", part="member"))
        out.append(K.ref("st-dome", "dome", member="reflector", part="member"))
        spacer_top = K.dome_outer(K.SPACER_R) - 0.01
        out.append(K.solid("st-spacer", "capsule", "reflector", "stainless-hot", "detail", 0.4, p0=[K.r6(t), -K.SPACER_R, 0.0], p1=[K.r6(spacer_top - t), -K.SPACER_R, 0.0], r0=K.r6(t), r1=K.r6(t),
                           member="reflector", part="member", repeat=THREE))
    mr = max(f, 0.03)
    out += [
        mass_solid("base", K.r6(mr * 1.6), [0.22, 0.0, 0.0]),
        mass_solid("bottle", K.r6(mr * 2.2), [0.27, 0.0, 0.0]),
        mass_solid("pole", mr, [0.465, 0.0, 0.0]),
        mass_solid("burner-head", mr, [0.2, 0.0, 0.0]),
        mass_solid("reflector", mr, [0.1, 0.0, 0.0]),
    ]
    return out


def volume(scale):
    """📦 One authored scale volume."""
    b = body(scale)
    return {"status": "authored", "denominator": scale, "body": b, "thermal": thermal(scale, b), "structure": structure(scale, b), "decisions": TX.VOLUME_DECISIONS[scale]}


def volumes():
    """🗃️ All four scales."""
    return {str(s): volume(s) for s in (50, 100, 200, 500)}
