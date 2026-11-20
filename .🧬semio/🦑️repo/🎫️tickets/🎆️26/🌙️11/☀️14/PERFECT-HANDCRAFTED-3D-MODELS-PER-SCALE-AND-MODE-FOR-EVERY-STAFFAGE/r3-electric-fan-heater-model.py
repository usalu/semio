"""🔥 Round 3 electric fan heater: every solid of every authored scale and list stated by hand; writes `volumes/appliances/electric-fan-heater.json` (the decisions come from `r3-electric-fan-heater-texts.json`)."""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-electric-fan-heater-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

ABS, PC, STEEL, PP, RUBBER, ZINC, WIRE = "abs-cream", "pc-heat", "steel-mesh", "pp-dial", "rubber-foot", "steel-zinc", "nichrome-coil"
T_CASE, T_COVER, T_GRILLE, T_SLAT, T_INTAKE, T_DARK, T_RUBBER, T_METAL = 0.78, 0.7, 0.2, 0.32, 0.3, 0.2, 0.15, 0.5
W, YB, YT, YS, R = K.HALF_W, K.Y_BOTTOM, K.Y_TOP, K.Y_SEAM, K.EDGE_R
CENTRE = ((YB + YT) / 2, 0.0)
FACE = dict(y0=YB, y1=YT, z0=-W, z1=W, r=R)


def ref(sid, of, **extra):
    s = {"id": sid, "of": of}
    s.update(extra)
    return s


def th(s, region, layer=None, **extra):
    s["region"] = region
    if layer:
        s["layer"] = layer
    s.update(extra)
    return s


def st(s, part, member="body", **extra):
    s.update({"member": member, "part": part})
    s.update(extra)
    return s


def opening(box3, r, steps=3, scale=1.0):
    y0, y1, half = box3
    return K.rounded_rect(y0, y1, -half, half, r, steps, scale, CENTRE)


def pads(sid, x, top_r=0.015, low_r=0.0125, h=K.PAD_H, z=K.FOOT_Z):
    profile = [(0, 0), (0, low_r), (0.0025, low_r + 0.002), (h, top_r), (h, 0)]
    return K.revolve(sid, RUBBER, "body", T_RUBBER, profile, (0, 1, 0), (x, 0, -z), repeat={"count": 2, "translation": [0, 0, K.r5(2 * z)]})


def bosses(sid, x, z=K.FOOT_Z):
    profile = [(K.PAD_H, 0), (K.PAD_H, 0.017), (YB + K.WALL, 0.017), (YB + K.WALL, 0)]
    return K.revolve(sid, ABS, "body", T_CASE, profile, (0, 1, 0), (x, 0, -z), repeat={"count": 2, "translation": [0, 0, K.r5(2 * z)]})


