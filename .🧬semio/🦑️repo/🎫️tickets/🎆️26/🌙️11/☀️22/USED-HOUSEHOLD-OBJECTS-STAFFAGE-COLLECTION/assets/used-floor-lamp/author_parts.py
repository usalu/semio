"""💡 Hand-chosen numbers of the used halogen uplighter floor lamp and the small geometric helpers that turn them into solids, shapes, areas and masses.

All numbers are metres, kilograms and seconds. Lamp coordinates are (h, r): height above the underside of the base plate along the lamp bone `u` and radius from the stem axis; the lamp bone frame is u up, v backwards (world -x), w left.

@see ./author-used-floor-lamp.py — the authoritative script that writes the asset document
@see ../../brief.md — the author brief
@see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — frames of shapes and solids
"""
import math

G = 9.80665
SCALES = [50, 100, 200, 500]
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}

CARD_T = 0.007
CARD_S = 0.45
RHO_STEEL = 7850.0
RHO_GLASS = 2500.0

BASE = {
    50: [(0.145, 0.0), (0.145, 0.004), (0.1438, 0.0068), (0.1405, 0.0093), (0.135, 0.0112), (0.120, 0.0145), (0.100, 0.0185), (0.075, 0.0222), (0.050, 0.0250), (0.030, 0.0266), (0.0235, 0.0272), (0.022, 0.0295), (0.022, 0.0455), (0.0205, 0.048)],
    100: [(0.145, 0.0), (0.145, 0.005), (0.140, 0.0095), (0.120, 0.0145), (0.090, 0.0200), (0.050, 0.0250), (0.023, 0.0270), (0.022, 0.047)],
    200: [(0.145, 0.0), (0.145, 0.007), (0.110, 0.016), (0.060, 0.024), (0.022, 0.027), (0.022, 0.048)],
    500: [(0.145, 0.0), (0.145, 0.010), (0.080, 0.022), (0.022, 0.030), (0.022, 0.048)],
}
BASE_TOP = 0.048
STEM_R = 0.0125
STEM_U = (0.046, 1.629)
STEM_T = 0.001
COUPLERS = [0.60, 1.15]
COUPLER_R = 0.014
COUPLER_HALF = 0.006
DIMMER = {
    50: [(0.950, 0.0), (0.950, 0.0165), (0.9515, 0.0195), (0.955, 0.0212), (0.960, 0.022), (1.026, 0.022), (1.031, 0.0212), (1.0345, 0.0195), (1.036, 0.0165), (1.036, 0.0)],
    100: [(0.950, 0.0), (0.950, 0.018), (0.957, 0.022), (1.029, 0.022), (1.036, 0.018), (1.036, 0.0)],
    200: [(0.950, 0.0), (0.950, 0.022), (1.036, 0.022), (1.036, 0.0)],
}
DIMMER_SEAM = 0.993
KNOB_U = 0.993
KNOB_V = -0.0155
KNOB = {
    50: [(0.0, 0.0), (0.0, 0.0145), (0.0075, 0.015), (0.0185, 0.015), (0.0225, 0.0125), (0.0225, 0.0)],
    100: [(0.0, 0.0), (0.0, 0.015), (0.019, 0.015), (0.0225, 0.0125), (0.0225, 0.0)],
}
KNOB_OUT = 0.0225

