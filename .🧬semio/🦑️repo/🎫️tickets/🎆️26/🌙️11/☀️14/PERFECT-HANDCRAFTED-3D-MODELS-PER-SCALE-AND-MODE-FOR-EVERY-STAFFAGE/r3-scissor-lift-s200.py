"""📏 The 1:200 scissor lift, authored for a 30 mm feature: one chassis block with door slabs, disc wheels, eight closed bars that read as one stack when stowed and as crossing arms when raised, one cylinder, a deck slab with corner posts and rails."""

CP, SS, DS, RS, RB = "chassis-paint", "scissor-steel", "deck-steel", "rail-steel", "rubber"


def build(S, K):
    L = K.ARM
    M = {"part": "member"}
    S.at((0, 0))
    S.prism("chassis-tub", "chassis", [(-0.855, 0.15), (-0.79, 0.06), (0.85, 0.06), (0.915, 0.15), (0.915, 0.4), (-0.855, 0.4)], (-0.24, 0.24), CP, 0.45, "chassis")
    S.bar("tray", "chassis", (-0.47, 0.47), (0.06, 0.385), (0.24, 0.375), CP, 0.45, "chassis", mirror=True)
    S.bar("battery-cover", "chassis", (-0.47, 0), (0.06, 0.385), (0.375, 0.405), CP, 0.3, "battery-box", role="detail", mirror=True)
    S.bar("pump-cover", "chassis", (0, 0.47), (0.06, 0.385), (-0.405, -0.375), CP, 0.35, "motor-pump", role="detail")
    S.bar("controller-cover", "chassis", (0, 0.47), (0.06, 0.385), (0.375, 0.405), CP, 0.45, "chassis", views="fp")
    S.bar("ladder-stile", "chassis", (-0.915, -0.855), (0.12, 0.85), (0.17, 0.2), CP, 0.35, "chassis", mirror=True)
    for end, bone, x in (("f", "wheel-f", K.AXLE_X), ("r", "wheel-r", -K.AXLE_X)):
        S.at((0, 0))
        S.roll(f"axle-{end}", "chassis", x, K.AXLE_Y, 0.03, (0.24, 0.3), CP, 0.35, "chassis", views="", mirror=True)
        for name, side in (("l", 1), ("r", -1)):
            S.wheel(f"tyre-{end}{name}", bone, side, [(-0.055, 0), (-0.055, 0.1525), (0.055, 0.1525), (0.055, 0)], (0.295, 0.405), RB, 0.12, "tyres", views="efp" if side < 0 else "fp", closed=False)
            S.ref(f"member-tyre-{end}{name}", f"tyre-{end}{name}", bone, "member")
            S.bar(f"support-contact-{end}-{name}", "chassis", (x - 0.04, x + 0.04), (0, 0.03), (side * K.AXLE_Z - 0.045, side * K.AXLE_Z + 0.045), RB, 0.1, target="structure", member="chassis", part="support", anchor=f"contact-{end}-{name}")
        S.roll(f"axle-bar-{end}", "chassis", x, K.AXLE_Y, 0.03, (-0.3, 0.3), CP, 0.4, target="structure", member="chassis", **M)
    for k in range(1, K.STAGES + 1):
        a, b = f"a{k}", f"b{k}"
        S.bar(f"arm-{a}", a, (-0.045, L + 0.045), (-0.055, 0.055), (0.27, 0.32), SS, 0.4, "arms", mirror=True)
        S.bar(f"arm-{b}", b, (-0.045, L + 0.045), (-0.055, 0.055), (0.215, 0.265), SS, 0.5, "arms", mirror=True)
        for name, u in (("root", 0), ("mid", L / 2), ("tip", L)):
            S.roll(f"tube-{b}-{name}", b, u, 0, 0.03, (-0.215, 0.215), SS, 0.5, "arms", views="fp")
        S.bar(f"member-{a}", a, (-0.045, L + 0.045), (-0.045, 0.045), (0.27, 0.32), SS, 0.4, mirror=True, target="structure", member=a, **M)
        S.bar(f"member-{b}", b, (-0.045, L + 0.045), (-0.045, 0.045), (0.215, 0.265), SS, 0.5, mirror=True, target="structure", member=b, **M)
        S.roll(f"joint-{a}", a, 0, 0, 0.03, (-0.32, 0.32), SS, 0.1, target="structure", member=a, part="joint")
        S.roll(f"joint-{b}", b, 0, 0, 0.03, (-0.32, 0.32), SS, 0.1, target="structure", member=b, part="joint")
    S.roll("roller-slide", "slide", 0, 0, 0.035, (0.17, 0.21), SS, 0.2, "arms", views="", mirror=True)
    S.ref("member-roller-slide", "roller-slide", "slide", "member")
    S.rod("cylinder", "lift-cylinder", (-0.03, 0.47), 0, 0, 0.05, CP, 0.3, "arms", views="fp")
    S.rod("cylinder-rod", "lift-rod", (-0.45, 0.02), 0, 0, 0.025, SS, 0.8, "arms", views="fp")
    S.ref("member-cylinder", "cylinder", "lift-cylinder", "member")
    S.ref("member-rod", "cylinder-rod", "lift-rod", "member")
    S.at(K.PLATFORM)
    P = "platform"
    S.bar("deck", P, (-0.815, 0.815), (0.86, 1.0), (-0.37, 0.37), DS, 0.5, "deck")
    for name, x in (("rear", (-0.815, -0.775)), ("front", (0.775, 0.815))):
        S.bar(f"post-{name}", P, x, (1.0, 2.11), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
        S.ref(f"member-post-{name}", f"post-{name}", P, "member")
    S.bar("top-rail", P, (-0.775, 0.775), (2.07, 2.11), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
    S.bar("mid-rail", P, (-0.775, 0.775), (1.53, 1.57), (0.33, 0.37), RS, 0.3, "rails", mirror=True)
    S.bar("toe-board", P, (-0.775, 0.775), (1.0, 1.15), (0.34, 0.37), RS, 0.3, "rails", mirror=True)
    S.bar("rear-top-rail", P, (-0.815, -0.775), (2.07, 2.11), (-0.33, 0.33), RS, 0.3, "rails", views="fp")
    S.bar("gate", P, (-0.815, -0.775), (1.53, 1.57), (-0.33, 0.33), RS, 0.3, "rails", views="fp")
    S.bar("gate-toe-board", P, (-0.815, -0.785), (1.0, 1.15), (-0.33, 0.33), RS, 0.3, "rails", views="fp")
    S.bar("member-deck-beam", P, (-0.815, 0.815), (0.86, 0.95), (0.33, 0.37), DS, 0.5, mirror=True, target="structure", member=P, **M)
    for name, x in (("rear", (-0.815, -0.775)), ("front", (0.775, 0.815))):
        S.bar(f"member-deck-end-{name}", P, x, (0.86, 0.95), (-0.33, 0.33), DS, 0.5, target="structure", member=P, **M)
    S.ref("member-top-rail", "top-rail", P, "member")
    S.own({"id": "mass-platform", "bone": P, "material": DS, "role": "body", "tone": 0.4, "kind": "ellipsoid", "radii": [0.12, 0.02, 0.12], "transform": {"translation": [0.7275, 0.075, 0]}}, "structure", member=P, part="mass")
    S.at(K.EXT)
    E = "deck-ext"
    S.bar("deck-ext", E, (-0.105, 0.815), (1.0, 1.035), (-0.33, 0.33), DS, 0.55, "deck")
    for name, x in (("rear", (-0.105, -0.065)), ("front", (0.775, 0.815))):
        S.bar(f"ext-post-{name}", E, x, (1.035, 2.11), (0.29, 0.33), RS, 0.3, "rails", mirror=True)
    S.bar("ext-top-rail", E, (-0.065, 0.775), (2.07, 2.11), (0.29, 0.33), RS, 0.3, "rails", mirror=True)
    S.bar("ext-mid-rail", E, (-0.065, 0.775), (1.53, 1.57), (0.29, 0.33), RS, 0.3, "rails", mirror=True)
    S.bar("front-top-rail", E, (0.775, 0.815), (2.07, 2.11), (-0.29, 0.29), RS, 0.3, "rails", views="fp")
    S.bar("front-mid-rail", E, (0.775, 0.815), (1.53, 1.57), (-0.29, 0.29), RS, 0.3, "rails", views="fp")
    S.bar("front-toe-board", E, (0.785, 0.815), (1.035, 1.15), (-0.29, 0.29), RS, 0.3, "rails", views="fp")
    S.bar("control-box", E, (0.47, 0.69), (1.8, 2.04), (-0.29, -0.17), CP, 0.15, "rails", role="detail")
    S.bar("member-ext-runner", E, (-0.105, 0.815), (1.0, 1.035), (0.27, 0.33), DS, 0.55, mirror=True, target="structure", member=E, **M)
    S.ref("member-ext-post-front", "ext-post-front", E, "member")
    S.at((0, 0))
    S.bar("frame-rail", "chassis", (-0.8, 0.86), (0.1, 0.22), (0.17, 0.23), CP, 0.45, mirror=True, target="structure", member="chassis", **M)
    S.bar("stack-rail", "chassis", (K.XR - 0.06, K.XF + 0.08), (0.32, 0.4), (0.17, 0.23), CP, 0.45, mirror=True, target="structure", member="chassis", **M)
    for i, x in enumerate((-0.8, 0.8)):
        S.bar(f"frame-cross-{i + 1}", "chassis", (x, x + 0.06), (0.1, 0.4), (-0.17, 0.17), CP, 0.45, target="structure", member="chassis", **M)
    S.bar("mass-batteries", "chassis", (-0.44, -0.04), (0.1, 0.33), (0.25, 0.37), CP, 0.3, mirror=True, target="structure", member="chassis", part="mass")
    S.bar("motor-vent-outlet", "chassis", (0.9, 0.93), (0.2, 0.32), (-0.07, 0.07), CP, 0.2, target="thermal", region="motor-pump", tags=["emitter", "motor-vent"])
    S.bar("battery-plume-outlet", "chassis", (-0.39, -0.09), (0.37, 0.4), (-0.375, -0.25), CP, 0.2, target="thermal", region="battery-box", tags=["emitter", "battery-plume"])
    return S


DECISIONS = [
    {
        "en": "Body for a 30 mm feature: the chassis is one tub block with the trays and three door slabs (battery both sides, power unit right, controller left) and two ladder stiles without rungs; each wheel is one disc; the arms are bars 110 mm deep, the stage pitch, so the stowed stack closes to one mass and the raised scissors read as eight crossing bars with the three pivot tubes of every inner arm; cylinder and rod are two plain rounds; the platform is one slab with corner posts, top and mid rails and 30 mm toe boards; the slide-out deck keeps its plate, its rail frame lies against the main rails, and the controller is a box.",
        "de": "Körper für 30 mm kleinstes Merkmal: das Fahrgestell ist ein Wannenblock mit den Seitenwannen und drei Türplatten (Batterie beidseits, Aggregat rechts, Steuerung links) und zwei Leiterholmen ohne Sprossen; jedes Rad ist eine Scheibe; die Arme sind 110 mm hohe Stäbe, gleich dem Stufenabstand, sodass das eingefahrene Paket zu einer Masse schließt und die gehobene Schere als acht kreuzende Stäbe mit den drei Gelenkrohren jedes Innenarms lesbar ist; Zylinder und Stange sind zwei schlichte Rundstäbe; die Bühne ist eine Platte mit Eckpfosten, Hand- und Knieleisten und 30-mm-Fußleisten; die Ausschubbühne behält ihre Platte, ihr Geländerrahmen liegt am Hauptgeländer an, die Steuerung ist ein Kasten."
    },
    {
        "en": "Thermal: seven regions on the merged masses; the two outlets are thermal-only patches on the front end and on the right battery tray because the body no longer shows them. Structure: solid frame and stack rails with two cross frames, axle bars, disc wheels on four pads; arms as bar pairs of the real 90 x 50 mm outline with one full-width pivot shaft per arm foot; cylinder and rod; platform beams and end members under the slab, corner posts and top rails; runners of the slide-out deck; batteries and platform as masses.",
        "de": "Thermik: sieben Regionen auf den zusammengefassten Massen; die beiden Austritte sind reine Thermik-Flächen an der Stirnseite und auf der rechten Batteriewanne, weil der Körper sie nicht mehr zeigt. Tragwerk: volle Längs- und Scherenträger mit zwei Querrahmen, Achsstäbe, Scheibenräder auf vier Aufstandsflächen; Arme als Stabpaare im realen Umriss 90 x 50 mm mit einer durchgehenden Gelenkwelle je Armfuß; Zylinder und Stange; Bühnenträger und Endträger unter der Platte, Eckpfosten und Handläufe; Läufer der Ausschubbühne; Batterien und Bühne als Massen."
    }
]