def body50():
    out = [pads("pad-back", -K.FOOT_X), pads("pad-front", K.FOOT_X), bosses("boss-back", -K.FOOT_X), bosses("boss-front", K.FOOT_X)]
    out.append(K.extrude("shell", ABS, "body", T_CASE, K.u_profile(YB, YS, W, R, K.WALL, 6), "yz", K.TUBE_X0, K.TUBE_X1))
    out.append(K.extrude("top-cover", ABS, "body", T_COVER, K.u_profile(YS, YT, W, R, K.WALL, 6, cap=True), "yz", K.TUBE_X0, K.TUBE_X1))
    face = K.rounded_rect(steps=6, **FACE)
    out.append(K.extrude("front-frame", ABS, "body", T_CASE, face, "yz", K.TUBE_X1, K.FRONT_LIP_X1, holes=[opening(K.OUTLET, K.OUTLET_R, scale=1 / (K.FRONT_LIP_SCALE + 0.003))], taper=[1, K.FRONT_LIP_SCALE]))
    out.append(K.extrude("rear-frame", ABS, "body", T_CASE, face, "yz", K.REAR_LIP_X0, K.TUBE_X0, holes=[opening(K.INTAKE_HOLE, 0.006)], taper=[K.REAR_LIP_SCALE, 1]))
    out.append(K.lateral_loft("handle", ABS, "body", 0.75, K.handle_path()))
    bar = K.CASSETTE_BAR
    y0, y1, half = K.OUTLET
    out.append(K.extrude("grille-frame", STEEL, "body", T_GRILLE, opening(K.OUTLET, K.OUTLET_R), "yz", K.CASSETTE_X0, K.CASSETTE_X1, holes=[opening((y0 + bar, y1 - bar, half - bar), 0.003, 2)]))
    mid = ((K.SLAT_IN[0] + K.SLAT_OUT[0]) / 2, K.SLAT_Y0, 0.0)
    out.append(K.box("louvres", STEEL, "detail", T_SLAT, (K.slat_chord(), K.SLAT_T, 2 * (half - bar) + 0.002), mid, rotation=K.quat_axis(2, -K.slat_angle()), repeat={"count": K.SLAT_COUNT, "translation": [0, K.SLAT_PITCH, 0]}, texture=True))
    py0, py1, phalf = K.INTAKE_PANEL
    out.append(K.extrude("intake-panel", PC, "body", T_INTAKE, K.rounded_rect(py0, py1, -phalf, phalf, K.INTAKE_PANEL_R, 4), "yz", K.REAR_X, K.REAR_LIP_X0, holes=K.intake_slots(), taper=[0.975, 1]))
    out.append(K.revolve("cord-grommet", PP, "detail", T_DARK, [(0, 0), (0, 0.005), (0.002, 0.007), (0.009, 0.007), (0.009, 0)], (1, 0, 0), (K.REAR_X, 0.040, 0.070)))
    out.append(K.revolve("dial", PP, "body", T_DARK, [(0.0045, 0), (0.0045, 0.0195), (0.006, 0.021), (0.012, K.KNOB_R), (0.012, 0.005), (0.0145, 0.005), (0.0145, 0)], (0, 0, 1), (0, 0, -K.KNOB_OUT), bone="knob"))
    out.append(K.box("dial-grip", PP, "body", T_DARK, (0.008, 0.040, 0.0075), (0, 0, -K.KNOB_OUT + 0.00375), corner=0.002, bone="knob"))
    out.append(K.box("mode-switch", PP, "body", T_DARK, (0.032, 0.016, 0.0085), (0, K.SWITCH_Y, -W + 0.00075), corner=0.002))
    out += internals50()
    return out


