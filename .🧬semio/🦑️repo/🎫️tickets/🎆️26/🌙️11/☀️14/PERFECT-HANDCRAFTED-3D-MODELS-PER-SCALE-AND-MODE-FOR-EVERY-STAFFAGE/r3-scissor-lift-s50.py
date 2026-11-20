"""🔩 The 1:50 scissor lift, part by part: chassis tub with swing-out trays, four solid-tyre wheels, four scissor stages of nested tube arms with pivot tubes, lift cylinder, platform frame, slide-out deck and folding guard rails; and its own load path."""
import math

CP, SS, DS, RS, RB = "chassis-paint", "scissor-steel", "deck-steel", "rail-steel", "rubber"
A_Z, B_Z = (0.27, 0.32), (0.215, 0.265)
TYRE = [(-0.055, 0.105), (-0.055, 0.138), (-0.047, 0.149), (-0.035, 0.1525), (0.035, 0.1525), (0.047, 0.149), (0.055, 0.138), (0.055, 0.105)]
RIM = [(-0.05, 0.105), (0.05, 0.105), (0.05, 0.092), (0.022, 0.08), (0.022, 0)]
CAP = [(0.022, 0.034), (0.044, 0.034), (0.05, 0.026), (0.05, 0)]


def chassis(S, K):
    S.at((0, 0))
    S.prism("chassis-tub", "chassis", [(-0.855, 0.15), (-0.79, 0.06), (0.8425, 0.06), (0.9075, 0.15), (0.9075, 0.4), (-0.855, 0.4)], (-0.24, 0.24), CP, 0.45, "chassis")
    S.bar("tray-left", "chassis", (-0.47, 0.47), (0.075, 0.385), (0.24, 0.385), CP, 0.45, "chassis", radius=0.01, views="fp")
    S.bar("tray-right", "chassis", (-0.47, 0), (0.075, 0.385), (-0.385, -0.24), CP, 0.45, "chassis", radius=0.01)
    S.bar("power-unit-bay", "chassis", (0, 0.47), (0.075, 0.385), (-0.385, -0.24), CP, 0.45, "motor-pump", radius=0.01)
    S.bar("battery-cover", "chassis", (-0.455, -0.02), (0.09, 0.37), (0.385, 0.395), CP, 0.3, "battery-box", role="detail", mirror=True)
    S.bar("pump-cover", "chassis", (0.02, 0.455), (0.09, 0.37), (-0.395, -0.385), CP, 0.35, "motor-pump", role="detail")
    S.bar("controller-cover", "chassis", (0.02, 0.455), (0.09, 0.37), (0.385, 0.395), CP, 0.4, "chassis", role="detail", views="fp")
    S.bar("battery-handle", "chassis", (-0.3, -0.18), (0.325, 0.345), (0.395, 0.405), CP, 0.15, "battery-box", role="detail", mirror=True)
    S.bar("pump-handle", "chassis", (0.06, 0.18), (0.325, 0.345), (-0.405, -0.395), CP, 0.15, "motor-pump", role="detail")
    S.bar("ground-controls", "chassis", (0.26, 0.42), (0.19, 0.31), (-0.405, -0.395), CP, 0.15, "motor-pump", role="detail")
    S.bar("controller-handle", "chassis", (0.06, 0.18), (0.325, 0.345), (0.395, 0.405), CP, 0.15, "chassis", role="detail", views="fp")
    S.bar("battery-plume-vent", "chassis", (-0.39, -0.09), (0.385, 0.3925), (-0.375, -0.25), CP, 0.2, "battery-box", role="detail", views="fp")["tags"] = ["battery-plume"]
    S.bar("motor-vent-grille", "chassis", (0.9075, 0.915), (0.2, 0.32), (-0.07, 0.07), CP, 0.2, "motor-pump", role="detail", views="fp")["tags"] = ["motor-vent"]
    S.bar("pothole-guard", "chassis", (-0.44, 0.44), (0.06, 0.075), (0.3, 0.38), CP, 0.35, "chassis", mirror=True)
    for end, x in (("f", K.AXLE_X), ("r", -K.AXLE_X)):
        S.roll(f"axle-{end}", "chassis", x, K.AXLE_Y, 0.03, (0.24, 0.3), CP, 0.35, "chassis", views="fp", mirror=True)
    S.bar("track", "chassis", (K.XF - 0.84, K.XF + 0.07), (0.4, 0.4267), (0.172, 0.21), CP, 0.4, "chassis", mirror=True)
    S.prism("base-lug", "chassis", [(K.XR - 0.045, 0.4), (K.XR + 0.045, 0.4), (K.XR + 0.045, 0.47), (K.XR + 0.02, 0.505), (K.XR - 0.02, 0.505), (K.XR - 0.045, 0.47)], (0.33, 0.342), CP, 0.4, "chassis", mirror=True)
    S.bar("ladder-stile", "chassis", (-0.915, -0.865), (0.12, 0.85), (0.19, 0.202), CP, 0.35, "chassis", mirror=True)
    S.bar("ladder-bracket", "chassis", (-0.865, -0.855), (0.2, 0.36), (0.185, 0.207), CP, 0.35, "chassis", views="p", mirror=True)
    for i, y in enumerate((0.3, 0.55, 0.8)):
        S.roll(f"ladder-rung-{i + 1}", "chassis", -0.89, y, 0.0125, (-0.19, 0.19), CP, 0.35, "chassis", role="detail")


