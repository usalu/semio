"""🧹 Vacuum cleaner volumes: every solid of body, thermal and structure at 1:50, 1:100, 1:200 and 1:500 stated by hand from the kit's measures, written to the authoring source `volumes/appliances/vacuum-cleaner.json`.

@see ./r3-vacuum-cleaner-kit.py
@see ./design.md
"""
import importlib.util
import json
import math
import sys
from pathlib import Path

_spec = importlib.util.spec_from_file_location("kit", Path(__file__).with_name("r3-vacuum-cleaner-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

RED, GREY, HOSE, STEEL, RUBBER, NYLON = "abs-red", "abs-grey", "hose-pvc", "tube-steel", "rubber-wheel", "nylon-bristle"
SY = K.SOCKET[1]
HEAD = [(-0.035, -0.041), (0.041, -0.041), (0.045, -0.037), (0.0465, -0.030), (0.044, -0.022), (0.036, -0.016), (0.012, -0.013), (-0.018, -0.013), (-0.031, -0.016), (-0.0365, -0.023), (-0.037, -0.034)]
HEAD_100 = [(-0.037, -0.041), (0.043, -0.041), (0.0465, -0.033), (0.043, -0.021), (0.03, -0.014), (-0.02, -0.013), (-0.034, -0.018), (-0.037, -0.03)]
HEAD_200 = [(-0.037, -0.047), (0.0465, -0.047), (0.0465, -0.024), (0.03, -0.013), (-0.025, -0.013), (-0.037, -0.022)]
HEAD_500 = [(-0.04, -0.047), (0.05, -0.047), (0.05, 0.0), (0.03, 0.028), (-0.025, 0.028), (-0.04, 0.0)]
YOKE = [(-0.022, -0.0165), (0.02, -0.0165), (0.018, -0.006)] + [(0.017 * math.cos(math.radians(a)), 0.017 * math.sin(math.radians(a))) for a in range(-20, 201, 20)] + [(-0.02, -0.006)]
SHELL_H = {50: [0.035, 0.037, 0.04, 0.044, 0.048, 0.053, 0.059, 0.066, 0.074, 0.083, 0.095, 0.13, 0.165, 0.175, 0.185], 100: [0.035, 0.04, 0.047, 0.056, 0.068, 0.085, 0.125, 0.185]}
LID_H = {50: [0.185, 0.193, 0.201, 0.209, 0.216, 0.223, 0.229, 0.234, 0.238, 0.241, 0.2435, 0.245], 100: [0.185, 0.205, 0.222, 0.235, 0.242, 0.245]}
CAN_KEYS = K.SHELL_KEYS[:-1] + K.LID_KEYS
CAN_H = {200: [0.035, 0.045, 0.06, 0.085, 0.185, 0.215, 0.235, 0.245], 500: [0.035, 0.055, 0.09, 0.185, 0.23, 0.245]}
TYRE = [(-0.0125, 0.044), (-0.0125, 0.052), (-0.0105, 0.058), (-0.005, 0.06), (0.005, 0.06), (0.0105, 0.058), (0.0125, 0.052), (0.0125, 0.044), (-0.0125, 0.044)]
RIM = [(-0.011, 0.0), (-0.011, 0.0445), (0.0095, 0.0445), (0.0095, 0.04), (0.004, 0.032), (0.004, 0.018), (0.008, 0.0145), (0.008, 0.0)]
CAP = [(0.004, 0.0), (0.004, 0.014), (0.0105, 0.0125), (0.012, 0.0)]
WHEEL_100 = [(-0.0125, 0.0), (-0.0125, 0.052), (-0.0105, 0.058), (-0.005, 0.06), (0.005, 0.06), (0.0105, 0.058), (0.0125, 0.052), (0.0125, 0.0)]
CASTER_TYRE = [(-0.006, 0.0), (-0.006, 0.02), (-0.004, 0.024), (0.0, 0.025), (0.004, 0.024), (0.006, 0.02), (0.006, 0.0)]
RL, RR = [0, 0, 0.0], [0, 1, 0, 0]


def hose_bones():
    return [f"hose-{i + 1}" for i in range(K.HOSE_N)]


def plan_c(y, grow=0.0):
    """📐 Half length, half width and centre of the shell plan section at height y."""
    xr, xf, w = K.plan_at(K.SHELL_KEYS, y)
    return (xf - xr) / 2 + grow, w + grow, (xf + xr) / 2


def plan_ring(y, grow=0.0, n=64):
    """⭕ Plan ring (z, x) of the shell at height y, for extrudes in the zx plane."""
    a, b, c = plan_c(y, grow)
    return [(z, x) for x, z in K.superellipse(a, b, K.PLAN_E, n, c, 0.0)]


def bumper_ring(grow):
    """🛡️ Plan ring (z, x) of the bumper band at the parting line, grown from the shell's top section."""
    return plan_ring(K.PARTING, grow, 72)


def grille_profile():
    return K.rounded_rect(0.074, -0.078, 0.168, 0.078, 0.008)


def grille_slots():
    return [K.rounded_rect(0.0825 + 0.017 * k, -0.068, 0.0915 + 0.017 * k, 0.068, 0.003) for k in range(5)]


def inset(poly, d):
    """↘️ A convex-ish profile pulled towards its centroid by d (wall of a hollow section)."""
    cx = sum(p[0] for p in poly) / len(poly)
    cy = sum(p[1] for p in poly) / len(poly)
    out = []
    for x, y in poly:
        L = math.hypot(x - cx, y - cy)
        out.append((x - (x - cx) / L * d * 1.6, y - (y - cy) / L * d))
    return out


def body_50():
    b = []
    b.append(K.loft("shell", "body", RED, K.stack_sections(K.SHELL_KEYS, SHELL_H[50]), tone=0.5))
    b.append(K.loft("lid", "body", RED, K.stack_sections(K.LID_KEYS, LID_H[50]), tone=0.62))
    b.append(K.extrude("handle", "body", RED, "xy", K.rounded_rect(-0.102, 0.236, 0.042, 0.282, 0.012), -0.016, 0.016, holes=[K.rounded_rect(-0.078, 0.2445, 0.018, 0.2645, 0.006)], tone=0.62))
    for side, z in (("l", 0.055), ("r", -0.055)):
        b.append(K.revolve(f"button-{side}", "body", GREY, [(0.236, 0.0), (0.236, 0.017), (0.244, 0.017), (0.248, 0.014), (0.249, 0.0)], (0, 1, 0), (-0.175, 0, z), tone=0.8))
    b.append(K.extrude("bumper", "body", GREY, "zx", bumper_ring(0.004), *K.BUMPER, holes=[bumper_ring(-0.006)], tone=0.25))
    b.append(K.revolve("hose-socket", "body", GREY, [(0.238, 0.0245), (0.238, 0.032), (0.258, 0.032), (0.262, 0.0285), (0.262, 0.0245), (0.238, 0.0245)], (1, 0, 0), (0, SY, 0), tone=0.25))
    b.append(K.revolve("cuff", "body", GREY, [(0.244, 0.0), (0.244, 0.0235), (0.268, 0.0235), (0.274, 0.0215), (0.282, 0.0205), (0.282, 0.0)], (1, 0, 0), (0, SY, 0), tone=0.3))
    b.append(K.extrude("exhaust-grille", "body", GREY, "yz", grille_profile(), -0.256, -0.245, holes=grille_slots(), tone=0.25))
    b.append(K.revolve("rear-tyre-l", "wheel", RUBBER, TYRE, (0, 0, 1), transform={"translation": [0, 0, K.WHEEL_Z]}, tone=0.2))
    b.append(K.revolve("rear-tyre-r", "wheel", RUBBER, TYRE, (0, 0, 1), transform={"translation": [0, 0, -K.WHEEL_Z], "rotation": RR}, tone=0.2))
    b.append(K.revolve("rear-rim-l", "wheel", GREY, RIM, (0, 0, 1), transform={"translation": [0, 0, K.WHEEL_Z]}, tone=0.6))
    b.append(K.revolve("rear-rim-r", "wheel", GREY, RIM, (0, 0, 1), transform={"translation": [0, 0, -K.WHEEL_Z], "rotation": RR}, tone=0.6))
    b.append(K.revolve("rear-hub-l", "wheel", GREY, CAP, (0, 0, 1), transform={"translation": [0, 0, K.WHEEL_Z]}, tone=0.8))
    b.append(K.revolve("rear-hub-r", "wheel", GREY, CAP, (0, 0, 1), transform={"translation": [0, 0, -K.WHEEL_Z], "rotation": RR}, tone=0.8))
    b.append(K.revolve("caster-wheel-l", "caster", RUBBER, CASTER_TYRE, (0, 0, 1), transform={"translation": [0, 0, 0.018]}, tone=0.2))
    b.append(K.revolve("caster-wheel-r", "caster", RUBBER, CASTER_TYRE, (0, 0, 1), transform={"translation": [0, 0, -0.018]}, tone=0.2))
    b.append(K.box("caster-fork", "caster", GREY, (0, 0.004, 0), (0.036, 0.026, 0.026), 0.005, tone=0.35))
    for h in hose_bones():
        b.append(K.capsule(f"{h}-tube", h, HOSE, (0, 0, 0), (K.HOSE_SEG, 0, 0), K.HOSE_R, tone=0.35))
        b.append(K.revolve(f"{h}-rib", h, HOSE, [(0.0135, 0.0172), (0.0135, 0.0195), (0.0165, 0.0195), (0.0165, 0.0172), (0.0135, 0.0172)], (1, 0, 0), role="texture", tone=0.15, texture=True, repeat={"count": 7, "translation": [0.02, 0, 0]}))
    b.append(K.revolve("hose-end", "hose-10", GREY, [(0.105, 0.0), (0.105, 0.0195), (0.112, 0.021), (0.153, 0.021), (0.153, 0.0)], (1, 0, 0), tone=0.3))
    b.append(K.ellipsoid("grip-swivel", "grip", GREY, (0, 0, 0), (0.02, 0.02, 0.02), tone=0.3))
    b.append(K.loft("grip-sleeve", "grip", GREY, K.axial_loft([(-0.006, 0.010, 0.010), (0.004, 0.019, 0.020), (0.02, 0.0205, 0.022), (0.05, 0.019, 0.0205, -0.002), (0.09, 0.0195, 0.0215, -0.003), (0.13, 0.0195, 0.0215, -0.002), (0.16, 0.0215, 0.023), (0.19, 0.0215, 0.023), (0.20, 0.019, 0.0195), (0.204, 0.012, 0.012)], 2.2), tone=0.3))
    b.append(K.box("air-slide", "grip", GREY, (0.075, 0.021, 0), (0.034, 0.008, 0.016), 0.003, tone=0.7))
    b.append(K.revolve("wand-tube", "wand", STEEL, K.tube_profile(0.0, 0.30, 0.0175), (1, 0, 0), tone=0.8))
    b.append(K.revolve("wand-collar", "wand", GREY, [(0.28, 0.0), (0.28, 0.0205), (0.286, 0.0215), (0.312, 0.0215), (0.318, 0.0195), (0.318, 0.0)], (1, 0, 0), tone=0.3))
    b.append(K.box("wand-lock", "wand", GREY, (0.299, 0.0225, 0), (0.016, 0.008, 0.012), 0.003, tone=0.6))
    b.append(K.revolve("wand-inner", "wand", STEEL, K.tube_profile(0.30, 0.74, 0.016), (1, 0, 0), tone=0.85))
    b.append(K.loft("nozzle-neck", "wand", GREY, K.axial_loft([(0.715, 0.0185, 0.0185), (0.73, 0.0195, 0.0195), (0.78, 0.0195, 0.0195), (0.82, 0.0185, 0.0185), (0.845, 0.016, 0.016), (0.862, 0.011, 0.011)]), tone=0.45))
    b.append(K.extrude("nozzle", "nozzle", GREY, "xy", HEAD, -0.1325, 0.1325, tone=0.45))
    b.append(K.extrude("nozzle-cap-l", "nozzle", RUBBER, "xy", K.offset(HEAD, 0.0015), 0.1315, K.NOZZLE_HALF, tone=0.2))
    b.append(K.extrude("nozzle-cap-r", "nozzle", RUBBER, "xy", K.offset(HEAD, 0.0015), -K.NOZZLE_HALF, -0.1315, tone=0.2))
    b.append(K.extrude("nozzle-yoke-l", "nozzle", GREY, "xy", YOKE, 0.021, 0.031, tone=0.45))
    b.append(K.extrude("nozzle-yoke-r", "nozzle", GREY, "xy", YOKE, -0.031, -0.021, tone=0.45))
    b.append(K.ellipsoid("swivel", "nozzle", GREY, (0, 0, 0), (0.02, 0.02, 0.02), tone=0.5))
    b.append(K.box("brush", "nozzle", NYLON, (0.038, -0.043, 0), (0.006, 0.008, 0.25), tone=0.15))
    b.append(K.box("brush-rear", "nozzle", NYLON, (-0.029, -0.043, 0), (0.006, 0.008, 0.25), tone=0.15))
    return b


def emitter(t):
    return K.extrude("th-exhaust-air", "body", GREY, "yz", K.circle(0.045, 0.121, 0.0, 32), -0.258, -0.256, role="detail", tone=0.5, region="exhaust", layer="grille", tags=["exhaust-air"])


def lid_skin(heights):
    return K.thermal(K.loft("th-lid", "body", RED, K.stack_sections(K.LID_KEYS, heights), caps=(False, True), tone=0.62), "lid", "lid")


def grille_plate(x0=-0.256, x1=-0.245):
    return K.thermal(K.extrude("th-grille", "body", GREY, "yz", grille_profile(), x0, x1, tone=0.25), "exhaust", "grille", sectionFaces=["+u"])


def thermal_50(body):
    t = [K.ref("th-shell", "shell", region="body-shell", layer="shell"), K.ref("th-bumper", "bumper", region="body-shell", layer="shell"), K.ref("th-socket", "hose-socket", region="body-shell", layer="shell")]
    t += [lid_skin(LID_H[50]), K.ref("th-handle", "handle", region="lid", layer="lid"), K.ref("th-button-l", "button-l", region="lid", layer="lid"), K.ref("th-button-r", "button-r", region="lid", layer="lid")]
    t += [grille_plate(), emitter(50)]
    t += [K.ref(f"th-{s}", s, region="wheels", layer="tyre") for s in ("rear-tyre-l", "rear-tyre-r", "rear-rim-l", "rear-rim-r", "caster-wheel-l", "caster-wheel-r")]
    t += [K.ref(f"th-{h}", f"{h}-tube", region="hose", layer="hose") for h in hose_bones()] + [K.ref("th-cuff", "cuff", region="hose", layer="hose"), K.ref("th-hose-end", "hose-end", region="hose", layer="hose")]
    t += [K.ref(f"th-{s}", s, region="wand", layer="tube") for s in ("wand-tube", "wand-collar", "wand-lock", "wand-inner")]
    t += [K.ref(f"th-{s}", s, region="grip", layer="sleeve") for s in ("grip-sleeve", "grip-swivel", "air-slide")]
    t += [K.ref(f"th-{s}", s, region="nozzle", layer="noz") for s in ("nozzle", "nozzle-cap-l", "nozzle-cap-r", "nozzle-yoke-l", "nozzle-yoke-r", "swivel", "nozzle-neck")]
    t += [K.ref("th-brush", "brush", region="brush", layer="bristle"), K.ref("th-brush-rear", "brush-rear", region="brush", layer="bristle")]
    return t


def tube_member(sid, bone, mat, a0, a1, r, wall, part="member"):
    return K.member(K.extrude(sid, bone, mat, "yz", K.circle(r, 0, 0, 24), a0, a1, holes=[K.circle(r - wall, 0, 0, 24)], tone=0.5), bone, part)


def chassis(scale):
    """🏗️ The canister's load path: base tub moulding and its 3 mm side wall (hollow)."""
    s = []
    hs = {50: [0.037, 0.042, 0.05, 0.06, 0.07, 0.08], 100: [0.037, 0.045, 0.06, 0.08], 200: [0.037, 0.05, 0.068], 500: None}[scale]
    if hs:
        s.append(K.member(K.loft("chassis-base", "body", RED, K.stack_sections(K.SHELL_KEYS, hs, grow=-0.0015), tone=0.5), "body", "member"))
    y0 = 0.08 if hs else 0.04
    s.append(K.member(K.extrude("chassis-wall", "body", RED, "zx", plan_ring(0.12, -0.0015), y0, 0.183, holes=[plan_ring(0.12, -0.0045)], tone=0.5), "body", "member"))
    return s


def masses(scale):
    f = K.FEATURE[scale]
    q = lambda r: max(r, f / 2 + 0.0005)
    return [
        K.member(K.ellipsoid("mass-body", "body", RED, (-0.05, 0.10, 0), (q(0.03), q(0.03), q(0.03)), role="detail", tone=0.15), "body", "mass"),
        K.member(K.ellipsoid("mass-wand", "wand", STEEL, (0.40, 0, 0), (q(0.03), q(0.016), q(0.016)), role="detail", tone=0.15), "wand", "mass"),
        K.member(K.ellipsoid("mass-nozzle", "nozzle", GREY, (0.005, -0.025, 0), (q(0.02), q(0.012), q(0.04)), role="detail", tone=0.15), "nozzle", "mass"),
        K.member(K.ellipsoid("mass-grip", "grip", GREY, (0.10, 0, 0), (q(0.025), q(0.012), q(0.012)), role="detail", tone=0.15), "grip", "mass"),
    ]


def supports(scale):
    f = K.FEATURE[scale]
    h = max(0.008, f)
    return [
        K.member(K.box("support-wheel-contact", "wheel", RUBBER, (0, -K.WHEEL_R + h / 2, 0), (max(0.02, f), h, 0.30), role="ground", tone=0.1), "wheel", "support", anchor="wheel-contact"),
        K.member(K.box("support-caster-contact", "caster", RUBBER, (0, -K.CASTER_R + h / 2, 0), (max(0.02, f), h, max(0.05, f)), role="ground", tone=0.1), "caster", "support", anchor="caster-contact"),
        K.member(K.box("support-nozzle-contact", "nozzle", NYLON, (0.0045, K.NOZZLE_FLOOR + h / 2, 0), (max(0.075, f), h, 0.25), role="ground", tone=0.1), "nozzle", "support", anchor="nozzle-contact"),
    ]


def structure_50():
    s = chassis(50)
    s.append(K.member(K.capsule("axle", "wheel", STEEL, (0, 0, -0.15), (0, 0, 0.15), 0.005, tone=0.5, slender=True), "wheel", "member"))
    for side, z in (("l", 0.128), ("r", -0.128)):
        s.append(K.member(K.ellipsoid(f"joint-wheel-{side}", "wheel", GREY, (0, 0, z), (0.012, 0.012, 0.009), role="detail", tone=0.35), "wheel", "joint"))
    s.append(K.member(K.capsule("caster-stem", "caster", STEEL, (0, 0.0, 0), (0, 0.016, 0), 0.0045, tone=0.5), "caster", "member"))
    s.append(K.member(K.capsule("joint-caster", "caster", STEEL, (0, 0, -0.027), (0, 0, 0.027), 0.004, role="detail", tone=0.35), "caster", "joint"))
    for h in hose_bones():
        s.append(tube_member(f"member-{h}", h, HOSE, 0.0, K.HOSE_SEG, K.HOSE_R, 0.0006))
        s.append(K.member(K.ellipsoid(f"joint-{h}", h, HOSE, (0, 0, 0), (0.008, 0.0195, 0.0195), role="detail", tone=0.35), h, "joint"))
    s.append(tube_member("member-grip", "grip", GREY, -0.005, 0.2, 0.019, 0.003))
    s.append(K.member(K.ellipsoid("joint-grip", "grip", GREY, (0, 0, 0), (0.02, 0.02, 0.02), role="detail", tone=0.35), "grip", "joint"))
    s.append(tube_member("member-wand", "wand", STEEL, 0.0, 0.30, 0.0175, 0.0006))
    s.append(tube_member("member-wand-inner", "wand", STEEL, 0.30, 0.74, 0.016, 0.0006))
    s.append(tube_member("member-neck", "wand", GREY, 0.715, 0.86, 0.0185, 0.002))
    s.append(K.member(K.extrude("member-nozzle", "nozzle", GREY, "xy", HEAD, -0.13, 0.13, holes=[inset(HEAD, 0.0025)], tone=0.5), "nozzle", "member"))
    s.append(K.member(K.capsule("joint-nozzle", "nozzle", STEEL, (0, 0, -0.031), (0, 0, 0.031), 0.005, role="detail", tone=0.35), "nozzle", "joint"))
    return s + supports(50) + masses(50)


def body_100():
    b = [K.loft("shell", "body", RED, K.stack_sections(K.SHELL_KEYS, SHELL_H[100]), tone=0.5), K.loft("lid", "body", RED, K.stack_sections(K.LID_KEYS, LID_H[100]), tone=0.62)]
    b.append(K.extrude("handle", "body", RED, "xy", K.rounded_rect(-0.102, 0.236, 0.042, 0.282, 0.014), -0.016, 0.016, tone=0.62))
    b.append(K.revolve("hose-socket", "body", GREY, K.tube_profile(0.238, 0.262, 0.032), (1, 0, 0), (0, SY, 0), tone=0.25))
    b.append(K.extrude("exhaust-grille", "body", GREY, "yz", grille_profile(), -0.26, -0.245, tone=0.25))
    b.append(K.revolve("rear-wheel-l", "wheel", RUBBER, WHEEL_100, (0, 0, 1), transform={"translation": [0, 0, K.WHEEL_Z]}, tone=0.2))
    b.append(K.revolve("rear-wheel-r", "wheel", RUBBER, WHEEL_100, (0, 0, 1), transform={"translation": [0, 0, -K.WHEEL_Z], "rotation": RR}, tone=0.2))
    b.append(K.revolve("caster-wheel", "caster", RUBBER, [(-0.0195, 0.0), (-0.0195, 0.021), (-0.016, 0.025), (0.016, 0.025), (0.0195, 0.021), (0.0195, 0.0)], (0, 0, 1), tone=0.2))
    for h in hose_bones():
        b.append(K.capsule(f"{h}-tube", h, HOSE, (0, 0, 0), (K.HOSE_SEG, 0, 0), K.HOSE_R, tone=0.35))
    b.append(K.loft("grip-sleeve", "grip", GREY, K.axial_loft([(-0.022, 0.011, 0.011), (-0.012, 0.021, 0.021), (0.01, 0.021, 0.022), (0.15, 0.02, 0.0215), (0.195, 0.0215, 0.023), (0.206, 0.012, 0.012)], 2.2), tone=0.3))
    b.append(K.revolve("wand-tube", "wand", STEEL, K.tube_profile(0.0, 0.74, 0.0175), (1, 0, 0), tone=0.8))
    b.append(K.loft("nozzle-neck", "wand", GREY, K.axial_loft([(0.715, 0.019, 0.019), (0.80, 0.0195, 0.0195), (0.862, 0.012, 0.012)]), tone=0.45))
    b.append(K.extrude("nozzle", "nozzle", GREY, "xy", HEAD_100, -K.NOZZLE_HALF, K.NOZZLE_HALF, tone=0.45))
    b.append(K.ellipsoid("swivel", "nozzle", GREY, (0, 0, 0), (0.02, 0.02, 0.02), tone=0.5))
    b.append(K.box("brush", "nozzle", NYLON, (0.038, -0.043, 0), (0.006, 0.008, 0.25), tone=0.15))
    b.append(K.box("brush-rear", "nozzle", NYLON, (-0.029, -0.043, 0), (0.006, 0.008, 0.25), tone=0.15))
    return b


def thermal_100(body):
    t = [K.ref("th-shell", "shell", region="body-shell", layer="shell"), K.ref("th-socket", "hose-socket", region="body-shell", layer="shell")]
    t += [lid_skin(LID_H[100]), K.ref("th-handle", "handle", region="lid", layer="lid")]
    t += [grille_plate(-0.26, -0.245), emitter(100)]
    t += [K.ref(f"th-{s}", s, region="wheels", layer="tyre") for s in ("rear-wheel-l", "rear-wheel-r", "caster-wheel")]
    t += [K.ref(f"th-{h}", f"{h}-tube", region="hose", layer="hose") for h in hose_bones()]
    t += [K.ref("th-wand", "wand-tube", region="wand", layer="tube"), K.ref("th-grip", "grip-sleeve", region="grip", layer="sleeve")]
    t += [K.ref(f"th-{s}", s, region="nozzle", layer="noz") for s in ("nozzle", "swivel", "nozzle-neck")]
    t += [K.ref("th-brush", "brush", region="brush", layer="bristle"), K.ref("th-brush-rear", "brush-rear", region="brush", layer="bristle")]
    return t


def structure_100():
    s = chassis(100)
    s.append(K.member(K.capsule("axle", "wheel", STEEL, (0, 0, -0.15), (0, 0, 0.15), 0.005, tone=0.5, slender=True), "wheel", "member"))
    s.append(K.member(K.ellipsoid("joint-wheel", "wheel", GREY, (0, 0, 0), (0.0155, 0.0155, 0.0155), role="detail", tone=0.35), "wheel", "joint"))
    for h in hose_bones():
        s.append(tube_member(f"member-{h}", h, HOSE, 0.0, K.HOSE_SEG, K.HOSE_R, 0.0006))
    s.append(tube_member("member-grip", "grip", GREY, -0.01, 0.2, 0.019, 0.003))
    s.append(K.member(K.ellipsoid("joint-grip", "grip", GREY, (0, 0, 0), (0.02, 0.02, 0.02), role="detail", tone=0.35), "grip", "joint"))
    s.append(tube_member("member-wand", "wand", STEEL, 0.0, 0.86, 0.0175, 0.0006))
    s.append(K.member(K.extrude("member-nozzle", "nozzle", GREY, "xy", HEAD_100, -0.13, 0.13, holes=[inset(HEAD_100, 0.0025)], tone=0.5), "nozzle", "member"))
    s.append(K.member(K.ellipsoid("joint-nozzle", "nozzle", GREY, (0, 0, 0), (0.016, 0.016, 0.03), role="detail", tone=0.35), "nozzle", "joint"))
    return s + supports(100) + masses(100)


def body_200():
    b = [K.loft("canister", "body", RED, K.stack_sections(CAN_KEYS, CAN_H[200]), tone=0.5)]
    b.append(K.extrude("handle", "body", RED, "xy", K.rounded_rect(-0.102, 0.236, 0.042, 0.282, 0.015), -0.016, 0.016, tone=0.62))
    b.append(K.revolve("hose-socket", "body", GREY, K.tube_profile(0.232, 0.262, 0.032), (1, 0, 0), (0, SY, 0), tone=0.25))
    disc = [(-0.015, 0.0), (-0.015, 0.06), (0.015, 0.06), (0.015, 0.0)]
    b.append(K.revolve("rear-wheel-l", "wheel", RUBBER, disc, (0, 0, 1), transform={"translation": [0, 0, 0.135]}, tone=0.2))
    b.append(K.revolve("rear-wheel-r", "wheel", RUBBER, disc, (0, 0, 1), transform={"translation": [0, 0, -0.135]}, tone=0.2))
    for h in hose_bones():
        b.append(K.capsule(f"{h}-tube", h, HOSE, (0, 0, 0), (K.HOSE_SEG, 0, 0), K.HOSE_R, tone=0.35))
    b.append(K.revolve("grip-sleeve", "grip", GREY, K.tube_profile(-0.02, 0.19, 0.021), (1, 0, 0), tone=0.3))
    b.append(K.revolve("wand-tube", "wand", STEEL, K.tube_profile(0.0, 0.85, 0.0175), (1, 0, 0), tone=0.8))
    b.append(K.extrude("nozzle", "nozzle", GREY, "xy", HEAD_200, -K.NOZZLE_HALF, K.NOZZLE_HALF, tone=0.45))
    b.append(K.ellipsoid("swivel", "nozzle", GREY, (0, 0, 0), (0.02, 0.02, 0.02), tone=0.5))
    return b


def thermal_coarse(scale, body):
    ids = {s["id"] for s in body}
    t = [K.thermal(K.loft("th-shell", "body", RED, K.stack_sections(K.SHELL_KEYS, [0.035, 0.05, 0.085, 0.185] if scale == 200 else [0.035, 0.07, 0.185]), tone=0.5), "body-shell", "shell")]
    t.append(lid_skin([0.185, 0.222, 0.24, 0.245] if scale == 200 else [0.185, 0.235, 0.245]))
    if "handle" in ids:
        t.append(K.ref("th-handle", "handle", region="lid", layer="lid"))
    if "hose-socket" in ids:
        t.append(K.ref("th-socket", "hose-socket", region="body-shell", layer="shell"))
    t += [grille_plate(), emitter(scale)]
    if scale == 200:
        t += [K.ref(f"th-{s}", s, region="wheels", layer="tyre") for s in ("rear-wheel-l", "rear-wheel-r")]
    else:
        t += [K.thermal(K.revolve(f"th-rear-wheel-{sd}", "wheel", RUBBER, WHEEL_100, (0, 0, 1), transform={"translation": [0, 0, z]}, tone=0.2), "wheels", "tyre") for sd, z in (("l", K.WHEEL_Z), ("r", -K.WHEEL_Z))]
    if scale == 200:
        t += [K.ref(f"th-{h}", f"{h}-tube", region="hose", layer="hose") for h in hose_bones()]
        t += [K.ref("th-wand", "wand-tube", region="wand", layer="tube"), K.ref("th-grip", "grip-sleeve", region="grip", layer="sleeve")]
        t += [K.ref("th-nozzle", "nozzle", region="nozzle", layer="noz"), K.ref("th-swivel", "swivel", region="nozzle", layer="noz")]
    else:
        t += [K.thermal(K.capsule(f"th-{h}", h, HOSE, (0, 0, 0), (K.HOSE_SEG, 0, 0), K.HOSE_R, tone=0.35), "hose", "hose") for h in hose_bones()]
        t += [K.thermal(K.revolve("th-wand", "wand", STEEL, K.tube_profile(0.0, 0.85, 0.0175), (1, 0, 0), tone=0.8), "wand", "tube")]
        t += [K.thermal(K.revolve("th-grip", "grip", GREY, K.tube_profile(-0.02, 0.19, 0.021), (1, 0, 0), tone=0.3), "grip", "sleeve")]
        t += [K.thermal(K.extrude("th-nozzle", "nozzle", GREY, "xy", HEAD_200, -K.NOZZLE_HALF, K.NOZZLE_HALF, tone=0.45), "nozzle", "noz")]
    t += [K.thermal(K.box(sid, "nozzle", NYLON, (x, -0.043, 0), (0.006, 0.008, 0.25), tone=0.15), "brush", "bristle") for sid, x in (("th-brush", 0.038), ("th-brush-rear", -0.029))]
    return t


def structure_200():
    s = chassis(200)
    s.append(K.member(K.capsule("axle", "wheel", STEEL, (0, 0, -0.15), (0, 0, 0.15), 0.005, tone=0.5, slender=True), "wheel", "member"))
    for h in hose_bones():
        s.append(tube_member(f"member-{h}", h, HOSE, 0.0, K.HOSE_SEG, K.HOSE_R, 0.0006))
    s.append(tube_member("member-grip", "grip", GREY, -0.01, 0.2, 0.019, 0.003))
    s.append(tube_member("member-wand", "wand", STEEL, 0.0, 0.86, 0.0175, 0.0006))
    s.append(K.member(K.extrude("member-nozzle", "nozzle", GREY, "xy", HEAD_200, -0.13, 0.13, tone=0.5), "nozzle", "member"))
    s.append(K.member(K.ellipsoid("joint-nozzle", "nozzle", GREY, (0, 0, 0), (0.0155, 0.0155, 0.03), role="detail", tone=0.35), "nozzle", "joint"))
    return s + supports(200) + masses(200)


def body_500():
    b = [K.loft("canister", "body", RED, K.stack_sections(CAN_KEYS, CAN_H[500]), tone=0.5)]
    b.append(K.revolve("rear-wheels", "wheel", RUBBER, K.tube_profile(-0.15, 0.15, K.WHEEL_R), (0, 0, 1), tone=0.2))
    for h in hose_bones():
        b.append(K.revolve(f"{h}-tube", h, HOSE, K.tube_profile(-0.012, K.HOSE_SEG + 0.012, 0.03), (1, 0, 0), tone=0.35))
    b.append(K.revolve("grip-sleeve", "grip", GREY, K.tube_profile(-0.012, 0.18, 0.03), (1, 0, 0), tone=0.3))
    b.append(K.revolve("wand-tube", "wand", STEEL, K.tube_profile(0.0, 0.83, 0.03), (1, 0, 0), tone=0.8))
    b.append(K.extrude("nozzle", "nozzle", GREY, "xy", HEAD_500, -K.NOZZLE_HALF, K.NOZZLE_HALF, tone=0.45))
    return b


def structure_500():
    s = chassis(500)
    s.append(K.member(K.capsule("axle", "wheel", STEEL, (0, 0, -0.15), (0, 0, 0.15), 0.005, tone=0.5, slender=True), "wheel", "member"))
    for h in hose_bones():
        s.append(K.member(K.revolve(f"member-{h}", h, HOSE, K.tube_profile(-0.012, K.HOSE_SEG + 0.012, 0.03), (1, 0, 0), tone=0.5), h, "member"))
    s.append(K.member(K.revolve("member-grip", "grip", GREY, K.tube_profile(0.0, 0.18, 0.03), (1, 0, 0), tone=0.5), "grip", "member"))
    s.append(K.member(K.revolve("member-wand", "wand", STEEL, K.tube_profile(0.0, 0.83, 0.03), (1, 0, 0), tone=0.5), "wand", "member"))
    s.append(K.member(K.extrude("member-nozzle", "nozzle", GREY, "xy", HEAD_500, -0.13, 0.13, tone=0.5), "nozzle", "member"))
    return s + supports(500) + masses(500)


DECISIONS = {
    50: [
        K.text("Body: the canister is a lower shell and a domed lid, each lofted through horizontal superelliptic plan sections taken from the drawn side and front outlines, with the carry handle and its finger opening, two foot buttons, a grey bumper band round the parting line, the hose socket with the hose cuff locked in it and the slotted exhaust grille on the rear face; rear wheels are tyre rings on dished rims with hub caps, the swivel caster a twin wheel in its fork; the 1.5 m hose is ten tubes with corrugation rings and an end fitting, the handle a shaped sleeve with air slide on a swivel, the telescopic wand an outer and an inner tube with the locking collar, the floor nozzle an elbow neck, a ball pivot in a yoke, a 0.285 m head with rubber end caps and two bristle strips.",
               "Körper: Der Kessel ist eine Unterschale und ein gewölbter Deckel, beide durch waagrechte superelliptische Grundrissschnitte aus den gezeichneten Seiten- und Vorderansichten geloftet, mit Tragegriff und Grifföffnung, zwei Fußtasten, grauem Stoßband um die Trennfuge, dem Schlauchstutzen mit eingerasteter Schlauchmuffe und dem geschlitzten Abluftgitter an der Rückseite; die Hinterräder sind Reifenringe auf Schüsselfelgen mit Nabenkappen, die Lenkrolle eine Doppelrolle in ihrer Gabel; der 1,5 m lange Schlauch besteht aus zehn Rohren mit Wellringen und einem Endstück, der Handgriff aus einer geformten Hülse mit Nebenluftschieber auf einem Drehgelenk, das Teleskoprohr aus Außen- und Innenrohr mit Verriegelungsring, die Bodendüse aus Krümmer, Kugelgelenk in einer Gabel, 0,285 m breitem Kopf mit Gummikappen und zwei Borstenleisten."),
        K.text("Thermal: the canister shell, bumper and socket are the body surfaces; the lid is an open-based dome so its seat on the shell is not counted, with handle and buttons; the exhaust grille is a plate whose back against the shell is a built-in face; tyres, rims and casters, hose tubes with cuff and end fitting, wand tubes, handle and the nozzle parts reference their body solids; the exhaust-air emitter is a 90 mm outlet patch on the grille. The motor and dust bag are not regions of this asset (its thermal data put their heat into shell, lid, grille and exhaust air), so they are not modelled.",
               "Thermik: Kesselschale, Stoßband und Stutzen sind die Körperflächen; der Deckel ist eine unten offene Kuppel, damit sein Sitz auf der Schale nicht zählt, samt Griff und Tasten; das Abluftgitter ist eine Platte, deren Rückseite an der Schale eine eingebaute Fläche ist; Reifen, Felgen und Rollen, Schlauchrohre mit Muffe und Endstück, Rohre, Handgriff und Düsenteile verweisen auf ihre Körper; der Emitter Abluft ist eine 90-mm-Austrittsfläche auf dem Gitter. Motor und Staubbeutel sind keine Regionen dieses Assets (seine Thermikdaten geben ihre Wärme an Schale, Deckel, Gitter und Abluft ab) und werden daher nicht modelliert."),
        K.text("Structure: the load path is the canister's base tub and its 3 mm side wall, the 10 mm steel axle with both hub bearings, the caster stem and axle pin, the ten 0.6 mm hose walls with their joints, the handle tube on its swivel, the outer and inner wand tubes and the nozzle elbow, the hollow 2.5 mm nozzle housing on its pivot pin; supports are a contact strip under both tyres, the caster tread and the two bristle strips; masses sit at the centres of mass of canister (motor and cord reel at the rear), wand, nozzle and handle.",
               "Tragwerk: Der Lastpfad ist die Bodenwanne des Kessels mit ihrer 3-mm-Seitenwand, die 10-mm-Stahlachse mit beiden Nabenlagern, Lenkrollenschaft und Achsstift, die zehn 0,6-mm-Schlauchwände mit ihren Gelenken, das Griffrohr auf seinem Drehgelenk, Außen- und Innenrohr und der Düsenkrümmer, das hohle 2,5-mm-Düsengehäuse auf seinem Gelenkbolzen; Auflager sind ein Kontaktstreifen unter beiden Reifen, die Lauffläche der Lenkrolle und die zwei Borstenleisten; Massen liegen in den Schwerpunkten von Kessel (Motor und Kabeltrommel hinten), Rohr, Düse und Handgriff."),
    ],
    100: [
        K.text("Body: shell and lid with fewer sections, the handle as a solid bridge (the 17 mm finger opening is at the 15 mm limit), socket and cuff as one collar, the grille as a plain 15 mm plate; bumper, buttons, rims, hub caps, corrugation rings, end fitting, air slide, locking collar, yoke and end caps are below 15 mm and merge; the twin caster is one roller, outer and inner wand tube one tube, the nozzle head one profile across its full 0.285 m on its two bristle strips.",
               "Körper: Schale und Deckel mit weniger Schnitten, der Griff als volle Brücke (die 17-mm-Grifföffnung liegt an der 15-mm-Grenze), Stutzen und Muffe als ein Ring, das Gitter als glatte 15-mm-Platte; Stoßband, Tasten, Felgen, Nabenkappen, Wellringe, Endstück, Nebenluftschieber, Verriegelungsring, Gabel und Endkappen liegen unter 15 mm und gehen auf; die Doppelrolle ist eine Rolle, Außen- und Innenrohr ein Rohr, der Düsenkopf ein Profil über die ganzen 0,285 m auf seinen zwei Borstenleisten."),
        K.text("Thermal and structure: the same regions on the simpler body (lid as open dome, grille plate with its built-in back, outlet patch); structure keeps the base tub with its wall, axle, the ten hose walls, handle and wand tubes, the hollow nozzle housing, joints only at the wheels, the handle swivel and the nozzle pivot, the three supports and four masses.",
               "Thermik und Tragwerk: dieselben Regionen am einfacheren Körper (Deckel als offene Kuppel, Gitterplatte mit eingebauter Rückseite, Austrittsfläche); das Tragwerk behält Bodenwanne mit Wand, Achse, die zehn Schlauchwände, Griff- und Rohrrohr, das hohle Düsengehäuse, Gelenke nur an den Rädern, am Griffgelenk und am Düsengelenk, die drei Auflager und vier Massen."),
    ],
    200: [
        K.text("Body: shell and lid are one canister loft, the handle a 37 mm block, the socket a 30 mm collar; wheels are 30 mm discs, the caster (25 mm) is not drawn at this scale; hose, wand and handle are plain tubes at their true 36, 35 and 42 mm; the nozzle head is one block including its bristles with the pivot ball; the grille is too thin for 30 mm and lives only in thermal.",
               "Körper: Schale und Deckel sind ein Kessel-Loft, der Griff ein 37-mm-Block, der Stutzen ein 30-mm-Ring; die Räder sind 30-mm-Scheiben, die Lenkrolle (25 mm) wird in diesem Maßstab nicht gezeichnet; Schlauch, Rohr und Griff sind glatte Rohre in wahren 36, 35 und 42 mm; der Düsenkopf ist ein Block samt Borsten mit der Gelenkkugel; das Gitter ist für 30 mm zu dünn und lebt nur in der Thermik."),
        K.text("Thermal: shell and lid as their own skins under the merged body, the grille plate, both bristle strips and the outlet patch as thermal-only solids at their layer thicknesses. Structure: base plate and hollow tub wall, axle, hose and tube walls (their bores exceed 30 mm), the nozzle as a solid block with its pivot, supports and masses.",
               "Thermik: Schale und Deckel als eigene Häute unter dem zusammengelegten Körper, Gitterplatte, beide Borstenleisten und Austrittsfläche als reine Thermikkörper in ihren Schichtdicken. Tragwerk: Bodenplatte und hohle Wannenwand, Achse, Schlauch- und Rohrwände (ihre Bohrungen übersteigen 30 mm), die Düse als voller Block mit Gelenk, Auflager und Massen."),
    ],
    500: [
        K.text("Body: the canister is one loft without handle, the wheel pair one 0.30 m drum (the 35 mm between the wheels under the canister is below 75 mm), hose, handle and wand are drawn and modelled as 60 mm tubes (0.12 mm on paper, widened by the body pen to a printable line), the handle a short tube in line with the wand, the nozzle a 75 mm block across 0.285 m; the minimum that still reads as a canister cleaner with hose and floor tool.",
               "Körper: Der Kessel ist ein Loft ohne Griff, das Radpaar eine 0,30 m breite Walze (die 35 mm zwischen den Rädern unter dem Kessel liegen unter 75 mm), Schlauch, Griff und Rohr werden als 60-mm-Rohre gezeichnet und modelliert (0,12 mm auf dem Papier, durch den Körperstift zu einer druckbaren Linie verbreitert), der Griff ein kurzes Rohr in der Flucht des Rohrs, die Düse ist ein 75-mm-Block über 0,285 m; das Minimum, das noch als Bodenstaubsauger mit Schlauch und Bodendüse lesbar ist."),
        K.text("Thermal: the regions keep their true sizes as thermal-only skins (hose, wand and handle at 36, 35 and 42 mm, nozzle head, brush, grille, outlet) under the generalised body. Structure: the hollow tub wall, axle, solid 60 mm hose, handle and wand members (their bores are below 75 mm), the nozzle block, supports and masses.",
               "Thermik: Die Regionen behalten als reine Thermikhäute ihre wahre Größe (Schlauch, Rohr und Griff in 36, 35 und 42 mm, Düsenkopf, Bürste, Gitter, Austritt) unter dem verallgemeinerten Körper. Tragwerk: die hohle Wannenwand, Achse, volle 60-mm-Glieder für Schlauch, Griff und Rohr (ihre Bohrungen liegen unter 75 mm), der Düsenblock, Auflager und Massen."),
    ],
}


def thermal_500(body):
    t = thermal_coarse(500, body)
    return t


def volumes():
    out = {}
    b50 = body_50()
    b100 = body_100()
    b200 = body_200()
    b500 = body_500()
    lists = {50: (b50, thermal_50(b50), structure_50()), 100: (b100, thermal_100(b100), structure_100()), 200: (b200, thermal_coarse(200, b200), structure_200()), 500: (b500, thermal_500(b500), structure_500())}
    for sc, (b, t, s) in lists.items():
        out[str(sc)] = {"status": "authored", "denominator": sc, "body": K.r5(b), "thermal": K.r5(t), "structure": K.r5(s), "decisions": DECISIONS[sc]}
    return out


def main():
    vol = volumes()
    if "--print" in sys.argv:
        for sc, v in vol.items():
            print(f"[DEBUG] 1:{sc} body {len(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])}", file=sys.stderr)
    K.SOURCE.write_text(json.dumps({"volumes": vol}, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(K.SOURCE)


if __name__ == "__main__":
    main()
