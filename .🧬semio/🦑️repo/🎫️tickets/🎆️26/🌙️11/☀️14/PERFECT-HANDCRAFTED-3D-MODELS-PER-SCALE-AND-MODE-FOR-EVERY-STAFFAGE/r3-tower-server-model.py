"""🖥 Round 3, appliances/tower-server: every body, thermal and structural solid of every authored scale, stated by hand in world measures (see r3-tower-server-kit.py for the frame), and the volume decisions."""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("r3_tower_server_kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-tower-server-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

STEEL, ABS, RUBBER = K.STEEL, K.ABS, K.RUBBER
T = K.SHEET
INNER_X = (K.PAN + 0.1925) / 2
INNER_Y = (K.FLOOR_IN + K.TOP_UNDER) / 2
INNER_Z = (K.COVER_IN + K.WALL_IN) / 2


def window(spec):
    """🪟 A (a0, a1, b0, b1) window as a ring."""
    return K.rect(*spec)


def slot_rings():
    """🎚 The seven expansion slot apertures of the rear panel as (z, y) rings."""
    return [K.rect(K.SLOT_Z[0], K.SLOT_Z[1], K.SLOT_TOP - k * K.SLOT_PITCH - K.SLOT_H / 2, K.SLOT_TOP - k * K.SLOT_PITCH + K.SLOT_H / 2) for k in range(K.SLOTS)]


def ports():
    """🔌 The connector blocks of the input and output field as (id, z0, z1, y0, y1): display, serial, two network sockets and a block of four USB sockets."""
    return [("port-display", 0.061, 0.0735, 0.300, 0.331), ("port-serial", 0.061, 0.0735, 0.262, 0.293), ("port-network", 0.060, 0.0735, 0.222, 0.255), ("port-usb", 0.058, 0.0735, 0.192, 0.216)]


def fan(prefix, centre, tone, blades):
    """🌀 An axial fan of 120 mm blowing along the depth: square frame with its round duct, hub on the axis and, where the scale shows them, seven pitched blades."""
    cx, cy, cz = centre
    half = K.FAN / 2
    out = [
        K.alongx(prefix + "-frame", ABS, tone, K.rounded(cz - half, cz + half, cy - half, cy + half, 0.006), (cx - 0.0125, cx + 0.0125), holes=[list(reversed(K.circle(cz, cy, 0.057)))]),
        K.turned_x(prefix + "-hub", ABS, tone, [(-0.010, 0.0), (-0.010, 0.021), (0.010, 0.021), (0.010, 0.0)], centre),
    ]
    if blades:
        blade = K.box(prefix + "-blades", ABS, tone + 0.1, (cx - 0.001, cx + 0.001), (cy + 0.020, cy + 0.055), (cz - 0.016, cz + 0.016), role="texture")
        blade["transform"]["rotation"] = [K.r6(math.sin(math.radians(16))), 0, 0, K.r6(math.cos(math.radians(16)))]
        blade["repeat"] = {"count": 7, "rotationAxis": [0, -1, 0], "rotationRad": K.r6(2 * math.pi / 7), "origin": K.uvw(*centre)}
        out.append(blade)
    return out


def feet(radius=0.006):
    """🦶 The four rubber feet, 40 by 30 by 12 mm, as the front and the rear pair."""
    out = []
    for sid, sign in (("feet-front", 1), ("feet-rear", -1)):
        x = sign * K.FOOT_X
        pad = K.up(sid, RUBBER, 0.2, K.rounded(x - K.FOOT_L / 2, x + K.FOOT_L / 2, -K.FOOT_Z - K.FOOT_W / 2, -K.FOOT_Z + K.FOOT_W / 2, radius), (0.0, K.FOOT))
        out.append(K.step(pad, 2, dz=2 * K.FOOT_Z))
    return out


def contact(drop):
    """🟩 The support of the contact anchor: the floor patch of its 0.4 m by 0.17 m footprint under the feet."""
    return st(K.box("support-feet-contact", RUBBER, 0.2, (-0.200, 0.200), (-drop, 0.0), (-0.085, 0.085), role="ground"), "support", "feet-contact")


def body50():
    """🧱 1:50: the riveted steel chassis with its removable cover, the clipped ABS bezel and everything that shows through an opening."""
    gx0, gx1, gy0, gy1 = K.GRILLE
    lx0, lx1, ly0, ly1 = K.LATCH
    px0, px1, pz0, pz1 = K.PSU_TOP
    mz0, mz1, my0, my1 = K.MESH
    bz0, bz1, by0, by1 = K.BAY
    iz0, iz1, iy0, iy1 = K.IO
    tub = [(K.COVER_IN, K.FOOT), (0.097, K.FOOT), (0.0991, 0.0129), (0.1, 0.015), (0.1, K.TOP_UNDER), (K.WALL_IN, K.TOP_UNDER), (K.WALL_IN, 0.015), (0.0986, 0.0134), (0.097, K.FLOOR_IN), (K.COVER_IN, K.FLOOR_IN)]
    psu_face_holes = K.hexes(-0.072, 0.028, 0.355, 0.435, 0.008, 0.011) + [list(reversed(K.rect(0.035, 0.065, 0.360, 0.384))), list(reversed(K.rect(0.040, 0.061, 0.400, 0.415)))]
    solids = [
        *feet(),
        K.alongx("tub", STEEL, 0.55, tub, (K.PAN, K.CHASSIS_FRONT)),
        K.alongz("cover", STEEL, 0.55, K.rect(K.PAN, K.CHASSIS_FRONT, K.FOOT, K.TOP_UNDER), (-K.HALF, K.COVER_IN), holes=[list(reversed(window(K.GRILLE))), list(reversed(window(K.LATCH)))]),
        K.up("top-panel", STEEL, 0.58, K.rect(K.PAN, K.CHASSIS_FRONT, -K.HALF, K.HALF), (K.TOP_UNDER, K.TOP), holes=[list(reversed(window(K.PSU_TOP)))]),
        K.alongx("rear-panel", STEEL, 0.5, K.rounded(-K.HALF, K.HALF, K.FOOT, K.TOP, 0.003, 2), (K.REAR, K.PAN), holes=[list(reversed(K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]))), list(reversed(K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058))), list(reversed(window(K.IO)))] + [list(reversed(ring)) for ring in slot_rings()]),
        K.alongx("front-plate", STEEL, 0.3, K.rect(K.COVER_IN, K.WALL_IN, K.FLOOR_IN, K.TOP_UNDER), (0.1925, K.CHASSIS_FRONT), holes=[list(reversed(K.circle(K.FRONT_FAN[2], K.FRONT_FAN[1], 0.057))), list(reversed(window(K.BAY)))]),
        K.alongx("front-bezel", ABS, 0.2, K.rounded(-K.HALF, K.HALF, K.FOOT, K.TOP, 0.008, 4), (K.CHASSIS_FRONT, K.FRONT), holes=[list(reversed(K.rounded(mz0, mz1, my0, my1, 0.004, 2))), list(reversed(window(K.BAY))), list(reversed(K.circle(K.BUTTON[0], K.BUTTON[1], 0.007, 16))), list(reversed(K.rect(0.040, 0.070, 0.364, 0.372)))]),
        K.alongx("intake-mesh", STEEL, 0.32, K.rounded(mz0, mz1, my0, my1, 0.004, 2), (0.2182, 0.2190), holes=K.hexes(mz0, mz1, my0, my1, 0.009, 0.013), role="detail", texture=True),
        K.turned_x("power-button", ABS, 0.75, [(0.0, 0.0), (0.0, 0.007), (0.0075, 0.007), (0.0075, 0.0)], (0.2175, K.BUTTON[1], K.BUTTON[0]), role="detail"),
        K.box("usb-ports", ABS, 0.08, (0.2160, 0.2235), (0.364, 0.372), (0.040, 0.070), role="detail"),
        K.box("optical-drive", ABS, 0.16, (0.030, 0.2225), (by0 + 0.0005, by1 - 0.0005), (bz0 + 0.0005, bz1 - 0.0005)),
        K.box("drive-tray", ABS, 0.3, (0.2225, 0.2233), (0.398, 0.414), (-0.064, 0.064), role="detail", texture=True),
        K.alongz("cpu-grille", STEEL, 0.4, window(K.GRILLE), (-K.HALF, -K.HALF + T), holes=K.hexes(gx0, gx1, gy0, gy1, 0.008, 0.011), role="detail", texture=True),
        K.box("latch-well", STEEL, 0.25, (lx0, lx1), (ly0, ly1), (K.COVER_IN, K.COVER_IN + 0.0075), role="detail"),
        K.box("latch-lever", ABS, 0.45, (lx0 + 0.006, lx1 - 0.006), (ly0 + 0.005, ly1 - 0.005), (-0.0985, K.COVER_IN), role="detail"),
        K.up("psu-top-grille", STEEL, 0.42, window(K.PSU_TOP), (K.TOP - T, K.TOP), holes=K.hexes(px0, px1, pz0, pz1, 0.008, 0.011), role="detail", texture=True),
        K.alongx("psu-case", STEEL, 0.45, K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]), (-0.2237, K.PSU_X[1]), holes=[list(reversed(K.rect(K.PSU_Z[0] + T, K.PSU_Z[1] - T, K.PSU_Y[0] + T, K.PSU_Y[1] - T)))]),
        K.alongx("psu-rear-vent", STEEL, 0.42, K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]), (K.PSU_X[0], -0.2237), holes=psu_face_holes, role="detail", texture=True),
        K.box("psu-core", STEEL, 0.14, (-0.215, -0.090), (0.360, 0.430), (-0.065, 0.065)),
        K.box("psu-inlet", ABS, 0.1, (-0.2215, -0.2135), (0.360, 0.384), (0.035, 0.065), role="detail"),
        K.box("psu-switch", ABS, 0.18, (K.REAR, -0.2165), (0.400, 0.415), (0.040, 0.061), role="detail"),
        K.alongx("rear-fan-guard", STEEL, 0.5, K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058), (K.REAR, K.REAR + T), holes=K.ring_slots(K.REAR_FAN[2], K.REAR_FAN[1], [(0.0155, 0.0255), (0.0285, 0.0385), (0.0415, 0.0515)], 0.0025), role="detail", texture=True),
        *fan("rear-fan", (-0.2005, K.REAR_FAN[1], K.REAR_FAN[2]), 0.14, True),
        K.alongx("io-shield", STEEL, 0.75, window(K.IO), (-0.2246, -0.2238), holes=[list(reversed(K.rect(z0, z1, y0, y1))) for _, z0, z1, y0, y1 in ports()], role="detail", texture=True),
        *[K.box(sid, ABS, 0.1, (-0.2245, -0.2165), (y0, y1), (z0, z1), role="detail") for sid, z0, z1, y0, y1 in ports()],
        K.step(K.box("slot-covers", STEEL, 0.62, (-0.2238, -0.2230), (K.SLOT_TOP - K.SLOT_H / 2, K.SLOT_TOP + K.SLOT_H / 2), K.SLOT_Z, role="detail", texture=True), K.SLOTS, dy=-K.SLOT_PITCH),
        K.box("mainboard", ABS, 0.24, K.BOARD_X, K.BOARD_Y, K.BOARD_Z, role="detail", texture=True),
        K.box("cpu-cooler-base", STEEL, 0.6, (-0.140, -0.100), (0.255, 0.295), (0.071, K.BOARD_Z[0])),
        K.step(K.box("cpu-cooler-fins", STEEL, 0.68, (-0.1425, -0.0975), (0.215, 0.335), (0.0445, 0.0455), role="detail", texture=True), 13, dz=-0.008),
        K.step(K.capsule("cpu-cooler-pipes-low", STEEL, 0.62, (-0.131, 0.245, 0.071), (-0.131, 0.245, -0.054), 0.004), 2, dx=0.022),
        K.step(K.capsule("cpu-cooler-pipes-high", STEEL, 0.62, (-0.131, 0.305, 0.071), (-0.131, 0.305, -0.054), 0.004), 2, dx=0.022),
        *fan("cpu-fan", K.CPU_FAN, 0.14, True),
        *fan("front-fan", K.FRONT_FAN, 0.14, True),
    ]
    return solids


