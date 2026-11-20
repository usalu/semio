"""🗿 Round 3 cast drinking fountain: the handcrafted body, thermal and structure lists of 1:50, 1:100, 1:200 and 1:500.

Every solid is stated here part by part from the kit's geometry; each scale is its own abstraction.

@see r3-drinking-fountain-kit.py
"""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-drinking-fountain-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

IRON, STEEL, BRASS, WATER = "cast-iron", "stainless-basin", "chrome-brass", "water"
FOUR = {"count": 4, "rotationAxis": [1, 0, 0], "rotationRad": K.r6(math.pi / 2), "origin": [0, 0, 0]}
TAPER = K.r6(K.SHAFT_R1 / K.SHAFT_R0)


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def bowl_loop(n, hole):
    """🥣 Closed 1.5 mm sheet loop of the bowl from the waste hole (or the axis) to the rim."""
    r0 = K.HOLE_R if hole else 0.0
    outer = K.bowl_curve(r0, K.BOWL_A, n)
    inner = K.bowl_curve(r0, K.BOWL_A - K.SHEET, n, True)
    return K.loop(outer + inner[::-1])


def jet_solid(n, **extra):
    """💧 The free jet at its true section along the parabola."""
    pts = K.jet_path(n)
    return K.tube("jet", "jet", WATER, "body", 0.5, [(x, y) for x, y, _ in pts], [r for _, _, r in pts], **extra)


def outlet_patch(scale):
    """🌫️ Emitter outlet (D2): the water surface in the mouth of the nozzle as the scale models that mouth (bore at 1:50 and 1:100, the end of the neck at 1:200, the end of the horn at 1:500)."""
    radius = {50: K.JET_R0, 100: 0.0075, 200: K.NECK_R, 500: 0.016}[scale]
    ox, oy = K.outlet()
    at = (ox - 0.004, oy + 0.004) if scale == 500 else (ox, oy)
    return K.revolve_about("th-jet-spray", "bubbler", WATER, "detail", 0.5, [[-0.002, 0.0], [-0.002, radius], [0.0005, radius], [0.0005, 0.0]], at, K.out_dir(), region="water-jet", layer="stream", tags=["jet-spray"])


def nozzle_start():
    """📍 World (X, Y) where the nozzle run ends and the tip ring begins."""
    return K.neck_path()[-1]


def support(feature, radius=0.2):
    """🟩 Fixed support at `base-fixed`: the bedding of the flange on the footing, one feature thick."""
    return K.revolve_up("support-base-fixed", "pedestal", IRON, "ground", 0.1, [[0.0, 0.0], [0.0, radius], [feature, radius], [feature, 0.0]], member="pedestal", part="support", anchor="base-fixed")


def mass(radius):
    """⚖️ The 62 kg of the castings at their centre of mass, 0.30 m up the axis."""
    return K.solid("mass-pedestal", "ellipsoid", "pedestal", IRON, "detail", 0.2, radii=[K.r6(radius * 1.4), radius, radius], transform={"translation": [0.3, 0.0, 0.0]}, member="pedestal", part="mass")


