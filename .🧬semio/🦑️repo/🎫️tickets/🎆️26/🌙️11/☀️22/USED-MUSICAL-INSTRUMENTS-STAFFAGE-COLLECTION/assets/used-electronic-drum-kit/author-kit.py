"""🥁 The one authoritative chisel of `used-electronic-drum-kit`: every hand-chosen number of the used mesh-head electronic drum kit (two-post aluminium rack with three mesh tom pads, a mesh snare pad, folding crash and ride booms, a hi-hat pad, the sound module, a kick tower with its pedal, a hi-hat controller pedal and a drum throne) and the generator that writes the asset document `🔣️.json` next to this file.

Run: `python3 author-kit.py` (writes `🔣️.json`); the numbers and their sources are listed in `../../reports/used-electronic-drum-kit.md`.

@see ./author_geo.py — bones, parts and projections
@see ../../brief.md — the binding brief of the wave
"""
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from author_geo import Bone, Part, box, capsule, depth, ellipsoid, extrude, hull, project, r6, revolve  # noqa: E402


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


SCALES = (50, 100, 200, 500)
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}
ROTATING = {"crash-boom", "ride-boom", "crash", "ride", "kick-footboard", "kick-beater", "hihat-footboard"}

POST_Z = 0.40
POST_R = 0.019
FOOT_R = 0.016
CROSS_X, CROSS_Y, CROSS_HALF = 0.045, 0.70, 0.43
POST_TOP = 1.0
CRASH_PIVOT = (0.0, 0.975, 0.60)
RIDE_PIVOT = (0.0, 0.975, -0.62)
BOOM_L = 0.30
HOLDER = 0.045
SNARE_B, SNARE_TILT = (-0.295, 0.545, 0.08), 5.0
TOM_B, TOM_TILT = (-0.098, 0.769), 20.0
TOM_Z = 0.15
FLOOR_B, FLOOR_TILT = (-0.261, 0.4956, -0.36), 8.0
MODULE_C, MODULE_SIZE, MODULE_ANGLE = (-0.11, 0.87, -0.43), (0.078, 0.206, 0.272), -20.0
HIHAT_B, HIHAT_TILT, HIHAT_Z = (-0.282, 0.85), 5.0, 0.29
PEDAL_Z = 0.25
KICK_BASE = (-0.02, 0.24, 0.0)
KICK_UP_X, KICK_RAIL_Z = -0.03, 0.18
PEDAL_X0, AXLE = -0.52, (-0.135, 0.15)
BEATER_REST, BEATER_STRIKE = 115.0, -36.54
THRONE_X, THRONE_TUBE_TOP, THRONE_LEG = -0.85, 0.36, 0.29
LEG_AZ = (180.0, 60.0, -60.0)
SEAT_RAISE = 0.09
CRASH_PLAY = (-120.0, -135.0)
RIDE_PLAY = (-110.0, -148.0)


def tilt_n(deg):
    """📐 Unit normal of a pad tilted `deg` towards the drummer (−x)."""
    t = math.radians(deg)
    return (-math.sin(t), math.cos(t), 0.0)


def rig():
    """🦴 The rig: a datum on the floor, the liftable rack with its two folding cymbal booms and tilters, the kick tower with its pedal, footboard and beater, the hi-hat controller with its footboard, and the throne with its height-adjustable seat."""
    m = lambda kg, x, y: {"kg": kg, "comX": x, "comY": y, "distribution": "point", "source": "authored-estimate", "reference": "roland-td17-2018"}
    tube = lambda ro, t, mat, en, de: {"areaM2": r6(math.pi * (ro**2 - (ro - t) ** 2)), "secondMomentM4": float(f"{math.pi / 4 * (ro**4 - (ro - t) ** 4):.4e}"), "extremeFibreM": ro, "material": mat, "title": T(en, de)}
    bones = [
        Bone("datum", None, 0, 0, 0, 0.04, {"type": "fixed"}, T("Floor datum of the kit", "Bodenbezug des Sets")),
        Bone("rack", "datum", 0, 0, 90, CROSS_Y, {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.4}, T("Two-post rack with crossbar, feet, clamps, pads and module", "Zwei-Säulen-Rack mit Querrohr, Füßen, Klemmen, Pads und Modul"), mass=m(0, 0, 0), section=tube(POST_R, 0.002, "alu-anodised", "Aluminium tube 38 x 2 mm", "Aluminiumrohr 38 x 2 mm")),
        Bone("crash-boom", "rack", CRASH_PIVOT[1], -CRASH_PIVOT[0], -180, BOOM_L, {"type": "revolute", "minDeg": -130, "maxDeg": 0}, T("Crash boom arm (folds down)", "Crash-Galgenarm (klappt nach unten)"), z=CRASH_PIVOT[2], mass=m(0, 0, 0), section=tube(0.011, 0.0015, "steel-chrome", "Steel tube 22 x 1.5 mm", "Stahlrohr 22 x 1,5 mm")),
        Bone("crash", "crash-boom", BOOM_L, 0, 0, 0.15, {"type": "revolute", "minDeg": -160, "maxDeg": 10}, T("Crash cymbal pad on its tilter", "Crash-Beckenpad auf seinem Neiger"), mass=m(0, 0, 0)),
        Bone("ride-boom", "rack", RIDE_PIVOT[1], -RIDE_PIVOT[0], -180, BOOM_L, {"type": "revolute", "minDeg": -130, "maxDeg": 0}, T("Ride boom arm (folds down)", "Ride-Galgenarm (klappt nach unten)"), z=RIDE_PIVOT[2], mass=m(0, 0, 0), section=tube(0.011, 0.0015, "steel-chrome", "Steel tube 22 x 1.5 mm", "Stahlrohr 22 x 1,5 mm")),
        Bone("ride", "ride-boom", BOOM_L, 0, 0, 0.165, {"type": "revolute", "minDeg": -160, "maxDeg": 10}, T("Ride cymbal pad on its tilter", "Ride-Beckenpad auf seinem Neiger"), mass=m(0, 0, 0)),
        Bone("kick-tower", "datum", 0.06, 0, 90, 0.42, {"type": "fixed"}, T("Kick tower with mesh kick pad", "Kickturm mit Mesh-Kickpad"), mass=m(0, 0, 0), section=tube(0.0125, 0.0015, "steel-black", "Steel tube 25 x 1.5 mm", "Stahlrohr 25 x 1,5 mm")),
        Bone("kick-pedal", "datum", PEDAL_X0, 0, 0, 0.42, {"type": "fixed"}, T("Kick pedal frame", "Rahmen der Fußmaschine"), mass=m(0, 0, 0)),
        Bone("kick-footboard", "kick-pedal", 0.02, 0.035, 16, 0.30, {"type": "revolute", "minDeg": -14, "maxDeg": 2}, T("Kick pedal footboard", "Trittplatte der Fußmaschine"), mass=m(0, 0, 0)),
        Bone("kick-beater", "kick-pedal", AXLE[0] - PEDAL_X0, AXLE[1], BEATER_REST, 0.215, {"type": "revolute", "minDeg": -40, "maxDeg": 15}, T("Kick beater on the axle", "Schlägel auf der Achse"), mass=m(0, 0, 0)),
        Bone("hihat-pedal", "datum", PEDAL_X0, 0, 0, 0.30, {"type": "fixed"}, T("Hi-hat controller pedal base", "Basis des Hi-Hat-Controllerpedals"), z=PEDAL_Z, mass=m(0, 0, 0)),
        Bone("hihat-footboard", "hihat-pedal", 0.015, 0.025, 9, 0.28, {"type": "revolute", "minDeg": -8, "maxDeg": 2}, T("Hi-hat controller footboard", "Trittplatte des Hi-Hat-Controllers"), mass=m(0, 0, 0)),
        Bone("throne-base", "datum", THRONE_X, 0, 90, THRONE_TUBE_TOP, {"type": "fixed"}, T("Throne tripod and tube", "Dreibein und Rohr des Hockers"), mass=m(0, 0, 0), section=tube(0.016, 0.0015, "steel-chrome", "Steel tube 32 x 1.5 mm", "Stahlrohr 32 x 1,5 mm")),
        Bone("throne-seat", "throne-base", THRONE_TUBE_TOP, 0, 0, 0.12, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.15}, T("Throne seat on its spindle", "Hockersitz auf seiner Spindel"), mass=m(0, 0, 0)),
    ]
    B = {b.id: b for b in bones}
    for b in bones:
        b.place(B)
    return bones, B


def pad(B, prefix, bone, base, n, R, h, rim, head, level, region_shell="pad-shells", tone=0.22, mass=0.0):
    """🥁 A mesh pad: ABS shell, rubber rim hoop and mesh head at 1:50; shell with rim lip and the mesh head as one sheet coarser."""
    a0, a1, r0, r1 = rim
    ha, hr = head
    parts = []
    if level == 50:
        parts.append(revolve(B, f"{prefix}-shell", bone, base, n, [[0, 0], [0, R], [h, R], [h, 0]], "abs-shell", region_shell, "body", tone, mass=mass * 0.7, draw={"group": prefix}))
        loop = [[a0, r0 + 0.002], [a0, r1 - 0.002], [a0 + 0.002, r1], [a1 - 0.002, r1], [a1, r1 - 0.002], [a1, r0 + 0.002], [a1 - 0.002, r0], [a0 + 0.002, r0], [a0, r0 + 0.002]]
        parts.append(revolve(B, f"{prefix}-rim", bone, base, n, loop, "rubber-epdm", region_shell, "body", tone - 0.06, flags={"slender": True}, mass=mass * 0.2, draw={"group": prefix}))
    else:
        prof = [[0, 0], [0, R], [a0, R], [a0, r1], [a1, r1], [a1, r0], [ha, r0], [ha, 0]]
        parts.append(revolve(B, f"{prefix}-body", bone, base, n, prof, "abs-shell", region_shell, "body", tone, mass=mass * 0.9, draw={"group": prefix}))
    parts.append(revolve(B, f"{prefix}-head", bone, base, n, [[ha, 0], [ha, hr], [ha + 0.003, hr], [ha + 0.003, 0]], "mesh-polyester", "mesh-heads", "detail" if level < 500 else "body", 0.42, flags={"sheet": True}, mass=mass * 0.1, draw={"head": True, "hatch": level <= 100}))
    return parts


def cymbal(B, prefix, bone, R, bell, level, mass):
    """🥏 A rubber cymbal pad on its tilter: holder rod along the pad normal and the pad profile (bow and bell) about it."""
    b = B[bone]
    v = (-math.sin(b.th), math.cos(b.th), 0.0)
    base = (b.o[0] + HOLDER * v[0], b.o[1] + HOLDER * v[1], b.o[2])
    t_edge = 0.010 if level < 200 else 0.016
    top = bell if level < 200 else 0.032
    prof = [[0, 0], [0, R], [t_edge, R], [top - 0.010, 0.33 * R], [top, 0.24 * R], [top, 0]] if level < 500 else [[0, 0], [0, R], [0.02, R], [0.02, 0]]
    rod_r = 0.006 if level <= 100 else 0.01
    rod_top = (b.o[0] + (HOLDER + 0.004) * v[0], b.o[1] + (HOLDER + 0.004) * v[1], b.o[2])
    rod = capsule(B, f"{prefix}-holder", bone, b.o, rod_top, rod_r, "steel-chrome", "rack", "detail" if level <= 100 else "body", 0.65, flags={"member": bone}, mass=0.05, draw={"hint": "rod-v", "len": HOLDER + 0.004})
    disc = revolve(B, f"{prefix}-pad", bone, base, v, prof, "rubber-epdm", "cymbal-pads", "body", 0.14, flags={"member": bone}, mass=mass - 0.05, draw={"profile": [(HOLDER + a, r) for a, r in prof]})
    return [rod, disc]


