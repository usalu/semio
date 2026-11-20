"""🩺 Round 3, furniture/operating-table: every part of every scale stated by hand (body), and the thermal and structural lists and the bilingual decisions built on them.

1:50 is the full construction; 1:100, 1:200 and 1:500 are each stated anew with merged masses.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_operating_table_kit", os.path.join(HERE, "r3-operating-table-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}
LAYER = {"table-pad": "vinyl", "table-frame": "carrier", "table-column": "cover", "table-base": "shell", "stand": "tube", "light-housing": "housing", "light-lens": "lens"}
DOME = [(0.03, 0.226), (0.042, 0.2), (0.054, 0.16), (0.064, 0.11), (0.071, 0.06), (0.075, 0.0)]


def pad(sid, x, zc, hw, ends=(True, True), inset=0.0, exponent=10, material=K.PAD, fine=True):
    """🛏 A table pad: an 80 mm cushion with rounded long edges; every end is pulled in over 20 mm."""
    yc, hh = (K.FRAME_Y1 + K.PAD_Y1) / 2, (K.PAD_Y1 - K.FRAME_Y1) / 2 - inset
    hw, x0, x1 = hw - inset, x[0] + inset, x[1] - inset
    full = lambda at: (at, yc, zc, hw, hh, exponent)
    nose = lambda at: (at, yc, zc, hw - 0.02, hh - 0.012, exponent)
    neck = lambda at: (at, yc, zc, hw - 0.006, hh - 0.004, exponent)
    sections = ([nose(x0), neck(x0 + 0.007), full(x0 + 0.02)] if ends[0] else [full(x0)]) + ([full(x1 - 0.02), neck(x1 - 0.007), nose(x1)] if ends[1] else [full(x1)])
    if not fine:
        sections = [s for s in sections if s[3] != hw - 0.006]
    return K.loft_x(sid, "top", material, "table-pad", 0.3, sections, outline_radius=0.02 if fine else 0.01)


def tray(sid, x, z, beam=0.025):
    """🧇 A top section: a stainless tray, two side beams 40 mm high under a 14 mm panel."""
    y0, y1, yp = K.FRAME_Y0, K.FRAME_Y1, K.FRAME_Y1 - 0.014
    profile = [(y0, z[0]), (y0, z[0] + beam), (yp, z[0] + beam), (yp, z[1] - beam), (y0, z[1] - beam), (y0, z[1]), (y1, z[1]), (y1, z[0])]
    return K.prism_x(sid, "top", K.STAINLESS, "table-frame", 0.55, profile, x)


def cheek(sid, side):
    """🪝 A saddle cheek: the plate that hangs the seat section on the tilt pin."""
    profile = [(0.13, K.FRAME_Y0)] + K.arc(0.0, K.PIVOT_Y, 0.036, -30, -150, 6) + [(-0.13, K.FRAME_Y0)]
    return K.prism_z(sid, "top", K.STAINLESS, "table-frame", 0.5, profile, K.span(side, 0.075, 0.09))


def head_block(sid, tone=0.6):
    """🧱 The bearing block of the column head, rounded about the tilt pin."""
    profile = [(-0.06, 0.69), (0.06, 0.69), (0.06, 0.72)] + K.arc(0.0, K.PIVOT_Y, 0.035, 0, 180, 6) + [(-0.06, 0.72)]
    return K.prism_z(sid, "column", K.STAINLESS, "table-column", tone, profile, (-0.07, 0.07))


def castor_fork(sid, bone, region, material, x, z, y, tone=0.4):
    """🍴 The fork web between the twin wheels of a castor."""
    return K.box(sid, bone, material, region, tone, (x - 0.02, x + 0.02), y, (z - 0.004, z + 0.004), role="detail", views="")


def table50():
    """🛠 The operating table at 1:50."""
    s = []
    B, C, T = "base", "column", "top"
    chamfer = [(K.BASE_Y0, -0.15), (K.BASE_Y0, 0.15), (K.BASE_SHOULDER, 0.15), (K.BASE_Y1, 0.13), (K.BASE_Y1, -0.13), (K.BASE_SHOULDER, -0.15)]
    s.append(K.prism_x("base-spine", B, K.STEEL, "table-base", 0.5, chamfer, (-K.SPINE_HALF_X - 0.01, K.SPINE_HALF_X + 0.01)))
    for name, x in (("head", -0.40), ("foot", 0.40)):
        s.append(K.loft_y("base-beam-" + name, B, K.STEEL, "table-base", 0.5, x, 0.0, [(K.BASE_Y0, 0.07, 0.27, 6), (K.BASE_SHOULDER, 0.07, 0.27, 6), (K.BASE_Y1, 0.056, 0.256, 6)]))
    s.append(K.prism_y("base-plinth", B, K.STEEL, "table-base", 0.45, [(z, x) for x, z in K.rrect((-0.175, 0.175), (-0.125, 0.125), 0.035)], (K.BASE_Y1 - 0.005, K.PLINTH_Y1 + 0.004)))
    for xn, x in (("head", -K.CASTOR_X), ("foot", K.CASTOR_X)):
        for zn, z in (("r", -K.CASTOR_Z), ("l", K.CASTOR_Z)):
            for wn, dz in (("a", -0.017), ("b", 0.017)):
                near = zn == "r" and wn == "a"
                s.append(K.wheel(f"castor-{xn}-{zn}-{wn}", B, "table-base", x, K.CASTOR_R, z + dz, K.CASTOR_R, 0.024, views="efp" if near else "f"))
            s.append(castor_fork(f"castor-{xn}-{zn}-fork", B, "table-base", K.STEEL, x, z, (0.03, 0.07)))
    for zn, side in (("r", -1), ("l", 1)):
        s.append(K.box(f"pedal-{zn}-arm", B, K.STEEL, "table-base", 0.35, (0.194, 0.206), (0.07, 0.082), K.span(side, 0.14, 0.2), role="detail", views="p"))
        s.append(K.box(f"pedal-{zn}", B, K.STEEL, "table-base", 0.3, (0.165, 0.235), (0.072, 0.084), K.span(side, 0.195, 0.24), radius=0.004, role="detail", views="efp" if side < 0 else "fp"))
    shroud = lambda sid, hx, hz, y, tone: K.prism_y(sid, C, K.STAINLESS, "table-column", tone, [(z, x) for x, z in K.rrect((-hx, hx), (-hz, hz), 0.03)], y)
    s.append(shroud("column-flange", 0.165, 0.145, (K.PLINTH_Y1, 0.176), 0.5))
    s.append(shroud("column-lower", 0.15, 0.13, (0.17, 0.42), 0.6))
    s.append(shroud("column-middle", 0.135, 0.115, (0.41, 0.57), 0.66))
    s.append(shroud("column-upper", 0.12, 0.10, (0.56, 0.681), 0.72))
    s.append(shroud("column-cap", 0.125, 0.105, (0.68, 0.695), 0.5))
    s.append(head_block("column-head"))
    s.append(K.cyl_z("tilt-pin", T, K.STAINLESS, "table-frame", 0.4, 0.0, K.PIVOT_Y, (-0.1, 0.1), 0.014, role="detail"))
    s += K.mirrored(lambda sid, side: cheek(sid, side), "saddle-cheek")
    s += K.mirrored(lambda sid, side: K.box(sid, T, K.STAINLESS, "table-frame", 0.5, (-0.13, 0.13), (0.766, K.FRAME_Y0), K.span(side, 0.07, 0.225), views="fp"), "saddle-plate")
    s.append(tray("frame-back", K.BACK_X, (-K.PAD_HALF_Z, K.PAD_HALF_Z)))
    s.append(tray("frame-seat", K.SEAT_X, (-K.PAD_HALF_Z, K.PAD_HALF_Z)))
    s += K.mirrored(lambda sid, side: tray(sid, K.LEG_X, K.span(side, *K.LEG_Z), beam=0.02), "frame-leg")
    s.append(K.box("frame-back-end", T, K.STAINLESS, "table-frame", 0.5, (K.BACK_X[0], K.BACK_X[0] + 0.03), (K.FRAME_Y0, K.FRAME_Y1 - 0.014), (-0.225, 0.225), views="f"))
    s.append(K.box("head-plate", T, K.STAINLESS, "table-frame", 0.55, K.HEAD_X, (0.80, K.FRAME_Y1), (-K.HEAD_HALF_Z, K.HEAD_HALF_Z), radius=0.004))
    s.append(K.box("head-clamp", T, K.STAINLESS, "table-frame", 0.45, (-0.9, -0.84), (0.776, 0.80), (-0.13, 0.13), radius=0.003))
    s += K.mirrored(lambda sid, side: K.cyl_x(sid, T, K.STAINLESS, "table-frame", 0.6, (-0.86, -0.67), 0.79, side * 0.11, 0.008, role="detail", views="ep"), "head-rod")
    for name, x in (("back", K.HINGE_X[0]), ("leg", K.HINGE_X[1])):
        s += K.mirrored(lambda sid, side, x=x: K.cyl_z(sid, T, K.STAINLESS, "table-frame", 0.4, x, 0.80, K.span(side, 0.2, 0.25), 0.022, role="detail", views="e"), "hinge-" + name)
    for name, x in (("back", (-0.68, -0.245)), ("seat", (-0.195, 0.235)), ("leg", (0.30, 0.80))):
        s += K.mirrored(lambda sid, side, x=x: K.box(sid, T, K.STAINLESS, "table-frame", 0.7, x, (K.RAIL_Y0, K.RAIL_Y1), K.span(side, K.RAIL_Z0, K.RAIL_Z1), radius=0.003), "rail-" + name)
        for end, at in (("a", x[0] + 0.05), ("b", x[1] - 0.05)):
            s += K.mirrored(lambda sid, side, at=at: K.cyl_z(sid, T, K.STAINLESS, "table-frame", 0.5, at, 0.80, K.span(side, 0.249, 0.271), 0.008, role="detail", views="p"), f"rail-{name}-post-{end}")
    s.append(pad("pad-head", K.HEAD_X, 0.0, K.HEAD_HALF_Z))
    s.append(pad("pad-back", K.BACK_X, 0.0, K.PAD_HALF_Z))
    s.append(pad("pad-seat", K.SEAT_X, 0.0, K.PAD_HALF_Z))
    s += K.mirrored(lambda sid, side: pad(sid, K.LEG_X, side * (K.LEG_Z[0] + K.LEG_Z[1]) / 2, (K.LEG_Z[1] - K.LEG_Z[0]) / 2), "pad-leg")
    return s


def head_profile(fine):
    """💡 The lamp housing about its axis: the underside pan around the lens seat, the rim, the cooling step and the dome; heights from the head joint."""
    rim = [(-0.022, 0.0), (-0.022, K.LENS_R), (-0.03, K.LENS_R), (-0.03, 0.244), (-0.024, 0.2485), (-0.015, 0.25), (0.008, 0.25), (0.016, 0.247), (0.022, 0.24)] if fine else [(-0.015, 0.0), (-0.015, K.LENS_R), (-0.03, K.LENS_R), (-0.03, 0.245), (-0.015, 0.25), (0.012, 0.25), (0.022, 0.24)]
    dome = DOME if fine else [DOME[0], DOME[2], DOME[4], DOME[5]]
    return [(K.HEAD_Y + a, r) for a, r in rim + dome]


def yoke(fine):
    """🪃 The half yoke of bent 25 mm tube from the swivel under the arm end to the tilt bearing on the rim of the lamp head."""
    A, R, st = "stand-arm", K.YOKE_R, "stand"
    x0, x1, y, b = K.HEAD_CX, K.YOKE_X1, K.YOKE_Y, K.YOKE_BEND
    out = [K.cyl_y("yoke-swivel", A, K.STEEL, st, 0.4, x0, 0.0, (y - 0.005, K.ARM_Y - 0.03), 0.0225)]
    out.append(K.tube("yoke-run", A, K.STEEL, st, 0.5, (x0, y, 0), (x1 - b, y, 0), R))
    bend = K.arc(x1 - b, y - b, b, 90, 0, 3 if fine else 1)
    for k in range(len(bend) - 1):
        out.append(K.tube(f"yoke-bend-{k + 1}", A, K.STEEL, st, 0.5, (bend[k][0], bend[k][1], 0), (bend[k + 1][0], bend[k + 1][1], 0), R))
    out.append(K.tube("yoke-drop", A, K.STEEL, st, 0.5, (x1, y - b, 0), (x1, K.HEAD_Y, 0), R))
    return out


def stand50():
    """🔦 The mobile surgical light at 1:50."""
    s = []
    SB, SC, A, H, st = "stand-base", "stand-column", "stand-arm", "light-head", "stand"
    X = K.STAND_X
    for name, sign in (("near", -1), ("far", 1)):
        x = X + sign * K.STAND_LEG_X
        s.append(K.box("stand-leg-" + name, SB, K.STEEL, st, 0.45, (x - 0.03, x + 0.03), K.STAND_LEG_Y, (-K.STAND_HALF_Z, K.STAND_HALF_Z), radius=0.008))
        for zn, z in (("r", -K.STAND_CASTOR_Z), ("l", K.STAND_CASTOR_Z)):
            cx = X + sign * K.STAND_CASTOR_X
            for wn, dz in (("a", -0.013), ("b", 0.013)):
                near = zn == "r" and wn == "a"
                s.append(K.wheel(f"stand-castor-{name}-{zn}-{wn}", SB, st, cx, K.STAND_CASTOR_R, z + dz, K.STAND_CASTOR_R, 0.018, bevel=0.004, views="efp" if near else "f"))
            s.append(castor_fork(f"stand-castor-{name}-{zn}-fork", SB, st, K.STEEL, cx + sign * 0.012, z, (0.03, 0.078)))
            s.append(K.cyl_y(f"stand-castor-{name}-{zn}-stem", SB, K.STEEL, st, 0.4, x, z, (0.07, 0.09), 0.012, role="detail", views="e" if zn == "r" else ""))
    s.append(K.box("stand-spine", SB, K.STEEL, st, 0.45, (X - 0.225, X + 0.225), K.STAND_LEG_Y, (-0.035, 0.035), views="ep"))
    s.append(K.box("stand-ballast", SB, K.STEEL, st, 0.5, (X - 0.13, X + 0.13), (0.085, K.BOX_Y1), (-0.09, 0.09), radius=0.012))
    s.append(K.turned_y("stand-socket", SC, K.STEEL, st, 0.4, X, 0.0, [(K.BOX_Y1, 0.0), (K.BOX_Y1, 0.045), (0.185, 0.045), (0.2, 0.034), (0.235, 0.034), (0.235, 0.0)]))
    s.append(K.cyl_y("stand-tube", SC, K.STEEL, st, 0.5, X, 0.0, (0.23, K.COLUMN_Y1), K.COLUMN_R))
    s.append(K.box("stand-driver", SC, K.STEEL, st, 0.4, (X + 0.02, X + 0.075), (1.0, 1.14), (-0.035, 0.035), radius=0.006))
    s.append(K.turned_y("stand-bearing", SC, K.STEEL, st, 0.4, X, 0.0, [(1.835, 0.0), (1.835, 0.03), (1.85, 0.037), (1.95, 0.037), (K.STAND_TOP, 0.03), (K.STAND_TOP, 0.0)]))
    s.append(K.cyl_z("arm-root", A, K.STEEL, st, 0.4, X, K.ARM_Y, (-0.031, 0.031), 0.035))
    s.append(K.loft_x("arm-body", A, K.STEEL, st, 0.5, [(X - 0.02, K.ARM_Y, 0, 0.02, 0.0275, 4), (K.HEAD_CX + 0.02, K.ARM_Y, 0, 0.02, 0.0275, 4)]))
    s.append(K.cyl_z("arm-tip", A, K.STEEL, st, 0.4, K.HEAD_CX, K.ARM_Y, (-0.028, 0.028), 0.035))
    s += yoke(True)
    s.append(K.cyl_x("head-bearing", H, K.ALU, "light-housing", 0.4, (K.HEAD_CX + 0.24, K.YOKE_X1 + 0.012), K.HEAD_Y, 0.0, 0.019))
    s.append(K.turned_y("head-housing", H, K.ALU, "light-housing", 0.35, K.HEAD_CX, 0.0, head_profile(True)))
    s.append(K.cyl_y("head-lens", H, K.LENS, "light-lens", 0.9, K.HEAD_CX, 0.0, (K.LENS_Y0, K.LENS_Y0 + 0.008), K.LENS_R, views="ef"))
    s.append(K.turned_y("head-handle", H, K.ALU, "light-housing", 0.4, K.HEAD_CX, 0.0, [(1.545, 0.0), (1.55, 0.013), (1.635, 0.016), (1.645, 0.03), (K.LENS_Y0 + 0.001, 0.03), (K.LENS_Y0 + 0.001, 0.0)], views="ef"))
    return s


def table100():
    """🛠 The operating table at 1:100: plain base bars, single wheels, two shrouds, section slabs, rails as one bar with their posts."""
    s = []
    B, C, T = "base", "column", "top"
    s.append(K.box("base-spine", B, K.STEEL, "table-base", 0.5, (-0.34, 0.34), (K.BASE_Y0, K.BASE_Y1), (-0.15, 0.15), radius=0.015))
    for name, x in (("head", -0.40), ("foot", 0.40)):
        s.append(K.box("base-beam-" + name, B, K.STEEL, "table-base", 0.5, (x - 0.07, x + 0.07), (K.BASE_Y0, K.BASE_Y1), (-0.27, 0.27), radius=0.02))
        for zn, z in (("r", -K.CASTOR_Z), ("l", K.CASTOR_Z)):
            s.append(K.wheel(f"castor-{name}-{zn}", B, "table-base", x, K.CASTOR_R, z, K.CASTOR_R, 0.05, bevel=0.01, views="efp" if zn == "r" else "f"))
    shroud = lambda sid, hx, hz, y, tone: K.prism_y(sid, C, K.STAINLESS, "table-column", tone, [(z, x) for x, z in K.rrect((-hx, hx), (-hz, hz), 0.03, 2)], y)
    s.append(shroud("column-lower", 0.15, 0.13, (K.BASE_Y1, 0.42), 0.6))
    s.append(shroud("column-upper", 0.128, 0.108, (0.42, 0.695), 0.7))
    s.append(K.prism_z("column-head", C, K.STAINLESS, "table-column", 0.6, [(-0.06, 0.695), (0.06, 0.695), (0.06, 0.74), (0.03, 0.775), (-0.03, 0.775), (-0.06, 0.74)], (-0.07, 0.07)))
    s += K.mirrored(lambda sid, side: K.prism_z(sid, T, K.STAINLESS, "table-frame", 0.5, [(0.13, K.FRAME_Y0), (0.035, 0.705), (-0.035, 0.705), (-0.13, K.FRAME_Y0)], K.span(side, 0.075, 0.09)), "saddle-cheek")
    slab = lambda sid, x, z: K.box(sid, T, K.STAINLESS, "table-frame", 0.55, x, (K.FRAME_Y0, K.FRAME_Y1), z)
    s.append(slab("frame-back", K.BACK_X, (-K.PAD_HALF_Z, K.PAD_HALF_Z)))
    s.append(slab("frame-seat", K.SEAT_X, (-K.PAD_HALF_Z, K.PAD_HALF_Z)))
    s += K.mirrored(lambda sid, side: slab(sid, K.LEG_X, K.span(side, *K.LEG_Z)), "frame-leg")
    s.append(K.box("head-plate", T, K.STAINLESS, "table-frame", 0.55, (K.HEAD_X[0], K.BACK_X[0]), (0.795, K.FRAME_Y1), (-K.HEAD_HALF_Z, K.HEAD_HALF_Z)))
    for name, x in (("back", (-0.68, -0.245)), ("seat", (-0.195, 0.235)), ("leg", (0.30, 0.80))):
        s += K.mirrored(lambda sid, side, x=x: K.box(sid, T, K.STAINLESS, "table-frame", 0.7, x, (K.RAIL_Y0, K.RAIL_Y1), K.span(side, 0.25, K.RAIL_Z1)), "rail-" + name)
    s.append(pad("pad-head", K.HEAD_X, 0.0, K.HEAD_HALF_Z, fine=False))
    s.append(pad("pad-back", K.BACK_X, 0.0, K.PAD_HALF_Z, fine=False))
    s.append(pad("pad-seat", K.SEAT_X, 0.0, K.PAD_HALF_Z, fine=False))
    s += K.mirrored(lambda sid, side: pad(sid, K.LEG_X, side * (K.LEG_Z[0] + K.LEG_Z[1]) / 2, (K.LEG_Z[1] - K.LEG_Z[0]) / 2, fine=False), "pad-leg")
    return s


def stand100():
    """🔦 The surgical light at 1:100: single wheels, plain column with its bearing, one arm, a three-piece yoke, a simpler head."""
    s = []
    SB, SC, A, H, st = "stand-base", "stand-column", "stand-arm", "light-head", "stand"
    X = K.STAND_X
    for name, sign in (("near", -1), ("far", 1)):
        x = X + sign * K.STAND_LEG_X
        s.append(K.box("stand-leg-" + name, SB, K.STEEL, st, 0.45, (x - 0.03, x + 0.03), (0.075, 0.115), (-K.STAND_HALF_Z, K.STAND_HALF_Z)))
        for zn, z in (("r", -K.STAND_CASTOR_Z), ("l", K.STAND_CASTOR_Z)):
            s.append(K.wheel(f"stand-castor-{name}-{zn}", SB, st, X + sign * K.STAND_CASTOR_X, K.STAND_CASTOR_R, z, K.STAND_CASTOR_R, 0.04, bevel=0.008, views="efp" if zn == "r" else "f"))
    s.append(K.box("stand-ballast", SB, K.STEEL, st, 0.5, (X - 0.225, X + 0.225), (0.085, K.BOX_Y1), (-0.09, 0.09)))
    s.append(K.cyl_y("stand-tube", SC, K.STEEL, st, 0.5, X, 0.0, (K.BOX_Y1, K.COLUMN_Y1), K.COLUMN_R))
    s.append(K.box("stand-driver", SC, K.STEEL, st, 0.4, (X + 0.02, X + 0.075), (1.0, 1.14), (-0.035, 0.035)))
    s.append(K.cyl_y("stand-bearing", SC, K.STEEL, st, 0.4, X, 0.0, (K.COLUMN_Y1, K.STAND_TOP), 0.037))
    s.append(K.loft_x("arm-body", A, K.STEEL, st, 0.5, [(X + 0.03, K.ARM_Y, 0, 0.012, 0.02, 2), (X + 0.015, K.ARM_Y, 0, 0.022, 0.033, 3), (K.HEAD_CX - 0.015, K.ARM_Y, 0, 0.022, 0.033, 3), (K.HEAD_CX - 0.03, K.ARM_Y, 0, 0.012, 0.02, 2)]))
    s += yoke(False)
    s.append(K.turned_y("head-housing", H, K.ALU, "light-housing", 0.35, K.HEAD_CX, 0.0, head_profile(False)))
    s.append(K.cyl_y("head-lens", H, K.LENS, "light-lens", 0.9, K.HEAD_CX, 0.0, (K.LENS_Y0, K.LENS_Y0 + 0.015), K.LENS_R, views="ef"))
    s.append(K.cyl_y("head-handle", H, K.ALU, "light-housing", 0.4, K.HEAD_CX, 0.0, (1.55, K.LENS_Y0 + 0.001), 0.016, views="ef"))
    return s


def table200():
    """🛠 The operating table at 1:200: an H of three bars on four wheels, a stepped column, one frame slab with the rails in its edge, three pads."""
    s = []
    B, C, T = "base", "column", "top"
    s.append(K.box("base-spine", B, K.STEEL, "table-base", 0.5, (-0.33, 0.33), (K.BASE_Y0, K.BASE_Y1), (-0.15, 0.15)))
    for name, x in (("head", -0.40), ("foot", 0.40)):
        s.append(K.box("base-beam-" + name, B, K.STEEL, "table-base", 0.5, (x - 0.07, x + 0.07), (K.BASE_Y0, K.BASE_Y1), (-0.27, 0.27)))
        s.append(K.cyl_z("castor-" + name, B, K.RUBBER, "table-base", 0.2, x, K.CASTOR_R, (-0.235, 0.235), K.CASTOR_R))
    s.append(K.box("column-lower", C, K.STAINLESS, "table-column", 0.6, (-0.15, 0.15), (K.BASE_Y1, 0.42), (-0.13, 0.13)))
    s.append(K.box("column-upper", C, K.STAINLESS, "table-column", 0.7, (-0.125, 0.125), (0.42, 0.70), (-0.105, 0.105)))
    s.append(K.box("column-head", C, K.STAINLESS, "table-column", 0.6, (-0.06, 0.06), (0.70, K.FRAME_Y0), (-0.09, 0.09)))
    s.append(K.box("frame", T, K.STAINLESS, "table-frame", 0.55, (K.BACK_X[0], K.LEG_X[1]), (K.FRAME_Y0, K.FRAME_Y1), (-K.RAIL_Z1, K.RAIL_Z1)))
    s.append(K.box("head-plate", T, K.STAINLESS, "table-frame", 0.55, (K.HEAD_X[0], K.BACK_X[0]), (0.79, K.FRAME_Y1), (-K.HEAD_HALF_Z, K.HEAD_HALF_Z)))
    s.append(K.box("pad-head", T, K.PAD, "table-pad", 0.3, K.HEAD_X, (K.FRAME_Y1, K.PAD_Y1), (-K.HEAD_HALF_Z, K.HEAD_HALF_Z), radius=0.015))
    s.append(K.box("pad-body", T, K.PAD, "table-pad", 0.3, (K.BACK_X[0], K.SEAT_X[1]), (K.FRAME_Y1, K.PAD_Y1), (-K.PAD_HALF_Z, K.PAD_HALF_Z), radius=0.015))
    s.append(K.box("pad-leg", T, K.PAD, "table-pad", 0.3, K.LEG_X, (K.FRAME_Y1, K.PAD_Y1), (-K.PAD_HALF_Z, K.PAD_HALF_Z), radius=0.015))
    return s


def stand200():
    """🔦 The surgical light at 1:200: two legs on rollers, ballast box, column, arm, an L yoke of two 30 mm bars and the head as one turned body."""
    s = []
    SB, SC, A, H, st = "stand-base", "stand-column", "stand-arm", "light-head", "stand"
    X = K.STAND_X
    for name, sign in (("near", -1), ("far", 1)):
        x = X + sign * K.STAND_LEG_X
        s.append(K.box("stand-leg-" + name, SB, K.STEEL, st, 0.45, (x - 0.03, x + 0.03), (0.075, 0.115), (-K.STAND_HALF_Z, K.STAND_HALF_Z)))
        s.append(K.cyl_z("stand-castor-" + name, SB, K.RUBBER, st, 0.2, X + sign * K.STAND_CASTOR_X, K.STAND_CASTOR_R, (-0.215, 0.215), K.STAND_CASTOR_R))
    s.append(K.box("stand-ballast", SB, K.STEEL, st, 0.5, (X - 0.22, X + 0.22), (0.075, K.BOX_Y1), (-0.09, 0.09)))
    s.append(K.cyl_y("stand-tube", SC, K.STEEL, st, 0.5, X, 0.0, (K.BOX_Y1, 1.87), K.COLUMN_R))
    s.append(K.cyl_y("stand-bearing", SC, K.STEEL, st, 0.4, X, 0.0, (1.87, K.STAND_TOP), 0.04))
    s.append(K.box("arm-body", A, K.STEEL, st, 0.5, (K.HEAD_CX - 0.03, X - 0.03), (K.ARM_Y - 0.03, K.ARM_Y + 0.03), (-0.022, 0.022)))
    s.append(K.box("yoke-swivel", A, K.STEEL, st, 0.4, (K.HEAD_CX - 0.02, K.HEAD_CX + 0.02), (K.YOKE_Y + 0.015, K.ARM_Y - 0.03), (-0.02, 0.02)))
    s.append(K.box("yoke-run", A, K.STEEL, st, 0.5, (K.HEAD_CX - 0.015, K.YOKE_X1 + 0.015), (K.YOKE_Y - 0.015, K.YOKE_Y + 0.015), (-0.015, 0.015)))
    s.append(K.box("yoke-drop", A, K.STEEL, st, 0.5, (K.YOKE_X1 - 0.015, K.YOKE_X1 + 0.015), (K.HEAD_Y - 0.015, K.YOKE_Y - 0.015), (-0.015, 0.015)))
    s.append(K.turned_y("head", H, K.ALU, "light-housing", 0.4, K.HEAD_CX, 0.0, [(K.LENS_Y0, 0.0), (K.LENS_Y0, 0.245), (1.675, 0.25), (1.705, 0.25), (1.725, 0.22), (1.752, 0.12), (1.765, 0.0)], regions=["light-housing", "light-lens"]))
    s.append(K.cyl_y("head-handle", H, K.ALU, "light-housing", 0.4, K.HEAD_CX, 0.0, (1.55, K.LENS_Y0 + 0.001), 0.016, views="ef"))
    return s


def table500():
    """🛠 The operating table at 1:500: base slab, column block, top slab with the narrower head piece."""
    B, C, T = "base", "column", "top"
    return [
        K.box("base", B, K.STEEL, "table-base", 0.5, (-K.BASE_HALF_X, K.BASE_HALF_X), (0.0, K.BASE_Y1), (-K.BASE_HALF_Z, K.BASE_HALF_Z)),
        K.box("column", C, K.STAINLESS, "table-column", 0.65, (-0.14, 0.14), (K.BASE_Y1, K.FRAME_Y0), (-0.12, 0.12)),
        K.box("top", T, K.PAD, "table-pad", 0.35, (K.BACK_X[0], K.LEG_X[1]), (K.FRAME_Y0, K.PAD_Y1), (-K.RAIL_Z1, K.RAIL_Z1), regions=["table-pad", "table-frame"]),
        K.box("top-head", T, K.PAD, "table-pad", 0.35, (K.HEAD_X[0], K.BACK_X[0]), (0.795, K.PAD_Y1), (-K.HEAD_HALF_Z, K.HEAD_HALF_Z), regions=["table-pad", "table-frame"]),
    ]


def stand500():
    """🔦 The surgical light at 1:500: foot slab, column, arm, hanger and the head disc."""
    SB, SC, A, H, st = "stand-base", "stand-column", "stand-arm", "light-head", "stand"
    X = K.STAND_X
    return [
        K.box("stand-foot", SB, K.STEEL, st, 0.45, (X - 0.28, X + 0.28), (0.0, 0.115), (-K.STAND_HALF_Z, K.STAND_HALF_Z)),
        K.cyl_y("stand-tube", SC, K.STEEL, st, 0.5, X, 0.0, (0.115, K.STAND_TOP), 0.03, slender=True),
        K.box("arm-body", A, K.STEEL, st, 0.5, (K.HEAD_CX - 0.03, X - 0.03), (K.ARM_Y - 0.03, K.ARM_Y + 0.03), (-0.03, 0.03)),
        K.box("yoke-drop", A, K.STEEL, st, 0.5, (K.HEAD_CX - 0.03, K.HEAD_CX + 0.03), (1.76, K.ARM_Y - 0.03), (-0.03, 0.03)),
        K.turned_y("head", H, K.ALU, "light-housing", 0.4, K.HEAD_CX, 0.0, [(K.LENS_Y0, 0.0), (K.LENS_Y0, 0.25), (1.71, 0.25), (1.765, 0.1), (1.765, 0.0)], regions=["light-housing", "light-lens"]),
    ]


PARTS = {"50": lambda: table50() + stand50(), "100": lambda: table100() + stand100(), "200": lambda: table200() + stand200(), "500": lambda: table500() + stand500()}


def parts(scale):
    """📋 The parts of one scale; `regions` (a merged form) is kept on the part, never on the solid."""
    out = PARTS[scale]()
    for p in out:
        merged = p["solid"].pop("regions", None)
        if merged:
            p["regions"] = merged
    return out


def body(plist):
    """🧊 The body list."""
    return [p["solid"] for p in plist]


def plume(scale):
    """♨ The outlet patch of the plume: the crown of the dome out to the emitter radius of 0.2 m, a skin that follows the dome and stands 1.5 mm proud of it."""
    inside, proud = (0.0005, 0.0015) if scale == "50" else (FEATURE[scale], 0.002)
    crown = {"200": [(0.075, 0.0), (0.062, 0.12), (0.0404, 0.2)], "500": [(0.075, 0.0), (0.075, 0.1), (0.0383, 0.2)]}.get(scale) or [(a, r) for a, r in reversed(DOME) if r <= 0.2]
    profile = [(K.HEAD_Y + a - inside, r) for a, r in crown] + [(K.HEAD_Y + a + proud, r) for a, r in reversed(crown)]
    solid = K.turned_y("th-light-plume", "light-head", K.ALU, "light-housing", 0.35, K.HEAD_CX, 0.0, profile, role="detail")["solid"]
    solid.update({"region": "light-housing", "tags": ["emitter", "light-plume"]})
    if scale == "50":
        solid["layer"] = "housing"
    return solid


def thermal(scale, plist):
    """🌡 The thermal list: every body surface is the surface of its region; at 1:50 the pads are a 1 mm vinyl skin over the foam core; the plume leaves through a patch on the dome."""
    out = []
    if scale == "50":
        cores = [("pad-head", K.HEAD_X, 0.0, K.HEAD_HALF_Z), ("pad-back", K.BACK_X, 0.0, K.PAD_HALF_Z), ("pad-seat", K.SEAT_X, 0.0, K.PAD_HALF_Z)]
        half, mid = (K.LEG_Z[1] - K.LEG_Z[0]) / 2, (K.LEG_Z[0] + K.LEG_Z[1]) / 2
        cores += [("pad-leg-r", K.LEG_X, -mid, half), ("pad-leg-l", K.LEG_X, mid, half)]
        for sid, x, zc, hw in cores:
            out.append(dict(pad("th-" + sid + "-foam", x, zc, hw, inset=0.001)["solid"], region="table-pad", layer="foam"))
    for p in plist:
        for region in p.get("regions", [p["region"]]):
            entry = {"id": "th-" + p["id"] + ("-" + region if "regions" in p else ""), "of": p["id"], "region": region}
            if scale == "50":
                entry["layer"] = LAYER[region]
            if p["id"] == "head-lens":
                entry["sectionFaces"] = ["+v"]
            out.append(entry)
    out.append(plume(scale))
    return out


def own(p, member, kind="member", sid=None):
    """🔩 A structural solid of its own from a part."""
    return dict(p["solid"], id=sid or p["id"], member=member, part=kind)


def of(sid, member, kind="member"):
    """🔗 A structural reference to a body solid that is the member itself."""
    return {"id": ("member-" if kind == "member" else "joint-") + sid, "of": sid, "member": member, "part": kind}


def lump(bone, x, y, radii):
    """⚖ The mass of a bone at its centre of mass."""
    u, v = K.local(bone, x, y)
    return {"id": "mass-" + bone, "kind": "ellipsoid", "bone": bone, "material": K.STEEL, "role": "detail", "tone": 0.15, "radii": [radii[1], radii[0], radii[2]] if K.FRAMES[bone][2] == 90 else list(radii), "transform": {"translation": [u, v, 0.0]}, "member": bone, "part": "mass"}


def ring(sid, bone, x, y, r0, r1, sides=32):
    """⭕ A flat ring standing on (x, 0) from y[0] to y[1]."""
    import math
    circle = lambda r: [(r * math.sin(2 * math.pi * k / sides), x + r * math.cos(2 * math.pi * k / sides)) for k in range(sides)]
    return K.prism_y(sid, bone, K.STEEL if bone != "light-head" else K.ALU, "", 0.3, circle(r1), y, holes=[circle(r0)])


def hollow(sid, bone, half, wall, y, material=K.STEEL):
    """🔲 A square hollow section standing on the column axis."""
    outer = [(-half, -half), (half, -half), (half, half), (-half, half)]
    inner = [(-half + wall, -half + wall), (half - wall, -half + wall), (half - wall, half - wall), (-half + wall, half - wall)]
    return K.prism_y(sid, bone, material, "", 0.3, outer, y, holes=[inner])


def supports(scale):
    """🟩 The eight contact pads under the castors, at the support anchors."""
    f = FEATURE[scale]
    t, a = max(0.008, f), max(0.03, f)
    out = []
    for bone, ox, cx, cz, word in (("base", 0.0, K.CASTOR_X, K.CASTOR_Z, "table"), ("stand-base", K.STAND_X, K.STAND_CASTOR_X, K.STAND_CASTOR_Z, "stand")):
        for xn, sx in (("rear", -1), ("front", 1)):
            for zn, sz in (("left", 1), ("right", -1)):
                anchor = f"castor-{word}-{xn}-{zn}"
                p = K.box("support-" + anchor, bone, K.RUBBER, "", 0.15, (ox + sx * cx - a / 2, ox + sx * cx + a / 2), (0.0, t), (sz * cz - a / 2, sz * cz + a / 2), role="detail")
                out.append(dict(p["solid"], member=bone, part="support", anchor=anchor))
    return out


def structure(scale, plist):
    """🏗 The load path: chassis bars on the castors, the telescopic lifting column inside the shrouds, the tilt pin and saddle, the side beams of the top sections, the stand legs, tube, arm links and spring, the yoke and the head carrier; pads, shrouds, panels and covers are absent."""
    ids = {p["id"] for p in plist}
    f = FEATURE[scale]
    fine, mid = scale == "50", scale in ("50", "100")
    X = K.STAND_X
    s = []
    t = max(0.02, f)
    s.append(own(K.box("member-base-spine", "base", K.STEEL, "", 0.3, (-0.36, 0.36), (0.07, 0.07 + max(0.05, f)), (-0.08, 0.08)), "base"))
    for name, x in (("head", -K.CASTOR_X), ("foot", K.CASTOR_X)):
        s.append(own(K.box("member-base-beam-" + name, "base", K.STEEL, "", 0.3, (x - 0.04, x + 0.04), (0.07, 0.07 + max(0.05, f)), (-0.25, 0.25)), "base"))
    if scale != "500":
        for p in plist:
            if p["id"].startswith("castor-") and p["material"] == K.RUBBER:
                s.append(of(p["id"], "base"))
            if p["id"].startswith("stand-castor-") and p["material"] == K.RUBBER:
                s.append(of(p["id"], "stand-base"))
    else:
        for name, x in (("head", -K.CASTOR_X), ("foot", K.CASTOR_X)):
            s.append(own(K.box("member-base-foot-" + name, "base", K.STEEL, "", 0.3, (x - 0.04, x + 0.04), (0.0, 0.08), (-0.25, 0.25)), "base"))
    foot = 0.11 if mid else 0.085
    s.append(own(K.box("joint-base-column", "base", K.STEEL, "", 0.3, (-foot, foot), (K.BASE_Y1 - max(0.02, f), K.BASE_Y1), (-foot, foot)), "base", "joint"))
    s.append(lump("base", 0.0, 0.08 if scale != "500" else 0.07, (0.2, max(0.02, f / 2), 0.07)))
    if mid:
        s.append(own(hollow("member-column-lower", "column", 0.085, 0.012 if fine else 0.015, (K.BASE_Y1, 0.46), K.STAINLESS), "column"))
        s.append(own(hollow("member-column-upper", "column", 0.068, 0.01 if fine else 0.015, (0.40, 0.695), K.STAINLESS), "column"))
        s.append(of("column-head", "column"))
    else:
        s.append(own(K.box("member-column", "column", K.STAINLESS, "", 0.3, (-0.085, 0.085), (K.BASE_Y1, K.PIVOT_Y), (-0.085, 0.085)), "column"))
    s.append(lump("column", 0.0, 0.46, (0.05, 0.07, 0.05) if scale != "500" else (0.06, 0.08, 0.06)))
    if fine:
        s.append(of("tilt-pin", "top", "joint"))
    else:
        s.append(own(K.cyl_z("joint-tilt-pin", "top", K.STAINLESS, "", 0.3, 0.0, K.PIVOT_Y, (-0.1, 0.1), max(0.014, f / 2)), "top", "joint"))
    if mid:
        s += [of("saddle-cheek-r", "top"), of("saddle-cheek-l", "top")]
    else:
        s.append(own(K.box("member-top-saddle", "top", K.STAINLESS, "", 0.3, (-0.06, 0.06), (K.FRAME_Y0 - max(0.04, f), K.FRAME_Y0), (-0.09, 0.09)), "top"))
    if fine:
        s += [of("saddle-plate-r", "top"), of("saddle-plate-l", "top"), of("head-plate", "top"), of("head-clamp", "top", "joint"), of("head-rod-r", "top"), of("head-rod-l", "top")]
        s += [of(h, "top", "joint") for h in ("hinge-back-r", "hinge-back-l", "hinge-leg-r", "hinge-leg-l")]
    bw = max(0.025, f)
    beams = (("back", K.BACK_X, 0.25), ("seat", K.SEAT_X, 0.25), ("leg", K.LEG_X, 0.25)) if mid else (("long", (K.BACK_X[0], K.LEG_X[1]), 0.25),)
    for name, x, z1 in beams:
        for zn, side in (("r", -1), ("l", 1)):
            s.append(own(K.box(f"member-top-beam-{name}-{zn}", "top", K.STAINLESS, "", 0.3, x, (K.FRAME_Y0, K.FRAME_Y0 + max(0.04, f)), K.span(side, z1 - bw, z1)), "top"))
    if fine:
        for zn, side in (("r", -1), ("l", 1)):
            s.append(own(K.box(f"member-top-beam-leg-inner-{zn}", "top", K.STAINLESS, "", 0.3, K.LEG_X, (K.FRAME_Y0, K.FRAME_Y1), K.span(side, K.LEG_Z[0], K.LEG_Z[0] + 0.02)), "top"))
    if not fine:
        s.append(own(K.box("member-top-head", "top", K.STAINLESS, "", 0.3, (K.HEAD_X[0], K.BACK_X[0] + 0.03), (0.795, 0.795 + max(0.025, f)), (-0.13, 0.13)), "top"))
    for k, x in enumerate((K.BACK_X[0], -0.02, K.LEG_X[0], K.LEG_X[1] - max(0.03, f))):
        if mid or k != 2:
            s.append(own(K.box(f"member-top-cross-{k + 1}", "top", K.STAINLESS, "", 0.3, (x, x + max(0.03, f)), (K.FRAME_Y0, K.FRAME_Y0 + max(0.026, f)), (-0.25 + bw, 0.25 - bw)), "top"))
    s.append(lump("top", 0.0, 0.85, (0.3, max(0.02, f / 2), 0.15)))
    if scale == "500":
        s.append(own(K.box("member-stand-leg-near", "stand-base", K.STEEL, "", 0.3, (X - 0.28, X - 0.2), (0.0, 0.1), (-K.STAND_HALF_Z, K.STAND_HALF_Z)), "stand-base"))
        s.append(own(K.box("member-stand-leg-far", "stand-base", K.STEEL, "", 0.3, (X + 0.2, X + 0.28), (0.0, 0.1), (-K.STAND_HALF_Z, K.STAND_HALF_Z)), "stand-base"))
    else:
        s += [of("stand-leg-near", "stand-base"), of("stand-leg-far", "stand-base")]
    if "stand-spine" in ids:
        s.append(of("stand-spine", "stand-base"))
    else:
        s.append(own(K.box("member-stand-spine", "stand-base", K.STEEL, "", 0.3, (X - 0.22, X + 0.22), (0.085 if scale != "500" else 0.02, 0.115 if scale != "500" else 0.1), (-max(0.035, f / 2), max(0.035, f / 2))), "stand-base"))
    if fine:
        s.append(of("stand-socket", "stand-column", "joint"))
        for name in ("near", "far"):
            for zn in ("r", "l"):
                s.append(of(f"stand-castor-{name}-{zn}-stem", "stand-base", "joint"))
    s.append(lump("stand-base", X, 0.1 if scale != "500" else 0.06, (0.1, max(0.015, f / 2), 0.06)))
    if fine:
        s.append(own(ring("member-stand-tube", "stand-column", X, (K.BOX_Y1, K.COLUMN_Y1), 0.0225, 0.025, 24), "stand-column"))
        s.append(of("stand-bearing", "stand-column", "joint"))
    else:
        s.append(of("stand-tube", "stand-column"))
        if "stand-bearing" in ids:
            s.append(of("stand-bearing", "stand-column", "joint"))
    s.append(lump("stand-column", X, K.BOX_Y1 + 0.9, (min(max(0.012, f / 2), 0.03), 0.08, min(max(0.012, f / 2), 0.03))))
    if mid:
        link = max(0.008, f)
        gap = 0.0275 - link if fine else 0.033 - link
        x = (K.HEAD_CX + 0.01, X - 0.01)
        s.append(own(K.box("member-arm-link-upper", "stand-arm", K.STEEL, "", 0.3, x, (K.ARM_Y + gap, K.ARM_Y + gap + link), (-0.015, 0.015)), "stand-arm"))
        s.append(own(K.box("member-arm-link-lower", "stand-arm", K.STEEL, "", 0.3, x, (K.ARM_Y - gap - link, K.ARM_Y - gap), (-0.015, 0.015)), "stand-arm"))
        if fine:
            s.append(own(K.tube("member-arm-spring", "stand-arm", K.STEEL, "", 0.3, (X - 0.05, K.ARM_Y + 0.008, 0), (K.HEAD_CX + 0.25, K.ARM_Y - 0.008, 0), 0.009), "stand-arm"))
            s += [of("arm-root", "stand-arm", "joint"), of("arm-tip", "stand-arm", "joint")]
        else:
            for name, x in (("root", X), ("tip", K.HEAD_CX)):
                s.append(own(K.cyl_z("joint-arm-" + name, "stand-arm", K.STEEL, "", 0.3, x + (-0.012 if name == "root" else 0.012), K.ARM_Y, (-0.02, 0.02), 0.02), "stand-arm", "joint"))
    else:
        s.append(of("arm-body", "stand-arm"))
    s += [of(p["id"], "stand-arm", "joint" if p["id"] == "yoke-swivel" else "member") for p in plist if p["id"].startswith("yoke-")]
    s.append(lump("stand-arm", X - 0.25, K.ARM_Y, (0.08, min(max(0.012, f / 2), 0.03), min(max(0.012, f / 2), 0.03))))
    if fine:
        s.append(own(ring("member-head-ring", "light-head", K.HEAD_CX, (K.HEAD_Y - 0.012, K.HEAD_Y + 0.006), 0.222, 0.242), "light-head"))
        s.append(own(K.box("member-head-spar", "light-head", K.ALU, "", 0.3, (K.HEAD_CX - 0.23, K.HEAD_CX + 0.245), (K.HEAD_Y - 0.01, K.HEAD_Y + 0.006), (-0.012, 0.012)), "light-head"))
        s += [of("head-bearing", "light-head", "joint"), of("head-handle", "light-head")]
    else:
        h = max(0.016, f)
        top = K.HEAD_Y + 0.01 if scale != "500" else 1.735
        reach = (0.23, 0.285) if scale != "500" else (0.18, 0.18)
        s.append(own(K.box("member-head-spar", "light-head", K.ALU, "", 0.3, (K.HEAD_CX - reach[0], K.HEAD_CX + reach[1]), (top - h, top), (-max(0.012, f / 2), max(0.012, f / 2))), "light-head"))
        if scale != "500":
            s.append(own(K.box("member-head-cross", "light-head", K.ALU, "", 0.3, (K.HEAD_CX - max(0.012, f / 2), K.HEAD_CX + max(0.012, f / 2)), (top - h, top), (-0.23, 0.23)), "light-head"))
    s.append(lump("light-head", K.HEAD_CX, K.HEAD_Y + 0.02 if scale != "500" else 1.71, (0.1, max(0.015, f / 2), 0.1)))
    return s + supports(scale)


def text(en, de):
    """💬 A bilingual sentence."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": [
        text("Body, the full construction: an H-shaped steel chassis (spine with chamfered shoulders, two rounded cross beams, plinth) on four twin-wheel castors of 100 mm with brake pedals at the sides; a telescopic column of three stainless shrouds with flange, cap and the bearing block of the tilt pin; the top as four section trays (back, seat, two leg plates) with hinge barrels, accessory rails 25 x 10 mm on posts, the head plate on two rods and a clamp, the saddle cheeks and plates that hang the seat section on the pin, and five pads with rounded edges. The light stand has two legs on twin castors of 75 mm, spine, ballast box, socket, 50 mm tube with the driver box, the bearing head, a spring arm with both joint barrels, a half yoke of bent 25 mm tube and the lamp head turned as one housing with lens seat, rim, cooling step and dome, the lens and the sterile handle.", "Körper in voller Konstruktion: ein H-förmiges Stahlfahrgestell (Mittelträger mit gefasten Schultern, zwei gerundete Querträger, Sockel) auf vier Doppelrollen von 100 mm mit seitlichen Bremspedalen; eine Teleskopsäule aus drei Edelstahlhüllen mit Flansch, Deckel und dem Lagerbock des Kippbolzens; die Platte als vier Segmentwannen (Rücken, Sitz, zwei Beinplatten) mit Gelenkhülsen, Zubehörschienen 25 x 10 mm auf Stützen, die Kopfplatte auf zwei Stäben mit Klemmstück, Sattelwangen und Sattelplatten, die das Sitzsegment am Bolzen aufhängen, und fünf Polster mit gerundeten Kanten. Der Leuchtenständer hat zwei Fußholme auf Doppelrollen von 75 mm, Mittelholm, Ballastkasten, Fußhülse, ein 50-mm-Rohr mit Treiberkasten, den Lagerkopf, einen Federarm mit beiden Gelenkhülsen, einen Halbbügel aus gebogenem 25-mm-Rohr und den Leuchtenkopf als ein gedrehtes Gehäuse mit Linsensitz, Rand, Kühlstufe und Kuppel, die Linse und den Sterilgriff."),
        text("Thermal: every body surface belongs to its region; each pad is its 1 mm vinyl skin over a foam core of its own; the lens counts its room face and rim only; the plume leaves through a skin on the crown of the dome. Structure: chassis bars, wheels and contact pads, the two hollow lifting sections inside the shrouds, bearing block, tilt pin, saddle, the side beams and cross bars of the sections, head rods, stand legs, the 2.5 mm tube, the two links and the spring of the arm, the yoke, and a ring with spar as the carrier of the lamp head; one mass per bone at its centre of mass.", "Thermik: jede Körperfläche gehört zu ihrer Region; jedes Polster ist seine 1 mm starke Vinylhaut über einem eigenen Schaumkern; die Linse zählt nur Raumseite und Rand; die Fahne verlässt die Kuppel durch eine Haut auf ihrem Scheitel. Tragwerk: Fahrgestellholme, Räder und Aufstandsflächen, die zwei hohlen Hubprofile in den Hüllen, Lagerbock, Kippbolzen, Sattel, die Längsholme und Querstege der Segmente, Kopfstäbe, Ständerholme, das 2,5-mm-Rohr, die zwei Lenker und die Feder des Arms, der Bügel sowie Ring und Holm als Träger des Leuchtenkopfes; je Knochen eine Masse im Schwerpunkt."),
    ],
    "100": [
        text("Stated anew for 15 mm: chassis as three rounded bars on four single wheels, the column as two shrouds under a faceted bearing block, the top sections as plain 40 mm slabs, each accessory rail merged with its posts into one bar, the head plate reaching to the back section, pads with a single-step rounding; the stand with single wheels, a ballast bar instead of spine and box, plain bearing head, the arm as one body with tapered ends, the yoke bend as one chamfer piece and a head with fewer profile points.", "Für 15 mm neu aufgestellt: Fahrgestell als drei gerundete Holme auf vier Einzelrädern, die Säule als zwei Hüllen unter einem facettierten Lagerbock, die Segmente als schlichte 40-mm-Platten, jede Zubehörschiene mit ihren Stützen zu einem Stab zusammengefasst, die Kopfplatte bis zum Rückensegment, Polster mit einstufiger Rundung; der Ständer mit Einzelrädern, einem Ballastholm statt Mittelholm und Kasten, schlichtem Lagerkopf, dem Arm als ein Körper mit verjüngten Enden, dem Bügelbogen als einem Fasenstück und einem Kopf mit weniger Profilpunkten."),
        text("Thermal: the body surfaces per region, pads without a separate core, the plume skin 15 mm deep in the dome. Structure: chassis bars and wheels, two hollow lifting sections, bearing block, pin, cheeks, side beams and cross bars, a head carrier, stand legs, tube, two arm links between joint barrels, yoke, and a cross of two bars in the lamp head.", "Thermik: die Körperflächen je Region, Polster ohne eigenen Kern, die Fahnenhaut 15 mm tief in der Kuppel. Tragwerk: Fahrgestellholme und Räder, zwei hohle Hubprofile, Lagerbock, Bolzen, Wangen, Längsholme und Querstege, ein Kopfträger, Ständerholme, Rohr, zwei Armlenker zwischen Gelenkhülsen, Bügel und ein Kreuz aus zwei Stäben im Leuchtenkopf."),
    ],
    "200": [
        text("Stated anew for 30 mm: the chassis as an H of three bars on two rollers per axle line, a two-step column with a head block, one frame slab that takes the rails into its edge, the head plate under its pad, a body pad and a leg pad; the stand as two legs on rollers, ballast box, tube, bearing head, a box arm, an L yoke of two 30 mm bars and the lamp head turned as one body with the handle.", "Für 30 mm neu aufgestellt: das Fahrgestell als H aus drei Holmen auf je einer Walze pro Achslinie, eine zweistufige Säule mit Kopfblock, eine Rahmenplatte, die die Schienen in ihre Kante aufnimmt, die Kopfplatte unter ihrem Polster, ein Rumpfpolster und ein Beinpolster; der Ständer als zwei Holme auf Walzen, Ballastkasten, Rohr, Lagerkopf, ein Kastenarm, ein L-Bügel aus zwei 30-mm-Stäben und der Leuchtenkopf als ein gedrehter Körper mit Griff."),
        text("Thermal: body surfaces per region; the head body stands for housing and lens. Structure: chassis bars, rollers, one lifting column, pin, saddle block, two long side beams with three cross bars and the head carrier, stand legs with a spine, tube, arm, yoke bars and a cross in the lamp head.", "Thermik: Körperflächen je Region; der Kopfkörper steht für Gehäuse und Linse. Tragwerk: Fahrgestellholme, Walzen, eine Hubsäule, Bolzen, Sattelblock, zwei lange Längsholme mit drei Querstegen und dem Kopfträger, Ständerholme mit Mittelholm, Rohr, Arm, Bügelstäbe und ein Kreuz im Leuchtenkopf."),
    ],
    "500": [
        text("Stated anew for 75 mm and already minimal: base slab, column block, the top as one slab with the narrower head piece; the stand as foot slab, a 60 mm column, arm, a hanger and the lamp head as a chamfered disc.", "Für 75 mm neu aufgestellt und bereits minimal: Fußplatte, Säulenblock, die Platte als ein Quader mit dem schmaleren Kopfteil; der Ständer als Fußplatte, 60-mm-Säule, Arm, Hänger und der Leuchtenkopf als gefaste Scheibe."),
        text("Thermal: the slabs stand for pad and frame, the disc for housing and lens. Structure: chassis bars with two foot bars, lifting column, pin and saddle block, two side beams with three cross bars and the head carrier, two stand legs with a spine, column, arm, hanger and one spar in the lamp head.", "Thermik: die Quader stehen für Polster und Rahmen, die Scheibe für Gehäuse und Linse. Tragwerk: Fahrgestellholme mit zwei Fußholmen, Hubsäule, Bolzen und Sattelblock, zwei Längsholme mit drei Querstegen und dem Kopfträger, zwei Ständerholme mit Mittelholm, Säule, Arm, Hänger und ein Holm im Leuchtenkopf."),
    ],
}


def volume(scale):
    """📦 The authored volume of one scale and its parts."""
    plist = parts(scale)
    return {"status": "authored", "denominator": int(scale), "body": body(plist), "thermal": thermal(scale, plist), "structure": structure(scale, plist), "decisions": DECISIONS[scale]}, plist