def body50():
    """🔬 1:50: every casting, moulding, flute and fixing."""
    b = []
    b.append(K.revolve_up("base", "pedestal", IRON, "body", 0.6, K.base_loop(4)))
    b.append(K.revolve_up("anchor-stud", "pedestal", IRON, "detail", 0.35, [[0.022, 0.0], [0.022, 0.006], [0.040, 0.006], [0.040, 0.0]], X=-K.BOLT_RC * math.cos(math.pi / 4), transform={"translation": [0.0, 0.0, K.r6(K.BOLT_RC * math.sin(math.pi / 4))]}, repeat=FOUR))
    b.append(K.extrude_up("anchor-nut", "pedestal", IRON, "detail", 0.3, K.circle(0.011, 6, K.BOLT_RC * math.cos(math.pi / 4), K.BOLT_RC * math.sin(math.pi / 4)), 0.022, 0.033, repeat=FOUR))
    b.append(K.extrude_up("door", "pedestal", IRON, "detail", 0.55, K.sector(K.DIE_R - 0.001, K.DIE_R + K.DOOR_PROUD, -90, K.DOOR_HALF, 8), K.DOOR_Y0, K.DOOR_Y1, texture=True))
    for i, (deg, y) in enumerate(((-90 - 22, 0.13), (-90 + 22, 0.13), (-90 - 22, 0.27), (-90 + 22, 0.27))):
        a = math.radians(deg)
        rr = K.DIE_R + K.DOOR_PROUD
        b.append(K.solid("door-screw-%d" % i, "ellipsoid", "pedestal", IRON, "texture", 0.3, radii=[0.005, 0.0025, 0.005], transform={"rotation": K.quat_u(deg), "translation": [y, K.r6(rr * math.cos(a)), K.r6(rr * math.sin(a))]}, texture=True))
    b.append(K.revolve_up("shaft-base", "pedestal", IRON, "body", 0.6, K.shaft_base_loop(3)))
    b.append(K.extrude_up("shaft", "pedestal", IRON, "body", 0.6, K.flute_ring(K.SHAFT_R0, K.FLUTE_D), K.SHAFT_Y0 - 0.001, K.SHAFT_Y1 + 0.001, holes=[K.circle(K.BORE_R0, 32)], taperTo=[1, TAPER]))
    b.append(K.revolve_up("astragal", "pedestal", IRON, "body", 0.6, K.astragal_loop(3)))
    b.append(K.revolve_up("capital", "pedestal", IRON, "body", 0.6, K.capital_loop(6)))
    b.append(K.revolve_about("button-boss", "pedestal", IRON, "body", 0.6, K.loop([[0.070, 0.0195], [0.070, 0.030], [0.094, 0.030], [0.096, 0.028], [0.096, 0.0195]]), (0.0, K.BTN_Y), (1, 0)))
    b.append(K.revolve_about("bezel", "pedestal", BRASS, "detail", 0.75, K.loop([[0.095, 0.0195], [0.095, 0.027], [0.101, 0.027], [0.104, 0.023], [0.104, 0.0195]]), (0.0, K.BTN_Y), (1, 0)))
    b.append(K.solid("button", "revolve", "button", BRASS, "body", 0.8, profile=K.open_profile([[-0.002, 0.0], [-0.002, K.BTN_R], [0.018, K.BTN_R], [0.0205, 0.017], [0.022, 0.011], [0.0225, 0.0]]), axis=[1, 0, 0], origin=[0.0, 0.0, 0.0]))
    b.append(K.revolve_up("basin", "basin", STEEL, "body", 0.5, bowl_loop(18, True)))
    b.append(K.revolve_up("rim-bead", "basin", STEEL, "body", 0.55, K.loop(K.arc(K.RIM_Y, K.BEAD_C, K.BEAD_R, 0, 360, 12)[:-1])))
    b.append(K.revolve_up("waste", "basin", STEEL, "detail", 0.4, [[0.765, 0.0], [0.765, 0.016], [0.769, 0.016], [0.769, 0.03], [0.7806, 0.03], [0.7806, 0.016], [0.7915, 0.016], [0.7915, 0.027], [0.7945, 0.027], [0.796, 0.024], [0.796, 0.0]]))
    holes = [K.circle(0.003, 8)] + [K.circle(0.003, 8, 0.012 * math.cos(math.radians(60 * k)), 0.012 * math.sin(math.radians(60 * k))) for k in range(6)]
    b.append(K.extrude_up("strainer", "basin", STEEL, "texture", 0.3, K.circle(0.021, 24), 0.796, 0.7975, holes=holes, texture=True))
    b.append(K.revolve_up("bubbler-base", "bubbler", BRASS, "body", 0.7, [[0.803, 0.0], [0.803, 0.017], [0.823, 0.017], [0.828, 0.0145], [0.828, 0.0]], X=K.BX))
    b.append(K.extrude_up("bubbler-nut", "bubbler", BRASS, "detail", 0.65, K.circle(0.0162, 6), 0.828, 0.838))
    b.append(K.tube("bubbler", "bubbler", BRASS, "body", 0.7, K.neck_path(3, 10), K.NECK_R))
    b.append(K.revolve_about("nozzle", "bubbler", BRASS, "detail", 0.8, K.loop([[-0.004, K.JET_R0], [-0.004, 0.0155], [K.TIP_L - 0.002, 0.0155], [K.TIP_L, 0.0135], [K.TIP_L, K.JET_R0]]), nozzle_start(), K.out_dir()))
    b.append(jet_solid(8))
    x1, y1, _ = K.jet_path(8)[-1]
    b.append(K.solid("drop-0", "ellipsoid", "jet", WATER, "detail", 0.5, radii=[0.005, 0.004, 0.004], transform={"translation": K.local("jet", x1 + 0.016, y1 + 0.017)}))
    b.append(K.solid("drop-1", "ellipsoid", "jet", WATER, "detail", 0.5, radii=[0.004, 0.004, 0.004], transform={"translation": K.local("jet", x1 - 0.013, y1 + 0.022, 0.009)}))
    return b


