"""🧱 Round 3, appliances/desktop-monitor: every body solid of every authored scale, stated by hand in bone-local coordinates.

Panel: bezel frame with the cover glass set 0.7 mm back in its opening, the LCD module behind it, the rear cover as a loft from the full 0.54 x 0.323 m edge to the flat back of the electronics hump, vent slots on the upper slope, the quick-release bracket on the VESA field. Stand: riser column with a flared foot and the tilt-hinge barrel at its head, base plate with a bevelled top edge on four rubber pads.
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_desktop_monitor_kit", os.path.join(HERE, "r3-desktop-monitor-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

HOUSING_50 = [
    (-0.0305, -0.0085, 0.1615, 0.2700, 40),
    (-0.0290, -0.0085, 0.1605, 0.2690, 40),
    (-0.0270, -0.0105, 0.1555, 0.2640, 8),
    (-0.0230, -0.0160, 0.1420, 0.2500, 8),
    (-0.0180, -0.0210, 0.1270, 0.2330, 8),
    (-0.0120, -0.0250, 0.1120, 0.2120, 8),
    (-0.0070, -0.0270, 0.0990, 0.1920, 8),
    (-0.0045, -0.0280, 0.0900, 0.1780, 8),
    (-0.0040, -0.0280, 0.0870, 0.1740, 8),
]
HOUSING_100 = [
    (-0.0295, -0.0085, 0.1615, 0.2700, 40),
    (-0.0270, -0.0105, 0.1555, 0.2640, 8),
    (-0.0180, -0.0210, 0.1270, 0.2330, 8),
    (-0.0090, -0.0260, 0.1050, 0.2020, 8),
    (-0.0040, -0.0280, 0.0880, 0.1750, 8),
]
SLAB_200 = [
    (-0.0400, -0.0085, 0.1615, 0.2700, 40),
    (-0.0275, -0.0085, 0.1615, 0.2700, 40),
]
HOUSING_200 = [
    (-0.0274, -0.0110, 0.1560, 0.2640, 8),
    (-0.0150, -0.0230, 0.1200, 0.2250, 8),
    (-0.0040, -0.0270, 0.0920, 0.1820, 8),
]
NECK_50 = [
    (-0.004, 0.0100, 0.0180, 0.0400, 4),
    (0.003, 0.0095, 0.0165, 0.0385, 4),
    (0.014, 0.0085, 0.0128, 0.0345, 4),
    (0.030, 0.0080, 0.0112, 0.0320, 4),
    (0.215, 0.0080, 0.0110, 0.0310, 4),
    (0.234, 0.0072, 0.0104, 0.0295, 4),
    (0.244, 0.0060, 0.0085, 0.0250, 4),
    (0.248, 0.0050, 0.0055, 0.0190, 4),
]
NECK_100 = [
    (-0.004, 0.0100, 0.0180, 0.0400, 4),
    (0.012, 0.0085, 0.0128, 0.0345, 4),
    (0.228, 0.0078, 0.0110, 0.0310, 4),
    (0.252, 0.0050, 0.0060, 0.0200, 4),
]
NECK_200 = [
    (-0.004, 0.0120, 0.0190, 0.0380, 4),
    (0.236, 0.0110, 0.0170, 0.0320, 4),
    (0.256, 0.0080, 0.0080, 0.0200, 4),
]
BASE_50 = [
    (0.0020, 0.105, 0.1030, 0.1230, 6),
    (0.0035, 0.105, 0.1050, 0.1250, 6),
    (0.0070, 0.105, 0.1050, 0.1250, 6),
    (0.0095, 0.105, 0.1030, 0.1230, 6),
    (0.0112, 0.105, 0.0985, 0.1185, 6),
    (0.0122, 0.105, 0.0915, 0.1115, 6),
]
BASE_100 = [
    (0.0000, 0.105, 0.1035, 0.1235, 6),
    (0.0075, 0.105, 0.1050, 0.1250, 6),
    (0.0110, 0.105, 0.0995, 0.1195, 6),
    (0.0122, 0.105, 0.0915, 0.1115, 6),
]
BASE_200 = [
    (0.0000, 0.105, 0.1050, 0.1250, 6),
    (0.0080, 0.105, 0.1050, 0.1250, 6),
    (0.0122, 0.105, 0.0950, 0.1150, 6),
]


def top_line(rows, v):
    """📈 The crest (centre u + half height) of a housing loft at depth v, linear between its sections as the kernel joins them."""
    for a, b in zip(rows, rows[1:]):
        if a[0] <= v <= b[0]:
            t = (v - a[0]) / (b[0] - a[0])
            return (a[1] + a[2]) * (1 - t) + (b[1] + b[2]) * t
    raise ValueError(v)


def vent_band(rows, v0, v1):
    """🌬 The vent band on the upper slope of the hump between depths v0 and v1: its middle (u, v), the unit tangent down the slope, the outward normal and the slope length."""
    p0, p1 = (top_line(rows, v0), v0), (top_line(rows, v1), v1)
    du, dv = p1[0] - p0[0], p1[1] - p0[1]
    length = math.hypot(du, dv)
    t = (du / length, dv / length)
    return ((p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2), t, (t[1], -t[0]), length


VENT = vent_band(HOUSING_50, -0.0225, -0.0130)


def vents50():
    """🌬 Four vent slots across the upper slope of the hump: dark strips laid on the surface along w, 3 mm across the slope."""
    (um, vm), t, n, length = VENT
    angle = math.atan2(t[1], t[0])
    step = length / 4
    u0 = um - t[0] * step * 1.5 - n[0] * 0.0002
    v0 = vm - t[1] * step * 1.5 - n[1] * 0.0002
    slot = K.box("vents", "panel", "housing-plastic", (u0 - 0.0015, u0 + 0.0015), (v0 - 0.0004, v0 + 0.0004), (-0.10, 0.10), tone=0.22, role="detail", rotation=K.about_w(angle), texture=True)
    return K.repeat(slot, 4, du=t[0] * step, dv=t[1] * step)


def glass_rect(u0, u1, half_w, radius):
    """🪟 The glass opening in (w, u)."""
    return K.rounded((-half_w, half_w), (u0, u1), radius)


PANEL_OUTLINE = K.rounded((-K.PANEL_W, K.PANEL_W), (K.PANEL_U0, K.PANEL_U1), 0.005)


def body50():
    """🖥 1:50: bezel frame, recessed cover glass, power light, rear cover loft, vent slots, bracket, hinge barrel, riser, base plate, four pads."""
    opening = glass_rect(K.GLASS_U0, K.GLASS_U1, K.GLASS_W, 0.0015)
    return [
        K.face("panel-slab", "panel", "bezel-plastic", PANEL_OUTLINE, (K.FRONT_V, K.MODULE_V), holes=[opening]),
        K.face("screen-glass", "panel", "display-glass", opening, (K.GLASS_V, K.MODULE_V), role="glazing"),
        K.blob("power-led", "panel", "bezel-plastic", (-0.1605, K.FRONT_V, -0.24), (0.004, 0.0015, 0.004), tone=0.9, role="glazing"),
        K.along_v("housing", "panel", "housing-plastic", HOUSING_50),
        vents50(),
        K.box("mount-plate", "panel", "stand-plastic", (-0.05, 0.04), (-0.0055, 0.0025), (-0.04, 0.04), radius=0.003),
        K.turned("hinge", "neck", "stand-plastic", [[0, 0], [0, 0.010], [0.06, 0.010], [0.06, 0]], [0, 0, 1], (K.NECK_LEN, 0.0, -0.03), tone=0.32),
        K.along_u("neck", "neck", "stand-plastic", NECK_50),
        K.along_v("base-plate", "base", "stand-plastic", BASE_50),
        K.repeat(K.box("foot-front", "base", "rubber-foot", (0.174, 0.196), (0.0, 0.003), (-0.106, -0.084), radius=0.001), 2, dw=0.19),
        K.repeat(K.box("foot-rear", "base", "rubber-foot", (0.014, 0.036), (0.0, 0.003), (-0.106, -0.084), radius=0.001), 2, dw=0.19),
    ]


def body100():
    """🖥 1:100: bezel frame and glass (bezel widened to one feature), rear cover in five sections, riser with the hinge merged into its head, base plate on the desk with the pads merged."""
    opening = glass_rect(-0.148, 0.140, 0.255, 0.002)
    return [
        K.face("panel-slab", "panel", "bezel-plastic", PANEL_OUTLINE, (K.FRONT_V, -0.0295), holes=[opening]),
        K.face("screen-glass", "panel", "display-glass", opening, (-0.0392, -0.0295), role="glazing"),
        K.along_v("housing", "panel", "housing-plastic", HOUSING_100),
        K.along_u("neck", "neck", "stand-plastic", NECK_100),
        K.along_v("base-plate", "base", "stand-plastic", BASE_100),
    ]


def body200():
    """🖥 1:200: the panel as one loft, a 12.5 mm slab (bezel, glass and module merged) stepping to the rear cover hump in three sections, a riser of one feature depth, the base plate."""
    return [
        K.along_v("panel", "panel", "bezel-plastic", SLAB_200 + HOUSING_200, tone=0.3),
        K.along_u("neck", "neck", "stand-plastic", NECK_200),
        K.along_v("base-plate", "base", "stand-plastic", BASE_200),
    ]