def internals50():
    out = []
    out.append(K.box("motor-bridge", ZINC, "detail", T_METAL, (0.008, 0.012, 2 * (W - K.WALL)), (-0.076, K.AXIS_Y, 0)))
    out.append(K.box("motor-stack", ZINC, "detail", T_METAL, (0.020, 0.046, 0.054), (-0.062, K.AXIS_Y, 0), corner=0.002))
    out.append(K.box("motor-coil", PP, "detail", T_DARK, (0.026, 0.024, 0.030), (-0.062, K.AXIS_Y - 0.035, 0), corner=0.004))
    x0, x1 = K.FAN_X
    out.append(K.revolve("fan-hub", PC, "detail", 0.35, [(0, 0), (0, 0.016), (0.018, 0.018), (0.024, 0.012), (0.024, 0)], (1, 0, 0), (x0, K.AXIS_Y, 0)))
    xm = (x0 + x1) / 2
    out.append(K.extrude("fan-blades", PC, "detail", 0.4, K.BLADE, "yz", -0.00075, 0.00075, transform={"rotation": K.quat_axis(1, 28), "translation": K.v3((xm, K.AXIS_Y, 0))}, repeat={"count": 5, "rotationAxis": [1, 0, 0], "rotationRad": round(2 * math.pi / 5, 6), "origin": K.v3((xm, K.AXIS_Y, 0))}, texture=True))
    ring = [(K.AXIS_Y + (K.FAN_R + 0.003) * math.cos(math.radians(a)), (K.FAN_R + 0.003) * math.sin(math.radians(a))) for a in range(0, 360, 10)]
    out.append(K.extrude("fan-shroud", PC, "detail", 0.35, K.rounded_rect(YB + K.WALL, YT - K.WALL, -(W - K.WALL), W - K.WALL, R - K.WALL, 4), "yz", -0.046, -0.034, holes=[ring]))
    hy0, hy1, hhalf = K.HEATER_BOX
    hx0, hx1 = K.HEATER_X
    out.append(K.extrude("heater-box", ZINC, "detail", T_METAL, K.rounded_rect(hy0, hy1, -hhalf, hhalf, 0.004, 2), "yz", hx0, hx1, holes=[K.rounded_rect(hy0 + 0.001, hy1 - 0.001, -hhalf + 0.001, hhalf - 0.001, 0.003, 2)]))
    for sid, x, y in (("coil-stage-1", 0.022, 0.105), ("coil-stage-2", 0.040, 0.105)):
        out.append(K.solid(sid, "capsule", "body", WIRE, "detail", 0.45, p0=K.v3((x, y, -hhalf + 0.0045)), p1=K.v3((x, y, hhalf - 0.0045)), r0=0.0035, r1=0.0035, repeat={"count": 3, "translation": [0, 0.04, 0]}, slender=True))
    span = W - K.WALL - hhalf
    out.append(K.box("heater-brackets", ZINC, "detail", T_METAL, (0.012, 0.008, span), ((hx0 + hx1) / 2, K.AXIS_Y, -(hhalf + span / 2)), repeat={"count": 2, "translation": [0, 0, K.r5(2 * hhalf + span)]}))
    return out


def shell_halves(steps, x0, x1):
    out = []
    for name, side, faces in (("right", -1, ["+w", "+v"]), ("left", 1, ["-w", "+v"])):
        out.append(th(K.extrude("th-shell-" + name, ABS, "body", T_CASE, K.u_half(YB, YS, W, R, K.WALL, steps, side), "yz", x0, x1, sectionFaces=faces), "casing", "wall"))
    return out


def cover_halves(steps, x0, x1):
    out = []
    for name, side, faces in (("right", -1, ["+w", "-v"]), ("left", 1, ["-w", "-v"])):
        out.append(th(K.extrude("th-cover-" + name, ABS, "body", T_COVER, K.u_half(YS, YT, W, R, K.WALL, steps, side, cap=True), "yz", x0, x1, sectionFaces=faces), "top-cover", "cover"))
    return out


def outlet_patch(thickness):
    """💨 The outlet patch of both warm-air emitters: a disc of the emitter diameter on the louvre tips."""
    return th(K.revolve("th-warm-air", STEEL, "detail", 0.6, [(0, 0), (0, 0.05), (thickness, 0.05), (thickness, 0)], (1, 0, 0), (K.FRONT_X - thickness, K.AXIS_Y, 0), tags=["warm-air-1", "warm-air-2"]), "grille", "bars")


def thermal50():
    out = shell_halves(6, K.TUBE_X0, K.TUBE_X1)
    out += [th(ref("th-front-frame", "front-frame", sectionFaces=["-u"]), "casing"), th(ref("th-rear-frame", "rear-frame", sectionFaces=["+u"]), "casing"), th(ref("th-handle", "handle"), "casing"), th(ref("th-boss-back", "boss-back"), "casing"), th(ref("th-boss-front", "boss-front"), "casing")]
    out += cover_halves(6, K.TUBE_X0, K.TUBE_X1)
    out.append(th(ref("th-grille-frame", "grille-frame", sectionFaces=["-u"]), "grille", "bars"))
    out.append(th(ref("th-louvres", "louvres", sectionFaces=["-v"]), "grille", "bars"))
    out.append(outlet_patch(0.0015))
    out.append(th(ref("th-intake-panel", "intake-panel", sectionFaces=["+u"]), "intake", "inlet"))
    out += [th(ref("th-dial", "dial"), "control", "dial"), th(ref("th-dial-grip", "dial-grip"), "control", "dial"), th(ref("th-mode-switch", "mode-switch"), "control", "dial")]
    out += [th(ref("th-pad-back", "pad-back"), "feet", "foot"), th(ref("th-pad-front", "pad-front"), "feet", "foot")]
    return out