HEAD_U = 1.60
REFLECTOR = {
    50: [(0.0, 0.0125), (0.0, 0.017), (0.028, 0.017), (0.092, 0.0688), (0.096, 0.0688), (0.096, 0.082), (0.100, 0.082), (0.100, 0.0688), (0.138, 0.0688), (0.138, 0.068), (0.093, 0.068), (0.029, 0.0162), (0.029, 0.0125), (0.0, 0.0125)],
    100: [(0.0, 0.0125), (0.0, 0.0172), (0.0285, 0.0172), (0.092, 0.069), (0.096, 0.069), (0.096, 0.082), (0.100, 0.082), (0.100, 0.069), (0.138, 0.069), (0.138, 0.0679), (0.0935, 0.0679), (0.0295, 0.0161), (0.0295, 0.0125), (0.0, 0.0125)],
    200: [(0.0, 0.0125), (0.0, 0.018), (0.030, 0.018), (0.094, 0.07), (0.096, 0.082), (0.100, 0.082), (0.100, 0.07), (0.138, 0.07), (0.138, 0.0675), (0.095, 0.0675), (0.031, 0.0162), (0.031, 0.0125), (0.0, 0.0125)],
    500: [(0.0, 0.0), (0.0, 0.018), (0.030, 0.018), (0.094, 0.07), (0.096, 0.082), (0.100, 0.082), (0.100, 0.07), (0.136, 0.07), (0.136, 0.0)],
}
REFLECTOR_OUTLINE = {
    50: [(0.0, 0.0), (0.0, 0.017), (0.028, 0.017), (0.092, 0.0688), (0.096, 0.0688), (0.096, 0.082), (0.100, 0.082), (0.100, 0.0688), (0.138, 0.0688), (0.138, 0.0)],
    100: [(0.0, 0.0), (0.0, 0.0172), (0.0285, 0.0172), (0.092, 0.069), (0.096, 0.082), (0.100, 0.082), (0.100, 0.069), (0.138, 0.069), (0.138, 0.0)],
    200: [(0.0, 0.0), (0.0, 0.018), (0.030, 0.018), (0.094, 0.07), (0.096, 0.082), (0.100, 0.082), (0.100, 0.07), (0.138, 0.07), (0.138, 0.0)],
    500: [(0.0, 0.0), (0.0, 0.018), (0.030, 0.018), (0.094, 0.07), (0.096, 0.082), (0.100, 0.082), (0.100, 0.07), (0.136, 0.07), (0.136, 0.0)],
}
TUBE_U = 0.112
TUBE_R = 0.005
TUBE_HALF = 0.054
HOLDER = {"u": 0.110, "w": 0.0605, "size": [0.020, 0.016, 0.013]}
GUARD_U = 0.139
GUARD_HALF = 0.0675
GUARD_R = 0.001
MOUTH_R = 0.0675

BOWL_U = 0.100
BOWL = {
    50: [(0.0, 0.074), (0.0, 0.100), (0.008, 0.125), (0.022, 0.148), (0.042, 0.164), (0.065, 0.174), (0.086, 0.179), (0.100, 0.180), (0.100, 0.176), (0.0865, 0.1752), (0.0665, 0.1702), (0.045, 0.1605), (0.026, 0.145), (0.0125, 0.123), (0.005, 0.0995), (0.004, 0.074), (0.0, 0.074)],
    100: [(0.0, 0.074), (0.0, 0.100), (0.010, 0.130), (0.030, 0.156), (0.060, 0.173), (0.100, 0.180), (0.100, 0.176), (0.0605, 0.169), (0.0335, 0.153), (0.0145, 0.128), (0.004, 0.100), (0.004, 0.074), (0.0, 0.074)],
    200: [(0.0, 0.074), (0.0, 0.100), (0.020, 0.145), (0.060, 0.173), (0.100, 0.180), (0.100, 0.176), (0.0615, 0.169), (0.0235, 0.142), (0.004, 0.100), (0.004, 0.074), (0.0, 0.074)],
    500: [(0.0, 0.074), (0.0, 0.100), (0.040, 0.163), (0.100, 0.180), (0.100, 0.176), (0.0425, 0.159), (0.004, 0.100), (0.004, 0.074), (0.0, 0.074)],
}
BOWL_T = 0.004
BOWL_RIM = 0.180
LAMP_H = HEAD_U + BOWL_U + 0.100

