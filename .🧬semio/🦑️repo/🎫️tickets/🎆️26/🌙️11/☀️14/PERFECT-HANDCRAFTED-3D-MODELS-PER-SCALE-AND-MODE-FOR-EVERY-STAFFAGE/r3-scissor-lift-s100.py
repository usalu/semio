"""🪜 The 1:100 scissor lift, authored for a 15 mm feature: tub and trays without handles, tyres with one wheel disc, plain tube arms with one boss per pivot, a two-part lift cylinder, platform pan on one side beam per side, guard rails without the gate's lower bar."""

CP, SS, DS, RS, RB = "chassis-paint", "scissor-steel", "deck-steel", "rail-steel", "rubber"
A_Z, B_Z = (0.27, 0.32), (0.215, 0.265)
TYRE = [(-0.055, 0.105), (-0.055, 0.137), (-0.04, 0.1525), (0.04, 0.1525), (0.055, 0.137), (0.055, 0.105)]
DISC = [(-0.05, 0.105), (0.05, 0.105), (0.05, 0.09), (0.03, 0.075), (0.03, 0)]


def build(S, K):
    L = K.ARM
    M = {"part": "member"}
    S.at((0, 0))
    S.prism("chassis-tub", "chassis", [(-0.855, 0.15), (-0.79, 0.06), (0.835, 0.06), (0.9, 0.15), (0.9, 0.4), (-0.855, 0.4)], (-0.24, 0.24), CP, 0.45, "chassis")
    S.bar("tray-left", "chassis", (-0.47, 0.47), (0.06, 0.385), (0.24, 0.39), CP, 0.45, "chassis", views="fp")
    S.bar("tray-right", "chassis", (-0.47, 0), (0.06, 0.385), (-0.39, -0.24), CP, 0.45, "chassis")
    S.bar("power-unit-bay", "chassis", (0, 0.47), (0.06, 0.385), (-0.39, -0.24), CP, 0.45, "motor-pump")
    S.bar("battery-cover", "chassis", (-0.455, -0.02), (0.09, 0.37), (0.39, 0.405), CP, 0.3, "battery-box", role="detail", mirror=True)
    S.bar("pump-cover", "chassis", (0.02, 0.455), (0.09, 0.37), (-0.405, -0.39), CP, 0.35, "motor-pump", role="detail")
    S.bar("controller-cover", "chassis", (0.02, 0.455), (0.09, 0.37), (0.39, 0.405), CP, 0.4, "chassis", role="detail", views="fp")
    S.bar("battery-plume-vent", "chassis", (-0.39, -0.09), (0.385, 0.4), (-0.385, -0.25), CP, 0.2, "battery-box", role="detail", views="")["tags"] = ["battery-plume"]
    S.bar("motor-vent-grille", "chassis", (0.9, 0.915), (0.2, 0.32), (-0.07, 0.07), CP, 0.2, "motor-pump", role="detail", views="fp")["tags"] = ["motor-vent"]
    for end, x in (("f", K.AXLE_X), ("r", -K.AXLE_X)):
        S.roll(f"axle-{end}", "chassis", x, K.AXLE_Y, 0.03, (0.24, 0.3), CP, 0.35, "chassis", views="fp", mirror=True)
    S.bar("track", "chassis", (K.XF - 0.84, K.XF + 0.07), (0.4, 0.427), (0.172, 0.21), CP, 0.4, "chassis", views="fp", mirror=True)
    S.prism("base-lug", "chassis", [(K.XR - 0.045, 0.4), (K.XR + 0.045, 0.4), (K.XR + 0.045, 0.48), (K.XR - 0.045, 0.48)], (0.33, 0.345), CP, 0.4, "chassis", mirror=True)
    S.bar("ladder-stile", "chassis", (-0.915, -0.865), (0.12, 0.85), (0.185, 0.205), CP, 0.35, "chassis", mirror=True)
    for i, y in enumerate((0.3, 0.55, 0.8)):
        S.roll(f"ladder-rung-{i + 1}", "chassis", -0.89, y, 0.0125, (-0.185, 0.185), CP, 0.35, "chassis", role="detail", views="fp")
    for end, bone in (("f", "wheel-f"), ("r", "wheel-r")):
        for name, side in (("l", 1), ("r", -1)):
            S.wheel(f"tyre-{end}{name}", bone, side, TYRE, (0.295, 0.405), RB, 0.12, "tyres", views="efp" if side < 0 else "fp")
            S.wheel(f"wheel-disc-{end}{name}", bone, side, DISC, (0.3, 0.4), CP, 0.6, "chassis", role="detail", views="e" if side < 0 else "", closed=False)
    for k in range(1, K.STAGES + 1):
        a, b = f"a{k}", f"b{k}"
        S.bar(f"arm-{a}", a, (-0.045, L + 0.045), (-0.045, 0.045), A_Z, SS, 0.4, "arms", mirror=True)
        S.roll(f"pin-{a}-root", a, 0, 0, 0.03, (0.32, 0.335), SS, 0.1, "arms", role="detail", mirror=True)
        S.roll(f"pin-{a}-mid", a, L / 2, 0, 0.03, (0.32, 0.335), SS, 0.1, "arms", role="detail", mirror=True)
        S.bar(f"arm-{b}", b, (-0.045, L + 0.045), (-0.045, 0.045), B_Z, SS, 0.5, "arms", mirror=True)
        for name, u in (("root", 0), ("mid", L / 2), ("tip", L)):
            reach = 0.172 if (k == 1 and name == "root") else 0.215
            S.roll(f"tube-{b}-{name}", b, u, 0, 0.025, (-reach, reach), SS, 0.5, "arms", views="fp")
            S.ref(f"member-tube-{b}-{name}", f"tube-{b}-{name}", b, "member")
        S.tube(f"member-{a}", a, (-0.045, L + 0.045), 0, 0.295, (0.09, 0.05), 0.004, SS, 0.4, mirror=True, member=a, **M)
        S.tube(f"member-{b}", b, (-0.045, L + 0.045), 0, 0.24, (0.09, 0.05), 0.004, SS, 0.5, mirror=True, member=b, **M)
        S.roll(f"joint-{a}", a, 0, 0, 0.025, (0.215, 0.32), SS, 0.1, mirror=True, target="structure", member=a, part="joint")
        S.roll(f"joint-{b}", b, 0, 0, 0.025, (0.215, 0.32), SS, 0.1, mirror=True, target="structure", member=b, part="joint")
    S.roll("pin-a4-tip", "a4", L, 0, 0.033, (0.33, 0.358), SS, 0.2, "arms", role="detail", mirror=True)
    S.roll("tube-b2-rod", "b2", K.CYL_S2, 0, 0.025, (-0.215, 0.215), SS, 0.5, "arms", views="fp")
    S.roll("roller-slide", "slide", 0, 0, 0.033, (0.172, 0.21), SS, 0.2, "arms", role="detail", views="fp", mirror=True)
    S.rod("cylinder-barrel", "lift-cylinder", (-0.03, 0.47), 0, 0, 0.05, CP, 0.3, "arms", views="fp")
    S.rod("cylinder-rod", "lift-rod", (-0.5, 0.02), 0, 0, 0.025, SS, 0.8, "arms", views="fp")
    S.at(K.PLATFORM)
    P = "platform"
    S.bar("deck", P, (-0.815, 0.815), (0.95, 1.0), (-0.37, 0.37), DS, 0.5, "deck")
    S.bar("deck-beam", P, (-0.815, 0.815), (0.86, 0.95), (0.33, 0.37), DS, 0.5, "deck", mirror=True)
    S.bar("deck-end-rear", P, (-0.815, -0.775), (0.86, 0.95), (-0.33, 0.33), DS, 0.5, "deck", views="fp")
    S.bar("deck-end-front", P, (0.775, 0.815), (0.86, 0.95), (-0.33, 0.33), DS, 0.5, "deck", views="fp")
    for name, x in (("rear", (-0.815, -0.775)), ("mid", (-0.02, 0.02)), ("front", (0.775, 0.815))):
        S.bar(f"post-{name}", P, x, (1.0, 2.11), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
    for name, x in (("a", (-0.775, -0.02)), ("b", (0.02, 0.775))):
        S.bar(f"top-rail-{name}", P, x, (2.07, 2.11), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
        S.bar(f"mid-rail-{name}", P, x, (1.53, 1.57), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
        S.bar(f"toe-board-{name}", P, x, (1.0, 1.15), (0.355, 0.37), RS, 0.3, "rails", mirror=True)
    S.bar("rear-top-rail", P, (-0.815, -0.775), (2.07, 2.11), (-0.33, 0.33), RS, 0.3, "rails", views="fp")
    S.bar("gate-stile", P, (-0.81, -0.785), (1.15, 1.545), (0.3, 0.325), RS, 0.3, "rails", views="fp", mirror=True)
    S.bar("gate-top", P, (-0.81, -0.785), (1.545, 1.57), (-0.325, 0.325), RS, 0.3, "rails", views="fp")
    S.bar("gate-toe-board", P, (-0.815, -0.8), (1.0, 1.15), (-0.33, 0.33), RS, 0.3, "rails", views="fp")
    S.at(K.EXT)
    E = "deck-ext"
    S.bar("deck-ext", E, (-0.105, 0.815), (1.0, 1.035), (-0.31, 0.31), DS, 0.55, "deck")
    for name, x in (("rear", (-0.105, -0.065)), ("front", (0.775, 0.815))):
        S.bar(f"ext-post-{name}", E, x, (1.035, 2.11), (0.27, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("ext-top-rail", E, (-0.065, 0.775), (2.07, 2.11), (0.27, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("ext-mid-rail", E, (-0.065, 0.775), (1.53, 1.57), (0.27, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("ext-toe-board", E, (-0.065, 0.775), (1.035, 1.15), (0.295, 0.31), RS, 0.3, "rails", mirror=True)
    S.bar("front-top-rail", E, (0.775, 0.815), (2.07, 2.11), (-0.27, 0.27), RS, 0.3, "rails", views="fp")
    S.bar("front-mid-rail", E, (0.775, 0.815), (1.53, 1.57), (-0.27, 0.27), RS, 0.3, "rails", views="fp")
    S.bar("front-toe-board", E, (0.8, 0.815), (1.035, 1.15), (-0.27, 0.27), RS, 0.3, "rails", views="fp")
    S.bar("control-box", E, (0.47, 0.69), (1.8, 2.04), (-0.27, -0.15), CP, 0.15, "rails", role="detail")
    S.at((0, 0))
    S.tube("frame-rail", "chassis", (-0.8, 0.85), 0.16, 0.2, (0.12, 0.06), 0.005, CP, 0.45, mirror=True, member="chassis", **M)
    S.tube("stack-rail", "chassis", (K.XR - 0.06, K.XF + 0.08), 0.36, 0.2, (0.08, 0.06), 0.005, CP, 0.45, mirror=True, member="chassis", **M)
    for i, x in enumerate((-0.8, -0.03, 0.79)):
        S.bar(f"frame-cross-{i + 1}", "chassis", (x, x + 0.06), (0.1, 0.22), (-0.17, 0.17), CP, 0.45, target="structure", member="chassis", **M)
    for i, x in enumerate((-0.79, 0.74)):
        S.bar(f"frame-upright-{i + 1}", "chassis", (x, x + 0.05), (0.22, 0.32), (0.17, 0.23), CP, 0.45, mirror=True, target="structure", member="chassis", **M)
    for end, x in (("f", K.AXLE_X), ("r", -K.AXLE_X)):
        S.roll(f"axle-tube-{end}", "chassis", x, K.AXLE_Y, 0.03, (-0.3, 0.3), CP, 0.4, target="structure", member="chassis", **M)
        for name, sign in (("l", 1), ("r", -1)):
            S.ref(f"member-tyre-{end}{name}", f"tyre-{end}{name}", f"wheel-{end}", "member")
            S.ref(f"member-wheel-disc-{end}{name}", f"wheel-disc-{end}{name}", f"wheel-{end}", "member")
            S.bar(f"support-contact-{end}-{name}", "chassis", (x - 0.04, x + 0.04), (0, 0.015), (sign * K.AXLE_Z - 0.045, sign * K.AXLE_Z + 0.045), RB, 0.1, target="structure", member="chassis", part="support", anchor=f"contact-{end}-{name}")
    S.bar("mass-batteries", "chassis", (-0.44, -0.04), (0.1, 0.33), (0.25, 0.385), CP, 0.3, mirror=True, target="structure", member="chassis", part="mass")
    S.bar("mass-power-unit", "chassis", (0.06, 0.4), (0.1, 0.3), (-0.385, -0.26), CP, 0.3, target="structure", member="chassis", part="mass")
    S.ref("member-roller-slide", "roller-slide", "slide", "member")
    S.ref("member-tube-b2-rod", "tube-b2-rod", "b2", "member")
    S.rod("member-cylinder", "lift-cylinder", (0, 0.47), 0, 0, 0.05, CP, 0.3, bore=0.042, target="structure", member="lift-cylinder", **M)
    S.ref("member-rod", "cylinder-rod", "lift-rod", "member")
    for id in ("deck-beam", "deck-end-rear", "deck-end-front", "post-rear", "post-mid", "post-front", "top-rail-a", "top-rail-b"):
        S.ref(f"member-{id}", id, P, "member")
    S.at(K.PLATFORM)
    for i, x in enumerate((-0.45, -0.02, 0.41)):
        S.bar(f"member-joist-{i + 1}", P, (x, x + 0.04), (0.95, 0.99), (-0.36, 0.36), DS, 0.5, target="structure", member=P, **M)
    S.roll("joint-platform", P, K.XR, K.TOP_Y, 0.025, (0.2, 0.275), SS, 0.1, mirror=True, target="structure", member=P, part="joint")
    S.own({"id": "mass-platform", "bone": P, "material": DS, "role": "body", "tone": 0.4, "kind": "ellipsoid", "radii": [0.12, 0.02, 0.12], "transform": {"translation": [0.7275, 0.075, 0]}}, "structure", member=P, part="mass")
    S.at(K.EXT)
    S.bar("member-ext-runner", E, (-0.105, 0.815), (1.0, 1.035), (0.28, 0.31), DS, 0.55, mirror=True, target="structure", member=E, **M)
    for id in ("ext-post-rear", "ext-post-front", "ext-top-rail", "front-top-rail"):
        S.ref(f"member-{id}", id, E, "member")
    return S


DECISIONS = [
    {
        "en": "Body for a 15 mm feature: chassis tub and trays keep the battery, power-unit and controller doors but lose handles, ground control panel and pothole guards (merged into the trays); each wheel is a tyre with chamfered shoulders and one wheel disc; the eight arms stay 90 x 50 mm tubes with one boss at the foot and centre pivots, all pivot tubes and both track rollers; the lift cylinder is a barrel and a rod; the platform is the pan on one beam per side with end members; guard rails keep every 40 mm tube, the toe boards are 15 mm, the gate is a frame of stiles, top bar and toe board; the controller is a plain box.",
        "de": "Körper für 15 mm kleinstes Merkmal: Fahrgestellwanne und Seitenwannen behalten Batterie-, Aggregat- und Steuerungstüren, verlieren aber Griffe, Bodensteuerpult und Schlaglochschutz (in den Wannen aufgegangen); jedes Rad ist ein Reifen mit gefasten Schultern und eine Radscheibe; die acht Arme bleiben Rohre 90 x 50 mm mit je einem Auge an Fuß- und Mittelgelenk, allen Gelenkrohren und beiden Laufrollen; der Hubzylinder ist Rohr und Stange; die Bühne ist die Wanne auf je einem Seitenträger mit Endträgern; das Geländer behält jedes 40-mm-Rohr, die Fußleisten sind 15 mm, das Tor ist ein Rahmen aus Holmen, Oberholm und Fußleiste; die Steuerung ist ein schlichter Kasten."
    },
    {
        "en": "Thermal: the same seven regions on the simplified parts, vent louvre and battery vent strip as outlets. Structure: box frame rails and stack rails with three cross members and two uprights per side, solid axle bars, tyres on wheel discs on four pads; arms as hollow 90 x 50 x 4 mm tube pairs whose pivot tubes are the body tubes, one pin pair per arm foot; cylinder pipe and rod; platform beams, three joists, posts and top rails; runners and rail frame of the slide-out deck; batteries, power unit and platform as masses.",
        "de": "Thermik: dieselben sieben Regionen auf den vereinfachten Teilen, Lamellengitter und Batterielüftungsstreifen als Austritte. Tragwerk: Kastenlängsträger und Scherenträger mit drei Querträgern und zwei Stützen je Seite, volle Achsstäbe, Reifen auf Radscheiben auf vier Aufstandsflächen; Arme als hohle Rohrpaare 90 x 50 x 4 mm, deren Gelenkrohre die Körperrohre sind, ein Bolzenpaar je Armfuß; Zylinderrohr und Stange; Bühnenträger, drei Querträger, Pfosten und Handläufe; Läufer und Geländerrahmen der Ausschubbühne; Batterien, Aggregat und Bühne als Massen."
    }
]