def top_band(steps, half=0.03):
    return K.extrude("member-top-band", ABS, "body", T_COVER, K.u_profile(YS, YT, W, R, K.WALL, steps, cap=True), "yz", -half, half)


def structure50():
    out = [st(ref("member-shell", "shell"), "member"), st(top_band(6), "member"), st(ref("member-front-ring", "front-frame"), "member"), st(ref("member-rear-ring", "rear-frame"), "member"), st(ref("member-handle", "handle"), "member"), st(ref("member-motor-bridge", "motor-bridge"), "member"), st(ref("member-heater-brackets", "heater-brackets"), "member"), st(ref("member-boss-back", "boss-back"), "member"), st(ref("member-boss-front", "boss-front"), "member")]
    out.append(st(K.revolve("member-knob", ZINC, "detail", 0.25, [(0, 0), (0, 0.004), (0.03, 0.004), (0.03, 0)], (0, 0, 1), (0, 0, -W - 0.0035), bone="knob"), "member", "knob"))
    out.append(st(K.revolve("joint-knob", ZINC, "detail", 0.25, [(0, 0), (0, 0.0075), (0.0075, 0.0075), (0.0075, 0)], (0, 0, 1), (0, 0, -W - 0.0025), bone="knob"), "joint", "knob"))
    out.append(st(K.revolve("joint-handle", ABS, "detail", 0.25, [(0, 0), (0, 0.014), (0.008, 0.014), (0.008, 0)], (0, 1, 0), (0, YT - 0.004, -K.HANDLE_Z), repeat={"count": 2, "translation": [0, 0, 2 * K.HANDLE_Z]}), "joint"))
    out.append(st(K.revolve("joint-motor-bridge", ZINC, "detail", 0.25, [(0, 0), (0, 0.005), (0.008, 0.005), (0.008, 0)], (0, 0, 1), (-0.076, K.AXIS_Y, -(W - K.WALL)), repeat={"count": 2, "translation": [0, 0, K.r5(2 * (W - K.WALL) - 0.008)]}), "joint"))
    hy0, hy1, hhalf = K.HEATER_BOX
    out.append(st(K.revolve("joint-heater-brackets", ZINC, "detail", 0.25, [(0, 0), (0, 0.005), (0.008, 0.005), (0.008, 0)], (0, 0, 1), (sum(K.HEATER_X) / 2, K.AXIS_Y, -(W - K.WALL)), repeat={"count": 2, "translation": [0, 0, K.r5(2 * (W - K.WALL) - 0.008)]}), "joint"))
    out.append(st(K.box("mass-motor", ZINC, "detail", 0.15, (0.020, 0.046, 0.054), (-0.062, K.AXIS_Y, 0), corner=0.002), "mass"))
    out.append(st(K.box("mass-heater", ZINC, "detail", 0.15, (0.03, 0.04, 0.05), (sum(K.HEATER_X) / 2, K.AXIS_Y, 0), corner=0.002), "mass"))
    out.append(st(ref("support-foot-back", "pad-back"), "support", anchor="foot-back"))
    out.append(st(ref("support-foot-front", "pad-front"), "support", anchor="foot-front"))
    return out


def feet100(sid, x):
    profile = [(0, 0), (0, 0.014), (0.004, 0.016), (YB + K.WALL, 0.016), (YB + K.WALL, 0)]
    return K.revolve(sid, RUBBER, "body", T_RUBBER, profile, (0, 1, 0), (x, 0, -K.FOOT_Z), repeat={"count": 2, "translation": [0, 0, K.r5(2 * K.FOOT_Z)]})