def body100():
    """🧱 1:100: the same machine in 15 mm plates: chassis angle, cover, top, rear and front plates, bezel, coarser perforations, the power supply, the rear fan and the cooler as plain masses."""
    mz0, mz1, my0, my1 = K.MESH
    gx0, gx1, gy0, gy1 = K.GRILLE
    px0, px1, pz0, pz1 = K.PSU_TOP
    slot_field = K.rect(K.SLOT_Z[0], K.SLOT_Z[1], 0.042, 0.176)
    rear, top_under, cover_in = -0.210, 0.435, -0.085
    return [
        *feet(0.0),
        K.alongx("chassis", STEEL, 0.55, [(cover_in, K.FOOT), (K.HALF, K.FOOT), (K.HALF, top_under), (0.085, top_under), (0.085, 0.027), (cover_in, 0.027)], (rear, K.CHASSIS_FRONT)),
        K.alongz("cover", STEEL, 0.55, K.rect(rear, K.CHASSIS_FRONT, K.FOOT, top_under), (-K.HALF, cover_in), holes=[list(reversed(window(K.GRILLE))), list(reversed(window(K.LATCH)))]),
        K.up("top-panel", STEEL, 0.58, K.rect(rear, K.CHASSIS_FRONT, -K.HALF, K.HALF), (top_under, K.TOP), holes=[list(reversed(window(K.PSU_TOP)))]),
        K.alongx("rear-panel", STEEL, 0.5, K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP), (K.REAR, rear), holes=[list(reversed(K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]))), list(reversed(K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058, 16))), list(reversed(window(K.IO))), list(reversed(slot_field))]),
        K.alongx("front-plate", STEEL, 0.3, K.rect(cover_in, 0.085, 0.027, top_under), (0.185, K.CHASSIS_FRONT), holes=[list(reversed(window(K.BAY)))]),
        K.alongx("front-bezel", ABS, 0.2, K.rounded(-K.HALF, K.HALF, K.FOOT, K.TOP, 0.008, 2), (K.CHASSIS_FRONT, K.FRONT), holes=[list(reversed(window(K.MESH))), list(reversed(window(K.BAY)))]),
        K.alongx("intake-mesh", STEEL, 0.32, window(K.MESH), (0.2170, 0.2190), holes=K.hexes(mz0, mz1, my0, my1, 0.016, 0.023, 0.006), role="detail", texture=True),
        K.box("optical-drive", ABS, 0.16, (0.030, 0.2220), (K.BAY[2], K.BAY[3]), (K.BAY[0], K.BAY[1])),
        K.alongz("cpu-grille", STEEL, 0.4, window(K.GRILLE), (-K.HALF, -0.098), holes=K.hexes(gx0, gx1, gy0, gy1, 0.015, 0.022, 0.006), role="detail", texture=True),
        K.box("latch-well", STEEL, 0.25, (K.LATCH[0], K.LATCH[1]), (K.LATCH[2], K.LATCH[3]), (cover_in, cover_in + 0.015), role="detail"),
        K.up("psu-top-grille", STEEL, 0.42, window(K.PSU_TOP), (0.448, K.TOP), holes=K.hexes(px0, px1, pz0, pz1, 0.015, 0.022, 0.006), role="detail", texture=True),
        K.box("psu-case", STEEL, 0.3, (-0.223, K.PSU_X[1]), K.PSU_Y, K.PSU_Z),
        K.alongx("psu-rear-vent", STEEL, 0.42, K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]), (K.REAR, -0.223), holes=K.hexes(-0.072, 0.028, 0.355, 0.435, 0.015, 0.022, 0.006) + [list(reversed(K.rect(0.035, 0.065, 0.360, 0.415)))], role="detail", texture=True),
        K.box("psu-inlet", ABS, 0.1, (-0.223, -0.208), (0.360, 0.415), (0.035, 0.065), role="detail"),
        K.alongx("rear-fan-guard", STEEL, 0.5, K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058, 16), (K.REAR, -0.223), holes=K.ring_slots(K.REAR_FAN[2], K.REAR_FAN[1], [(0.017, 0.034), (0.040, 0.0545)], 0.004, steps=4), role="detail", texture=True),
        *fan("rear-fan", (-0.1975, K.REAR_FAN[1], K.REAR_FAN[2]), 0.14, False),
        K.alongx("io-shield", STEEL, 0.75, window(K.IO), (-0.2245, -0.2225), holes=[list(reversed(K.rect(0.058, 0.0735, 0.192, 0.331)))], role="detail", texture=True),
        K.box("ports", ABS, 0.1, (-0.2245, -0.2095), (0.192, 0.331), (0.058, 0.0735), role="detail"),
        K.alongx("slot-panel", STEEL, 0.62, slot_field, (-0.2245, -0.2225), role="detail", texture=True),
        K.box("cpu-cooler", STEEL, 0.6, (-0.1425, -0.0725), (0.215, 0.335), (-0.055, 0.085)),
    ]


