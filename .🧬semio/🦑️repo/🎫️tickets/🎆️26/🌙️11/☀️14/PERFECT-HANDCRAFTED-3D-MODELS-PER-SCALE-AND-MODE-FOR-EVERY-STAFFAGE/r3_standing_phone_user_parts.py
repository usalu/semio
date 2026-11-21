"""🎧 What makes the standing adult a phone user, stated by hand for the coarser scales (1:100, 1:200, 1:500): the ponytail on its own bone (world of the rest pose), the headphones round the neck (world of the rest pose, carried by the trunk), the two hands in the texting grip and the phone (grip frame, see `r3_standing_phone_user_kit.grip`), and the hand bones of the skeleton. The 1:50 parts are in `r3_standing_phone_user_body50`.
"""
import math

from r3_standing_phone_user_kit import HAND_U, HAND_V, PHONE_LENGTH, PHONE_THICK, PHONE_WIDTH, capsule, ellipsoid_axes, frame_loft, free_loft, grip, grip_dir, on_hand, posed, ref, rounded_plate, st, th

SKIN, HAIR, POLYMER, GLASS = "skin", "hair", "headphone-polymer", "phone-glass-metal"
BONE = 0.85
FLOOR = 0.0375
TAILS = {
    50: [(-0.083, 1.613, 0.016, 0.016), (-0.093, 1.601, 0.022, 0.019), (-0.099, 1.576, 0.027, 0.022), (-0.102, 1.546, 0.029, 0.023), (-0.105, 1.518, 0.025, 0.019), (-0.110, 1.498, 0.018, 0.013), (-0.116, 1.486, 0.009, 0.007)],
    100: [(-0.083, 1.613, 0.017, 0.017), (-0.096, 1.590, 0.026, 0.021), (-0.102, 1.546, 0.029, 0.023), (-0.107, 1.507, 0.021, 0.016), (-0.114, 1.489, 0.011, 0.009)],
    200: [(-0.083, 1.613, 0.018, 0.018), (-0.098, 1.582, 0.028, 0.022), (-0.104, 1.530, 0.026, 0.020), (-0.112, 1.495, 0.016, 0.015)],
}
PALMS = {
    100: [(-0.016, 0.000, 0.000, 0.025, 0.0170, 2.2), (0.020, 0.002, 0.000, 0.033, 0.0158, 2.3), (0.062, 0.001, 0.001, 0.040, 0.0140, 2.4), (0.094, -0.001, 0.003, 0.039, 0.0110, 2.4)],
}
SLABS = {
    "r": [(0.086, 0.000, 0.004, 0.037, 0.0095), (0.120, 0.002, 0.0105, 0.037, 0.0085), (0.150, 0.004, 0.0110, 0.033, 0.0080), (0.172, 0.005, 0.0112, 0.022, 0.0076)],
    "l": [(0.086, 0.000, 0.003, 0.037, 0.0095), (0.120, 0.002, 0.0040, 0.037, 0.0085), (0.150, 0.004, 0.0080, 0.033, 0.0080), (0.172, 0.005, 0.0120, 0.022, 0.0076)],
}
THUMBS = {"r": [(-0.052, -0.074, -0.006), (-0.006, -0.041, 0.0128), (0.012, -0.023, 0.0126)], "l": [(-0.054, -0.073, 0.008), (-0.010, -0.042, 0.0380), (0.010, -0.025, 0.0386)]}
MASSES = {
    200: {"r": [(-0.014, 0.000, 0.000, 0.025, 0.0170, 2.3), (0.040, 0.004, 0.002, 0.040, 0.0165, 2.4), (0.110, 0.004, 0.004, 0.040, 0.0155, 2.4), (0.170, 0.005, 0.004, 0.024, 0.0150, 2.4)], "l": [(-0.014, 0.000, 0.000, 0.025, 0.0170, 2.3), (0.040, 0.004, 0.000, 0.040, 0.0165, 2.4), (0.110, 0.004, -0.001, 0.040, 0.0155, 2.4), (0.170, 0.005, -0.001, 0.024, 0.0150, 2.4)]},
    500: {"r": [(-0.012, 0.000, 0.000, 0.030, 0.0240, 2.3), (0.070, 0.006, 0.000, 0.050, 0.0260, 2.6), (0.165, 0.004, 0.000, 0.030, 0.0210, 2.4)], "l": [(-0.012, 0.000, 0.000, 0.030, 0.0240, 2.3), (0.070, 0.006, 0.000, 0.050, 0.0260, 2.6), (0.165, 0.004, 0.000, 0.030, 0.0210, 2.4)]},
}
BAND = [(0.078, 1.450, 0.096), (0.048, 1.476, 0.091), (0.010, 1.493, 0.087), (-0.030, 1.501, 0.080), (-0.060, 1.504, 0.056), (-0.077, 1.504, 0.026), (-0.081, 1.504, 0.0)]


