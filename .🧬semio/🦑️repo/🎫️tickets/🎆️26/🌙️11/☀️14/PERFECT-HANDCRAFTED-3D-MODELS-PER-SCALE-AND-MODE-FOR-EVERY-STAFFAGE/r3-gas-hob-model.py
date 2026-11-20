"""🍳 Round 3 domino gas hob: states every solid of every authored scale and list by hand (body, thermal, structure at 1:50, 1:100, 1:200) from the measures of `r3-gas-hob-kit.py` and writes `volumes/appliances/gas-hob.json`; the decisions come from `r3-gas-hob-texts.json`."""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-gas-hob-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
SOURCE = os.path.join(HERE, "volumes", "appliances", "gas-hob.json")
TEXTS = os.path.join(HERE, "r3-gas-hob-texts.json")

ST = "steel-enamel"
BR = "brass"
CI = "cast-iron"
BK = "bakelite"
FL = "flame"
RB = "rubber"
BY = K.BURNER_Y
FY = K.FLAME_Y
TY = K.TIP_Y
TRY = K.TRIVET_Y
KX, KY = K.KNOB_X, K.KNOB_Y
BZ = K.BURNER_Z


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def th(s, region, sid=None, faces=None, tags=None):
    t = dict(s)
    t["id"] = sid or "th-" + s["id"]
    t["region"] = region
    if faces:
        t["sectionFaces"] = faces
    if tags:
        t["tags"] = tags
    return t


def st(s, member, part, sid=None, anchor=None):
    t = dict(s)
    t["id"] = sid or s["id"]
    t["member"] = member
    t["part"] = part
    if anchor:
        t["anchor"] = anchor
    return t


def plate_outline(r=K.PLATE_R, z0=K.PLATE_Z0, z1=K.PLATE_Z1, x=K.PLATE_X):
    return K.rounded_rect(-x, x, z0, z1, r)


def housing_outline(x=K.HOUSING_X, z0=K.HOUSING_Z0, z1=K.HOUSING_Z1, r=K.HOUSING_R):
    return K.rounded_rect(-x, x, z0, z1, r)


def trivet_frame(sid, bar, lo, hi, r_out=0.012, tone=0.45):
    h = K.TRIVET_HALF
    return K.extrude(sid, "trivet", CI, "body", tone, K.rounded_rect(-h, h, BZ - h, BZ + h, r_out), "zx", lo, hi, holes=[K.rounded_rect(-h + bar, h - bar, BZ - h + bar, BZ + h - bar, max(r_out - bar, 0.002))])


def trivet_legs(sid, z, side, top, tone=0.45):
    c = K.TRIVET_HALF - side / 2
    return K.box(sid, "trivet", CI, "body", tone, (side, top, side), (-c, top / 2, z), corner=0.0015, repeat={"count": 2, "translation": [K.r5(2 * c), 0, 0]})


def tongue(sid, bone, theta0, count, step, v0, path=K.TONGUE, scale=1.0, tone=0.92, role="body"):
    return K.loft(sid, bone, FL, role, tone, K.tongue_sections(theta0, path, BZ, v0, scale), repeat=K.ring_repeat(count, step))


def plume(sid, r0, r1, lo, hi):
    return K.revolve(sid, "flame-tip", FL, "detail", 0.95, [(lo, r0), (lo, r1), (hi, r1), (hi, r0)], region="flame", tags=["flame-plume"])