def parts_for(level, B):
    """🧱 Every solid of one scale, authored in world rest metres: 1:50 with clamps, rods, holders, braces, spring and the traces of use; 1:100 with merged pad bodies and no traces; 1:200 with 20 mm rods, merged pedal frames and no braces; 1:500 as tubes, pad discs, cymbal discs and one block per pedal."""
    F = FEATURE[level]
    P = []
    fine, coarse, tiny = level <= 100, level >= 200, level == 500
    rod = lambda d: (d if fine else max(d, 0.02)) / 2
    cap = lambda r: r if level < 500 else max(r, 0.01)
    detail = "detail" if fine else "body"
    # rack: posts, feet, T-blocks, crossbar, clamps
    for side, z in (("l", POST_Z), ("r", -POST_Z)):
        P.append(capsule(B, f"post-{side}", "rack", (0, 0.03, z), (0, POST_TOP - POST_R, z), POST_R, "alu-anodised", "rack", "body", 0.62, flags={"member": "rack"}, mass=0.62))
        P.append(capsule(B, f"foot-{side}", "rack", (-0.30 + FOOT_R, FOOT_R, z), (0.30 - FOOT_R, FOOT_R, z), FOOT_R, "alu-anodised", "rack", "body", 0.58, flags={"member": "rack"}, mass=0.30))
        if not tiny:
            P.append(box(B, f"t-block-{side}", "rack", (0, 0.045, z), (0.07, 0.06, 0.06), "zinc-die-cast", "rack", "body", 0.36, corner=0.006, flags={"joint": "rack"}, mass=0.35))
            P.append(box(B, f"cross-clamp-{side}", "rack", (0.0225, CROSS_Y, z), (0.09, 0.07, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.006, flags={"joint": "rack"}, mass=0.35))
            P.append(box(B, f"top-clamp-{side}", "rack", (0, 0.975, z), (0.05, 0.05, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.005, mass=0.25))
    P.append(capsule(B, "crossbar", "rack", (CROSS_X, CROSS_Y, -CROSS_HALF + POST_R), (CROSS_X, CROSS_Y, CROSS_HALF - POST_R), POST_R, "alu-anodised", "rack", "body", 0.6, flags={"member": "rack"}, mass=0.48))
    for side, pivot, zp in (("crash", CRASH_PIVOT, POST_Z), ("ride", RIDE_PIVOT, -POST_Z)):
        sgn = 1 if pivot[2] > 0 else -1
        P.append(capsule(B, f"{side}-extension", "rack", (0, pivot[1], zp), (0, pivot[1], pivot[2] - sgn * 0.025), 0.011, "steel-chrome", "rack", "body", 0.7, flags={"member": "rack"}, mass=0.15))
        P.append(revolve(B, f"{side}-pivot", "rack", (0, pivot[1], pivot[2] - 0.025), (0, 0, 1), [[0, 0], [0, 0.022], [0.05, 0.022], [0.05, 0]], "zinc-die-cast", "rack", "body", 0.32, flags={"joint": side + "-boom"}, mass=0.2))
    # snare on its arm from the crossbar
    n = tilt_n(SNARE_TILT)
    P += pad(B, "snare", "rack", SNARE_B, n, 0.150, 0.072, (0.060, 0.083, 0.148, 0.166), (0.072, 0.148), level, mass=2.4)
    if not tiny:
        P.append(box(B, "snare-bracket", "rack", (SNARE_B[0] - 0.012 * n[0], SNARE_B[1] - 0.012 * n[1], SNARE_B[2]), (0.05, 0.024, 0.05), "zinc-die-cast", "rack", "body", 0.3, angle=SNARE_TILT, mass=0.2))
        P.append(box(B, "snare-clamp", "rack", (CROSS_X, CROSS_Y, SNARE_B[2]), (0.06, 0.07, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.006, mass=0.3))
    P.append(capsule(B, "snare-arm", "rack", (0.04, 0.69, SNARE_B[2]), (-0.29, 0.528, SNARE_B[2]), rod(0.0127), "steel-chrome", "rack", "body", 0.68, flags={"member": "rack"}, mass=0.25))
    # rack toms on L-rods
    n = tilt_n(TOM_TILT)
    for side, z in (("1", TOM_Z), ("2", -TOM_Z)):
        base = (TOM_B[0], TOM_B[1], z)
        P += pad(B, f"tom{side}", "rack", base, n, 0.104, 0.055, (0.045, 0.066, 0.102, 0.118), (0.055, 0.102), level, mass=1.9)
        if not tiny:
            P.append(box(B, f"tom{side}-bracket", "rack", (base[0] - 0.012 * n[0], base[1] - 0.012 * n[1], z), (0.045, 0.024, 0.045), "zinc-die-cast", "rack", "body", 0.3, angle=TOM_TILT, mass=0.15))
            P.append(box(B, f"tom{side}-clamp", "rack", (CROSS_X, CROSS_Y, z), (0.06, 0.07, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.006, mass=0.3))
        P.append(capsule(B, f"tom{side}-arm", "rack", (0.04, 0.71, z), (-0.09, 0.755, z), rod(0.0127), "steel-chrome", "rack", "body", 0.68, flags={"member": "rack"}, mass=0.12))
    # floor tom on the right post
    n = tilt_n(FLOOR_TILT)
    P += pad(B, "floor-tom", "rack", FLOOR_B, n, 0.104, 0.055, (0.045, 0.066, 0.102, 0.118), (0.055, 0.102), level, mass=1.9)
    if not tiny:
        P.append(box(B, "floor-tom-bracket", "rack", (FLOOR_B[0] - 0.012 * n[0], FLOOR_B[1] - 0.012 * n[1], FLOOR_B[2]), (0.045, 0.024, 0.045), "zinc-die-cast", "rack", "body", 0.3, angle=FLOOR_TILT, mass=0.15))
        P.append(box(B, "floor-tom-clamp", "rack", (0, 0.47, -POST_Z), (0.06, 0.06, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.006, mass=0.3))
    P.append(capsule(B, "floor-tom-arm", "rack", (-0.01, 0.47, -POST_Z + 0.005), (-0.255, 0.483, FLOOR_B[2] - 0.002), rod(0.0127), "steel-chrome", "rack", "body", 0.68, flags={"member": "rack"}, mass=0.15))
    # hi-hat pad on its fixed boom from the left post
    if not tiny:
        P.append(box(B, "hihat-clamp", "rack", (0, 0.80, POST_Z), (0.06, 0.05, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.006, mass=0.3))
    P.append(capsule(B, "hihat-arm", "rack", (-0.012, 0.80, POST_Z - 0.005), (HIHAT_B[0] + 0.002, 0.825, HIHAT_Z), 0.011, "steel-chrome", "rack", "body", 0.7, flags={"member": "rack"}, mass=0.3))
    hn = tilt_n(HIHAT_TILT)
    hb = (HIHAT_B[0], HIHAT_B[1], HIHAT_Z)
    P.append(capsule(B, "hihat-holder", "rack", (HIHAT_B[0] + 0.002, 0.825, HIHAT_Z), (hb[0] + 0.004 * hn[0], hb[1] + 0.004 * hn[1], HIHAT_Z), 0.006 if fine else 0.01, "steel-chrome", "rack", detail, 0.65, mass=0.03))
    te, tb = (0.010, 0.028) if level < 200 else ((0.016, 0.032) if level == 200 else (0.02, 0.02))
    hprof = [[0, 0], [0, 0.13], [te, 0.13], [tb - 0.010, 0.045], [tb, 0.032], [tb, 0]] if level < 500 else [[0, 0], [0, 0.13], [0.02, 0.13], [0.02, 0]]
    P.append(revolve(B, "hihat-pad", "rack", hb, hn, hprof, "rubber-epdm", "cymbal-pads", "body", 0.14, mass=0.6))
    # sound module on the right post
    ang = math.radians(MODULE_ANGLE)
    ax, ay = (math.cos(ang), math.sin(ang)), (-math.sin(ang), math.cos(ang))
    mc = MODULE_C
    at = lambda du, dv, dw=0.0: (mc[0] + ax[0] * du + ay[0] * dv, mc[1] + ax[1] * du + ay[1] * dv, mc[2] + dw)
    P.append(box(B, "module", "rack", mc, MODULE_SIZE, "abs-module", "module", "body", 0.24, angle=MODULE_ANGLE, corner=0.008 if level < 500 else 0.0, mass=1.2))
    if not tiny:
        P.append(box(B, "module-clamp", "rack", (0, 0.87, -POST_Z), (0.06, 0.06, 0.06), "zinc-die-cast", "rack", "body", 0.34, corner=0.006, mass=0.3))
    P.append(capsule(B, "module-mount", "rack", (-0.02, 0.87, -POST_Z - 0.003), at(0.036, 0.0, 0.0), 0.012, "steel-chrome", "rack", "body", 0.66, mass=0.1))
    if fine:
        P.append(box(B, "module-display", "rack", at(-0.0405, 0.045), (0.002, 0.05, 0.09), "display-glass", "module", "glazing", 0.55, angle=MODULE_ANGLE, flags={"sheet": True}))
    if level == 50:
        P.append(revolve(B, "module-knob", "rack", at(-0.039, -0.035, 0.07), (-ax[0], -ax[1], 0), [[0, 0], [0, 0.016], [0.014, 0.014], [0.014, 0]], "abs-module", "module", "detail", 0.12))
        P.append(box(B, "module-buttons", "rack", at(-0.0395, -0.065, -0.03), (0.002, 0.03, 0.16), "abs-module", "module", "texture", 0.5, angle=MODULE_ANGLE, flags={"texture": True}))
        P.append(box(B, "module-sticker", "rack", at(-0.0395, 0.075, -0.085), (0.0015, 0.035, 0.07), "paper-card", "module", "texture", 0.92, angle=MODULE_ANGLE, flags={"texture": True}))
        P.append(box(B, "price-tag", "rack", (CROSS_X, CROSS_Y - POST_R - 0.045 + 0.0005, 0.30), (0.0015, 0.09, 0.06), "paper-card", "rack", "detail", 0.95, flags={"sheet": True}, mass=0.004))
        sn = tilt_n(SNARE_TILT)
        st_ = (sn[1], -sn[0])
        P.append(revolve(B, "snare-wear", "rack", (SNARE_B[0] + 0.075 * sn[0] + 0.02 * st_[0], SNARE_B[1] + 0.075 * sn[1] + 0.02 * st_[1], SNARE_B[2] + 0.01), sn, [[0, 0], [0, 0.05], [0.0008, 0.05], [0.0008, 0]], "mesh-polyester", "mesh-heads", "texture", 0.62, flags={"texture": True}))
    # cymbal booms and pads (folding bones)
    for side, R, bell, mass in (("crash", 0.150, 0.030, 1.05), ("ride", 0.165, 0.034, 1.35)):
        bone = side + "-boom"
        b = B[bone]
        tip = b.world((BOOM_L, 0, 0))
        P.append(capsule(B, f"{side}-tube", bone, b.o, tip, 0.011, "steel-chrome", "rack", "body", 0.72, flags={"member": bone}, mass=0.43, draw={"hint": "rod-u", "len": BOOM_L, "r": 0.011}))
        P += cymbal(B, side, side, R, bell, level, mass)
    # kick tower
    kt = "kick-tower"
    for side, z in (("l", KICK_RAIL_Z), ("r", -KICK_RAIL_Z)):
        P.append(capsule(B, f"kick-rail-{side}", kt, (-0.10 + 0.015, 0.015, z), (0.22 - 0.015, 0.015, z), 0.015, "steel-black", "kick-tower", "body", 0.2, flags={"member": kt}, mass=0.35))
        P.append(capsule(B, f"kick-upright-{side}", kt, (KICK_UP_X, 0.02, z), (KICK_UP_X, 0.42 - 0.0125, z), 0.0125, "steel-black", "kick-tower", "body", 0.2, flags={"member": kt}, mass=0.3))
        if fine:
            P.append(capsule(B, f"kick-brace-{side}", kt, (KICK_UP_X + 0.01, 0.38, z), (0.20, 0.03, z), 0.01, "steel-black", "kick-tower", "body", 0.22, flags={"member": kt}, mass=0.2))
        if not tiny:
            s = 1 if z > 0 else -1
            P.append(box(B, f"kick-bracket-{side}", kt, (-0.045, 0.24, s * 0.158), (0.03, 0.04, 0.025), "steel-black", "kick-tower", "body", 0.26, mass=0.05))
    P.append(capsule(B, "kick-top-bar", kt, (KICK_UP_X, 0.42 - 0.0125, -KICK_RAIL_Z), (KICK_UP_X, 0.42 - 0.0125, KICK_RAIL_Z), 0.0125, "steel-black", "kick-tower", "body", 0.2, flags={"member": kt}, mass=0.2))
    P.append(capsule(B, "kick-front-bar", kt, (-0.10, 0.03, -KICK_RAIL_Z), (-0.10, 0.03, KICK_RAIL_Z), 0.012, "steel-black", "kick-tower", "body", 0.2, flags={"member": kt}, mass=0.2))
    P += pad(B, "kick", kt, KICK_BASE, (-1.0, 0.0, 0.0), 0.148, 0.045, (0.036, 0.058, 0.145, 0.160), (0.045, 0.145), level, region_shell="kick-tower", tone=0.2, mass=1.9)
    if level == 50:
        P.append(revolve(B, "kick-wear", kt, (-0.0685, 0.331, 0.0), (-1.0, 0.0, 0.0), [[0, 0], [0, 0.035], [0.0008, 0.035], [0.0008, 0]], "mesh-polyester", "mesh-heads", "texture", 0.65, flags={"texture": True}))
    # kick pedal
    kp = "kick-pedal"
    if not tiny:
        base_t = 0.008 if level == 50 else 0.012
        P.append(box(B, "kick-base", kp, (-0.3225, base_t / 2, 0), (0.395, base_t, 0.13), "steel-black", "pedals", "body", 0.3, corner=0.002 if level == 50 else 0.0, flags={"member": kp}, mass=1.0))
        P.append(box(B, "kick-clamp", kp, (-0.115, 0.03, 0), (0.03, 0.04, 0.10), "steel-black", "pedals", "body", 0.28, flags={"member": kp}, mass=0.25))
        if level < 200:
            for side, z in (("l", 0.055), ("r", -0.055)):
                P.append(box(B, f"kick-post-{side}", kp, (AXLE[0], 0.085, z), (0.02, 0.16, 0.012), "steel-black", "pedals", "body", 0.3, flags={"member": kp}, mass=0.25))
        else:
            P.append(box(B, "kick-frame", kp, (AXLE[0], 0.085, 0), (0.02, 0.16, 0.122), "steel-black", "pedals", "body", 0.3, flags={"member": kp}, mass=0.5))
        P.append(capsule(B, "kick-axle", kp, (AXLE[0], AXLE[1], -0.068), (AXLE[0], AXLE[1], 0.068), 0.008, "steel-black", "pedals", "body", 0.35, flags={"joint": "kick-beater"}, mass=0.15))
        P.append(box(B, "kick-heel-block", kp, (-0.50, 0.022, 0), (0.03, 0.03, 0.09), "steel-black", "pedals", "body", 0.32, flags={"member": kp}, mass=0.1))
        P.append(capsule(B, "kick-heel-hinge", kp, (-0.50, 0.035, -0.04), (-0.50, 0.035, 0.04), 0.01, "steel-black", "pedals", "body", 0.35, flags={"joint": "kick-footboard"}, mass=0.1))
        if level == 50:
            P.append(capsule(B, "kick-spring", kp, (AXLE[0], 0.012, -0.066), (AXLE[0], 0.13, -0.066), 0.005, "steel-black", "pedals", "detail", 0.45, mass=0.05))
        ft = 0.012 if level == 50 else 0.015
        prof = [(-0.01, -ft / 2), (0.30, -ft / 2), (0.30, ft / 2), (-0.01, ft / 2)]
        P.append(extrude(B, "kick-board", "kick-footboard", prof, -0.045, 0.045, "alu-cast", "pedals", "body", 0.5, flags={"member": "kick-footboard"}, mass=0.5, draw={"hint": "sweep-w", "profile": prof, "w": (-0.045, 0.045)}))
    else:
        wedge = [(-0.52, 0.0), (-0.10, 0.0), (-0.10, 0.05), (-0.125, 0.165), (-0.145, 0.165), (-0.52, 0.04)]
        loc = [(x - PEDAL_X0, y) for x, y in wedge]
        P.append(extrude(B, "kick-pedal-block", kp, loc, -0.065, 0.065, "steel-black", "pedals", "body", 0.3, flags={"member": kp, "members": ["kick-footboard"]}, mass=1.75))
    bb = B["kick-beater"]
    sr = 0.004 if level < 500 else 0.01
    P.append(capsule(B, "kick-shaft", "kick-beater", bb.o, bb.world((0.165, 0, 0)), sr, "steel-chrome", "pedals", "body", 0.7, flags={"member": "kick-beater", "slender": level < 500}, mass=0.06, draw={"hint": "rod-u", "len": 0.165, "r": sr}))
    circ = [(0.185 + 0.03 * math.cos(2 * math.pi * i / 16), 0.03 * math.sin(2 * math.pi * i / 16)) for i in range(16)]
    P.append(extrude(B, "kick-felt", "kick-beater", circ, -0.025, 0.025, "felt", "pedals", "body", 0.85, mass=0.09, draw={"hint": "sweep-w", "profile": circ, "w": (-0.025, 0.025)}))
    # hi-hat controller pedal
    hp, hz = "hihat-pedal", PEDAL_Z
    if not tiny:
        if level < 200:
            P.append(box(B, "hihat-base", hp, (-0.37, 0.005, hz), (0.30, 0.010, 0.10), "steel-black", "pedals", "body", 0.3, corner=0.002 if level == 50 else 0.0, flags={"member": hp}, mass=0.6))
            P.append(box(B, "hihat-housing", hp, (-0.25, 0.02, hz), (0.06, 0.03, 0.10), "abs-shell", "pedals", "body", 0.22, flags={"member": hp}, mass=0.4))
        else:
            P.append(box(B, "hihat-base", hp, (-0.37, 0.0175, hz), (0.30, 0.035, 0.10), "steel-black", "pedals", "body", 0.3, flags={"member": hp}, mass=1.0))
        P.append(box(B, "hihat-heel-block", hp, (-0.505, 0.016, hz), (0.026, 0.022, 0.08), "steel-black", "pedals", "body", 0.32, flags={"member": hp}, mass=0.05))
        P.append(capsule(B, "hihat-heel-hinge", hp, (-0.505, 0.022, hz - 0.04), (-0.505, 0.022, hz + 0.04), 0.009, "steel-black", "pedals", "body", 0.35, flags={"joint": "hihat-footboard"}, mass=0.1))
        hft = 0.010 if level == 50 else 0.015
        hprof2 = [(-0.008, -hft / 2), (0.28, -hft / 2), (0.28, hft / 2), (-0.008, hft / 2)]
        P.append(extrude(B, "hihat-board", "hihat-footboard", hprof2, -0.045, 0.045, "alu-cast", "pedals", "body", 0.5, flags={"member": "hihat-footboard"}, mass=0.3, draw={"hint": "sweep-w", "profile": hprof2, "w": (hz - 0.045, hz + 0.045)}))
    else:
        P.append(box(B, "hihat-pedal-block", hp, (-0.37, 0.04, hz), (0.30, 0.08, 0.10), "steel-black", "pedals", "body", 0.3, flags={"member": hp, "members": ["hihat-footboard"]}, mass=1.4))
    # throne
    tbn = "throne-base"
    P.append(capsule(B, "throne-tube", tbn, (THRONE_X, 0.10, 0), (THRONE_X, THRONE_TUBE_TOP - 0.016, 0), 0.016, "steel-chrome", "throne-base", "body", 0.74, flags={"member": tbn}, mass=0.8))
    if not tiny:
        P.append(revolve(B, "throne-collar", tbn, (THRONE_X, 0.08, 0), (0, 1, 0), [[0, 0], [0, 0.03], [0.04, 0.03], [0.04, 0]], "steel-chrome", "throne-base", "body", 0.6, flags={"joint": tbn}, mass=0.3))
        P.append(revolve(B, "throne-lock", tbn, (THRONE_X, 0.325, 0), (0, 1, 0), [[0, 0], [0, 0.026], [0.035, 0.026], [0.035, 0]], "steel-chrome", "throne-base", "body", 0.55, flags={"joint": "throne-seat"}, mass=0.25))
    for i, az in enumerate(LEG_AZ):
        c, s = math.cos(math.radians(az)), math.sin(math.radians(az))
        P.append(capsule(B, f"throne-leg-{'abc'[i]}", tbn, (THRONE_X + 0.028 * c, 0.10, 0.028 * s), (THRONE_X + THRONE_LEG * c, 0.011, THRONE_LEG * s), 0.011, "steel-chrome", "throne-base", "body", 0.72, flags={"member": tbn}, mass=0.45))
        if level == 50:
            P.append(capsule(B, f"throne-brace-{'abc'[i]}", tbn, (THRONE_X + 0.17 * c, 0.055, 0.17 * s), (THRONE_X + 0.02 * c, 0.21, 0.02 * s), 0.006, "steel-chrome", "throne-base", "detail", 0.7, flags={"member": tbn}, mass=0.1))
    if level == 50:
        P.append(revolve(B, "throne-brace-collar", tbn, (THRONE_X, 0.195, 0), (0, 1, 0), [[0, 0], [0, 0.026], [0.025, 0.026], [0.025, 0]], "steel-chrome", "throne-base", "body", 0.6, flags={"joint": tbn}, mass=0.1))
    ts = "throne-seat"
    if not tiny:
        P.append(capsule(B, "seat-spindle", ts, (THRONE_X, 0.26, 0), (THRONE_X, 0.39, 0), 0.0125, "steel-chrome", "throne-base", "body", 0.75, flags={"member": ts}, mass=0.3))
        P.append(revolve(B, "seat-plate", ts, (THRONE_X, 0.395, 0), (0, 1, 0), [[0, 0], [0, 0.12], [0.008, 0.12], [0.008, 0]], "steel-black", "throne-seat", "body", 0.3, flags={"member": ts}, mass=0.4))
        cushion = [[0, 0], [0, 0.150], [0.008, 0.162], [0.03, 0.166], [0.065, 0.163], [0.078, 0.152], [0.08, 0.13], [0.08, 0]] if fine else [[0, 0], [0, 0.16], [0.07, 0.166], [0.08, 0.14], [0.08, 0]]
        P.append(revolve(B, "seat-cushion", ts, (THRONE_X, 0.40, 0), (0, 1, 0), cushion, "pvc-vinyl", "throne-seat", "body", 0.1, mass=0.9))
    else:
        P.append(revolve(B, "seat-cushion", ts, (THRONE_X, 0.34, 0), (0, 1, 0), [[0, 0], [0, 0.03], [0.055, 0.03], [0.055, 0.166], [0.13, 0.166], [0.14, 0.14], [0.14, 0]], "pvc-vinyl", "throne-seat", "body", 0.1, flags={"member": ts}, mass=1.6))
    if level == 50:
        P.append(box(B, "seat-tape", ts, (THRONE_X + 0.05, 0.4804, 0.03), (0.07, 0.0008, 0.05), "pvc-vinyl", "throne-seat", "texture", 0.7, flags={"texture": True}))
    return P


MATERIALS = [
    ("alu-anodised", "Aluminium tube, anodised, worn at the clamps", "Aluminiumrohr, eloxiert, an den Klemmen abgegriffen", 0.77, 0.6, 2700, 160, 900, 69e9, 215e6, "literature", "en-1999-1-1"),
    ("zinc-die-cast", "Die-cast zinc clamp, black paint chipped", "Zinkdruckguss-Klemme, schwarzer Lack abgeplatzt", 0.85, 0.85, 6600, 110, 420, 96e9, 280e6, "literature", "incropera-2018"),
    ("steel-chrome", "Chromed steel tube, dulled", "Verchromtes Stahlrohr, matt geworden", 0.18, 0.4, 7850, 50, 460, 210e9, 235e6, "literature", "incropera-2018"),
    ("steel-black", "Black powder-coated steel", "Schwarz pulverbeschichteter Stahl", 0.9, 0.9, 7850, 50, 460, 210e9, 235e6, "literature", "incropera-2018"),
    ("alu-cast", "Cast aluminium footboard, polished by soles", "Aluminiumguss-Trittplatte, von Sohlen blank gerieben", 0.3, 0.5, 2700, 150, 900, 70e9, 160e6, "literature", "en-1999-1-1"),
    ("abs-shell", "Black ABS pad shell", "Schwarze ABS-Padschale", 0.9, 0.9, 1050, 0.17, 1400, 2.2e9, 40e6, "literature", "iso-10456-2007"),
    ("abs-module", "Dark grey ABS module housing", "Dunkelgraues ABS-Modulgehäuse", 0.9, 0.85, 1050, 0.17, 1400, 2.2e9, 40e6, "literature", "iso-10456-2007"),
    ("rubber-epdm", "Black rubber of rims and cymbal pads, scuffed", "Schwarzer Gummi von Rändern und Beckenpads, verschrammt", 0.92, 0.92, 1200, 0.25, 1600, None, None, "literature", "iso-10456-2007"),
    ("mesh-polyester", "Two-ply polyester mesh head, worn grey at the centre", "Zweilagiges Polyester-Meshfell, in der Mitte grau abgespielt", 0.88, 0.85, 1380, 0.15, 1200, None, None, "authored-estimate", "incropera-2018"),
    ("display-glass", "LCD cover", "LCD-Abdeckung", 0.9, 0.8, 1200, 0.2, 1400, None, None, "authored-estimate", "incropera-2018"),
    ("felt", "Felt beater head, flattened on the striking face", "Filz-Schlägelkopf, an der Schlagfläche abgeflacht", 0.9, 0.7, 300, 0.05, 1300, None, None, "authored-estimate", "iso-10456-2007"),
    ("pvc-vinyl", "Black PVC-coated foam cushion, cracked and taped", "Schwarzes PVC-bezogenes Schaumpolster, rissig und geklebt", 0.9, 0.9, 120, 0.05, 1400, None, None, "authored-estimate", "iso-10456-2007"),
    ("paper-card", "Price tag card", "Preisschild aus Karton", 0.9, 0.3, 700, 0.12, 1300, None, None, "literature", "iso-10456-2007"),
]


def materials():
    """🧱 The materials with their thermal and structural properties; aged surfaces state their aged emissivity."""
    out = []
    for mid, en, de, eps, alpha, rho, k, cp, E, fy, src, ref in MATERIALS:
        m = {"id": mid, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
        if E:
            m["elasticModulusPa"] = E
            m["strengthPa"] = fy
        m["source"], m["reference"] = src, ref
        out.append(m)
    return out


TEXTURE_VIEWS = {"module-buttons": ("plan",), "module-sticker": ("plan",), "snare-wear": ("plan",), "seat-tape": ("plan",), "price-tag": ("front",), "module-display": ("plan",), "module-knob": ("plan",)}
HIDDEN_VIEWS = {"kick-wear": ()}


def octagon(r):
    """🛑 A 16-gon of radius r (profile of a tube across its axis)."""
    return [[r6(r * math.cos(2 * math.pi * i / 16)), r6(r * math.sin(2 * math.pi * i / 16))] for i in range(16)]


def shape(sid, bone, role, z, tone, region, material, geom, hatch=None, solid=None):
    """✏️ One authored 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": round(z, 4), "filled": True, "tone": tone, "region": region, "material": material}
    s.update(geom)
    if hatch:
        s["hatch"] = hatch
    if solid:
        s["solid"] = solid
    return s


def capsule2d(B, bone, view, p0, p1, r):
    """💊 The exact projection of a capsule: bone-local in the elevation, world rest in front and plan."""
    if view == "elevation":
        a, b = B[bone].local(p0), B[bone].local(p1)
        q0, q1 = (a[0], a[1]), (b[0], b[1])
    else:
        q0, q1 = project(view, p0), project(view, p1)
    if math.dist(q0, q1) < 1e-6:
        return {"kind": "ellipse", "cx": r6(q0[0]), "cy": r6(q0[1]), "rx": r6(r), "ry": r6(r)}
    return {"kind": "capsule", "x0": r6(q0[0]), "y0": r6(q0[1]), "x1": r6(q1[0]), "y1": r6(q1[1]), "r0": r6(r), "r1": r6(r)}


def hull2d(B, bone, view, pts):
    """🧮 The convex outline of a world point cloud in one view (bone-local in the elevation)."""
    ring = hull([project(view, p) for p in pts])
    if view == "elevation":
        zc = sum(p[2] for p in pts) / len(pts)
        ring = [B[bone].local((x, y, zc))[:2] for x, y in ring]
    return {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in ring], "closed": True}


def shapes_for(level, parts, B):
    """🖊️ The three views of one scale, drawn from the same numbers as its solids: hulls and capsules of fixed parts, profile solids for the parts of the folding and pedalling bones."""
    views = {"elevation": [], "front": [], "plan": []}
    groups = {}
    for p in parts:
        if "group" in p.draw:
            groups.setdefault(p.draw["group"], []).append(p)
    done = set()
    for idx, p in enumerate(parts):
        tex = p.flags.get("texture", False)
        if tex and level != 50:
            continue
        allowed = TEXTURE_VIEWS.get(p.id, HIDDEN_VIEWS.get(p.id, ("elevation", "front", "plan")))
        if p.draw.get("head"):
            allowed = ("plan",) if not p.id.startswith("kick") else ()
        for view in allowed:
            out = views[view]
            if "group" in p.draw:
                g = p.draw["group"]
                if (g, view) in done:
                    continue
                done.add((g, view))
                members = groups[g]
                pts = [q for m in members for q in m.pts]
                z = 5 + max(depth(view, q) for q in pts) + idx * 1e-4
                sh = shape(g, p.bone, "body", z, p.tone, p.region, p.material, hull2d(B, p.bone, view, pts))
                if view == "elevation":
                    del sh["region"]
                    sh["regions"] = [p.region, "mesh-heads"]
                out.append(sh)
                continue
            z = 5 + max(depth(view, q) for q in p.pts) + idx * 1e-4 + (0.002 if p.draw.get("head") else 0)
            role = "texture" if tex else p.role
            hatch = "mesh" if p.draw.get("hatch") and view == "plan" else None
            rot = p.bone in ROTATING
            if rot and view == "elevation":
                if "profile" in p.draw and p.solid["kind"] == "revolve":
                    prof = p.draw["profile"]
                    right = [(r, a) for a, r in prof][1:-1]
                    ring = right + [(-r, a) for r, a in reversed(right)]
                    geom = {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in ring], "closed": True}
                elif p.draw.get("hint") == "sweep-w":
                    geom = {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in p.draw["profile"]], "closed": True}
                else:
                    geom = capsule2d(B, p.bone, view, *p.draw["capsule"])
                out.append(shape(p.id, p.bone, role, z, p.tone, p.region, p.material, geom, hatch))
                continue
            if rot:
                hint = p.draw.get("hint")
                if "profile" in p.draw and p.solid["kind"] == "revolve":
                    ring = [(r, a) for a, r in p.draw["profile"]]
                    geom = {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in ring], "closed": True}
                    sol = {"kind": "revolved", "axisBone": p.bone, "axis": "v", "axisX": 0, "mirror": True}
                elif hint == "rod-u":
                    r = p.draw["r"]
                    geom = {"kind": "poly", "points": octagon(r), "closed": True}
                    sol = {"kind": "extruded", "sweep": "u", "z0": r6(-r), "z1": r6(p.draw["len"] + r)}
                elif hint == "rod-v":
                    r = p.solid["r0"]
                    geom = {"kind": "poly", "points": octagon(r), "closed": True}
                    sol = {"kind": "extruded", "sweep": "v", "z0": r6(-r), "z1": r6(p.draw["len"] + r)}
                else:
                    geom = {"kind": "poly", "points": [[r6(x), r6(y)] for x, y in p.draw["profile"]], "closed": True}
                    sol = {"kind": "extruded", "sweep": "w", "z0": r6(p.draw["w"][0]), "z1": r6(p.draw["w"][1])}
                out.append(shape(p.id, p.bone, role, z, p.tone, p.region, p.material, geom, hatch, sol))
                continue
            geom = capsule2d(B, p.bone, view, *p.draw["capsule"]) if "capsule" in p.draw else hull2d(B, p.bone, view, p.pts)
            out.append(shape(p.id, p.bone, role, z, p.tone, p.region, p.material, geom, hatch))
    return views


REFERENCES = [
    ("hornbostel-sachs-1914", "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46(4-5), 553-590."),
    ("mimo-2011", "MIMO Consortium (2011). Revision of the Hornbostel-Sachs Classification of Musical Instruments, class 5 electrophones, 53 radioelectric (electronic) instruments; an electronic drum kit is a set of percussion controllers (piezo and switch sensors under mesh and rubber pads) driving an electronic sound generator."),
    ("roland-td17-2018", "Roland Corporation (2018). V-Drums TD-17 series: TD-17 Owner's Manual (main specifications of the sound module, power supply DC 9 V) and product specifications of the MDS-Compact stand, PDX-12 and PDX-8 mesh pads, CY-5, CY-12C and CY-13R cymbal pads, KD-10 kick pad and FD-9 hi-hat controller. https://www.roland.com/global/products/td-17/ (used for the class of a mid-range mesh-head kit; every number of this asset is a rounded class estimate, not one product)."),
    ("fletcher-rossing-1998", "Fletcher, N. H., Rossing, T. D. (1998). The Physics of Musical Instruments, 2nd ed. Springer, New York, chapter 18 Drums (membranes, stick and beater impact)."),
    ("dahl-2004", "Dahl, S. (2004). Playing the accent: comparing striking velocity and timing in an ostinato rhythm performed by four drummers. Acta Acustica united with Acustica 90(4), 762-776."),
    ("din-33402-2", "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (popliteal height and seat height of adults)."),
    ("en-1999-1-1", "EN 1999-1-1:2007+A2:2013 Eurocode 9: Design of aluminium structures - Part 1-1: General structural rules (properties of wrought aluminium alloys)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties of anodised aluminium, chrome, paints and polymers; free convection)."),
    ("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values."),
]

PERMANENT = ["kick-tower-base", "kick-pedal-base", "hihat-pedal-base", "throne-foot-a", "throne-foot-b", "throne-foot-c"]
FEET = ["foot-l-front", "foot-l-rear", "foot-r-front", "foot-r-rear"]
GRIPS = ["grip-l", "grip-r"]
REGION_IDS = ["rack", "pad-shells", "mesh-heads", "cymbal-pads", "module", "kick-tower", "pedals", "throne-seat", "throne-base"]


def anchors(B):
    """📍 Contacts, supports, grips, loads and the emitter outlet, in bone-local metres."""
    A = []
    L = lambda bone, q: B[bone].local(q)
    def add(aid, bone, q, kind, en, de, **kw):
        l = L(bone, q)
        a = {"id": aid, "bone": bone, "x": r6(l[0]), "y": r6(l[1])}
        if abs(q[2] - B[bone].o[2]) > 1e-9:
            a["z"] = r6(q[2] - B[bone].o[2])
        a.update({"kind": kind, "title": T(en, de)})
        a.update(kw)
        A.append(a)
    for side, z in (("l", POST_Z), ("r", -POST_Z)):
        for end, x in (("front", 0.284), ("rear", -0.284)):
            add(f"foot-{side}-{end}", "rack", (x, 0, z), "contact", f"Rubber cap at the {end} of the {'left' if side == 'l' else 'right'} rack foot", f"Gummikappe am {'vorderen' if end == 'front' else 'hinteren'} Ende des {'linken' if side == 'l' else 'rechten'} Rackfußes", support="contact", footprintLengthM=0.04, footprintWidthM=0.03, frictionCoefficient=0.6)
    for side, z in (("l", 0.22), ("r", -0.22)):
        add(f"grip-{side}", "rack", (CROSS_X, CROSS_Y, z), "support", f"{'Left' if side == 'l' else 'Right'} hand on the crossbar when the folded rack is carried", f"{'Linke' if side == 'l' else 'Rechte'} Hand am Querrohr, wenn das gefaltete Rack getragen wird", support="pin")
    add("kick-tower-base", "kick-tower", (0.06, 0, 0), "contact", "Kick tower rails on the floor", "Kufen des Kickturms auf dem Boden", support="contact", footprintLengthM=0.32, footprintWidthM=0.36, frictionCoefficient=0.6)
    add("kick-pedal-base", "kick-pedal", (-0.3225, 0, 0), "contact", "Kick pedal base plate on the floor", "Bodenplatte der Fußmaschine auf dem Boden", support="contact", footprintLengthM=0.39, footprintWidthM=0.13, frictionCoefficient=0.6)
    add("hihat-pedal-base", "hihat-pedal", (-0.37, 0, PEDAL_Z), "contact", "Hi-hat controller base on the floor", "Basis des Hi-Hat-Controllers auf dem Boden", support="contact", footprintLengthM=0.30, footprintWidthM=0.10, frictionCoefficient=0.6)
    for i, az in enumerate(LEG_AZ):
        c, s = math.cos(math.radians(az)), math.sin(math.radians(az))
        add(f"throne-foot-{'abc'[i]}", "throne-base", (THRONE_X + THRONE_LEG * c, 0, THRONE_LEG * s), "contact", f"Throne foot {'abc'[i].upper()}", f"Hockerfuß {'abc'[i].upper()}", support="contact", footprintLengthM=0.04, footprintWidthM=0.025, frictionCoefficient=0.6)
    add("seat-centre", "throne-seat", (THRONE_X, 0.48, 0), "load", "Centre of the seat cushion", "Mitte des Sitzpolsters")
    sn = tilt_n(SNARE_TILT)
    add("snare-head", "rack", (SNARE_B[0] + 0.075 * sn[0], SNARE_B[1] + 0.075 * sn[1], SNARE_B[2]), "load", "Striking spot on the snare mesh head", "Schlagstelle auf dem Snare-Meshfell")
    fb = B["kick-footboard"]
    add("kick-toe", "kick-footboard", fb.world((0.24, 0.006, 0)), "load", "Ball of the right foot on the kick footboard", "Fußballen rechts auf der Trittplatte der Fußmaschine")
    hf = B["hihat-footboard"]
    add("hihat-toe", "hihat-footboard", hf.world((0.22, 0.005, 0)), "load", "Ball of the left foot on the hi-hat controller", "Fußballen links auf dem Hi-Hat-Controller")
    ang = math.radians(MODULE_ANGLE)
    top = (MODULE_C[0] - math.sin(ang) * 0.103, MODULE_C[1] + math.cos(ang) * 0.103, MODULE_C[2])
    add("module-top", "rack", top, "emitter", "Top edge of the sound module, where its warm air leaves", "Oberkante des Soundmoduls, wo seine warme Luft aufsteigt")
    return A


def masses(B, parts):
    """⚖️ Bone masses and centres from the 1:50 parts (component masses of the class, distributed over their solids)."""
    acc = {}
    for p in parts:
        if p.mass <= 0:
            continue
        n = len(p.pts)
        c = tuple(sum(q[i] for q in p.pts) / n for i in range(3))
        m, s = acc.get(p.bone, (0.0, (0.0, 0.0, 0.0)))
        acc[p.bone] = (m + p.mass, tuple(s[i] + p.mass * c[i] for i in range(3)))
    out = {}
    for bone, (m, s) in acc.items():
        c = tuple(x / m for x in s)
        out[bone] = (m, c)
    return out


def region(rid, en, de, kind, model, material, area, length, geometry, layer, extra=None):
    """🌡️ One thermal region with its one layer."""
    lid, len_, lde, lkind, lmat, t, k, rho, cp, src = layer
    r = {"id": rid, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": area, "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.4,
         "layers": [{"id": lid, "title": T(len_, lde), "kind": lkind, "material": lmat, "thicknessM": t, "resistanceM2KW": float(f"{t / k:.3g}"), "heatCapacityJM2K": round(rho * cp * t), "source": src, "reference": "iso-10456-2007" if src == "literature" else "incropera-2018"}],
         "heatCapacityJM2K": round(rho * cp * t)}
    if extra:
        r.update(extra)
    r["source"] = "authored-estimate"
    r["reference"] = "incropera-2018"
    return r


AREAS = {"rack": 1.0, "pad-shells": 0.8, "mesh-heads": 0.2, "cymbal-pads": 0.35, "module": 0.2, "kick-tower": 0.5, "pedals": 0.3, "throne-seat": 0.2, "throne-base": 0.2}
AREA_FILE = os.path.join(HERE, "author-areas.json")
if os.path.exists(AREA_FILE):
    AREAS.update(json.load(open(AREA_FILE)))


def thermal():
    """🌡️ The thermal model: nine passive regions and the module, warmed by its 6 W, the plume above it, in a showroom at 21 °C."""
    R = [
        region("rack", "Rack tubes, clamps and boom arms", "Rackrohre, Klemmen und Galgenarme", "surface", "passive", "alu-anodised", AREAS["rack"], 0.038, "cylinder", ("tube-wall", "Aluminium tube wall 2 mm", "Aluminiumrohrwand 2 mm", "metal", "alu-anodised", 0.002, 160, 2700, 900, "literature")),
        region("pad-shells", "Pad shells and rubber rims of snare and toms", "Padschalen und Gummiränder von Snare und Toms", "surface", "passive", "abs-shell", AREAS["pad-shells"], 0.23, "cylinder", ("abs-wall", "ABS shell 3 mm", "ABS-Schale 3 mm", "polymer", "abs-shell", 0.003, 0.17, 1050, 1400, "literature")),
        region("mesh-heads", "Mesh heads, worn grey at the centre", "Meshfelle, in der Mitte grau abgespielt", "surface", "passive", "mesh-polyester", AREAS["mesh-heads"], 0.25, "horizontal-plate-up", ("mesh", "Two-ply polyester mesh 1 mm", "Zweilagiges Polyestergewebe 1 mm", "polymer", "mesh-polyester", 0.001, 0.15, 1380, 1200, "authored-estimate")),
        region("cymbal-pads", "Rubber cymbal pads (hi-hat, crash, ride)", "Gummi-Beckenpads (Hi-Hat, Crash, Ride)", "surface", "passive", "rubber-epdm", AREAS["cymbal-pads"], 0.3, "horizontal-plate-up", ("rubber", "Rubber skin 4 mm over the pad core", "Gummihaut 4 mm über dem Padkern", "polymer", "rubber-epdm", 0.004, 0.25, 1200, 1600, "literature")),
        region("module", "Sound module housing", "Gehäuse des Soundmoduls", "surface", "operating", "abs-module", AREAS["module"], 0.21, "vertical-plate", ("housing", "ABS housing 2.5 mm", "ABS-Gehäuse 2,5 mm", "polymer", "abs-module", 0.0025, 0.17, 1050, 1400, "literature"), {"operating": {"key": "power", "heatW": 4.5}, "ratedPowerW": 6, "ratedPowerBasis": "electrical-input"}),
        region("kick-tower", "Kick tower frame and kick pad shell", "Rahmen des Kickturms und Schale des Kickpads", "surface", "passive", "steel-black", AREAS["kick-tower"], 0.4, "vertical-plate", ("steel-wall", "Powder-coated steel tube 1.5 mm", "Pulverbeschichtetes Stahlrohr 1,5 mm", "metal", "steel-black", 0.0015, 50, 7850, 460, "literature")),
        region("pedals", "Kick pedal and hi-hat controller", "Fußmaschine und Hi-Hat-Controller", "surface", "passive", "steel-black", AREAS["pedals"], 0.3, "horizontal-plate-up", ("plate", "Steel and cast aluminium 3 mm", "Stahl und Aluminiumguss 3 mm", "metal", "steel-black", 0.003, 50, 7850, 460, "literature")),
        region("throne-seat", "Throne cushion, cracked vinyl with tape", "Hockerpolster, rissiges Kunstleder mit Klebeband", "surface", "passive", "pvc-vinyl", AREAS["throne-seat"], 0.33, "horizontal-plate-up", ("cushion", "Foam cushion with PVC skin 60 mm", "Schaumpolster mit PVC-Haut 60 mm", "polymer", "pvc-vinyl", 0.06, 0.04, 120, 1400, "authored-estimate")),
        region("throne-base", "Chromed throne tripod and tube", "Verchromtes Hockerdreibein und Rohr", "surface", "passive", "steel-chrome", AREAS["throne-base"], 0.03, "cylinder", ("chrome-wall", "Chromed steel tube 1.5 mm", "Verchromtes Stahlrohr 1,5 mm", "metal", "steel-chrome", 0.0015, 50, 7850, 460, "literature")),
    ]
    return {
        "environment": {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
        "passengers": [],
        "operating": [{"id": "power", "title": T("Sound module power", "Stromversorgung des Soundmoduls"), "levels": [{"v": 0, "title": T("Off (unplugged)", "Aus (ausgesteckt)")}, {"v": 1, "title": T("On (6 W at the adapter)", "An (6 W am Netzteil)")}]}],
        "regions": R,
        "emitters": [{"id": "module-plume", "kind": "plume", "title": T("Free convection above the warm module", "Freie Konvektion über dem warmen Modul"), "anchor": "module-top", "directionDeg": 0, "diameterM": 0.2, "region": "module", "source": "authored-estimate", "reference": "incropera-2018"}],
        "assumptions": [
            T("Only the sound module draws power: about 6 W at the 9 V plug-in adapter, of which about 4.5 W leave the module housing as heat and the rest in the adapter at the wall socket, which is not part of the asset. The pads, cables and pedals carry only sensor currents and are passive.", "Nur das Soundmodul nimmt Leistung auf: etwa 6 W am 9-V-Steckernetzteil, davon verlassen etwa 4,5 W das Modulgehäuse als Wärme und der Rest das Netzteil an der Steckdose, das nicht Teil des Assets ist. Pads, Kabel und Pedale führen nur Sensorströme und sind passiv."),
            T("Indoor showroom of a used-instrument dealer, 21 °C, no sun; aged surfaces: worn anodising (emissivity 0.77 instead of about 0.8), dulled chrome (0.18 instead of about 0.07), scuffed rubber and a cracked vinyl cushion.", "Ausstellungsraum eines Gebrauchtinstrumentenhändlers, 21 °C, keine Sonne; gealterte Oberflächen: abgegriffene Eloxierung (Emissionsgrad 0,77 statt etwa 0,8), matt gewordenes Chrom (0,18 statt etwa 0,07), verschrammter Gummi und ein rissiges Kunstlederpolster."),
            T("Played: the seated drummer warms the cushion through the contact of the thighs (32 °C over 45 % of its area); the strikes themselves add no measurable heat.", "Gespielt: der sitzende Schlagzeuger erwärmt das Polster über den Kontakt der Oberschenkel (32 °C auf 45 % der Fläche); die Schläge selbst bringen keine messbare Wärme ein."),
        ],
    }


def binding(power, seated=False):
    """🔗 The thermal binding of a pose: every region and the plume."""
    regs = {}
    for rid in REGION_IDS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if rid in ("pedals", "kick-tower", "throne-base"):
            b["contact"] = {"fraction": 0.05, "tempC": 20}
        if rid == "throne-seat" and seated:
            b = {"exposedFraction": 0.55, "sunlitFraction": 0, "contact": {"fraction": 0.45, "tempC": 32}}
        regs[rid] = b
    return {"operating": {"power": power}, "regions": regs, "emitters": {"module-plume": power}}


def load(lid, kind, en, de, anchor, fy, fx=0.0, src="authored-estimate", ref="dahl-2004"):
    """🧲 An applied load at an anchor."""
    return {"id": lid, "kind": kind, "title": T(en, de), "anchor": anchor, "fxN": fx, "fyN": fy, "source": src, "reference": ref}


PLAY_BONES = {"crash-boom": {"angleDeg": CRASH_PLAY[0]}, "crash": {"angleDeg": CRASH_PLAY[1]}, "ride-boom": {"angleDeg": RIDE_PLAY[0]}, "ride": {"angleDeg": RIDE_PLAY[1]}, "throne-seat": {"dx": SEAT_RAISE}}
PLAY_LOADS = [
    load("seated-drummer", "occupant", "Seated drummer on the cushion (75 kg, 550 N on the seat)", "Sitzender Schlagzeuger auf dem Polster (75 kg, 550 N auf dem Sitz)", "seat-centre", -550.0, ref="din-33402-2"),
    load("right-foot", "live", "Right foot resting on the kick footboard", "Rechter Fuß auf der Trittplatte der Fußmaschine", "kick-toe", -80.0),
    load("left-foot", "live", "Left foot holding the hi-hat controller closed", "Linker Fuß hält den Hi-Hat-Controller geschlossen", "hihat-toe", -60.0),
    load("stick-snare", "live", "Quasi-static share of a stick stroke on the snare", "Quasistatischer Anteil eines Stockschlags auf die Snare", "snare-head", -40.0, ref="fletcher-rossing-1998"),
]


def poses():
    """🧍 The named poses (stored, carried, played) and the key poses of the clips."""
    stored_contacts = FEET + PERMANENT
    P = [
        {"id": "stored", "title": T("Stored at the dealer, booms folded", "Beim Händler abgestellt, Galgen eingeklappt"), "description": T("The used kit as it stands in the dealer's back room: crash and ride booms folded down beside the posts with their pads hanging upright, the throne at its lowest height, the module unplugged, the price tag tied to the crossbar.", "Das gebrauchte Set, wie es im Lager des Händlers steht: Crash- und Ride-Galgen neben den Säulen nach unten geklappt, ihre Pads hängen senkrecht, der Hocker in der niedrigsten Höhe, das Modul ausgesteckt, das Preisschild am Querrohr."), "bones": {}, "ground": "auto", "contacts": stored_contacts, "thermal": binding(0), "loads": []},
        {"id": "carried", "title": T("Folded rack carried", "Gefaltetes Rack getragen"), "description": T("One person lifts the folded rack with its pads and module 0.25 m off the floor by the crossbar, both hands on it; kick tower, pedals and throne stay behind on the floor.", "Eine Person hebt das gefaltete Rack mit Pads und Modul am Querrohr 0,25 m vom Boden, beide Hände am Rohr; Kickturm, Pedale und Hocker bleiben auf dem Boden."), "bones": {"rack": {"dy": 0.25}}, "ground": "auto", "contacts": GRIPS + PERMANENT, "thermal": binding(0), "loads": []},
        {"id": "played", "title": T("Played seated", "Im Sitzen gespielt"), "description": T("Set up for playing: crash and ride booms raised with the pads tilted towards the drummer, the throne raised to 0.57 m, the module switched on; the seated drummer's weight, both feet and a stick stroke load the kit.", "Zum Spielen aufgebaut: Crash- und Ride-Galgen hochgestellt, die Pads zum Schlagzeuger geneigt, der Hocker auf 0,57 m angehoben, das Modul eingeschaltet; das Gewicht des sitzenden Schlagzeugers, beide Füße und ein Stockschlag belasten das Set."), "bones": dict(PLAY_BONES), "ground": "auto", "contacts": stored_contacts, "thermal": binding(1, True), "loads": PLAY_LOADS},
        {"id": "kick-hit", "key": True, "title": T("Kick beater on the pad", "Schlägel am Kickpad"), "description": T("The footboard pressed down by 12 degrees and the felt beater on the kick mesh head.", "Die Trittplatte um 12 Grad niedergedrückt und der Filzschlägel am Kick-Meshfell."), "bones": {**PLAY_BONES, "kick-footboard": {"angleDeg": -12}, "kick-beater": {"angleDeg": BEATER_STRIKE}}, "ground": "auto", "contacts": stored_contacts, "thermal": binding(1, True), "loads": [PLAY_LOADS[0], load("right-foot", "live", "Right foot pressing the kick footboard", "Rechter Fuß drückt die Trittplatte der Fußmaschine", "kick-toe", -220.0), PLAY_LOADS[2], PLAY_LOADS[3]]},
        {"id": "booms-up", "key": True, "title": T("Booms raised, throne still low", "Galgen hochgestellt, Hocker noch tief"), "description": T("Half-way through setting up: the booms are raised, the throne is still at its lowest height and the module is off.", "Mitten im Aufbau: die Galgen sind hochgestellt, der Hocker ist noch in der niedrigsten Höhe und das Modul ist aus."), "bones": {k: v for k, v in PLAY_BONES.items() if k != "throne-seat"}, "ground": "auto", "contacts": stored_contacts, "thermal": binding(0), "loads": []},
    ]
    return P


def spans(anchors_, a, b):
    """⏱️ Contact spans of several anchors."""
    return [{"anchor": x, "fromS": a, "toS": b} for x in anchors_]


def clips():
    """🎞️ carry (stored → carried → stored), kick-stroke (beater strike and rebound, looping) and set-up (unfold, raise, switch on)."""
    return [
        {"id": "carry", "title": T("Carry the folded rack", "Gefaltetes Rack tragen"), "description": T("A person takes the folded rack by the crossbar, lifts it 0.25 m, holds it for three seconds and sets it down again; the kick tower, the pedals and the throne stay on the floor.", "Eine Person fasst das gefaltete Rack am Querrohr, hebt es 0,25 m an, hält es drei Sekunden und setzt es wieder ab; Kickturm, Pedale und Hocker bleiben auf dem Boden."), "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored", "ease": "smooth"}, {"t": 2.5, "pose": "carried", "ease": "smooth"}, {"t": 5.5, "pose": "carried", "ease": "smooth"}, {"t": 7.0, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "stored", "ease": "smooth"}],
         "contacts": spans(PERMANENT, 0, 8) + spans(FEET, 0, 1.05) + spans(FEET, 6.95, 8) + spans(GRIPS, 0.5, 7.5),
         "phases": [{"id": "grip", "fromS": 0, "toS": 1.0, "title": T("Hands on the crossbar", "Hände am Querrohr")}, {"id": "lift", "fromS": 1.0, "toS": 2.5, "title": T("Lifting", "Anheben")}, {"id": "hold", "fromS": 2.5, "toS": 5.5, "title": T("Carried", "Getragen")}, {"id": "set-down", "fromS": 5.5, "toS": 8, "title": T("Setting down", "Absetzen")}],
         "events": [{"t": 1.05, "kind": "contact-off", "title": T("Rack feet leave the floor", "Rackfüße verlassen den Boden"), "target": "foot-l-front"}, {"t": 6.95, "kind": "contact-on", "title": T("Rack feet back on the floor", "Rackfüße wieder auf dem Boden"), "target": "foot-l-front"}],
         "tracks": []},
        {"id": "kick-stroke", "title": T("Kick stroke", "Kickschlag"), "description": T("Played seated: the right foot presses the footboard, the felt beater swings 37 degrees onto the kick mesh head and rebounds, once every 0.75 s in a steady beat.", "Im Sitzen gespielt: der rechte Fuß drückt die Trittplatte, der Filzschlägel schwingt 37 Grad auf das Kick-Meshfell und federt zurück, einmal alle 0,75 s im gleichmäßigen Takt."), "durationS": 0.75, "fps": 40, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "in"}, {"t": 0.12, "pose": "kick-hit", "ease": "out"}, {"t": 0.34, "pose": "played", "ease": "smooth"}, {"t": 0.75, "pose": "played", "ease": "smooth"}],
         "contacts": spans(PERMANENT + FEET, 0, 0.75),
         "phases": [{"id": "press", "fromS": 0, "toS": 0.12, "title": T("Foot presses, beater swings", "Fuß drückt, Schlägel schwingt")}, {"id": "rebound", "fromS": 0.12, "toS": 0.34, "title": T("Beater rebounds", "Schlägel federt zurück")}, {"id": "rest", "fromS": 0.34, "toS": 0.75, "title": T("Beater at rest", "Schlägel in Ruhe")}],
         "events": [{"t": 0.12, "kind": "motion", "title": T("Beater strikes the kick pad", "Schlägel trifft das Kickpad"), "target": "kick-beater"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 1}]}]},
        {"id": "set-up", "title": T("Set up and switch on", "Aufbauen und einschalten"), "description": T("From storage to playing: the crash and ride booms swing up and their tilters turn the pads towards the drummer, the throne is raised by 9 cm, then the module is plugged in and switched on.", "Vom Lager zum Spielen: Crash- und Ride-Galgen schwenken hoch und ihre Neiger drehen die Pads zum Schlagzeuger, der Hocker wird um 9 cm angehoben, dann wird das Modul eingesteckt und eingeschaltet."), "durationS": 14, "fps": 10, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 7, "pose": "booms-up", "ease": "smooth"}, {"t": 10, "pose": "played", "ease": "smooth"}, {"t": 14, "pose": "played", "ease": "smooth"}],
         "contacts": spans(PERMANENT + FEET, 0, 14),
         "phases": [{"id": "unfold", "fromS": 0, "toS": 7, "title": T("Booms up", "Galgen hoch")}, {"id": "seat", "fromS": 7, "toS": 10, "title": T("Throne raised", "Hocker angehoben")}, {"id": "power", "fromS": 10, "toS": 14, "title": T("Module on", "Modul an")}],
         "events": [{"t": 11, "kind": "operating", "title": T("Module switched on", "Modul eingeschaltet"), "target": "power"}],
         "tracks": [{"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 11, "v": 1}]}, {"target": "emitter:module-plume", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 11, "v": 1}]}]},
    ]


PENS = {
    50: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.13, "ground": 0.35},
    100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    200: {"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25},
    500: {"body": 0.2, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
MESH_HATCH = {"id": "mesh", "title": T("Woven mesh head", "Gewebtes Meshfell"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}


def solid_doc(p):
    """🧊 The document form of a body solid."""
    d = {"id": p.id, "kind": p.solid["kind"], "bone": p.bone, "material": p.material, "role": p.role, "tone": p.tone}
    d.update({k: v for k, v in p.solid.items() if k != "kind"})
    for flag in ("slender", "sheet", "texture", "envelope"):
        if p.flags.get(flag):
            d[flag] = True
    return d


def volume(level, parts, B, anchor_list, mass_of):
    """🔧 The authored model of one scale: body, thermal by reference plus the plume outlet, and the structure (members, joints, supports, masses)."""
    F = FEATURE[level]
    body = [solid_doc(p) for p in parts]
    therm = [{"id": f"t-{p.id}", "of": p.id, "region": p.region} for p in parts if not p.flags.get("texture")]
    ang = math.radians(MODULE_ANGLE)
    top = (MODULE_C[0] - math.sin(ang) * 0.104, MODULE_C[1] + math.cos(ang) * 0.104, MODULE_C[2])
    plume = box(B, "t-module-plume", "rack", top, PLUME_SIZE[level], "abs-module", "module", "body", 0.24, angle=MODULE_ANGLE, flags={"sheet": True})
    pd = solid_doc(plume)
    pd.update({"region": "module", "tags": ["module-plume"]})
    therm.append(pd)
    st = []
    for p in parts:
        if "member" in p.flags:
            s = {"id": f"s-{p.id}", "of": p.id, "member": p.flags["member"], "part": "member"}
            if p.flags.get("members"):
                s["members"] = p.flags["members"]
            st.append(s)
        if "joint" in p.flags:
            st.append({"id": f"j-{p.id}", "of": p.id, "member": p.flags["joint"], "part": "joint"})
    amap = {a["id"]: a for a in anchor_list}
    for aid in PERMANENT:
        a = amap[aid]
        bone = B[a["bone"]]
        q = bone.world((a["x"], a["y"], a.get("z", 0.0)))
        L, W = min(a["footprintLengthM"], 0.40), min(a["footprintWidthM"], 0.40)
        if aid.startswith("throne"):
            L, W = max(0.03, F), max(0.03, F)
        sp = box(B, f"support-{aid}", a["bone"], (q[0], F / 2, q[2]), (max(L, F), F, max(W, F)), "rubber-epdm", "rack", "detail", 0.3)
        d = solid_doc(sp)
        d.update({"member": a["bone"], "part": "support", "anchor": aid})
        st.append(d)
    for bone, (m, c) in sorted(mass_of.items()):
        r = max(0.6 * F, 0.008)
        own = [p for p in parts if p.bone == bone and not p.flags.get("texture")]
        cen = lambda p: tuple(sum(q[i] for q in p.pts) / len(p.pts) for i in range(3))
        if not own:
            continue
        c = min((cen(p) for p in own), key=lambda q: math.dist(q, c))
        mp = ellipsoid(B, f"mass-{bone}", bone, c, (r, r, r), "steel-black", "rack", "detail", 0.15)
        d = solid_doc(mp)
        d.update({"member": bone, "part": "mass"})
        st.append(d)
    return {"status": "authored", "denominator": level, "body": body, "thermal": therm, "structure": st, "decisions": VOLUME_DECISIONS[level]}


PLUME_SIZE = {50: (0.06, 0.002, 0.22), 100: (0.066, 0.002, 0.24), 200: (0.07, 0.002, 0.25), 500: (0.077, 0.002, 0.26)}

VOLUME_DECISIONS = {
    50: [T("Own 1:50 model of a used two-post mesh-head e-drum kit: aluminium posts and crossbar 38 mm and feet 32 mm with die-cast T-blocks and cross clamps; snare (12 inch), two rack toms and a floor tom (8 inch) each as ABS shell, rubber rim hoop and a 3 mm mesh head sheet on die-cast brackets and 12.7 mm L-rods; hi-hat pad on its fixed boom; crash and ride pads on folding 22 mm booms with tilters and holder rods; module with display and knob on its post clamp; kick tower of 30 and 25 mm steel tubes with braces and the 10 inch mesh pad; kick pedal with base, posts, axle, heel hinge, spring, footboard, 8 mm shaft and felt beater; hi-hat controller; throne tripod with braces, lock, spindle, plate and cushion.", "Eigenes 1:50-Modell eines gebrauchten Zwei-Säulen-E-Drum-Sets mit Meshfellen: Aluminiumsäulen und Querrohr 38 mm und Füße 32 mm mit T-Stücken und Kreuzklemmen aus Druckguss; Snare (12 Zoll), zwei Racktoms und ein Standtom (8 Zoll) jeweils als ABS-Schale, Gummireif und 3-mm-Meshfell als Platte auf Druckgusshaltern und 12,7-mm-L-Stangen; Hi-Hat-Pad auf festem Galgen; Crash- und Ride-Pad auf klappbaren 22-mm-Galgen mit Neigern und Halterstangen; Modul mit Anzeige und Drehknopf an der Säulenklemme; Kickturm aus 30- und 25-mm-Stahlrohren mit Streben und dem 10-Zoll-Meshpad; Fußmaschine mit Bodenplatte, Säulen, Achse, Fersengelenk, Feder, Trittplatte, 8-mm-Schaft und Filzschlägel; Hi-Hat-Controller; Hockerdreibein mit Streben, Klemme, Spindel, Platte und Polster."),
         T("Traces of use as geometry: the grey worn patch at the centre of the snare head, the beater patch on the kick head, the taped tear in the throne vinyl, the dealer's sticker on the module and the price tag tied to the crossbar; worn anodising, dulled chrome and scuffed rubber live in the materials.", "Spielspuren als Geometrie: der grau abgespielte Fleck in der Mitte des Snarefells, der Schlägelfleck auf dem Kickfell, der geklebte Riss im Hockerbezug, der Händleraufkleber auf dem Modul und das Preisschild am Querrohr; abgegriffene Eloxierung, matt gewordenes Chrom und verschrammter Gummi stecken in den Materialien."),
         T("Thermal by reference to every body solid except the wear textures, plus the plume outlet on the top edge of the module; structure: tubes, rods, booms, footboards, shaft and plates as members by reference, clamps, pivots, hinges, axle and collars as joints, a support pad at each permanent floor contact and one mass marker per bone.", "Thermik als Verweis auf jeden Körper außer den Abnutzungstexturen, dazu der Austritt der Wärmefahne an der Oberkante des Moduls; Tragwerk: Rohre, Stangen, Galgen, Trittplatten, Schaft und Platten als Stäbe per Verweis, Klemmen, Drehlager, Gelenke, Achse und Manschetten als Knoten, ein Auflagerpolster an jedem dauernden Bodenkontakt und eine Massenmarke je Knochen.")],
    100: [T("Own 1:100 model: each pad is one turned body with its rim lip (shell and hoop merged) and its mesh head as a 3 mm sheet; the module display stays, the knob, buttons and sticker are below 15 mm and omitted; the price tag, the wear patches and the taped tear are omitted; the pedal plates are 12 to 15 mm; the throne braces and their collar (12 mm) are omitted, the kick braces (20 mm) stay; the 12.7 mm rods and holders keep their true section because they are drawn at it.", "Eigenes 1:100-Modell: jedes Pad ist ein gedrehter Körper mit Randwulst (Schale und Reif zusammengefasst) und sein Meshfell eine 3-mm-Platte; die Modulanzeige bleibt, Drehknopf, Tasten und Aufkleber liegen unter 15 mm und entfallen; Preisschild, Abnutzungsflecken und geklebter Riss entfallen; die Pedalplatten sind 12 bis 15 mm dick; die Hockerstreben und ihre Manschette (12 mm) entfallen, die Kickstreben (20 mm) bleiben; die 12,7-mm-Stangen und Halter behalten ihren wahren Querschnitt, weil sie so gezeichnet sind."),
          T("Thermal and structure as at 1:50 by reference to the merged solids of this scale.", "Thermik und Tragwerk wie bei 1:50 als Verweis auf die zusammengefassten Körper dieses Maßstabs.")],
    200: [T("Own 1:200 model: rods and holders widen to 20 mm, the cymbal pads to 16 mm at the edge and 32 mm at the bell; the kick pedal posts merge into one frame block, the hi-hat base and housing into one 35 mm block; the module is a plain box without display; the kick braces are omitted; pads as at 1:100.", "Eigenes 1:200-Modell: Stangen und Halter werden 20 mm dick, die Beckenpads am Rand 16 mm und an der Glocke 32 mm; die Säulen der Fußmaschine gehen in einem Rahmenblock auf, Basis und Gehäuse des Hi-Hat-Controllers in einem 35-mm-Block; das Modul ist ein schlichter Kasten ohne Anzeige; die Kickstreben entfallen; Pads wie bei 1:100."),
          T("Thermal and structure by reference to the solids of this scale; the plume outlet stays on the module.", "Thermik und Tragwerk als Verweis auf die Körper dieses Maßstabs; der Austritt der Wärmefahne bleibt am Modul.")],
    500: [T("Own 1:500 model, the kit is 3 mm on paper: tubes, booms and rods at their true 22 to 38 mm (20 mm for the rods) as drawn lines, pads as turned discs with their head sheets, cymbal pads as flat 20 mm discs, the module as a box; the clamps, brackets, T-blocks and collars (60 mm and less) are left out and the tubes meet directly; the kick pedal is one wedge block that holds the footboard, the hi-hat controller one 80 mm block, both named as members of their footboards; the beater keeps its shaft and felt head; the throne is its tube, three legs and one turned seat.", "Eigenes 1:500-Modell, das Set ist auf Papier 3 mm groß: Rohre, Galgen und Stangen in ihrem wahren Querschnitt von 22 bis 38 mm (20 mm für die Stangen) als gezeichnete Linien, Pads als gedrehte Scheiben mit ihren Fellplatten, Beckenpads als flache 20-mm-Scheiben, das Modul als Kasten; Klemmen, Halter, T-Stücke und Manschetten (60 mm und weniger) entfallen und die Rohre treffen sich direkt; die Fußmaschine ist ein Keilblock, der die Trittplatte enthält, der Hi-Hat-Controller ein 80-mm-Block, beide als Stäbe ihrer Trittplatten benannt; der Schlägel behält Schaft und Filzkopf; der Hocker ist sein Rohr, drei Beine und ein gedrehter Sitz."),
          T("Thermal by reference with the plume outlet; structure: tubes and blocks as members, the pivots as boom joints, supports and mass markers at one feature.", "Thermik als Verweis mit dem Austritt der Wärmefahne; Tragwerk: Rohre und Blöcke als Stäbe, die Drehlager als Galgenknoten, Auflager und Massenmarken mit einem Merkmal.")],
}

REP_DECISIONS = {
    50: {
        "silhouette": T("Side elevation 1.48 m long and 1.0 m high, 30 x 20 mm on paper: the throne on its tripod at the left, the two pedals, the kick tower with its round pad, the rack posts with the hanging crash and ride pads, the module tilted towards the drummer, the snare low on its arm and the toms above the crossbar. Front 1.54 m wide: both posts with their feet, the crossbar, the hanging cymbal pads beside the posts, the kick tower in the middle. Plan: the pads as round mesh heads, the T-feet, the tripod and the cymbals seen edge-on.", "Seitenansicht 1,48 m lang und 1,0 m hoch, auf Papier 30 x 20 mm: links der Hocker auf seinem Dreibein, die beiden Pedale, der Kickturm mit seinem runden Pad, die Racksäulen mit den hängenden Crash- und Ride-Pads, das zum Schlagzeuger geneigte Modul, die Snare tief an ihrem Arm und die Toms über dem Querrohr. Vorderansicht 1,54 m breit: beide Säulen mit Füßen, das Querrohr, die hängenden Beckenpads neben den Säulen, der Kickturm in der Mitte. Grundriss: die Pads als runde Meshfelle, die T-Füße, das Dreibein und die Becken in Kantenansicht."),
        "retained": T("Every tube, clamp, rod, bracket and pad; the holder rods and pivots of the folding booms; module, display and knob; kick tower braces and pad brackets; pedal posts, axle, heel hinges and spring; throne braces, collar and lock; the price tag and the wear patches.", "Jedes Rohr, jede Klemme, Stange, Halterung und jedes Pad; Halterstangen und Drehlager der Klappgalgen; Modul, Anzeige und Drehknopf; Kickturm-Streben und Padhalter; Pedalsäulen, Achse, Fersengelenke und Feder; Hockerstreben, Manschette und Klemme; das Preisschild und die Abnutzungsflecken."),
        "merged": T("Shell and rim of each pad are drawn as one outline, the mesh head as its own filled disc in the plan.", "Schale und Reif jedes Pads sind eine Umrisslinie, das Meshfell eine eigene gefüllte Scheibe im Grundriss."),
        "omitted": T("Cables, the drive strap of the kick pedal, screws and wing nuts; the kick head faces the drummer and is seen in none of the three views.", "Kabel, das Antriebsband der Fußmaschine, Schrauben und Flügelmuttern; das Kickfell blickt zum Schlagzeuger und ist in keiner der drei Ansichten zu sehen."),
        "thinElements": T("Rods 12.7 mm, holders 12 mm, throne braces 12 mm and the beater shaft 8 mm are filled capsules of their true width; the 3 mm heads and the cymbal edges keep their real thickness.", "Stangen 12,7 mm, Halter 12 mm, Hockerstreben 12 mm und der Schlägelschaft 8 mm sind gefüllte Kapseln in wahrer Breite; die 3-mm-Felle und die Beckenränder behalten ihre echte Dicke."),
        "hatching": T("The mesh heads carry a 0.5 mm cross hatch in the plan.", "Die Meshfelle tragen im Grundriss eine Kreuzschraffur von 0,5 mm."),
        "styles": T("Aluminium tubes light grey 0.6, clamps 0.34, chrome 0.7, pad shells and rubber black 0.14 to 0.22, mesh 0.42, module 0.24 with a 0.55 display, kick tower and pedal frames 0.2 to 0.3, footboards 0.5, cushion 0.1, price tag 0.95.", "Aluminiumrohre hellgrau 0,6, Klemmen 0,34, Chrom 0,7, Padschalen und Gummi schwarz 0,14 bis 0,22, Mesh 0,42, Modul 0,24 mit 0,55 Anzeige, Kickturm und Pedalrahmen 0,2 bis 0,3, Trittplatten 0,5, Polster 0,1, Preisschild 0,95."),
        "overlays": T("Thermal: the module is the only warm part when played; structure: the rack on its four rubber caps, the permanent floor contacts of kick tower, pedals and throne, the grips when carried.", "Thermik: das Modul ist im Spiel der einzige warme Teil; Tragwerk: das Rack auf seinen vier Gummikappen, die dauernden Bodenkontakte von Kickturm, Pedalen und Hocker, die Griffe beim Tragen."),
    },
    100: {
        "silhouette": T("The same three views at 15 x 10 mm: pads as one outline each with their mesh heads in the plan, all tubes and rods at true width.", "Dieselben drei Ansichten mit 15 x 10 mm: Pads als je ein Umriss mit ihren Meshfellen im Grundriss, alle Rohre und Stangen in wahrer Breite."),
        "retained": T("Tubes, clamps, rods, brackets, pads, booms with holders, module with display, kick tower with braces, pedals with posts, axle and hinges, throne with collar and lock.", "Rohre, Klemmen, Stangen, Halter, Pads, Galgen mit Haltern, Modul mit Anzeige, Kickturm mit Streben, Pedale mit Säulen, Achse und Gelenken, Hocker mit Manschette und Klemme."),
        "merged": T("Shell and rim of each pad are one turned body.", "Schale und Reif jedes Pads sind ein gedrehter Körper."),
        "omitted": T("Price tag, wear patches, sticker, buttons, knob, spring, throne braces and their collar (below 15 mm), cables and strap.", "Preisschild, Abnutzungsflecken, Aufkleber, Tasten, Drehknopf, Feder, Hockerstreben und ihre Manschette (unter 15 mm), Kabel und Band."),
        "thinElements": T("Rods and holders 12 to 13 mm and the 8 mm beater shaft as filled capsules; head sheets 3 mm.", "Stangen und Halter 12 bis 13 mm und der 8-mm-Schlägelschaft als gefüllte Kapseln; Fellplatten 3 mm."),
        "hatching": T("Mesh heads cross-hatched at 0.5 mm in the plan.", "Meshfelle im Grundriss mit 0,5 mm kreuzschraffiert."),
        "styles": T("Tones as at 1:50.", "Tonwerte wie bei 1:50."),
        "overlays": T("As at 1:50.", "Wie bei 1:50."),
    },
    200: {
        "silhouette": T("7 x 5 mm: rack frame, pad outlines, cymbal pads, module box, kick tower, two pedal blocks and the throne.", "7 x 5 mm: Rackrahmen, Padumrisse, Beckenpads, Modulkasten, Kickturm, zwei Pedalblöcke und der Hocker."),
        "retained": T("Tubes, clamps, 20 mm rods, pads with heads, booms, module, kick tower frame without braces, pedal frames and footboards, throne.", "Rohre, Klemmen, 20-mm-Stangen, Pads mit Fellen, Galgen, Modul, Kickturmrahmen ohne Streben, Pedalrahmen und Trittplatten, Hocker."),
        "merged": T("Kick pedal posts into one frame block, hi-hat base and housing into one block.", "Säulen der Fußmaschine zu einem Rahmenblock, Basis und Gehäuse des Hi-Hat-Controllers zu einem Block."),
        "omitted": T("Display, knob, kick braces and every part already omitted at 1:100.", "Anzeige, Drehknopf, Kickstreben und jeder schon bei 1:100 weggelassene Teil."),
        "thinElements": T("The beater shaft stays a true 8 mm line; rods are drawn 20 mm wide.", "Der Schlägelschaft bleibt eine wahre 8-mm-Linie; Stangen sind 20 mm breit gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:50.", "Tonwerte wie bei 1:50."),
        "overlays": T("As at 1:50.", "Wie bei 1:50."),
    },
    500: {
        "silhouette": T("3 x 2 mm: the rack as lines, pads and cymbal pads as small discs, the module, the kick tower, two pedal blocks and the throne.", "3 x 2 mm: das Rack als Linien, Pads und Beckenpads als kleine Scheiben, das Modul, der Kickturm, zwei Pedalblöcke und der Hocker."),
        "retained": T("Posts, feet, crossbar, booms, rods, pads, cymbal pads, module, kick tower tubes and pad, pedal blocks, beater, throne tube, legs and seat.", "Säulen, Füße, Querrohr, Galgen, Stangen, Pads, Beckenpads, Modul, Kickturmrohre und -pad, Pedalblöcke, Schlägel, Hockerrohr, Beine und Sitz."),
        "merged": T("Each pedal into one block with its footboard; seat plate, spindle and cushion into one turned seat.", "Jedes Pedal mit seiner Trittplatte zu einem Block; Sitzplatte, Spindel und Polster zu einem gedrehten Sitz."),
        "omitted": T("Clamps, brackets, T-blocks, collars, holders' detail and every part omitted at 1:200.", "Klemmen, Halter, T-Stücke, Manschetten, Halterdetails und jeder bei 1:200 weggelassene Teil."),
        "thinElements": T("Tubes 22 to 38 mm and rods 20 mm are filled capsules that read as lines with the 0.2 mm pen; the beater shaft is drawn 20 mm.", "Rohre 22 bis 38 mm und Stangen 20 mm sind gefüllte Kapseln, die mit dem 0,2-mm-Stift als Linien lesbar sind; der Schlägelschaft ist 20 mm gezeichnet."),
        "hatching": T("None.", "Keine."),
        "styles": T("Tones as at 1:50.", "Tonwerte wie bei 1:50."),
        "overlays": T("As at 1:50.", "Wie bei 1:50."),
    },
}


def main():
    """🖨️ Builds every scale and writes the asset document."""
    bones, B = rig()
    per = {lvl: parts_for(lvl, B) for lvl in SCALES}
    mass_of = masses(B, per[50])
    for b in bones:
        if b.mass is not None:
            if b.id in mass_of:
                m, c = mass_of[b.id]
                l = b.local(c)
                b.mass = {"kg": round(m, 3), "comX": r6(l[0]), "comY": r6(l[1]), "distribution": "point", "source": "authored-estimate", "reference": "roland-td17-2018"}
            else:
                b.mass = None
    pts = [q for p in per[50] for q in p.pts]
    lo = [min(q[i] for q in pts) for i in range(3)]
    hi = [max(q[i] for q in pts) for i in range(3)]
    total = round(sum(m for m, _ in mass_of.values()), 2)
    dims = {"heightM": round(hi[1], 3), "widthM": round(hi[0] - lo[0], 3), "depthM": round(hi[2] - lo[2], 3), "massKg": total}
    A = anchors(B)
    rig_doc = {"kind": "mechanical", "root": "datum", "bones": []}
    for b in bones:
        d = {"id": b.id, "parent": b.parent, "title": b.title, "x": r6(b.x), "y": r6(b.y)}
        if b.z:
            d["z"] = r6(b.z)
        d.update({"angleDeg": b.angle, "lengthM": b.length, "joint": b.joint})
        if b.mass:
            d["mass"] = b.mass
        if b.section:
            d["section"] = b.section
        rig_doc["bones"].append(d)
    reps, vols = {}, {}
    for lvl in SCALES:
        v = shapes_for(lvl, per[lvl], B)
        reps[str(lvl)] = {"status": "authored", "denominator": lvl, "validRange": {"minDenominator": RANGES[lvl][0], "maxDenominator": RANGES[lvl][1]}, "pens": PENS[lvl], "hatches": [MESH_HATCH] if lvl <= 100 else [], "geometry": v, "decisions": REP_DECISIONS[lvl]}
        vols[str(lvl)] = volume(lvl, per[lvl], B, A, mass_of)
    asset = {
        "id": "used-electronic-drum-kit",
        "category": "appliances",
        "title": T("Used Electronic Drum Kit", "Gebrauchtes E-Drum-Set"),
        "summary": T(f"A used mesh-head electronic drum kit as a dealer stores it: a two-post aluminium rack 1.0 m high with a 12-inch snare, three 8-inch toms, hi-hat, crash and ride pads on folding booms and the sound module, a kick tower with pedal, a hi-hat controller and a drum throne, {dims['massKg']:.1f} kg together. The snare and kick heads are worn grey where they were struck, the cushion is taped, the module draws 6 W when switched on.", f"Ein gebrauchtes E-Drum-Set mit Meshfellen, wie es ein Händler lagert: ein 1,0 m hohes Zwei-Säulen-Aluminiumrack mit 12-Zoll-Snare, drei 8-Zoll-Toms, Hi-Hat-, Crash- und Ride-Pads auf Klappgalgen und dem Soundmodul, ein Kickturm mit Fußmaschine, ein Hi-Hat-Controller und ein Schlagzeughocker, zusammen {dims['massKg']:.1f} kg".replace(".", ",", 1) + ". Snare- und Kickfell sind an den Schlagstellen grau abgespielt, das Polster ist geklebt, das Modul nimmt eingeschaltet 6 W auf."),
        "tags": ["used", "instrument", "music", "second-hand", "percussion", "stage", "electronic", "drum-kit", "mesh-head", "rack", "aluminium", "rubber", "electrical"],
        "provenance": {"author": "Semio staffage content (used-instruments wave, row 16)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": T(PROV_EN, PROV_DE)},
        "dimensions": dims,
        "materials": materials(),
        "rig": rig_doc,
        "anchors": A,
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(),
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": PERMANENT, "assumptions": STRUCT_ASSUMPTIONS, "clinical": False},
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "kick-stroke", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    with open(os.path.join(HERE, "🔣️.json"), "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    print("[DEBUG] dims", dims, "masses", {k: round(v[0], 2) for k, v in mass_of.items()})
    for lvl in SCALES:
        print("[DEBUG]", lvl, {k: len(v) for k, v in reps[str(lvl)]["geometry"].items()}, {k: len(vols[str(lvl)][k]) for k in ("body", "thermal", "structure")})


PROV_EN = "A used electronic drum kit of the mid-range mesh-head class of the 2010s and 2020s (Roland TD-17KV, Alesis, Yamaha DTX classes as types, no brand or logo): Hornbostel-Sachs (MIMO 2011) class 53, electronic instruments, played through percussion controllers. Two-post aluminium rack (posts 38 mm at z +-0.40 m, 1.0 m high, crossbar 38 mm at 0.70 m, T-feet 32 mm x 0.60 m) with a 12-inch dual-zone mesh snare pad (330 mm over the rim, head 0.62 m above the floor), two 8-inch rack toms (236 mm) on L-rods tilted 20 degrees and an 8-inch floor tom on the right post, a 10-inch hi-hat pad on a fixed boom, a 12-inch crash and a 13-inch ride pad on 0.30 m booms that fold down beside the posts, the sound module (272 x 206 x 78 mm) on the right post, a kick tower 0.42 m high with a 10-inch mesh pad, a kick pedal with felt beater, a hi-hat controller pedal and a tripod throne with a 0.33 m seat, 0.48 m stored and 0.57 m played. Masses are component masses of the class (pads 1.9 to 2.4 kg, cymbal pads 0.6 to 1.35 kg, module 1.2 kg, kick tower 3.7 kg with its pad, throne 4.3 kg), the tubes checked as section x density. Stored at the dealer with the booms folded and the throne low, carried folded by the crossbar with kick tower, pedals and throne left behind, played seated with the booms up."
PROV_DE = "Ein gebrauchtes E-Drum-Set der Mittelklasse mit Meshfellen der 2010er und 2020er Jahre (Klassen Roland TD-17KV, Alesis, Yamaha DTX als Typen, keine Marke und kein Logo): Hornbostel-Sachs (MIMO 2011) Klasse 53, elektronische Instrumente, gespielt über Schlagzeug-Controller. Zwei-Säulen-Aluminiumrack (Säulen 38 mm bei z +-0,40 m, 1,0 m hoch, Querrohr 38 mm auf 0,70 m, T-Füße 32 mm x 0,60 m) mit einem 12-Zoll-Dual-Zone-Mesh-Snarepad (330 mm über dem Reif, Fell 0,62 m über dem Boden), zwei 8-Zoll-Racktoms (236 mm) auf L-Stangen, 20 Grad geneigt, und einem 8-Zoll-Standtom an der rechten Säule, einem 10-Zoll-Hi-Hat-Pad auf festem Galgen, einem 12-Zoll-Crash- und einem 13-Zoll-Ride-Pad auf 0,30-m-Galgen, die neben den Säulen nach unten klappen, dem Soundmodul (272 x 206 x 78 mm) an der rechten Säule, einem 0,42 m hohen Kickturm mit 10-Zoll-Meshpad, einer Fußmaschine mit Filzschlägel, einem Hi-Hat-Controllerpedal und einem Dreibeinhocker mit 0,33-m-Sitz, 0,48 m abgestellt und 0,57 m gespielt. Die Massen sind Bauteilmassen der Klasse (Pads 1,9 bis 2,4 kg, Beckenpads 0,6 bis 1,35 kg, Modul 1,2 kg, Kickturm 3,7 kg mit Pad, Hocker 4,3 kg), die Rohre als Querschnitt mal Dichte geprüft. Beim Händler mit eingeklappten Galgen und tiefem Hocker abgestellt, gefaltet am Querrohr getragen, wobei Kickturm, Pedale und Hocker stehen bleiben, im Sitzen mit hochgestellten Galgen gespielt."
STRUCT_ASSUMPTIONS = [
    T("One rigid body on elastic supports: the rack on the rubber caps at both ends of its two T-feet when stored and played, on the two hands at the crossbar when carried; kick tower rails, pedal bases and the three throne feet stand on the floor in every pose.", "Ein starrer Körper auf elastischen Auflagern: das Rack abgestellt und gespielt auf den Gummikappen an beiden Enden seiner zwei T-Füße, getragen an den zwei Händen am Querrohr; Kufen des Kickturms, Pedalbasen und die drei Hockerfüße stehen in jeder Pose auf dem Boden."),
    T("Played loads are authored estimates: 550 N of a 75 kg drummer on the seat, 80 N of the resting right foot on the kick footboard (220 N while pressing), 60 N of the left foot on the hi-hat controller and 40 N as the quasi-static share of a stick stroke on the snare head (peak impact forces of a few hundred newtons last a few milliseconds).", "Die Spiellasten sind Schätzwerte: 550 N eines 75 kg schweren Schlagzeugers auf dem Sitz, 80 N des ruhenden rechten Fußes auf der Trittplatte (220 N beim Treten), 60 N des linken Fußes auf dem Hi-Hat-Controller und 40 N als quasistatischer Anteil eines Stockschlags auf das Snarefell (Spitzenkräfte von einigen hundert Newton dauern wenige Millisekunden)."),
    T("Rack tubes: aluminium 38 x 2 mm, 2.26 cm2, 0.61 kg/m; boom tubes steel 22 x 1.5 mm, 0.76 kg/m; the masses of pads, module and clamps are catalogue-typical component masses of the class.", "Rackrohre: Aluminium 38 x 2 mm, 2,26 cm2, 0,61 kg/m; Galgenrohre Stahl 22 x 1,5 mm, 0,76 kg/m; die Massen von Pads, Modul und Klemmen sind katalogtypische Bauteilmassen der Klasse."),
]

if __name__ == "__main__":
    main()