CABLE_R = 0.003
CABLE_EXIT = (0.005, 0.145)
CABLE_L = 0.50
RISER_L = 0.30
TAPE = (0.19, 0.25, 0.0045)
GROMMET = [(-0.006, 0.0), (-0.006, 0.005), (0.004, 0.005), (0.008, 0.0035), (0.008, 0.0)]
PLUG_COLLAR = [(-0.002, 0.0), (-0.002, 0.0045), (0.012, 0.0055), (0.012, 0.0)]
PLUG_BODY = {"u0": 0.012, "u1": 0.045, "top": -0.017, "bottom": 0.003, "half": 0.018}
PLUG_L = 0.045

GRIP_LOWER = 0.85
GRIP_UPPER = 1.45
CARRY_LIFT = 0.05
CARRY_TILT = -6.0
CARD_SHIFT = 0.65
PAD_R = 0.11


def rnd(value, digits=5):
    """🔢 Rounds a number or a nested list of numbers for the document."""
    if isinstance(value, (list, tuple)):
        return [rnd(v, digits) for v in value]
    out = round(value, digits)
    return 0.0 if out == 0 else out


def d(value, digits):
    """🇩🇪 A number with a decimal comma for the German texts."""
    return f"{value:.{digits}f}".replace(".", ",")


def T(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def base_profile(s):
    """🥏 The (axial, radial) revolve profile of the domed base plate at one scale, closed through the axis."""
    return [(0.0, 0.0)] + [(h, r) for r, h in BASE[s]] + [(BASE_TOP, 0.0)]


def revolve_area(profile):
    """🏺 Surface area of a solid of revolution of an (axial, radial) polyline: a closed loop or an open profile that starts and ends on the axis."""
    return sum(math.pi * (r0 + r1) * math.hypot(a1 - a0, r1 - r0) for (a0, r0), (a1, r1) in zip(profile, profile[1:]))


def revolve_volume(profile):
    """🫙 Volume enclosed by a revolve profile (Pappus on the shoelace polygon of the closed profile)."""
    pts = profile if profile[0] == profile[-1] else profile + [profile[0]]
    v = 0.0
    for (a0, r0), (a1, r1) in zip(pts, pts[1:]):
        v += math.pi * (a1 - a0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
    return abs(v)


def revolve_moment(profile):
    """⚖️ First axial moment of the volume of a revolve profile (for its centre along the axis)."""
    pts = profile if profile[0] == profile[-1] else profile + [profile[0]]
    m = 0.0
    for (a0, r0), (a1, r1) in zip(pts, pts[1:]):
        m += math.pi * (a1 - a0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3 * (a0 + a1) / 2
    return abs(m)


def fk(chain):
    """🦴 World angle (degrees) and origin of the last bone of a planar chain [(x, y, angleDeg), …] given in parent-local coordinates, starting at the world origin with angle 0."""
    ox, oy, th = 0.0, 0.0, 0.0
    out = []
    for x, y, a in chain:
        c, s = math.cos(math.radians(th)), math.sin(math.radians(th))
        ox, oy = ox + x * c - y * s, oy + x * s + y * c
        th += a
        out.append((ox, oy, th))
    return out


def cable_angles(lift_y, tilt):
    """🔌 Relative angles of the cable run (lying back from the base exit to the floor) and of the riser (flat on the floor) for a lamp lifted to `lift_y` and tilted by `tilt`."""
    exit_ = fk([(0.0, lift_y, 90.0), (0.0, 0.0, tilt), (CABLE_EXIT[0], CABLE_EXIT[1], 0.0)])[-1]
    drop = exit_[1] - CABLE_R
    phi = 180.0 + math.degrees(math.asin(drop / CABLE_L))
    lamp_world = 90.0 + tilt
    return phi - lamp_world, 180.0 - phi, exit_


def coupler_profile(u):
    """🔩 The (axial, radial) profile of a threaded splice coupler of the stem centred at height `u`, closed through the axis."""
    return [(u - COUPLER_HALF, 0.0), (u - COUPLER_HALF, COUPLER_R - 0.0005), (u - COUPLER_HALF + 0.002, COUPLER_R), (u + COUPLER_HALF - 0.002, COUPLER_R), (u + COUPLER_HALF, COUPLER_R - 0.0005), (u + COUPLER_HALF, 0.0)]