def body200():
    """🧱 1:200: 20 mm plates around one dark core: the openings of grille, power supply, fan, ports and intake are windows onto that core."""
    rear, front, top_under, cover_in = -0.205, 0.195, 0.430, -0.080
    return [
        K.step(K.box("feet", RUBBER, 0.2, (0.160, 0.200), (0.0, K.FOOT), (-0.090, 0.090)), 2, dx=-0.360),
        K.alongx("chassis", STEEL, 0.55, [(cover_in, K.FOOT), (K.HALF, K.FOOT), (K.HALF, top_under), (0.080, top_under), (0.080, 0.032), (cover_in, 0.032)], (rear, front)),
        K.alongz("cover", STEEL, 0.55, K.rect(rear, front, K.FOOT, top_under), (-K.HALF, cover_in), holes=[list(reversed(window(K.GRILLE)))]),
        K.up("top-panel", STEEL, 0.58, K.rect(rear, front, -K.HALF, K.HALF), (top_under, K.TOP), holes=[list(reversed(K.rect(-0.185, -0.105, -0.085, 0.085)))]),
        K.alongx("rear-panel", STEEL, 0.5, K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP), (K.REAR, rear), holes=[list(reversed(K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]))), list(reversed(K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058, 12))), list(reversed(K.rect(K.SLOT_Z[0], 0.0895, 0.042, 0.176)))]),
        K.alongx("front-bezel", ABS, 0.2, K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP), (front, K.FRONT), holes=[list(reversed(window(K.MESH))), list(reversed(window(K.BAY)))]),
        K.box("core", STEEL, 0.16, (rear, front), (0.032, top_under), (cover_in, 0.080)),
    ]


