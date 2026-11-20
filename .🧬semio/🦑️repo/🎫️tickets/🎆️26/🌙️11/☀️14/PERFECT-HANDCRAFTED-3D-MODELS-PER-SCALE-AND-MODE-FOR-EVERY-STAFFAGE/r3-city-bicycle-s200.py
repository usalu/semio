"""🚲 Round 3 vehicles/city-bicycle at 1:200 (smallest feature 30 mm): the Dutch city bicycle reduced to what this scale can show, authored anew. Each wheel is one ring with a hub and six spokes, the frame is five tubes and four stays at their real width, the carrier a hoop on two legs, the saddle one deep pad, chain case, two crank bars with pedal blocks, sheet mudguards, a fork of two blades under a crown bar, one stem bar, the swept-back handlebar, two lamps and the folded stand."""
import importlib.util
import math
import os

T = os.path.dirname(os.path.abspath(__file__))


def load(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


pt = load("r3-city-bicycle-parts")
kit = pt.kit
R, B, F, P = pt.R3, pt.B3, pt.F3, kit.P
sp = kit.steer_pt
ALL, SIDE = pt.ALL, pt.SIDE
PAINT, CHROME, ALLOY, RUBBER, PLASTIC, SKIN = "paint", "chrome", "alloy", "rubber", "plastic", "saddle-skin"
TUBE, GUARD, CASE, STEEL, DARK = 0.3, 0.55, 0.6, 0.7, 0.15
PENS = {"body": 0.18, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.25}
PEN = 0.15


def wheel(S, bone, c):
    tag = bone[-1]
    ring = [(-0.0185, 0.335), (-0.0185, 0.305), (-0.0125, 0.297), (0.0125, 0.297), (0.0185, 0.305), (0.0185, 0.335), (0.01, 0.345), (-0.01, 0.345), (-0.0185, 0.335)]
    S.put(S.rev(f"wheel-{tag}", bone, ring, c, [0, 0, 1], RUBBER, DARK), "tyres")
    S.therm(S.rev(f"rim-{tag}", bone, [(-0.0155, 0.33), (-0.0155, 0.299), (0.0155, 0.299), (0.0155, 0.33), (-0.0155, 0.33)], c, [0, 0, 1], ALLOY, 0.78), "rims-spokes", own=True)
    S.struct_of(f"wheel-{tag}", bone)
    outer, inner = pt.ring(c, 0.345, 0.297, 32)
    S.swept(f"wheel-{tag}", bone, outer, -0.0185, 0.0185, "body", DARK, None, RUBBER, holes=[inner], tol=0.0012, views=SIDE, regions=["tyres", "rims-spokes"])
    for k, half in enumerate(pt.half_rings(c, 0.345, 0.297, 16)):
        S.swept(f"wheel-{tag}-{k + 1}", bone, half, -0.0185, 0.0185, "body", DARK, None, RUBBER, tol=0.0012, views=("front", "plan"), regions=["tyres", "rims-spokes"])
    S.put(S.cyl(f"hub-{tag}", bone, [c[0], c[1], -0.05], [c[0], c[1], 0.05], 0.035, CHROME, STEEL), "hub-brakes")
    S.struct(S.cyl(f"bearing-{tag}", bone, [c[0], c[1], -0.05], [c[0], c[1], 0.05], 0.015, CHROME, 0.5), bone, "joint")
    S.disc(f"hub-{tag}", bone, c, 0.07, -0.05, 0.05, "detail", STEEL, "hub-brakes", CHROME, pen=PEN)
    a, b = [0.03, 0.0, 0.0], [0.298, 0.0, 0.0]
    s = S.cap(f"spoke-{tag}", bone, kit.add(c, a), kit.add(c, b), 0.001, ALLOY, 0.6, role="texture", slender=True, texture=True, repeat={"count": 6, "rotationAxis": [0, 0, 1], "rotationRad": round(math.radians(60), 6), "origin": [0, 0, 0]})
    S.put(s, "rims-spokes")
    S.struct_of(f"spoke-{tag}", bone)
    for j in range(6):
        q = math.radians(60 * j)
        S.sliver(f"spoke-{tag}-{j + 1}", bone, kit.add(c, [0.03 * math.cos(q), 0.03 * math.sin(q), 0]), kit.add(c, [0.298 * math.cos(q), 0.298 * math.sin(q), 0]), "texture", 0.6, "rims-spokes", ALLOY, PEN)
    S.struct(S.ball(f"mass-wheel-{tag}", bone, [0.03, 0.03, 0.03], c, CHROME, 0.5), bone, "mass")


def bar(S, i, bone, a, b, r, region, mat=PAINT, tone=TUBE, role="body", depth=None, capped=False):
    S.put((S.cap if capped else S.cyl)(i, bone, a, b, r, mat, tone, role=role), region)
    S.struct_of(i, bone)
    S.member(i, bone, a, b, 2 * r, role, tone, region, mat, pen=PEN, depth=depth)


SADDLE = [(-0.222, 0.975, 0.016, 0.02), (-0.3, 0.972, 0.045, 0.027), (-0.4, 0.972, 0.098, 0.031), (-0.49, 0.978, 0.09, 0.028)]


def frame(S):
    reg = "frame-tubes"
    bar(S, "seat-tube", "frame", B, kit.add(pt.POST_TOP, [pt.ST[0], pt.ST[1], 0], 0.02), 0.0143, reg)
    bar(S, "down-tube", "frame", pt.DOWN_HEAD, B, 0.0159, reg)
    bar(S, "head-tube", "frame", sp(-0.004, 0), sp(0.246, 0), 0.0185, reg)
    loop = [pt.LOOP_HEAD] + kit.bezier(pt.LOOP_Q1, pt.LOOP_C, pt.LOOP_END, 3)
    S.put(S.tube("loop-tube", "frame", [(p, 0.0143, 0.0143) for p in loop], PAINT, TUBE), reg)
    S.struct_of("loop-tube", "frame")
    S.swept("loop-tube", "frame", kit.ribbon(loop, 0.0143), -0.0143, 0.0143, "body", TUBE, reg, PAINT, pen=PEN, tol=0.003)
    for side, name in ((1, "l"), (-1, "r")):
        for key, (a, b, r0, r1) in pt.stay_points().items():
            a, b = pt.mirror(a, side), pt.mirror(b, side)
            bar(S, f"{key}-{name}", "frame", a, b, 0.009 if key == "chainstay" else 0.0075, reg, depth=[max(a[0], b[0]), max(a[1], b[1]), side * 0.03] if key == "chainstay" else None, capped=True)
    x1 = pt.RACK_X[1]
    y, z = pt.RACK_Y, pt.RACK_Z
    xf = pt.SEAT_TOP[0] - 0.012
    hoop = [[xf, y, z], [x1, y, z], [x1, y, -z], [xf, y, -z]]
    S.put(S.tube("rack-hoop", "frame", [(p, 0.005, 0.005) for p in kit.bent(hoop, [0.03, 0.03], 2)], PAINT, TUBE, hint=[0.0, 1.0, 0.0], role="detail"), reg)
    S.struct_of("rack-hoop", "frame")
    S.flat("elevation", "rack-hoop", "frame", "detail", TUBE, reg, PAINT, [[xf, y - 0.005], [x1 - 0.005, y - 0.005], [x1 - 0.005, y + 0.005], [xf, y + 0.005]], [xf, y, -z], pen=0.16)
    S.flat("front", "rack-hoop", "frame", "detail", TUBE, reg, PAINT, [[-z - 0.005, y - 0.005], [z + 0.005, y - 0.005], [z + 0.005, y + 0.005], [-z - 0.005, y + 0.005]], [xf, y, -z], pen=PEN)
    S.flat("plan", "rack-hoop", "frame", "detail", TUBE, reg, PAINT, kit.ribbon([[p[0], p[2]] for p in hoop], 0.005), [xf, y, -z], pen=PEN, tol=0.0)
    for side, name in ((1, "l"), (-1, "r")):
        bar(S, f"rack-leg-{name}", "frame", [-0.7, y, side * z], [R[0], R[1] + 0.014, side * 0.073], 0.005, reg, role="detail")
    S.put(S.tube("saddle", "frame", [([x, yc, 0.0], w, t, 2.6) for x, yc, w, t in SADDLE], SKIN, 0.2), "saddle")
    S.flat("elevation", "saddle", "frame", "body", 0.2, "saddle", SKIN, [[x, yc + t] for x, yc, w, t in SADDLE] + [[x, yc - t] for x, yc, w, t in SADDLE][::-1], [-0.22, 1.01, -0.1], tol=0.003)
    S.flat("plan", "saddle", "frame", "body", 0.2, "saddle", SKIN, [[x, w] for x, yc, w, t in SADDLE] + [[x, -w] for x, yc, w, t in SADDLE][::-1], [-0.22, 1.01, -0.1], tol=0.003)
    S.flat("front", "saddle", "frame", "body", 0.2, "saddle", SKIN, [[0.098 * math.copysign(abs(math.cos(a)) ** (2 / 2.6), math.cos(a)), 0.974 + 0.031 * math.copysign(abs(math.sin(a)) ** (2 / 2.6), math.sin(a))] for a in [2 * math.pi * k / 12 for k in range(12)]], [-0.22, 1.01, -0.1], tol=0.003)
    out = pt.case_outline(3)
    S.put(S.prism("chaincase", "frame", out, pt.CASE_Z[0], pt.CASE_Z[1], PAINT, CASE), "drivetrain")
    S.swept("chaincase", "frame", out, pt.CASE_Z[0], pt.CASE_Z[1], "body", CASE, "drivetrain", PAINT, pen=PEN, tol=0.002)
    c = kit.polar(R, kit.FENDER_R + 0.02, pt.TAIL_DEG)
    S.put(S.box("tail-lamp", "frame", [0.04, 0.05, 0.045], c, PLASTIC, 0.3, corner=0.006, rotation=kit.rot_w(pt.TAIL_DEG)), "lamps")
    e = [math.cos(math.radians(pt.TAIL_DEG)), math.sin(math.radians(pt.TAIL_DEG))]
    t = [-e[1], e[0]]
    S.swept("tail-lamp", "frame", [[c[0] + e[0] * a + t[0] * b, c[1] + e[1] * a + t[1] * b] for a, b in ((-0.02, -0.025), (0.02, -0.025), (0.02, 0.025), (-0.02, 0.025))], -0.0225, 0.0225, "body", 0.3, "lamps", PLASTIC, pen=PEN)
    fx, fy, m = pt.frame_com()
    S.struct(S.ball("mass-frame", "frame", [0.045, 0.045, 0.045], [fx, fy, 0], PAINT, 0.5), "frame", "mass")


def guards(S):
    reg = "mudguards"
    for i, bone, c, a0, a1 in (("fender-r", "frame", R, 0.0, 200.0), ("fender-f", "steer", F, 62.0, 215.0)):
        deg = kit.BONES[bone][1]
        o = kit.loc(bone, c)
        cs, sn = math.cos(math.radians(a0 - deg)), math.sin(math.radians(a0 - deg))
        s = S.solid(i, "revolve", bone, PAINT, GUARD, profile=[[kit.r5(z), kit.r5(r)] for z, r in pt.fender_profile(n=2)], axis=[0, 0, 1], origin=kit.vec(o), sweepRad=round(math.radians(a1 - a0), 6))
        s["transform"] = {"rotation": kit.rot_w(a0 - deg), "translation": kit.vec([o[0] - (o[0] * cs - o[1] * sn), o[1] - (o[0] * sn + o[1] * cs), 0.0])}
        S.put(s, reg)
        band = kit.arc(c, kit.FENDER_R + 0.0004, a0, a1, 14) + kit.arc(c, kit.FENDER_R - 0.0104, a1, a0, 14)
        S.swept(i, bone, band, -0.024, 0.024, "body", GUARD, reg, PAINT, pen=PEN, tol=0.0015)


def drive(S):
    reg = "drivetrain"
    for sign, side, name in ((1, -1, "n"), (-1, 1, "f")):
        z = side * 0.084
        a, b = [B[0], B[1], z], [B[0] + sign * kit.CRANK, B[1], z]
        S.put(S.cap(f"crank-arm-{name}", "crank", a, b, 0.011, PAINT, 0.35), reg)
        S.struct_of(f"crank-arm-{name}", "crank")
        S.plank(f"crank-arm-{name}", "crank", [a[0] - sign * 0.011, a[1]], [b[0] + sign * 0.011, b[1]], 0.022, z, "body", 0.35, reg, PAINT, pen=PEN)
        bone = f"pedal-{name}"
        e = [B[0] + sign * kit.CRANK, B[1], 0.0]
        zc = side * 0.145
        S.put(S.box(f"pedal-{name}", bone, [0.08, 0.03, 0.1], [e[0], e[1], zc], RUBBER, 0.15), reg)
        S.struct_of(f"pedal-{name}", bone)
        S.swept(f"pedal-{name}", bone, [[e[0] - 0.04, e[1] - 0.015], [e[0] + 0.04, e[1] - 0.015], [e[0] + 0.04, e[1] + 0.015], [e[0] - 0.04, e[1] + 0.015]], zc - 0.05, zc + 0.05, "body", 0.15, reg, RUBBER, pen=PEN)
    S.put(S.cyl("crank-axle", "crank", [B[0], B[1], -0.095], [B[0], B[1], 0.095], 0.02, PAINT, TUBE), "frame-tubes")
    S.struct_of("crank-axle", "crank")
    S.struct(S.cyl("bb-bearing", "crank", [B[0], B[1], -0.034], [B[0], B[1], 0.034], 0.015, CHROME, 0.5), "crank", "joint")
    S.disc("crank-axle", "crank", B, 0.04, -0.095, 0.095, "body", TUBE, "frame-tubes", PAINT, pen=PEN)


def steer(S):
    reg = "steer-assembly"
    S.struct(S.cyl("steerer", "steer", sp(0.0, 0), sp(pt.CROWN_U, 0), 0.0127, PAINT, 0.4), "steer")
    S.struct(S.cyl("head-bearing", "steer", sp(0.1, 0), sp(0.14, 0), 0.0182, CHROME, 0.5), "steer", "joint")
    a, b = sp(pt.CROWN_U, 0, -0.058), sp(pt.CROWN_U, 0, 0.058)
    S.put(S.cap("fork-crown", "steer", a, b, 0.013, PAINT, TUBE), reg)
    S.struct_of("fork-crown", "steer")
    S.disc("fork-crown", "steer", sp(pt.CROWN_U, 0), 0.026, -0.071, 0.071, "body", TUBE, reg, PAINT, pen=PEN)
    blade = [pt.BLADE[k] for k in (0, 4, 7, 9)]
    for side, name in ((1, "l"), (-1, "r")):
        S.put(S.tube(f"fork-{name}", "steer", [(sp(u, v, side * z), 0.0115, 0.0115) for u, v, z, rz, rp in blade], PAINT, TUBE), reg)
        S.struct_of(f"fork-{name}", "steer")
        span = sorted((side * 0.0316, side * 0.0523))
        tip = sorted((side * 0.0447, side * 0.0588))
        S.swept(f"fork-{name}", "steer", kit.ribbon([sp(u, v)[:2] for u, v, z, rz, rp in blade], 0.0115), span[0], span[1], "body", TUBE, reg, PAINT, pen=PEN, tip=tip, tol=0.0)
    S.put(S.cap("stem", "steer", sp(0.0, 0), pt.CLAMP, 0.0125, CHROME, STEEL), reg)
    S.struct_of("stem", "steer")
    S.plank("stem", "steer", sp(0.0, 0), [pt.CLAMP[0] - 0.012, pt.CLAMP[1] + 0.006], 0.022, 0.0, "body", STEEL, reg, CHROME, pen=PEN)
    for side, name in ((1, "l"), (-1, "r")):
        path = pt.bar_path(side, 2)
        end = pt.mirror(pt.BAR_END, side)
        g0 = pt.grip_start(side)
        S.put(S.tube(f"handlebar-{name}", "steer", [(p, 0.012, 0.012) for p in path[:-1]] + [(kit.lerp(path[-2], g0, 0.9), 0.012, 0.012), (g0, 0.016, 0.016), (end, 0.016, 0.016)], CHROME, 0.45, hint=[0.0, 1.0, 0.0]), reg)
        knots = path[:-1] + [g0, end]
        for k in range(len(knots) - 1):
            S.struct(S.cyl(f"handlebar-{name}-{k + 1}", "steer", knots[k], knots[k + 1], 0.012, CHROME, 0.5), "steer")
            S.member(f"handlebar-{name}-{k + 1}", "steer", knots[k], knots[k + 1], 0.032 if k == len(knots) - 2 else 0.024, "body", 0.45, reg, CHROME, pen=PEN)
    ax = pt.LAMP_AXIS
    S.put(S.rev("head-lamp", "steer", [(-0.062, 0.012), (-0.012, 0.034), (0.03, 0.038), (0.036, 0.02)], pt.LAMP, ax, PLASTIC, 0.3), "lamps")
    S.member("head-lamp", "steer", kit.add(pt.LAMP, ax, -0.03), kit.add(pt.LAMP, ax, 0.034), 0.074, "body", 0.3, "lamps", PLASTIC, pen=PEN)
    S.struct(S.ball("mass-steer", "steer", [0.03, 0.03, 0.03], sp(0.3161, -0.0086), PAINT, 0.5), "steer", "mass")
    S.struct(S.box("contact-pad-f", "steer", [0.04, 0.03, 0.037], [F[0], 0.015, 0.0], RUBBER, 0.1, rotation=kit.rot_w(70)), "steer", "support", anchor="contact-f")


def stand(S):
    reg = "frame-tubes"
    deg = math.radians(kit.STAND_REST_DEG)
    u = [math.cos(deg), math.sin(deg), 0.0]
    at = lambda d, z: [P[0] + u[0] * d, P[1] + u[1] * d, z]
    for side, name in ((1, "l"), (-1, "r")):
        a, b = at(0.0, side * 0.03), at(kit.STAND_LENGTH - 0.009, side * 0.09)
        S.put(S.cap(f"stand-leg-{name}", "stand", a, b, 0.009, PAINT, TUBE), reg)
        S.struct_of(f"stand-leg-{name}", "stand")
        span, tip = sorted((side * 0.021, side * 0.039)), sorted((side * 0.081, side * 0.099))
        quad = [[P[0] + u[0] * d - u[1] * h, P[1] + u[1] * d + u[0] * h] for d, h in ((-0.009, 0.009), (-0.009, -0.009), (kit.STAND_LENGTH, -0.009), (kit.STAND_LENGTH, 0.009))]
        S.swept(f"stand-leg-{name}", "stand", quad, span[0], span[1], "body", TUBE, reg, PAINT, pen=PEN, tip=tip, tol=0.0)
    S.struct(S.cyl("stand-hinge", "stand", at(0, -0.03), at(0, 0.03), 0.015, CHROME, 0.5), "stand", "joint")


def build():
    S = kit.Scale(200, PENS)
    wheel(S, "wheel-r", R)
    wheel(S, "wheel-f", F)
    frame(S)
    guards(S)
    drive(S)
    steer(S)
    stand(S)
    return S


DECISIONS = [
    {
        "en": "1:200 is authored anew for the 30 mm feature: each wheel is one ring that merges tyre and rim, a hub cylinder that includes brake unit and axle, and six spokes; the frame is the seat tube with its post in one, the down tube, the loop tube with four sections, the head tube and one even bar per stay, all at their real width because the drawing shows them as bars of that width; the carrier is one hoop on a single leg per side; the saddle is one pad that takes in springs and clamp; the chain case has a seven-point outline; the bottom bracket with crank axle and bosses is one cylinder, each crank one bar with one pedal block; the mudguards are a folded sheet; the fork is a crown bar and two four-section blades; quill, stem head and clamp are one bar; each handlebar half is one bent tube that thickens into its grip; the head lamp with its bracket is one housing; the stand is two straight legs. Bell, levers, bridges, dropouts, stand bracket and saddle springs are below the feature and left out.",
        "de": "1:200 ist für das 30-mm-Merkmal neu aufgebaut: jedes Rad ist ein Ring aus Reifen und Felge, ein Nabenzylinder samt Bremseinheit und Achse und sechs Speichen; der Rahmen besteht aus Sattelrohr und Stütze in einem, dem Unterrohr, dem Bügelrohr mit vier Schnitten, dem Steuerrohr und je einem gleichmäßigen Stab je Strebe, alle in wirklicher Breite, weil die Zeichnung sie als Stäbe dieser Breite zeigt; der Gepäckträger ist ein Bügel auf einem Bein je Seite; der Sattel ist ein Polster, das Federn und Kloben aufnimmt; der Kettenkasten hat eine Sieben-Punkt-Kontur; Tretlager mit Kurbelachse und Naben ist ein Zylinder, jede Kurbel ein Stab mit einem Pedalblock; die Schutzbleche sind ein gekantetes Blech; die Gabel besteht aus Kronenstab und zwei Scheiden mit vier Schnitten; Schaft, Vorbaukopf und Klemme sind ein Stab; jede Lenkerhälfte ist ein gebogenes Rohr, das sich zum Griff verdickt; der Scheinwerfer mit seinem Halter ist ein Gehäuse; der Ständer besteht aus zwei geraden Beinen. Klingel, Hebel, Stege, Ausfallenden, Ständerkonsole und Sattelfedern liegen unter dem Merkmal und entfallen.",
    },
    {
        "en": "Thermal: references to the body solids; the wheel ring is the tyre surface and the rim is its own ring inside it, the bottom bracket cylinder belongs to the frame tubes. Only the sum of the areas is compared at this scale.",
        "de": "Thermik: Verweise auf die Körper; der Radring ist die Reifenoberfläche und die Felge ein eigener Ring darin, der Tretlagerzylinder gehört zu den Rahmenrohren. In diesem Maßstab wird nur die Flächensumme verglichen.",
    },
    {
        "en": "Structure: frame tubes, stays, carrier hoop and legs, a solid steerer in the head tube with one bearing sleeve, crown, blades and stem bar, the handlebar as round bars along its bend, wheel ring and six spokes per wheel with the hub bearing, bottom bracket cylinder with crank bars and bearing, pedal blocks, stand legs with the hinge, the front tyre patch as a 30 mm support and the masses of frame, steer assembly and wheels.",
        "de": "Tragwerk: Rahmenrohre, Streben, Gepäckträgerbügel und -beine, ein massiver Gabelschaft im Steuerrohr mit einer Lagerhülse, Krone, Scheiden und Vorbaustab, der Lenker als Rundstäbe entlang seiner Biegung, Radring und sechs Speichen je Rad mit dem Nabenlager, Tretlagerzylinder mit Kurbelstäben und Lager, Pedalblöcke, Ständerbeine mit dem Gelenk, die vordere Reifenaufstandsfläche als 30-mm-Auflager und die Massen von Rahmen, Lenkeinheit und Rädern.",
    },
]

DRAWN = {
    "silhouette": {
        "en": "The true orthographic views of the 1:200 body. Side: two wheel rings with hub and six spokes, the frame tubes and stays as bars of their real width, carrier hoop with one leg, saddle pad, chain case, crank bars with pedal blocks, mudguards, fork, stem, handlebar, both lamps and the folded stand. Front and top show the same parts.",
        "de": "Die wahren orthogonalen Ansichten des 1:200-Körpers. Seite: zwei Radringe mit Nabe und sechs Speichen, die Rahmenrohre und Streben als Stäbe in wirklicher Breite, Gepäckträgerbügel mit einem Bein, Sattelpolster, Kettenkasten, Kurbelstäbe mit Pedalblöcken, Schutzbleche, Gabel, Vorbau, Lenker, beide Leuchten und der hochgeklappte Ständer. Vorder- und Draufsicht zeigen dieselben Teile.",
    },
    "retained": {"en": "Wheel rings, hubs, all frame tubes and stays, carrier, saddle, chain case, cranks, pedals, mudguards, fork, stem, handlebar, lamps and stand.", "de": "Radringe, Naben, alle Rahmenrohre und Streben, Gepäckträger, Sattel, Kettenkasten, Kurbeln, Pedale, Schutzbleche, Gabel, Vorbau, Lenker, Leuchten und Ständer."},
    "merged": {"en": "Tyre and rim are one ring; hub, brake unit and axle one cylinder; seat tube and post one bar; springs and clamp part of the saddle pad; bottom bracket, crank axle and bosses one cylinder; quill, stem head and clamp one bar; handlebar and grip one tube; lamp and bracket one housing; the 36 real spokes are six.", "de": "Reifen und Felge sind ein Ring; Nabe, Bremseinheit und Achse ein Zylinder; Sattelrohr und Stütze ein Stab; Federn und Kloben Teil des Sattelpolsters; Tretlager, Kurbelachse und Naben ein Zylinder; Schaft, Vorbaukopf und Klemme ein Stab; Lenker und Griff ein Rohr; Lampe und Halter ein Gehäuse; aus den 36 wirklichen Speichen werden sechs."},
    "omitted": {"en": "Bell, brake levers, bridges, dropouts, stand bracket, second carrier leg, mudguard stays, chain, cables.", "de": "Klingel, Bremshebel, Stege, Ausfallenden, Ständerkonsole, zweites Gepäckträgerbein, Schutzblechstreben, Kette, Züge."},
    "thinElements": {"en": "Tubes of 29 to 37 mm are filled bars 0.14 to 0.19 mm wide with a 0.15 mm outline, stays 0.08 mm, carrier rod 0.05 mm: on paper each reads as one line of 0.2 to 0.3 mm; spokes are 0.15 mm strokes.", "de": "Rohre von 29 bis 37 mm sind gefüllte Stäbe von 0,14 bis 0,19 mm Breite mit 0,15 mm Kontur, Streben 0,08 mm, Gepäckträgerstab 0,05 mm: auf dem Papier liest sich jedes als eine Linie von 0,2 bis 0,3 mm; Speichen sind 0,15-mm-Striche."},
    "hatching": {"en": "None.", "de": "Keine."},
    "styles": {"en": "Dark wheel rings and pedals, mid-tone tubes, lighter mudguards and chain case, light hubs, dark saddle.", "de": "Dunkle Radringe und Pedale, mitteltonige Rohre, hellere Schutzbleche und Kettenkasten, helle Naben, dunkler Sattel."},
    "overlays": {"en": "The wheel ring carries the tyre and the rim region, every other shape the region of its part. Straight tubes are members, plates and bent tubes extruded profiles, so one definition gives side, front and top and follows steering (a twist about the head tube axis), rolling wheels, turning cranks and the folding stand; only the saddle and the carrier hoop have an outline per view.", "de": "Der Radring trägt die Reifen- und die Felgenregion, jede andere Form die Region ihres Teils. Gerade Rohre sind Stäbe, Platten und gebogene Rohre extrudierte Profile; so liefert eine Definition Seite, Vorderansicht und Draufsicht und folgt dem Lenken (einer Drehung um die Steuerrohrachse), den rollenden Rädern, den drehenden Kurbeln und dem klappbaren Ständer; nur Sattel und Gepäckträgerbügel haben je Ansicht eine Kontur."},
}