def scale50():
    """🔎 1:50, smallest feature 7.5 mm: every assembly of the real hob."""
    housing = K.extrude("housing", "hob", ST, "body", 0.4, housing_outline(), "zx", K.FOOT_H, K.PLATE_Y0 + 0.001)
    plate = K.extrude("plate", "hob", ST, "body", 0.3, housing_outline(), "zx", K.PLATE_Y0, K.PLATE_Y1, holes=[K.circle_zx(BZ, 0.0, K.WELL_R)])
    rim = K.extrude("plate-rim", "hob", ST, "body", 0.3, plate_outline(), "zx", K.PLATE_Y0, K.PLATE_Y1, holes=[housing_outline()])
    well = K.revolve("burner-well", "hob", ST, "body", 0.3, K.WELL, origin=(0, 0, BZ))
    feet_f = K.revolve("feet-front", "hob", RB, "body", 0.15, [(0, 0), (0, K.FOOT_R), (K.FOOT_H - 0.0015, K.FOOT_R), (K.FOOT_H + 0.0005, K.FOOT_R - 0.002), (K.FOOT_H + 0.0005, 0)], origin=(-K.FEET_X, 0, K.FEET_Z[0]), repeat={"count": 2, "translation": [2 * K.FEET_X, 0, 0]})
    feet_b = dict(feet_f, id="feet-back", origin=[-K.FEET_X, 0, K.FEET_Z[1]])
    zf = K.HOUSING_Z0 - 0.0003
    slots_f = K.box("vent-slots", "hob", ST, "detail", 0.15, (0.003, 0.008, 0.0008), (-0.11, 0.016, zf), texture=True, repeat={"count": 9, "translation": [0.02, 0, 0]})
    slots_r = K.box("vent-slots-right", "hob", ST, "detail", 0.15, (0.0008, 0.008, 0.003), (K.HOUSING_X + 0.0003, 0.016, -0.2), texture=True, repeat={"count": 11, "translation": [0, 0, 0.04]})
    slots_l = dict(slots_r, id="vent-slots-left", transform={"translation": [-K.HOUSING_X - 0.0003, 0.016, -0.2]})
    bead = [
        K.capsule("bead-front", "hob", ST, "detail", 0.3, (-K.HOUSING_X + 0.004, K.BEAD_Y, zf), (K.HOUSING_X - 0.004, K.BEAD_Y, zf), 0.0008, texture=True),
        K.capsule("bead-back", "hob", ST, "detail", 0.3, (-K.HOUSING_X + 0.004, K.BEAD_Y, K.HOUSING_Z1 + 0.0003), (K.HOUSING_X - 0.004, K.BEAD_Y, K.HOUSING_Z1 + 0.0003), 0.0008, texture=True),
        K.capsule("bead-right", "hob", ST, "detail", 0.3, (K.HOUSING_X + 0.0003, K.BEAD_Y, K.HOUSING_Z0 + 0.004), (K.HOUSING_X + 0.0003, K.BEAD_Y, K.HOUSING_Z1 - 0.004), 0.0008, texture=True),
        K.capsule("bead-left", "hob", ST, "detail", 0.3, (-K.HOUSING_X - 0.0003, K.BEAD_Y, K.HOUSING_Z0 + 0.004), (-K.HOUSING_X - 0.0003, K.BEAD_Y, K.HOUSING_Z1 - 0.004), 0.0008, texture=True),
    ]
    cup = K.revolve("burner-ring", "burner", BR, "body", 0.5, K.CUP)
    cap = K.revolve("burner-cap", "burner", BR, "body", 0.55, K.CAP)
    ports = K.box("burner-ports", "burner", BR, "detail", 0.15, (0.0012, 0.005, 0.0024), (K.CUP[3][1] + 0.0003, 0.0075, BZ), texture=True, repeat=K.ring_repeat(K.PORT_N, 360.0 / K.PORT_N))
    ring = K.revolve("flame-ring", "flame", FL, "body", 0.92, K.FLAME_RING)
    right = tongue("flame-right", "flame-body", 75.0, 11, 15.0, 0.0)
    left = tongue("flame-left", "flame-body", 255.0, 11, 15.0, 0.0)
    centre_path = [(0.0442, -0.0045, 0.0034, 0.004)] + K.TONGUE
    centre = tongue("flame-centre", "flame-tip", 90.0, 2, 180.0, FY - TY, path=centre_path, tone=0.97)
    knob = K.revolve("knob", "knob", BK, "body", 0.6, K.KNOB, origin=(0, 0, K.KNOB_FACE - 0.0), axis=(0, 0, -1))
    rw, rh, rd = K.RIDGE
    zr = K.KNOB_FACE - K.KNOB[-1][0] - rd / 2 + 0.0005
    ridge = K.box("knob-ridge", "knob", BK, "body", 0.6, (rw, rh, rd), (0, 0, zr), corner=0.002)
    pointer = K.box("knob-pointer", "knob", BK, "detail", 0.95, (0.0012, 0.0065, 0.0004), (0, 0.0068, zr - rd / 2 - 0.0001), texture=True)
    frame = trivet_frame("trivet-rail", K.TRIVET_BAR, K.TRIVET_TOP - K.TRIVET_BAR, K.TRIVET_TOP)
    legs_f = trivet_legs("trivet-legs-front", BZ - K.TRIVET_HALF + K.TRIVET_LEG / 2, K.TRIVET_LEG, K.TRIVET_TOP - K.TRIVET_BAR + 0.0005)
    legs_b = trivet_legs("trivet-legs-back", BZ + K.TRIVET_HALF - K.TRIVET_LEG / 2, K.TRIVET_LEG, K.TRIVET_TOP - K.TRIVET_BAR + 0.0005)
    fingers = K.extrude("trivet-fingers", "trivet", CI, "body", 0.45, [(x, y) for x, y in K.FINGER], "xy", BZ - K.TRIVET_BAR / 2, BZ + K.TRIVET_BAR / 2, repeat=K.ring_repeat(4, 90.0))
    body = [housing, feet_f, feet_b, plate, rim, well] + bead + [slots_f, slots_r, slots_l, cup, cap, ports, ring, right, left, centre, knob, ridge, pointer, frame, legs_f, legs_b, fingers]

    thermal = [
        ref("th-plate", "plate", region="plate", sectionFaces=["-v"]),
        ref("th-plate-rim", "plate-rim", region="plate"),
        ref("th-plate-well", "burner-well", region="plate", sectionFaces=["-v"]),
        ref("th-knob", "knob", region="knob", sectionFaces=["+w"]),
        ref("th-knob-ridge", "knob-ridge", region="knob"),
        ref("th-burner-ring", "burner-ring", region="burner", sectionFaces=["-v"]),
        ref("th-burner-cap", "burner-cap", region="burner"),
        ref("th-flame-ring", "flame-ring", region="flame"),
        ref("th-flame-right", "flame-right", region="flame"),
        ref("th-flame-left", "flame-left", region="flame"),
        ref("th-flame-centre", "flame-centre", region="flame"),
        plume("th-flame-plume", 0.036, 0.062, FY + 0.0185 - TY, FY + 0.0265 - TY),
        ref("th-trivet-rail", "trivet-rail", region="trivet"),
        ref("th-trivet-legs-front", "trivet-legs-front", region="trivet"),
        ref("th-trivet-legs-back", "trivet-legs-back", region="trivet"),
        ref("th-trivet-fingers", "trivet-fingers", region="trivet"),
    ]

    tray = K.extrude("member-housing", "hob", ST, "body", 0.4, housing_outline(), "zx", K.FOOT_H, K.PLATE_Y0 + 0.001, holes=[housing_outline(K.HOUSING_X - 0.0008, K.HOUSING_Z0 + 0.0008, K.HOUSING_Z1 - 0.0008, K.HOUSING_R - 0.0008)])
    pad = K.extrude("support-base-contact", "hob", RB, "body", 0.3, K.rounded_rect(-0.13, 0.13, -0.22, 0.235, 0.01), "zx", 0.0, K.FOOT_H, holes=[K.rounded_rect(-0.11, 0.11, -0.2, 0.215, 0.004)])
    seat = K.revolve("joint-burner", "burner", BR, "body", 0.45, [(-0.0103, 0.0335), (-0.0103, 0.0395), (-0.002, 0.0395), (-0.002, 0.0335)])
    spindle = K.revolve("member-knob-spindle", "knob", BR, "body", 0.45, [(0, 0), (0, 0.00375), (0.0365, 0.00375), (0.0365, 0)], origin=(0, 0, K.KNOB_FACE - 0.004), axis=(0, 0, 1))
    tap = K.box("joint-knob", "knob", BR, "body", 0.45, (0.022, 0.018, 0.026), (0, 0, K.KNOB_FACE + 0.045), corner=0.003)
    pipe = K.capsule("member-gas-pipe", "hob", "steel-enamel", "body", 0.4, (KX, KY, K.KNOB_FACE + 0.058), (0.0, 0.016, BZ), 0.004, slender=True)
    seats_f = K.box("joint-trivet-legs-front", "trivet", CI, "body", 0.45, (0.0095, 0.0075, 0.0095), (-(K.TRIVET_HALF - K.TRIVET_LEG / 2), 0.00375, BZ - K.TRIVET_HALF + K.TRIVET_LEG / 2), repeat={"count": 2, "translation": [K.r5(2 * (K.TRIVET_HALF - K.TRIVET_LEG / 2)), 0, 0]})
    seats_b = dict(seats_f, id="joint-trivet-legs-back", transform={"translation": [-(K.TRIVET_HALF - K.TRIVET_LEG / 2), 0.00375, BZ + K.TRIVET_HALF - K.TRIVET_LEG / 2]})
    structure = [
        st(tray, "hob", "member"),
        ref("member-plate", "plate", member="hob", part="member"),
        ref("member-plate-rim", "plate-rim", member="hob", part="member"),
        ref("member-burner-well", "burner-well", member="hob", part="member"),
        ref("member-feet-front", "feet-front", member="hob", part="member"),
        ref("member-feet-back", "feet-back", member="hob", part="member"),
        st(pipe, "hob", "member"),
        st(pad, "hob", "support", anchor="base-contact"),
        st(K.ellipsoid("mass-hob", "hob", ST, "detail", 0.4, (0.02, 0.009, 0.02), (0, 0.02, 0.007)), "hob", "mass"),
        ref("member-burner", "burner-ring", member="burner", part="member"),
        st(seat, "burner", "joint"),
        st(K.ellipsoid("mass-burner", "burner", BR, "detail", 0.45, (0.0085, 0.0085, 0.0085), (0, 0.007, BZ)), "burner", "mass"),
        st(spindle, "knob", "member"),
        st(tap, "knob", "joint"),
        ref("member-trivet-rail", "trivet-rail", member="trivet", part="member"),
        ref("member-trivet-legs-front", "trivet-legs-front", member="trivet", part="member"),
        ref("member-trivet-legs-back", "trivet-legs-back", member="trivet", part="member"),
        ref("member-trivet-fingers", "trivet-fingers", member="trivet", part="member"),
        st(seats_f, "trivet", "joint"),
        st(seats_b, "trivet", "joint"),
        st(K.ellipsoid("mass-trivet", "trivet", CI, "detail", 0.3, (0.0075, 0.0075, 0.0075), (K.TRIVET_HALF - K.TRIVET_BAR / 2, K.TRIVET_TOP - K.TRIVET_BAR / 2, BZ), repeat=K.ring_repeat(4, 90.0)), "trivet", "mass"),
        ref("member-flame", "flame-ring", member="flame", part="member"),
        ref("member-flame-right", "flame-right", member="flame-body", part="member"),
        ref("member-flame-left", "flame-left", member="flame-body", part="member"),
        ref("member-flame-tip", "flame-centre", member="flame-tip", part="member"),
        st(K.ellipsoid("joint-flame-body", "flame-body", FL, "detail", 0.45, (0.0075, 0.0075, 0.0075), (0, 0.0035, BZ)), "flame-body", "joint"),
    ]
    return body, thermal, structure


