"""🍲 Round 3 cooking pot: the body, thermal and structure lists of every scale, each stated part by part for that scale.

Usage: imported by r3-cooking-pot-build.py (`volumes()`).
"""
import importlib.util
import math
import os

T_DIR = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_cooking_pot_kit", os.path.join(T_DIR, "r3-cooking-pot-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

STEEL = 0.4
GRIP = 0.2
WATER = 0.45
WET = ["-u", "+v", "-v", "+w", "-w"]


def emitters(n, proud, bone="water"):
    """♨️ The two outlets on the water surface (D2): the vapour leaves the outer ring of the surface, the plume rises from the core that the boiling column necks to."""
    y = K.FILL
    return [
        K.disc("th-steam", bone, "water", "detail", 0.9, y, y + proud, K.RI, n, hole=0.04, region="water", layer="water-skin", tags=["steam"]),
        K.disc("th-steam-plume", bone, "water", "detail", 0.95, y, y + 2 * proud, 0.04, n, region="water", layer="water-skin", tags=["steam-plume"]),
    ]


def masses(pot, flat=0.004):
    out = []
    if pot:
        out.append(K.ellipsoid_up("mass-pot", "pot", "stainless", "detail", 0.15, (0.0, 0.024, 0.0), flat, 0.034, member="pot", part="mass", fine=True))
    out.append(K.ellipsoid_up("mass-water", "water", "water", "detail", 0.15, (0.0, 0.03, 0.0), 0.016, 0.016, member="water", part="mass", fine=True))
    return out


def support(n):
    return K.disc("support-base-contact", "pot", "stainless", "ground", 0.1, -0.002, 0.0, K.CAP_R, n, member="pot", part="support", anchor="base-contact", tags=["base-contact"])