def wheels(S, K):
    for end, bone in (("f", "wheel-f"), ("r", "wheel-r")):
        for name, side in (("l", 1), ("r", -1)):
            near = "efp" if side < 0 else "fp"
            S.wheel(f"tyre-{end}{name}", bone, side, TYRE, (0.295, 0.405), RB, 0.12, "tyres", views=near)
            S.wheel(f"rim-{end}{name}", bone, side, RIM, (0.3, 0.4), CP, 0.6, "chassis", role="detail", views="e" if side < 0 else "", closed=False)
            S.wheel(f"hub-{end}{name}", bone, side, CAP, (0.372, 0.4), CP, 0.3, "chassis", role="detail", views="e" if side < 0 else "", closed=False)
            S.body.append({"id": f"nut-{end}{name}", "bone": bone, "material": CP, "role": "detail", "tone": 0.15, "kind": "revolve", "profile": [[0, 0.009], [0.012, 0.009]], "axis": [0, 0, side], "origin": [0.055, 0, round(side * (K.AXLE_Z + 0.022), 5)], "repeat": {"count": 5, "rotationAxis": [0, 0, 1], "rotationRad": round(2 * math.pi / 5, 6), "origin": [0, 0, 0]}})
            S.regions[f"nut-{end}{name}"] = "chassis"
            if side < 0:
                for i in range(5):
                    cx, cy = 0.055 * math.cos(2 * math.pi * i / 5), 0.055 * math.sin(2 * math.pi * i / 5)
                    S._draw(f"nut-{end}{name}-{i + 1}", bone, K.ngon(cx, cy, 0.009, 8), -0.384, -0.372, CP, 0.15, "detail", "chassis", "e", False, circle=(K.r5(cx), K.r5(cy), 0.009))