CAP100 = 0.015
SLATS100 = (4, 0.04, 0.085)
PLATE100 = 0.0925


def section_low(steps):
    """⬇️ The closed section of the housing below the cover seam as [y, z] points."""
    return [(YS, -W)] + K.arc(YB + R, -W + R, R, 180, 270, steps)[0:0] + [(YB + R - R * math.sin(math.radians(90 * k / steps)), -W + R - R * math.cos(math.radians(90 * k / steps))) for k in range(steps + 1)] + [(YB + R - R * math.sin(math.radians(90 * k / steps)), W - R + R * math.cos(math.radians(90 * k / steps))) for k in range(steps, -1, -1)] + [(YS, W)]


def section_high(steps):
    """⬆️ The closed section of the top cover above the seam as [y, z] points."""
    return [(YS, W)] + [(YT - R + R * math.sin(math.radians(90 * k / steps)), W - R + R * math.cos(math.radians(90 * k / steps))) for k in range(steps + 1)] + [(YT - R + R * math.sin(math.radians(90 * k / steps)), -W + R - R * math.cos(math.radians(90 * k / steps))) for k in range(steps, -1, -1)] + [(YS, -W)]


def body100():
    x0, x1 = K.REAR_LIP_X0 + CAP100, K.FRONT_LIP_X1 - CAP100
    out = [feet100("foot-back", -K.FOOT_X), feet100("foot-front", K.FOOT_X)]
    out.append(K.extrude("shell-block", ABS, "body", T_CASE, section_low(3), "yz", x0, x1))
    out.append(K.extrude("cover-block", ABS, "body", T_COVER, section_high(3), "yz", x0, x1))
    face = K.rounded_rect(steps=3, **FACE)
    out.append(K.extrude("front-cap", ABS, "body", T_CASE, face, "yz", x1, K.FRONT_LIP_X1, taper=[1, K.FRONT_LIP_SCALE]))
    out.append(K.extrude("rear-cap", ABS, "body", T_CASE, face, "yz", K.REAR_LIP_X0, x0, taper=[K.REAR_LIP_SCALE, 1]))
    out.append(K.lateral_loft("handle", ABS, "body", 0.75, K.handle_path(bar=0.0075, post=0.0085, bend_steps=2, foot=False, root_y=0.264), n=3.0))
    y0, y1, half = K.OUTLET
    out.append(K.box("grille-plate", STEEL, "body", T_GRILLE, (0.015, y1 - y0, 2 * half), (PLATE100 - 0.0075, (y0 + y1) / 2, 0), corner=0.004))
    count, pitch, first = SLATS100
    dx, dy = K.FRONT_X - PLATE100, 0.006
    out.append(K.box("louvres", STEEL, "detail", T_SLAT, (math.hypot(dx, dy), K.SLAT_T, 2 * half - 0.012), (PLATE100 + dx / 2, first, 0), rotation=K.quat_axis(2, -math.degrees(math.atan2(dy, dx))), repeat={"count": count, "translation": [0, pitch, 0]}, texture=True))
    py0, py1, phalf = K.INTAKE_PANEL
    out.append(K.box("intake-plate", PC, "body", T_INTAKE, (0.015, py1 - py0, 2 * phalf), (K.REAR_X + 0.0075, (py0 + py1) / 2, 0), corner=0.005))
    out.append(K.revolve("dial", PP, "body", T_DARK, [(0, 0), (0, 0.020), (0.002, K.KNOB_R), (0.015, K.KNOB_R), (0.015, 0)], (0, 0, 1), (0, 0, -K.KNOB_OUT), bone="knob"))
    return out