def scale100():
    """🔍 1:100, smallest feature 15 mm: housing and plate one rounded block, knob without ridge, burner one turned puck, twelve 15 mm flame tongues, pan support of 12 mm bars."""
    block = K.extrude("plate", "hob", ST, "body", 0.35, plate_outline(), "zx", 0.0, K.PLATE_Y1)
    knob = K.revolve("knob", "knob", BK, "body", 0.6, [(0, 0), (0, 0.013), (0.0215, 0.013), (0.0235, 0.011), (0.0235, 0)], origin=(0, 0, K.KNOB_FACE), axis=(0, 0, -1))
    burner = K.revolve("burner", "burner", BR, "body", 0.5, [(-0.002, 0), (-0.002, 0.046), (0.0165, 0.047), (0.0195, 0.03), (0.02, 0)])
    path = [(0.046, 0.002, 0.0075, 0.0075), (0.052, 0.009, 0.0075, 0.0075), (0.0555, 0.0165, 0.0055, 0.005), (0.056, 0.0235, 0.0012, 0.0012)]
    crown = tongue("flame-crown", "flame-body", 0.0, 12, 30.0, 0.0, path=path)
    frame = trivet_frame("trivet-rail", 0.01, 0.036, K.TRIVET_TOP, r_out=0.013, tone=0.3)
    legs_f = trivet_legs("trivet-legs-front", BZ - K.TRIVET_HALF + 0.005, 0.01, 0.0365, tone=0.3)
    legs_b = trivet_legs("trivet-legs-back", BZ + K.TRIVET_HALF - 0.005, 0.01, 0.0365, tone=0.3)
    fingers = K.box("trivet-fingers", "trivet", CI, "body", 0.3, (0.0635, 0.01, 0.01), (0.0955, 0.043, BZ), repeat=K.ring_repeat(4, 90.0))
    body = [block, knob, burner, crown, frame, legs_f, legs_b, fingers]
    thermal = [
        th(K.extrude("plate", "hob", ST, "body", 0.35, plate_outline(), "zx", 0.025, K.PLATE_Y1), "plate", faces=["-v"]),
        ref("th-knob", "knob", region="knob", sectionFaces=["+w"]),
        ref("th-burner", "burner", region="burner", sectionFaces=["-v"]),
        ref("th-flame-crown", "flame-crown", region="flame"),
        plume("th-flame-plume", 0.035, 0.062, FY + 0.016 - TY, FY + 0.031 - TY),
        ref("th-trivet-rail", "trivet-rail", region="trivet"),
        ref("th-trivet-legs-front", "trivet-legs-front", region="trivet"),
        ref("th-trivet-legs-back", "trivet-legs-back", region="trivet"),
        ref("th-trivet-fingers", "trivet-fingers", region="trivet"),
    ]
    tray = K.extrude("member-housing", "hob", ST, "body", 0.4, plate_outline(), "zx", 0.0, 0.025, holes=[plate_outline(K.PLATE_R - 0.0008, K.PLATE_Z0 + 0.0008, K.PLATE_Z1 - 0.0008, K.PLATE_X - 0.0008)])
    structure = [
        st(tray, "hob", "member"),
        st(K.extrude("member-plate", "hob", ST, "body", 0.3, plate_outline(), "zx", 0.025, K.PLATE_Y1, holes=[K.circle_zx(BZ, 0.0, 0.046, 32)]), "hob", "member"),
        st(K.extrude("support-base-contact", "hob", RB, "body", 0.3, K.rounded_rect(-0.135, 0.135, -0.225, 0.24, 0.012), "zx", 0.0, 0.015, holes=[K.rounded_rect(-0.105, 0.105, -0.195, 0.21, 0.004)]), "hob", "support", anchor="base-contact"),
        st(K.ellipsoid("mass-hob", "hob", ST, "detail", 0.4, (0.02, 0.0095, 0.02), (0, 0.0145, 0.007)), "hob", "mass"),
        ref("member-burner", "burner", member="burner", part="member"),
        st(K.revolve("joint-burner", "burner", BR, "body", 0.45, [(-0.015, 0.03), (-0.015, 0.046), (0.0, 0.046), (0.0, 0.03)]), "burner", "joint"),
        st(K.revolve("member-knob-spindle", "knob", BR, "body", 0.45, [(0, 0), (0, 0.0075), (0.04, 0.0075), (0.04, 0)], origin=(0, 0, K.KNOB_FACE - 0.004), axis=(0, 0, 1)), "knob", "member"),
        st(K.box("joint-knob", "knob", BR, "body", 0.45, (0.024, 0.018, 0.03), (0, 0, K.KNOB_FACE + 0.05), corner=0.003), "knob", "joint"),
        ref("member-trivet-rail", "trivet-rail", member="trivet", part="member"),
        ref("member-trivet-legs-front", "trivet-legs-front", member="trivet", part="member"),
        ref("member-trivet-legs-back", "trivet-legs-back", member="trivet", part="member"),
        ref("member-trivet-fingers", "trivet-fingers", member="trivet", part="member"),
        st(K.ellipsoid("mass-trivet", "trivet", CI, "detail", 0.3, (0.0085, 0.0085, 0.0085), (K.TRIVET_HALF - 0.005, 0.042, BZ), repeat=K.ring_repeat(4, 90.0)), "trivet", "mass"),
        st(K.revolve("member-flame", "flame", FL, "body", 0.92, [(-0.0045, 0.035), (-0.0045, 0.05), (0.0105, 0.05), (0.0105, 0.035)]), "flame", "member"),
        ref("member-flame-crown", "flame-crown", member="flame-body", part="member"),
        st(tongue("member-flame-tip", "flame-tip", 90.0, 2, 180.0, FY - TY, path=path), "flame-tip", "member"),
    ]
    return body, thermal, structure