def scissor(S, K):
    L = K.ARM
    for k in range(1, K.STAGES + 1):
        a, b = f"a{k}", f"b{k}"
        S.at((0, 0))
        S.bar(f"arm-{a}", a, (-0.045, L + 0.045), (-0.045, 0.045), A_Z, SS, 0.4, "arms", radius=0.006, mirror=True)
        for name, u in (("root", 0), ("mid", L / 2), ("tip", L)):
            if k == K.STAGES and name == "tip":
                S.roll(f"roller-{a}", a, u, 0, 0.033, (0.33, 0.358), SS, 0.2, "arms", role="detail", mirror=True)
            S.roll(f"pin-{a}-{name}", a, u, 0, 0.032, (0.32, 0.33), SS, 0.1, "arms", role="detail", mirror=True)
        S.bar(f"arm-{b}", b, (-0.045, L + 0.045), (-0.045, 0.045), B_Z, SS, 0.5, "arms", radius=0.006, mirror=True)
        for name, u in (("root", 0), ("mid", L / 2), ("tip", L)):
            reach = 0.172 if (k == 1 and name == "root") else 0.215
            S.roll(f"tube-{b}-{name}", b, u, 0, 0.025, (-reach, reach), SS, 0.5, "arms", views="fp")
    S.roll("tube-b2-rod", "b2", K.CYL_S2, 0, 0.025, (-0.215, 0.215), SS, 0.5, "arms", views="fp")
    S.roll("roller-slide", "slide", 0, 0, 0.033, (0.172, 0.21), SS, 0.2, "arms", role="detail", views="fp", mirror=True)
    S.roll("cylinder-eye", "lift-cylinder", 0, 0, 0.045, (-0.03, 0.03), CP, 0.3, "arms", views="fp")
    S.rod("cylinder-barrel", "lift-cylinder", (0.03, 0.45), 0, 0, 0.05, CP, 0.3, "arms", views="fp")
    S.rod("cylinder-gland", "lift-cylinder", (0.45, 0.47), 0, 0, 0.056, CP, 0.25, "arms", views="fp")
    S.bar("cylinder-port", "lift-cylinder", (0.07, 0.13), (-0.07, -0.048), (-0.02, 0.02), CP, 0.25, "arms", role="detail", views="fp")
    S.rod("cylinder-rod", "lift-rod", (-0.5, -0.03), 0, 0, 0.025, SS, 0.8, "arms", views="fp")
    S.roll("rod-eye", "lift-rod", 0, 0, 0.038, (-0.025, 0.025), CP, 0.3, "arms", views="fp")