REGION = {
    "pedestal-cast": ("casting", ["base", "anchor-stud", "anchor-nut", "door", "door-screw-0", "door-screw-1", "door-screw-2", "door-screw-3", "shaft-base", "shaft", "astragal", "capital", "button-boss", "pedestal", "pedestal-foot"]),
    "basin-steel": ("sheet", ["basin", "rim-bead", "waste", "strainer"]),
    "bubbler-brass": ("tube", ["bezel", "button", "bubbler-base", "bubbler-nut", "bubbler", "nozzle"]),
    "water-jet": ("stream", ["jet", "drop-0", "drop-1"]),
}


def skin(scale):
    """🧥 The outer skins of the hollow castings as filled solids: the cavity of the bolted stack is closed, so only these faces exchange heat (C31; the product's outer area has no flood from outside)."""
    D = K.DIE_R
    cradle = [[K.cradle_y(r), r] for r in (K.CUP_R, 0.095, 0.065, 0.03, 0.0)]
    if scale == 50:
        cup = [[K.cradle_y(K.CUP_R + (K.CUP_HOLE - K.CUP_R) * i / 6), K.CUP_R + (K.CUP_HOLE - K.CUP_R) * i / 6] for i in range(7)]
        return {
            "base": K.revolve_up("th-base", "pedestal", IRON, "body", 0.6, [[0.0, 0.0]] + K.base_outer(4) + [[0.36, 0.0]]),
            "shaft-base": K.revolve_up("th-shaft-base", "pedestal", IRON, "body", 0.6, [[0.359, 0.0], [0.359, 0.112]] + K.shaft_base_loop(3)[4:-4] + [[K.SHAFT_Y0, 0.0]]),
            "shaft": K.extrude_up("th-shaft", "pedestal", IRON, "body", 0.6, K.flute_ring(K.SHAFT_R0, K.FLUTE_D), K.SHAFT_Y0 - 0.001, K.SHAFT_Y1 + 0.001, taperTo=[1, TAPER]),
            "astragal": K.revolve_up("th-astragal", "pedestal", IRON, "body", 0.6, [[K.SHAFT_Y1, 0.0]] + K.astragal_loop(3)[1:-2] + [[0.705, 0.0]]),
            "capital": K.revolve_up("th-capital", "pedestal", IRON, "body", 0.6, [[0.704, 0.0], [0.704, 0.082]] + K.capital_outer(6)[1:] + cup + [[K.cradle_y(K.CUP_HOLE), 0.0]]),
        }
    if scale == 100:
        return {
            "base": K.revolve_up("th-base", "pedestal", IRON, "body", 0.6, [[0.0, 0.0], [0.0, 0.2], [0.022, 0.2], [0.022, 0.166], [0.042, 0.166], [0.046, 0.148], [0.062, 0.130], [0.078, D], [0.322, D], [0.330, 0.126], [0.345, 0.138], [0.36, 0.138], [0.36, 0.0]]),
            "shaft": K.revolve_up("th-shaft", "pedestal", IRON, "body", 0.6, [[0.359, 0.0], [0.359, 0.114], [0.384, 0.114], [0.396, 0.089], [0.69, 0.077], [0.69, 0.088], [0.705, 0.088], [0.705, 0.0]]),
            "capital": K.revolve_up("th-capital", "pedestal", IRON, "body", 0.6, [[0.704, 0.0], [0.704, 0.082], [0.767, 0.082], [0.776, 0.086], [0.788, 0.097], [0.799, 0.111], [0.806, 0.122], [K.cradle_y(K.CUP_R), 0.122]] + cradle),
        }
    return {}


def thermal(body, scale):
    """🌡️ The thermal list: every surface region by region, by reference to its body solid or, for the hollow castings, as the filled outer skin, and the water surface in the nozzle mouth as the emitter outlet."""
    ids = [s["id"] for s in body]
    skins = skin(scale)
    out = []
    for region, (layer, members) in REGION.items():
        for sid in ids:
            if sid in members:
                out.append(K.tag(skins[sid], region=region, layer=layer) if sid in skins else K.ref("th-" + sid, sid, region=region, layer=layer))
    return out + [outlet_patch(scale)]