def thermal100():
    out = [th(ref("th-shell-block", "shell-block", sectionFaces=["+v"]), "casing"), th(ref("th-front-cap", "front-cap"), "casing"), th(ref("th-rear-cap", "rear-cap"), "casing"), th(ref("th-handle", "handle"), "casing")]
    out.append(th(ref("th-cover-block", "cover-block", sectionFaces=["-v", "+u", "-u"]), "top-cover"))
    out.append(th(ref("th-grille-plate", "grille-plate", sectionFaces=["-u"]), "grille"))
    out.append(th(ref("th-louvres", "louvres", sectionFaces=["-v"]), "grille"))
    out.append(outlet_patch(0.002))
    out.append(th(ref("th-intake-plate", "intake-plate", sectionFaces=["+u"]), "intake"))
    out.append(th(ref("th-dial", "dial"), "control"))
    out += [th(ref("th-foot-back", "foot-back", sectionFaces=["+v"]), "feet"), th(ref("th-foot-front", "foot-front", sectionFaces=["+v"]), "feet")]
    return out


def structure100():
    x0, x1 = K.REAR_LIP_X0 + CAP100, K.FRONT_LIP_X1 - CAP100
    out = [st(K.extrude("member-shell", ABS, "body", T_CASE, K.u_profile(YB, YS, W, R, K.WALL, 3), "yz", x0, x1), "member"), st(top_band(3), "member"), st(ref("member-handle", "handle"), "member")]
    out.append(st(K.box("member-motor-bridge", ZINC, "detail", T_METAL, (0.015, 0.015, 2 * (W - K.WALL)), (-0.068, K.AXIS_Y, 0)), "member"))
    out.append(st(K.revolve("member-knob", ZINC, "detail", 0.25, [(0, 0), (0, 0.0075), (0.03, 0.0075), (0.03, 0)], (0, 0, 1), (0, 0, -W), bone="knob"), "member", "knob"))
    out.append(st(K.revolve("joint-knob", ZINC, "detail", 0.25, [(0, 0), (0, 0.012), (0.015, 0.012), (0.015, 0)], (0, 0, 1), (0, 0, -W - 0.002), bone="knob"), "joint", "knob"))
    out.append(st(K.revolve("joint-handle", ABS, "detail", 0.25, [(0, 0), (0, 0.015), (0.015, 0.015), (0.015, 0)], (0, 1, 0), (0, YT - 0.010, -K.HANDLE_Z), repeat={"count": 2, "translation": [0, 0, 2 * K.HANDLE_Z]}), "joint"))
    out.append(st(K.box("mass-motor", ZINC, "detail", 0.15, (0.024, 0.046, 0.054), (-0.048, K.AXIS_Y, 0)), "mass"))
    out.append(st(ref("support-foot-back", "foot-back"), "support", anchor="foot-back"))
    out.append(st(ref("support-foot-front", "foot-front"), "support", anchor="foot-front"))
    return out


BASE200 = (0.073, 0.09)
BLOCK200 = ((K.FRONT_LIP_X1 - K.REAR_LIP_X0, YT - YB, 2 * W), ((K.FRONT_LIP_X1 + K.REAR_LIP_X0) / 2, (YT + YB) / 2, 0))


def body200():
    out = [K.box("base", RUBBER, "body", T_RUBBER, (2 * BASE200[0], 0.03, 2 * BASE200[1]), (0, 0.015, 0))]
    out.append(K.extrude("block", ABS, "body", T_CASE, K.rounded_rect(steps=3, **FACE), "yz", K.REAR_LIP_X0, K.FRONT_LIP_X1))
    y0, y1, half = K.OUTLET
    out.append(K.box("grille", STEEL, "body", T_GRILLE, (0.03, y1 - y0, 2 * half), (K.FRONT_X - 0.015, (y0 + y1) / 2, 0)))
    out.append(K.extrude("handle-block", ABS, "body", T_CASE, [(YT - 0.01, -0.066), (YT - 0.01, 0.066), (0.298, 0.066), (0.3065, 0.0625), (0.31, 0.054), (0.31, -0.054), (0.3065, -0.0625), (0.298, -0.066)], "yz", -0.015, 0.015))
    return out