def body500():
    """🧱 1:500: a steel block on a recessed rubber plinth with the dark bezel block and its intake recess in front and the recesses of power supply and fan in the rear plate."""
    return [
        K.box("plinth", RUBBER, 0.2, (-0.200, 0.200), (0.0, K.FOOT), (-0.090, 0.090)),
        K.box("case", STEEL, 0.55, (-0.150, 0.150), (K.FOOT, K.TOP), (-K.HALF, K.HALF)),
        K.alongx("rear-panel", STEEL, 0.55, K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP), (K.REAR, -0.150), holes=[list(reversed(K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1]))), list(reversed(K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058, 12)))]),
        K.alongx("front-bezel", ABS, 0.2, K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP), (0.150, K.FRONT), holes=[list(reversed(window(K.MESH)))]),
    ]


def th(solid, region, layer=None, world=None):
    """🌡 Marks a solid as a thermal solid of a region (and layer); `world` names the faces that lie against the construction."""
    solid["region"] = region
    if layer:
        solid["layer"] = layer
    if world == "all":
        solid["sectionFaces"] = list(K.ALL_FACES)
    elif world:
        solid["sectionFaces"] = K.faces(*world)
    return solid


def emitters(scale):
    """💨 The two outlets as patches on the surface they leave: the rear fan opening and the warm top of the case over the power supply."""
    steps = {"50": 24, "100": 16, "200": 12, "500": 8}[scale]
    lift = {"50": 0.001, "100": 0.002, "200": 0.004, "500": 0.008}[scale]
    fan_patch = K.alongx("th-rear-exhaust", STEEL, 0.3, K.circle(K.REAR_FAN[2], K.REAR_FAN[1], 0.058, steps), (K.REAR - lift, K.REAR), role="texture", tags=["rear-exhaust"])
    plume = K.up("th-case-plume", STEEL, 0.3, K.rounded(-0.213, 0.050, -0.095, 0.095, 0.03, 2 if scale in ("50", "100") else 1), (K.TOP, K.TOP + lift), role="texture", tags=["case-plume"])
    return [th(fan_patch, "rear-panel", world="all"), th(plume, "psu-vent", world="all")]


def frame_skins(prefix, material, tone, region, layer, x, thick):
    """🖼 The four outward skins of a rim running through the depth (top, floor side, cover side, wall side), each with its inner face against the construction."""
    return [
        th(K.box(prefix + "-top", material, tone, x, (K.TOP - thick, K.TOP), (-K.HALF, K.HALF)), region, layer, ["-y", "-x", "+x"]),
        th(K.box(prefix + "-floor", material, tone, x, (K.FOOT, K.FOOT + thick), (-K.HALF, K.HALF)), region, layer, ["+y", "-x", "+x"]),
        th(K.box(prefix + "-cover", material, tone, x, (K.FOOT + thick, K.TOP - thick), (-K.HALF, -K.HALF + thick)), region, layer, ["+z", "-x", "+x", "+y", "-y"]),
        th(K.box(prefix + "-wall", material, tone, x, (K.FOOT + thick, K.TOP - thick), (K.HALF - thick, K.HALF)), region, layer, ["-z", "-x", "+x", "+y", "-y"]),
    ]