def wall_loop(scale):
    """🧮 The load-bearing wall of the pedestal as one tube: floor plate, die wall, the shaft at the root of the flutes, neck, cup."""
    root0, root1 = K.SHAFT_R0 - K.FLUTE_D, K.SHAFT_R1 - K.FLUTE_D
    bore1 = K.BORE_R0 * K.SHAFT_R1 / K.SHAFT_R0
    if scale == 50:
        outer = [[0.0075, 0.0], [0.0075, 0.2], [0.022, 0.2], [0.022, K.DIE_R], [0.36, K.DIE_R], [0.36, root0], [0.705, root1], [0.705, 0.082]] + K.quarter(0.767, 0.082, 0.806, 0.118, 3, "plumb") + [[K.cradle_y(K.CUP_R) - 0.001, K.CUP_R]]
        cup = [[K.cradle_y(r) - 0.001, r] for r in (0.09, 0.06, K.CUP_HOLE)]
        roof = [[K.bowl_y(r) - K.CUP_T - 0.0005, r] for r in (K.CUP_HOLE, 0.04, 0.056)]
        inner = [[0.705, 0.056], [0.705, bore1], [0.36, K.BORE_R0], [0.348, K.BORE_R0], [0.348, 0.106], [0.016, 0.106], [0.016, 0.0]]
        return K.loop(outer + cup + roof + inner)
    outer = [[0.015, 0.106], [0.015, 0.2], [0.03, 0.2], [0.03, K.DIE_R], [0.36, K.DIE_R], [0.36, root0], [0.705, root1], [0.705, 0.082], [0.767, 0.082], [0.788, 0.095], [0.806, 0.119]]
    inner = [[0.796, 0.09], [0.767, 0.07], [0.705, 0.056], [0.705, bore1], [0.36, K.BORE_R0], [0.348, K.BORE_R0], [0.348, 0.106]]
    return K.loop(outer + inner)


def structure50():
    """🏗️ 1:50 load path: wall tube of the stacked castings, tie rod with nuts, anchor studs, bowl shell clamped by the waste backnut, goose-neck on its shank."""
    s = [K.revolve_up("member-pedestal", "pedestal", IRON, "body", 0.4, wall_loop(50), member="pedestal", part="member")]
    s.append(K.revolve_up("tie-rod", "pedestal", IRON, "detail", 0.3, [[0.016, 0.0], [0.016, 0.008], [0.761, 0.008], [0.761, 0.0]], member="pedestal", part="member", slender=True))
    s.append(K.extrude_up("tie-nut-foot", "pedestal", IRON, "detail", 0.3, K.circle(0.0139, 6), 0.016, 0.029, member="pedestal", part="joint"))
    s.append(K.extrude_up("tie-nut-top", "pedestal", IRON, "detail", 0.3, K.circle(0.0139, 6), 0.745, 0.757, member="pedestal", part="joint"))
    s.append(K.revolve_up("tie-yoke", "pedestal", IRON, "detail", 0.3, [[0.735, 0.0], [0.735, 0.056], [0.745, 0.056], [0.745, 0.0]], member="pedestal", part="joint"))
    s.append(K.ref("joint-anchor-stud", "anchor-stud", member="pedestal", part="joint"))
    s.append(K.ref("joint-anchor-nut", "anchor-nut", member="pedestal", part="joint"))
    s.append(K.ref("member-basin", "basin", member="basin", part="member"))
    s.append(K.ref("member-rim-bead", "rim-bead", member="basin", part="member"))
    s.append(K.ref("joint-waste", "waste", member="basin", part="joint"))
    s.append(K.ref("member-bubbler", "bubbler", member="bubbler", part="member"))
    s.append(K.ref("joint-bubbler-base", "bubbler-base", member="bubbler", part="joint"))
    s.append(K.ref("joint-bubbler-nut", "bubbler-nut", member="bubbler", part="joint"))
    s.append(support(0.0075))
    s.append(mass(0.05))
    return s


