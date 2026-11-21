"""🛴 Round 3, 1:50 of vehicles/electric-kick-scooter: every assembly of the real folding e-scooter stated by hand (hollow deck extrusion with nose and tail castings, rubber mat, gooseneck, head tube with headset, rear stays, channel mudguards with tail light and latch catch, side stand, fork with steerer, crown and two blades, folding hinge, tapered stem with cockpit, handlebar with grips, brake lever, throttle and bell, pneumatic tyres, hub motor, cast rear wheel with brake rotor and caliper), its thermal entries, its load path and its three drawings."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-electric-kick-scooter-parts")
kit = pt.kit
AF, AR, P = pt.AF, pt.AR, pt.P
RG = pt.REGION
sp, tp = kit.steer_pt, kit.stem_pt
AV = pt.AXIS_V
PENS = {"body": 0.25, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5}
ALU, RUB, TAPE, PLA, STEEL, MOT = "anodised", "rubber", "griptape", "plastic", "steel-disc", "motor-alloy"


def deck(S):
    x0, x1 = pt.DECK_X
    shell = S.solid("deck-shell", "extrude", "deck", ALU, 0.6, profile=pt.DECK_SECTION, holes=[kit.inset(pt.DECK_SECTION, 0.003)], plane="yz", **{"from": x0, "to": x1})
    S.body.append(shell)
    S.therm(S.solid("deck-shell", "extrude", "deck", ALU, 0.6, profile=pt.DECK_SECTION, plane="yz", **{"from": x0, "to": x1}), RG["frame"], own=True)
    S.struct_of("deck-shell", "deck")
    for i, plan in (("deck-nose", pt.NOSE_PLAN), ("deck-tail", pt.TAIL_PLAN)):
        S.put(S.solid(i, "extrude", "deck", ALU, 0.6, profile=plan, plane="zx", **{"from": pt.DECK_Y[0], "to": pt.DECK_Y[1]}), RG["frame"])
        S.struct_of(i, "deck")
    S.flat("elevation", "deck", "deck", "body", 0.3, RG["frame"], ALU, [[-0.316, 0.095], [0.262, 0.095], [0.268, 0.103], [0.268, 0.15], [-0.316, 0.15]], [0.268, 0.15, -0.075], tol=0)
    S.flat("front", "deck", "deck", "body", 0.3, RG["frame"], ALU, [[z, y] for y, z in pt.DECK_SECTION], [0.268, 0.15, -0.075], tol=0)
    plan = [[x, z] for z, x in pt.NOSE_PLAN[1:-1]] + [[x, z] for z, x in pt.TAIL_PLAN[2:]]
    S.flat("plan", "deck", "deck", "body", 0.3, RG["frame"], ALU, plan, [0.268, 0.15, -0.075], tol=0)
    S.put(S.solid("deck-mat", "extrude", "deck", TAPE, 0.15, profile=pt.rounded(pt.MAT_X[0], pt.MAT_X[1], pt.MAT_HALF, 0.02), plane="zx", **{"from": pt.MAT_Y[0], "to": pt.MAT_Y[1]}), RG["mat"])
    S.flat("elevation", "deck-mat", "deck", "body", 0.12, RG["mat"], TAPE, [[pt.MAT_X[0], 0.146], [pt.MAT_X[1], 0.146], [pt.MAT_X[1], pt.MAT_Y[1]], [pt.MAT_X[0], pt.MAT_Y[1]]], [pt.MAT_X[1], 0.158, -0.066], tol=0)
    S.flat("front", "deck-mat", "deck", "body", 0.12, RG["mat"], TAPE, [[-0.066, 0.146], [0.066, 0.146], [0.066, 0.158], [-0.066, 0.158]], [pt.MAT_X[1], 0.158, -0.066], tol=0)
    S.flat("plan", "deck-mat", "deck", "body", 0.12, RG["mat"], TAPE, [[x, z] for z, x in pt.rounded(pt.MAT_X[0], pt.MAT_X[1], pt.MAT_HALF, 0.02)], [pt.MAT_X[1], 0.158, -0.066], tol=0.0008)
    for side, sign in (("l", 1), ("r", -1)):
        z0, z1 = sorted((sign * pt.STAY_Z[0], sign * pt.STAY_Z[1]))
        S.put(S.prism("stay-" + side, "deck", pt.stay_outline(), z0, z1, ALU, 0.6), RG["frame"])
        S.struct_of("stay-" + side, "deck")
        S.swept("stay-" + side, "deck", pt.stay_outline(), z0, z1, "body", 0.3, RG["frame"], ALU)
    neck = S.tube("neck", "deck", [([p[0][0], p[0][1], 0.0], p[1], p[2], 4.0) for p in pt.NECK], ALU, 0.6)
    S.put(neck, RG["frame"])
    S.struct_of("neck", "deck")
    S.flat("elevation", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_side(), [0.42, 0.33, -0.06], tol=0.0005)
    S.flat("plan", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_plan(), [0.42, 0.33, -0.06], tol=0.0005)
    S.flat("front", "neck", "deck", "body", 0.3, RG["frame"], ALU, pt.neck_front(), [0.3, 0.33, -0.06], tol=0.0005)
    lo, hi = pt.axis_pt(pt.HEAD_Y[0]), pt.axis_pt(pt.HEAD_Y[1])
    L = kit.norm(kit.sub(hi, lo))
    up = kit.unit(kit.sub(hi, lo))
    S.body.append(S.rev("head-tube", "deck", [[0, 0.019], [L, 0.019], [L, pt.HEAD_R], [0, pt.HEAD_R], [0, 0.019]], lo, up, ALU, 0.6))
    S.therm(S.cyl("head-tube", "deck", lo, hi, pt.HEAD_R, ALU, 0.6), RG["frame"], own=True)
    S.struct_of("head-tube", "deck")
    S.swept("head-tube", "deck", pt.quad(lo, hi, pt.HEAD_R), -pt.HEAD_R, pt.HEAD_R, "body", 0.3, RG["frame"], ALU, tol=0)
    for i, a, b in (("cup-lower", kit.add(lo, up, -pt.CUP), lo), ("cup-upper", hi, kit.add(hi, up, pt.CUP))):
        S.put(S.rev(i, "deck", [[0, 0.015], [pt.CUP, 0.015], [pt.CUP, pt.CUP_R], [0, pt.CUP_R], [0, 0.015]], a, up, STEEL, 0.35, role="detail"), RG["frame"])
        S.structure.append({"id": i, "of": i, "member": "fork", "part": "joint"})
        S.swept(i, "deck", pt.quad(a, b, pt.CUP_R), -pt.CUP_R, pt.CUP_R, "detail", 0.2, RG["frame"], STEEL, tol=0)
    a, b = [0.105, 0.1, 0.082], [-0.03, 0.094, 0.086]
    S.put(S.cap("side-stand", "deck", a, b, 0.006, STEEL, 0.3, role="detail"), RG["frame"])
    S.plank("side-stand", "deck", a, b, 0.012, 0.084, "detail", 0.2, RG["frame"], STEEL)
    S.therm(S.box("bms", "deck", [0.09, 0.02, 0.1], [0.17, 0.123, 0.0], PLA, 0.2, role="detail", corner=0.004), RG["bms"], own=True)
    S.struct(S.box("mass-battery", "deck", [0.36, 0.036, 0.11], [-0.04, 0.122, 0.0], STEEL, 0.2, role="detail", corner=0.008), "deck", "mass")


def rear(S):
    a0, a1 = pt.GUARD_REAR
    S.put(pt.channel(S, "fender-r", "deck", AR, pt.CHANNEL, a0, a1, PLA, 0.3), RG["guard"])
    S.swept("fender-r", "deck", pt.band(AR, pt.GUARD_R[0], pt.GUARD_R[1], a0, a1, 24), -pt.GUARD_HALF, pt.GUARD_HALF, "body", 0.5, RG["guard"], PLA, tol=0.0004)
    lamp = [[-0.0265, 0.105], [-0.0265, 0.1205], [0.0265, 0.1205], [0.0265, 0.105], [-0.0265, 0.105]]
    S.put(pt.channel(S, "tail-light", "deck", AR, lamp, 162, 178, PLA, 0.45, role="detail"), RG["guard"])
    S.swept("tail-light", "deck", pt.band(AR, 0.105, 0.1205, 162, 178, 3), -0.0265, 0.0265, "detail", 0.75, RG["guard"], PLA, tol=0)
    catch = [pt.polar(AR, 0.119, 43.5), pt.polar(AR, 0.136, 43.5), pt.polar(AR, 0.136, 38.5), pt.polar(AR, 0.119, 37.5)]
    S.put(S.prism("latch-catch", "deck", catch, 0.014, 0.03, PLA, 0.3, role="detail"), RG["guard"])
    S.swept("latch-catch", "deck", catch, 0.014, 0.03, "detail", 0.4, RG["guard"], PLA, tol=0)
    S.put(S.rev("tyre-r", "wheel-r", pt.TYRE, AR + (0.0,), [0, 0, 1], RUB, 0.07), RG["tyre"])
    pt.ring2d(S, "tyre-r", "wheel-r", AR, 0.108, 0.07, -0.025, 0.025, "body", 0.1, RG["tyre"], RUB)
    S.put(S.rev("rim-r", "wheel-r", pt.RIM, AR + (0.0,), [0, 0, 1], ALU, 0.6), RG["rim"])
    S.struct_of("rim-r", "wheel-r")
    pt.ring2d(S, "rim-r", "wheel-r", AR, 0.077, 0.062, -0.0225, 0.0225, "body", 0.75, RG["rim"], ALU)
    windows = [pt.window(k) for k in range(5)]
    S.put(S.prism("web-r", "wheel-r", pt.arc(AR, 0.064, 0, 360, 48)[:-1], -0.005, 0.005, ALU, 0.6, holes=windows), RG["rim"])
    S.struct_of("web-r", "wheel-r")
    pt.ring2d(S, "web-r", "wheel-r", AR, 0.064, None, -0.005, 0.005, "body", 0.7, RG["rim"], ALU, windows=windows)
    S.put(S.cyl("hub-r", "wheel-r", AR + (-0.04,), AR + (0.04,), 0.02, ALU, 0.55), RG["rim"])
    S.struct_of("hub-r", "wheel-r")
    S.disc("hub-r", "wheel-r", AR, 0.04, -0.04, 0.04, "body", 0.45, RG["rim"], ALU)
    S.put(S.cyl("axle-r", "wheel-r", AR + (-0.0575,), AR + (0.0575,), 0.007, STEEL, 0.3, role="detail"), RG["rim"])
    S.structure.append({"id": "axle-r", "of": "axle-r", "member": "wheel-r", "part": "joint"})
    S.disc("axle-r", "wheel-r", AR, 0.014, -0.0575, 0.0575, "detail", 0.15, RG["rim"], STEEL)
    z0, z1 = pt.ROTOR_Z
    S.put(S.rev("disc-r", "wheel-r", [[0, pt.ROTOR[0]], [z1 - z0, pt.ROTOR[0]], [z1 - z0, pt.ROTOR[1]], [0, pt.ROTOR[1]], [0, pt.ROTOR[0]]], AR + (z0,), [0, 0, 1], STEEL, 0.7, role="texture"), RG["rotor"])
    pt.ring2d(S, "disc-r", "wheel-r", AR, pt.ROTOR[1], pt.ROTOR[0], z0, z1, "detail", 0.6, RG["rotor"], STEEL)
    c = pt.polar(AR, 0.046, 58)
    S.put(S.box("caliper", "deck", [0.04, 0.03, 0.03], c + [0.04], STEEL, 0.25, role="detail", corner=0.006, rotation=kit.rot_w(-32)), RG["frame"])
    d = [math.cos(math.radians(-32)) * 0.02, math.sin(math.radians(-32)) * 0.02]
    S.swept("caliper", "deck", pt.quad([c[0] - d[0], c[1] - d[1]], [c[0] + d[0], c[1] + d[1]], 0.015), 0.025, 0.055, "detail", 0.2, RG["frame"], STEEL, tol=0)


def fork(S):
    S.put(S.cyl("hinge-base", "fork", sp(0.0), sp(0.0273), 0.027, ALU, 0.5), RG["steer"])
    S.swept("hinge-base", "fork", pt.quad(sp(0.0), sp(0.0273), 0.027), -0.027, 0.027, "body", 0.3, RG["steer"], ALU, tol=0)
    S.put(S.cyl("hinge-pin", "fork", [P[0], P[1], -0.033], [P[0], P[1], 0.033], 0.011, STEEL, 0.3, role="detail"), RG["steer"])
    S.structure.append({"id": "hinge-pin", "of": "hinge-pin", "member": "stem", "part": "joint"})
    S.disc("hinge-pin", "fork", P, 0.022, -0.033, 0.033, "detail", 0.15, RG["steer"], STEEL)
    S.body.append(S.pipe("steerer", "fork", sp(0.02), sp(0.134), 0.0145, 0.003, STEEL, 0.4))
    S.therm(S.cyl("steerer", "fork", sp(0.02), sp(0.134), 0.0145, STEEL, 0.4), RG["steer"], own=True)
    S.struct_of("steerer", "fork")
    S.put(S.box("crown", "fork", [0.028, 0.044, 0.102], sp(0.1415, 0.0015), ALU, 0.55, corner=0.007), RG["steer"])
    S.struct_of("crown", "fork")
    S.swept("crown", "fork", pt.quad(sp(0.1275, 0.0015), sp(0.1555, 0.0015), 0.022), -0.051, 0.051, "body", 0.35, RG["steer"], ALU, tol=0)
    for side, sign in (("l", 1), ("r", -1)):
        z0, z1 = sorted((sign * pt.BLADE_Z[0], sign * pt.BLADE_Z[1]))
        S.put(S.prism("blade-" + side, "fork", pt.blade_outline(), z0, z1, ALU, 0.6), RG["steer"])
        S.struct_of("blade-" + side, "fork")
        S.swept("blade-" + side, "fork", pt.blade_outline(), z0, z1, "body", 0.4, RG["steer"], ALU, tol=0.0004)
    a0, a1 = pt.GUARD_FRONT
    S.put(pt.channel(S, "fender-f", "fork", AF, pt.CHANNEL, a0, a1, PLA, 0.3), RG["guard"])
    S.swept("fender-f", "fork", pt.band(AF, pt.GUARD_R[0], pt.GUARD_R[1], a0, a1, 16), -pt.GUARD_HALF, pt.GUARD_HALF, "body", 0.5, RG["guard"], PLA, tol=0.0004)
    S.put(S.rev("tyre-f", "wheel-f", pt.TYRE, AF + (0.0,), [0, 0, 1], RUB, 0.07), RG["tyre"])
    pt.ring2d(S, "tyre-f", "wheel-f", AF, 0.108, 0.07, -0.025, 0.025, "body", 0.1, RG["tyre"], RUB)
    S.put(S.rev("motor-f", "wheel-f", pt.MOTOR, AF + (0.0,), [0, 0, 1], MOT, 0.5), RG["motor"])
    S.struct_of("motor-f", "wheel-f")
    S.disc("motor-f", "wheel-f", AF, 0.154, -0.0225, 0.0225, "body", 0.75, RG["motor"], MOT)
    S.disc("motor-cover", "wheel-f", AF, 0.116, -0.033, 0.033, "detail", 0.55, RG["motor"], MOT)
    S.put(S.cyl("axle-f", "wheel-f", AF + (-0.052,), AF + (0.052,), 0.007, STEEL, 0.3, role="detail"), RG["motor"])
    S.structure.append({"id": "axle-f", "of": "axle-f", "member": "wheel-f", "part": "joint"})
    S.disc("axle-f", "wheel-f", AF, 0.014, -0.052, 0.052, "detail", 0.15, RG["motor"], STEEL)
    S.struct(S.cyl("mass-motor", "wheel-f", AF + (-0.02,), AF + (0.02,), 0.05, STEEL, 0.2, role="detail"), "wheel-f", "mass")


def stem(S):
    up = [-kit.DOWN[0], -kit.DOWN[1], 0.0]
    r0, r1 = pt.STEM_R
    S.put(S.rev("stem-foot", "stem", [[0.0, 0.027], [0.03, 0.027], [0.058, r0]], tp(0.0, AV), up, ALU, 0.6), RG["steer"])
    S.struct_of("stem-foot", "stem")
    S.body.append(S.rev("stem-tube", "stem", [[0, r0 - 0.0025], [0.64, r1 - 0.0025], [0.64, r1], [0, r0], [0, r0 - 0.0025]], tp(0.05, AV), up, ALU, 0.65))
    S.therm(S.rev("stem-tube", "stem", [[0, r0], [0.64, r1]], tp(0.05, AV), up, ALU, 0.65), RG["steer"], own=True)
    S.struct_of("stem-tube", "stem")
    S.put(S.rev("stem-head", "stem", [[0.668, 0.0168], [0.69, 0.026], [0.728, 0.026], [0.737, 0.019], [0.737, 0.0]], tp(0.0, AV), up, ALU, 0.6), RG["steer"])
    S.struct_of("stem-head", "stem")
    for i, line, half in (("stem-tube", [[0.058, r0], [0.668, r1 - 0.0022]], 0.0205), ("stem-foot", [[0.0, 0.027], [0.03, 0.027], [0.058, r0]], 0.027), ("stem-head", [[0.668, r1 - 0.0022], [0.69, 0.026], [0.728, 0.026], [0.737, 0.019]], 0.026)):
        outline = [tp(u, AV + r)[:2] for u, r in line] + [tp(u, AV - r)[:2] for u, r in line[::-1]]
        S.swept(i, "stem", outline, -half, half, "body", 0.4, RG["steer"], ALU, tol=0)
    S.put(S.box("fold-lever", "stem", [0.07, 0.009, 0.02], tp(0.09, AV - 0.0262), PLA, 0.2, role="detail", corner=0.004), RG["steer"])
    S.swept("fold-lever", "stem", pt.quad(tp(0.055, AV - 0.0262), tp(0.125, AV - 0.0262), 0.0045), -0.01, 0.01, "detail", 0.2, RG["steer"], PLA, tol=0)
    S.put(S.box("display", "stem", [0.004, 0.03, 0.034], tp(0.7355, AV), PLA, 0.15, role="glazing"), RG["steer"])
    S.put(S.cyl("headlight", "stem", tp(0.706, AV - 0.022), tp(0.706, AV - 0.033), 0.012, PLA, 0.9, role="glazing"), RG["steer"])
    S.swept("headlight", "stem", pt.quad(tp(0.706, AV - 0.025), tp(0.706, AV - 0.033), 0.012), -0.012, 0.012, "glazing", 0.9, RG["steer"], PLA, tol=0)
    B = tp(pt.BAR_U, AV)
    S.put(S.cyl("bar-clamp", "stem", B[:2] + [-0.034], B[:2] + [0.034], 0.0165, ALU, 0.5), RG["steer"])
    S.disc("bar-clamp", "stem", B, 0.033, -0.034, 0.034, "detail", 0.3, RG["steer"], ALU)
    S.put(S.cyl("bar", "stem", B[:2] + [-pt.BAR_HALF], B[:2] + [pt.BAR_HALF], 0.011, ALU, 0.65), RG["steer"])
    S.struct_of("bar", "stem")
    S.disc("bar", "stem", B, 0.022, -pt.BAR_HALF, pt.BAR_HALF, "body", 0.4, RG["steer"], ALU)
    grip = [[0.0, 0.0175], [0.005, 0.0175], [0.007, 0.0155], [0.11, 0.0155], [0.112, 0.018], [0.12, 0.018]]
    for side, sign in (("l", 1), ("r", -1)):
        S.put(S.rev("grip-" + side, "stem", grip, B[:2] + [sign * pt.GRIP[0]], [0, 0, sign], RUB, 0.1), RG["steer"])
        z0, z1 = sorted((sign * pt.GRIP[0], sign * pt.GRIP[1]))
        S.disc("grip-" + side, "stem", B, 0.032, z0, z1, "body", 0.1, RG["steer"], RUB)
    S.put(S.box("brake-perch", "stem", [0.02, 0.03, 0.016], tp(0.714, AV - 0.013, 0.082), PLA, 0.2, role="detail", corner=0.004), RG["steer"])
    S.swept("brake-perch", "stem", pt.quad(tp(0.714, AV + 0.002), tp(0.714, AV - 0.028), 0.01), 0.074, 0.09, "detail", 0.2, RG["steer"], PLA, tol=0)
    lever = [(tp(0.711, AV - 0.027, 0.086), 0.0045, 0.0045), (tp(0.708, AV - 0.042, 0.106), 0.0045, 0.0045), (tp(0.706, AV - 0.046, 0.15), 0.0045, 0.0045), (tp(0.705, AV - 0.042, 0.192), 0.0045, 0.0045)]
    S.put(S.tube("brake-lever", "stem", lever, ALU, 0.5, hint=(0.0, 1.0, 0.0), role="detail"), RG["steer"])
    S.swept("brake-lever-root", "stem", pt.quad(tp(0.711, AV - 0.027), tp(0.708, AV - 0.044), 0.0045), 0.086, 0.106, "detail", 0.2, RG["steer"], ALU, tol=0)
    S.swept("brake-lever", "stem", pt.quad(tp(0.7065, AV - 0.0395), tp(0.7065, AV - 0.0485), 0.0045), 0.106, 0.192, "detail", 0.2, RG["steer"], ALU, tol=0)
    S.put(S.cyl("throttle", "stem", B[:2] + [-0.092], B[:2] + [-0.074], 0.0165, PLA, 0.2, role="detail"), RG["steer"])
    S.disc("throttle", "stem", B, 0.033, -0.092, -0.074, "detail", 0.2, RG["steer"], PLA)
    S.put(S.box("throttle-paddle", "stem", [0.01, 0.03, 0.014], tp(0.706, AV + 0.02, -0.083), PLA, 0.2, role="detail", corner=0.003), RG["steer"])
    S.swept("throttle-paddle", "stem", pt.quad(tp(0.706, AV + 0.005), tp(0.706, AV + 0.035), 0.005), -0.09, -0.076, "detail", 0.2, RG["steer"], PLA, tol=0)
    S.put(S.ball("bell", "stem", [0.018, 0.012, 0.018], tp(0.722, AV + 0.021, 0.052), STEEL, 0.75, role="detail"), RG["steer"])
    c = kit.loc("stem", tp(0.722, AV + 0.021))
    s = {"kind": "ellipse", "id": "bell", "bone": "stem", "role": "detail", "z": 0, "filled": True, "tone": 0.7, "region": RG["steer"], "material": STEEL, "cx": pt.r4(c[0]), "cy": pt.r4(c[1]), "rx": 0.018, "ry": 0.012, "solid": {"kind": "extruded", "sweep": "w", "z0": 0.034, "z1": 0.07}}
    S.shape(("elevation", "front", "plan"), s, tp(0.74, AV + 0.031, 0.034))
    S.struct(S.ball("mass-stem", "stem", [0.09, 0.014, 0.014], tp(0.36, AV), STEEL, 0.2, role="detail"), "stem", "mass")


def build():
    S = kit.Scale(50, PENS)
    deck(S)
    rear(S)
    fork(S)
    stem(S)
    return S


DECISIONS = [
    {"en": "Body: the real folding e-scooter, part by part. A hollow extruded aluminium deck (0.53 m long, 0.15 m wide, 55 mm deep, 3 mm wall, chamfered battery tray) closed by a rounded nose casting and a tapered tail casting, a rubber mat on top, two cast stays to the rear axle, a gooseneck of six rounded-rectangle sections up to the hollow head tube with both headset cups, a folded side stand; the fork is a steerer tube, a crown and two flat blades with rounded drop-outs around the hub motor, with the hinge base and hinge pin on top; channel-section mudguards over both wheels (the rear one with tail light and latch catch); pneumatic 8 1/2 x 2 tyres, the hub motor shell with side covers and rim flanges, a cast five-spoke rear wheel with hub, axle, brake rotor and caliper; a tapered hollow stem on its hinge foot with fold lever, cockpit head with display and headlight, handlebar with bar clamp, two flanged grips, brake lever with perch, throttle with paddle and the bell that meets the catch when folded.", "de": "Körper: der wirkliche Klapp-E-Roller, Teil für Teil. Ein hohles stranggepresstes Aluminium-Trittbrett (0,53 m lang, 0,15 m breit, 55 mm hoch, 3 mm Wand, gefaste Akkuwanne), geschlossen durch ein gerundetes Buggussteil und ein verjüngtes Heckgussteil, eine Gummimatte obenauf, zwei Gussstreben zur Hinterachse, ein Schwanenhals aus sechs Rundrechteck-Querschnitten hinauf zum hohlen Steuerrohr mit beiden Steuersatzschalen, ein hochgeklappter Seitenständer; die Gabel besteht aus Schaftrohr, Krone und zwei flachen Scheiden mit gerundeten Ausfallenden um den Nabenmotor, darauf Gelenksockel und Gelenkbolzen; Schutzbleche mit Rinnenquerschnitt über beiden Rädern (das hintere mit Rücklicht und Rasthaken); Luftreifen 8 1/2 x 2, das Nabenmotorgehäuse mit Seitendeckeln und Felgenhörnern, ein gegossenes Fünfspeichen-Hinterrad mit Nabe, Achse, Bremsscheibe und Bremssattel; eine konische hohle Lenkstange auf ihrem Gelenkfuß mit Klapphebel, Cockpitkopf mit Anzeige und Scheinwerfer, Lenker mit Klemme, zwei Griffen mit Bund, Bremshebel mit Schelle, Gasgriff mit Daumenhebel und die Klingel, die geklappt den Rasthaken trifft."},
    {"en": "Thermal: every region is the surface of its own parts by reference; the hollow deck extrusion, head tube, steerer and stem tube stand in the thermal list as closed bodies of the same outline, because their sealed cavities are no outer surface; the controller is a real hidden box inside the deck nose; frame castings, neck, head tube, stays, side stand and caliper belong to the deck region, the whole steering assembly to stem-bars, both mudguards with tail light and catch to fenders, the cast rear wheel to rear-rim, the rotor ring to brake-disc. All regions have one layer or none, so no nested shells are needed.", "de": "Thermik: Jede Region ist die Oberfläche ihrer eigenen Teile per Verweis; das hohle Trittbrettprofil, Steuerrohr, Gabelschaft und Lenkstangenrohr stehen in der Thermikliste als geschlossene Körper gleichen Umrisses, weil ihre abgeschlossenen Hohlräume keine Außenfläche sind; die Steuerung ist ein wirklicher verdeckter Kasten im Trittbrettbug; Rahmengussteile, Hals, Steuerrohr, Streben, Seitenständer und Bremssattel gehören zur Trittbrettregion, die ganze Lenkeinheit zu stem-bars, beide Schutzbleche mit Rücklicht und Haken zu fenders, das gegossene Hinterrad zu rear-rim, der Scheibenring zu brake-disc. Alle Regionen haben eine oder keine Schicht, daher sind keine verschachtelten Schalen nötig."},
    {"en": "Structure: the real load path. The deck box girder with nose, tail, stays, neck and head tube carries the rider's load to the rear axle and to the steering head; steerer, crown and both blades carry it to the front axle; stem tube, hinge foot, cockpit head and handlebar are the cantilever above the hinge; the motor shell and the cast rear wheel (rim, spoke web, hub) are the wheel members. Joints: both headset cups (steering), the hinge pin (folding) and the two axles; masses: the battery pack in the deck, the motor stator and the stem. The asset declares no structural support anchors, so there are no support solids.", "de": "Tragwerk: der wirkliche Lastpfad. Der Trittbrett-Kastenträger mit Bug, Heck, Streben, Hals und Steuerrohr trägt die Fahrerlast zur Hinterachse und zum Steuerkopf; Gabelschaft, Krone und beide Scheiden tragen sie zur Vorderachse; Lenkstangenrohr, Gelenkfuß, Cockpitkopf und Lenker sind der Kragarm über dem Gelenk; das Motorgehäuse und das gegossene Hinterrad (Felge, Speichenscheibe, Nabe) sind die Radstäbe. Gelenke: beide Steuersatzschalen (Lenken), der Gelenkbolzen (Klappen) und die zwei Achsen; Massen: der Akkupack im Trittbrett, der Motorstator und die Lenkstange. Das Asset deklariert keine Auflageranker, daher gibt es keine Auflagerkörper."},
]

DRAWN = {
    "silhouette": {"en": "All three views are the true orthographic views of the 1:50 body. Side: tyre rings, hub motor with cover and axle, cast rear wheel with five spokes, hub, brake rotor and caliper, deck with rubber mat, stays, gooseneck, head tube with headset cups, fork crown and blades, hinge base and pin, both channel mudguards with tail light and latch catch, side stand, stem with hinge foot and cockpit head, fold lever, headlight, handlebar end with grip, brake lever, throttle and bell. Front and top show the same parts from ahead and from above.", "de": "Alle drei Ansichten sind die wahren orthogonalen Ansichten des 1:50-Körpers. Seite: Reifenringe, Nabenmotor mit Deckel und Achse, gegossenes Hinterrad mit fünf Speichen, Nabe, Bremsscheibe und Bremssattel, Trittbrett mit Gummimatte, Streben, Schwanenhals, Steuerrohr mit Steuersatzschalen, Gabelkrone und -scheiden, Gelenksockel und Bolzen, beide Rinnen-Schutzbleche mit Rücklicht und Rasthaken, Seitenständer, Lenkstange mit Gelenkfuß und Cockpitkopf, Klapphebel, Scheinwerfer, Lenkerende mit Griff, Bremshebel, Gasgriff und Klingel. Vorder- und Draufsicht zeigen dieselben Teile von vorn und von oben."},
    "retained": {"en": "Every assembly of the body that is at least 7.5 mm thick.", "de": "Jede Baugruppe des Körpers ab 7,5 mm Dicke."},
    "merged": {"en": "Hinge foot, stem tube and cockpit head are one outline; deck extrusion, nose and tail castings are one outline; the controller sits inside the deck (hidden region).", "de": "Gelenkfuß, Lenkstangenrohr und Cockpitkopf sind ein Umriss; Trittbrettprofil, Bug- und Heckgussteil sind ein Umriss; die Steuerung sitzt im Trittbrett (verdeckte Region)."},
    "omitted": {"en": "Cables, bolts, reflectors, charging port, tyre tread, the display glass and the steerer inside the head tube.", "de": "Kabel, Schrauben, Reflektoren, Ladebuchse, Reifenprofil, das Anzeigeglas und der Gabelschaft im Steuerrohr."},
    "thinElements": {"en": "The 8.5 mm rubber mat is drawn 12 mm high (0.24 mm on paper); the 6 mm mudguard channels (0.12 mm), the 3 mm brake rotor and the 9 mm levers are filled shapes at their real size with a 0.18 or 0.25 mm outline.", "de": "Die 8,5 mm dicke Gummimatte ist 12 mm hoch gezeichnet (0,24 mm auf dem Papier); die 6 mm hohen Schutzblechrinnen (0,12 mm), die 3 mm dicke Bremsscheibe und die 9 mm dicken Hebel sind gefüllte Formen in wirklicher Größe mit 0,18 oder 0,25 mm Kontur."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Near black tyres, grips and mat, light motor shell, rim and bell, mid-tone aluminium frame, fork and stem, lighter plastic mudguards, dark levers and steel parts.", "de": "Fast schwarze Reifen, Griffe und Matte, helles Motorgehäuse, Felge und Klingel, mitteltoniger Aluminiumrahmen, Gabel und Lenkstange, hellere Kunststoff-Schutzbleche, dunkle Hebel und Stahlteile."},
    "overlays": {"en": "Each visible region has its own shapes; the controller is hidden. Every shape of the fork, stem and wheel bones is an extruded profile, so folding, steering and rolling carry it in all three views. Steering: the fork bone runs from the steering head down the steering axis (8 degrees rake) to the front axle and carries the front wheel and the stem, so the steered braking pose is a twist about that axis; the stem bone starts at the hinge pin behind the steering head and folds by 95 degrees.", "de": "Jede sichtbare Region hat eigene Formen; die Steuerung ist verdeckt. Jede Form der Gabel-, Lenkstangen- und Radknochen ist ein extrudiertes Profil, sodass Klappen, Lenken und Rollen sie in allen drei Ansichten mitnehmen. Lenkung: Der Gabelknochen läuft vom Steuerkopf die Lenkachse (8 Grad Neigung) hinab zur Vorderachse und trägt Vorderrad und Lenkstange, die gelenkte Bremspose ist eine Drehung um diese Achse; der Lenkstangenknochen beginnt am Gelenkbolzen hinter dem Steuerkopf und klappt um 95 Grad."},
}