def scale_50():
    n = 72
    body = [
        K.disc("base", "pot", "stainless", "body", STEEL, 0.0, K.FLOOR, K.CAP_R, n, r_top=K.EDGE_R),
        K.disc("corner", "pot", "stainless", "body", STEEL, K.FLOOR, K.KNEE_Y, K.EDGE_R, n, hole=K.FOOT_R, r_top=K.RO),
        K.disc("wall", "pot", "stainless", "body", STEEL, K.KNEE_Y, K.LIP_Y, K.RO, n, hole=K.RI),
        K.disc("pouring-rim", "pot", "stainless", "body", STEEL, K.LIP_Y, K.H, K.RO, n, hole=K.RI, r_top=K.RIM),
        K.disc("water-foot", "water", "water", "body", WATER, K.FLOOR + 0.0002, K.KNEE_Y + 0.0002, K.FOOT_R, n, r_top=K.RI),
        K.disc("water", "water", "water", "body", WATER, K.KNEE_Y + 0.0002, K.FILL - K.BOB, K.RI, n),
        K.disc("boil-surface", "surface", "water", "body", 0.55, K.FILL - 2 * K.BOB, K.FILL, K.RI - 0.0002, n),
    ]
    for tag, x, z, r in K.BUBBLES:
        body.append(K.ellipsoid_up("bubble-" + tag, "surface", "water", "detail", 0.9, (x, K.FILL, z), max(0.00375, 0.7 * r), r, fine=True))
    body += [K.bow("bow-left", -1, GRIP, 6), K.bow("bow-right", 1, GRIP, 6)]
    names = [name for name, _ in K.azimuths()]
    for name, az in K.azimuths():
        body.append(K.pad("pad-" + name, az, GRIP))
    for name, az in K.azimuths():
        body.append(K.rivet("rivet-" + name, az, 0.5, False))
    for name, az in K.azimuths():
        body.append(K.rivet("rivet-set-" + name, az, 0.5, True))
    rings = [(K.FLOOR, K.EDGE_R), (K.KNEE_Y, K.RO), (0.05, K.RO), (K.LIP_Y, K.RO), (K.H, K.RIM)]
    thermal = [
        K.of("th-base", "base", region="base", layer="base-skin"),
        K.skin("th-wall", "pot", "stainless", "body", STEEL, rings, region="wall", layer="wall-skin"),
        K.of("th-water-foot", "water-foot", region="water", layer="water-skin", sectionFaces=WET),
        K.of("th-water", "water", region="water", layer="water-skin", sectionFaces=WET),
        K.of("th-boil-surface", "boil-surface", region="water", layer="water-skin", sectionFaces=WET),
        K.of("th-bow-left", "bow-left", region="handle"),
        K.of("th-bow-right", "bow-right", region="handle"),
    ]
    thermal += [K.of("th-pad-" + name, "pad-" + name, region="handle", sectionFaces=["-u" if name.startswith("right") else "+u"]) for name in names]
    thermal += [K.of("th-rivet-" + name, "rivet-" + name, region="handle") for name in names]
    thermal += emitters(n, 0.0006, "surface")
    structure = [
        K.of("member-base-plate", "base", member="pot", part="member"),
        K.of("member-corner", "corner", member="pot", part="member"),
        K.of("member-wall", "wall", member="pot", part="member"),
        K.of("member-rim", "pouring-rim", member="pot", part="member"),
        K.of("member-bow-left", "bow-left", member="handles", part="member"),
        K.of("member-bow-right", "bow-right", member="handles", part="member"),
    ]
    structure += [K.of("joint-pad-" + name, "pad-" + name, member="handles", part="joint") for name in names]
    structure += [K.of("joint-rivet-" + name, "rivet-" + name, member="handles", part="joint") for name in names]
    structure += masses(True) + [support(n)]
    decisions = [
        K.text(
            "Body 1:50, every part of a 16 cm stainless stew pot at its real size: the 5 mm sandwich base (floor and encapsulated disc as one plate with a bevelled edge, 150 mm on the hob), the 8 mm base corner as a conical ring, the 0.8 mm deep-drawn wall, the 45 degree pouring rim out to 172 mm, 1 L of water to 55 mm whose top 8 mm are the boiling surface layer on the surface bone (it bobs by 4 mm into the water body below, so no pose opens a gap) with nine bubbles of 8 to 16 mm, and two loop handles of 6 mm round rod, each forged flat into two 12 x 18 mm pads that an 8 mm rivet holds on the wall (set heads inside the pot). All round parts are 72-sided prisms so that rim, wall and water stay round.",
            "Körper 1:50, jedes Teil eines 16-cm-Edelstahl-Kochtopfs in wahrer Größe: der 5-mm-Sandwichboden (Topfboden und Kapselscheibe als eine Platte mit gefaster Kante, 150 mm auf dem Kochfeld), die 8-mm-Bodenrundung als Kegelring, die 0,8 mm starke tiefgezogene Wand, der 45-Grad-Schüttrand bis 172 mm, 1 l Wasser bis 55 mm, dessen obere 8 mm die kochende Oberflächenschicht am Oberflächenknochen sind (sie wogt um 4 mm in den Wasserkörper darunter, sodass keine Pose eine Lücke öffnet) mit neun Blasen von 8 bis 16 mm, sowie zwei Bügelgriffe aus 6 mm Rundstahl, jeder zu zwei 12 x 18 mm großen Laschen flachgeschmiedet, die je eine 8-mm-Niete an der Wand hält (Schließköpfe im Topfinneren). Alle runden Teile sind 72-seitige Prismen, damit Rand, Wand und Wasser rund bleiben.",
        ),
        K.text(
            "Thermal 1:50: the base is the 5 mm sandwich plate, the wall is the outer skin from the base corner to the lip of the pouring rim (its 0.054 m2 are the one face that exchanges with the room and, wetted to half its height, with the water), the water is the real 1 L body with its surface layer, whose faces against floor and wall count for no area so that the free surface of 0.0201 m2 remains, the handles are the two bows with their pads and rivet domes (the pad faces riveted onto the wall count for no area, the set heads inside belong to the wall), and the two emitters are outlets flush on the bobbing water surface: the steam leaves the outer ring, the plume rises from the 80 mm core.",
            "Thermik 1:50: der Boden ist die 5-mm-Sandwichplatte, die Wand ist die Außenhaut von der Bodenrundung bis zur Kante des Schüttrands (ihre 0,054 m2 sind die eine Fläche, die mit dem Raum und, bis zur halben Höhe benetzt, mit dem Wasser tauscht), das Wasser ist der wirkliche 1-l-Körper mit seiner Oberflächenschicht, dessen Flächen an Boden und Wand nicht zählen, sodass die freie Oberfläche von 0,0201 m2 bleibt, die Griffe sind die zwei Bügel mit Laschen und Nietköpfen (die auf die Wand genieteten Laschenflächen zählen nicht, die Schließköpfe innen gehören zur Wand), und die zwei Emitter sind bündige Auslässe auf der wogenden Wasseroberfläche: der Dampf verlässt den äußeren Ring, die Fahne steigt aus dem 80-mm-Kern.",
        ),
        K.text(
            "Structure 1:50: the vessel is a monocoque, so the load path is the sheet itself: the water (1 kg, magenta ball at its centre of mass) presses on the floor and the sandwich base plate, which stands on the 150 mm contact pad of the hob (green); the wall with its corner ring and the stiffening pouring rim carries its own weight and the handle forces; each bow is a rod member of the handles bone that the two riveted pads (joints) fix to the wall. The pot mass of 0.54 kg is the flat disc at its centre of mass. Water, surface layer, bubbles and the set heads carry nothing and are absent.",
            "Tragwerk 1:50: das Gefäß ist eine Schale, der Lastpfad ist also das Blech selbst: das Wasser (1 kg, magentafarbene Kugel im Schwerpunkt) drückt auf den Boden und die Sandwichbodenplatte, die auf der 150-mm-Aufstandsfläche des Kochfelds steht (grün); die Wand mit ihrem Rundungsring und dem aussteifenden Schüttrand trägt ihr Eigengewicht und die Griffkräfte; jeder Bügel ist ein Stabglied des Griffknochens, das die zwei genieteten Laschen (Gelenke) an der Wand halten. Die Topfmasse von 0,54 kg ist die flache Scheibe in ihrem Schwerpunkt. Wasser, Oberflächenschicht, Blasen und Schließköpfe tragen nichts und fehlen.",
        ),
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    n = 72
    body = [
        K.disc("base", "pot", "stainless", "body", STEEL, 0.0, K.FLOOR, 0.0768, n, r_top=K.RO),
        K.disc("wall", "pot", "stainless", "body", STEEL, K.FLOOR, K.LIP_Y, K.RO, n, hole=K.RI),
        K.disc("pouring-rim", "pot", "stainless", "body", STEEL, K.LIP_Y, K.H, K.RO, n, hole=K.RI, r_top=K.RIM),
        K.disc("water", "water", "water", "body", WATER, K.FLOOR + 0.0002, K.FILL, K.RI - 0.0002, n),
        K.bow("bow-left", -1, GRIP, 3, padded=False),
        K.bow("bow-right", 1, GRIP, 3, padded=False),
    ]
    thermal = [
        K.of("th-base", "base", region="base"),
        K.skin("th-wall", "pot", "stainless", "body", STEEL, [(K.FLOOR, K.RO), (K.LIP_Y, K.RO), (K.H, K.RIM)], region="wall"),
        K.of("th-water", "water", region="water", sectionFaces=WET),
        K.of("th-bow-left", "bow-left", region="handle"),
        K.of("th-bow-right", "bow-right", region="handle"),
    ] + emitters(n, 0.001)
    structure = [
        K.of("member-base-plate", "base", member="pot", part="member"),
        K.of("member-wall", "wall", member="pot", part="member"),
        K.of("member-rim", "pouring-rim", member="pot", part="member"),
        K.of("member-bow-left", "bow-left", member="handles", part="member"),
        K.of("member-bow-right", "bow-right", member="handles", part="member"),
    ] + masses(True, 0.0075) + [support(n)]
    decisions = [
        K.text(
            "Body 1:100, drawn anew: the sandwich base, the floor and the base corner are one 5 mm disc with a chamfered edge, the wall keeps its real 0.8 mm (a hollow part keeps its wall at this scale) and the 45 degree pouring rim its 172 mm, the water is one plain cylinder to 55 mm, and each handle is the bare loop of 6 mm rod with its 20 x 32 mm opening, the bends in three steps; pads, rivets, surface layer and bubbles are below 15 mm and are merged or left out.",
            "Körper 1:100, neu gezeichnet: Sandwichboden, Topfboden und Bodenrundung sind eine 5-mm-Scheibe mit gefaster Kante, die Wand behält ihre wirklichen 0,8 mm (ein Hohlteil behält in diesem Maßstab seine Wand) und der 45-Grad-Schüttrand seine 172 mm, das Wasser ist ein schlichter Zylinder bis 55 mm, und jeder Griff ist der bloße Bügel aus 6 mm Rundstahl mit seiner 20 x 32 mm großen Öffnung, die Bögen in drei Schritten; Laschen, Nieten, Oberflächenschicht und Blasen liegen unter 15 mm und sind zusammengeführt oder weggelassen.",
        ),
        K.text(
            "Thermal 1:100: base disc, water body (wetted faces not counted) and bows by reference, the wall as the outer skin with the rim flare, the steam ring and the plume core as 1 mm outlets on the water. Structure 1:100: base plate, wall shell, rim and the two bows as members, the pot and water masses at their centres of mass, the contact pad under the base.",
            "Thermik 1:100: Bodenscheibe, Wasserkörper (benetzte Flächen nicht gezählt) und Bügel als Verweis, die Wand als Außenhaut mit der Randaufweitung, Dampfring und Fahnenkern als 1-mm-Auslässe auf dem Wasser. Tragwerk 1:100: Bodenplatte, Wandschale, Rand und die zwei Bügel als Glieder, Topf- und Wassermasse in ihren Schwerpunkten, die Aufstandsfläche unter dem Boden.",
        ),
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


LUG_200 = [(0.07787, 0.023), (0.113, 0.025), (0.125, 0.013), (0.125, -0.013), (0.113, -0.025), (0.07787, -0.023), (0.08038, -0.0115), (0.0812, 0.0), (0.08038, 0.0115)]
LUG_500 = [(0.07726, 0.025), (0.125, 0.025), (0.125, -0.025), (0.07726, -0.025), (0.08023, -0.0125), (0.0812, 0.0), (0.08023, 0.0125)]


def scale_200():
    n = 64
    body = [
        K.disc("base", "pot", "stainless", "body", STEEL, 0.0, K.FLOOR, K.RO, n),
        K.disc("wall", "pot", "stainless", "body", STEEL, K.FLOOR, K.LIP_Y, K.RO, n, hole=K.RI),
        K.disc("rim-band", "pot", "stainless", "body", STEEL, K.LIP_Y, K.H, K.RIM, n, hole=K.RI),
        K.disc("water", "water", "water", "body", WATER, K.FLOOR + 0.0002, K.FILL, K.RI - 0.0002, n),
        K.lug("lug-left", -1, GRIP, 0.081, 0.089, LUG_200),
        K.lug("lug-right", 1, GRIP, 0.081, 0.089, LUG_200),
    ]
    thermal = [
        K.of("th-base", "base", region="base"),
        K.skin("th-wall", "pot", "stainless", "body", STEEL, [(K.FLOOR, K.RO), (K.LIP_Y - 0.0001, K.RO), (K.LIP_Y + 0.0001, K.RIM), (K.H, K.RIM)], region="wall"),
        K.of("th-water", "water", region="water", sectionFaces=WET),
        K.of("th-lug-left", "lug-left", region="handle"),
        K.of("th-lug-right", "lug-right", region="handle"),
    ] + emitters(n, 0.002)
    structure = [
        K.of("member-base-plate", "base", member="pot", part="member"),
        K.of("member-wall", "wall", member="pot", part="member"),
        K.of("member-rim", "rim-band", member="pot", part="member"),
        K.of("member-lug-left", "lug-left", member="handles", part="member"),
        K.of("member-lug-right", "lug-right", member="handles", part="member"),
    ] + masses(False) + [support(n)]
    decisions = [
        K.text(
            "Body 1:200, drawn anew: a plain 5 mm base disc, the 0.8 mm wall, the pouring rim generalised to a 5 mm band of 172 mm, the water cylinder, and each loop handle filled to a flat 8 mm lug with clipped corners, because its 20 x 32 mm opening is at the 30 mm print feature.",
            "Körper 1:200, neu gezeichnet: eine schlichte 5-mm-Bodenscheibe, die 0,8-mm-Wand, der Schüttrand zu einem 5-mm-Band von 172 mm verallgemeinert, der Wasserzylinder und jeder Bügelgriff zu einer flachen 8-mm-Lasche mit gekappten Ecken gefüllt, weil seine 20 x 32 mm große Öffnung am Druckmerkmal von 30 mm liegt.",
        ),
        K.text(
            "Thermal 1:200: base, water and lugs by reference, the wall as the outer skin with the step of the rim band, the two outlets 2 mm proud. Structure 1:200: base plate, wall, rim band and the two lugs as members, the water mass and the contact pad; the pot mass is merged into the shell.",
            "Thermik 1:200: Boden, Wasser und Laschen als Verweis, die Wand als Außenhaut mit der Stufe des Randbands, die zwei Auslässe 2 mm erhaben. Tragwerk 1:200: Bodenplatte, Wand, Randband und die zwei Laschen als Glieder, die Wassermasse und die Aufstandsfläche; die Topfmasse geht in der Schale auf.",
        ),
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500():
    n = 48
    body = [
        K.disc("base", "pot", "stainless", "body", STEEL, 0.0, K.FLOOR, K.RO, n),
        K.disc("wall", "pot", "stainless", "body", STEEL, K.FLOOR, K.H, K.RO, n, hole=K.RI),
        K.disc("water", "water", "water", "body", WATER, K.FLOOR + 0.0002, K.FILL, K.RI - 0.0002, n),
        K.lug("lug-left", -1, GRIP, 0.08, 0.09, LUG_500),
        K.lug("lug-right", 1, GRIP, 0.08, 0.09, LUG_500),
    ]
    thermal = [
        K.of("th-base", "base", region="base"),
        K.skin("th-wall", "pot", "stainless", "body", STEEL, [(K.FLOOR, K.RO), (K.H, K.RO)], region="wall"),
        K.of("th-water", "water", region="water", sectionFaces=WET),
        K.of("th-lug-left", "lug-left", region="handle"),
        K.of("th-lug-right", "lug-right", region="handle"),
    ] + emitters(n, 0.0015)
    structure = [
        K.of("member-base-plate", "base", member="pot", part="member"),
        K.of("member-wall", "wall", member="pot", part="member"),
        K.of("member-lug-left", "lug-left", member="handles", part="member"),
        K.of("member-lug-right", "lug-right", member="handles", part="member"),
    ] + masses(False) + [support(n)]
    decisions = [
        K.text(
            "Body 1:500 (digital scale, the pot is a third of a millimetre on paper): the open vessel in five parts, a base disc, the straight 0.8 mm wall (the pouring rim is dropped), the water, and two square 50 x 10 mm lugs for the handles. Thermal: base, water and lugs by reference, the wall skin, two outlets; structure: base plate, wall and lugs as members, the water mass and the contact pad.",
            "Körper 1:500 (digitaler Maßstab, der Topf misst ein Drittel Millimeter auf Papier): das offene Gefäß in fünf Teilen, eine Bodenscheibe, die gerade 0,8-mm-Wand (der Schüttrand entfällt), das Wasser und zwei eckige 50 x 10 mm große Laschen für die Griffe. Thermik: Boden, Wasser und Laschen als Verweis, die Wandhaut, zwei Auslässe; Tragwerk: Bodenplatte, Wand und Laschen als Glieder, die Wassermasse und die Aufstandsfläche.",
        ),
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volumes():
    return {"50": scale_50(), "100": scale_100(), "200": scale_200(), "500": scale_500()}