def body100():
    """🔎 1:100: the three castings with their mouldings as steps, smooth shaft, bowl with a folded lip, neck on its base, button on its boss."""
    b = []
    base = [[0.0, 0.0], [0.0, 0.2], [0.022, 0.2], [0.022, 0.166], [0.042, 0.166], [0.046, 0.148], [0.062, 0.130], [0.078, DIE], [0.322, DIE], [0.330, 0.126], [0.345, 0.138], [0.36, 0.138], [0.36, 0.07], [0.348, 0.07], [0.348, 0.106], [0.016, 0.106], [0.016, 0.0]]
    b.append(K.revolve_up("base", "pedestal", IRON, "body", 0.6, K.loop(base)))
    b.append(K.extrude_up("door", "pedestal", IRON, "detail", 0.55, K.sector(K.DIE_R - 0.001, K.DIE_R + K.DOOR_PROUD, -90, K.DOOR_HALF, 5), K.DOOR_Y0, K.DOOR_Y1, texture=True))
    bore1 = K.BORE_R0 * K.SHAFT_R1 / K.SHAFT_R0
    shaft = [[0.352, 0.062], [0.352, 0.072], [0.359, 0.072], [0.359, 0.114], [0.384, 0.114], [0.396, 0.089], [0.69, 0.077], [0.69, 0.088], [0.705, 0.088], [0.705, 0.066], [0.69, bore1], [0.396, K.BORE_R0], [0.36, 0.062]]
    b.append(K.revolve_up("shaft", "pedestal", IRON, "body", 0.6, K.loop(shaft)))
    cap = [[0.699, 0.056], [0.699, 0.067], [0.704, 0.067], [0.704, 0.082], [0.767, 0.082], [0.776, 0.086], [0.788, 0.097], [0.799, 0.111], [0.806, 0.122], [K.cradle_y(K.CUP_R), 0.122]]
    cap += [[K.cradle_y(r), r] for r in (K.CUP_R, 0.095, 0.065, 0.03, 0.0)]
    cap += [[K.bowl_y(r) - K.CUP_T - 0.0005, r] for r in (0.0, 0.03, 0.056)]
    b.append(K.revolve_up("capital", "pedestal", IRON, "body", 0.6, K.loop(cap)))
    b.append(K.revolve_about("bezel", "pedestal", BRASS, "body", 0.7, [[0.07, 0.0], [0.07, 0.03], [0.096, 0.03], [0.104, 0.024], [0.104, 0.0]], (0.0, K.BTN_Y), (1, 0)))
    b.append(K.solid("button", "revolve", "button", BRASS, "body", 0.8, profile=K.open_profile([[0.0, 0.0], [0.0, K.BTN_R], [0.019, K.BTN_R], [0.0225, 0.012], [0.0225, 0.0]]), axis=[1, 0, 0], origin=[0.0, 0.0, 0.0]))
    outer = K.bowl_curve(0.0, K.BOWL_A, 10)
    inner = K.bowl_curve(0.0, K.BOWL_A - K.SHEET, 10, True)
    lip = [[0.8975, 0.19], [0.8975, 0.1985], [0.887, 0.1985], [0.887, 0.2], [0.897, 0.2], [0.899, 0.198], [0.899, 0.1885]]
    b.append(K.revolve_up("basin", "basin", STEEL, "body", 0.5, K.loop(outer[:-1] + lip + inner[::-1][1:])))
    b.append(K.revolve_up("bubbler-base", "bubbler", BRASS, "body", 0.7, [[0.803, 0.0], [0.803, 0.017], [0.826, 0.017], [0.838, 0.014], [0.838, 0.0]], X=K.BX))
    path = K.neck_path(2, 6)
    dx, dy = K.out_dir()
    path[-1] = (path[-1][0] + K.TIP_L * dx, path[-1][1] + K.TIP_L * dy)
    b.append(K.tube("bubbler", "bubbler", BRASS, "body", 0.7, path, [K.NECK_R] * (len(path) - 1) + [0.015]))
    b.append(jet_solid(4, texture=True))
    return b


DIE = K.DIE_R


def structure100():
    """🏗️ 1:100 load path: the wall tube, tie rod, bowl shell on the cup, the neck, the footing pad and the mass."""
    return [
        K.revolve_up("member-pedestal", "pedestal", IRON, "body", 0.4, wall_loop(100), member="pedestal", part="member"),
        K.revolve_up("tie-rod", "pedestal", IRON, "detail", 0.3, [[0.016, 0.0], [0.016, 0.008], [0.761, 0.008], [0.761, 0.0]], member="pedestal", part="member", slender=True),
        K.revolve_up("tie-yoke", "pedestal", IRON, "detail", 0.3, [[0.73, 0.0], [0.73, 0.056], [0.745, 0.056], [0.745, 0.0]], member="pedestal", part="joint"),
        K.ref("member-basin", "basin", member="basin", part="member"),
        K.revolve_up("joint-waste", "basin", STEEL, "detail", 0.3, [[0.765, 0.0], [0.765, 0.024], [0.78, 0.024], [0.78, 0.0]], member="basin", part="joint"),
        K.ref("member-bubbler", "bubbler", member="bubbler", part="member"),
        K.ref("joint-bubbler-base", "bubbler-base", member="bubbler", part="joint"),
        support(0.015),
        mass(0.045),
    ]