def scale200():
    """🗺️ 1:200, smallest feature 30 mm: the hob is one rounded block, the burner a 30 mm puck, the flame a crown, the pan support a 30 mm frame on four posts."""
    block = K.extrude("plate", "hob", ST, "body", 0.35, plate_outline(0.012), "zx", 0.0, K.PLATE_Y1)
    burner = K.revolve("burner", "burner", BR, "body", 0.5, [(-0.002, 0), (-0.002, 0.05), (0.022, 0.05), (0.03, 0.04), (0.031, 0)])
    crown = K.revolve("flame-crown", "flame-body", FL, "body", 0.92, [(0.0, 0.0), (0.0, 0.062), (0.01, 0.062), (0.019, 0.052), (0.0255, 0.034), (0.028, 0.0)])
    frame = trivet_frame("trivet", 0.015, 0.033, K.TRIVET_TOP, r_out=0.018, tone=0.3)
    posts_f = trivet_legs("trivet-posts-front", BZ - K.TRIVET_HALF + 0.0075, 0.015, K.TRIVET_TOP, tone=0.3)
    posts_b = trivet_legs("trivet-posts-back", BZ + K.TRIVET_HALF - 0.0075, 0.015, K.TRIVET_TOP, tone=0.3)
    knob = K.revolve("knob", "knob", BK, "body", 0.6, [(-0.004, 0), (-0.004, 0.015), (0.026, 0.015), (0.026, 0)], origin=(0, 0, K.KNOB_FACE), axis=(0, 0, -1))
    body = [block, knob, burner, crown, frame, posts_f, posts_b]
    thermal = [
        th(K.extrude("plate", "hob", ST, "body", 0.35, plate_outline(0.012), "zx", 0.01, K.PLATE_Y1), "plate", faces=["-v"]),
        ref("th-knob", "knob", region="knob", sectionFaces=["+w"]),
        ref("th-burner", "burner", region="burner", sectionFaces=["-v"]),
        ref("th-flame-crown", "flame-crown", region="flame"),
        plume("th-flame-plume", 0.032, 0.062, FY + 0.01 - TY, FY + 0.04 - TY),
        ref("th-trivet", "trivet", region="trivet"),
        ref("th-trivet-posts-front", "trivet-posts-front", region="trivet"),
        ref("th-trivet-posts-back", "trivet-posts-back", region="trivet"),
    ]
    walls = K.extrude("member-housing", "hob", ST, "body", 0.4, plate_outline(0.012), "zx", 0.0, K.PLATE_Y1, holes=[K.rounded_rect(-K.PLATE_X + 0.03, K.PLATE_X - 0.03, K.PLATE_Z0 + 0.03, K.PLATE_Z1 - 0.03, 0.004)])
    structure = [
        st(walls, "hob", "member"),
        st(K.box("member-plate", "hob", ST, "body", 0.3, (2 * K.PLATE_X - 0.06, 0.03, K.PLATE_Z1 - K.PLATE_Z0 - 0.06), (0, 0.025, (K.PLATE_Z0 + K.PLATE_Z1) / 2)), "hob", "member"),
        st(K.extrude("support-base-contact", "hob", RB, "body", 0.3, K.rounded_rect(-0.14, 0.14, -0.226, 0.25, 0.012), "zx", 0.0, 0.03, holes=[K.rounded_rect(-0.1, 0.1, -0.186, 0.21, 0.004)]), "hob", "support", anchor="base-contact"),
        st(K.ellipsoid("mass-hob", "hob", ST, "detail", 0.4, (0.03, 0.015, 0.03), (0, 0.02, 0.007)), "hob", "mass"),
        ref("member-burner", "burner", member="burner", part="member"),
        st(K.revolve("member-knob-spindle", "knob", BR, "body", 0.45, [(0, 0), (0, 0.015), (0.06, 0.015), (0.06, 0)], origin=(0, 0, K.KNOB_FACE - 0.02), axis=(0, 0, 1)), "knob", "member"),
        ref("member-trivet", "trivet", member="trivet", part="member"),
        ref("member-trivet-posts-front", "trivet-posts-front", member="trivet", part="member"),
        ref("member-trivet-posts-back", "trivet-posts-back", member="trivet", part="member"),
        st(K.ellipsoid("mass-trivet", "trivet", CI, "detail", 0.3, (0.015, 0.015, 0.015), (K.TRIVET_HALF - 0.0075, 0.0405, BZ), repeat=K.ring_repeat(4, 90.0)), "trivet", "mass"),
        st(K.revolve("member-flame", "flame", FL, "body", 0.92, [(-0.012, 0.0), (-0.012, 0.05), (0.018, 0.05), (0.018, 0.0)]), "flame", "member"),
        ref("member-flame-crown", "flame-crown", member="flame-body", part="member"),
        st(K.ellipsoid("member-flame-tip", "flame-tip", FL, "body", 0.92, (0.015, 0.015, 0.015), (0, -0.003, BZ)), "flame-tip", "member"),
    ]
    return body, thermal, structure


def main():
    texts = json.load(open(TEXTS, encoding="utf-8"))["volumes"]
    vol = {}
    for den, fn in (("50", scale50), ("100", scale100), ("200", scale200)):
        b, t, s = fn()
        vol[den] = {"status": "authored", "denominator": int(den), "body": b, "thermal": t, "structure": s, "decisions": texts[den]}
    vol["500"] = {"status": "unavailable", "denominator": 500}
    with open(SOURCE, "w", encoding="utf-8") as f:
        json.dump({"volumes": vol}, f, indent=2, ensure_ascii=False)
        f.write("\n")
    for den in ("50", "100", "200"):
        print(den, {k: len(vol[den][k]) for k in ("body", "thermal", "structure")})


if __name__ == "__main__":
    main()
