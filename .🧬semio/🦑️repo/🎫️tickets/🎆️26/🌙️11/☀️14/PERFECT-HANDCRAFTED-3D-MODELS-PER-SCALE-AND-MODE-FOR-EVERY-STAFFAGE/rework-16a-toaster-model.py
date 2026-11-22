"""🍞 Two-slice toaster, rebuilt by hand: pressed chrome wrap on a moulded plinth, dark cap with two slots, lowering lever in its slot, browning dial and two buttons on the narrow end face, crumb-tray pull and mains cord at the other end; rig, body, thermal, structure and the 2D drawings of every scale."""
import math
import re

from rework_16a_lib import FACES, box, revolve, extrude, capsule, vloft, xloft, ellipsoid, superellipse, rrect, of, shape, poly, rect, ellipse, mirror_outline, pts, run

SLUG = "toaster"
HALF_L, HALF_D, TOP, SEAM = 0.134, 0.085, 0.188, 0.176
END_R, END_H, SIDE_R, SIDE_H = 0.034, 0.038, 0.022, 0.022
E = 6
SLOT_Z0, SLOT_Z1, SLOT_X0, SLOT_X1 = 0.017, 0.049, -0.080, 0.060
SLICE_X, SLICE_Y, SLICE_Z0, SLICE_Z1 = -0.010, 0.085, 0.0265, 0.0395
END = 0.134
LEVER_Y, DIAL_Y, BUTTON_Z = 0.140, 0.056, 0.038
FOOT_X, FOOT_Z = 0.100, 0.056
CORD_Z, CORD_R, CORD_X = 0.050, 0.00375, -0.13625
BREAD = [[0.046, 0.0], [0.05, 0.0015], [0.052, 0.005], [0.052, 0.081], [0.0555, 0.084], [0.058, 0.089], [0.0585, 0.095], [0.057, 0.101], [0.053, 0.106], [0.046, 0.11], [0.034, 0.113], [0.018, 0.1145], [0.0, 0.115], [-0.018, 0.1145], [-0.034, 0.113], [-0.046, 0.11], [-0.053, 0.106], [-0.057, 0.101], [-0.0585, 0.095], [-0.058, 0.089], [-0.0555, 0.084], [-0.052, 0.081], [-0.052, 0.005], [-0.05, 0.0015], [-0.046, 0.0]]
CRUMB = [[0.043, 0.0035], [0.0472, 0.0048], [0.0485, 0.008], [0.0485, 0.0825], [0.052, 0.0855], [0.0545, 0.09], [0.055, 0.095], [0.0535, 0.1], [0.05, 0.104], [0.044, 0.107], [0.033, 0.1097], [0.0175, 0.1111], [0.0, 0.1115], [-0.0175, 0.1111], [-0.033, 0.1097], [-0.044, 0.107], [-0.05, 0.104], [-0.0535, 0.1], [-0.055, 0.095], [-0.0545, 0.09], [-0.052, 0.0855], [-0.0485, 0.0825], [-0.0485, 0.008], [-0.0472, 0.0048], [-0.043, 0.0035]]
BREAD_100 = [[0.048, 0.0], [0.052, 0.004], [0.052, 0.081], [0.057, 0.086], [0.0585, 0.095], [0.055, 0.104], [0.044, 0.111], [0.022, 0.1145], [0.0, 0.115], [-0.022, 0.1145], [-0.044, 0.111], [-0.055, 0.104], [-0.0585, 0.095], [-0.057, 0.086], [-0.052, 0.081], [-0.052, 0.004], [-0.048, 0.0]]
BREAD_200 = [[0.052, 0.015], [0.052, 0.081], [0.058, 0.089], [0.057, 0.101], [0.048, 0.109], [0.026, 0.114], [0.0, 0.115], [-0.026, 0.114], [-0.048, 0.109], [-0.057, 0.101], [-0.058, 0.089], [-0.052, 0.081], [-0.052, 0.015]]
BREAD_500 = [[0.052, 0.045], [0.057, 0.089], [0.05, 0.107], [0.028, 0.114], [0.0, 0.115], [-0.028, 0.114], [-0.05, 0.107], [-0.057, 0.089], [-0.052, 0.045]]
DIAL_HALF = [[0.0, 0.0], [0.0, 0.016], [0.003, 0.016], [0.0042, 0.0135], [0.011, 0.0125], [0.012, 0.011], [0.012, 0.0]]
TURNED = {"kind": "revolved", "axisBone": "dial", "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}
FOOT = [[0.0, 0.0], [0.0, 0.0105], [0.002, 0.0122], [0.012, 0.0135], [0.012, 0.0]]


def wrap(y):
    """🫓 Half depth and half length of the wrap at a height: straight walls, a 34 x 38 mm end shoulder and a 22 mm long shoulder."""
    dx = 0.0 if y <= TOP - END_H else END_R * (1 - math.sqrt(max(0.0, 1 - ((y - TOP + END_H) / END_H) ** 2)))
    dz = 0.0 if y <= TOP - SIDE_H else SIDE_R * (1 - math.sqrt(max(0.0, 1 - ((y - TOP + SIDE_H) / SIDE_H) ** 2)))
    return (y, HALF_D - dz, HALF_L - dx)


HEM = [(0.028, 0.080, 0.129), (0.0295, 0.0838, 0.1328), (0.033, HALF_D, HALF_L)]
PLINTH = [(0.010, 0.074, 0.123), (0.014, 0.079, 0.128), (0.030, 0.079, 0.128)]
WALL = {
    "50": HEM + [wrap(y) for y in (0.150, 0.156, 0.162, 0.166, 0.170, 0.173, SEAM)],
    "100": [(0.030, 0.080, 0.129), (0.034, HALF_D, HALF_L)] + [wrap(y) for y in (0.150, 0.160, 0.168, SEAM)],
}
RIM = {
    "50": [wrap(y) for y in (SEAM, 0.179, 0.182, 0.184, 0.186, 0.1872, TOP)],
    "100": [wrap(y) for y in (SEAM, 0.182, 0.186, TOP)],
}
BLOCK = {
    "200": [(0.0, HALF_D, HALF_L)] + [wrap(y) for y in (0.150, 0.166, 0.178, 0.185, TOP)],
    "500": [(0.0, HALF_D, HALF_L)] + [wrap(y) for y in (0.150, 0.172, TOP)],
}
DECK = (HALF_D - SIDE_R, HALF_L - END_R)


def slot(sign, grow=0.0, radius=0.005):
    """🕳️ One bread slot in the (w, u) plane of the deck."""
    z0, z1 = (SLOT_Z0 - grow, SLOT_Z1 + grow) if sign > 0 else (-SLOT_Z1 - grow, -SLOT_Z0 + grow)
    return rrect(z0, SLOT_X0 - grow, z1, SLOT_X1 + grow, radius + grow)


def feet():
    """🦶 Four tapered rubber feet as two pairs."""
    return [revolve(f"foot-{name}", "base", "rubber-foot", FOOT, [0, 1, 0], [0.0, 0.0, 0.0], tone=0.15, transform={"translation": [x, 0.0, -FOOT_Z]}, repeat={"count": 2, "translation": [0, 0, 2 * FOOT_Z]}) for name, x in (("left", -FOOT_X), ("right", FOOT_X))]


def cord():
    """🔌 Strain-relief sleeve and the mains cord dropping to the worktop and running along the back end."""
    return [
        revolve("cord-sleeve", "base", "pp-black", [[-0.136, 0.0], [-0.136, 0.0045], [-0.131, 0.006], [-0.124, 0.006], [-0.124, 0.0]], [1, 0, 0], [0.0, 0.020, CORD_Z], role="detail", tone=0.2),
        capsule("cord-drop", "base", "pp-black", [-0.133, 0.020, CORD_Z], [CORD_X, CORD_R, CORD_Z], CORD_R, role="detail", tone=0.2),
        capsule("cord-run", "base", "pp-black", [CORD_X, CORD_R, CORD_Z], [CORD_X, CORD_R, -0.076], CORD_R, role="detail", tone=0.2),
    ]


def lifters(**tags):
    """🛗 The two lifter bars of the carriage under the slices; their ends run in the slots of the chamber end walls."""
    return [box(f"lifter-{name}", "slices", "chrome-steel", [0.160, 0.0075, 0.010], [0.008, -0.00375, z], tone=0.5, **tags) for name, z in (("front", -0.033), ("rear", 0.033))]


def body_50():
    slices = []
    for name, z0, z1 in (("front", -SLICE_Z1, -SLICE_Z0), ("rear", SLICE_Z0, SLICE_Z1)):
        slices.append(extrude(f"slice-{name}", "slices", "bread-crust", BREAD, "xy", z0, z1, tone=0.55, holes=[CRUMB]))
        slices.append(extrude(f"crumb-{name}", "slices", "bread-crust", CRUMB, "xy", z0, z1, tone=0.82))
    return [
        vloft("base-tray", "base", "pp-black", PLINTH, E, tone=0.2),
        *feet(),
        box("crumb-tray", "base", "chrome-steel", [0.008, 0.012, 0.090], [-0.130, 0.021, 0.0], role="detail", tone=0.7, radius=0.002),
        *cord(),
        vloft("casing", "base", "chrome-steel", WALL["50"], E, tone=0.82, caps=False),
        vloft("top-rim", "base", "mica-steel", RIM["50"], E, tone=0.5, caps=False),
        extrude("top-plate", "base", "mica-steel", superellipse(DECK[0], DECK[1], E, 48), "zx", 0.180, TOP, tone=0.4, holes=[slot(-1), slot(1)]),
        extrude("chamber", "base", "mica-steel", [[-0.060, -0.088], [0.060, -0.088], [0.060, 0.068], [-0.060, 0.068]], "zx", 0.040, 0.180, tone=0.22, holes=[slot(-1, -0.001, 0.004), slot(1, -0.001, 0.004)]),
        box("crumb-pan", "base", "chrome-steel", [0.150, 0.0075, 0.110], [-0.010, 0.03625, 0.0], role="detail", tone=0.7),
        extrude("lever-slot", "base", "pp-black", rrect(0.086, -0.0055, 0.152, 0.0055, 0.005), "yz", END - 0.0015, END + 0.0012, role="texture", tone=0.12, texture=True),
        xloft("lever", "lever", "pp-black", [(-0.006, 0.004, 0.004), (0.003, 0.004, 0.004), (0.0036, 0.0078, 0.019), (0.012, 0.0085, 0.020), (0.015, 0.0075, 0.019), (0.016, 0.006, 0.017)], 4, tone=0.2),
        revolve("dial", "dial", "pp-black", [[-0.002, 0.0], [-0.002, 0.016], [0.003, 0.016], [0.0042, 0.0135], [0.011, 0.0125], [0.012, 0.011], [0.012, 0.0]], [1, 0, 0], [0.0, 0.0, 0.0], tone=0.25),
        box("dial-pointer", "dial", "chrome-steel", [0.0012, 0.009, 0.002], [0.0123, 0.0062, 0.0], role="texture", tone=0.9, texture=True),
        revolve("cancel-button", "base", "pp-black", [[-0.004, 0.0], [-0.004, 0.0085], [0.003, 0.0085], [0.0042, 0.007], [0.0042, 0.0]], [1, 0, 0], [END, DIAL_Y, -BUTTON_Z], tone=0.2),
        revolve("defrost-button", "base", "pp-black", [[-0.004, 0.0], [-0.004, 0.0085], [0.003, 0.0085], [0.0042, 0.007], [0.0042, 0.0]], [1, 0, 0], [END, DIAL_Y, BUTTON_Z], tone=0.2),
        *lifters(),
        *slices,
    ]


def cards(spans):
    """🔥 Element cards (mica wound with resistance ribbon) beside the slots."""
    card = [[SLOT_X0 + 0.004, 0.05], [SLOT_X1 - 0.004, 0.05], [SLOT_X1 - 0.004, 0.165], [SLOT_X0 + 0.004, 0.165]]
    return [extrude(f"th-element-{name}", "base", "mica-steel", card, "xy", z0, z1, tone=0.3, region="casing", sectionFaces=FACES) for name, z0, z1 in spans]


def slot_air(sign, name):
    """♨️ Outlet patch in a slot mouth around its slice."""
    z = (SLICE_Z0 - 0.001, SLICE_Z1 + 0.001) if sign > 0 else (-SLICE_Z1 - 0.001, -SLICE_Z0 + 0.001)
    hole = [[z[0], SLICE_X - 0.0595], [z[1], SLICE_X - 0.0595], [z[1], SLICE_X + 0.0595], [z[0], SLICE_X + 0.0595]]
    return extrude(f"th-slot-air-{name}", "base", "mica-steel", slot(sign, -0.0002), "zx", 0.1872, 0.1884, role="detail", tone=0.4, holes=[hole], tags=["slot-air"], sectionFaces=FACES, region="top-plate", layer="plate")


def thermal_50():
    return [
        vloft("th-casing", "base", "chrome-steel", PLINTH + WALL["50"], E, tone=0.82, region="casing", layer="sheet", sectionFaces=["+v"]),
        of("th-crumb-tray", "crumb-tray", region="casing", layer="sheet"),
        of("th-cord-sleeve", "cord-sleeve", region="casing", layer="sheet"),
        of("th-cord-drop", "cord-drop", region="casing", layer="sheet"),
        of("th-cord-run", "cord-run", region="casing", layer="sheet"),
        of("th-top-rim", "top-rim", region="top-plate", layer="plate"),
        of("th-top-plate", "top-plate", region="top-plate", layer="plate", sectionFaces=["-v"]),
        *[of(f"th-{part}-{name}", f"{part}-{name}", region="bread", layer="crust") for name in ("front", "rear") for part in ("slice", "crumb")],
        of("th-lever", "lever", region="control", layer="knob"),
        of("th-dial", "dial", region="control", layer="knob"),
        of("th-cancel-button", "cancel-button", region="control", layer="knob"),
        of("th-defrost-button", "defrost-button", region="control", layer="knob"),
        of("th-foot-left", "foot-left", region="feet", layer="foot"),
        of("th-foot-right", "foot-right", region="feet", layer="foot"),
        slot_air(-1, "front"),
        slot_air(1, "rear"),
        *cards([("outer-front", -0.0595, -0.052), ("inner-front", -0.0135, -0.006), ("inner-rear", 0.006, 0.0135), ("outer-rear", 0.052, 0.0595)]),
    ]


def tube(wall, y0, y1):
    """🧱 Toasting-chamber tube of sheet steel."""
    outer = [[-0.060, -0.088], [0.060, -0.088], [0.060, 0.068], [-0.060, 0.068]]
    inner = [[-0.060 + wall, -0.088 + wall], [0.060 - wall, -0.088 + wall], [0.060 - wall, 0.068 - wall], [-0.060 + wall, 0.068 - wall]]
    return extrude("chassis", "base", "mica-steel", outer, "zx", y0, y1, tone=0.5, holes=[inner], member="base", part="member")


def structure_50():
    return [
        of("base-frame", "base-tray", member="base", part="member"),
        tube(0.001, 0.030, 0.180),
        extrude("end-plate", "base", "mica-steel", [[0.030, -0.055], [0.180, -0.055], [0.180, 0.055], [0.030, 0.055]], "yz", 0.122, 0.1295, tone=0.5, holes=[[[0.086, -0.008], [0.152, -0.008], [0.152, 0.008], [0.086, 0.008]]], member="base", part="member"),
        box("end-strut", "base", "mica-steel", [0.0615, 0.0075, 0.110], [0.09875, 0.17625, 0.0], tone=0.5, member="base", part="member"),
        capsule("guide-rod", "base", "chrome-steel", [0.112, 0.031, 0.0], [0.112, 0.176, 0.0], 0.0025, role="detail", tone=0.5, slender=True, member="base", part="member"),
        of("member-lifter-front", "lifter-front", member="slices", part="member"),
        of("member-lifter-rear", "lifter-rear", member="slices", part="member"),
        box("yoke", "slices", "chrome-steel", [0.008, 0.030, 0.076], [0.088, 0.00775, 0.0], tone=0.5, member="slices", part="member"),
        box("yoke-arm", "slices", "chrome-steel", [0.036, 0.0075, 0.012], [0.108, 0.011, 0.0], tone=0.5, member="slices", part="member"),
        box("joint-slices", "slices", "chrome-steel", [0.012, 0.030, 0.012], [0.122, 0.00775, 0.0], role="detail", tone=0.5, member="slices", part="joint"),
        box("lever-arm", "lever", "chrome-steel", [0.034, 0.008, 0.008], [-0.011, 0.0, 0.0], tone=0.5, member="lever", part="member"),
        ellipsoid("joint-lever", "lever", "chrome-steel", [0.005, 0.005, 0.005], [-0.022, 0.0, 0.0], member="lever", part="joint"),
        capsule("dial-spindle", "dial", "chrome-steel", [-0.016, 0.0, 0.0], [0.004, 0.0, 0.0], 0.004, tone=0.5, member="dial", part="member"),
        ellipsoid("joint-dial", "dial", "chrome-steel", [0.004, 0.0065, 0.0065], [-0.008, 0.0, 0.0], member="dial", part="joint"),
        of("support-foot-left", "foot-left", member="base", part="support", anchor="foot-left"),
        of("support-foot-right", "foot-right", member="base", part="support", anchor="foot-right"),
        box("mass-base", "base", "mica-steel", [0.05, 0.03, 0.018], [0.0, 0.09, 0.0], role="detail", tone=0.5, member="base", part="mass"),
        ellipsoid("mass-slices", "slices", "bread-crust", [0.006, 0.006, 0.006], [0.0, 0.055, 0.0], member="slices", part="mass"),
    ]


def body_100():
    return [
        vloft("base-tray", "base", "pp-black", [(0.012, 0.079, 0.128), (0.031, 0.079, 0.128)], E, tone=0.2),
        *feet(),
        cord()[0],
        vloft("casing", "base", "chrome-steel", WALL["100"], E, tone=0.82, caps=False),
        vloft("top-rim", "base", "mica-steel", RIM["100"], E, tone=0.5, caps=False),
        extrude("top-plate", "base", "mica-steel", superellipse(DECK[0], DECK[1], E, 28), "zx", 0.173, TOP, tone=0.4, holes=[slot(-1, 0.0, 0.006), slot(1, 0.0, 0.006)]),
        extrude("chamber", "base", "mica-steel", [[-0.060, -0.090], [0.060, -0.090], [0.060, 0.070], [-0.060, 0.070]], "zx", 0.040, 0.173, tone=0.22, holes=[rrect(-SLOT_Z1, SLOT_X0, -SLOT_Z0, SLOT_X1, 0.004, 1), rrect(SLOT_Z0, SLOT_X0, SLOT_Z1, SLOT_X1, 0.004, 1)]),
        xloft("lever", "lever", "pp-black", [(-0.004, 0.0075, 0.019), (0.012, 0.0085, 0.020), (0.016, 0.0065, 0.018)], 4, tone=0.2),
        revolve("dial", "dial", "pp-black", [[-0.003, 0.0], [-0.003, 0.016], [0.004, 0.016], [0.012, 0.0125], [0.012, 0.0]], [1, 0, 0], [0.0, 0.0, 0.0], tone=0.25),
        box("carriage", "slices", "chrome-steel", [0.160, 0.015, 0.086], [0.008, -0.0075, 0.0], tone=0.5),
        extrude("slice-front", "slices", "bread-crust", BREAD_100, "xy", -0.0405, -0.0255, tone=0.7),
        extrude("slice-rear", "slices", "bread-crust", BREAD_100, "xy", 0.0255, 0.0405, tone=0.7),
    ]


def thermal_100():
    return [
        vloft("th-casing", "base", "chrome-steel", [(0.012, 0.079, 0.128), (0.0295, 0.079, 0.128)] + WALL["100"], E, tone=0.82, region="casing", layer="sheet", sectionFaces=["+v"]),
        of("th-cord-sleeve", "cord-sleeve", region="casing", layer="sheet"),
        of("th-top-rim", "top-rim", region="top-plate", layer="plate"),
        of("th-top-plate", "top-plate", region="top-plate", layer="plate", sectionFaces=["-v"]),
        of("th-slice-front", "slice-front", region="bread", layer="crust"),
        of("th-slice-rear", "slice-rear", region="bread", layer="crust"),
        of("th-lever", "lever", region="control", layer="knob"),
        of("th-dial", "dial", region="control", layer="knob"),
        of("th-foot-left", "foot-left", region="feet", layer="foot"),
        of("th-foot-right", "foot-right", region="feet", layer="foot"),
        *[box(f"th-slot-air-{name}", "base", "mica-steel", [0.016, 0.001, 0.030], [SLOT_X0 + 0.009, 0.1876, z], role="detail", tone=0.4, tags=["slot-air"], sectionFaces=FACES, repeat={"count": 2, "translation": [0.122, 0, 0]}, region="top-plate", layer="plate") for name, z in (("front", -0.033), ("rear", 0.033))],
        *cards([("outer-front", -0.066, -0.051), ("centre", -0.0125, 0.0125), ("outer-rear", 0.051, 0.066)]),
    ]


def structure_100():
    return [
        of("base-frame", "base-tray", member="base", part="member"),
        tube(0.002, 0.031, 0.173),
        box("end-plate", "base", "mica-steel", [0.015, 0.142, 0.110], [0.1215, 0.102, 0.0], tone=0.5, member="base", part="member"),
        capsule("guide-rod", "base", "chrome-steel", [0.112, 0.031, 0.0], [0.112, 0.176, 0.0], 0.0025, role="detail", tone=0.5, slender=True, member="base", part="member"),
        box("rod-bracket", "base", "mica-steel", [0.061, 0.015, 0.020], [0.0985, 0.1725, 0.0], tone=0.5, member="base", part="member"),
        extrude("carriage", "slices", "chrome-steel", [[-0.072, -0.015], [0.128, -0.015], [0.128, 0.023], [0.084, 0.023], [0.084, 0.0], [-0.072, 0.0]], "xy", -0.040, 0.040, tone=0.5, member="slices", part="member"),
        box("lever-arm", "lever", "chrome-steel", [0.034, 0.015, 0.015], [-0.011, 0.0, 0.0], tone=0.5, member="lever", part="member"),
        capsule("dial-spindle", "dial", "chrome-steel", [-0.016, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0075, tone=0.5, member="dial", part="member"),
        of("support-foot-left", "foot-left", member="base", part="support", anchor="foot-left"),
        of("support-foot-right", "foot-right", member="base", part="support", anchor="foot-right"),
        box("mass-base", "base", "mica-steel", [0.05, 0.03, 0.02], [0.0, 0.09, 0.0], role="detail", tone=0.5, member="base", part="mass"),
        ellipsoid("mass-slices", "slices", "bread-crust", [0.0075, 0.0075, 0.0075], [0.0, 0.055, 0.0], member="slices", part="mass"),
    ]


def body_200():
    return [
        vloft("casing", "base", "chrome-steel", BLOCK["200"], E, tone=0.82),
        extrude("top-plate", "base", "mica-steel", superellipse(DECK[0], DECK[1], E, 32), "zx", 0.158, 0.1885, tone=0.4),
        xloft("lever", "lever", "pp-black", [(-0.014, 0.015, 0.020), (0.016, 0.015, 0.020)], 4, tone=0.2),
        extrude("slice-front", "slices", "bread-crust", BREAD_200, "xy", -0.048, -0.018, tone=0.7),
        extrude("slice-rear", "slices", "bread-crust", BREAD_200, "xy", 0.018, 0.048, tone=0.7),
    ]


def bank(holes):
    """🔥 Element bank around the slots."""
    return extrude("th-element-bank", "base", "mica-steel", [[-0.064, -0.086], [0.064, -0.086], [0.064, 0.066], [-0.064, 0.066]], "zx", 0.05, 0.155, tone=0.3, holes=holes, region="casing", sectionFaces=FACES)


def thermal_200():
    return [
        of("th-casing", "casing", region="casing", layer="sheet"),
        of("th-top-plate", "top-plate", region="top-plate", layer="plate", sectionFaces=["-v"]),
        of("th-slice-front", "slice-front", region="bread", layer="crust"),
        of("th-slice-rear", "slice-rear", region="bread", layer="crust"),
        of("th-lever", "lever", region="control", layer="knob"),
        revolve("th-dial", "dial", "pp-black", [[0.0, 0.0], [0.0, 0.016], [0.003, 0.016], [0.003, 0.0]], [1, 0, 0], [0.0, 0.0, 0.0], role="detail", tone=0.25, region="control", layer="knob", sectionFaces=["-u"]),
        box("th-feet", "base", "rubber-foot", [0.030, 0.008, 0.142], [-FOOT_X, 0.004, 0.0], role="detail", tone=0.15, repeat={"count": 2, "translation": [2 * FOOT_X, 0, 0]}, region="feet", layer="foot", sectionFaces=["+v"]),
        *[box(f"th-slot-air-{name}", "base", "mica-steel", [0.016, 0.001, 0.030], [SLOT_X0 + 0.010, 0.1891, z], role="detail", tone=0.4, tags=["slot-air"], sectionFaces=FACES, repeat={"count": 2, "translation": [0.120, 0, 0]}, region="top-plate", layer="plate") for name, z in (("front", -0.033), ("rear", 0.033))],
        bank([[[-0.048, -0.078], [-0.018, -0.078], [-0.018, 0.058], [-0.048, 0.058]], [[0.018, -0.078], [0.048, -0.078], [0.048, 0.058], [0.018, 0.058]]]),
    ]


def structure_200():
    return [
        box("base-frame", "base", "pp-black", [0.256, 0.030, 0.158], [0.0, 0.027, 0.0], tone=0.3, member="base", part="member"),
        tube(0.006, 0.042, 0.158),
        extrude("carriage", "slices", "chrome-steel", [[-0.072, -0.030], [0.128, -0.030], [0.128, 0.035], [0.090, 0.035], [0.090, 0.0], [-0.072, 0.0]], "xy", -0.040, 0.040, tone=0.5, member="slices", part="member"),
        box("lever-arm", "lever", "chrome-steel", [0.040, 0.030, 0.030], [-0.012, 0.0, 0.0], tone=0.5, member="lever", part="member"),
        revolve("dial-spindle", "dial", "chrome-steel", [[-0.034, 0.0], [-0.034, 0.015], [0.0, 0.015], [0.0, 0.0]], [1, 0, 0], [0.0, 0.0, 0.0], tone=0.5, member="dial", part="member"),
        box("support-foot-left", "base", "rubber-foot", [0.030, 0.030, 0.142], [-FOOT_X, 0.015, 0.0], role="ground", tone=0.1, member="base", part="support", anchor="foot-left"),
        box("support-foot-right", "base", "rubber-foot", [0.030, 0.030, 0.142], [FOOT_X, 0.015, 0.0], role="ground", tone=0.1, member="base", part="support", anchor="foot-right"),
        ellipsoid("mass-base", "base", "mica-steel", [0.015, 0.015, 0.015], [0.0, 0.09, 0.0], member="base", part="mass"),
    ]


def body_500():
    return [
        vloft("casing", "base", "chrome-steel", BLOCK["500"], E, tone=0.82),
        xloft("lever", "lever", "pp-black", [(-0.059, 0.0375, 0.0375), (0.016, 0.0375, 0.0375)], 4, tone=0.2),
        extrude("slice", "slices", "bread-crust", BREAD_500, "xy", -0.045, 0.045, tone=0.7),
    ]


def thermal_500():
    return [
        of("th-casing", "casing", region="casing", layer="sheet"),
        extrude("th-top-plate", "base", "mica-steel", superellipse(DECK[0], DECK[1], E, 12), "zx", 0.187, 0.1885, tone=0.4, region="top-plate", layer="plate", sectionFaces=["-v"]),
        of("th-slice", "slice", region="bread", layer="crust"),
        of("th-control", "lever", region="control", layer="knob"),
        box("th-feet", "base", "rubber-foot", [0.230, 0.008, 0.030], [0.0, 0.004, -FOOT_Z], role="detail", tone=0.15, repeat={"count": 2, "translation": [0, 0, 2 * FOOT_Z]}, region="feet", layer="foot", sectionFaces=["+v"]),
        box("th-slot-air", "base", "mica-steel", [0.012, 0.001, 0.090], [SLOT_X0 + 0.008, 0.1891, 0.0], role="detail", tone=0.4, tags=["slot-air"], sectionFaces=FACES, repeat={"count": 2, "translation": [0.124, 0, 0]}, region="top-plate", layer="plate"),
        bank([[[-0.048, -0.078], [0.048, -0.078], [0.048, 0.058], [-0.048, 0.058]]]),
    ]


def structure_500():
    return [
        extrude("chassis", "base", "mica-steel", [[-0.070, -0.120], [0.070, -0.120], [0.070, 0.120], [-0.070, 0.120]], "zx", 0.012, 0.160, tone=0.5, holes=[[[-0.060, -0.110], [0.060, -0.110], [0.060, 0.110], [-0.060, 0.110]]], member="base", part="member"),
        box("carriage", "slices", "chrome-steel", [0.200, 0.020, 0.080], [0.028, -0.010, 0.0], tone=0.5, member="slices", part="member"),
        box("lever-arm", "lever", "chrome-steel", [0.040, 0.075, 0.040], [-0.012, -0.0175, 0.0], tone=0.5, member="lever", part="member"),
        revolve("dial-spindle", "dial", "chrome-steel", [[-0.040, 0.0], [-0.040, 0.010], [0.0, 0.010], [0.0, 0.0]], [1, 0, 0], [0.0, 0.0, 0.0], tone=0.5, member="dial", part="member"),
        box("support-foot-left", "base", "rubber-foot", [0.030, 0.012, 0.142], [-FOOT_X, 0.006, 0.0], role="ground", tone=0.1, member="base", part="support", anchor="foot-left"),
        box("support-foot-right", "base", "rubber-foot", [0.030, 0.012, 0.142], [FOOT_X, 0.006, 0.0], role="ground", tone=0.1, member="base", part="support", anchor="foot-right"),
        box("mass-base", "base", "mica-steel", [0.04, 0.04, 0.04], [0.0, 0.09, 0.0], role="detail", tone=0.5, member="base", part="mass"),
    ]


DECISIONS = {
    "50": [
        ("Body: four tapered rubber feet of 24 mm carry the moulded black plinth; the pressed chrome wrap, 268 mm long and 170 mm deep, is a skin of ten sections from its hem over the plinth to the cap seam at 0.176 m, with an end shoulder of 34 x 38 mm and a long shoulder of 22 mm; the dark cap is a rim of seven sections and a deck 8 mm deep with two real slots of 140 x 32 mm, set 10 mm towards the cord end so the mechanism has its compartment behind the control end; under the deck the toasting chamber with its two wells over the crumb pan, and in it the two lifter bars of the carriage on which the slices stand. The narrow end face carries the lowering lever (paddle 40 x 17 mm on a stem in a 70 mm slot), the browning dial of 32 mm with pointer and the cancel and defrost buttons beside it; the other end carries the crumb-tray pull and the strain-relief sleeve, from which the 7.5 mm cord drops to the worktop and runs along the end before it is cut at the outline. Each slice is a crust ring around its crumb.", "Körper: Vier konische Gummifüße von 24 mm tragen den gespritzten schwarzen Sockel; der gepresste Chrommantel, 268 mm lang und 170 mm tief, ist eine Haut aus zehn Schnitten vom Saum über dem Sockel bis zur Haubenfuge bei 0,176 m, mit einer Stirnschulter von 34 x 38 mm und einer Längsschulter von 22 mm; die dunkle Haube ist ein Rand aus sieben Schnitten und ein 8 mm tiefes Deck mit zwei echten Schlitzen von 140 x 32 mm, um 10 mm zum Kabelende versetzt, damit die Mechanik hinter der Bedienstirn ihr Fach hat; unter dem Deck die Röstkammer mit zwei Schächten über dem Krümelblech und darin die zwei Hubleisten des Schlittens, auf denen die Scheiben stehen. Die schmale Stirnseite trägt den Absenkhebel (Griffplatte 40 x 17 mm an einem Steg in einem 70-mm-Schlitz), den Bräunungsregler von 32 mm mit Zeiger und daneben Stopp- und Auftautaste; die andere Stirn trägt den Griff der Krümelschublade und die Knickschutztülle, aus der das 7,5-mm-Kabel auf die Arbeitsplatte fällt und an der Stirn entlangläuft, bevor es am Umriss endet. Jede Scheibe ist ein Krustenring um ihre Krume."),
        ("Thermal: the casing is the closed outer envelope of plinth and wrap up to the cap seam (its top face under the cap is marked) plus tray pull, sleeve and cord; the top plate is the rim skin and the deck with its slot rims, underside marked because it faces the elements; bread, controls and feet reuse their bodies; the slot air leaves through two outlet patches in the slot mouths around the slices; the heat sources are four element cards of 132 x 115 mm (mica wound with resistance ribbon) on both sides of each slot inside the casing.", "Thermik: Das Gehäuse ist die geschlossene Außenhülle von Sockel und Mantel bis zur Haubenfuge (ihre Oberseite unter der Haube ist markiert) samt Schubladengriff, Tülle und Kabel; die Deckplatte ist die Randhaut und das Deck mit seinen Schlitzrändern, die Unterseite markiert, weil sie zu den Heizkarten zeigt; Brot, Bedienung und Füße verwenden ihre Körper; die Schlitzluft tritt durch zwei Auslassflächen in den Schlitzmündungen um die Scheiben aus; die Wärmequellen sind vier Heizkarten von 132 x 115 mm (Glimmer mit Widerstandsband) beidseits jedes Schlitzes im Gehäuse."),
        ("Structure: the plinth is the base frame on its four feet (the two supports are the foot pairs); on it stands the chamber tube of 1 mm sheet steel; behind the control end a slotted end plate rises from the plinth and is tied to the chamber by a strut, which holds the top of the guide rod that stands on the plinth; the carriage is two lifter bars under the slices that pass through the chamber end to the yoke, whose arm ends in the sleeve sliding on the rod; the lever arm reaches from the sleeve through the end slot to the paddle, the dial spindle runs from the end plate through the end face; masses of the appliance and of the bread at their centres.", "Tragwerk: Der Sockel ist der Grundrahmen auf seinen vier Füßen (die zwei Auflager sind die Fußpaare); darauf steht das Kammerrohr aus 1 mm Stahlblech; hinter der Bedienstirn steigt eine geschlitzte Stirnplatte vom Sockel auf und ist über eine Strebe mit der Kammer verbunden, die das obere Ende der auf dem Sockel stehenden Führungsstange fasst; der Schlitten besteht aus zwei Hubleisten unter den Scheiben, die durch die Kammerstirn zum Joch laufen, dessen Arm in der auf der Stange gleitenden Hülse endet; der Hebelarm reicht von der Hülse durch den Stirnschlitz zur Griffplatte, die Reglerachse läuft von der Stirnplatte durch die Stirnseite; Massen des Geräts und des Brots in ihren Schwerpunkten."),
    ],
    "100": [
        ("Body for a 15 mm feature: plinth as one straight band, the wrap with five sections and the cap rim with four, the deck 15 mm deep with the two slots, the chamber below; lever paddle and dial stay as real parts on the narrow end face, the buttons, the lever slot, the tray pull and the dial pointer are left out; the slices are single solids thickened from 13 to 15 mm and stand on the carriage, one 15 mm bar in the chamber. Feet and cord sleeve keep their true size; the cord itself is left out.", "Körper für ein 15-mm-Merkmal: Sockel als ein gerades Band, der Mantel mit fünf Schnitten und der Haubenrand mit vier, das Deck 15 mm tief mit den zwei Schlitzen, darunter die Kammer; Hebelgriff und Drehknopf bleiben echte Teile an der schmalen Stirnseite, Tasten, Hebelschlitz, Schubladengriff und Reglerzeiger entfallen; die Scheiben sind Einzelkörper, von 13 auf 15 mm verdickt, und stehen auf dem Schlitten, einer 15-mm-Leiste in der Kammer. Füße und Kabeltülle behalten ihre wahre Größe; das Kabel selbst entfällt."),
        ("Thermal: closed envelope of plinth and wrap, cap rim and deck, bread, lever, dial and feet as their bodies, two outlet patches per slot; the two element cards between the slots are one centre element 25 mm thick and the outer cards are 15 mm thick.", "Thermik: geschlossene Hülle aus Sockel und Mantel, Haubenrand und Deck, Brot, Hebel, Drehknopf und Füße als ihre Körper, je Schlitz zwei Auslassflächen; die zwei Heizkarten zwischen den Schlitzen sind ein Mittelelement von 25 mm, die äußeren Karten sind 15 mm dick."),
        ("Structure: plinth frame, chamber tube, end plate with the rod bracket and the guide rod; the carriage is one part of lifter, yoke and sleeve; lever arm and dial spindle 15 mm thick; supports are the foot pairs.", "Tragwerk: Sockelrahmen, Kammerrohr, Stirnplatte mit Stangenhalter und Führungsstange; der Schlitten ist ein Teil aus Hubleiste, Joch und Hülse; Hebelarm und Reglerachse 15 mm dick; Auflager sind die Fußpaare."),
    ],
    "200": [
        ("Body for a 30 mm feature: one rounded block from the worktop with both shoulders in six sections, the dark deck let into its top, the lever as a 40 x 30 x 30 mm paddle on the narrow end face and the two slices thickened to 30 mm; feet, plinth, dial, buttons and cord are below the feature and merge or fall away.", "Körper für ein 30-mm-Merkmal: ein gerundeter Block ab der Arbeitsplatte mit beiden Schultern in sechs Schnitten, das dunkle Deck in seine Oberseite eingelassen, der Hebel als Griffplatte von 40 x 30 x 30 mm an der schmalen Stirnseite und die zwei auf 30 mm verdickten Scheiben; Füße, Sockel, Drehknopf, Tasten und Kabel liegen unter dem Merkmal und gehen auf oder entfallen."),
        ("Thermal: block as casing, deck as top plate, slices as bread, lever and a dial disc as controls, two foot strips, outlet patches at the slot ends and one element bank around the two slots.", "Thermik: Block als Gehäuse, Deck als Deckplatte, Scheiben als Brot, Hebel und eine Reglerscheibe als Bedienung, zwei Fußstreifen, Auslassflächen an den Schlitzenden und eine Heizbank um die zwei Schlitze."),
        ("Structure: base frame, chamber tube with 6 mm walls, carriage, lever arm, dial spindle, two foot-pair supports and the mass.", "Tragwerk: Grundrahmen, Kammerrohr mit 6 mm Wand, Schlitten, Hebelarm, Reglerachse, zwei Fußpaar-Auflager und die Masse."),
    ],
    "500": [
        ("Body for a 75 mm feature: the block with its shoulders in four sections, the lever end as one 75 mm lug that stands 16 mm proud of the narrow end face, and both slices as one 90 mm loaf top.", "Körper für ein 75-mm-Merkmal: der Block mit seinen Schultern in vier Schnitten, die Hebelstirn als eine 75-mm-Nase, die 16 mm vor der schmalen Stirnseite steht, und beide Scheiben als eine 90 mm breite Brotkuppe."),
        ("Thermal: block, top-plate skin, loaf, lever lug, two foot strips, outlet patches and one element bank around one well.", "Thermik: Block, Deckplattenhaut, Brot, Hebelnase, zwei Fußstreifen, Auslassflächen und eine Heizbank um einen Schacht."),
        ("Structure: chamber frame, carriage, lever arm, dial spindle, two supports and the mass.", "Tragwerk: Kammerrahmen, Schlitten, Hebelarm, Reglerachse, zwei Auflager und die Masse."),
    ],
}


def volumes():
    lists = {"50": (body_50, thermal_50, structure_50), "100": (body_100, thermal_100, structure_100), "200": (body_200, thermal_200, structure_200), "500": (body_500, thermal_500, structure_500)}
    return {scale: {"status": "authored", "denominator": int(scale), "body": b(), "thermal": t(), "structure": s(), "decisions": [{"en": en, "de": de} for en, de in DECISIONS[scale]]} for scale, (b, t, s) in lists.items()}


def side(stations, index):
    """📐 Right half of an outline from loft stations: index 2 for the long side view, 1 for the end view."""
    return [[s[index], s[0]] for s in stations]


def drawing_50():
    wall, rim = WALL["50"], RIM["50"]
    foot = lambda cx: poly([[cx - 0.012, 0.0], [cx + 0.012, 0.0], [cx + 0.012, 0.0135], [cx - 0.012, 0.0135]])
    elevation = [
        shape("cord-run", "base", "detail", 2.5, 0.2, "casing", "pp-black", **ellipse(CORD_X, CORD_R, CORD_R, CORD_R)),
        shape("cord-sleeve", "base", "detail", 2.6, 0.2, "casing", "pp-black", **poly([[-0.124, 0.014], [-0.131, 0.014], [-0.136, 0.0155], [-0.140, 0.004], [-0.1325, 0.004], [-0.1315, 0.0255], [-0.131, 0.026], [-0.124, 0.026]])),
        shape("foot-left", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **foot(-FOOT_X)),
        shape("foot-right", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **foot(FOOT_X)),
        shape("crumb-tray", "base", "detail", 2.85, 0.7, "casing", "chrome-steel", **rect(-0.134, 0.015, 0.008, 0.012)),
        shape("base-tray", "base", "body", 2.9, 0.2, "casing", "pp-black", **poly([[-0.123, 0.010], [0.123, 0.010], [0.128, 0.014], [0.128, 0.030], [-0.128, 0.030], [-0.128, 0.014]])),
        shape("slice-rear", "slices", "body", 2.4, 0.7, "bread", "bread-crust", **poly(BREAD)),
        shape("slice-front", "slices", "body", 2.5, 0.78, "bread", "bread-crust", **poly(BREAD)),
        shape("crumb-edge", "slices", "detail", 2.6, 0.3, "bread", filled=False, **poly(CRUMB[3:22], False)),
        shape("casing", "base", "body", 3, 0.82, "casing", "chrome-steel", **poly(mirror_outline(side(wall + rim[1:], 2)))),
        shape("top-plate", "base", "body", 3.2, 0.55, "top-plate", "mica-steel", **poly(mirror_outline(side(rim, 2)))),
        shape("seam-top", "base", "detail", 3.3, 0.3, "top-plate", filled=False, **poly([[-rim[0][2], SEAM], [rim[0][2], SEAM]], False)),
        shape("highlight", "base", "detail", 3.3, 0.1, "casing", filled=False, **poly([[-0.122, 0.05], [-0.122, 0.14]], False)),
        shape("dial", "dial", "body", 3.5, 0.25, "control", "pp-black", **poly(DIAL_HALF), solid=TURNED),
        shape("lever", "lever", "body", 3.5, 0.2, "control", "pp-black", **poly([[0.0, -0.004], [0.0036, -0.0078], [0.012, -0.0085], [0.016, -0.006], [0.016, 0.006], [0.012, 0.0085], [0.0036, 0.0078], [0.0, 0.004]])),
    ]
    front = [
        shape("foot-left", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **foot(FOOT_Z)),
        shape("foot-right", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **foot(-FOOT_Z)),
        shape("base-tray", "base", "body", 2.9, 0.2, "casing", "pp-black", **poly([[-0.074, 0.010], [0.074, 0.010], [0.079, 0.014], [0.079, 0.030], [-0.079, 0.030], [-0.079, 0.014]])),
        shape("slice-rear", "slices", "body", 2.4, 0.7, "bread", "bread-crust", **rect(SLICE_Z0, SLICE_Y, 0.013, 0.115, 0.003)),
        shape("slice-front", "slices", "body", 2.5, 0.78, "bread", "bread-crust", **rect(-SLICE_Z1, SLICE_Y, 0.013, 0.115, 0.003)),
        shape("casing", "base", "body", 3, 0.82, "casing", "chrome-steel", **poly(mirror_outline(side(wall + rim[1:], 1)))),
        shape("top-plate", "base", "body", 3.2, 0.55, "top-plate", "mica-steel", **poly(mirror_outline(side(rim, 1)))),
        shape("seam-top", "base", "detail", 3.3, 0.3, "top-plate", filled=False, **poly([[-rim[0][1], SEAM], [rim[0][1], SEAM]], False)),
        shape("lever-slot", "base", "detail", 3.3, 0.12, "casing", "pp-black", **rect(-0.0055, 0.086, 0.011, 0.066, 0.005)),
        shape("cancel-button", "base", "body", 3.4, 0.2, "control", "pp-black", **ellipse(-BUTTON_Z, DIAL_Y, 0.0085, 0.0085)),
        shape("defrost-button", "base", "body", 3.4, 0.2, "control", "pp-black", **ellipse(BUTTON_Z, DIAL_Y, 0.0085, 0.0085)),
        shape("dial", "dial", "body", 3.5, 0.25, "control", "pp-black", **ellipse(0.0, DIAL_Y, 0.016, 0.016)),
        shape("dial-pointer", "dial", "detail", 3.6, 0.9, "control", filled=False, **poly([[0.0, DIAL_Y + 0.002], [0.0, DIAL_Y + 0.0107]], False)),
        shape("lever", "lever", "body", 3.5, 0.2, "control", "pp-black", **rect(-0.020, LEVER_Y - 0.0085, 0.040, 0.017, 0.004)),
    ]
    plan = [
        shape("cord-run", "base", "detail", 2.5, 0.2, "casing", "pp-black", **rect(-0.140, -0.07975, 0.0075, 0.1335, 0.00375)),
        shape("cord-sleeve", "base", "detail", 2.6, 0.2, "casing", "pp-black", **rect(-0.136, CORD_Z - 0.006, 0.012, 0.012, 0.002)),
        shape("lever", "lever", "body", 2.7, 0.2, "control", "pp-black", **poly([[0.130, -0.004], [0.1376, -0.019], [0.146, -0.020], [0.150, -0.017], [0.150, 0.017], [0.146, 0.020], [0.1376, 0.019], [0.130, 0.004]])),
        shape("dial", "dial", "body", 2.7, 0.25, "control", "pp-black", **poly(DIAL_HALF), solid=TURNED),
        shape("casing", "base", "body", 3, 0.82, "casing", "chrome-steel", **poly(superellipse(HALF_L, HALF_D, E, 40))),
        shape("top-plate", "base", "body", 3.2, 0.55, "top-plate", "mica-steel", **poly(superellipse(rim[0][2], rim[0][1], E, 40))),
        shape("seam-top", "base", "detail", 3.3, 0.3, "top-plate", filled=False, **poly(superellipse(DECK[1], DECK[0], E, 40))),
        shape("slot-1", "base", "body", 3.3, 0.12, "top-plate", "mica-steel", **rect(SLOT_X0, SLOT_Z0, SLOT_X1 - SLOT_X0, SLOT_Z1 - SLOT_Z0, 0.005)),
        shape("slot-2", "base", "body", 3.3, 0.12, "top-plate", "mica-steel", **rect(SLOT_X0, -SLOT_Z1, SLOT_X1 - SLOT_X0, SLOT_Z1 - SLOT_Z0, 0.005)),
        shape("slice-rear", "slices", "body", 3.4, 0.7, "bread", "bread-crust", **rect(SLICE_X - 0.0585, SLICE_Z0, 0.117, 0.013, 0.003)),
        shape("slice-front", "slices", "body", 3.45, 0.78, "bread", "bread-crust", **rect(SLICE_X - 0.0585, -SLICE_Z1, 0.117, 0.013, 0.003)),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def drawing_100():
    wall, rim = WALL["100"], RIM["100"]
    elevation = [
        shape("cord-sleeve", "base", "detail", 2.6, 0.2, "casing", "pp-black", **rect(-0.136, 0.014, 0.012, 0.012)),
        shape("foot-left", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **rect(-FOOT_X - 0.012, 0.0, 0.024, 0.0135)),
        shape("foot-right", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **rect(FOOT_X - 0.012, 0.0, 0.024, 0.0135)),
        shape("base-tray", "base", "body", 2.9, 0.2, "casing", "pp-black", **rect(-0.128, 0.012, 0.256, 0.019)),
        shape("slice-front", "slices", "body", 2.5, 0.78, "bread", "bread-crust", **poly(BREAD_100)),
        shape("casing", "base", "body", 3, 0.82, "casing", "chrome-steel", **poly(mirror_outline(side(wall + rim[1:], 2)))),
        shape("top-bevel", "base", "detail", 3.3, 0.3, "top-plate", filled=False, **poly([[-rim[0][2], SEAM], [rim[0][2], SEAM]], False)),
        shape("dial", "dial", "body", 3.5, 0.25, "control", "pp-black", **poly([[0.0, 0.0], [0.0, 0.016], [0.012, 0.016], [0.012, 0.0]]), solid=TURNED),
        shape("lever", "lever", "body", 3.5, 0.2, "control", "pp-black", **rect(0.0, -0.0085, 0.016, 0.017)),
    ]
    front = [
        shape("foot-left", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **rect(FOOT_Z - 0.012, 0.0, 0.024, 0.0135)),
        shape("foot-right", "base", "body", 2.8, 0.15, "feet", "rubber-foot", **rect(-FOOT_Z - 0.012, 0.0, 0.024, 0.0135)),
        shape("base-tray", "base", "body", 2.9, 0.2, "casing", "pp-black", **rect(-0.079, 0.012, 0.158, 0.019)),
        shape("slice-rear", "slices", "body", 2.4, 0.78, "bread", "bread-crust", **rect(0.0255, SLICE_Y, 0.015, 0.115, 0.004)),
        shape("slice-front", "slices", "body", 2.5, 0.78, "bread", "bread-crust", **rect(-0.0405, SLICE_Y, 0.015, 0.115, 0.004)),
        shape("casing", "base", "body", 3, 0.82, "casing", "chrome-steel", **poly(mirror_outline(side(wall + rim[1:], 1)))),
        shape("top-bevel", "base", "detail", 3.3, 0.3, "top-plate", filled=False, **poly([[-rim[0][1], SEAM], [rim[0][1], SEAM]], False)),
        shape("dial", "dial", "body", 3.5, 0.25, "control", "pp-black", **ellipse(0.0, DIAL_Y, 0.016, 0.016)),
        shape("lever", "lever", "body", 3.5, 0.2, "control", "pp-black", **rect(-0.020, LEVER_Y - 0.0085, 0.040, 0.017)),
    ]
    plan = [
        shape("lever", "lever", "body", 2.7, 0.2, "control", "pp-black", **rect(0.130, -0.020, 0.020, 0.040)),
        shape("dial", "dial", "body", 2.7, 0.25, "control", "pp-black", **poly([[0.0, 0.0], [0.0, 0.016], [0.012, 0.016], [0.012, 0.0]]), solid=TURNED),
        shape("casing", "base", "body", 3, 0.82, "casing", "chrome-steel", **poly(superellipse(HALF_L, HALF_D, E, 24))),
        shape("top-bevel", "base", "detail", 3.3, 0.3, "top-plate", filled=False, **poly(superellipse(rim[0][2], rim[0][1], E, 24))),
        shape("slice-rear", "slices", "body", 3.4, 0.78, "bread", "bread-crust", **rect(SLICE_X - 0.0585, 0.0255, 0.117, 0.015, 0.004)),
        shape("slice-front", "slices", "body", 3.45, 0.78, "bread", "bread-crust", **rect(SLICE_X - 0.0585, -0.0405, 0.117, 0.015, 0.004)),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


def drawing_200():
    block = BLOCK["200"]
    regions = ["casing", "top-plate", "feet"]
    elevation = [
        shape("slice-rear", "slices", "body", 2.4, 0.78, "bread", "bread-crust", **poly(BREAD_200)),
        shape("slice-front", "slices", "body", 2.5, 0.78, "bread", "bread-crust", **poly(BREAD_200)),
        shape("casing", "base", "body", 3, 0.82, regions, "chrome-steel", **poly(mirror_outline(side(block, 2)))),
        shape("lever", "lever", "body", 3.5, 0.2, "control", "pp-black", **rect(-0.014, -0.015, 0.030, 0.030)),
    ]
    front = [
        shape("slice-rear", "slices", "body", 2.4, 0.78, "bread", "bread-crust", **rect(0.018, 0.100, 0.030, 0.100, 0.008)),
        shape("slice-front", "slices", "body", 2.5, 0.78, "bread", "bread-crust", **rect(-0.048, 0.100, 0.030, 0.100, 0.008)),
        shape("casing", "base", "body", 3, 0.82, regions, "chrome-steel", **poly(mirror_outline(side(block, 1)))),
        shape("lever", "lever", "body", 3.5, 0.2, "control", "pp-black", **rect(-0.020, LEVER_Y - 0.015, 0.040, 0.030)),
    ]
    plan = [
        shape("lever", "lever", "body", 2.7, 0.2, "control", "pp-black", **rect(0.120, -0.020, 0.030, 0.040)),
        shape("casing", "base", "body", 3, 0.82, regions, "chrome-steel", **poly(superellipse(HALF_L, HALF_D, E, 16))),
        shape("slice-rear", "slices", "body", 3.4, 0.78, "bread", "bread-crust", **rect(SLICE_X - 0.058, 0.018, 0.116, 0.030, 0.006)),
        shape("slice-front", "slices", "body", 3.45, 0.78, "bread", "bread-crust", **rect(SLICE_X - 0.058, -0.048, 0.116, 0.030, 0.006)),
    ]
    return {"elevation": elevation, "plan": plan, "front": front}


TEXT_50 = {
    "silhouette": ("A chromed two-slice toaster 6 mm long on paper, seen on its long side: the wrap with its rounded end shoulders stands on a recessed black plinth with two tapered feet, the dark cap closes it above the seam, the bread rises 12 mm out of the cap, the lever paddle and the dial stand proud of the narrow end on the right and the cord sleeve with the dropping cord on the left. Front view: the control end, 0.17 m deep with its 22 mm shoulders, with the lever in its slot above the dial and its two buttons. Top view: the pillow-shaped outline with the cap, the edge of its flat deck and the two slots with the bread in them, lever and dial in front of the narrow end, sleeve and cord behind the other.", "Ein verchromter Zweischeiben-Toaster, auf dem Papier 6 mm lang, von seiner Längsseite gesehen: Der Mantel mit seinen runden Stirnschultern steht auf einem zurückgesetzten schwarzen Sockel mit zwei konischen Füßen, die dunkle Haube schließt ihn über der Fuge, das Brot ragt 12 mm aus der Haube, rechts stehen Hebelgriff und Drehknopf vor der schmalen Stirn, links die Kabeltülle mit dem abfallenden Kabel. Vorderansicht: die Bedienstirn, 0,17 m tief mit ihren 22-mm-Schultern, mit dem Hebel in seinem Schlitz über dem Drehknopf und seinen zwei Tasten. Draufsicht: der kissenförmige Umriss mit der Haube, der Kante ihres flachen Decks und den zwei Schlitzen mit dem Brot darin, Hebel und Drehknopf vor der schmalen Stirn, Tülle und Kabel hinter der anderen."),
    "retained": ("Overall length 0.29 m from the cord to the lever paddle, wrap 0.268 m x 0.17 m, cap height 0.188 m, feet 0.0135 m and the 12 mm of bread above the cap exactly; plinth, crumb-tray pull, cap seam, lever slot, lever, dial with pointer, both buttons, sleeve, cord and the chrome highlight.", "Gesamtlänge 0,29 m vom Kabel bis zum Hebelgriff, Mantel 0,268 m x 0,17 m, Haubenhöhe 0,188 m, Füße 0,0135 m und die 12 mm Brot über der Haube exakt; Sockel, Griff der Krümelschublade, Haubenfuge, Hebelschlitz, Hebel, Drehknopf mit Zeiger, beide Tasten, Tülle, Kabel und die Chromlichtkante."),
    "merged": ("The slot rims are merged into the cap; the crust is one crumb line instead of a ring; both slices have the same outline, so the rear one lies exactly behind the front one; sleeve and dropping cord are one shape in the long-side view.", "Die Schlitzränder gehen in der Haube auf; die Kruste ist eine Krumenlinie statt eines Rings; beide Scheiben haben denselben Umriss, die hintere liegt also genau hinter der vorderen; Tülle und abfallendes Kabel sind in der Längsansicht eine Form."),
    "omitted": ("Logo, the cord beyond the outline with its plug, bun-warmer rack, bread guard wires in the slots and screws; the rear feet in the long-side view; the buttons in the long-side and top view, where the dial covers them.", "Logo, das Kabel jenseits des Umrisses mit seinem Stecker, Brötchenaufsatz, Brotführungsdrähte in den Schlitzen und Schrauben; in der Längsansicht die hinteren Füße; die Tasten in Längsansicht und Draufsicht, wo der Drehknopf sie verdeckt."),
    "thinElements": ("The lever slot is 0.01 m = 0.2 mm wide and filled dark; the cord is 7.5 mm = 0.15 mm thick; the crumb line and the dial pointer are 0.13 mm strokes; the buttons are 17 mm discs standing 4 mm proud.", "Der Hebelschlitz ist 0,01 m = 0,2 mm breit und dunkel gefüllt; das Kabel ist 7,5 mm = 0,15 mm dick; Krumenlinie und Reglerzeiger sind 0,13-mm-Striche; die Tasten sind 17-mm-Scheiben, die 4 mm vorstehen."),
}
TEXT_100 = {
    "silhouette": ("A toaster 3 mm long and 2 mm high: the chromed block with its end shoulders on a dark plinth strip and two stub feet, the seam line of the cap, the bread shoulders above it, lever and dial as stubs on the narrow end on the right and the cord sleeve on the left. Front view: the control end with the rounded wrap on plinth and feet, the lever bar above the round dial and the two slices thickened to 15 mm. Top view: the outline with the cap line and the two bread tops, the stubs of lever and dial in front of the narrow end.", "Ein Toaster, 3 mm lang und 2 mm hoch: der verchromte Block mit seinen Stirnschultern auf einem dunklen Sockelstreifen und zwei Stummelfüßen, die Fugenlinie der Haube, darüber die Brotschultern, rechts Hebel und Drehknopf als Stummel an der schmalen Stirn und links die Kabeltülle. Vorderansicht: die Bedienstirn mit dem gerundeten Mantel auf Sockel und Füßen, dem Hebelbalken über dem runden Drehknopf und den zwei auf 15 mm verdickten Scheiben. Draufsicht: der Umriss mit der Haubenlinie und den zwei Brotoberseiten, die Stummel von Hebel und Drehknopf vor der schmalen Stirn."),
    "retained": ("Overall size, plinth, the bread shoulders, cap seam, lever, dial and the cord sleeve in the long-side view.", "Gesamtgröße, Sockel, Brotschultern, Haubenfuge, Hebel, Drehknopf und in der Längsansicht die Kabeltülle."),
    "merged": ("The rear slice merges behind the front one; the cap is only its seam line; the buttons, the lever slot and the crumb-tray pull join the casing.", "Die hintere Scheibe geht hinter der vorderen auf; die Haube ist nur ihre Fugenlinie; Tasten, Hebelschlitz und Schubladengriff gehen im Gehäuse auf."),
    "omitted": ("Highlight, dial pointer, crumb line, guard wires, logo, the cord beyond its sleeve. Front and top view omit the slots and the rear feet.", "Lichtkante, Reglerzeiger, Krumenlinie, Führungsdrähte, Logo, das Kabel jenseits der Tülle. Vorder- und Draufsicht lassen die Schlitze und die hinteren Füße weg."),
    "thinElements": ("The rubber feet are 0.0135 m = 0.135 mm high and 0.24 mm wide and stay true; the cord sleeve is 12 mm = 0.12 mm thick and stays true; the slices are 0.013 m thick in reality and thickened to 0.015 m (0.15 mm) in the front and top view so they print.", "Die Gummifüße sind 0,0135 m = 0,135 mm hoch und 0,24 mm breit und bleiben wahr; die Kabeltülle ist 12 mm = 0,12 mm dick und bleibt wahr; die Scheiben sind real 0,013 m dick und in Vorder- und Draufsicht auf 0,015 m (0,15 mm) verdickt, damit sie drucken."),
}
TEXT_200 = {
    "silhouette": ("A toaster 1.5 mm long and 1 mm high: one rounded block from the worktop up with a bread dome rising 0.06 mm out of the top edge and the lever stub on the narrow end on the right. Front view: the 0.17 m deep block with the lever bar across its middle and the two slices, thickened to 30 mm, above it. Top view: the outline with the two bread tops and the lever stub in front of the narrow end.", "Ein Toaster, 1,5 mm lang und 1 mm hoch: ein gerundeter Block ab der Arbeitsplatte mit einer Brotkuppe, die 0,06 mm aus der Oberkante ragt, und rechts dem Hebelstummel an der schmalen Stirn. Vorderansicht: der 0,17 m tiefe Block mit dem Hebelbalken quer über seiner Mitte und darüber die zwei auf 30 mm verdickten Scheiben. Draufsicht: der Umriss mit den zwei Brotoberseiten und dem Hebelstummel vor der schmalen Stirn."),
    "retained": ("Wrap length 0.268 m, height 0.188 m, the lever paddle 16 mm proud of the narrow end, the bread shoulders above the top edge and the two slices as two.", "Mantellänge 0,268 m, Höhe 0,188 m, der Hebelgriff 16 mm vor der schmalen Stirn, die Brotschultern über der Oberkante und die zwei Scheiben als zwei."),
    "merged": ("Wrap, plinth, cap and feet merge into one block (its shape lists the three regions); the rear slice lies behind the front one in the long-side view.", "Mantel, Sockel, Haube und Füße gehen in einem Block auf (seine Form listet die drei Regionen); die hintere Scheibe liegt in der Längsansicht hinter der vorderen."),
    "omitted": ("Feet, plinth, dial, buttons, lever slot, seam, cord and every line; front and top view carry no slots or feet.", "Füße, Sockel, Drehknopf, Tasten, Hebelschlitz, Fuge, Kabel und jede Linie; Vorder- und Draufsicht tragen weder Schlitze noch Füße."),
    "thinElements": ("The 0.0135 m feet (0.07 mm) are merged into the block; the lever paddle is thickened from 17 mm to 0.03 m (0.15 mm); the slices are thickened from 0.013 m to 0.03 m (0.15 mm) with a 0.18 mm gap between them; the bread above the cap is 0.012 m = 0.06 mm and stays true.", "Die 0,0135 m hohen Füße (0,07 mm) gehen im Block auf; der Hebelgriff ist von 17 mm auf 0,03 m (0,15 mm) verdickt; die Scheiben sind von 0,013 m auf 0,03 m (0,15 mm) verdickt, mit 0,18 mm Abstand; das Brot über der Haube ist 0,012 m = 0,06 mm hoch und bleibt wahr."),
    "overlays": ("The block lists casing, top plate and feet, so the thermal overlay paints it with their area-weighted value; the slices keep the bread and the lever the controls.", "Der Block listet Gehäuse, Deckplatte und Füße, die Thermalansicht färbt ihn mit deren flächengewichtetem Wert; die Scheiben behalten das Brot und der Hebel die Bedienung."),
}
NOTE_EN = "Construction as modelled: four rubber feet of 24 mm and 13.5 mm height under a black polypropylene plinth, a pressed chromed sheet wrap 0.268 m long and 0.17 m deep with end shoulders of 34 mm x 38 mm and long shoulders of 22 mm radius, a dark steel cap from 0.176 m to 0.188 m with two slots of 140 mm x 32 mm 66 mm apart, a toasting chamber of sheet steel inside, and a carriage on a guide rod behind the control end; the lowering lever travels 40 mm in a slot of the narrow end face above the browning dial and the cancel and defrost buttons, its paddle stands 16 mm proud; the crumb tray pulls out of the other end, where the strain-relief sleeve and the mains cord stand 6 mm proud, which gives the 0.29 m overall length."
NOTE_DE = "Aufbau wie modelliert: vier Gummifüße von 24 mm und 13,5 mm Höhe unter einem schwarzen Polypropylen-Sockel, ein gepresster verchromter Blechmantel von 0,268 m Länge und 0,17 m Tiefe mit Stirnschultern von 34 mm x 38 mm und Längsschultern von 22 mm Radius, eine dunkle Stahlhaube von 0,176 m bis 0,188 m mit zwei Schlitzen von 140 mm x 32 mm im Abstand von 66 mm, innen eine Röstkammer aus Stahlblech und ein Schlitten an einer Führungsstange hinter der Bedienstirn; der Absenkhebel läuft 40 mm in einem Schlitz der schmalen Stirnseite über dem Bräunungsregler und den Tasten für Stopp und Auftauen, sein Griff steht 16 mm vor; die Krümelschublade wird aus der anderen Stirn gezogen, wo Knickschutztülle und Netzkabel 6 mm vorstehen, das ergibt die Gesamtlänge von 0,29 m."


def mutate(asset):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    asset["dimensions"]["widthM"] = 0.29
    next(r for r in asset["thermal"]["regions"] if r["id"] == "control")["areaM2"] = 0.0068
    for lang, old, new in (("en", "0.30 m wide", "0.29 m long"), ("de", "0,30 m breit", "0,29 m lang")):
        asset["summary"][lang] = asset["summary"][lang].replace(old, new)
    bones["slices"].update(x=SLICE_X, y=SLICE_Y)
    bones["lever"].update(x=END, y=LEVER_Y, lengthM=0.016, title={"en": "Lowering lever on the narrow end face", "de": "Absenkhebel an der schmalen Stirnseite"})
    bones["dial"].update(x=END, y=DIAL_Y, lengthM=0.012, joint={"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -10, "twistMaxDeg": 270}, title={"en": "Browning dial on the narrow end face", "de": "Bräunungsregler an der schmalen Stirnseite"})
    for anchor in asset["anchors"]:
        if anchor["id"] == "slot-outlet":
            anchor["x"] = SLICE_X
        if anchor["id"].startswith("foot-"):
            anchor["footprintLengthM"] = anchor["footprintWidthM"] = 0.024
    for pose in asset["poses"]:
        pose["bones"]["dial"] = {"twistDeg": 120}
    notes = asset["provenance"]["notes"]
    notes["en"] = re.sub(r"Construction as modelled:.*?overall depth\.", NOTE_EN, notes["en"].replace("Long-side view of a typical chromed two-slice pop-up toaster with its controls on the long face", "A typical chromed two-slice pop-up toaster with its lever and controls on the narrow end face"))
    notes["de"] = re.sub(r"Aufbau wie modelliert:.*?Gesamttiefe von 0,17 m\.", NOTE_DE, notes["de"].replace("Längsansicht eines typischen verchromten Zweischeiben-Toasters mit der Bedienung auf der Längsseite", "Ein typischer verchromter Zweischeiben-Toaster mit Hebel und Bedienung an der schmalen Stirnseite"))
    for scale, drawing, text in (("50", drawing_50, TEXT_50), ("100", drawing_100, TEXT_100), ("200", drawing_200, TEXT_200)):
        rep = asset["representations"][scale]
        rep["geometry"] = drawing()
        for key, (en, de) in text.items():
            rep["decisions"][key] = {"en": en, "de": de}


run(SLUG, mutate, volumes())