def body200():
    """🔭 1:200: one turned pedestal, the bowl, a plain neck hook, the button."""
    ped = [[0.0, 0.0], [0.0, 0.2], [0.03, 0.2], [0.03, 0.16], [0.075, DIE], [0.33, DIE], [0.33, 0.138], [0.36, 0.138], [0.36, 0.105], [0.395, 0.09], [0.705, 0.08], [0.767, 0.082], [0.806, 0.122], [K.cradle_y(K.CUP_R), 0.122], [K.cradle_y(0.07), 0.07], [K.cradle_y(0.0), 0.0]]
    outer = K.bowl_curve(0.0, K.BOWL_A, 7)
    inner = K.bowl_curve(0.0, K.BOWL_A - K.SHEET, 7, True)
    lip = [[K.RIM_Y + 0.006, 0.2], [K.RIM_Y + 0.006, 0.1885]]
    path = [(K.BX, 0.8), (K.BX, K.BEND_Y)] + [K.bend_point(a) for a in (150, 115, 80, 50)]
    dx, dy = K.out_dir()
    path.append((path[-1][0] + 0.032 * dx, path[-1][1] + 0.032 * dy))
    return [
        K.revolve_up("pedestal", "pedestal", IRON, "body", 0.6, ped),
        K.revolve_about("button", "button", BRASS, "body", 0.8, [[-0.02, 0.0], [-0.02, 0.027], [0.0, 0.027], [0.0, K.BTN_R], [0.0225, K.BTN_R], [0.0225, 0.0]], (K.BTN_X, K.BTN_Y), (1, 0)),
        K.revolve_up("basin", "basin", STEEL, "body", 0.5, K.loop(outer + lip + inner[::-1])),
        K.tube("bubbler", "bubbler", BRASS, "body", 0.7, path, K.NECK_R),
        jet_solid(3, texture=True),
    ]


def structure200():
    """🏗️ 1:200 load path: the pedestal as a stepped column of its wall radii, the bowl, the neck, pad and mass."""
    col = [[0.03, 0.0], [0.03, DIE], [0.36, DIE], [0.36, 0.086], [0.705, 0.074], [0.705, 0.082], [0.767, 0.082], [0.8, 0.11], [0.8, 0.0]]
    return [
        K.revolve_up("member-pedestal", "pedestal", IRON, "body", 0.4, col, member="pedestal", part="member"),
        K.ref("member-basin", "basin", member="basin", part="member"),
        K.ref("member-bubbler", "bubbler", member="bubbler", part="member"),
        support(0.03),
        mass(0.04),
    ]


def body500():
    """🛰️ 1:500: the sign of the fountain: flared foot, die, shaft, capital with the bowl, a horn for the neck."""
    shaft = [[0.0, 0.0], [0.0, 0.2], [0.075, DIE], [0.36, DIE], [0.36, 0.09], [0.72, 0.08], [0.8, 0.122], [0.813, 0.122], [0.806, 0.1], [0.795, 0.07], [0.788, 0.0]]
    outer = K.bowl_curve(0.0, K.BOWL_A, 4)
    inner = K.bowl_curve(0.0, K.BOWL_A - K.SHEET, 4, True)
    lip = [[K.RIM_Y + 0.006, 0.2], [K.RIM_Y + 0.006, 0.1885]]
    horn = [(K.BX, 0.816), (K.BX, 0.925), (-0.083, 0.966), (-0.048, 0.978), (K.outlet()[0] - 0.004, K.outlet()[1] + 0.004)]
    return [
        K.revolve_up("pedestal", "pedestal", IRON, "body", 0.6, shaft),
        K.revolve_about("button", "button", BRASS, "body", 0.8, [[-0.02, 0.0], [-0.02, 0.0225], [0.0225, 0.0225], [0.0225, 0.0]], (K.BTN_X, K.BTN_Y), (1, 0)),
        K.revolve_up("basin", "basin", STEEL, "body", 0.5, K.loop(outer + lip + inner[::-1])),
        K.tube("bubbler", "bubbler", BRASS, "body", 0.7, horn, [0.027, 0.024, 0.02, 0.018, 0.016]),
        jet_solid(2, texture=True),
    ]