def thermal200():
    out = [th(K.extrude("th-casing", ABS, "body", T_CASE, section_low(3), "yz", K.REAR_LIP_X0, K.FRONT_LIP_X1, sectionFaces=["+v"]), "casing"), th(ref("th-handle", "handle-block"), "casing")]
    out.append(th(K.extrude("th-top-cover", ABS, "body", T_COVER, section_high(3), "yz", K.REAR_LIP_X0, K.FRONT_LIP_X1, sectionFaces=["-v"]), "top-cover"))
    y0, y1, half = K.OUTLET
    out.append(th(K.box("th-grille", STEEL, "body", T_GRILLE, (K.FRONT_X - K.FRONT_LIP_X1, y1 - y0, 2 * half), ((K.FRONT_X + K.FRONT_LIP_X1) / 2, (y0 + y1) / 2, 0), sectionFaces=["-u"]), "grille", "bars"))
    out.append(outlet_patch(0.003))
    py0, py1, phalf = K.INTAKE_PANEL
    out.append(th(K.box("th-intake", PC, "body", T_INTAKE, (K.REAR_LIP_X0 - K.REAR_X, py1 - py0, 2 * phalf), ((K.REAR_X + K.REAR_LIP_X0) / 2, (py0 + py1) / 2, 0), sectionFaces=["+u"]), "intake", "inlet"))
    out.append(th(K.revolve("th-control", PP, "body", T_DARK, [(0, 0), (0, K.KNOB_R), (K.KNOB_OUT - W, K.KNOB_R), (K.KNOB_OUT - W, 0)], (0, 0, 1), (0, 0, -K.KNOB_OUT), bone="knob"), "control", "dial"))
    out.append(th(K.box("th-feet", RUBBER, "body", T_RUBBER, (2 * BASE200[0], YB, 2 * BASE200[1]), (0, YB / 2, 0), sectionFaces=["+v"]), "feet", "foot"))
    return out


def structure200():
    out = [st(K.extrude("member-shell", ABS, "body", T_CASE, K.u_profile(YB, YS, W, R, K.WALL, 2), "yz", K.REAR_LIP_X0, K.FRONT_LIP_X1), "member"), st(top_band(2, 0.035), "member"), st(ref("member-handle", "handle-block"), "member")]
    out.append(st(K.revolve("member-knob", ZINC, "detail", 0.25, [(0, 0), (0, 0.015), (0.03, 0.015), (0.03, 0)], (0, 0, 1), (0, 0, -W), bone="knob"), "member", "knob"))
    out.append(st(K.box("mass-motor", ZINC, "detail", 0.15, (0.03, 0.046, 0.054), (-0.048, K.AXIS_Y, 0)), "mass"))
    for name, x in (("back", -K.FOOT_X), ("front", K.FOOT_X)):
        out.append(st(K.box("support-foot-" + name, RUBBER, "detail", 0.25, (0.03, 0.03, 2 * BASE200[1]), (x, 0.015, 0)), "support", anchor="foot-" + name))
    return out


def main():
    path = os.path.join(HERE, "volumes", "appliances", "electric-fan-heater.json")
    doc = json.load(open(path))
    texts = {}
    tp = os.path.join(HERE, "r3-electric-fan-heater-texts.json")
    if os.path.exists(tp):
        texts = json.load(open(tp)).get("volumes", {})
    vol = doc["volumes"]
    built = {"50": (body50, thermal50, structure50), "100": (body100, thermal100, structure100), "200": (body200, thermal200, structure200)}
    for key, (b, t, s) in built.items():
        vol[key] = {"status": "authored", "denominator": int(key), "body": b(), "thermal": t(), "structure": s(), "decisions": texts.get(key, vol[key]["decisions"])}
    with open(path, "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for key in ("50", "100", "200"):
        print(key, {k: len(vol[key][k]) for k in ("body", "thermal", "structure")})


main()
