"""🪑 Round 3, furniture/office-swivel-chair: the handcrafted body of every authored scale, stated solid by solid in world metres (x ahead, y up, z left; seat-low rest pose).

The chair is an upholstered task chair after EN 1335-1: five-star glass-fibre polyamide base on hooded twin-wheel castors of 50 mm (EN 12529), gas spring with a 50 mm standpipe and a 28 mm piston tube, steel tilt mechanism with seat plate, levers and tension knob, plastic seat pan with a foam cushion and a waterfall front, height-adjustable T-armrests, and a doubly curved backrest (lumbar S-curve, wrapped about the sitter) of a polyamide shell with an upholstered pad on an L-shaped steel carrier.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("chair_kit", os.path.join(HERE, "r3-office-swivel-chair-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

PA, STEEL, FABRIC, RUBBER = "pa-plastic", "steel-painted", "fabric-upholstery", "rubber"
UP, AHEAD, LEFT = (0, 1, 0), (1, 0, 0), (0, 0, 1)
ARMS = ["front", "side-a", "side-b", "back-a", "back-b"]

BACK_AXIS_TILT = math.radians(10.0)
BACK_AXIS_AT = (0.207, 0.645)
BACK_AXIS = (-math.sin(BACK_AXIS_TILT), math.cos(BACK_AXIS_TILT), 0.0)
BACK_TOWARD = (-math.cos(BACK_AXIS_TILT), -math.sin(BACK_AXIS_TILT), 0.0)

PAD_FRONT = [(0.505, -0.2660), (0.515, -0.2630), (0.530, -0.2580), (0.580, -0.2475), (0.645, -0.2430), (0.720, -0.2500), (0.800, -0.2660), (0.880, -0.2870), (0.950, -0.3075), (0.978, -0.3160), (0.990, -0.3200)]
PAD_THICK = [0.004, 0.016, 0.024, 0.030, 0.032, 0.030, 0.028, 0.026, 0.022, 0.014, 0.004]
SHELL_Y = [0.497, 0.505, 0.530, 0.580, 0.645, 0.720, 0.800, 0.880, 0.950, 0.990, 0.998]
SHELL_FRONT = [-0.2650, -0.2680, -0.2800, -0.2755, -0.2730, -0.2780, -0.2920, -0.3110, -0.3275, -0.3220, -0.3210]
SHELL_BACK = [-0.2730, -0.2810, -0.2950, -0.2905, -0.2880, -0.2930, -0.3070, -0.3260, -0.3420, -0.3350, -0.3270]
SIDE_Y = [0.542, 0.600, 0.700, 0.800, 0.900, 0.943]
SIDE_Z = [0.213, 0.218, 0.220, 0.216, 0.208, 0.205]
SHELL_WING_Y = {50: [0.497, 0.500, 0.505, 0.512, 0.520, 0.530, 0.542, 0.580, 0.645, 0.720, 0.800, 0.880, 0.920, 0.943, 0.960, 0.975, 0.986, 0.994, 0.998], 100: [0.497, 0.505, 0.520, 0.542, 0.645, 0.800, 0.943, 0.975, 0.992, 0.998], 200: [0.497, 0.512, 0.542, 0.645, 0.800, 0.943, 0.980, 0.998]}
PAD_WING_Y = {50: [0.505, 0.509, 0.515, 0.522, 0.530, 0.542, 0.580, 0.645, 0.720, 0.800, 0.880, 0.920, 0.950, 0.965, 0.978, 0.986, 0.990], 100: [0.505, 0.513, 0.530, 0.645, 0.800, 0.943, 0.972, 0.990]}


def interp(xs, ys, x):
    if x <= xs[0]:
        return ys[0]
    for i in range(1, len(xs)):
        if x <= xs[i]:
            t = (x - xs[i - 1]) / (xs[i] - xs[i - 1])
            return ys[i - 1] + t * (ys[i] - ys[i - 1])
    return ys[-1]


def edge(y):
    """🪟 Half width of the backrest outline at the height y: sides that widen to the lumbar, a 45 mm quarter round at the bottom corner and a 55 mm quarter round at the top corner."""
    if y < 0.542:
        return 0.168 + math.sqrt(max(0.0, 0.045 ** 2 - (0.542 - y) ** 2))
    if y > 0.943:
        return 0.150 + math.sqrt(max(0.0, 0.055 ** 2 - (y - 0.943) ** 2))
    return interp(SIDE_Y, SIDE_Z, y)


def back_coords(x, y):
    """📐 A point of the symmetry plane as [axial, radial] of the backrest axis (tilted 10 degrees back, in front of the sitter)."""
    dx, dy = x - BACK_AXIS_AT[0], y - BACK_AXIS_AT[1]
    return [dx * BACK_AXIS[0] + dy * BACK_AXIS[1], dx * BACK_TOWARD[0] + dy * BACK_TOWARD[1]]


def back_point(axial, radial, azimuth):
    """📍 The world point at [axial, radial] turned by the azimuth (radians, positive to the left) about the backrest axis."""
    c, s = math.cos(azimuth), math.sin(azimuth)
    return (BACK_AXIS_AT[0] + axial * BACK_AXIS[0] + radial * c * BACK_TOWARD[0], BACK_AXIS_AT[1] + axial * BACK_AXIS[1] + radial * c * BACK_TOWARD[1], radial * s)


def pad_x(y):
    return interp([row[0] for row in PAD_FRONT], [row[1] for row in PAD_FRONT], y)


def pad_t(y):
    return interp([row[0] for row in PAD_FRONT], PAD_THICK, y)


def back_loop(front, back):
    """🔁 A closed [axial, radial] loop from the front curve (bottom to top) and the back curve of a layer of the backrest."""
    loop = [back_coords(x, y) for y, x in front] + [back_coords(x, y) for y, x in reversed(back)]
    return loop + [loop[0]]


def back_patch(solid_id, material, role, tone, front, back, sweep_deg):
    return K.patch(solid_id, "backrest", material, role, tone, (BACK_AXIS_AT[0], BACK_AXIS_AT[1], 0), BACK_AXIS, BACK_TOWARD, sweep_deg, back_loop(front, back))


def back_wing(solid_id, material, role, tone, side, stations, exponent=3.0):
    """🪽 The side band of a layer of the backrest: a loft up the lumbar curve, each station (y, x front, x back, z inner, z outer) a flat rounded section tangent to the wrap."""
    sections = []
    for y, xf, xb, z_in, z_out in stations:
        af, rf = back_coords(xf, y)
        ab, rb = back_coords(xb, y)
        axial, radial = (af + ab) / 2, (rf + rb) / 2
        zc = (z_in + z_out) / 2
        azimuth = math.asin(zc / radial)
        at = back_point(axial, radial, side * azimuth)
        tangent = (math.sin(azimuth) * -BACK_TOWARD[0], math.sin(azimuth) * -BACK_TOWARD[1], side * math.cos(azimuth))
        normal = (math.cos(azimuth) * BACK_TOWARD[0], math.cos(azimuth) * BACK_TOWARD[1], side * math.sin(azimuth))
        sections.append((at, (z_out - z_in) / 2 / math.cos(azimuth), abs(rb - rf) / 2, tangent, normal))
    return K.loft(solid_id, "backrest", material, role, tone, sections, exponent)


def arm_point(arm, s, y):
    a = math.radians(K.AZIMUTH[arm])
    return (s * math.cos(a), y, s * math.sin(a))


def arm_across(arm):
    a = math.radians(K.AZIMUTH[arm])
    return (-math.sin(a), 0, math.cos(a))


def arm_loft(arm, stations, exponent=3.0):
    """🌟 One arm of the star: a loft along its radius, each station (radius, centre height, half width, half height)."""
    return K.loft(f"arm-{arm}", f"arm-{arm}", PA, "body", 0.3, [(arm_point(arm, s, yc), hw, hh, arm_across(arm), UP) for s, yc, hw, hh in stations], exponent)


def castor_at(arm):
    px, pz = K.pivot_of(arm)
    return px, pz, px - K.TRAIL


def seat_loft(solid_id, material, role, tone, stations):
    """🛋 A loft along the depth of the seat, each station (x, centre height, half width, half height, exponent)."""
    return K.loft(solid_id, "seat", material, role, tone, [((x, yc, 0), hw, hh, LEFT, UP, e) for x, yc, hw, hh, e in stations])


def path_loft(solid_id, bone, material, role, tone, points, z, half_width, half_thickness, exponent=4.0, widths=None):
    """➰ A strap bent in a vertical plane: stations (x, y) at the lateral position z, the section normal to the path, as wide (lateral) as half_width and as thick as half_thickness."""
    sections = []
    for i, (x, y) in enumerate(points):
        a, b = points[max(0, i - 1)], points[min(len(points) - 1, i + 1)]
        tx, ty = b[0] - a[0], b[1] - a[1]
        n = math.hypot(tx, ty)
        sections.append(((x, y, z), widths[i] if widths else half_width, half_thickness, LEFT, (-ty / n, tx / n, 0)))
    return K.loft(solid_id, bone, material, role, tone, sections, exponent)


def armrest_post(solid_id, side, stations, x=-0.02, exponent=4.0):
    """💪 The carrier of an armrest: a strap from under the seat that bends up into the post; stations (|z|, y, half depth along x, half thickness)."""
    sections = []
    for i, (z, y, hx, ht) in enumerate(stations):
        a, b = stations[max(0, i - 1)], stations[min(len(stations) - 1, i + 1)]
        tz, ty = b[0] - a[0], b[1] - a[1]
        n = math.hypot(tz, ty)
        sections.append(((x, y, side * z), hx, ht, AHEAD, (0, tz / n, -side * ty / n)))
    cut = 3 if len(sections) > 4 else 2
    name = solid_id.replace("armrest-post-", "")
    return [K.loft(f"armrest-strap-{name}", "seat", PA, "body", 0.35, sections[:cut], exponent), K.loft(solid_id, "seat", PA, "body", 0.35, sections[cut - 1:], exponent)]


def armrest_pad(solid_id, side, stations, z=0.2875):
    return K.loft(solid_id, "seat", PA, "body", 0.4, [((x, yc, side * z), hw, hh, LEFT, UP, e) for x, yc, hw, hh, e in stations])


def body50():
    s = []
    s.append(K.revolve("hub", "base", PA, "body", 0.3, (0, 0, 0), UP, [(0.058, 0.0255), (0.058, 0.038), (0.063, 0.042), (0.112, 0.039), (0.122, 0.036), (0.127, 0.031), (0.127, 0.0255), (0.058, 0.0255)]))
    for arm in ARMS:
        px, pz, ax = castor_at(arm)
        s.append(arm_loft(arm, [(0.030, 0.0945, 0.0270, 0.0265), (0.065, 0.0915, 0.0255, 0.0245), (0.150, 0.0845, 0.0215, 0.0205), (0.235, 0.0775, 0.0180, 0.0165), (0.292, 0.0730, 0.0160, 0.0140)]))
        s.append(K.revolve(f"arm-{arm}-boss", f"arm-{arm}", PA, "body", 0.28, (px, 0, pz), UP, [(0.058, 0), (0.058, 0.0195), (0.060, 0.021), (0.083, 0.021), (0.088, 0.018), (0.0895, 0.012), (0.0895, 0)]))
        bone = f"castor-{arm}"
        s.append(K.revolve(f"castor-{arm}-neck", bone, STEEL, "detail", 0.2, (px, 0, pz), UP, [(0.050, 0), (0.050, 0.008), (0.058, 0.008), (0.058, 0)]))
        s.append(K.revolve(f"castor-{arm}-head", bone, PA, "body", 0.25, (px, 0, pz), UP, [(0.034, 0), (0.034, 0.011), (0.038, 0.013), (0.050, 0.013), (0.052, 0.011), (0.052, 0)]))
        s.append(K.extrude_xy(f"castor-{arm}-web", bone, PA, "body", 0.25, [(ax + 0.033, 0.052), (ax + 0.034, 0.040), (ax + 0.022, 0.026), (ax + 0.011, 0.016), (ax, 0.0125), (ax - 0.010, 0.016), (ax - 0.022, 0.030), (ax - 0.0235, 0.040), (ax - 0.016, 0.049), (ax - 0.004, 0.0535), (ax + 0.012, 0.0545)], pz - 0.0105, pz + 0.0105))
        outer = [(ax + 0.0295 * math.cos(math.radians(t)), K.AXLE_Y + 0.0295 * math.sin(math.radians(t))) for t in (12, 35, 60, 90, 120, 145, 168)]
        inner = [(ax + 0.0265 * math.cos(math.radians(t)), K.AXLE_Y + 0.0265 * math.sin(math.radians(t))) for t in (168, 145, 120, 90, 60, 35, 12)]
        s.append(K.extrude_xy(f"castor-{arm}-hood", bone, PA, "body", 0.27, outer + inner, pz - 0.025, pz + 0.025))
        for name, sign in (("l", 1), ("r", -1)):
            centre = (ax, K.AXLE_Y, pz + sign * 0.0185)
            s.append(K.revolve(f"castor-{arm}-tyre-{name}", bone, RUBBER, "body", 0.2, centre, (0, 0, sign), [(-0.0065, 0.017), (-0.0065, 0.0225), (-0.0045, 0.025), (0.0045, 0.025), (0.0065, 0.0225), (0.0065, 0.017), (-0.0065, 0.017)]))
            s.append(K.revolve(f"castor-{arm}-rim-{name}", bone, PA, "detail", 0.3, centre, (0, 0, sign), [(-0.0065, 0), (-0.0065, 0.0172), (0.0065, 0.0172), (0.0078, 0.012), (0.0078, 0)]))
    s.append(K.revolve("column-sleeve", "column-sleeve", STEEL, "body", 0.25, (0, 0, 0), UP, [(0.036, 0), (0.036, 0.008), (0.042, 0.008), (0.042, 0.0225), (0.060, 0.0245), (0.127, 0.0255), (0.132, 0.025), (0.279, 0.025), (0.279, 0)]))
    s.append(K.revolve("column-collar", "column-sleeve", STEEL, "detail", 0.15, (0, 0, 0), UP, [(0.272, 0), (0.272, 0.0265), (0.285, 0.0265), (0.287, 0.0245), (0.287, 0)]))
    s.append(K.revolve("column", "column", STEEL, "body", 0.7, (0, 0, 0), UP, [(0.150, 0), (0.150, 0.014), (0.360, 0.014), (0.360, 0)]))
    s.append(K.revolve("column-cone", "column", STEEL, "body", 0.22, (0, 0, 0), UP, [(0.314, 0), (0.314, 0.0165), (0.343, 0.0215), (0.343, 0)]))
    s.append(K.loft("seat-mechanism", "seat", STEEL, "body", 0.22, [((-0.020, 0.341, 0), 0.085, 0.055, AHEAD, LEFT), ((-0.020, 0.352, 0), 0.105, 0.068, AHEAD, LEFT), ((-0.025, 0.378, 0), 0.125, 0.080, AHEAD, LEFT)], 5.0))
    s.append(K.box("seat-plate", "seat", STEEL, "body", 0.2, (-0.010, 0.381, 0), (0.30, 0.008, 0.20), 0.003))
    s.append(K.box("seat-hinge", "seat", STEEL, "body", 0.2, (-0.175, 0.364, 0), (0.070, 0.028, 0.070), 0.005))
    s.append(K.revolve("seat-hinge-pin", "seat", STEEL, "detail", 0.15, (K.PIVOT[0], K.PIVOT[1], 0), LEFT, [(-0.042, 0), (-0.042, 0.009), (0.042, 0.009), (0.042, 0)]))
    s.append(K.capsule("seat-lever", "seat", STEEL, "detail", 0.2, (0.020, 0.362, -0.070), (0.036, 0.368, -0.236), 0.004, 0.004))
    s.append(K.ellipsoid("seat-lever-paddle", "seat", PA, "detail", 0.25, (0.040, 0.369, -0.250), (0.028, 0.005, 0.016)))
    s.append(K.capsule("seat-lock-lever", "seat", STEEL, "detail", 0.2, (-0.060, 0.362, 0.070), (-0.050, 0.368, 0.236), 0.004, 0.004))
    s.append(K.ellipsoid("seat-lock-paddle", "seat", PA, "detail", 0.25, (-0.046, 0.369, 0.250), (0.028, 0.005, 0.016)))
    s.append(K.revolve("seat-knob", "seat", PA, "detail", 0.25, (0, 0.357, 0), AHEAD, [(0.092, 0), (0.092, 0.018), (0.104, 0.024), (0.130, 0.024), (0.135, 0.020), (0.135, 0)]))
    s.append(seat_loft("seat-shell", PA, "detail", 0.3, [(0.232, 0.4030, 0.190, 0.0060, 4), (0.215, 0.4000, 0.215, 0.0100, 4), (0.170, 0.3970, 0.228, 0.0120, 4), (0.000, 0.3970, 0.232, 0.0120, 4), (-0.170, 0.3980, 0.222, 0.0120, 4), (-0.205, 0.4010, 0.205, 0.0100, 4), (-0.215, 0.4040, 0.180, 0.0060, 4)]))
    s.append(seat_loft("seat-cushion", FABRIC, "body", 0.62, [(0.240, 0.4180, 0.195, 0.0100, 3.5), (0.232, 0.4200, 0.220, 0.0170, 4), (0.212, 0.4215, 0.234, 0.0215, 4.5), (0.160, 0.4225, 0.240, 0.0225, 5), (0.000, 0.4215, 0.240, 0.0215, 5), (-0.140, 0.4235, 0.234, 0.0235, 5), (-0.190, 0.4250, 0.224, 0.0240, 4.5), (-0.212, 0.4250, 0.205, 0.0200, 4), (-0.220, 0.4250, 0.180, 0.0120, 3.5)]))
    for name, side in (("left", 1), ("right", -1)):
        s.extend(armrest_post(f"armrest-post-{name}", side, [(0.170, 0.3770, 0.030, 0.0060), (0.246, 0.3770, 0.030, 0.0070), (0.271, 0.3850, 0.029, 0.0105), (0.283, 0.4050, 0.028, 0.0140), (0.285, 0.4350, 0.028, 0.0150), (0.285, 0.5650, 0.028, 0.0150)]))
        s.append(K.loft(f"armrest-stem-{name}", "seat", PA, "body", 0.3, [((-0.02, 0.540, side * 0.285), 0.024, 0.0115, AHEAD, LEFT), ((-0.02, 0.638, side * 0.285), 0.024, 0.0115, AHEAD, LEFT)], 4.0))
        s.append(K.loft(f"armrest-cap-{name}", "seat", PA, "body", 0.3, [((-0.105, 0.6400, side * 0.2865), 0.026, 0.0065, LEFT, UP), ((-0.060, 0.6360, side * 0.2865), 0.032, 0.0090, LEFT, UP), ((0.030, 0.6360, side * 0.2865), 0.032, 0.0090, LEFT, UP), ((0.075, 0.6400, side * 0.2865), 0.026, 0.0065, LEFT, UP)], 4.0))
        s.append(armrest_pad(f"armrest-pad-{name}", side, [(-0.140, 0.6475, 0.0240, 0.0080, 3), (-0.132, 0.6475, 0.0350, 0.0115, 3.5), (-0.112, 0.6475, 0.0415, 0.0125, 4), (0.000, 0.6475, 0.0425, 0.0125, 4), (0.075, 0.6470, 0.0425, 0.0125, 4), (0.098, 0.6455, 0.0360, 0.0115, 3.5), (0.110, 0.6440, 0.0240, 0.0080, 3)]))
    s.append(path_loft("backrest-strut-foot", "backrest", STEEL, "body", 0.22, [(-0.160, 0.3640), (-0.238, 0.3640)], 0, 0.030, 0.005))
    s.append(path_loft("backrest-strut", "backrest", STEEL, "body", 0.22, [(-0.230, 0.3640), (-0.238, 0.3640), (-0.263, 0.3720), (-0.283, 0.3950), (-0.293, 0.4300), (-0.2985, 0.5000), (-0.3010, 0.5800), (-0.2995, 0.6800)], 0, 0.030, 0.005))
    s.append(K.box("backrest-bracket", "backrest", PA, "body", 0.35, (-0.2985, 0.600, 0), (0.026, 0.180, 0.100), 0.008))
    pad_front = PAD_FRONT
    pad_back = [(y, x - t) for (y, x), t in zip(PAD_FRONT, PAD_THICK)]
    shell_front = list(zip(SHELL_Y, SHELL_FRONT))
    shell_back = list(zip(SHELL_Y, SHELL_BACK))
    s.append(back_patch("backrest-shell", PA, "body", 0.5, shell_front, shell_back, 40.0))
    s.append(back_patch("backrest-pad", FABRIC, "body", 0.6, pad_front, pad_back, 39.0))
    for name, side in (("left", 1), ("right", -1)):
        shell_stations = [(y, interp(SHELL_Y, SHELL_FRONT, y), interp(SHELL_Y, SHELL_BACK, y) + 0.0025, 0.132, edge(y) + 0.004) for y in SHELL_WING_Y[50]]
        s.append(back_wing(f"backrest-shell-{name}", PA, "body", 0.5, side, shell_stations))
        pad_stations = [(y, pad_x(y), pad_x(y) - max(pad_t(y), 0.008), 0.130, edge(y) - 0.004) for y in PAD_WING_Y[50]]
        s.append(back_wing(f"backrest-pad-{name}", FABRIC, "body", 0.6, side, pad_stations))
    return s


def pick(rows, *indices):
    return [rows[i] for i in indices]


def body100():
    s = []
    s.append(K.revolve("hub", "base", PA, "body", 0.3, (0, 0, 0), UP, [(0.058, 0.0255), (0.058, 0.040), (0.118, 0.038), (0.127, 0.032), (0.127, 0.0255), (0.058, 0.0255)]))
    for arm in ARMS:
        px, pz, ax = castor_at(arm)
        s.append(arm_loft(arm, [(0.030, 0.0945, 0.0270, 0.0265), (0.150, 0.0845, 0.0215, 0.0205), (0.292, 0.0730, 0.0160, 0.0140)]))
        s.append(K.revolve(f"arm-{arm}-boss", f"arm-{arm}", PA, "body", 0.28, (px, 0, pz), UP, [(0.058, 0), (0.058, 0.021), (0.086, 0.021), (0.0895, 0.016), (0.0895, 0)]))
        bone = f"castor-{arm}"
        s.append(K.revolve(f"castor-{arm}-head", bone, PA, "body", 0.25, (px, 0, pz), UP, [(0.034, 0), (0.034, 0.013), (0.058, 0.013), (0.058, 0)]))
        outer = [(ax + 0.0295 * math.cos(math.radians(t)), K.AXLE_Y + 0.0295 * math.sin(math.radians(t))) for t in (12, 50, 90, 130, 168)]
        inner = [(ax + 0.0140 * math.cos(math.radians(t)), K.AXLE_Y + 0.0140 * math.sin(math.radians(t))) for t in (168, 90, 12)]
        s.append(K.extrude_xy(f"castor-{arm}-hood", bone, PA, "body", 0.27, outer + inner, pz - 0.0105, pz + 0.0105))
        for name, sign in (("l", 1), ("r", -1)):
            s.append(K.revolve(f"castor-{arm}-wheel-{name}", bone, RUBBER, "body", 0.2, (ax, K.AXLE_Y, pz + sign * 0.0175), (0, 0, sign), [(-0.0075, 0), (-0.0075, 0.0225), (-0.005, 0.025), (0.005, 0.025), (0.0075, 0.0225), (0.0075, 0)]))
    s.append(K.revolve("column-sleeve", "column-sleeve", STEEL, "body", 0.25, (0, 0, 0), UP, [(0.040, 0), (0.040, 0.0225), (0.127, 0.0255), (0.272, 0.0255), (0.272, 0.0265), (0.287, 0.0265), (0.287, 0)]))
    s.append(K.revolve("column", "column", STEEL, "body", 0.7, (0, 0, 0), UP, [(0.150, 0), (0.150, 0.014), (0.314, 0.014), (0.314, 0.0165), (0.343, 0.0215), (0.343, 0)]))
    s.append(K.loft("seat-mechanism", "seat", STEEL, "body", 0.22, [((-0.020, 0.341, 0), 0.088, 0.057, AHEAD, LEFT), ((-0.025, 0.385, 0), 0.130, 0.084, AHEAD, LEFT)], 5.0))
    s.append(K.box("seat-hinge", "seat", STEEL, "body", 0.2, (-0.175, 0.364, 0), (0.070, 0.028, 0.070)))
    s.append(K.revolve("seat-knob", "seat", PA, "detail", 0.25, (0, 0.357, 0), AHEAD, [(0.092, 0), (0.092, 0.024), (0.135, 0.022), (0.135, 0)]))
    s.append(seat_loft("seat-shell", PA, "detail", 0.3, [(0.230, 0.4020, 0.195, 0.0075, 4), (0.180, 0.3970, 0.228, 0.0120, 4), (0.000, 0.3970, 0.232, 0.0120, 4), (-0.180, 0.3985, 0.220, 0.0120, 4), (-0.214, 0.4035, 0.185, 0.0075, 4)]))
    s.append(seat_loft("seat-cushion", FABRIC, "body", 0.62, [(0.240, 0.4185, 0.198, 0.0105, 3.5), (0.220, 0.4210, 0.230, 0.0205, 4), (0.160, 0.4225, 0.240, 0.0225, 5), (0.000, 0.4215, 0.240, 0.0215, 5), (-0.150, 0.4240, 0.233, 0.0235, 5), (-0.205, 0.4250, 0.212, 0.0215, 4), (-0.220, 0.4250, 0.182, 0.0125, 3.5)]))
    for name, side in (("left", 1), ("right", -1)):
        s.extend(armrest_post(f"armrest-post-{name}", side, [(0.170, 0.3780, 0.030, 0.0080), (0.248, 0.3780, 0.030, 0.0080), (0.278, 0.3950, 0.029, 0.0120), (0.285, 0.4350, 0.028, 0.0150), (0.285, 0.6380, 0.027, 0.0140)]))
        s.append(armrest_pad(f"armrest-pad-{name}", side, [(-0.140, 0.6475, 0.0250, 0.0085, 3), (-0.118, 0.6475, 0.0410, 0.0125, 4), (0.080, 0.6470, 0.0425, 0.0125, 4), (0.110, 0.6440, 0.0250, 0.0085, 3)]))
    s.append(path_loft("backrest-strut-foot", "backrest", STEEL, "body", 0.22, [(-0.160, 0.3640), (-0.242, 0.3640)], 0, 0.030, 0.005))
    s.append(path_loft("backrest-strut", "backrest", STEEL, "body", 0.22, [(-0.232, 0.3640), (-0.242, 0.3640), (-0.276, 0.3850), (-0.293, 0.4300), (-0.300, 0.5400), (-0.2995, 0.6800)], 0, 0.030, 0.005))
    s.append(K.box("backrest-bracket", "backrest", PA, "body", 0.35, (-0.2985, 0.600, 0), (0.026, 0.180, 0.100)))
    keep = (0, 2, 4, 6, 8, 10)
    pad_front = pick(PAD_FRONT, *keep)
    pad_back = [(y, x - t) for (y, x), t in zip(pad_front, pick(PAD_THICK, *keep))]
    shell_y, shell_f, shell_b = pick(SHELL_Y, *keep), pick(SHELL_FRONT, *keep), pick(SHELL_BACK, *keep)
    s.append(back_patch("backrest-shell", PA, "body", 0.5, list(zip(shell_y, shell_f)), [(y, x - 0.003) for y, x in zip(shell_y, shell_b)], 40.0))
    s.append(back_patch("backrest-pad", FABRIC, "body", 0.6, pad_front, pad_back, 39.0))
    for name, side in (("left", 1), ("right", -1)):
        s.append(back_wing(f"backrest-shell-{name}", PA, "body", 0.5, side, [(y, interp(shell_y, shell_f, y), interp(shell_y, shell_b, y), 0.132, edge(y) + 0.004) for y in SHELL_WING_Y[100]]))
        s.append(back_wing(f"backrest-pad-{name}", FABRIC, "body", 0.6, side, [(y, pad_x(y), pad_x(y) - max(pad_t(y), 0.010), 0.130, edge(y) - 0.004) for y in PAD_WING_Y[100]]))
    return s


BACK200_Y = [0.497, 0.530, 0.645, 0.800, 0.950, 0.998]
BACK200_FRONT = [-0.2650, -0.2580, -0.2430, -0.2660, -0.3075, -0.3210]
BACK200_BACK = [-0.2780, -0.2980, -0.2910, -0.3100, -0.3450, -0.3300]


def body200():
    s = []
    s.append(K.revolve("hub", "base", PA, "body", 0.3, (0, 0, 0), UP, [(0.058, 0), (0.058, 0.040), (0.122, 0.037), (0.127, 0.030), (0.127, 0)]))
    for arm in ARMS:
        px, pz, ax = castor_at(arm)
        s.append(arm_loft(arm, [(0.030, 0.0945, 0.0270, 0.0265), (0.285, 0.0735, 0.0170, 0.0145), (0.318, 0.0735, 0.0150, 0.0140)], 2.6))
        s.append(K.revolve(f"arm-{arm}-boss", f"arm-{arm}", PA, "body", 0.28, (px, 0, pz), UP, [(0.036, 0), (0.036, 0.015), (0.058, 0.019), (0.0895, 0.019), (0.0895, 0)]))
        s.append(K.revolve(f"castor-{arm}-wheel", f"castor-{arm}", RUBBER, "body", 0.2, (ax, K.AXLE_Y, pz), LEFT, [(-0.025, 0), (-0.025, 0.023), (-0.021, 0.025), (0.021, 0.025), (0.025, 0.023), (0.025, 0)]))
    s.append(K.revolve("column-sleeve", "column-sleeve", STEEL, "body", 0.25, (0, 0, 0), UP, [(0.040, 0), (0.040, 0.023), (0.127, 0.0255), (0.287, 0.0255), (0.287, 0)]))
    s.append(K.revolve("column", "column", STEEL, "body", 0.7, (0, 0, 0), UP, [(0.150, 0), (0.150, 0.014), (0.345, 0.014), (0.345, 0)]))
    s.append(K.revolve("column-cone", "column", STEEL, "body", 0.22, (0, 0, 0), UP, [(0.312, 0), (0.312, 0.017), (0.343, 0.022), (0.343, 0)]))
    s.append(K.loft("seat-mechanism", "seat", STEEL, "body", 0.22, [((-0.020, 0.341, 0), 0.090, 0.058, AHEAD, LEFT), ((-0.045, 0.385, 0), 0.150, 0.084, AHEAD, LEFT)], 4.0))
    s.append(seat_loft("seat-cushion", FABRIC, "body", 0.62, [(0.240, 0.4150, 0.200, 0.0150, 3.5), (0.205, 0.4160, 0.236, 0.0290, 4.5), (0.000, 0.4150, 0.240, 0.0290, 5), (-0.180, 0.4170, 0.228, 0.0310, 4.5), (-0.220, 0.4200, 0.185, 0.0170, 3.5)]))
    for name, side in (("left", 1), ("right", -1)):
        s.extend(armrest_post(f"armrest-post-{name}", side, [(0.170, 0.3780, 0.030, 0.0080), (0.255, 0.3780, 0.030, 0.0090), (0.285, 0.4200, 0.028, 0.0150), (0.285, 0.6380, 0.028, 0.0150)]))
        s.append(armrest_pad(f"armrest-pad-{name}", side, [(-0.140, 0.6475, 0.0280, 0.0095, 3), (-0.115, 0.6475, 0.0420, 0.0125, 4), (0.085, 0.6465, 0.0420, 0.0125, 4), (0.110, 0.6440, 0.0280, 0.0095, 3)]))
    s.append(path_loft("backrest-strut-foot", "backrest", STEEL, "body", 0.22, [(-0.160, 0.3640), (-0.247, 0.3640)], 0, 0.030, 0.005))
    s.append(path_loft("backrest-strut", "backrest", STEEL, "body", 0.22, [(-0.235, 0.3640), (-0.247, 0.3640), (-0.290, 0.4100), (-0.3000, 0.5400), (-0.2990, 0.6600)], 0, 0.030, 0.005))
    front, back = list(zip(BACK200_Y, BACK200_FRONT)), list(zip(BACK200_Y, BACK200_BACK))
    s.append(back_patch("backrest-shell", PA, "body", 0.5, front, back, 40.0))
    for name, side in (("left", 1), ("right", -1)):
        s.append(back_wing(f"backrest-shell-{name}", PA, "body", 0.5, side, [(y, interp(BACK200_Y, BACK200_FRONT, y), interp(BACK200_Y, BACK200_BACK, y) + 0.003, 0.130, edge(y)) for y in SHELL_WING_Y[200]], 2.8))
    return s


BACK500_Y = [0.497, 0.560, 0.645, 0.800, 0.920, 0.994]
BACK500_FRONT = [-0.2650, -0.2500, -0.2430, -0.2660, -0.2990, -0.3210]
BACK500_BACK = [-0.2800, -0.2920, -0.2890, -0.3100, -0.3400, -0.3300]


def body500():
    s = []
    s.append(K.revolve("hub", "base", PA, "body", 0.3, (0, 0, 0), UP, [(0.050, 0), (0.050, 0.040), (0.127, 0.036), (0.127, 0)]))
    for arm in ARMS:
        px, pz, ax = castor_at(arm)
        s.append(arm_loft(arm, [(0.030, 0.0930, 0.0270, 0.0270), (0.318, 0.0740, 0.0180, 0.0160)], 2.4))
        s.append(K.revolve(f"arm-{arm}-boss", f"arm-{arm}", PA, "body", 0.28, (px, 0, pz), UP, [(0.040, 0), (0.040, 0.019), (0.090, 0.019), (0.090, 0)]))
        s.append(K.revolve(f"castor-{arm}-wheel", f"castor-{arm}", RUBBER, "body", 0.2, (ax, K.AXLE_Y, pz), LEFT, [(-0.025, 0), (-0.025, 0.025), (0.025, 0.025), (0.025, 0)]))
    s.append(K.revolve("column-sleeve", "column-sleeve", STEEL, "body", 0.25, (0, 0, 0), UP, [(0.045, 0), (0.045, 0.025), (0.287, 0.025), (0.287, 0)]))
    s.append(K.revolve("column", "column", STEEL, "body", 0.7, (0, 0, 0), UP, [(0.160, 0), (0.160, 0.014), (0.345, 0.014), (0.345, 0)]))
    s.append(K.loft("seat-mechanism", "seat", STEEL, "body", 0.22, [((-0.030, 0.341, 0), 0.100, 0.060, AHEAD, LEFT), ((-0.040, 0.386, 0), 0.150, 0.085, AHEAD, LEFT)], 4.0))
    s.append(seat_loft("seat-cushion", FABRIC, "body", 0.62, [(0.240, 0.4150, 0.205, 0.0180, 4), (0.190, 0.4160, 0.240, 0.0300, 5), (-0.170, 0.4165, 0.230, 0.0305, 5), (-0.220, 0.4190, 0.190, 0.0200, 4)]))
    for name, side in (("left", 1), ("right", -1)):
        s.extend(armrest_post(f"armrest-post-{name}", side, [(0.200, 0.3790, 0.030, 0.0090), (0.262, 0.3790, 0.030, 0.0100), (0.285, 0.4250, 0.028, 0.0150), (0.285, 0.6400, 0.028, 0.0150)]))
        s.append(armrest_pad(f"armrest-pad-{name}", side, [(-0.140, 0.6475, 0.0330, 0.0110, 3.5), (-0.015, 0.6475, 0.0425, 0.0125, 4), (0.110, 0.6450, 0.0330, 0.0110, 3.5)]))
    s.append(path_loft("backrest-strut-foot", "backrest", STEEL, "body", 0.22, [(-0.160, 0.3640), (-0.252, 0.3640)], 0, 0.030, 0.005))
    s.append(path_loft("backrest-strut", "backrest", STEEL, "body", 0.22, [(-0.240, 0.3640), (-0.252, 0.3640), (-0.295, 0.4200), (-0.3000, 0.6000)], 0, 0.030, 0.005))
    ys, fronts, backs = BACK500_Y, BACK500_FRONT, BACK500_BACK
    s.append(back_patch("backrest-shell", PA, "body", 0.5, list(zip(ys, fronts)), list(zip(ys, backs)), 50.0))
    return s


def foam50():
    """🧽 The foam cores of the upholstery at 1:50: the cushion and the pad of the backrest inset by the 1 mm of fabric."""
    t = 0.001
    out = [seat_loft("th-seat-cushion-foam", FABRIC, "body", 0.62, [(0.239, 0.4180, 0.194, 0.0090, 3.5), (0.232, 0.4200, 0.219, 0.0160, 4), (0.212, 0.4215, 0.233, 0.0205, 4.5), (0.160, 0.4225, 0.239, 0.0215, 5), (0.000, 0.4215, 0.239, 0.0205, 5), (-0.140, 0.4235, 0.233, 0.0225, 5), (-0.190, 0.4250, 0.223, 0.0230, 4.5), (-0.212, 0.4250, 0.204, 0.0190, 4), (-0.219, 0.4250, 0.179, 0.0110, 3.5)])]
    rows = list(zip(PAD_FRONT, PAD_THICK))[1:-1]
    front = [(y, x - t) for (y, x), _ in rows]
    back = [(y, x - k + t) for (y, x), k in rows]
    out.append(back_patch("th-backrest-pad-foam", FABRIC, "body", 0.6, front, back, 38.8))
    for name, side in (("left", 1), ("right", -1)):
        out.append(back_wing(f"th-backrest-pad-{name}-foam", FABRIC, "body", 0.6, side, [(y, pad_x(y) - t, pad_x(y) - max(pad_t(y), 0.008) + t, 0.130, edge(y) - 0.005) for y in PAD_WING_Y[50][2:-2]]))
    return out


def pad_coarse(scale):
    """🧵 The upholstered pad of the backrest where the body of a coarse scale merges it into the shell: the thermal surface of the fabric region."""
    ys, fronts = (BACK200_Y, BACK200_FRONT) if scale == 200 else (BACK500_Y, BACK500_FRONT)
    rows = list(zip(ys, fronts))
    return back_patch("th-backrest-pad", FABRIC, "body", 0.6, [(y + (0.008 if i == 0 else -0.008 if i == len(rows) - 1 else 0), x) for i, (y, x) in enumerate(rows)], [(y + (0.008 if i == 0 else -0.008 if i == len(rows) - 1 else 0), x - (0.006 if i in (0, len(rows) - 1) else 0.026)) for i, (y, x) in enumerate(rows)], 39.0 if scale == 200 else 49.0)


def tube(solid_id, bone, y0, y1, r_in, r_out, tone=0.3):
    return K.revolve(solid_id, bone, STEEL, "body", tone, (0, 0, 0), UP, [(y0, r_in), (y0, r_out), (y1, r_out), (y1, r_in), (y0, r_in)])
