"""🫖 Electric jug kettle, rebuilt by hand: turned stainless jug on its power base with cord, a short pressed pouring beak instead of a teapot spout, a lid on a real hinge at the handle with its release button, a C handle swept along its centre line, a rocker switch on its own pivot, water windows on both sides; rig, body, thermal, structure and the 2D drawings of every scale."""
import copy

from rework_16a_lib import FACES, box, revolve, capsule, xloft, wloft, sweep, band, edges, moved, ellipsoid, of, shape, poly, rect, ellipse, run

SLUG = "electric-kettle"
BODY_Y = 0.030
HANDLE = (0.072, 0.120)
HINGE = (0.066, 0.209)
LID_C = 0.066
SWITCH = (0.1255, 0.098)
Y = [0, 1, 0]
BASE = [[0.0, 0.0], [0.0, 0.0855], [0.0015, 0.0885], [0.004, 0.09], [0.021, 0.09], [0.0255, 0.0889], [0.0285, 0.0858], [0.03, 0.0815], [0.03, 0.0785], [0.0272, 0.0775], [0.026, 0.0762], [0.026, 0.0]]
SKIRT = [[-0.004, 0.068], [-0.004, 0.0737], [-0.0025, 0.0752], [0.0145, 0.0752], [0.016, 0.0746], [0.016, 0.0739], [0.0085, 0.0739], [0.0085, 0.068], [-0.004, 0.068]]
SHELL = [[0.0105, 0.0], [0.0105, 0.074], [0.166, 0.06956], [0.1712, 0.0688], [0.1736, 0.0668], [0.1741, 0.0625], [0.175, 0.0625], [0.175, 0.0672], [0.1732, 0.069], [0.1705, 0.0699], [0.167, 0.07023], [0.0085, 0.07476], [0.0085, 0.0]]
ENVELOPE = [[-0.004, 0.0], [-0.004, 0.0737], [-0.0025, 0.0752], [0.0145, 0.0752], [0.016, 0.0746], [0.167, 0.07023], [0.1705, 0.0699], [0.1732, 0.069], [0.175, 0.0672], [0.175, 0.0]]
WATER = [[0.0005, 0.0], [0.0005, 0.0739], [0.05999, 0.0722], [0.05999, 0.0]]
BEAK = {
    "50": [(-0.050, 0.148, 0.030, 0.034), (-0.066, 0.157, 0.022, 0.027), (-0.080, 0.1645, 0.014, 0.019), (-0.090, 0.170, 0.008, 0.012), (-0.095, 0.1725, 0.004, 0.007)],
    "100": [(-0.050, 0.148, 0.030, 0.034), (-0.074, 0.161, 0.018, 0.023), (-0.094, 0.172, 0.0075, 0.0085)],
    "200": [(-0.046, 0.146, 0.032, 0.036), (-0.092, 0.171, 0.015, 0.015)],
    "500": [(-0.040, 0.140, 0.0375, 0.0375), (-0.092, 0.168, 0.0375, 0.0375)],
}
GRIP = {
    "50": [(0.068, 0.207, 0.0115, 0.019), (0.082, 0.2075, 0.0115, 0.019), (0.096, 0.207, 0.011, 0.017), (0.109, 0.202, 0.0105, 0.0155), (0.117, 0.191, 0.010, 0.0145), (0.120, 0.175, 0.010, 0.014), (0.120, 0.140, 0.0105, 0.0145), (0.119, 0.105, 0.010, 0.014), (0.115, 0.080, 0.010, 0.014), (0.106, 0.064, 0.010, 0.0145), (0.092, 0.055, 0.0105, 0.015), (0.070, 0.052, 0.011, 0.016)],
    "100": [(0.068, 0.207, 0.0115, 0.019), (0.097, 0.207, 0.011, 0.017), (0.116, 0.193, 0.010, 0.0145), (0.120, 0.140, 0.0105, 0.0145), (0.116, 0.082, 0.010, 0.014), (0.100, 0.058, 0.0105, 0.015), (0.070, 0.052, 0.011, 0.016)],
    "200": [(0.068, 0.203, 0.015, 0.015), (0.097, 0.203, 0.015, 0.015), (0.110, 0.197, 0.015, 0.015), (0.116, 0.183, 0.015, 0.015), (0.116, 0.090, 0.015, 0.015), (0.110, 0.071, 0.015, 0.015), (0.096, 0.061, 0.015, 0.015), (0.070, 0.058, 0.015, 0.015)],
    "500": [(0.068, 0.190, 0.015, 0.02), (0.108, 0.185, 0.015, 0.02), (0.108, 0.080, 0.015, 0.02), (0.068, 0.072, 0.015, 0.02)],
}
LID = {
    "50": [[0.012, 0.0], [0.012, 0.0605], [0.0095, 0.063], [0.004, 0.063], [0.004, 0.0652], [0.0028, 0.066], [0.0005, 0.066], [-0.0012, 0.0642], [-0.0035, 0.057], [-0.0062, 0.045], [-0.0084, 0.031], [-0.0096, 0.018], [-0.0101, 0.008], [-0.0102, 0.0]],
    "100": [[0.012, 0.0], [0.012, 0.062], [0.004, 0.062], [0.004, 0.066], [0.0005, 0.066], [-0.0035, 0.057], [-0.0084, 0.031], [-0.0102, 0.0]],
    "200": [[0.009, 0.0], [0.009, 0.060], [0.004, 0.060], [0.004, 0.066], [-0.004, 0.064], [-0.021, 0.03], [-0.021, 0.0]],
    "500": [[0.030, 0.0], [0.030, 0.058], [0.004, 0.058], [0.004, 0.066], [-0.0102, 0.02], [-0.0102, 0.0]],
}
LID_SKIN = [[0.004, 0.0], [0.004, 0.0652], [0.0028, 0.066], [0.0005, 0.066], [-0.0012, 0.0642], [-0.0035, 0.057], [-0.0062, 0.045], [-0.0084, 0.031], [-0.0096, 0.018], [-0.0101, 0.008], [-0.0102, 0.0]]
PADDLE = [(0.0, 0.004, 0.007), (0.006, 0.00375, 0.008), (0.020, 0.00375, 0.009), (0.022, 0.003, 0.008)]
KNUCKLE = [(-0.005, 0.005, 0.016), (0.004, 0.005, 0.016), (0.016, 0.004, 0.014)]


def in_handle(stations):
    """🤏 World stations of the grip in the handle bone's frame."""
    return moved(stations, -HANDLE[0], -HANDLE[1])