def structure500():
    """🏗️ 1:500 load path: a column of the wall radii on its pad, the bowl and the neck."""
    col = [[0.075, 0.0], [0.075, DIE], [0.36, DIE], [0.36, 0.086], [0.72, 0.076], [0.8, 0.11], [0.8, 0.0]]
    return [
        K.revolve_up("member-pedestal", "pedestal", IRON, "body", 0.4, col, member="pedestal", part="member"),
        K.ref("member-basin", "basin", member="basin", part="member"),
        K.ref("member-bubbler", "bubbler", member="bubbler", part="member"),
        K.revolve_up("support-base-fixed", "pedestal", IRON, "ground", 0.1, [[0.0, 0.0], [0.0, 0.2], [0.075, DIE], [0.075, 0.0]], member="pedestal", part="support", anchor="base-fixed"),
        mass(0.0375),
    ]


DECISIONS = {
    50: [
        text("Body as built: three stacked castings of 12 mm wall on one axis (base with flange, torus, scotia, die and cornice; shaft with base torus, sixteen flutes 6 mm deep between flat arrises tapering from 184 to 160 mm, and astragal; capital with neck band, cavetto bell and the cup that cradles the bowl), four M12 anchor studs with hexagon nuts on a 362 mm bolt circle, the access door with four screws on the right side of the die, the push button sliding in a cast boss with a chrome bezel, the spun 1.5 mm bowl of 0.40 m with rolled rim bead and waste fitting with strainer, the 28 mm goose-neck on its base with union nut and nozzle tip, and the jet as the parabola of 3 L/min leaving the 12 mm mouth at 0.44 m/s and thinning to 6 mm at the bowl, with two splash drops.",
             "Körper wie gebaut: drei gestapelte Gussteile mit 12 mm Wand auf einer Achse (Fuß mit Flansch, Wulst, Kehle, Sockelkörper und Gesims; Schaft mit Basiswulst, sechzehn 6 mm tiefen Kannelüren zwischen flachen Stegen, von 184 auf 160 mm verjüngt, und Halsring; Kapitell mit Halsband, Kehlglocke und der Schale, die das Becken trägt), vier M12-Ankerbolzen mit Sechskantmuttern auf einem Lochkreis von 362 mm, die Revisionstür mit vier Schrauben an der rechten Seite des Sockelkörpers, der Drucktaster in einer Gussnabe mit Chromblende, das gedrückte 1,5-mm-Becken von 0,40 m mit Rollrand und Ablaufventil mit Sieb, der 28-mm-Schwanenhals auf seinem Fuß mit Überwurfmutter und Düsenspitze sowie der Strahl als Parabel von 3 L/min, die den 12-mm-Mund mit 0,44 m/s verlässt und bis zum Becken auf 6 mm ausdünnt, mit zwei Spritztropfen."),
        text("Thermal by reference to the body surfaces, region by region; the castings are closed hollow shells, so only their outer faces exchange heat (C31) and their thermal solids are the filled outer skins; the emitter outlet is the wet mouth of the nozzle (D2). Supply pipe and drain inside the pedestal belong to no thermal region and are not modelled.",
             "Thermik durch Verweis auf die Körperflächen, Region für Region; die Gussteile sind geschlossene Hohlkörper, also tauschen nur ihre Außenflächen Wärme (C31), und ihre Thermalkörper sind die gefüllten Außenhäute; der Emitterauslass ist der nasse Düsenmund (D2). Zulauf und Ablauf im Sockel gehören zu keiner Thermalregion und sind nicht modelliert."),
        text("Structure as the load path: the wall tube of the castings at the root of the flutes, clamped by a 16 mm tie rod between the floor nut and the yoke under the cup, four anchor studs with nuts into the footing, the bowl shell held by the waste fitting and its backnut, the goose-neck on base and union nut, the fixed bedding of the flange and the mass of the castings at 0.30 m.",
             "Tragwerk als Lastpfad: das Wandrohr der Gussteile am Kannelürengrund, verspannt durch einen 16-mm-Zuganker zwischen Bodenmutter und Joch unter der Schale, vier Ankerbolzen mit Muttern ins Fundament, die Beckenschale gehalten vom Ablaufventil mit Gegenmutter, der Schwanenhals auf Fuß und Überwurfmutter, die eingespannte Auflage des Flansches und die Masse der Gussteile auf 0,30 m."),
    ],
    100: [
        text("Own 1:100 model: the three castings with their mouldings as plain steps and chamfers, the shaft as a smooth cone at the mean of arris and flute, the door as a plate, the bowl with a folded lip in place of the bead and without the waste, the neck as one tube with a swollen tip on a plain base, boss and bezel as one chrome boss with the button, and the jet at its true section; anchor bolts, screws, flutes, union nut, strainer and drops are below 15 mm and left out.",
             "Eigenes 1:100-Modell: die drei Gussteile mit Profilen als einfache Stufen und Fasen, der Schaft als glatter Kegel im Mittel von Steg und Kannelüre, die Tür als Platte, das Becken mit Falzrand statt Wulst und ohne Ablauf, der Hals als ein Rohr mit verdickter Spitze auf einfachem Fuß, Nabe und Blende als eine Chromnabe mit dem Taster und der Strahl im wahren Querschnitt; Ankerbolzen, Schrauben, Kannelüren, Überwurfmutter, Sieb und Tropfen liegen unter 15 mm und entfallen."),
        text("Thermal by reference, the castings as filled outer skins; the jet keeps its true 12 to 6 mm section (flagged as texture so that it is not thickened) and the emitter outlet is the nozzle mouth.",
             "Thermik durch Verweis, die Gussteile als gefüllte Außenhäute; der Strahl behält seinen wahren Querschnitt von 12 bis 6 mm (als Textur markiert, damit er nicht verdickt wird), der Emitterauslass ist der Düsenmund."),
        text("Structure: wall tube with its flange ring, tie rod and yoke, bowl shell on a waste clamp, neck on its base, bedding pad and mass.",
             "Tragwerk: Wandrohr mit Flanschring, Zuganker und Joch, Beckenschale auf der Ablaufklemmung, Hals auf seinem Fuß, Auflagerplatte und Masse."),
    ],
    200: [
        text("Own 1:200 model: the pedestal as one turned solid (flange, splayed foot, die, cornice step, tapering shaft, bell), the bowl as its sheet with a plain lip, the neck as a plain 28 mm hook from the bowl floor to the mouth, the button with its boss as one stepped stud and the jet at its true section; cavities, door, mouldings and the tip ring are merged.",
             "Eigenes 1:200-Modell: der Sockel als ein gedrehter Vollkörper (Flansch, ausgestellter Fuß, Sockelkörper, Gesimsstufe, verjüngter Schaft, Glocke), das Becken als Blech mit einfachem Rand, der Hals als einfacher 28-mm-Haken vom Beckenboden bis zum Mund, der Taster mit Nabe als ein gestufter Zapfen und der Strahl im wahren Querschnitt; Hohlräume, Tür, Profile und Düsenring sind zusammengefasst."),
        text("Thermal by reference; the solid pedestal shows the same outer surface as the hollow castings.",
             "Thermik durch Verweis; der volle Sockel zeigt dieselbe Außenfläche wie die hohlen Gussteile."),
        text("Structure: a stepped column of the wall radii, the bowl, the neck, the bedding pad and the mass.",
             "Tragwerk: eine gestufte Säule der Wandradien, das Becken, der Hals, die Auflagerplatte und die Masse."),
    ],
    500: [
        text("Own 1:500 sign: a splayed foot with the die, the shaft flaring into the capital, the bowl as a four-step dish, the neck as a horn of 54 to 32 mm that keeps the hook, the button as a 45 mm stud and the jet at its true section; nothing under 75 mm is kept as a separate part. The model is minimal.",
             "Eigenes 1:500-Zeichen: ein ausgestellter Fuß mit Sockelkörper, der Schaft, der ins Kapitell ausläuft, das Becken als vierstufige Schale, der Hals als Horn von 54 bis 32 mm, das den Haken behält, der Taster als 45-mm-Zapfen und der Strahl im wahren Querschnitt; nichts unter 75 mm bleibt als eigenes Teil. Das Modell ist minimal."),
        text("Thermal by reference with the nozzle mouth as the outlet.", "Thermik durch Verweis mit dem Düsenmund als Auslass."),
        text("Structure: one column of the wall radii on the splayed bedding with the bowl, the neck and the mass.", "Tragwerk: eine Säule der Wandradien auf der ausgestellten Auflage mit Becken, Hals und Masse."),
    ],
}


def volumes():
    """📦 The four authored scale volumes."""
    out = {}
    for scale, body, structure in ((50, body50(), structure50()), (100, body100(), structure100()), (200, body200(), structure200()), (500, body500(), structure500())):
        out[str(scale)] = {"status": "authored", "denominator": scale, "body": body, "thermal": thermal(body, scale), "structure": structure, "decisions": DECISIONS[scale]}
    return out