def thermal(scale, body):
    """🌡 The thermal list of a scale: the outer skins of the six regions in their real layer thickness, the two outlets, and at 1:50 and 1:100 the warm internals behind them."""
    rear = {"50": K.PAN, "100": -0.210, "200": K.REAR + T, "500": K.REAR + T}[scale]
    front = {"50": K.CHASSIS_FRONT, "100": K.CHASSIS_FRONT, "200": 0.195, "500": 0.150}[scale]
    fine = scale in ("50", "100")
    psu_face = K.rect(K.PSU_Z[0], K.PSU_Z[1], K.PSU_Y[0], K.PSU_Y[1])
    side = K.rect(rear, front, K.FOOT, K.TOP)
    cover_holes = [list(reversed(window(K.GRILLE)))] + ([list(reversed(window(K.LATCH)))] if scale == "50" else [])
    top_window = K.rect(-0.185, -0.105, -0.085, 0.085) if scale == "200" else window(K.PSU_TOP)
    out = [
        th(K.alongz("th-cover-skin", STEEL, 0.55, side, (-K.HALF, -K.HALF + T), holes=cover_holes), "side-panel", "sheet", ["+z"]),
        th(K.alongz("th-wall-skin", STEEL, 0.55, side, (K.HALF - T, K.HALF)), "side-panel", "sheet", ["-z"]),
        th(K.up("th-top-skin", STEEL, 0.58, K.rect(rear, front, -K.WALL_IN, K.WALL_IN), (K.TOP - T, K.TOP), holes=[list(reversed(top_window))]), "side-panel", "sheet", ["-y"]),
        th(K.alongz("th-cpu-grille", STEEL, 0.4, window(K.GRILLE), (-K.HALF, -K.HALF + T), role="detail"), "cpu-grille", "grille", ["+z"]),
    ]
    bezel_outline = K.rounded(-K.HALF, K.HALF, K.FOOT, K.TOP, 0.008, 4 if scale == "50" else 2) if fine else K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP)
    if fine:
        out += [
            th(K.alongx("th-bezel-face", ABS, 0.2, bezel_outline, (K.FRONT - 0.0025, K.FRONT), holes=[list(reversed(window(K.MESH))), list(reversed(window(K.BAY)))]), "front-bezel", "bezel", ["-x"]),
            *frame_skins("th-bezel-skirt", ABS, 0.2, "front-bezel", "bezel", (front, K.FRONT - 0.0025), 0.0025),
            th(K.alongx("th-intake-mesh", STEEL, 0.32, window(K.MESH), (0.2165, 0.2190), role="detail"), "front-bezel", "bezel", ["-x"]),
            th(K.alongx("th-bay-face", ABS, 0.16, window(K.BAY), (0.2200, 0.2225), role="detail"), "front-bezel", "bezel", ["-x"]),
        ]
    else:
        out.append(th(K.alongx("th-bezel-face", ABS, 0.2, bezel_outline, (K.FRONT - 0.0025, K.FRONT)), "front-bezel", "bezel", ["-x"]))
    out += [
        th(K.alongx("th-psu-rear", STEEL, 0.42, psu_face, (K.REAR, K.REAR + T), role="detail"), "psu-vent", "psu-shell", ["+x"]),
        th(K.up("th-psu-top", STEEL, 0.42, top_window, (K.TOP - T, K.TOP), role="detail"), "psu-vent", "psu-shell", ["-y"]),
        th(K.alongx("th-rear-face", STEEL, 0.5, K.rounded(-K.HALF, K.HALF, K.FOOT, K.TOP, 0.003, 2) if scale == "50" else K.rect(-K.HALF, K.HALF, K.FOOT, K.TOP), (K.REAR, K.REAR + T), holes=[list(reversed(psu_face))]), "rear-panel", "rear-sheet", ["+x"]),
    ]
    if fine:
        out += frame_skins("th-rear-rim", STEEL, 0.5, "rear-panel", "rear-sheet", (K.REAR + T, rear), T)
    out += [{"id": "th-" + solid["id"], "of": solid["id"], "region": "feet", "layer": "foot", "sectionFaces": K.faces("+y")} for solid in body if solid["material"] == RUBBER]
    out += emitters(scale)
    if fine:
        out += [
            th(K.box("th-psu-case", STEEL, 0.45, K.PSU_X if scale == "50" else (-0.223, K.PSU_X[1]), (K.PSU_Y[0], K.PSU_Y[1] - (0.001 if scale == "50" else 0.003)), K.PSU_Z), "psu-vent", world="all"),
            th(K.box("th-heat-sink", STEEL, 0.6, (-0.1425, -0.0975) if scale == "50" else (-0.1425, -0.0725), (0.215, 0.335), (-0.052, 0.046) if scale == "50" else (-0.055, 0.085)), "cpu-grille", world="all"),
            th(K.box("th-rear-fan", ABS, 0.14, (-0.213, -0.188) if scale == "50" else (-0.210, -0.185), (K.REAR_FAN[1] - 0.06, K.REAR_FAN[1] + 0.06), (K.REAR_FAN[2] - 0.06, K.REAR_FAN[2] + 0.06)), "rear-panel", world="all"),
        ]
    if scale == "50":
        out.append(th(K.box("th-front-fan", ABS, 0.14, (0.1675, 0.1925), (K.FRONT_FAN[1] - 0.06, K.FRONT_FAN[1] + 0.06), (-0.06, 0.06)), "front-bezel", world="all"))
    return out


def st(solid, part, anchor=None):
    """🏗 Marks a solid as a structural solid of the case bone."""
    solid["member"] = "case"
    solid["part"] = part
    if anchor:
        solid["anchor"] = anchor
    return solid


def angle_xz(sid, corner, to_x, to_z, y, leg=0.020):
    """📐 A standing folded edge: an angle of sheet steel with its corner at a world (x, z) point and legs towards `to_x` and `to_z` (signs)."""
    cx, cz = corner
    return K.up(sid, STEEL, 0.5, [(cx, cz), (cx + to_x * leg, cz), (cx + to_x * leg, cz + to_z * T), (cx + to_x * T, cz + to_z * T), (cx + to_x * T, cz + to_z * leg), (cx, cz + to_z * leg)], y)


def angle_zy(sid, corner, to_z, to_y, x, leg=0.020):
    """📐 A folded edge running through the depth: corner at a world (z, y) point."""
    cz, cy = corner
    return K.alongx(sid, STEEL, 0.5, [(cz, cy), (cz + to_z * leg, cy), (cz + to_z * leg, cy + to_y * T), (cz + to_z * T, cy + to_y * T), (cz + to_z * T, cy + to_y * leg), (cz, cy + to_y * leg)], x)


def angle_xy(sid, corner, to_x, to_y, z, leg=0.020):
    """📐 A folded edge running across the width: corner at a world (x, y) point."""
    cx, cy = corner
    return K.alongz(sid, STEEL, 0.5, [(cx, cy), (cx + to_x * leg, cy), (cx + to_x * leg, cy + to_y * T), (cx + to_x * T, cy + to_y * T), (cx + to_x * T, cy + to_y * leg), (cx, cy + to_y * leg)], z)