def lid(scale, **more):
    """🎩 The lid turned about its own centre in the frame of the lid bone (u from the hinge towards the beak, v downwards)."""
    return revolve("lid", "lid", "stainless", LID[scale], Y, [LID_C, 0.0, 0.0], tone=0.6, **more)


def cord():
    return [
        revolve("cord-sleeve", "base", "pp-plastic", [[0.086, 0.0], [0.086, 0.006], [0.097, 0.006], [0.103, 0.0045], [0.103, 0.0]], [1, 0, 0], [0.0, 0.010, 0.0], role="detail", tone=0.2),
        capsule("cord-drop", "base", "pp-plastic", [0.100, 0.010, 0.0], [0.111, 0.00375, 0.0], 0.00375, role="detail", tone=0.2),
        capsule("cord-run", "base", "pp-plastic", [0.111, 0.00375, 0.0], [0.127, 0.00375, 0.0], 0.00375, role="detail", tone=0.2),
    ]


def gauge(name, w):
    """🪟 Water window: a proud rounded glazing strip on the wall."""
    turn = [-0.707107, 0.0, 0.0, 0.707107]
    rows = [(0.02, 0.07343, 0.0075), (0.0235, 0.07333, 0.0125), (0.0295, 0.07316, 0.014), (0.1305, 0.07027, 0.014), (0.1365, 0.0701, 0.0125), (0.14, 0.07, 0.0075)]
    sections = [{"at": [0.0, y, w * r], "rx": half, "ry": 0.0022, "exponent": 5, "rotation": turn} for y, r, half in rows]
    return {"id": f"gauge-{name}", "kind": "loft", "bone": "body", "material": "water", "role": "glazing", "tone": 0.8, "sections": sections, "capStart": True, "capEnd": True}


def release():
    return revolve("lid-release", "handle", "pp-plastic", [[0.091, 0.0], [0.091, 0.008], [0.1018, 0.008], [0.103, 0.0068], [0.103, 0.0]], Y, [0.022, 0.0, 0.0], role="detail", tone=0.65)


def steam(scale):
    """♨️ Outlet patches at the beak: the steam over its mouth and the plume as a sleeve over its end."""
    s = BEAK[scale]
    tip, before = s[-1], s[-2]
    du, dv = tip[0] - before[0], tip[1] - before[1]
    n = (du * du + dv * dv) ** 0.5
    du, dv = du / n, dv / n
    mouth = [(tip[0] - 0.0012 * du, tip[1] - 0.0012 * dv, tip[2] + 0.0012, tip[3] + 0.0012), (tip[0] + 0.001 * du, tip[1] + 0.001 * dv, tip[2] + 0.0012, tip[3] + 0.0012)]
    sleeve = [(a, b, c + 0.0012, d + 0.0012) for a, b, c, d in s[-3:]] if len(s) > 2 else [(a, b, c + 0.0012, d + 0.0012) for a, b, c, d in s]
    sleeve.append((tip[0] + 0.0014 * du, tip[1] + 0.0014 * dv, tip[2] + 0.0016, tip[3] + 0.0016))
    a = sweep("th-steam", "body", "water", mouth, 2.0, tone=0.45, tags=["steam"], region="water", layer="water-skin")
    b = sweep("th-steam-plume", "body", "stainless", sleeve, 2.0, tone=0.55, tags=["steam-plume"], region="shell", layer="shell-skin")
    a["capStart"] = b["capStart"] = False
    return [a, b]


def body_50():
    return [
        revolve("base", "base", "pp-plastic", BASE, Y, [0.0, 0.0, 0.0], tone=0.2),
        *cord(),
        revolve("skirt", "body", "pp-plastic", SKIRT, Y, [0.0, 0.0, 0.0], tone=0.2),
        revolve("shell", "body", "stainless", SHELL, Y, [0.0, 0.0, 0.0], tone=0.55),
        sweep("spout", "body", "stainless", BEAK["50"], 2.4, tone=0.55),
        gauge("right", -1),
        gauge("left", 1),
        revolve("water", "water", "water", WATER, Y, [0.0, 0.0, 0.0], tone=0.45),
        sweep("handle", "handle", "pp-plastic", in_handle(GRIP["50"]), 2.3, tone=0.2),
        release(),
        xloft("switch", "switch", "pp-plastic", PADDLE, 3, role="detail", tone=0.7),
        lid("50"),
        xloft("lid-hinge", "lid", "pp-plastic", KNUCKLE, 3, tone=0.2),
    ]