def platform(S, K):
    S.at(K.PLATFORM)
    P = "platform"
    S.bar("deck", P, (-0.815, 0.815), (0.95, 1.0), (-0.37, 0.37), DS, 0.5, "deck")
    S.bar("deck-channel-web", P, (-0.815, 0.815), (0.86, 0.95), (0.36, 0.37), DS, 0.5, "deck", mirror=True)
    S.bar("deck-channel-foot", P, (-0.815, 0.815), (0.86, 0.87), (0.33, 0.36), DS, 0.5, "deck", views="p", mirror=True)
    S.bar("deck-channel-head", P, (-0.815, 0.815), (0.94, 0.95), (0.33, 0.36), DS, 0.5, "deck", views="", mirror=True)
    S.bar("deck-end-rear", P, (-0.815, -0.775), (0.86, 0.95), (-0.33, 0.33), DS, 0.5, "deck", views="fp")
    S.bar("deck-end-front", P, (0.775, 0.815), (0.86, 0.95), (-0.33, 0.33), DS, 0.5, "deck", views="fp")
    S.bar("deck-joist", P, (-0.02, 0.02), (0.905, 0.95), (-0.33, 0.33), DS, 0.5, "deck", views="")
    S.bar("deck-lug", P, (K.XR - 0.04, K.XR + 0.04), (0.86, 0.95), (0.2, 0.212), DS, 0.45, "deck", views="", mirror=True)
    for name, x in (("rear", (-0.815, -0.775)), ("mid", (-0.02, 0.02)), ("front", (0.775, 0.815))):
        S.bar(f"post-{name}", P, x, (1.0, 2.11), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
    for name, x in (("a", (-0.775, -0.02)), ("b", (0.02, 0.775))):
        S.bar(f"top-rail-{name}", P, x, (2.07, 2.11), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
        S.bar(f"mid-rail-{name}", P, x, (1.53, 1.57), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
        S.bar(f"toe-board-{name}", P, x, (1.0, 1.15), (0.36, 0.368), RS, 0.3, "rails", mirror=True)
    S.bar("rear-top-rail", P, (-0.815, -0.775), (2.07, 2.11), (-0.33, 0.33), RS, 0.3, "rails", views="fp")
    S.bar("gate-stile", P, (-0.81, -0.785), (1.03, 1.57), (0.3, 0.325), RS, 0.3, "rails", views="fp", mirror=True)
    S.bar("gate-top", P, (-0.81, -0.785), (1.545, 1.57), (-0.3, 0.3), RS, 0.3, "rails", views="fp")
    S.bar("gate-bottom", P, (-0.81, -0.785), (1.15, 1.175), (-0.3, 0.3), RS, 0.3, "rails", views="fp")
    S.bar("gate-toe-board", P, (-0.812, -0.804), (1.005, 1.15), (-0.3, 0.3), RS, 0.3, "rails", views="fp")
    S.at(K.EXT)
    E = "deck-ext"
    S.bar("deck-ext", E, (-0.105, 0.815), (1.0, 1.035), (-0.31, 0.31), DS, 0.55, "deck")
    for name, x in (("rear", (-0.105, -0.065)), ("front", (0.775, 0.815))):
        S.bar(f"ext-post-{name}", E, x, (1.035, 2.11), (0.27, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("ext-top-rail", E, (-0.065, 0.775), (2.07, 2.11), (0.27, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("ext-mid-rail", E, (-0.065, 0.775), (1.53, 1.57), (0.27, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("ext-toe-board", E, (-0.065, 0.775), (1.035, 1.15), (0.302, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("front-top-rail", E, (0.775, 0.815), (2.07, 2.11), (-0.27, 0.27), RS, 0.3, "rails", views="fp")
    S.bar("front-mid-rail", E, (0.775, 0.815), (1.53, 1.57), (-0.27, 0.27), RS, 0.3, "rails", views="fp")
    S.bar("front-toe-board", E, (0.807, 0.815), (1.035, 1.15), (-0.27, 0.27), RS, 0.3, "rails", views="fp")
    S.bar("control-box", E, (0.47, 0.69), (1.8, 2.04), (-0.27, -0.15), CP, 0.15, "rails", role="detail", radius=0.015)
    S.post("joystick", E, 0.53, (2.04, 2.105), -0.21, 0.013, CP, 0.1, "rails", role="detail", views="fp")
    S.post("emergency-stop", E, 0.63, (2.04, 2.065), -0.21, 0.022, CP, 0.25, "rails", role="detail", views="fp")


def structure(S, K):
    L = K.ARM
    M = {"part": "member"}
    S.at((0, 0))
    S.tube("frame-rail", "chassis", (-0.8, 0.855), 0.16, 0.2, (0.12, 0.06), 0.005, CP, 0.45, mirror=True, member="chassis", **M)
    S.tube("stack-rail", "chassis", (K.XR - 0.06, K.XF + 0.08), 0.36, 0.2, (0.08, 0.06), 0.005, CP, 0.45, mirror=True, member="chassis", **M)
    for i, x in enumerate((-0.8, -0.25, 0.19, 0.795)):
        S.bar(f"frame-cross-{i + 1}", "chassis", (x, x + 0.06), (0.1, 0.22), (-0.17, 0.17), CP, 0.45, target="structure", member="chassis", **M)
    for i, x in enumerate((-0.79, -0.03, 0.74)):
        S.bar(f"frame-upright-{i + 1}", "chassis", (x, x + 0.05), (0.22, 0.32), (0.17, 0.23), CP, 0.45, mirror=True, target="structure", member="chassis", **M)
    for end, x in (("f", K.AXLE_X), ("r", -K.AXLE_X)):
        S.roll(f"axle-tube-{end}", "chassis", x, K.AXLE_Y, 0.03, (-0.3, 0.3), CP, 0.4, bore=0.022, target="structure", member="chassis", **M)
        for name, sign in (("l", 1), ("r", -1)):
            S.ref(f"member-tyre-{end}{name}", f"tyre-{end}{name}", f"wheel-{end}", "member")
            S.ref(f"member-rim-{end}{name}", f"rim-{end}{name}", f"wheel-{end}", "member")
            S.bar(f"support-contact-{end}-{name}", "chassis", (x - 0.04, x + 0.04), (0, 0.0075), (sign * K.AXLE_Z - 0.045, sign * K.AXLE_Z + 0.045), RB, 0.1, target="structure", member="chassis", part="support", anchor=f"contact-{end}-{name}")
    S.ref("member-track", "track", "chassis", "member")
    S.ref("member-base-lug", "base-lug", "chassis", "member")
    S.bar("mass-batteries", "chassis", (-0.44, -0.04), (0.1, 0.33), (0.25, 0.375), CP, 0.3, mirror=True, target="structure", member="chassis", part="mass")
    S.bar("mass-power-unit", "chassis", (0.06, 0.4), (0.1, 0.3), (-0.375, -0.26), CP, 0.3, target="structure", member="chassis", part="mass")
    for k in range(1, K.STAGES + 1):
        a, b = f"a{k}", f"b{k}"
        S.tube(f"member-{a}", a, (-0.045, L + 0.045), 0, 0.295, (0.09, 0.05), 0.004, SS, 0.4, mirror=True, member=a, **M)
        S.tube(f"member-{b}", b, (-0.045, L + 0.045), 0, 0.24, (0.09, 0.05), 0.004, SS, 0.5, mirror=True, member=b, **M)
        for name, u in (("root", 0), ("mid", L / 2), ("tip", L)):
            reach = 0.172 if (k == 1 and name == "root") else 0.215
            S.roll(f"member-tube-{b}-{name}", b, u, 0, 0.025, (-reach, reach), SS, 0.5, bore=0.02, target="structure", member=b, **M)
        S.roll(f"joint-{a}", a, 0, 0, 0.02, (0.2, 0.33), SS, 0.1, mirror=True, target="structure", member=a, part="joint")
        S.roll(f"joint-{b}", b, 0, 0, 0.02, (0.2, 0.33), SS, 0.1, mirror=True, target="structure", member=b, part="joint")
        S.roll(f"joint-centre-{k}", a, L / 2, 0, 0.02, (0.2, 0.33), SS, 0.1, mirror=True, target="structure", member=a, part="joint")
    S.roll("member-tube-b2-rod", "b2", K.CYL_S2, 0, 0.025, (-0.215, 0.215), SS, 0.5, bore=0.02, target="structure", member="b2", **M)
    S.ref("member-roller-slide", "roller-slide", "slide", "member")
    S.ref("member-roller-a4", "roller-a4", "a4", "joint")
    S.rod("member-cylinder", "lift-cylinder", (0.03, 0.45), 0, 0, 0.05, CP, 0.3, bore=0.042, target="structure", member="lift-cylinder", **M)
    S.roll("joint-cylinder", "lift-cylinder", 0, 0, 0.02, (-0.215, 0.215), SS, 0.1, target="structure", member="lift-cylinder", part="joint")
    S.rod("member-rod", "lift-rod", (-0.5, -0.03), 0, 0, 0.025, SS, 0.8, target="structure", member="lift-rod", **M)
    S.roll("joint-rod", "lift-rod", 0, 0, 0.02, (-0.215, 0.215), SS, 0.1, target="structure", member="lift-rod", part="joint")
    S.at(K.PLATFORM)
    P = "platform"
    for id in ("deck-channel-web", "deck-channel-foot", "deck-channel-head", "deck-end-rear", "deck-end-front", "deck-lug"):
        S.ref(f"member-{id}", id, P, "member")
    for i, x in enumerate((-0.6, -0.3, -0.02, 0.26, 0.56)):
        S.bar(f"member-joist-{i + 1}", P, (x, x + 0.04), (0.95, 0.99), (-0.36, 0.36), DS, 0.5, target="structure", member=P, **M)
    S.roll("joint-platform", P, K.XR, K.TOP_Y, 0.02, (0.2, 0.275), SS, 0.1, mirror=True, target="structure", member=P, part="joint")
    for id in ("post-rear", "post-mid", "post-front", "top-rail-a", "top-rail-b", "rear-top-rail"):
        S.ref(f"member-{id}", id, P, "member")
    S.own({"id": "mass-platform", "bone": P, "material": DS, "role": "body", "tone": 0.4, "kind": "ellipsoid", "radii": [0.12, 0.02, 0.12], "transform": {"translation": [0.7275, 0.075, 0]}}, "structure", member=P, part="mass")
    S.at(K.EXT)
    E = "deck-ext"
    S.bar("member-ext-runner", E, (-0.105, 0.815), (1.0, 1.035), (0.28, 0.31), DS, 0.55, mirror=True, target="structure", member=E, **M)
    for i, x in enumerate((-0.105, 0.34, 0.785)):
        S.bar(f"member-ext-cross-{i + 1}", E, (x, x + 0.03), (1.0, 1.035), (-0.28, 0.28), DS, 0.55, target="structure", member=E, **M)
    for id in ("ext-post-rear", "ext-post-front", "ext-top-rail", "front-top-rail"):
        S.ref(f"member-{id}", id, E, "member")


def build(S, K):
    chassis(S, K)
    wheels(S, K)
    scissor(S, K)
    platform(S, K)
    structure(S, K)
    return S


DECISIONS = [
    {
        "en": "Body: the machine as built, stowed. Chassis tub with chamfered ends, 0.06 m ground clearance, swing-out trays between the axles with battery, power-unit and controller doors, handles and the ground control panel, retracted pothole guards, three-rung entry ladder; four 305 x 110 mm solid tyres as bodies of revolution with rounded shoulders on dished steel rims with hub caps and five wheel nuts each; four scissor stages of 90 x 50 mm tubes (outer arms a, inner arms b nested beside them) with pivot bosses, full-width pivot tubes on the inner arms, track rollers at the sliding ends, and the lift cylinder with gland, port block and rod between the centre pivot of stage 1 and the lower cross tube of stage 2; platform pan on roller channels, slide-out deck riding on top with its own inboard rails, 40 mm folding guard rails with toe boards, half-height rear gate and the platform controller with joystick and emergency stop.",
        "de": "Körper: die Maschine wie gebaut, eingefahren. Fahrgestellwanne mit angeschrägten Enden, 0,06 m Bodenfreiheit, ausschwenkbare Wannen zwischen den Achsen mit Batterie-, Aggregat- und Steuerungstüren, Griffen und Bodensteuerpult, eingefahrener Schlaglochschutz, dreisprossige Einstiegsleiter; vier Vollgummireifen 305 x 110 mm als Rotationskörper mit runden Schultern auf geschüsselten Stahlfelgen mit Nabenkappe und je fünf Radmuttern; vier Scherenstufen aus Rohren 90 x 50 mm (Außenarme a, Innenarme b daneben verschachtelt) mit Gelenkaugen, durchgehenden Gelenkrohren an den Innenarmen, Laufrollen an den Gleitenden und dem Hubzylinder mit Stopfbuchse, Anschlussblock und Kolbenstange zwischen dem Mittelgelenk der Stufe 1 und dem unteren Querrohr der Stufe 2; Bühnenwanne auf Rollenprofilen, obenauf laufende Ausschubbühne mit eigenem innenliegendem Geländer, klappbares 40-mm-Geländer mit Fußleisten, halbhohes Hecktor und Bühnensteuerung mit Joystick und Not-Halt."
    },
    {
        "en": "Thermal: every region is the surface of its real parts (arms with pivot tubes and cylinder, chassis with rims and ladder, battery doors, power-unit door, deck with channels and slide-out deck, rails with controller, tyres); each region has one sheet-steel or rubber layer, so there are no nested shells; the motor vent is the louvre on the front end plate and the battery plume leaves through the vent strip on top of the right battery tray. Structure: its own load path, not the body: two box frame rails with cross members and uprights to the stack rails, axle tubes, tyres on rims on four contact pads; every arm as a pair of hollow 90 x 50 x 4 mm tubes with hollow pivot tubes and pins at each joint; cylinder barrel as a pipe with rod and both pins; platform channels, end members and joists, guard-rail posts and top rails, runners of the slide-out deck; batteries, power unit and platform as masses.",
        "de": "Thermik: jede Region ist die Oberfläche ihrer realen Teile (Arme mit Gelenkrohren und Zylinder, Fahrgestell mit Felgen und Leiter, Batterietüren, Aggregattür, Boden mit Profilen und Ausschubbühne, Geländer mit Steuerung, Reifen); jede Region hat eine Blech- oder Gummischicht, daher keine verschachtelten Schalen; die Motorlüftung ist das Lamellengitter an der vorderen Stirnplatte, die Batteriewärme steigt durch den Lüftungsstreifen auf der rechten Batteriewanne. Tragwerk: der eigene Lastpfad, nicht der Körper: zwei Kastenlängsträger mit Querträgern und Stützen zu den Scherenträgern, Achsrohre, Reifen auf Felgen auf vier Aufstandsflächen; jeder Arm als Paar hohler Rohre 90 x 50 x 4 mm mit hohlen Gelenkrohren und Bolzen an jedem Gelenk; Zylinderrohr als Rohr mit Stange und beiden Bolzen; Bühnenprofile, Endträger und Querträger, Geländerpfosten und Handläufe, Läufer der Ausschubbühne; Batterien, Aggregat und Bühne als Massen."
    }
]