def structure50():
    """🏗 1:50: the load path of the riveted chassis in its real 0.8 mm sheet: floor pan, mainboard wall, the folded edges as posts and rails, shelf and cages, riveted corners, the four feet on the floor patch of the contact, and the heavy components as masses where they are fixed."""
    x0, x1 = K.PAN, 0.1925
    y0, y1 = K.FLOOR_IN, K.TOP_UNDER
    z0, z1 = K.COVER_IN, K.WALL_IN
    floor = [(z0, y0 + 0.020), (z0, y0 - T), (z1, y0 - T), (z1, y0 + 0.020), (z1 - T, y0 + 0.020), (z1 - T, y0), (z0 + T, y0), (z0 + T, y0 + 0.020)]
    wall = [(x0, z1 - 0.020), (x0, z1), (x1, z1), (x1, z1 - 0.020), (x1 - T, z1 - 0.020), (x1 - T, z1 - T), (x0 + T, z1 - T), (x0 + T, z1 - 0.020)]
    bay = [(z0, 0.4392), (z1 - T, 0.4392), (z1 - T, 0.4384), (0.075, 0.4384), (0.075, 0.380), (0.0742, 0.380), (0.0742, 0.4384), (-0.0742, 0.4384), (-0.0742, 0.380), (-0.075, 0.380), (-0.075, 0.4384), (z0, 0.4384)]
    cage = [(-0.055, 0.130), (-0.055, y0), (0.055, y0), (0.055, 0.130), (0.0542, 0.130), (0.0542, y0 + T), (-0.0542, y0 + T), (-0.0542, 0.130)]
    shelf = [(0.060, 0.3512), (z1 - T, 0.3512), (z1 - T, 0.3312), (z1 - 2 * T, 0.3312), (z1 - 2 * T, 0.3504), (0.060, 0.3504)]
    joint = lambda sid, x, y, z: K.box(sid, STEEL, 0.4, (x - 0.005, x + 0.005), (y - 0.005, y + 0.005), (z - 0.005, z + 0.005), role="detail")
    out = [
        st(K.alongx("member-case-floor", STEEL, 0.5, floor, (x0, x1)), "member"),
        st(K.up("member-case-board-wall", STEEL, 0.5, wall, (y0 + 0.020, y1)), "member"),
        st(angle_xz("member-case-post-front", (x1, z0), -1, 1, (y0 + 0.020, y1)), "member"),
        st(angle_xz("member-case-post-rear", (x0, z0), 1, 1, (y0 + 0.020, y1)), "member"),
        st(angle_zy("member-case-rail-top-cover", (z0, y1), 1, -1, (x0, x1)), "member"),
        st(angle_zy("member-case-rail-top-wall", (z1 - T, y1), -1, -1, (x0 + 0.020, x1 - 0.020)), "member"),
        st(angle_xy("member-case-rail-top-front", (x1, y1 - T), -1, -1, (z0 + 0.020, z1 - 0.020)), "member"),
        st(angle_xy("member-case-rail-top-rear", (x0, y1 - T), 1, -1, (z0 + 0.020, z1 - 0.020)), "member"),
        st(angle_xy("member-case-rail-floor-front", (x1, y0), -1, 1, (z0 + T, z1 - T)), "member"),
        st(angle_xy("member-case-rail-floor-rear", (x0, y0), 1, 1, (z0 + T, z1 - T)), "member"),
        st(K.alongx("member-case-psu-shelf", STEEL, 0.5, shelf, (x0, K.PSU_X[1])), "member"),
        st(K.alongx("member-case-drive-cage", STEEL, 0.5, cage, (0.030, 0.190)), "member"),
        st(K.alongx("member-case-bay-cage", STEEL, 0.5, bay, (0.030, 0.190)), "member"),
        st(K.half_turn(joint("joint-case-floor-a", x1 - 0.005, y0 + 0.005, z1 - 0.005), "y", (INNER_X, 0, INNER_Z)), "joint"),
        st(K.half_turn(joint("joint-case-floor-b", x1 - 0.005, y0 + 0.005, z0 + 0.005), "y", (INNER_X, 0, INNER_Z)), "joint"),
        st(K.half_turn(joint("joint-case-top-a", x1 - 0.005, y1 - 0.005, z1 - 0.005), "y", (INNER_X, 0, INNER_Z)), "joint"),
        st(K.half_turn(joint("joint-case-top-b", x1 - 0.005, y1 - 0.005, z0 + 0.005), "y", (INNER_X, 0, INNER_Z)), "joint"),
        {"id": "member-case-feet-front", "of": "feet-front", "member": "case", "part": "member"},
        {"id": "member-case-feet-rear", "of": "feet-rear", "member": "case", "part": "member"},
        contact(0.0015),
        st(K.box("mass-case-psu", STEEL, 0.15, (x0, K.PSU_X[1]), (0.352, 0.438), K.PSU_Z, role="detail"), "mass"),
        st(K.box("mass-case-board", ABS, 0.15, K.BOARD_X, K.BOARD_Y, (0.075, z1 - T), role="detail"), "mass"),
        st(K.box("mass-case-cooler", STEEL, 0.15, (-0.1425, -0.0725), (0.215, 0.335), (-0.055, 0.075), role="detail"), "mass"),
        st(K.step(K.box("mass-case-drives", STEEL, 0.15, (0.040, 0.187), (y0 + 0.008, y0 + 0.034), (-0.0508, 0.0508), role="detail"), 2, dy=0.036), "mass"),
        st(K.box("mass-case-optical", ABS, 0.15, (0.030, 0.190), (0.3855, 0.4265), (-0.073, 0.073), role="detail"), "mass"),
    ]
    return out


def structure_coarse(scale):
    """🏗 1:100 to 1:500: the same load path in bars and plates of the scale: floor, mainboard wall, the two posts of the cover side, the top frame, feet and masses."""
    bar = {"100": 0.015, "200": 0.020, "500": 0.030}[scale]
    x0 = {"100": -0.210, "200": -0.205, "500": -0.150}[scale]
    x1 = {"100": 0.185, "200": 0.195, "500": 0.150}[scale]
    y0, y1 = K.FOOT, {"100": 0.435, "200": 0.430, "500": 0.450}[scale]
    z0, z1 = {"100": -0.085, "200": -0.080, "500": -0.100}[scale], K.HALF
    out = [
        st(K.box("member-case-floor", STEEL, 0.5, (x0, x1), (y0, y0 + bar), (z0, z1)), "member"),
        st(K.box("member-case-board-wall", STEEL, 0.5, (x0, x1), (y0 + bar, y1 - bar), (z1 - bar, z1)), "member"),
        st(K.step(K.box("member-case-posts", STEEL, 0.5, (x0, x0 + bar), (y0 + bar, y1 - bar), (z0, z0 + bar)), 2, dx=x1 - x0 - bar), "member"),
        st(K.up("member-case-top-frame", STEEL, 0.5, K.rect(x0, x1, z0, z1), (y1 - bar, y1), holes=[list(reversed(K.rect(x0 + bar, x1 - bar, z0 + bar, z1 - bar)))]), "member"),
    ]
    if scale == "100":
        out.append(st(K.box("member-case-psu-shelf", STEEL, 0.5, (x0, K.PSU_X[1]), (0.337, 0.352), (0.060, z1 - bar)), "member"))
        out.append(st(K.step(K.box("joint-case-corners", STEEL, 0.4, (x1 - bar, x1), (y1 - bar, y1), (z0, z0 + bar), role="detail"), 2, dz=z1 - z0 - bar), "joint"))
        out += [{"id": "member-case-feet-front", "of": "feet-front", "member": "case", "part": "member"}, {"id": "member-case-feet-rear", "of": "feet-rear", "member": "case", "part": "member"}, contact(0.003)]
        out += [
            st(K.box("mass-case-psu", STEEL, 0.15, (x0, K.PSU_X[1]), (0.352, y1 - bar), K.PSU_Z, role="detail"), "mass"),
            st(K.box("mass-case-board", ABS, 0.15, (x0, 0.033), (0.045, 0.335), (-0.055, z1 - bar), role="detail"), "mass"),
            st(K.box("mass-case-drives", STEEL, 0.15, (0.040, x1), (y0 + bar, 0.100), (-0.0508, 0.0508), role="detail"), "mass"),
            st(K.box("mass-case-optical", ABS, 0.15, (0.030, x1), (0.3855, y1 - bar), (-0.073, 0.073), role="detail"), "mass"),
        ]
    elif scale == "200":
        out += [{"id": "member-case-feet", "of": "feet", "member": "case", "part": "member"}, contact(0.006)]
        out += [
            st(K.box("mass-case-psu", STEEL, 0.15, (x0, -0.085), (0.350, y1 - bar), K.PSU_Z, role="detail"), "mass"),
            st(K.box("mass-case-board", ABS, 0.15, (x0, 0.030), (0.045, 0.335), (-0.050, z1 - bar), role="detail"), "mass"),
            st(K.box("mass-case-drives", STEEL, 0.15, (0.040, x1), (y0 + bar, 0.100), (-0.050, 0.050), role="detail"), "mass"),
        ]
    else:
        out += [{"id": "member-case-plinth", "of": "plinth", "member": "case", "part": "member"}, st(K.box("support-feet-contact", RUBBER, 0.2, (-0.200, 0.200), (-0.015, 0.0), (-0.090, 0.090), role="ground"), "support", "feet-contact")]
        out += [
            st(K.box("mass-case-psu", STEEL, 0.15, (x0, -0.075), (0.345, y1 - bar), K.PSU_Z, role="detail"), "mass"),
            st(K.box("mass-case-internals", ABS, 0.15, (x0, x1), (y0 + bar, 0.320), (-0.060, z1 - bar), role="detail"), "mass"),
        ]
    return out


