"""🌬️ Rework 11: the 12 kW monobloc outdoor unit, every solid stated by hand for 1:50, 1:100, 1:200 and 1:500.

Usage: python3 rework-11-heat-pump-outdoor-unit-build.py [--publish]
Frame of bone `unit`: u ahead (fan blows to +u), v up, w to the left; casing u -0.215..0.215, v 0.15..1.425, w -0.55..0.55; fan axis at v 0.8, w -0.15; compressor column w 0.25..0.55.
"""
import math
import sys

from rework_11_lib import *

SLUG = "heat-pump-outdoor-unit"
PC, GS, AL, RB, CU, PS, SS = "powder-coat", "galvanised-steel", "aluminium", "rubber", "copper", "painted-steel", "stainless-steel"
FAN = [0.105, 0.8, -0.15]
AXIS_X, AXIS_Y, AXIS_Z = [1, 0, 0], [0, 1, 0], [0, 0, 1]

BLADE = [
    ([0.04, 0.06932, 0.00974], 0.05, [-0.58455, -0.29784, -0.34263, 0.67245]),
    ([0.04, 0.09768, 0.0214], 0.058, [-0.55353, -0.29432, -0.36576, 0.68789]),
    ([0.04, 0.12699, 0.05893], 0.068, [-0.466, -0.26905, -0.42144, 0.72996]),
    ([0.04, 0.13204, 0.12233], 0.078, [-0.34125, -0.20912, -0.47883, 0.78137]),
    ([0.04, 0.09233, 0.19969], 0.084, [-0.1802, -0.11702, -0.53192, 0.81908]),
    ([0.04, 0.02372, 0.24887], 0.08, [-0.03938, -0.02656, -0.55856, 0.8281]),
    ([0.04, -0.03343, 0.26591], 0.062, [0.0515, 0.0354, -0.5653, 0.82252]),
    ([0.04, -0.06209, 0.26893], 0.03, [0.09329, 0.06412, -0.56277, 0.81883]),
]
BLADE_PICK = {"50": [0, 1, 2, 3, 4, 5, 6, 7], "100": [0, 2, 3, 4, 5, 6, 7], "200": [0, 2, 4, 6, 7], "500": [0, 3, 5, 7]}
BLADE_THICK = {"50": 0.004, "100": 0.004, "200": 0.004, "500": 0.004}


def arc(cx, cy, radius, a0, a1, n):
    return [[r(cx + radius * math.cos(math.radians(a0 + (a1 - a0) * k / n))), r(cy + radius * math.sin(math.radians(a0 + (a1 - a0) * k / n)))] for k in range(n + 1)]


def casing(t, top=1.425, hole=48):
    """The folded sheet casing with sheets of thickness `t`."""
    return [
        ext("front-panel", "unit", PC, 0.55, [[0.15, -0.55], [top, -0.55], [top, 0.2475], [0.15, 0.2475]], "yz", 0.215 - t, 0.215, holes=[ring(0.8, -0.15, 0.3, hole)]),
        ext("column-panel", "unit", PC, 0.5, [[0.15, 0.2475], [top, 0.2475], [top, 0.55], [0.15, 0.55]], "yz", 0.215 - t, 0.215),
        box("side-strip", "unit", PC, 0.55, [0.0875, top - 0.15, t], [0.16375, (top + 0.15) / 2, -0.55 + t / 2]),
        ext("rear-post", "unit", PC, 0.5, [[-0.55, -0.215], [-0.49, -0.215], [-0.49, -0.215 + t], [-0.55 + t, -0.215 + t], [-0.55 + t, -0.155], [-0.55, -0.155]], "zx", 0.15, top),
        box("top-panel", "unit", PC, 0.55, [0.43, t, 1.1], [0, top - t / 2, 0]),
        box("partition", "unit", GS, 0.45, [0.43 - 2 * t, top - 0.15 - t, t], [0, (top - t + 0.15) / 2, 0.2425 + t / 2]),
        box("column-back", "unit", PC, 0.5, [t, top - 0.15, 0.3075], [-0.215 + t / 2, (top + 0.15) / 2, 0.39625]),
        box("service-panel", "unit", PC, 0.5, [0.43, top - 0.15, t], [0, (top + 0.15) / 2, 0.55 - t / 2]),
    ]


def fan_set(scale):
    """Hub and three sickle blades on the bone `fan` (origin on the motor face, axis along u)."""
    thick = BLADE_THICK[scale]
    sections = [(BLADE[i][0], BLADE[i][1], thick, 2.2, BLADE[i][2]) for i in BLADE_PICK[scale]]
    return [
        rev("fan-hub", "fan", GS, 0.2, [[0, 0], [0, 0.075], [0.055, 0.075], [0.073, 0.064], [0.084, 0.042], [0.089, 0]], AXIS_X, [0, 0, 0], role="detail"),
        loft("fan-blades", "fan", AL, 0.45, sections, role="detail", capStart=True, capEnd=True, repeat=rep(3, rotation=2.0944, axis=AXIS_X, origin=[0, 0, 0])),
    ]


MOTOR = rev("fan-motor", "unit", GS, 0.2, [[0.02, 0], [0.02, 0.062], [0.03, 0.07], [0.105, 0.07], [0.105, 0]], AXIS_X, [0, 0.8, -0.15], role="detail")
COIL_ROUND = [[0.2425, -0.205]] + arc(-0.45, -0.135, 0.07, 270, 180, 8) + [[-0.52, 0.12], [-0.48, 0.12]] + arc(-0.45, -0.135, 0.03, 180, 270, 8) + [[0.2425, -0.165]]
COIL_SQUARE = [[0.2425, -0.215], [-0.52, -0.215], [-0.52, 0.12], [-0.48, 0.12], [-0.48, -0.175], [0.2425, -0.175]]


def compressor(scale):
    """The hermetic scroll compressor of the column: shell 170 mm, 0.40 m high, on its foot plate."""
    c = [-0.045, 0, 0.365] if scale == "500" else [-0.07, 0, 0.42]
    if scale == "50":
        shell = [[0.1925, 0], [0.1925, 0.079], [0.205, 0.085], [0.5, 0.085], [0.512, 0.0865], [0.524, 0.085], [0.553, 0.076], [0.578, 0.055], [0.594, 0.028], [0.6, 0]]
    elif scale == "100":
        shell = [[0.195, 0], [0.195, 0.085], [0.52, 0.085], [0.56, 0.072], [0.59, 0.04], [0.6, 0]]
    elif scale == "200":
        shell = [[0.15, 0], [0.15, 0.085], [0.52, 0.085], [0.58, 0.055], [0.6, 0]]
    else:
        shell = [[0.165, 0], [0.165, 0.085], [0.54, 0.085], [0.6, 0]]
    return rev("compressor-shell", "unit", PS, 0.15, shell, AXIS_Y, c)