def thermal_50():
    return [
        of("th-base", "base", region="plate", layer="plate-skin"),
        of("th-cord-sleeve", "cord-sleeve", region="plate", layer="plate-skin"),
        of("th-cord-drop", "cord-drop", region="plate", layer="plate-skin"),
        of("th-cord-run", "cord-run", region="plate", layer="plate-skin"),
        revolve("th-element", "body", "stainless", [[0.002, 0.0], [0.002, 0.062], [0.0085, 0.062], [0.0085, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, sectionFaces=["+v", "-v"], region="plate", layer="plate-skin"),
        revolve("th-shell", "body", "stainless", ENVELOPE, Y, [0.0, 0.0, 0.0], tone=0.55, sectionFaces=["+v"], region="shell", layer="shell-skin"),
        of("th-spout", "spout", region="shell", layer="shell-skin"),
        of("th-water", "water", region="water", layer="water-skin"),
        of("th-gauge-right", "gauge-right", region="water", layer="water-skin"),
        of("th-gauge-left", "gauge-left", region="water", layer="water-skin"),
        revolve("th-lid", "lid", "stainless", LID_SKIN, Y, [LID_C, 0.0, 0.0], tone=0.6, sectionFaces=["+v"], region="lid", layer="lid-skin"),
        of("th-lid-hinge", "lid-hinge", region="lid", layer="lid-skin"),
        of("th-handle", "handle", region="handle", layer="handle-skin"),
        of("th-lid-release", "lid-release", region="handle", layer="handle-skin"),
        of("th-switch", "switch", region="handle", layer="handle-skin"),
        *steam("50"),
    ]


def core(scale, factor):
    """🦴 Load-bearing core of the handle."""
    return sweep("member-handle", "handle", "pp-plastic", [(u, v, hp * factor, hw * factor) for u, v, hp, hw in in_handle(GRIP[scale])], 2.0, role="detail", tone=0.3, member="handle", part="member")


def handle_joints():
    top, bottom = in_handle([(0.060, 0.207), (0.076, 0.207)]), in_handle([(0.062, 0.052), (0.078, 0.052)])
    return [
        capsule("joint-handle-top", "handle", "stainless", [*top[0], 0.0], [*top[1], 0.0], 0.0045, role="detail", tone=0.3, member="handle", part="joint"),
        capsule("joint-handle-bottom", "handle", "stainless", [*bottom[0], 0.0], [*bottom[1], 0.0], 0.0045, role="detail", tone=0.3, member="handle", part="joint"),
    ]


def structure_50():
    return [
        revolve("member-base-wall", "base", "stainless", [[0.0, 0.081], [0.0, 0.0885], [0.026, 0.0885], [0.026, 0.081], [0.0, 0.081]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="member"),
        revolve("member-base-deck", "base", "stainless", [[0.0185, 0.016], [0.0185, 0.0815], [0.026, 0.0815], [0.026, 0.016], [0.0185, 0.016]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="member"),
        box("member-base-rib", "base", "pp-plastic", [0.064, 0.0075, 0.0075], [0.049, 0.0148, 0.0], role="detail", tone=0.3, repeat={"count": 4, "rotationAxis": [0, 1, 0], "rotationRad": 1.570796}, member="base", part="member"),
        revolve("joint-base", "body", "stainless", [[-0.0115, 0.0], [-0.0115, 0.016], [0.009, 0.016], [0.009, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="body", part="joint"),
        of("member-body-skirt", "skirt", member="body", part="member"),
        revolve("member-body-wall", "body", "stainless", [[0.0085, 0.0], [0.0085, 0.07476], [0.168, 0.0702], [0.168, 0.0695], [0.0105, 0.074], [0.0105, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="body", part="member"),
        of("member-spout", "spout", member="body", part="member"),
        capsule("joint-lid", "lid", "stainless", [0.0, 0.0, -0.017], [0.0, 0.0, 0.017], 0.00375, role="detail", tone=0.3, member="lid", part="joint"),
        of("member-lid-hinge", "lid-hinge", member="lid", part="member"),
        of("member-lid", "lid", member="lid", part="member"),
        core("50", 0.55),
        *handle_joints(),
        of("member-switch", "switch", member="switch", part="member"),
        ellipsoid("joint-switch", "switch", "stainless", [0.004, 0.004, 0.008], [0.0, 0.0, 0.0], tone=0.3, member="switch", part="joint"),
        revolve("member-water", "water", "water", [[0.0005, 0.0], [0.0005, 0.0725], [0.008, 0.0725], [0.008, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.45, member="water", part="member"),
        ellipsoid("mass-base", "base", "stainless", [0.03, 0.0055, 0.03], [0.0, 0.009, 0.0], tone=0.15, member="base", part="mass"),
        ellipsoid("mass-body", "body", "stainless", [0.014, 0.014, 0.014], [0.0, 0.09, 0.0], tone=0.15, member="body", part="mass"),
        ellipsoid("mass-water", "water", "stainless", [0.032, 0.016, 0.032], [0.0, 0.03024, 0.0], tone=0.15, member="water", part="mass"),
        revolve("support-base-contact", "base", "stainless", [[0.0, 0.0785], [0.0, 0.0855], [0.0075, 0.0855], [0.0075, 0.0785], [0.0, 0.0785]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="support", anchor="base-contact", tags=["base-contact"]),
    ]


def body_100():
    return [
        revolve("base", "base", "pp-plastic", [[0.0, 0.0], [0.0, 0.086], [0.004, 0.09], [0.025, 0.09], [0.03, 0.085], [0.03, 0.0]], Y, [0.0, 0.0, 0.0], tone=0.2),
        cord()[0],
        revolve("shell", "body", "stainless", [[0.0, 0.0], [0.0, 0.0752], [0.016, 0.0746], [0.170, 0.0701], [0.175, 0.0672], [0.175, 0.0625], [0.172, 0.0625], [0.172, 0.066], [0.0105, 0.0705], [0.0105, 0.0]], Y, [0.0, 0.0, 0.0], tone=0.55),
        sweep("spout", "body", "stainless", BEAK["100"], 2.4, tone=0.55),
        revolve("water", "water", "water", [[0.0005, 0.0], [0.0005, 0.073], [0.06, 0.0715], [0.06, 0.0]], Y, [0.0, 0.0, 0.0], tone=0.45),
        sweep("handle", "handle", "pp-plastic", in_handle(GRIP["100"]), 2.3, tone=0.2),
        xloft("switch", "switch", "pp-plastic", PADDLE, 3, role="detail", tone=0.7),
        lid("100"),
        xloft("lid-hinge", "lid", "pp-plastic", KNUCKLE, 3, tone=0.2),
    ]


def thermal_100():
    return [
        of("th-base", "base", region="plate", layer="plate-skin"),
        of("th-cord-sleeve", "cord-sleeve", region="plate", layer="plate-skin"),
        revolve("th-element", "body", "stainless", [[-0.006, 0.0], [-0.006, 0.062], [0.009, 0.062], [0.009, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, sectionFaces=["+v", "-v"], region="plate", layer="plate-skin"),
        revolve("th-shell", "body", "stainless", [[0.0, 0.0], [0.0, 0.0752], [0.016, 0.0746], [0.170, 0.0701], [0.175, 0.0672], [0.175, 0.0]], Y, [0.0, 0.0, 0.0], tone=0.55, sectionFaces=["+v"], region="shell", layer="shell-skin"),
        of("th-spout", "spout", region="shell", layer="shell-skin"),
        of("th-water", "water", region="water", layer="water-skin"),
        revolve("th-lid", "lid", "stainless", [[0.004, 0.0], [0.004, 0.066], [0.0005, 0.066], [-0.0035, 0.057], [-0.0084, 0.031], [-0.0102, 0.0]], Y, [LID_C, 0.0, 0.0], tone=0.6, sectionFaces=["+v"], region="lid", layer="lid-skin"),
        of("th-lid-hinge", "lid-hinge", region="lid", layer="lid-skin"),
        of("th-handle", "handle", region="handle", layer="handle-skin"),
        of("th-switch", "switch", region="handle", layer="handle-skin"),
        *steam("100"),
    ]


def structure_100():
    return [
        revolve("member-base", "base", "pp-plastic", [[0.03, 0.0], [0.03, 0.085], [0.025, 0.09], [0.004, 0.09], [0.0, 0.086], [0.0, 0.071], [0.015, 0.071], [0.015, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="member"),
        revolve("joint-base", "body", "stainless", [[-0.015, 0.0], [-0.015, 0.016], [0.0, 0.016], [0.0, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="body", part="joint"),
        revolve("member-body", "body", "stainless", [[0.015, 0.0], [0.015, 0.060], [0.175, 0.0572], [0.175, 0.0672], [0.170, 0.0701], [0.0, 0.0752], [0.0, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="body", part="member"),
        of("member-spout", "spout", member="body", part="member"),
        capsule("joint-lid", "lid", "stainless", [0.0, 0.0, -0.017], [0.0, 0.0, 0.017], 0.0075, role="detail", tone=0.3, member="lid", part="joint"),
        of("member-lid", "lid", member="lid", part="member"),
        core("100", 0.75),
        capsule("joint-handle", "handle", "stainless", [-0.012, 0.087, 0.0], [0.004, 0.087, 0.0], 0.0075, role="detail", tone=0.3, member="handle", part="joint"),
        of("member-switch", "switch", member="switch", part="member"),
        revolve("member-water", "water", "water", [[0.0005, 0.0], [0.0005, 0.072], [0.0155, 0.072], [0.0155, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.45, member="water", part="member"),
        ellipsoid("mass-base", "base", "stainless", [0.03, 0.0075, 0.03], [0.0, 0.0095, 0.0], tone=0.15, member="base", part="mass"),
        ellipsoid("mass-water", "water", "stainless", [0.032, 0.016, 0.032], [0.0, 0.03024, 0.0], tone=0.15, member="water", part="mass"),
        revolve("support-base-contact", "base", "stainless", [[0.0, 0.071], [0.0, 0.086], [0.015, 0.086], [0.015, 0.071], [0.0, 0.071]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="support", anchor="base-contact", tags=["base-contact"]),
    ]


JUG_200 = [[0.0, 0.0], [0.0, 0.0752], [0.016, 0.0746], [0.170, 0.0701], [0.175, 0.067], [0.175, 0.0]]
BASE_200 = [[0.0, 0.0], [0.0, 0.086], [0.004, 0.09], [0.026, 0.09], [0.03, 0.086], [0.03, 0.0]]


def coarse(scale, jug, base_profile):
    body = [
        revolve("base", "base", "pp-plastic", base_profile, Y, [0.0, 0.0, 0.0], tone=0.2),
        revolve("jug", "body", "stainless", jug, Y, [0.0, 0.0, 0.0], tone=0.55),
        sweep("spout", "body", "stainless", BEAK[scale], 2.4, tone=0.55),
        sweep("handle", "handle", "pp-plastic", in_handle(GRIP[scale]), 2.3, tone=0.2),
        lid(scale),
    ]
    thermal = [
        of("th-base", "base", region="plate", layer="plate-skin"),
        of("th-jug", "jug", region="shell", layer="shell-skin"),
        of("th-spout", "spout", region="shell", layer="shell-skin"),
        revolve("th-water", "water", "water", [[0.0005, 0.0], [0.0005, 0.07], [0.05999, 0.0683], [0.05999, 0.0]] if scale == "200" else [[0.0, 0.0], [0.0, 0.066], [0.075, 0.064], [0.075, 0.0]], Y, [0.0, 0.0, 0.0], tone=0.45, region="water", layer="water-skin"),
        of("th-lid", "lid", region="lid", layer="lid-skin"),
        of("th-handle", "handle", region="handle", layer="handle-skin"),
        xloft("th-switch", "switch", "pp-plastic", [(0.0, 0.00375, 0.008), (0.022, 0.00375, 0.008)], 3, role="detail", tone=0.7, region="handle", layer="handle-skin"),
        *steam(scale),
    ]
    wall = 0.010 if scale == "200" else 0.02
    structure = [
        revolve("member-base", "base", "pp-plastic", [[0.03, 0.0], [0.03, 0.086], [0.026, 0.09], [0.004, 0.09], [0.0, 0.086], [0.0, 0.056], [wall, 0.056], [wall, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="member"),
        revolve("member-body", "body", "stainless", [[0.03, 0.0], [0.03, 0.0742 - 3 * wall / 2], [0.175, 0.067 - 3 * wall / 2], [0.175, 0.067], [0.170, 0.0701], [0.0, 0.0752], [0.0, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="body", part="member"),
        core(scale, 1.0),
        of("member-lid", "lid", member="lid", part="member"),
        box("member-switch", "switch", "pp-plastic", [0.03, 0.03, 0.03], [0.004, 0.011, 0.0], role="detail", tone=0.3, member="switch", part="member"),
        revolve("member-water", "water", "water", [[0.021, 0.0], [0.021, 0.06], [0.051, 0.06], [0.051, 0.0]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.45, member="water", part="member"),
        ellipsoid("mass-water", "water", "stainless", [0.04, 0.016, 0.04] if scale == "200" else [0.0375, 0.0375, 0.0375], [0.0, 0.065, 0.0], tone=0.15, member="water", part="mass"),
        revolve("support-base-contact", "base", "stainless", [[0.0, 0.056], [0.0, 0.086], [0.03, 0.086], [0.03, 0.056], [0.0, 0.056]], Y, [0.0, 0.0, 0.0], role="detail", tone=0.3, member="base", part="support", anchor="base-contact", tags=["base-contact"]),
    ]
    return body, thermal, structure


DECISIONS = {
    "50": [
        ("Body: the power base is turned from its real section (30 mm, rounded top edge, a 4 mm recess for the foot skirt) and carries the strain-relief sleeve and the 7.5 mm cord under the handle; the jug is a hollow 0.7 mm stainless cone with floor, rolled rim and seat collar on a 16 mm black foot skirt; the pouring beak is a short pressed cone of five sections square to its rising centre line, 26 mm proud of the wall just under the rim, not a teapot spout; water windows as proud glazing strips on both sides; the C handle is swept through twelve sections from the hinge housing over the rim along the grip (20 x 28 mm, 37 mm finger clearance) back into the wall above the skirt, with the lid release button on its top and the rocker switch as a paddle on its own pivot low on the back of the grip; the domed lid with its skirt in the seat collar is turned about its own centre in the frame of the lid bone and hangs on a knuckle on the hinge axis at the handle.", "Körper: Der Sockel ist nach seinem echten Schnitt gedreht (30 mm, gerundete Oberkante, 4 mm Mulde für die Fußschürze) und trägt unter dem Griff die Knickschutztülle und das 7,5-mm-Kabel; der Krug ist ein hohler Edelstahlkegel von 0,7 mm mit Boden, gerolltem Rand und Sitzkragen auf einer 16 mm hohen schwarzen Fußschürze; die Ausgussschnaupe ist ein kurzer gepresster Kegel aus fünf Schnitten quer zu seiner steigenden Mittellinie, 26 mm vor der Wand knapp unter dem Rand, kein Teekannenausguss; Wasserfenster als vorstehende Glasstreifen auf beiden Seiten; der C-Griff ist über zwölf Schnitte vom Scharniergehäuse über dem Rand entlang des Griffstücks (20 x 28 mm, 37 mm Fingerfreiheit) zurück in die Wand über der Schürze geführt, mit der Deckeltaste auf seiner Oberseite und dem Wippschalter als Wippe an eigenem Drehpunkt unten am Griffrücken; der gewölbte Deckel mit seiner Schürze im Sitzkragen ist im System des Deckelknochens um seine eigene Mitte gedreht und hängt an einem Scharnierauge auf der Scharnierachse am Griff."),
        ("Thermal: plate = the power base with sleeve and cord and the concealed element bonded under the jug floor; shell = the closed outer envelope of skin, skirt and rim plus the beak, its rim disc under the lid marked; water = the fill and both windows; lid = dome and flange above the seat (underside into the sealed jug marked) and the hinge knuckle; handle = handle, release button and switch. Emitters are outlet patches: steam over the beak mouth, the plume as a sleeve over the end of the beak.", "Thermik: Platte = Sockel mit Tülle und Kabel und das unter den Krugboden gebondete verdeckte Heizelement; Hülle = die geschlossene Außenhülle aus Mantel, Schürze und Rand samt Schnaupe, die Randscheibe unter dem Deckel markiert; Wasser = Füllung und beide Fenster; Deckel = Wölbung und Krempe über dem Sitz (Unterseite zum dichten Krug markiert) und das Scharnierauge; Griff = Griff, Deckeltaste und Schalter. Emitter sind Austrittsflächen: Dampf über der Schnaupenmündung, die Fahne als Hülse über dem Ende der Schnaupe."),
        ("Structure: the base housing (deck, wall and four ribs, open below) on its rim support ring, the 360-degree connector as the joint to the jug, stainless wall, skirt and beak as the body members, the handle core with its two screw bosses as joints, the lid with its knuckle on the hinge pin, the switch paddle on its pivot, the water's footprint on the floor, masses of base, jug and water at their centres.", "Tragwerk: das Sockelgehäuse (Deck, Wand und vier Rippen, unten offen) auf seinem Randauflager, der 360-Grad-Stecker als Gelenk zum Krug, Edelstahlwand, Schürze und Schnaupe als Stäbe des Körpers, der Griffkern mit seinen zwei Schraubdomen als Gelenke, der Deckel mit seinem Auge auf dem Scharnierstift, die Schalterwippe an ihrem Drehpunkt, die Aufstandsfläche des Wassers auf dem Boden, Massen von Sockel, Krug und Wasser in ihren Schwerpunkten."),
    ],
    "100": [
        ("Body for a 15 mm feature: base with one chamfer, the jug as a thick-walled cup with skirt and rim in one profile, the beak with three sections, the handle swept through seven sections, the lid with a simplified dome; hinge knuckle, switch paddle and cord sleeve keep their true size; windows, release button and the cord itself are left out.", "Körper für ein 15-mm-Merkmal: Sockel mit einer Fase, der Krug als dickwandiger Becher mit Schürze und Rand in einem Profil, die Schnaupe mit drei Schnitten, der Griff über sieben Schnitte geführt, der Deckel mit vereinfachter Wölbung; Scharnierauge, Schalterwippe und Kabeltülle behalten ihre wahre Größe; Fenster, Deckeltaste und das Kabel selbst entfallen."),
        ("Thermal: base with sleeve, a 15 mm element disc, the closed jug envelope with the beak, the fill, the lid skin with its knuckle, handle and switch, steam patch and plume sleeve at the beak.", "Thermik: Sockel mit Tülle, eine 15 mm starke Heizscheibe, die geschlossene Krughülle mit Schnaupe, die Füllung, die Deckelhaut mit ihrem Auge, Griff und Schalter, Dampffläche und Fahnenhülse an der Schnaupe."),
        ("Structure: base ring, connector joint, jug wall, beak, hinge pin, lid, handle core with one boss joint, switch, water footprint, masses and the support ring.", "Tragwerk: Sockelring, Steckergelenk, Krugwand, Schnaupe, Scharnierstift, Deckel, Griffkern mit einem Domgelenk, Schalter, Aufstandsfläche des Wassers, Massen und der Auflagerring."),
    ],
    "200": [
        ("Body for a 30 mm feature: base, the jug as one solid cone, the beak with two sections, the handle 30 mm thick through eight sections and the lid, whose dome is raised from 14 to 25 mm so that it still prints at the end of the band (1:350).", "Körper für ein 30-mm-Merkmal: Sockel, der Krug als ein voller Kegel, die Schnaupe mit zwei Schnitten, der Griff 30 mm stark über acht Schnitte und der Deckel, dessen Wölbung von 14 auf 25 mm erhöht ist, damit er am Ende des Bandes (1:350) noch druckt."),
        ("Thermal: base, jug, beak, water, lid, handle, a switch patch and the two steam patches.", "Thermik: Sockel, Krug, Schnaupe, Wasser, Deckel, Griff, eine Schalterfläche und die zwei Dampfflächen."),
        ("Structure: base ring, jug wall 15 mm, handle, lid, switch block, water footprint, mass and support ring.", "Tragwerk: Sockelring, Krugwand 15 mm, Griff, Deckel, Schalterblock, Aufstandsfläche des Wassers, Masse und Auflagerring."),
    ],
    "500": [
        ("Body for a 75 mm feature: base disc, jug cone, a stub beak, a square handle loop and the lid as a plug with its dome.", "Körper für ein 75-mm-Merkmal: Sockelscheibe, Krugkegel, eine Stummelschnaupe, eine eckige Griffschlaufe und der Deckel als Stopfen mit seiner Wölbung."),
        ("Thermal: the five regions as their bodies, a full-width water disc, a switch patch and the steam patches.", "Thermik: die fünf Regionen als ihre Körper, eine Wasserscheibe über die ganze Breite, eine Schalterfläche und die Dampfflächen."),
        ("Structure: base ring, jug wall, handle, lid, switch block, water footprint, mass and support ring.", "Tragwerk: Sockelring, Krugwand, Griff, Deckel, Schalterblock, Aufstandsfläche des Wassers, Masse und Auflagerring."),
    ],
}


def volumes():
    c200, c500 = coarse("200", JUG_200, BASE_200), coarse("500", [[0.0, 0.0], [0.0, 0.075], [0.175, 0.069], [0.175, 0.0]], [[0.0, 0.0], [0.0, 0.09], [0.03, 0.09], [0.03, 0.0]])
    lists = {"50": (body_50(), thermal_50(), structure_50()), "100": (body_100(), thermal_100(), structure_100()), "200": c200, "500": c500}
    return {scale: {"status": "authored", "denominator": int(scale), "body": b, "thermal": t, "structure": s, "decisions": [{"en": en, "de": de} for en, de in DECISIONS[scale]]} for scale, (b, t, s) in lists.items()}


def lid_shapes(scale, view_z):
    """🎩 The lid as a turned 2D solid about its own centre on the lid bone, right in every view at every hinge angle."""
    half = [[LID_C, 0.004]] + [[LID_C + r, v] for v, r in LID[scale] if v <= 0.004 and r > 0] + [[LID_C, LID[scale][-1][0]]]
    turned = {"kind": "revolved", "axisBone": "lid", "axis": "v", "axisX": -LID_C, "axisZ": 0, "mirror": True}
    return shape("lid", "lid", "body", view_z, 0.45, "lid", "stainless", **poly(half), solid=turned)


def beak_outline(scale):
    plus, minus = edges(BEAK[scale])
    return minus + plus[::-1]


def beak_plan(scale):
    s = BEAK[scale]
    return [[u, -hw] for u, _, _, hw in s] + [[u, hw] for u, _, _, hw in reversed(s)]


def drawing(scale):
    fine, mid = scale == "50", scale == "100"
    grip_local = in_handle(GRIP[scale])
    base_side = [[0.0855, 0.0], [0.0885, 0.0015], [0.09, 0.004], [0.09, 0.021], [0.0889, 0.0255], [0.0858, 0.0285], [0.0815, 0.03]] if fine else [[0.086, 0.0], [0.09, 0.004], [0.09, 0.025], [0.085, 0.03]]
    base_poly = base_side + [[-a, b] for a, b in reversed(base_side)]
    wall = [[0.0752, 0.0], [0.0752, 0.0145], [0.0746, 0.016], [0.0702, 0.167], [0.0699, 0.1705], [0.069, 0.1732], [0.0672, 0.175]] if fine else [[0.0752, 0.0], [0.0701, 0.17], [0.0672, 0.175]]
    jug = wall + [[-a, b] for a, b in reversed(wall)]
    jug_world = [[a, b + BODY_Y] for a, b in jug]
    merged = ["shell", "water"]
    cord = shape("cord", "base", "detail" if fine else "body", 3.5, 0.2, "plate", "pp-plastic", **(poly([[0.086, 0.004], [0.097, 0.004], [0.103, 0.0055], [0.111, 0.0], [0.13075, 0.0], [0.13075, 0.0075], [0.114, 0.0075], [0.104, 0.0145], [0.097, 0.016], [0.086, 0.016]]) if fine else rect(0.086, 0.004, 0.017, 0.012)))
    elevation, front, plan = [], [], []
    if fine or mid:
        elevation.append(shape("water", "water", "body", 3.1, 0.45, "water", "water", **poly([[0.074, 0.0005], [0.0723, 0.06], [-0.0723, 0.06], [-0.074, 0.0005]])))
    elevation.append(shape("spout", "body", "body", 3.3, 0.4, "shell", "stainless", **poly(beak_outline(scale))))
    if fine:
        elevation.append(shape("body", "body", "body", 3.4, 0.4, "shell", "stainless", filled=False, **poly(jug)))
        elevation.append(shape("skirt-line", "body", "detail", 3.45, 0.3, "shell", "pp-plastic", filled=False, **poly([[-0.0746, 0.016], [0.0746, 0.016]], False)))
        elevation.append(shape("gauge", "body", "glazing", 3.5, 0.7, "water", "water", **rect(-0.014, 0.02, 0.028, 0.12, 0.0075)))
    else:
        elevation.append(shape("body", "body", "body", 3.4, 0.4, merged, "stainless", **poly(jug)))
    elevation.append(shape("handle", "handle", "body", 3.5, 0.2, "handle", "pp-plastic", **poly(band(grip_local))))
    if fine:
        elevation.append(shape("lid-release", "handle", "detail", 3.45, 0.65, "handle", "pp-plastic", **rect(0.014, 0.091, 0.016, 0.012)))
    if fine or mid:
        elevation.append(shape("switch", "switch", "detail" if fine else "body", 3.55, 0.65, "handle", "pp-plastic", **rect(0.0, -0.00375, 0.022, 0.0075, 0.002)))
        elevation.append(shape("lid-hinge", "lid", "body", 3.65, 0.2, "lid", "pp-plastic", **rect(-0.005, -0.005, 0.021, 0.010, 0.004)))
        elevation.append(cord)
    elevation.append(lid_shapes(scale, 3.6))
    elevation.append(shape("base", "base", "body", 3.6, 0.5, "plate", "pp-plastic", **poly(base_poly)))

    front.append(shape("base", "base", "body", 3.6, 0.5, "plate", "pp-plastic", **poly(base_poly)))
    front.append(shape("body", "body", "body", 3.4, 0.5 if fine else 0.4, "shell" if fine else merged, "stainless", **poly(jug_world)))
    if fine:
        front.append(shape("skirt-line", "body", "detail", 3.45, 0.3, "shell", "pp-plastic", filled=False, **poly([[-0.0746, 0.046], [0.0746, 0.046]], False)))
    half_w = max(s[3] for s in GRIP[scale])
    low, high = min(s[1] - s[2] for s in GRIP[scale]), max(s[1] + s[2] for s in GRIP[scale])
    front.append(lid_shapes(scale, 3.45 if fine else 3.6))
    front.append(shape("handle", "handle", "body", 3.5, 0.2, "handle", "pp-plastic", **rect(-half_w, low, 2 * half_w, high - low, 0.01)))
    if fine:
        front.append(shape("handle-grip", "handle", "detail", 3.55, 0.6, "handle", "pp-plastic", filled=False, **poly([[0.0, 0.11], [0.0, 0.19]], False)))
        front.append(shape("lid-release", "handle", "detail", 3.56, 0.65, "handle", "pp-plastic", **rect(-0.008, 0.211, 0.016, 0.012)))
    if fine or mid:
        front.append(shape("switch", "switch", "detail" if fine else "body", 3.56, 0.65, "handle", "pp-plastic", **rect(-0.009, 0.076, 0.018, 0.022, 0.002)))

    plan.append(shape("base", "base", "body", 1, 0.5, "plate", "pp-plastic", **ellipse(0, 0, 0.09, 0.09)))
    if fine or mid:
        plan.append(shape("cord", "base", "detail" if fine else "body", 1.5, 0.2, "plate", "pp-plastic", **(rect(0.086, -0.006, 0.04475, 0.012, 0.00375) if fine else rect(0.086, -0.006, 0.017, 0.012))))
    plan.append(shape("body", "body", "body", 2, 0.4, "shell" if fine else merged, "stainless", **ellipse(0, 0, 0.0752, 0.0752)))
    if fine:
        plan.append(shape("body-shoulder", "body", "detail", 2.1, 0.3, "shell", "stainless", filled=False, **ellipse(0, 0, 0.0699, 0.0699)))
    plan.append(shape("spout", "body", "body", 2.5, 0.4, "shell", "stainless", **poly(beak_plan(scale))))
    plan.append(shape("handle", "handle", "body", 2.6, 0.2, "handle", "pp-plastic", **poly([[0.068, -half_w], [0.100, -0.9 * half_w], [0.1305 if scale != "200" else 0.131, -0.75 * half_w], [0.1305 if scale != "200" else 0.131, 0.75 * half_w], [0.100, 0.9 * half_w], [0.068, half_w]])))
    if fine or mid:
        plan.append(shape("switch", "switch", "detail" if fine else "body", 2.7, 0.65, "handle", "pp-plastic", **rect(0.1255, -0.009, 0.0085, 0.018, 0.002)))
    plan.append(lid_shapes(scale, 3))
    if fine:
        plan.append(shape("lid-release", "handle", "detail", 3.1, 0.65, "handle", "pp-plastic", **ellipse(0.094, 0, 0.008, 0.008)))
    return {"elevation": elevation, "plan": plan, "front": front}


TEXT = {
    "50": {
        "silhouette": ("A 0.23 m wide jug kettle on its power base, 4.6 mm by 4.5 mm on paper: the base with its rounded top edge and the cord leaving under the handle, the tapered stainless jug with its black foot skirt and rolled rim, the short pouring beak under the rim at the left, the C handle at the right running from the hinge housing over the rim down into the wall, with the release button on top and the rocker switch low on the back of the grip, the water window, the 1 L fill behind the outline and the domed lid hanging on its hinge at the handle. Front, 3.6 mm by 4.5 mm: the jug seen from the handle side, the grip with its grip line, the release button and the switch in front of the body, the skirt line and the lid. Plan, 4.6 mm by 3.6 mm: the base disc with the cord, the jug with its rim ring, the beak wedge, the handle with the release button and the lid.", "Ein 0,23 m breiter Wasserkocher auf seinem Sockel, 4,6 mm mal 4,5 mm auf dem Papier: der Sockel mit gerundeter Oberkante und dem Kabel unter dem Griff, der konische Edelstahlkrug mit schwarzer Fußschürze und gerolltem Rand, links die kurze Ausgussschnaupe unter dem Rand, rechts der C-Griff vom Scharniergehäuse über dem Rand hinunter in die Wand, mit der Deckeltaste oben und dem Wippschalter unten am Griffrücken, das Wasserfenster, die 1-l-Füllung hinter dem Umriss und der gewölbte Deckel an seinem Scharnier am Griff. Vorderansicht, 3,6 mm mal 4,5 mm: der Krug von der Griffseite, das Griffstück mit Grifflinie, Deckeltaste und Schalter vor dem Körper, die Schürzenlinie und der Deckel. Draufsicht, 4,6 mm mal 3,6 mm: die Sockelscheibe mit dem Kabel, der Krug mit Randring, der Schnaupenkeil, der Griff mit der Deckeltaste und der Deckel."),
        "retained": ("Base with cord, foot skirt, jug, beak, window, water fill, handle with release button and switch, lid with its hinge knuckle.", "Sockel mit Kabel, Fußschürze, Krug, Schnaupe, Fenster, Wasserfüllung, Griff mit Deckeltaste und Schalter, Deckel mit Scharnierauge."),
        "omitted": ("The limescale filter behind the beak, the cord beyond the handle with its plug and the rubber feet. The beak is hidden behind the body in front; water and window are not visible from the front or from above.", "Der Kalkfilter hinter der Schnaupe, das Kabel jenseits des Griffs mit seinem Stecker und die Gummifüße. Die Schnaupe ist vorn hinter dem Körper verborgen; Wasser und Fenster sind von vorn und von oben nicht sichtbar."),
        "thinElements": ("The window is 28 mm wide, the handle 20 mm thick, the switch paddle and the cord 7.5 mm, the hinge knuckle 10 mm, all at or above the 7.5 mm feature.", "Das Fenster ist 28 mm breit, der Griff 20 mm stark, Schalterwippe und Kabel 7,5 mm, das Scharnierauge 10 mm, alle auf oder über dem 7,5-mm-Merkmal."),
    },
    "100": {
        "silhouette": ("A 2.3 mm wide jug: body, the short beak under the rim, the handle with its switch stub, the domed lid on its hinge and the base with its round-over and the cord sleeve; the 1 L fill lies behind the body. Front, 1.8 mm by 2.25 mm: base, body, handle bar and lid. Plan, 2.3 mm by 1.8 mm: base disc with sleeve, body disc, beak, handle and lid disc.", "Ein 2,3 mm breiter Krug: Körper, die kurze Schnaupe unter dem Rand, der Griff mit seinem Schalterstummel, der gewölbte Deckel an seinem Scharnier und der Sockel mit Rundung und Kabeltülle; die 1-l-Füllung liegt hinter dem Körper. Vorderansicht, 1,8 mm mal 2,25 mm: Sockel, Körper, Griffbalken und Deckel. Draufsicht, 2,3 mm mal 1,8 mm: Sockelscheibe mit Tülle, Körperscheibe, Schnaupe, Griff und Deckelscheibe."),
        "retained": ("Body, beak, handle with switch, lid with hinge knuckle, base with cord sleeve and the water fill.", "Körper, Schnaupe, Griff mit Schalter, Deckel mit Scharnierauge, Sockel mit Kabeltülle und die Wasserfüllung."),
        "merged": ("The body outline carries shell and water; the foot skirt is part of the body.", "Der Körperumriss trägt Hülle und Wasser; die Fußschürze ist Teil des Körpers."),
        "omitted": ("Window, release button, the cord beyond its sleeve, skirt line, rim ring and the grip line.", "Fenster, Deckeltaste, das Kabel jenseits der Tülle, Schürzenlinie, Randring und die Grifflinie."),
        "thinElements": ("The lid dome rises 14 mm (0.14 mm) and stays true; the switch paddle is 7.5 mm and the sleeve 12 mm and stay true.", "Die Deckelwölbung steigt 14 mm (0,14 mm) und bleibt wahr; die Schalterwippe ist 7,5 mm und die Tülle 12 mm und bleiben wahr."),
    },
    "200": {
        "silhouette": ("A 1.15 mm wide jug of five shapes: body, beak, handle, lid and base. Front, 0.9 mm by 1.1 mm: base and body with the lid on top and the handle bar in front. Plan, 1.15 mm by 0.9 mm: the base disc under the jug disc with the beak wedge, the handle and the lid.", "Ein 1,15 mm breiter Krug aus fünf Formen: Körper, Schnaupe, Griff, Deckel und Sockel. Vorderansicht, 0,9 mm mal 1,1 mm: Sockel und Körper mit dem Deckel darauf und dem Griffbalken davor. Draufsicht, 1,15 mm mal 0,9 mm: die Sockelscheibe unter der Krugscheibe mit Schnaupenkeil, Griff und Deckel."),
        "retained": ("The five masses.", "Die fünf Massen."),
        "merged": ("Body, window and water share one outline.", "Körper, Fenster und Wasser teilen einen Umriss."),
        "omitted": ("Water, window, switch, release button, hinge knuckle, cord and every detail line; the handle is drawn 30 mm thick.", "Wasser, Fenster, Schalter, Deckeltaste, Scharnierauge, Kabel und jede Detaillinie; der Griff ist 30 mm stark gezeichnet."),
        "thinElements": ("The base is 30 mm tall (0.15 mm), the handle 30 mm thick; the lid dome is raised from 14 mm to 25 mm (0.125 mm) so that it prints down to 1:350.", "Der Sockel ist 30 mm hoch (0,15 mm), der Griff 30 mm stark; die Deckelwölbung ist von 14 mm auf 25 mm (0,125 mm) erhöht, damit sie bis 1:350 druckt."),
    },
}


def mutate(asset):
    asset["dimensions"].update(heightM=0.223, widthM=0.23)
    bones = asset["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    by["handle"]["title"] = {"en": "Handle with hinge housing and release button", "de": "Griff mit Scharniergehäuse und Deckeltaste"}
    by["handle"]["mass"]["kg"] = 0.115
    by["lid"].update(x=HINGE[0], y=HINGE[1] - BODY_Y, angleDeg=180, lengthM=0.132, title={"en": "Lid on its hinge at the handle, rattles when boiling", "de": "Deckel an seinem Scharnier am Griff, klappert beim Kochen"}, joint={"type": "revolute", "minDeg": -85, "maxDeg": 0})
    by["lid"]["mass"].update(comX=LID_C, comY=0.0)
    if "switch" not in by:
        bones.insert(bones.index(by["handle"]) + 1, {"id": "switch", "parent": "handle", "x": round(SWITCH[0] - HANDLE[0], 5), "y": round(SWITCH[1] - HANDLE[1], 5), "angleDeg": -80, "title": {"en": "Rocker switch", "de": "Wippschalter"}, "lengthM": 0.022, "joint": {"type": "revolute", "minDeg": -20, "maxDeg": 0}, "mass": {"kg": 0.005, "comX": 0.011, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    for anchor in asset["anchors"]:
        if anchor["id"] == "spout-tip":
            anchor.update(x=-0.0965, y=0.1735)
    poses = {p["id"]: p for p in asset["poses"]}
    poses["off"]["bones"] = {}
    poses["heating"]["bones"] = {"switch": {"angleDeg": -20}}
    poses["boiling"]["bones"] = {"switch": {"angleDeg": -20}, "lid": {"angleDeg": -1.4}}
    if "filling" not in poses:
        filling = copy.deepcopy(poses["off"])
        filling.update(id="filling", title={"en": "Lid open for filling", "de": "Deckel zum Füllen offen"}, description={"en": "The kettle is off and the lid stands open on its hinge at 80 degrees.", "de": "Der Wasserkocher ist aus, der Deckel steht an seinem Scharnier 80 Grad offen."}, bones={"lid": {"angleDeg": -80}})
        asset["poses"].insert(1, filling)
    lift = {0.005: -2.3, 0.001: -0.5, 0.004: -1.8}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            dy = key.get("bones", {}).get("lid", {}).get("dy")
            if dy is not None:
                key["bones"]["lid"] = {"angleDeg": lift[dy]}
    notes = asset["provenance"]["notes"]
    swaps = {
        "en": [("a covered spout rising from two thirds of the height to 0.19 m at the left", "a short covered pouring beak pressed out of the wall just under the rim at the left"), ("a water window on the side and a C handle at the right whose rocker switch sits on the back of the grip; the loose domed lid with its knob rests on the seat collar and rattles in the steam", "a water window on each side and a C handle at the right that carries the lid hinge and its release button on top and the rocker switch low on the back of the grip; the domed lid turns on that hinge, rests on the seat collar and rattles in the steam; the cord leaves the base under the handle"), ("handle 0.12, lid 0.1", "handle 0.115, switch 0.005, lid 0.1")],
        "de": [("einem geschlossenen Ausguss, der von zwei Dritteln der Höhe links auf 0,19 m steigt", "einer kurzen geschlossenen Ausgussschnaupe, links knapp unter dem Rand aus der Wand gepresst"), ("einem Wasserfenster an der Seite und einem C-Griff rechts, dessen Wippschalter am Rücken des Griffs sitzt; der lose gewölbte Deckel mit Knopf liegt auf dem Sitzkragen und klappert im Dampf", "einem Wasserfenster auf jeder Seite und einem C-Griff rechts, der oben das Deckelscharnier mit seiner Taste und unten am Griffrücken den Wippschalter trägt; der gewölbte Deckel dreht an diesem Scharnier, liegt auf dem Sitzkragen und klappert im Dampf; das Kabel verlässt den Sockel unter dem Griff"), ("Griff 0,12, Deckel 0,1", "Griff 0,115, Schalter 0,005, Deckel 0,1")],
    }
    for lang, pairs in swaps.items():
        for old, new in pairs:
            if new not in notes[lang]:
                assert old in notes[lang], old
                notes[lang] = notes[lang].replace(old, new)
    for scale in ("50", "100", "200"):
        rep = asset["representations"][scale]
        rep["geometry"] = drawing(scale)
        for key, (en, de) in TEXT[scale].items():
            rep["decisions"][key] = {"en": en, "de": de}


run(SLUG, mutate, volumes())