def tail(sid, rows, tone=0.75):
    """🎀 The ponytail on its own bone as a loft along a path in the median plane of the rest pose (upright head): from the tie down the back of the head, its end resting on the hood; rows are (x, y, half breadth, half thickness). Every pose turns the bone so that it keeps falling under gravity."""
    sections = []
    for i, (x, y, rz, rt) in enumerate(rows):
        p, q = rows[max(i - 1, 0)], rows[min(i + 1, len(rows) - 1)]
        t = (q[0] - p[0], q[1] - p[1])
        n = math.hypot(*t)
        sections.append(((x, y, 0.0), (0.0, 0.0, 1.0), (-t[1] / n, t[0] / n, 0.0), rz, rt))
    return frame_loft(sid, "ponytail", HAIR, tone, sections)


def ponytail(scale):
    return [tail("ponytail", TAILS[scale])] if scale in TAILS else []


def cup_frame(sign):
    axis = (0.50, 0.16, sign * 0.85)
    n = math.sqrt(sum(v * v for v in axis))
    axis = tuple(v / n for v in axis)
    a = (0.86, 0.0, -sign * 0.50)
    b = (axis[1] * a[2] - axis[2] * a[1], axis[2] * a[0] - axis[0] * a[2], axis[0] * a[1] - axis[1] * a[0])
    centre = (0.090, 1.412, sign * 0.094)
    return a, b, lambda t: tuple(centre[i] + axis[i] * t for i in range(3))


def band_halves(tone, ra, rb, points=BAND, sub=1):
    """🎧 The headband as two halves that meet behind the neck, each a strap from above its cup round to the back."""
    centre = [(points[-1][0], points[-1][1], -0.004)]
    return [free_loft("headphone-band-" + side, "torso", POLYMER, tone, [(x, y, sign * z, ra, rb) for x, y, z in points + centre], e=3.0, sub=sub) for side, sign in (("l", 1), ("r", -1))]


def headphones(scale):
    out = []
    if scale == 100:
        out += band_halves(0.3, 0.0077, 0.015, [BAND[0], BAND[2], BAND[4], BAND[6]])
    for side, sign in (("l", 1), ("r", -1)):
        a, b, along = cup_frame(sign)
        if scale == 100:
            cup = [(along(-0.024), a, b, 0.037, 0.041), (along(-0.012), a, b, 0.043, 0.046), (along(0.010), a, b, 0.041, 0.044), (along(0.027), a, b, 0.018, 0.020)]
            out.append(frame_loft("headphone-cup-" + side, "torso", POLYMER, 0.85, cup))
        elif scale == 200:
            out.append(ellipsoid_axes("headphone-cup-" + side, "torso", POLYMER, 0.85, along(0.001), a, b, (0.043, 0.046, 0.026)))
    return out


def hand_rows(side, rows):
    width, thick = grip_dir(HAND_V, side), grip_dir((0, 0, 1), side)
    return [(grip(on_hand(s, t, n), side), width, thick, hw, ht, *rest) for s, t, n, hw, ht, *rest in rows]


def hands(scale):
    out = []
    with posed():
        for side in "lr":
            s, bone = "-" + side, "hand-" + side
            if scale == 100:
                out.append(frame_loft("hand" + s, bone, SKIN, 0.3, hand_rows(side, PALMS[100])))
                out.append(frame_loft("fingers" + s, bone, SKIN, 0.3, hand_rows(side, SLABS[side]), e=3.0))
                t = [grip(p, side) for p in THUMBS[side]]
                out.append(capsule("thumb" + s + "-0", bone, SKIN, 0.3, t[0], t[1], 0.0115, 0.0095))
                out.append(capsule("thumb" + s + "-1", bone, SKIN, 0.3, t[1], t[2], 0.0095, 0.0080))
            else:
                out.append(frame_loft("hand" + s, bone, SKIN, 0.3, hand_rows(side, MASSES[scale][side])))
    return out