def body50():
    t = 0.0075
    b = [ext("feet", "unit", RB, 0.2, [[0, 0.34], [0, 0.52], [0.075, 0.52], [0.09, 0.505], [0.09, 0.355], [0.075, 0.34]], "yz", -0.17, 0.17, repeat=rep(2, [0, 0, -0.86]))]
    b.append(box("base-tray", "unit", GS, 0.3, [0.43, 0.06, 1.1], [0, 0.12, 0], corner=0.005))
    b += casing(t)
    b += [
        box("service-grip", "unit", PC, 0.25, [0.06, 0.12, 0.0075], [0.13, 1.04, 0.55375], role="detail", corner=0.003),
        rev("water-connections", "unit", GS, 0.35, [[0, 0], [0, 0.021], [0.02, 0.021], [0.02, 0]], AXIS_X, [-0.235, 0.3, 0.34], role="detail", repeat=rep(2, [0, 0, 0.12])),
        box("hinge-brackets", "unit", PC, 0.35, [0.02, 0.02, 0.06], [-0.225, 1.41, 0.31], role="detail", repeat=rep(2, [0, 0, -0.62])),
        rev("hinge-knuckles", "unit", PC, 0.3, [[0, 0.009], [0.06, 0.009]], AXIS_Z, [-0.235, 1.41, 0.28], role="detail", repeat=rep(2, [0, 0, -0.62])),
        ext("coil", "unit", AL, 0.22, COIL_ROUND, "zx", 0.165, 1.4, role="detail"),
        box("coil-tube-sheet", "unit", GS, 0.4, [0.0075, 1.235, 0.056], [0.12375, 0.7825, -0.5], role="detail"),
        rev("coil-hairpins-outer", "unit", CU, 0.6, torus(-0.511, 0.0125, 0.00475), AXIS_Z, [0.1275, 0.2025, 0], role="detail", sweepRad=math.pi, repeat=rep(24, [0, 0.05, 0])),
        rev("coil-hairpins-inner", "unit", CU, 0.6, torus(-0.489, 0.0125, 0.00475), AXIS_Z, [0.1275, 0.2275, 0], role="detail", sweepRad=math.pi, repeat=rep(23, [0, 0.05, 0])),
        box("side-louvres", "unit", PC, 0.4, [0.275, 0.0075, 0.03], [-0.0175, 0.195, -0.536], role="detail", rot=quat(AXIS_X, -40), repeat=rep(25, [0, 0.05, 0])),
        cap("guard-wires-vertical", "unit", GS, 0.3, [-0.2105, 0.17, -0.48], [-0.2105, 1.395, -0.48], 0.002, role="detail", repeat=rep(15, [0, 0, 0.05]), texture=True),
        cap("guard-wires-horizontal", "unit", GS, 0.3, [-0.2145, 0.185, -0.488], [-0.2145, 0.185, 0.2405], 0.002, role="detail", repeat=rep(13, [0, 0.1, 0]), texture=True),
        rev("bell-mouth", "unit", GS, 0.3, [[0.075, 0.345], [0.09, 0.312], [0.11, 0.296], [0.135, 0.29], [0.18, 0.29], [0.2075, 0.298], [0.2075, 0.3055], [0.18, 0.2975], [0.135, 0.2975], [0.112, 0.3035], [0.095, 0.3185], [0.0825, 0.345], [0.075, 0.345]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("motor-bracket", "unit", GS, 0.3, [0.02, 1.2525, 0.03], [0.01, 0.79125, -0.26], role="detail", repeat=rep(2, [0, 0, 0.22])),
        box("motor-plate", "unit", GS, 0.3, [0.02, 0.14, 0.19], [0.01, 0.8, -0.15], role="detail"),
        MOTOR,
    ]
    b += fan_set("50")
    b.append(rev("guard-rim", "unit", GS, 0.25, torus(0.2175, 0.312, 0.005), AXIS_X, [0, 0.8, -0.15], slender=True))
    for k, (ax, rad) in enumerate([(0.22076, 0.27), (0.22344, 0.23), (0.22568, 0.19), (0.2275, 0.15), (0.22888, 0.11)]):
        b.append(rev("guard-ring-%d" % (k + 1), "unit", GS, 0.3, torus(ax, rad, 0.0025), AXIS_X, [0, 0.8, -0.15], role="detail", slender=True))
    b += [
        cap("guard-spokes", "unit", GS, 0.3, [0.23002, 0.86, -0.15], [0.2175, 1.108, -0.15], 0.0025, role="detail", slender=True, repeat=rep(8, rotation=0.7854, axis=AXIS_X, origin=[0, 0.8, -0.15])),
        rev("guard-cap", "unit", GS, 0.35, [[0.227, 0], [0.227, 0.06], [0.233, 0.058], [0.235, 0.05], [0.235, 0]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("lid-plate", "lid", PC, 0.35, [0.47, 0.0075, 1.115], [0.235, 0.03625, 0]),
        ext("lid-skirt", "lid", PC, 0.35, [[-0.5575, 0.02], [-0.5575, 0.47], [0.5575, 0.47], [0.5575, 0.02], [0.55, 0.02], [0.55, 0.4625], [-0.55, 0.4625], [-0.55, 0.02]], "zx", -0.005, 0.0325),
        rev("lid-knuckles", "lid", PC, 0.3, [[0, 0.009], [0.06, 0.009]], AXIS_Z, [0, 0, 0.34], role="detail", repeat=rep(2, [0, 0, -0.74])),
        box("lid-hinge-leaves", "lid", PC, 0.3, [0.008, 0.0325, 0.06], [0, 0.01625, 0.37], role="detail", repeat=rep(2, [0, 0, -0.74])),
        box("tray-heater", "unit", CU, 0.5, [0.0075, 0.0075, 0.7], [-0.185, 0.15375, -0.14], role="detail"),
        compressor("50"),
        box("compressor-foot-plate", "unit", PS, 0.15, [0.19, 0.0075, 0.19], [-0.07, 0.18875, 0.42]),
        rev("compressor-grommets-front", "unit", RB, 0.1, [[0.15, 0], [0.15, 0.016], [0.185, 0.016], [0.185, 0]], AXIS_Y, [0.005, 0, 0.345], role="detail", repeat=rep(2, [0, 0, 0.15])),
        rev("compressor-grommets-rear", "unit", RB, 0.1, [[0.15, 0], [0.15, 0.016], [0.185, 0.016], [0.185, 0]], AXIS_Y, [-0.145, 0, 0.345], role="detail", repeat=rep(2, [0, 0, 0.15])),
        box("compressor-terminal", "unit", PS, 0.1, [0.07, 0.08, 0.035], [-0.07, 0.4, 0.5175], role="detail", corner=0.005),
        cap("compressor-suction-stub", "unit", CU, 0.6, [0.01, 0.47, 0.42], [0.05, 0.47, 0.42], 0.011, role="detail"),
        cap("discharge-line-rise", "unit", CU, 0.6, [-0.07, 0.59, 0.42], [-0.07, 0.67, 0.42], 0.0065, role="detail"),
        cap("discharge-line-run", "unit", CU, 0.6, [-0.07, 0.67, 0.42], [0.04, 0.67, 0.42], 0.0065, role="detail"),
        cap("discharge-line-valve", "unit", CU, 0.6, [0.04, 0.67, 0.42], [0.04, 0.73, 0.42], 0.0065, role="detail"),
        rev("reversing-valve", "unit", CU, 0.55, [[-0.022, 0], [-0.018, 0.013], [-0.01, 0.016], [0.09, 0.016], [0.098, 0.013], [0.102, 0]], AXIS_X, [0, 0.74, 0.42], role="detail"),
        box("reversing-valve-pilot", "unit", PS, 0.15, [0.035, 0.03, 0.03], [0.04, 0.74, 0.385], role="detail", corner=0.004),
        cap("hot-gas-line-rise", "unit", CU, 0.6, [0.0, 0.75, 0.42], [0.0, 1.16, 0.42], 0.008, role="detail"),
        cap("hot-gas-line-port", "unit", CU, 0.6, [0.0, 1.16, 0.42], [-0.0825, 1.16, 0.435], 0.008, role="detail"),
        cap("suction-line-valve", "unit", CU, 0.6, [0.04, 0.75, 0.42], [0.04, 0.8, 0.42], 0.008, role="detail"),
        cap("suction-line-cross", "unit", CU, 0.6, [0.04, 0.8, 0.42], [0.04, 0.8, 0.5], 0.008, role="detail"),
        cap("suction-line-run", "unit", CU, 0.6, [0.04, 0.8, 0.5], [0.14, 0.8, 0.5], 0.008, role="detail"),
        cap("suction-line-drop", "unit", CU, 0.6, [0.14, 0.8, 0.5], [0.13, 0.47, 0.485], 0.008, role="detail"),
        cap("coil-gas-line-rise", "unit", CU, 0.6, [0.08, 0.75, 0.42], [0.08, 0.86, 0.42], 0.008, role="detail"),
        cap("coil-gas-line-cross", "unit", CU, 0.6, [0.08, 0.86, 0.42], [0.08, 0.86, 0.275], 0.008, role="detail"),
        cap("coil-gas-line-run", "unit", CU, 0.6, [0.08, 0.86, 0.275], [-0.19, 0.86, 0.275], 0.008, role="detail"),
        rev("accumulator", "unit", PS, 0.15, [[0.15, 0], [0.15, 0.04], [0.158, 0.045], [0.45, 0.045], [0.468, 0.035], [0.478, 0.012], [0.478, 0]], AXIS_Y, [0.11, 0, 0.47]),
        cap("accumulator-outlet-rise", "unit", CU, 0.6, [0.095, 0.47, 0.455], [0.095, 0.53, 0.455], 0.008, role="detail"),
        cap("accumulator-outlet-run", "unit", CU, 0.6, [0.095, 0.53, 0.455], [0.05, 0.53, 0.42], 0.008, role="detail"),
        cap("accumulator-outlet-drop", "unit", CU, 0.6, [0.05, 0.53, 0.42], [0.05, 0.47, 0.42], 0.008, role="detail"),
        box("condenser", "unit", SS, 0.5, [0.075, 0.526, 0.119], [-0.12, 0.95, 0.4], corner=0.003),
        rev("condenser-refrigerant-ports", "unit", CU, 0.6, [[0, 0], [0, 0.011], [0.02, 0.011], [0.02, 0]], AXIS_X, [-0.0825, 0.74, 0.365], role="detail", repeat=rep(2, [0, 0.42, 0.07])),
        rev("condenser-water-ports", "unit", SS, 0.5, [[-0.02, 0], [-0.02, 0.016], [0, 0.016], [0, 0]], AXIS_X, [-0.1575, 0.74, 0.365], role="detail", repeat=rep(2, [0, 0.42, 0.07])),
        box("condenser-brackets", "unit", GS, 0.4, [0.04, 0.0075, 0.0975], [-0.12, 0.76, 0.29625], role="detail", repeat=rep(2, [0, 0.38, 0])),
        cap("water-return-drop", "unit", CU, 0.6, [-0.185, 0.74, 0.365], [-0.185, 0.3, 0.34], 0.014, role="detail"),
        cap("water-return-stub", "unit", CU, 0.6, [-0.185, 0.3, 0.34], [-0.22, 0.3, 0.34], 0.014, role="detail"),
        cap("water-flow-drop", "unit", CU, 0.6, [-0.185, 1.16, 0.435], [-0.185, 0.3, 0.46], 0.014, role="detail"),
        cap("water-flow-stub", "unit", CU, 0.6, [-0.185, 0.3, 0.46], [-0.22, 0.3, 0.46], 0.014, role="detail"),
        cap("liquid-line-port", "unit", CU, 0.6, [-0.0625, 0.74, 0.365], [-0.04, 0.74, 0.365], 0.005, role="detail"),
        cap("liquid-line-drop", "unit", CU, 0.6, [-0.04, 0.74, 0.365], [-0.04, 0.625, 0.3], 0.005, role="detail"),
        cap("liquid-line-run", "unit", CU, 0.6, [-0.04, 0.625, 0.3], [0.105, 0.625, 0.3], 0.005, role="detail"),
        rev("expansion-valve-body", "unit", CU, 0.5, [[0.6, 0], [0.6, 0.012], [0.645, 0.012], [0.645, 0]], AXIS_Y, [0.12, 0, 0.3], role="detail"),
        rev("expansion-valve-coil", "unit", PS, 0.15, [[0.645, 0], [0.645, 0.021], [0.69, 0.021], [0.695, 0.012], [0.695, 0]], AXIS_Y, [0.12, 0, 0.3], role="detail"),
        cap("injection-line-drop", "unit", CU, 0.6, [0.12, 0.6, 0.3], [0.12, 0.5, 0.3], 0.005, role="detail"),
        cap("injection-line-run", "unit", CU, 0.6, [0.12, 0.5, 0.3], [-0.15, 0.5, 0.285], 0.005, role="detail"),
        rev("distributor", "unit", CU, 0.55, [[0.5, 0], [0.5, 0.006], [0.52, 0.006], [0.535, 0.014], [0.56, 0.014], [0.56, 0]], AXIS_Y, [-0.15, 0, 0.285], role="detail"),
        cap("feeder-tube-1", "unit", CU, 0.6, [-0.15, 0.56, 0.285], [-0.18, 0.3, 0.25], 0.00375, role="detail"),
        cap("feeder-tube-2", "unit", CU, 0.6, [-0.15, 0.56, 0.285], [-0.18, 0.6, 0.25], 0.00375, role="detail"),
        cap("feeder-tube-3", "unit", CU, 0.6, [-0.15, 0.56, 0.285], [-0.172, 0.9, 0.25], 0.00375, role="detail"),
        cap("feeder-tube-4", "unit", CU, 0.6, [-0.15, 0.56, 0.285], [-0.172, 1.2, 0.25], 0.00375, role="detail"),
        cap("coil-header", "unit", CU, 0.6, [-0.19, 0.2, 0.275], [-0.19, 1.38, 0.275], 0.011, role="detail"),
        cap("coil-header-stubs", "unit", CU, 0.6, [-0.19, 0.24, 0.275], [-0.19, 0.24, 0.245], 0.00475, role="detail", repeat=rep(12, [0, 0.1, 0])),
        box("inverter-box", "unit", GS, 0.35, [0.34, 0.19, 0.24], [0, 1.305, 0.4], corner=0.004),
        box("inverter-heat-sink", "unit", AL, 0.5, [0.16, 0.0075, 0.05], [0, 1.245, 0.2175], role="detail", repeat=rep(7, [0, 0.02, 0])),
    ]
    return b


def body100():
    t = 0.015
    b = [ext("feet", "unit", RB, 0.2, [[0, 0.34], [0, 0.52], [0.075, 0.52], [0.09, 0.505], [0.09, 0.355], [0.075, 0.34]], "yz", -0.17, 0.17, repeat=rep(2, [0, 0, -0.86]))]
    b.append(box("base-tray", "unit", GS, 0.3, [0.43, 0.06, 1.1], [0, 0.12, 0]))
    b += casing(t, hole=32)
    b += [
        rev("water-connections", "unit", GS, 0.35, [[0, 0], [0, 0.021], [0.02, 0.021], [0.02, 0]], AXIS_X, [-0.235, 0.3, 0.34], role="detail", repeat=rep(2, [0, 0, 0.12])),
        box("hinge-brackets", "unit", PC, 0.35, [0.02, 0.02, 0.06], [-0.225, 1.41, 0.31], role="detail", repeat=rep(2, [0, 0, -0.62])),
        ext("coil", "unit", AL, 0.22, COIL_ROUND, "zx", 0.165, 1.4, role="detail"),
        box("coil-tube-sheet", "unit", GS, 0.4, [0.015, 1.235, 0.056], [0.1275, 0.7825, -0.5], role="detail"),
        box("side-louvres", "unit", PC, 0.4, [0.275, 0.015, 0.03], [-0.0175, 0.22, -0.535], role="detail", rot=quat(AXIS_X, -40), repeat=rep(12, [0, 0.1, 0])),
        rev("bell-mouth", "unit", GS, 0.3, [[0.075, 0.345], [0.09, 0.312], [0.11, 0.296], [0.135, 0.29], [0.2, 0.29], [0.2, 0.305], [0.135, 0.305], [0.115, 0.311], [0.1, 0.324], [0.09, 0.345], [0.075, 0.345]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("motor-bracket", "unit", GS, 0.3, [0.02, 1.2525, 0.03], [0.01, 0.79125, -0.26], role="detail", repeat=rep(2, [0, 0, 0.22])),
        box("motor-plate", "unit", GS, 0.3, [0.02, 0.14, 0.19], [0.01, 0.8, -0.15], role="detail"),
        MOTOR,
    ]
    b += fan_set("100")
    b += [
        rev("guard-rim", "unit", GS, 0.25, torus(0.2175, 0.312, 0.005, 8), AXIS_X, [0, 0.8, -0.15], slender=True),
        rev("guard-ring-1", "unit", GS, 0.3, torus(0.22076, 0.27, 0.0025, 6), AXIS_X, [0, 0.8, -0.15], role="detail", slender=True),
        rev("guard-ring-2", "unit", GS, 0.3, torus(0.22568, 0.19, 0.0025, 6), AXIS_X, [0, 0.8, -0.15], role="detail", slender=True),
        cap("guard-spokes", "unit", GS, 0.3, [0.23002, 0.86, -0.15], [0.2175, 1.108, -0.15], 0.0025, role="detail", slender=True, repeat=rep(8, rotation=0.7854, axis=AXIS_X, origin=[0, 0.8, -0.15])),
        rev("guard-cap", "unit", GS, 0.35, [[0.22, 0], [0.22, 0.06], [0.235, 0.05], [0.235, 0]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("lid-plate", "lid", PC, 0.35, [0.47, 0.015, 1.115], [0.235, 0.0325, 0]),
        ext("lid-skirt", "lid", PC, 0.35, [[-0.5575, 0.02], [-0.5575, 0.47], [0.5575, 0.47], [0.5575, 0.02], [0.5425, 0.02], [0.5425, 0.455], [-0.5425, 0.455], [-0.5425, 0.02]], "zx", -0.005, 0.025),
        rev("lid-knuckles", "lid", PC, 0.3, [[0, 0.009], [0.06, 0.009]], AXIS_Z, [0, 0, 0.34], role="detail", repeat=rep(2, [0, 0, -0.74])),
        box("lid-hinge-leaves", "lid", PC, 0.3, [0.015, 0.025, 0.06], [0, 0.0125, 0.37], role="detail", repeat=rep(2, [0, 0, -0.74])),
        compressor("100"),
        box("compressor-foot-plate", "unit", PS, 0.15, [0.19, 0.045, 0.19], [-0.07, 0.1725, 0.42]),
        cap("compressor-suction-stub", "unit", CU, 0.6, [0.01, 0.47, 0.42], [0.05, 0.47, 0.42], 0.011, role="detail"),
        cap("discharge-line-rise", "unit", CU, 0.6, [-0.07, 0.59, 0.42], [-0.07, 0.67, 0.42], 0.0075, role="detail"),
        cap("discharge-line-run", "unit", CU, 0.6, [-0.07, 0.67, 0.42], [0.04, 0.67, 0.42], 0.0075, role="detail"),
        cap("discharge-line-valve", "unit", CU, 0.6, [0.04, 0.67, 0.42], [0.04, 0.73, 0.42], 0.0075, role="detail"),
        rev("reversing-valve", "unit", CU, 0.55, [[-0.022, 0], [-0.015, 0.016], [0.095, 0.016], [0.102, 0]], AXIS_X, [0, 0.74, 0.42], role="detail"),
        cap("hot-gas-line-rise", "unit", CU, 0.6, [0.0, 0.75, 0.42], [0.0, 1.16, 0.42], 0.008, role="detail"),
        cap("hot-gas-line-port", "unit", CU, 0.6, [0.0, 1.16, 0.42], [-0.0825, 1.16, 0.435], 0.008, role="detail"),
        cap("suction-line-valve", "unit", CU, 0.6, [0.04, 0.75, 0.42], [0.04, 0.8, 0.42], 0.008, role="detail"),
        cap("suction-line-cross", "unit", CU, 0.6, [0.04, 0.8, 0.42], [0.04, 0.8, 0.5], 0.008, role="detail"),
        cap("suction-line-run", "unit", CU, 0.6, [0.04, 0.8, 0.5], [0.14, 0.8, 0.5], 0.008, role="detail"),
        cap("suction-line-drop", "unit", CU, 0.6, [0.14, 0.8, 0.5], [0.13, 0.47, 0.485], 0.008, role="detail"),
        cap("coil-gas-line-rise", "unit", CU, 0.6, [0.08, 0.75, 0.42], [0.08, 0.86, 0.42], 0.008, role="detail"),
        cap("coil-gas-line-cross", "unit", CU, 0.6, [0.08, 0.86, 0.42], [0.08, 0.86, 0.275], 0.008, role="detail"),
        cap("coil-gas-line-run", "unit", CU, 0.6, [0.08, 0.86, 0.275], [-0.185, 0.86, 0.275], 0.008, role="detail"),
        rev("accumulator", "unit", PS, 0.15, [[0.15, 0], [0.15, 0.045], [0.45, 0.045], [0.47, 0.03], [0.478, 0]], AXIS_Y, [0.11, 0, 0.47]),
        cap("accumulator-outlet-rise", "unit", CU, 0.6, [0.095, 0.47, 0.455], [0.095, 0.53, 0.455], 0.008, role="detail"),
        cap("accumulator-outlet-run", "unit", CU, 0.6, [0.095, 0.53, 0.455], [0.05, 0.53, 0.42], 0.008, role="detail"),
        cap("accumulator-outlet-drop", "unit", CU, 0.6, [0.05, 0.53, 0.42], [0.05, 0.47, 0.42], 0.008, role="detail"),
        box("condenser", "unit", SS, 0.5, [0.075, 0.526, 0.119], [-0.12, 0.95, 0.4]),
        box("condenser-brackets", "unit", GS, 0.4, [0.04, 0.015, 0.09], [-0.12, 0.76, 0.3], role="detail", repeat=rep(2, [0, 0.38, 0])),
        cap("water-return-port", "unit", CU, 0.6, [-0.15, 0.74, 0.365], [-0.184, 0.74, 0.365], 0.014, role="detail"),
        cap("water-return-drop", "unit", CU, 0.6, [-0.184, 0.74, 0.365], [-0.184, 0.3, 0.34], 0.014, role="detail"),
        cap("water-return-stub", "unit", CU, 0.6, [-0.184, 0.3, 0.34], [-0.22, 0.3, 0.34], 0.014, role="detail"),
        cap("water-flow-port", "unit", CU, 0.6, [-0.15, 1.16, 0.435], [-0.184, 1.16, 0.435], 0.014, role="detail"),
        cap("water-flow-drop", "unit", CU, 0.6, [-0.184, 1.16, 0.435], [-0.184, 0.3, 0.46], 0.014, role="detail"),
        cap("water-flow-stub", "unit", CU, 0.6, [-0.184, 0.3, 0.46], [-0.22, 0.3, 0.46], 0.014, role="detail"),
        rev("expansion-valve", "unit", PS, 0.15, [[0.6, 0], [0.6, 0.012], [0.645, 0.021], [0.695, 0.021], [0.695, 0]], AXIS_Y, [0.12, 0, 0.3], role="detail"),
        cap("coil-header", "unit", CU, 0.6, [-0.185, 0.2, 0.275], [-0.185, 1.38, 0.275], 0.011, role="detail"),
        box("inverter-box", "unit", GS, 0.35, [0.34, 0.19, 0.24], [0, 1.305, 0.4]),
        box("inverter-heat-sink", "unit", AL, 0.5, [0.16, 0.015, 0.05], [0, 1.25, 0.2175], role="detail", repeat=rep(4, [0, 0.035, 0])),
    ]
    return b


def body200():
    t = 0.03
    b = [box("feet", "unit", RB, 0.2, [0.34, 0.09, 0.18], [0, 0.045, 0.43], repeat=rep(2, [0, 0, -0.86]))]
    b.append(box("base-tray", "unit", GS, 0.3, [0.43, 0.06, 1.1], [0, 0.12, 0]))
    b += [s for s in casing(t, top=1.405, hole=24) if s["id"] != "rear-post"]
    b += [
        rev("water-connections", "unit", GS, 0.35, [[0, 0], [0, 0.021], [0.03, 0.021], [0.03, 0]], AXIS_X, [-0.235, 0.3, 0.34], role="detail", repeat=rep(2, [0, 0, 0.12])),
        ext("coil", "unit", AL, 0.22, COIL_SQUARE, "zx", 0.15, 1.375, role="detail"),
        box("side-louvres", "unit", PC, 0.4, [0.275, 0.03, 0.03], [-0.0175, 0.28, -0.535], role="detail", repeat=rep(6, [0, 0.2, 0])),
        rev("bell-mouth", "unit", GS, 0.3, [[0.075, 0.36], [0.095, 0.312], [0.135, 0.29], [0.185, 0.29], [0.185, 0.32], [0.135, 0.32], [0.115, 0.335], [0.105, 0.36], [0.075, 0.36]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("motor-bracket", "unit", GS, 0.3, [0.03, 1.225, 0.03], [0.005, 0.7625, -0.26], role="detail", repeat=rep(2, [0, 0, 0.22])),
        box("motor-plate", "unit", GS, 0.3, [0.03, 0.14, 0.19], [0.005, 0.8, -0.15], role="detail"),
        MOTOR,
    ]
    b += fan_set("200")
    b += [
        rev("guard-rim", "unit", GS, 0.25, torus(0.22, 0.305, 0.015, 6), AXIS_X, [0, 0.8, -0.15]),
        rev("guard-cap", "unit", GS, 0.35, [[0.2, 0], [0.2, 0.06], [0.235, 0.05], [0.235, 0]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("lid", "lid", PC, 0.35, [0.47, 0.045, 1.115], [0.235, 0.0175, 0]),
        compressor("200"),
        rev("accumulator", "unit", PS, 0.15, [[0.15, 0], [0.15, 0.045], [0.46, 0.045], [0.478, 0]], AXIS_Y, [0.11, 0, 0.465]),
        box("condenser", "unit", SS, 0.5, [0.075, 0.526, 0.119], [-0.12, 0.95, 0.4]),
        box("condenser-brackets", "unit", GS, 0.4, [0.04, 0.03, 0.07], [-0.12, 0.76, 0.3075], role="detail", repeat=rep(2, [0, 0.38, 0])),
        box("inverter-box", "unit", GS, 0.35, [0.34, 0.17, 0.24], [0, 1.285, 0.395]),
        box("inverter-heat-sink", "unit", AL, 0.5, [0.16, 0.14, 0.05], [0, 1.285, 0.2175], role="detail"),
    ]
    return b


def body500():
    b = [box("feet", "unit", RB, 0.2, [0.34, 0.09, 0.18], [0, 0.045, 0.43], repeat=rep(2, [0, 0, -0.86]))]
    b += [
        box("base-tray", "unit", GS, 0.3, [0.43, 0.075, 1.1], [0, 0.1275, 0]),
        ext("front-panel", "unit", PC, 0.55, [[0.165, -0.55], [1.405, -0.55], [1.405, 0.55], [0.165, 0.55]], "yz", 0.14, 0.215, holes=[ring(0.8, -0.15, 0.3, 16)]),
        ext("column-shell", "unit", PC, 0.5, [[0.25, -0.215], [0.55, -0.215], [0.55, 0.14], [0.475, 0.14], [0.475, -0.14], [0.25, -0.14]], "zx", 0.165, 1.405),
        box("top-panel", "unit", PC, 0.55, [0.355, 0.075, 1.1], [-0.0375, 1.3675, 0]),
        box("coil", "unit", AL, 0.22, [0.075, 1.165, 0.725], [-0.1775, 0.7475, -0.1125], role="detail"),
        box("side-grille", "unit", PC, 0.4, [0.355, 1.165, 0.075], [-0.0375, 0.7475, -0.5125], role="detail"),
        box("lid", "lid", PC, 0.35, [0.47, 0.045, 1.115], [0.235, 0.0175, 0]),
        rev("bell-mouth", "unit", GS, 0.3, [[0.065, 0.375], [0.14, 0.3], [0.14, 0.375], [0.065, 0.375]], AXIS_X, [0, 0.8, -0.15], role="detail"),
        box("motor-post", "unit", GS, 0.3, [0.075, 1.165, 0.075], [-0.0175, 0.7475, -0.15], role="detail"),
        rev("fan-motor", "unit", GS, 0.2, [[0.02, 0], [0.02, 0.07], [0.105, 0.07], [0.105, 0]], AXIS_X, [0, 0.8, -0.15], role="detail"),
    ]
    b += fan_set("500")
    b += [
        compressor("500"),
        rev("accumulator", "unit", PS, 0.15, [[0.165, 0], [0.165, 0.045], [0.478, 0.045], [0.478, 0]], AXIS_Y, [0.09, 0, 0.4]),
        box("condenser", "unit", SS, 0.5, [0.075, 0.5, 0.119], [-0.09, 0.9, 0.39]),
        box("inverter-box", "unit", GS, 0.35, [0.26, 0.15, 0.2], [0, 1.24, 0.36]),
    ]
    return b


REGION = {
    "feet": "feet", "base-tray": "base-tray", "tray-heater": "base-tray", "front-panel": "casing", "side-strip": "casing", "rear-post": "casing", "top-panel": "casing", "partition": "casing",
    "water-connections": "casing", "hinge-brackets": "casing", "hinge-knuckles": "casing", "motor-bracket": "casing", "motor-plate": "casing", "motor-post": "casing",
    "column-panel": "compressor-casing", "column-back": "compressor-casing", "service-panel": "compressor-casing", "service-grip": "compressor-casing", "column-shell": "compressor-casing",
    "coil": "coil-guard", "coil-tube-sheet": "coil-guard", "coil-hairpins-outer": "coil-guard", "coil-hairpins-inner": "coil-guard", "side-louvres": "coil-guard", "side-grille": "coil-guard",
    "guard-wires-vertical": "coil-guard", "guard-wires-horizontal": "coil-guard",
    "bell-mouth": "fan-guard", "fan-motor": "fan-guard", "fan-hub": "fan-guard", "fan-blades": "fan-guard", "guard-rim": "fan-guard",
    "guard-ring-1": "fan-guard", "guard-ring-2": "fan-guard", "guard-ring-3": "fan-guard", "guard-ring-4": "fan-guard", "guard-ring-5": "fan-guard",
    "lid-plate": "top-lid", "lid-skirt": "top-lid", "lid-knuckles": "top-lid", "lid-hinge-leaves": "top-lid", "lid": "top-lid",
    "compressor-shell": "compressor", "compressor-foot-plate": "compressor", "compressor-grommets-front": "compressor", "compressor-grommets-rear": "compressor", "compressor-terminal": "compressor",
    "discharge-line-rise": "compressor", "discharge-line-run": "compressor", "discharge-line-valve": "compressor", "reversing-valve": "compressor", "reversing-valve-pilot": "compressor",
    "hot-gas-line-rise": "compressor", "hot-gas-line-port": "compressor",
    "condenser": "condenser", "condenser-refrigerant-ports": "condenser", "condenser-water-ports": "condenser", "condenser-brackets": "condenser",
    "water-return-port": "condenser", "water-return-drop": "condenser", "water-return-stub": "condenser", "water-flow-port": "condenser", "water-flow-drop": "condenser", "water-flow-stub": "condenser",
    "liquid-line-port": "condenser", "liquid-line-drop": "condenser", "liquid-line-run": "condenser",
    "compressor-suction-stub": "suction", "suction-line-valve": "suction", "suction-line-cross": "suction", "suction-line-run": "suction", "suction-line-drop": "suction",
    "coil-gas-line-rise": "suction", "coil-gas-line-cross": "suction", "coil-gas-line-run": "suction", "accumulator": "suction",
    "accumulator-outlet-rise": "suction", "accumulator-outlet-run": "suction", "accumulator-outlet-drop": "suction",
    "expansion-valve-body": "suction", "expansion-valve-coil": "suction", "expansion-valve": "suction", "injection-line-drop": "suction", "injection-line-run": "suction", "distributor": "suction",
    "feeder-tube-1": "suction", "feeder-tube-2": "suction", "feeder-tube-3": "suction", "feeder-tube-4": "suction", "coil-header": "suction", "coil-header-stubs": "suction",
    "inverter-box": "inverter", "inverter-heat-sink": "inverter",
}
VAPOUR = {"50": ([0.0075, 0.08, 0.55], -0.20875), "100": ([0.015, 0.08, 0.55], -0.205), "200": ([0.03, 0.08, 0.55], -0.2), "500": ([0.075, 0.08, 0.55], -0.1775)}


def thermal(scale, body):
    out = []
    for s in body:
        if s["id"] in ("guard-cap", "guard-spokes"):
            own = dict(s)
            own["id"] = "th-" + s["id"]
            out.append(th(None, "fan-guard", tags=["discharge-heating", "discharge-cooling"], own=own))
        elif REGION[s["id"]] in AREAS:
            own = dict(s)
            own["id"] = "th-" + s["id"]
            out.append(th(None, REGION[s["id"]], own=own))
        else:
            out.append(th(s["id"], REGION[s["id"]]))
    if scale == "500":
        out.append(th(None, "fan-guard", tags=["discharge-heating", "discharge-cooling"], own=rev("th-outlet", "unit", GS, 0.35, [[0.16, 0], [0.16, 0.075], [0.235, 0.075], [0.235, 0]], AXIS_X, [0, 0.8, -0.15], role="detail")))
    size, x = VAPOUR[scale]
    out.append(th(None, "coil-guard", tags=["defrost-vapour"], own=box("th-defrost-vapour", "unit", AL, 0.5, size, [x, 1.34, -0.125], role="detail")))
    return out


def structure(scale, body):
    ids = {s["id"] for s in body}
    f = FEATURE[scale]
    if scale == "50":
        t = 0.0075
        angle = lambda sw, su: [[sw * 0.55, su * 0.215], [sw * 0.51, su * 0.215], [sw * 0.51, su * (0.215 - t)], [sw * (0.55 - t), su * (0.215 - t)], [sw * (0.55 - t), su * 0.175], [sw * 0.55, su * 0.175]]
        out = [
            st("post-front-right", "unit", "member", own=ext("", "unit", GS, 0.45, angle(-1, 1), "zx", 0.15, 1.4175)),
            st("post-front-left", "unit", "member", own=ext("", "unit", GS, 0.45, angle(1, 1), "zx", 0.15, 1.4175)),
            st("post-rear-left", "unit", "member", own=ext("", "unit", GS, 0.45, angle(1, -1), "zx", 0.15, 1.4175)),
            st("post-rear-right", "unit", "member", of="rear-post"),
            st("top-rail-front", "unit", "member", own=box("", "unit", GS, 0.45, [0.04, 0.04, 1.1], [0.195, 1.3975, 0])),
            st("top-rail-rear", "unit", "member", own=box("", "unit", GS, 0.45, [0.04, 0.04, 1.1], [-0.195, 1.3975, 0])),
            st("top-rail-right", "unit", "member", own=box("", "unit", GS, 0.45, [0.43, 0.04, 0.04], [0, 1.3975, -0.53])),
            st("top-rail-left", "unit", "member", own=box("", "unit", GS, 0.45, [0.43, 0.04, 0.04], [0, 1.3975, 0.53])),
        ]
    else:
        inset = {"100": 0.2, "200": 0.2, "500": 0.1775}[scale]
        side = {"100": 0.535, "200": 0.535, "500": 0.5125}[scale]
        w = {"100": 0.03, "200": 0.03, "500": 0.075}[scale]
        y0, y1 = (0.165, 1.33) if scale == "500" else (0.15, 1.375)
        out = [st("post-%s-%s" % (a, b), "unit", "member", own=box("", "unit", GS, 0.45, [w, y1 - y0, w], [sx * inset, (y0 + y1) / 2, sz * side])) for a, sx in (("front", 1), ("rear", -1)) for b, sz in (("right", -1), ("left", 1))]
        out.append(st("top-frame", "unit", "member", own=ext("", "unit", GS, 0.45, [[-0.55, -0.215], [0.55, -0.215], [0.55, 0.215], [-0.55, 0.215]], "zx", y1, 1.405, holes=[[[-0.55 + w, -0.215 + w], [0.55 - w, -0.215 + w], [0.55 - w, 0.215 - w], [-0.55 + w, 0.215 - w]]])))
    out += [st("base-pan", "unit", "member", of="base-tray"), st("feet-blocks", "unit", "member", of="feet")]
    if "partition" in ids:
        out.append(st("partition-plate", "unit", "member", of="partition"))
    if "motor-bracket" in ids:
        out += [st("motor-bracket-bars", "unit", "member", of="motor-bracket"), st("motor-bracket-plate", "unit", "member", of="motor-plate")]
    else:
        out.append(st("motor-bracket-post", "unit", "member", of="motor-post"))
    out.append(st("mounted-fan-motor", "unit", "member", of="fan-motor"))
    if "compressor-foot-plate" in ids:
        out.append(st("compressor-base", "unit", "member", of="compressor-foot-plate"))
    if "compressor-grommets-front" in ids:
        out += [st("compressor-mounts-front", "unit", "joint", of="compressor-grommets-front"), st("compressor-mounts-rear", "unit", "joint", of="compressor-grommets-rear")]
    if "condenser-brackets" in ids:
        out.append(st("condenser-straps", "unit", "member", of="condenser-brackets"))
    out += [
        st("compressor", "unit", "mass", of="compressor-shell"),
        st("condenser-mass", "unit", "mass", of="condenser"),
        st("inverter-mass", "unit", "mass", of="inverter-box"),
        st("accumulator-mass", "unit", "mass", of="accumulator"),
    ]
    if "lid" in ids:
        out.append(st("lid-member", "lid", "member", of="lid"))
        out.append(st("hinge-lid", "lid", "joint", own=rev("", "lid", PC, 0.3, [[0, f / 2], [0.8, f / 2]], AXIS_Z, [f / 2, 0.0025, -0.4])))
    else:
        out += [st("lid-plate-member", "lid", "member", of="lid-plate"), st("lid-skirt-member", "lid", "member", of="lid-skirt"), st("hinge-lid", "lid", "joint", of="lid-knuckles")]
        if "hinge-knuckles" in ids:
            out.append(st("hinge-casing", "unit", "joint", of="hinge-knuckles"))
    out += [
        st("fan-rotor", "fan", "member", of="fan-hub"),
        st("fan-impeller", "fan", "member", of="fan-blades"),
        st("fan-bearing", "fan", "joint", own=rev("", "fan", GS, 0.3, [[-max(f, 0.02), 0], [-max(f, 0.02), max(f / 2, 0.012)], [0, max(f / 2, 0.012)], [0, 0]], AXIS_X, [0, 0, 0])),
        st("support-contact-l", "unit", "support", own=box("", "unit", RB, 0.2, [0.34, f, 0.18], [0, f / 2, 0.43]), anchor="contact-l"),
        st("support-contact-r", "unit", "support", own=box("", "unit", RB, 0.2, [0.34, f, 0.18], [0, f / 2, -0.43]), anchor="contact-r"),
        st("mass-unit", "unit", "mass", own=ell("", "unit", PC, 0.2, [max(0.03, f / 2)] * 3, [0, 0.55, 0])),
        st("mass-lid", "lid", "mass", own=ell("", "lid", PC, 0.2, [max(0.03, f / 2), 0.0225, max(0.03, f / 2)], [0.235, 0.0175, 0])),
        st("mass-fan", "fan", "mass", own=ell("", "fan", AL, 0.2, [max(0.02, f / 2)] * 3, [0.04, 0, 0])),
    ]
    return out


DECISIONS = {
    "50": [
        text("Built as the real monobloc: galvanised base pan on two chamfered rubber feet, casing of 7.5 mm sheets (front panel with the 0.60 m fan opening, column panel, right strip, folded rear post, top, service panel with grip, column back with two 42 mm water connections, partition) under a hinged cover on two knuckle pairs.", "Gebaut wie der echte Monoblock: verzinkte Bodenwanne auf zwei gefasten Gummifüßen, Gehäuse aus 7,5-mm-Blechen (Frontblech mit der 0,60-m-Lüfteröffnung, Säulenblech, rechter Streifen, gekanteter Eckpfosten, Deckblech, Wartungsblech mit Griff, Säulenrückwand mit zwei 42-mm-Wasseranschlüssen, Trennwand) unter einer an zwei Scharnierpaaren angeschlagenen Haube."),
        text("The evaporator is the L-shaped fin pack, 40 mm deep and 1.24 m high, with its tube sheet and 47 copper hairpin bends of 9.5 mm tube at the front right end, the 22 mm suction header with twelve stubs and the distributor with four feeder tubes at the partition end; 25 louvres at 40 degrees guard the side, a welded wire grid the back.", "Der Verdampfer ist das L-förmige Lamellenpaket, 40 mm tief und 1,24 m hoch, mit Rohrboden und 47 Kupfer-Umlenkbögen aus 9,5-mm-Rohr am vorderen rechten Ende, dem 22-mm-Saugsammler mit zwölf Stutzen und dem Verteiler mit vier Einspritzrohren an der Trennwand; 25 Lamellen unter 40 Grad schützen die Seite, ein Schweißgitter die Rückseite."),
        text("The fan turns on its own bone: spinner hub and three sickle blades of 0.55 m diameter on the external-rotor motor, which is bolted to a plate between two bracket bars, inside the flared bell mouth and behind the domed grille of rim, five rings, eight spokes and cap.", "Der Lüfter dreht auf eigenem Knochen: Nabenhaube und drei Sichelflügel von 0,55 m Durchmesser am Außenläufermotor, der auf einer Platte zwischen zwei Halterstäben sitzt, in der ausgestellten Einlaufdüse hinter dem gewölbten Gitter aus Rand, fünf Ringen, acht Speichen und Kappe."),
        text("The column holds the refrigerant circuit at true size: scroll compressor of 170 mm shell and 0.40 m height on foot plate and four rubber grommets with terminal box, 13 mm discharge line to the four-way reversing valve with its pilot, 16 mm hot-gas line to the brazed plate condenser (75 x 119 x 526 mm) with 28 mm water pipes down to the connections, 10 mm liquid line to the electronic expansion valve, injection line to the distributor, and the suction path from the coil header through the valve and the 90 mm accumulator back to the compressor; the inverter box with its heat sink fins in the air stream sits on top.", "Die Säule enthält den Kältekreis in wahrer Größe: Scrollverdichter mit 170 mm Mantel und 0,40 m Höhe auf Fußplatte und vier Gummipuffern mit Klemmkasten, 13-mm-Druckleitung zum Vierwege-Umschaltventil mit Pilotventil, 16-mm-Heißgasleitung zum gelöteten Plattenverflüssiger (75 x 119 x 526 mm) mit 28-mm-Wasserrohren hinunter zu den Anschlüssen, 10-mm-Flüssigkeitsleitung zum elektronischen Expansionsventil, Einspritzleitung zum Verteiler und der Saugweg vom Sammler über das Ventil und den 90-mm-Flüssigkeitsabscheider zurück zum Verdichter; obenauf sitzt der Inverterkasten mit seinen Kühlrippen im Luftstrom."),
        text("Thermal: every body solid carries its region; the hidden regions compressor (hot), condenser (warm in heating), suction (cold in heating) and inverter make the cycle readable in a cut. Structure: four corner angles, top rails, base pan and feet carry the unit; the compressor stands on its grommets, the fan hangs on the motor bearing, the lid on its hinges.", "Thermisch: Jeder Körper trägt seine Region; die verdeckten Regionen Verdichter (heiß), Verflüssiger (im Heizbetrieb warm), Saugseite (im Heizbetrieb kalt) und Inverter machen den Kreisprozess im Schnitt lesbar. Tragwerk: vier Eckwinkel, obere Riegel, Bodenwanne und Füße tragen das Gerät; der Verdichter steht auf seinen Puffern, der Lüfter hängt am Motorlager, der Deckel an seinen Scharnieren."),
    ],
    "100": [
        text("Modelled for 15 mm: sheets of 15 mm, twelve louvres of 100 mm pitch, the grille as rim, two rings, eight spokes and cap, the bell mouth with a 15 mm wall; the fan with seven-section blades turns on its own bone.", "Für 15 mm modelliert: Bleche von 15 mm, zwölf Lamellen mit 100 mm Teilung, das Gitter als Rand, zwei Ringe, acht Speichen und Kappe, die Einlaufdüse mit 15 mm Wand; der Lüfter mit Flügeln aus sieben Schnitten dreht auf eigenem Knochen."),
        text("The refrigerant circuit keeps every part of 15 mm or more: compressor on its foot block, accumulator, reversing valve, plate condenser with brackets and water pipes, expansion valve, suction header, the 15 to 22 mm lines, inverter box with four heat sink fins; hairpins, feeder tubes, stubs and grommets are left to 1:50.", "Der Kältekreis behält jedes Teil ab 15 mm: Verdichter auf Fußblock, Flüssigkeitsabscheider, Umschaltventil, Plattenverflüssiger mit Haltern und Wasserrohren, Expansionsventil, Saugsammler, die Leitungen von 15 bis 22 mm, Inverterkasten mit vier Kühlrippen; Umlenkbögen, Einspritzrohre, Stutzen und Puffer bleiben dem Maßstab 1:50."),
        text("Thermal regions and the structural frame follow the 1:50 model with box posts and one top frame.", "Thermische Regionen und Tragrahmen folgen dem 1:50-Modell mit Kastenpfosten und einem oberen Rahmen."),
    ],
    "200": [
        text("Modelled for 30 mm: sheets of 30 mm, box feet, six louvre bars, the coil with square corner, the grille as a 30 mm rim with cap, the cover as one plate; the fan keeps hub and three blades of five sections on its own bone.", "Für 30 mm modelliert: Bleche von 30 mm, Quaderfüße, sechs Lamellenstäbe, der Wärmetauscher mit eckiger Ecke, das Gitter als 30-mm-Rand mit Kappe, die Haube als eine Platte; der Lüfter behält Nabe und drei Flügel aus fünf Schnitten auf eigenem Knochen."),
        text("Inside the column remain the parts of 30 mm or more: compressor, accumulator, plate condenser on its brackets, inverter box with the heat sink as one block; lines and valves are below the feature.", "In der Säule bleiben die Teile ab 30 mm: Verdichter, Flüssigkeitsabscheider, Plattenverflüssiger auf seinen Haltern, Inverterkasten mit dem Kühlkörper als Block; Leitungen und Ventile liegen unter dem Merkmal."),
        text("Thermal regions as at 1:50; structure with box posts, top frame, base pan, feet, motor bracket and the masses of compressor, condenser, accumulator and inverter.", "Thermische Regionen wie bei 1:50; Tragwerk mit Kastenpfosten, oberem Rahmen, Bodenwanne, Füßen, Motorhalter und den Massen von Verdichter, Verflüssiger, Abscheider und Inverter."),
    ],
    "500": [
        text("Modelled for 75 mm: walls of 75 mm (front with the fan opening, column shell, top), the coil as the back slab and the louvred side as one grille slab, box feet, the cover as one plate, the bell mouth as a 75 mm bead and one motor post.", "Für 75 mm modelliert: Wände von 75 mm (Front mit Lüfteröffnung, Säulenschale, Decke), der Wärmetauscher als Rückwandscheibe und die Lamellenseite als eine Gitterscheibe, Quaderfüße, die Haube als eine Platte, die Einlaufdüse als 75-mm-Wulst und ein Motorpfosten."),
        text("The fan keeps motor, hub and three blades of four sections on its own bone; the column keeps compressor, accumulator, plate condenser and inverter box.", "Der Lüfter behält Motor, Nabe und drei Flügel aus vier Schnitten auf eigenem Knochen; die Säule behält Verdichter, Abscheider, Plattenverflüssiger und Inverterkasten."),
        text("Thermal regions as at 1:50; structure with four posts, top frame, base pan, feet and the masses.", "Thermische Regionen wie bei 1:50; Tragwerk mit vier Pfosten, oberem Rahmen, Bodenwanne, Füßen und den Massen."),
    ],
}

MATERIALS = [
    {"id": CU, "title": text("Copper tube", "Kupferrohr"), "emissivity": 0.6, "solarAbsorptivity": 0.6, "densityKgM3": 8930, "conductivityWmK": 390, "specificHeatJKgK": 385, "source": "literature", "reference": "incropera-2011"},
    {"id": PS, "title": text("Painted steel (black)", "Lackierter Stahl (schwarz)"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "source": "literature", "reference": "incropera-2011"},
    {"id": SS, "title": text("Stainless steel", "Edelstahl"), "emissivity": 0.3, "solarAbsorptivity": 0.4, "densityKgM3": 7900, "conductivityWmK": 15, "specificHeatJKgK": 500, "source": "literature", "reference": "incropera-2011"},
]


def region(id, en, de, kind, material, area, length, geometry, layer_en, layer_de, thick, cap_j, key, heat, hidden_en, hidden_de):
    return {
        "id": id, "title": text(en, de), "kind": kind, "model": "operating", "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0,
        "layers": [{"id": "wall", "title": text(layer_en, layer_de), "kind": "metal", "material": material, "thicknessM": thick, "resistanceM2KW": 0, "heatCapacityJM2K": cap_j, "source": "literature", "reference": "incropera-2011"}],
        "heatCapacityJM2K": cap_j, "operating": {"key": key, "heatW": heat}, "source": "authored-estimate", "reference": "en-14511", "hidden": text(hidden_en, hidden_de),
    }


AREAS = {"compressor": 0.41, "condenser": 0.39, "suction": 0.33, "inverter": 0.45}
REGIONS = [
    region("compressor", "Scroll compressor, discharge line and reversing valve", "Scrollverdichter, Druckleitung und Umschaltventil", "engine", PS, AREAS["compressor"], 0.4, "cylinder", "Steel shell 3 mm", "Stahlmantel 3 mm", 0.003, 10800, "compressor", 290,
           "Inside the closed compressor column: the hermetic scroll compressor on its grommets, the discharge and hot-gas lines and the four-way valve. They carry the hot compressed gas (about 60 to 90 C at the shell and the line); no drawing shows them, the 3D thermal model does when cut.", "Im geschlossenen Verdichterraum: der hermetische Scrollverdichter auf seinen Puffern, Druck- und Heißgasleitung und das Vierwegeventil. Sie führen das heiße verdichtete Gas (etwa 60 bis 90 C an Mantel und Leitung); keine Zeichnung zeigt sie, das thermische 3D-Modell im Schnitt schon."),
    region("condenser", "Plate heat exchanger with water pipes and liquid line", "Plattenwärmetauscher mit Wasserrohren und Flüssigkeitsleitung", "heater", SS, AREAS["condenser"], 0.5, "vertical-plate", "Stainless plate pack wall 1 mm", "Edelstahl-Plattenpaket, Wand 1 mm", 0.001, 3950, "refrigerant-duty", 165,
           "Inside the compressor column: the brazed plate heat exchanger, condenser in heating (about 35 to 45 C with the heating water) and evaporator in cooling and defrost, with its water pipes and the liquid line; no drawing shows it.", "Im Verdichterraum: der gelötete Plattenwärmetauscher, im Heizbetrieb Verflüssiger (etwa 35 bis 45 C mit dem Heizwasser), im Kühl- und Abtaubetrieb Verdampfer, mit seinen Wasserrohren und der Flüssigkeitsleitung; keine Zeichnung zeigt ihn."),
    region("suction", "Suction side: accumulator, header, expansion valve and lines", "Saugseite: Abscheider, Sammler, Expansionsventil und Leitungen", "surface", CU, AREAS["suction"], 0.3, "cylinder", "Copper tube wall 1 mm", "Kupferrohrwand 1 mm", 0.001, 3440, "refrigerant-duty", -38,
           "Inside the compressor column: the low-pressure side from the expansion valve over distributor and coil header to the accumulator and the suction stub, a few kelvin below the outdoor air in heating and warm in cooling and defrost; no drawing shows it.", "Im Verdichterraum: die Niederdruckseite vom Expansionsventil über Verteiler und Sammler zum Abscheider und Saugstutzen, im Heizbetrieb einige Kelvin unter der Außenluft, im Kühl- und Abtaubetrieb warm; keine Zeichnung zeigt sie."),
    region("inverter", "Inverter box with heat sink", "Inverterkasten mit Kühlkörper", "engine", GS, AREAS["inverter"], 0.2, "vertical-plate", "Steel plate 1 mm", "Stahlblech 1 mm", 0.001, 3600, "compressor", 110,
           "On top of the compressor column under the cover: the frequency inverter of the compressor, which loses about 3 to 4 percent of the drive power as heat through its fins in the fan chamber; no drawing shows it.", "Oben im Verdichterraum unter der Haube: der Frequenzumrichter des Verdichters, der etwa 3 bis 4 Prozent der Antriebsleistung über seine Rippen in der Lüfterkammer als Wärme verliert; keine Zeichnung zeigt ihn."),
]
FAN_BONE = {
    "id": "fan", "parent": "unit", "title": text("Axial fan impeller on the motor shaft", "Axiallüfterrad auf der Motorwelle"), "x": 0.105, "y": 0.8, "z": -0.15, "angleDeg": 0, "lengthM": 0.089,
    "joint": {"type": "revolute", "continuousTwist": True}, "mass": {"kg": 3, "comX": 0.04, "comY": 0, "distribution": "point", "source": "authored-estimate"},
}


def root(asset):
    """Rig, clips, 2D bindings, materials and thermal regions of the catalog document, each stated absolutely so that a second run changes nothing."""
    bones = asset["rig"]["bones"]
    find(bones, "unit")["mass"]["kg"] = 140
    find(bones, "unit")["title"] = text("Casing with coil, fan motor, refrigerant circuit, base pan and feet", "Gehäuse mit Wärmetauscher, Lüftermotor, Kältekreis, Bodenwanne und Füßen")
    asset["rig"]["bones"] = [b for b in bones if b["id"] != "fan"] + [FAN_BONE]
    for rep_ in asset["representations"].values():
        for shape in rep_.get("geometry", {}).get("front", []):
            if shape["id"].startswith("fan-blade-") or shape["id"] == "fan-hub":
                shape["bone"] = "fan"
    asset["materials"] = [m for m in asset["materials"] if m["id"] not in (CU, PS, SS)] + MATERIALS
    regions = [x for x in asset["thermal"]["regions"] if x["id"] not in AREAS]
    asset["thermal"]["regions"] = regions + REGIONS
    for pose in asset["poses"]:
        for id in AREAS:
            pose["thermal"]["regions"][id] = {"exposedFraction": 1, "sunlitFraction": 0}
    modulation = find(asset["clips"], "heating-modulation")
    modulation["keys"] = [{"t": 5 * k, "pose": "heating", "ease": "linear", "bones": {"fan": {"twistDeg": 720 * k}}} for k in range(5)]
    sequence = find(asset["clips"], "defrost-sequence")
    turn = {0: 0, 1: 0, 3: 180, 10: 1260, 11: 1350, 17: 1350, 18.5: 1440, 25: 2520, 26: 2610, 28: 2610, 30: 2610}
    for key in sequence["keys"]:
        key["bones"] = {"fan": {"twistDeg": turn[key["t"]]}}


def main():
    document, before = load_catalog(SLUG)
    asset = document["asset"]
    root(asset)
    volumes = {}
    for scale, make in (("50", body50), ("100", body100), ("200", body200), ("500", body500)):
        body = make()
        volumes[scale] = {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal(scale, body), "structure": structure(scale, body), "decisions": DECISIONS[scale]}
    save(SLUG, document, volumes, before, "--publish" in sys.argv)


main()