def structure(scale):
    """🏗 The structural list of a scale."""
    return structure50() if scale == "50" else structure_coarse(scale)


def body(scale):
    """🧱 The body list of a scale."""
    return {"50": body50, "100": body100, "200": body200, "500": body500}[scale]()


DECISIONS = {
    "50": [
        ("Body: a pedestal server as it is built. A chassis of 0.8 mm sheet steel (floor and the wall that carries the mainboard), a removable side cover with its release latch and a honeycomb field over the processor cooler, a top panel with the intake field of the power supply, a 12 mm rear panel with the openings of power supply, 120 mm fan, port field and seven expansion slots, and a 25 mm ABS bezel with mesh, drive bay, power button and USB sockets on four rubber feet. Behind the openings stand the parts that show through them: power supply with inlet and switch, rear fan with guard, port field, slot covers, tower cooler with fins, heat pipes and fan, mainboard, front fan and optical drive. Cover, top, rear panel and bezel are plates as deep as their folded edges (10, 10, 12 and 25 mm); the 5 mm honeycomb is drawn as 8 mm cells.",
         "Körper: ein Tower-Server, wie er gebaut ist. Ein Chassis aus 0,8 mm Stahlblech (Boden und die Wand, die das Mainboard trägt), ein abnehmbarer Seitendeckel mit Entriegelung und Wabenfeld über dem Prozessorkühler, ein Deckblech mit dem Ansaugfeld des Netzteils, eine 12 mm tiefe Rückwand mit den Öffnungen für Netzteil, 120-mm-Lüfter, Anschlussfeld und sieben Steckplätze sowie eine 25 mm tiefe ABS-Blende mit Gitter, Laufwerksschacht, Taster und USB-Buchsen auf vier Gummifüßen. Hinter den Öffnungen stehen die Teile, die durch sie sichtbar sind: Netzteil mit Buchse und Schalter, Rücklüfter mit Schutzgitter, Anschlussfeld, Slotbleche, Turmkühler mit Lamellen, Wärmerohren und Lüfter, Mainboard, Frontlüfter und optisches Laufwerk. Deckel, Deckblech, Rückwand und Blende sind Platten in der Tiefe ihrer Abkantungen (10, 10, 12 und 25 mm); die 5-mm-Waben sind als 8-mm-Zellen gezeichnet."),
        ("Thermal: every region is its outer skin in the real layer thickness, 0.8 mm steel for cover, wall, top, grille, power supply face and rear panel with its rim, 2.5 mm ABS for the bezel face and skirt with mesh and bay face, the rubber feet themselves; faces lying against the construction are marked. The exhaust is a patch in the rear fan opening and the plume a patch on the top, both a millimetre proud. Power supply, heat sink and the two fans stand inside as the warm parts behind those skins and count for no area.",
         "Thermisch: jede Region ist ihre Außenhaut in der wirklichen Schichtdicke, 0,8 mm Stahl für Deckel, Wand, Deckblech, Gitter, Netzteilfläche und Rückwand mit Rand, 2,5 mm ABS für Blendenfläche und -zarge mit Gitter und Schachtfront, dazu die Gummifüße selbst; Flächen, die an der Konstruktion anliegen, sind markiert. Die Abluft ist ein Feld in der Öffnung des Rücklüfters und die Auftriebsfahne ein Feld auf dem Deckblech, beide einen Millimeter erhaben. Netzteil, Kühlkörper und die beiden Lüfter stehen als warme Teile hinter diesen Häuten im Inneren und zählen nicht zur Fläche."),
        ("Structure: the chassis as a load path in real sheet thickness: floor pan with upturned edges on the four feet, the mainboard wall, two posts on the cover side, the folded top and floor rails, the shelf under the power supply, the drive cage on the floor and the bay cage under the top rails, eight riveted corners, the four feet on the floor patch of the contact; the masses are the power supply on its shelf, mainboard and cooler on the wall, two disk drives in the cage and the optical drive in the bay. Cover, top panel, rear panel and bezel carry nothing and are absent.",
         "Tragwerk: das Chassis als Lastpfad in wirklicher Blechdicke: Bodenwanne mit aufgestellten Rändern auf den vier Füßen, die Mainboard-Wand, zwei Pfosten auf der Deckelseite, die gekanteten Dach- und Bodenholme, der Winkel unter dem Netzteil, der Laufwerkskäfig auf dem Boden und der Schachtkäfig unter den Dachholmen, acht genietete Ecken, die vier Füße auf dem Bodenfeld des Kontakts; die Massen sind das Netzteil auf seinem Winkel, Mainboard und Kühler an der Wand, zwei Festplatten im Käfig und das optische Laufwerk im Schacht. Deckel, Deckblech, Rückwand und Blende tragen nichts und fehlen."),
    ],
    "100": [
        ("Body: built anew in 15 mm plates: chassis angle, cover with grille window and latch recess, top with the intake field, rear panel with the openings of power supply, fan and ports and one field for the slots, front plate and bezel with mesh and bay; the perforations are 15 mm cells, the fan guard two rings, the ports one block, power supply and cooler plain masses. Button, USB sockets, blades, fins, heat pipes, mainboard and front fan are left out; the feet are plain pads.",
         "Körper: neu aus 15-mm-Platten gebaut: Chassiswinkel, Deckel mit Gitterfenster und Griffmulde, Deckblech mit Ansaugfeld, Rückwand mit den Öffnungen für Netzteil, Lüfter und Anschlüsse und einem Feld für die Steckplätze, Frontblech und Blende mit Gitter und Schacht; die Lochungen sind 15-mm-Zellen, das Lüftergitter zwei Ringe, die Anschlüsse ein Block, Netzteil und Kühler schlichte Massen. Taster, USB-Buchsen, Flügel, Lamellen, Wärmerohre, Mainboard und Frontlüfter entfallen; die Füße sind schlichte Klötze."),
        ("Thermal: the same six skins without the latch opening, the bezel as face, skirt, mesh and bay face, the outlets two millimetres proud; power supply, cooler and rear fan remain as internals, the front fan is left out.",
         "Thermisch: dieselben sechs Häute ohne die Griffmulde, die Blende als Fläche, Zarge, Gitter und Schachtfront, die Auslässe zwei Millimeter erhaben; Netzteil, Kühler und Rücklüfter bleiben als Innenteile, der Frontlüfter entfällt."),
        ("Structure: floor and mainboard wall as 15 mm plates, the two posts of the cover side, one top frame, the shelf under the power supply, the two front top corners as joints, the four feet on the floor patch of the contact, and four masses: power supply, mainboard with cooler, the drives as one block and the optical drive.",
         "Tragwerk: Boden und Mainboard-Wand als 15-mm-Platten, die zwei Pfosten der Deckelseite, ein Dachrahmen, der Winkel unter dem Netzteil, die beiden vorderen oberen Ecken als Knoten, die vier Füße auf dem Bodenfeld des Kontakts und vier Massen: Netzteil, Mainboard mit Kühler, die Laufwerke als ein Block und das optische Laufwerk."),
    ],
    "200": [
        ("Body: 20 mm plates around one dark core: the grille, the intake field of the power supply, the openings of power supply, fan and ports and the mesh and bay of the bezel are windows onto that core; the feet of each end are one cross skid. Latch, guards, perforations and all single components are merged into the core.",
         "Körper: 20-mm-Platten um einen dunklen Kern: Gitter, Ansaugfeld des Netzteils, die Öffnungen für Netzteil, Lüfter und Anschlüsse sowie Gitter und Schacht der Blende sind Fenster auf diesen Kern; die Füße jedes Endes sind eine Querkufe. Entriegelung, Schutzgitter, Lochungen und alle einzelnen Bauteile sind im Kern zusammengefasst."),
        ("Thermal: the case sheet is one sleeve over both sides and the top, the bezel one face plate, grille, power supply faces and rear face single skins, the feet the two skids; no internals. Only the sum of the areas is compared at this scale.",
         "Thermisch: das Gehäuseblech ist eine Hülse über beide Seiten und das Deckblech, die Blende eine Frontplatte, Gitter, Netzteilflächen und Rückwand einzelne Häute, die Füße die beiden Kufen; keine Innenteile. In diesem Maßstab wird nur die Summe der Flächen verglichen."),
        ("Structure: floor, mainboard wall, two posts and the top frame in 20 mm, the two skids on the floor patch of the contact, and three masses: power supply, mainboard with cooler and the drives.",
         "Tragwerk: Boden, Mainboard-Wand, zwei Pfosten und Dachrahmen in 20 mm, die beiden Kufen auf dem Bodenfeld des Kontakts und drei Massen: Netzteil, Mainboard mit Kühler und die Laufwerke."),
    ],
    "500": [
        ("Body: a digital model only, the drawing is not printable at this scale: a steel block on a recessed rubber plinth, the dark bezel block in front with the intake as a window onto the case, and a rear plate with the recesses of power supply and fan. Minimal: four solids are the least that still tells front from rear.",
         "Körper: nur ein digitales Modell, die Zeichnung ist in diesem Maßstab nicht druckbar: ein Stahlblock auf einem zurückgesetzten Gummisockel, vorn der dunkle Blendenblock mit dem Einlass als Fenster auf das Gehäuse und eine Rückplatte mit den Nischen für Netzteil und Lüfter. Minimal: vier Körper sind das Wenigste, das Vorder- und Rückseite noch unterscheidet."),
        ("Thermal: sleeve, grille patch, bezel face, the two power supply faces, rear face, the plinth as the feet and the two outlets.",
         "Thermisch: Hülse, Gitterfeld, Blendenfläche, die beiden Netzteilflächen, Rückwand, der Sockel als Füße und die zwei Auslässe."),
        ("Structure: floor, mainboard wall, two posts and top frame in 30 mm, the plinth on the floor patch of the contact, and two masses: the power supply and everything else.",
         "Tragwerk: Boden, Mainboard-Wand, zwei Pfosten und Dachrahmen in 30 mm, der Sockel auf dem Bodenfeld des Kontakts und zwei Massen: das Netzteil und alles Übrige."),
    ],
}


def decisions(scale):
    """📝 The bilingual volume decisions of a scale."""
    return [{"en": en, "de": de} for en, de in DECISIONS[scale]]