def phone(scale):
    half = PHONE_THICK / 2
    return [rounded_plate("phone", "phone", GLASS, 0.92 if scale == 100 else 0.6, 0.0, PHONE_LENGTH, PHONE_WIDTH / 2, 0.010, -half, half)] if scale < 500 else []


def hand_bones(scale):
    out = []
    with posed():
        for side in "lr":
            s, bone = "-" + side, "hand-" + side
            if scale == 100:
                plate = [(0.006, 0.000, 0.000, 0.022, 0.0095, 2.4), (0.040, 0.001, 0.000, 0.026, 0.0085, 2.6), (0.092, 0.000, 0.002, 0.034, 0.0078, 2.8)]
                rays = [(sx, t, n, 0.032 if i < 2 else 0.021, 0.0076, 2.8) for i, (sx, t, n, _hw, _ht) in enumerate(SLABS[side])]
                t = [grip(p, side) for p in THUMBS[side]]
                out += [
                    st(frame_loft("metacarpus" + s, bone, SKIN, BONE, hand_rows(side, plate)), bone),
                    st(frame_loft("finger-bones" + s, bone, SKIN, BONE, hand_rows(side, rays)), bone),
                    st(capsule("thumb-bones" + s + "-0", bone, SKIN, BONE, grip(on_hand(0.018, 0.016, 0.0), side), t[1], 0.0078, 0.0078), bone),
                    st(capsule("thumb-bones" + s + "-1", bone, SKIN, BONE, t[1], t[2], 0.0078, 0.0076), bone),
                ]
            elif scale == 200:
                rows = [(0.004, 0.000, 0.000, 0.022, 0.0152, 2.4), (0.080, 0.003, MASSES[200][side][2][2], 0.034, 0.0152, 2.6), (0.162, 0.004, MASSES[200][side][3][2], 0.020, 0.0152, 2.6)]
                out.append(st(frame_loft("hand-bones" + s, bone, SKIN, BONE, hand_rows(side, rows)), bone))
            else:
                rows = [(0.000, 0.000, 0.000, 0.025, 0.0246, 2.4), (0.075, 0.004, 0.000, 0.037, 0.0246, 2.6), (0.158, 0.004, 0.000, 0.025, 0.0246, 2.6)]
                out.append(st(frame_loft("hand-bones" + s, bone, SKIN, BONE, hand_rows(side, rows)), bone))
    return out


def phone_member(scale):
    if scale < 500:
        return [{"id": "phone-frame", "of": "phone", "member": "phone", "part": "member"}]
    return [st(rounded_plate("phone-frame", "phone", GLASS, BONE, 0.0, PHONE_LENGTH, 0.0375, 0.010, -FLOOR, FLOOR), "phone")]


def thermal_parts(scale):
    """🌡 Thermal solids of hair tail, hands, headphones and phone at a coarser scale: references to the body solids where they exist; where the scale no longer models the part in the body (headphone band and phone at 1:500, headphones at 1:500), a solid of its own gives the region its place."""
    out = [ref("ponytail", "hair")] if scale in TAILS else []
    if scale == 100:
        out += [ref(i, "hand-skin") for i in ("hand-l", "hand-r", "fingers-l", "fingers-r", "thumb-l-0", "thumb-l-1", "thumb-r-0", "thumb-r-1")]
        out += [ref(i, "headphones") for i in ("headphone-band-l", "headphone-band-r", "headphone-cup-l", "headphone-cup-r")]
    else:
        out += [ref(i, "hand-skin") for i in ("hand-l", "hand-r")]
    if scale == 200:
        out += [ref(i, "headphones") for i in ("headphone-cup-l", "headphone-cup-r")]
    if scale == 500:
        for side, sign in (("l", 1), ("r", -1)):
            a, b, along = cup_frame(sign)
            out.append(th(ellipsoid_axes("th-headphone-cup-" + side, "torso", POLYMER, 0.85, along(0.001), a, b, (0.043, 0.046, FLOOR)), "headphones"))
        out.append(th(rounded_plate("th-phone", "phone", GLASS, 0.6, 0.0, PHONE_LENGTH, 0.0375, 0.010, -FLOOR, FLOOR), "phone"))
    else:
        out.append(ref("phone", "phone"))
    return out
