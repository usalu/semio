"""🌡️ Round 3, appliances/room-thermostat: writes the handcrafted `volumes` source (1:50, 1:100 and the two digital scales 1:200 and 1:500, which have no drawing). Every section, radius and station is stated here by hand.

Usage: python3 r3-room-thermostat-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_room_thermostat_kit", os.path.join(HERE, "r3-room-thermostat-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

H, D = "housing", "dial"
CU = K.CENTRE_U
HOUSING_AT = (CU, 0, 0)
BACK = ["+w"]
BOTH = ["+w", "-w"]
WALLS = ["top", "left", "bottom", "right"]
REBATE = ["+w", "+u", "-u", "+v", "-v"]


def casing(**extra):
    return {"region": "casing", "layer": "shell", **extra}


def knob(**extra):
    return {"region": "dial", "layer": "knob", **extra}


def member(bone, part, **extra):
    return {"member": bone, "part": part, **extra}


def scale_50():
    body = [
        K.loft("housing", H, K.ABS, "body", 0.9, [(-0.0015, 0.0430), (-0.0010, 0.04432), (-0.0003, 0.04483), (0.0005, 0.0450), (0.0104, 0.0450), (0.0105, 0.0435), (0.0135, 0.0435)], 13, CU),
        K.box("display", H, K.LCD, "glazing", 0.12, (0.021, 0.036, 0.002), (K.DISPLAY_U, 0, -0.0008), 0.0005),
        K.ellipsoid("led", H, K.LED, "glazing", 0.35, (K.LED_R, K.LED_R, 0.002), (K.LED_AT[0], K.LED_AT[1], K.W_FACE)),
        K.extrude("dial", D, K.ABS, "body", 0.62, K.fluted(0.0210, 0.0203, 24), K.W_GRIP, K.W_FACE),
        K.extrude("dial-cap", D, K.ABS, "body", 0.62, K.circle(0.0196, 48), K.W_CAP, -0.0055, taper=[0.96, 1]),
        K.extrude("pointer", D, K.ABS, "detail", 0.95, [[0.004, -0.0055], [0.016, -0.003], [0.0178, -0.0018], [0.0185, 0], [0.0178, 0.0018], [0.016, 0.003], [0.004, 0.0055]], K.W_INDEX, -0.006),
    ]
    thermal = [
        K.extrude("th-casing-front", H, K.ABS, "body", 0.9, K.rounded_square(K.HALF, K.CORNER_R, 6), K.W_FACE, K.W_SHELL, holes=[K.rect(0.02773, 0, 0.01988, 0.03558)], taper=[0.9556, 1], at=HOUSING_AT, **casing(sectionFaces=BACK)),
        *[K.extrude(f"th-casing-wall-{name}", H, K.ABS, "body", 0.9, K.wall_panel(side, K.HALF, K.CORNER_R, 0.043, 0.006, 6), K.W_SHELL, K.W_BASE, at=HOUSING_AT, **casing(sectionFaces=[K.INWARD[side]])) for side, name in enumerate(WALLS)],
        K.extrude("th-casing-base", H, K.ABS, "body", 0.9, K.rounded_square(0.0435, 0.0065, 6), K.W_BASE, K.W_WALL, at=HOUSING_AT, **casing(sectionFaces=BOTH)),
        K.box("th-display", H, K.LCD, "glazing", 0.12, (0.021, 0.036, 0.001), (K.DISPLAY_U, 0, -0.0013), region="display", layer="glass", sectionFaces=REBATE),
        K.of("th-led", "led", region="led", layer="lens", sectionFaces=BACK),
        K.of("th-dial", "dial", **knob(sectionFaces=BACK)),
        K.of("th-dial-cap", "dial-cap", **knob()),
        K.of("th-pointer", "pointer", **knob()),
    ]
    structure = [
        K.extrude("member-housing", H, K.ABS, "body", 0.5, K.rounded_square(0.0435, 0.0065, 6), 0.0064, K.W_WALL, at=HOUSING_AT, **member(H, "member")),
        K.extrude("support-wall-screw-top", H, K.ABS, "detail", 0.25, K.circle(0.00375, 24, K.SCREW_PITCH, 0), 0.0045, K.W_WALL, **member(H, "support", anchor="wall-screw-top")),
        K.extrude("support-wall-screw-bottom", H, K.ABS, "detail", 0.25, K.stadium(0, 0, 0.006, 0.00375, 12), 0.0045, K.W_WALL, **member(H, "support", anchor="wall-screw-bottom")),
        K.extrude("member-dial", D, K.ABS, "body", 0.5, K.circle(0.00375, 16), -0.0125, 0.0064, **member(D, "member")),
        K.extrude("joint-dial", H, K.ABS, "detail", 0.35, K.circle(0.006, 24, K.DIAL_U, 0), -0.0006, 0.0064, **member(D, "joint")),
        K.box("mass-housing", H, K.ABS, "detail", 0.15, (0.016, 0.040, 0.007), (CU, 0, 0.0029), 0.001, **member(H, "mass")),
        K.extrude("mass-dial", D, K.ABS, "detail", 0.15, K.circle(0.0065, 24), K.W_GRIP, -0.0025, **member(D, "mass")),
    ]
    decisions = [
        {
            "en": "Body: the housing is one loft of rounded-square sections from the face to the wall: a 2 mm edge radius at the face, the 90 mm cover skirt, a 1.5 mm shadow gap and the 87 mm base plate that lies on the wall. The 36 by 21 mm display lens is let into the face and stands 0.3 mm proud, the 7 mm lamp is a low dome. The knob is a 42 mm grip ring with 24 finger flutes, 8.5 mm high, a slightly conical cap that raises the face to 11.5 mm and the index, a tongue 14.5 mm long that narrows from 11 mm at the hub to a rounded 4 mm tip and stands 0.5 mm proud of the cap, into which it is moulded. Round parts are stated as 48-sided outlines so that they stay round at any zoom.",
            "de": "Körper: Das Gehäuse ist ein Loft aus gerundeten Quadratschnitten von der Front zur Wand: 2 mm Kantenradius an der Front, die 90 mm breite Haube, 1,5 mm Schattenfuge und die 87 mm breite Grundplatte, die an der Wand liegt. Die Anzeigescheibe von 36 mal 21 mm ist in die Front eingelassen und steht 0,3 mm vor, die 7-mm-Lampe ist eine flache Kuppel. Der Knopf besteht aus einem 42 mm breiten Griffring mit 24 Griffmulden von 8,5 mm Höhe, einer leicht konischen Kappe, die die Stirnfläche auf 11,5 mm hebt, und dem Zeiger, einer 14,5 mm langen Zunge, die sich von 11 mm an der Nabe auf eine gerundete Spitze von 4 mm verjüngt und 0,5 mm über die Kappe vorsteht, in die sie eingespritzt ist. Runde Teile sind als 48-eckige Umrisse angegeben, damit sie bei jeder Vergrößerung rund bleiben."
        },
        {
            "en": "Thermal: the casing is the real hollow ABS shell with its 2 mm wall (face plate with the window opening, skirt ring, base plate against the wall), so a cut shows the air space of the electronics; the base plate's wall face is a mounting face and counts for no area. The display is the 1 mm lens in its rebate, the lamp reuses its dome, the dial reuses the three knob solids of the body (grip ring, cap and index) as one closed moulding, since hub and spindle fill its inside; the ring's face on the housing is a mounting face. No region is hidden: the electronics heat the casing region, as the thermal data declares.",
            "de": "Thermik: Das Gehäuse ist die echte hohle ABS-Schale mit 2 mm Wand (Frontplatte mit Fensteröffnung, Haubenring, Grundplatte an der Wand), ein Schnitt zeigt also den Luftraum der Elektronik; die Wandseite der Grundplatte ist Montagefläche und zählt nicht zur Fläche. Die Anzeige ist die 1 mm dicke Scheibe in ihrem Falz, die Lampe verwendet ihre Kuppel, der Drehknopf verwendet die drei Knopfkörper des Körpers (Griffring, Kappe und Zeiger) als ein geschlossenes Formteil, weil Nabe und Achse sein Inneres füllen; die Ringfläche auf dem Gehäuse ist Montagefläche. Keine Region ist verdeckt: Die Elektronik heizt die Gehäuseregion, wie es die Thermikdaten angeben."
        },
        {
            "en": "Structure: the load path is the base plate, which spans between the two fixing screws 60 mm apart (bone housing), the upper screw as a round pin, the lower one in its vertical slot as a roller, both flush with the wall plane. The dial hangs on its 6 mm potentiometer spindle, which turns in the 16 mm potentiometer body behind the face plate (joint). The masses are the circuit board with relay and terminals at the centre of the housing and the knob hub on the spindle. The cover is cladding and is absent.",
            "de": "Tragwerk: Der Lastpfad ist die Grundplatte, die zwischen den beiden Befestigungsschrauben im Abstand von 60 mm spannt (Knochen housing), die obere Schraube als runder Stift, die untere in ihrem senkrechten Langloch als Rollenlager, beide bündig mit der Wandebene. Der Drehknopf hängt an seiner 6-mm-Potentiometerachse, die im 16 mm großen Potentiometergehäuse hinter der Frontplatte dreht (Gelenk). Die Massen sind die Leiterplatte mit Relais und Klemmen in der Gehäusemitte und die Knopfnabe auf der Achse. Die Haube ist Verkleidung und fehlt."
        },
    ]
    return {"status": "authored", "denominator": 50, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_100():
    body = [
        K.extrude("housing", H, K.ABS, "body", 0.9, K.rounded_square(K.HALF, 0.004, 2), K.W_FACE, K.W_WALL, at=HOUSING_AT),
        K.box("display", H, K.LCD, "glazing", 0.12, (0.021, 0.036, 0.003), (K.DISPLAY_U, 0, -0.0003)),
        K.extrude("dial", D, K.ABS, "body", 0.45, K.circle(0.021, 32), K.W_INDEX, 0.0015, taper=[0.94, 1]),
    ]
    thermal = [
        K.extrude("th-casing-front", H, K.ABS, "body", 0.9, K.rounded_square(K.HALF, 0.004, 2), K.W_FACE, K.W_SHELL, at=HOUSING_AT, **casing(sectionFaces=BACK)),
        *[K.extrude(f"th-casing-wall-{name}", H, K.ABS, "body", 0.9, K.wall_panel(side, K.HALF, 0.004, 0.043, 0.002, 2), K.W_SHELL, 0.0115, at=HOUSING_AT, **casing(sectionFaces=[K.INWARD[side]])) for side, name in enumerate(WALLS)],
        K.extrude("th-casing-base", H, K.ABS, "body", 0.9, K.rounded_square(K.HALF, 0.004, 2), 0.0115, K.W_WALL, at=HOUSING_AT, **casing(sectionFaces=BOTH)),
        K.box("th-display", H, K.LCD, "glazing", 0.12, (0.021, 0.036, 0.001), (K.DISPLAY_U, 0, -0.0013), region="display", layer="glass", sectionFaces=REBATE),
        K.ellipsoid("th-led", H, K.LED, "glazing", 0.35, (K.LED_R, K.LED_R, 0.002), (K.LED_AT[0], K.LED_AT[1], K.W_FACE), region="led", layer="lens", sectionFaces=BACK),
        K.of("th-dial", "dial", region="dial", sectionFaces=BACK),
    ]
    structure = [
        K.box("member-housing", H, K.ABS, "body", 0.5, (0.075, 0.015, 0.015), (CU, 0, 0.006), **member(H, "member")),
        K.extrude("support-wall-screw-top", H, K.ABS, "detail", 0.25, K.circle(0.0075, 12, K.SCREW_PITCH, 0), K.W_FACE, K.W_WALL, **member(H, "support", anchor="wall-screw-top")),
        K.extrude("support-wall-screw-bottom", H, K.ABS, "detail", 0.25, K.circle(0.0075, 12, 0, 0), K.W_FACE, K.W_WALL, **member(H, "support", anchor="wall-screw-bottom")),
        K.extrude("member-dial", D, K.ABS, "body", 0.5, K.circle(0.0075, 12), K.W_INDEX, 0.0015, **member(D, "member")),
    ]
    decisions = [
        {
            "en": "Body: three masses at their true size. The housing is a plain 90 mm prism, 15 mm deep, with the corner radius reduced to 4 mm as in the drawing (an 8 mm corner is below the 15 mm feature); the face-edge radius, the shadow gap and the base plate step are merged into it. The display lens stays as a dark plate because 36 by 21 mm is above the feature; the knob is one slightly conical 42 mm disc, 12 mm proud, with lamp, index, flutes and cap step merged.",
            "de": "Körper: drei Massen in wahrer Größe. Das Gehäuse ist ein schlichtes Prisma von 90 mm und 15 mm Tiefe mit auf 4 mm verkleinertem Eckradius wie in der Zeichnung (eine 8-mm-Ecke liegt unter dem Mindestmaß von 15 mm); Kantenradius der Front, Schattenfuge und Stufe der Grundplatte gehen darin auf. Die Anzeigescheibe bleibt als dunkle Platte, weil 36 mal 21 mm über dem Mindestmaß liegen; der Knopf ist eine leicht konische Scheibe von 42 mm, 12 mm vorstehend, in der Lampe, Zeiger, Griffmulden und Kappenstufe aufgehen."
        },
        {
            "en": "Thermal: all four regions keep a solid. The casing stays a hollow 2 mm shell (face plate, skirt ring, base plate against the wall) with square-cut sections, so a cut still shows the air space; the display is its 1 mm lens, the lamp its 7 mm dome at the true size (drawn below the feature, it only marks where the lamp sits), and the dial is the solid knob of the body, its layer merged, with the face on the housing as mounting face.",
            "de": "Thermik: Alle vier Regionen behalten einen Körper. Das Gehäuse bleibt eine hohle 2-mm-Schale (Frontplatte, Haubenring, Grundplatte an der Wand) mit einfachen Schnitten, ein Schnitt zeigt also weiter den Luftraum; die Anzeige ist ihre 1 mm dicke Scheibe, die Lampe ihre 7-mm-Kuppel in wahrer Größe (unter dem Mindestmaß gezeichnet, sie markiert nur den Ort der Lampe), und der Drehknopf ist der massive Knopf des Körpers mit zusammengefasster Schicht und der Fläche am Gehäuse als Montagefläche."
        },
        {
            "en": "Structure: nothing in this 15 mm deep device is thinner than the feature, so the load path is drawn at the feature size: the base plate as a 15 mm bar from screw to screw, both screws as 15 mm round bosses on the screw axis, flush with the wall (pin and slot can no longer be told apart), and the spindle merged with the knob hub into one 15 mm shaft that ends on the bar. Potentiometer, circuit board and the dial's mass are merged into these members.",
            "de": "Tragwerk: Nichts in diesem 15 mm tiefen Gerät ist dünner als das Mindestmaß, der Lastpfad ist deshalb in Mindestmaßgröße gezeichnet: die Grundplatte als 15-mm-Stab von Schraube zu Schraube, beide Schrauben als runde 15-mm-Dome auf der Schraubenachse, bündig mit der Wand (Stift und Langloch sind nicht mehr zu unterscheiden), und die Achse mit der Knopfnabe zu einer 15-mm-Welle zusammengefasst, die am Stab endet. Potentiometer, Leiterplatte und die Masse des Knopfs gehen in diesen Gliedern auf."
        },
    ]
    return {"status": "authored", "denominator": 100, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_200():
    chamfered = [[0.045, 0.039], [0.039, 0.045], [-0.039, 0.045], [-0.045, 0.039], [-0.045, -0.039], [-0.039, -0.045], [0.039, -0.045], [0.045, -0.039]]
    body = [
        K.extrude("housing", H, K.ABS, "body", 0.9, chamfered, K.W_FACE, K.W_WALL, at=HOUSING_AT),
        K.box("display", H, K.LCD, "glazing", 0.12, (0.021, 0.036, 0.002), (K.DISPLAY_U, 0, -0.001)),
        K.extrude("dial", D, K.ABS, "body", 0.45, K.circle(0.021, 16), K.W_INDEX, 0.0),
    ]
    thermal = [
        K.of("th-casing", "housing", region="casing", sectionFaces=BACK),
        K.of("th-display", "display", region="display", sectionFaces=REBATE),
        K.ellipsoid("th-led", H, K.LED, "glazing", 0.35, (K.LED_R, K.LED_R, 0.0015), (K.LED_AT[0], K.LED_AT[1], K.W_FACE), region="led", sectionFaces=BACK),
        K.of("th-dial", "dial", region="dial", sectionFaces=BACK),
    ]
    structure = [
        K.box("member-housing", H, K.ABS, "body", 0.5, (0.075, 0.020, 0.006), (CU, 0, 0.0105), **member(H, "member")),
        K.box("support-wall-screw-top", H, K.ABS, "detail", 0.25, (0.014, 0.014, 0.009), (K.SCREW_PITCH, 0, 0.009), **member(H, "support", anchor="wall-screw-top")),
        K.box("support-wall-screw-bottom", H, K.ABS, "detail", 0.25, (0.014, 0.014, 0.009), (0, 0, 0.009), **member(H, "support", anchor="wall-screw-bottom")),
        K.extrude("member-dial", D, K.ABS, "body", 0.5, K.circle(0.005, 8), -0.0125, 0.0075, **member(D, "member")),
    ]
    decisions = [
        {
            "en": "Digital scale without a drawing (the 0.45 mm object cannot be printed at 1:200). Body: the housing is a 90 mm prism, 15 mm deep, whose 8 mm corner radius is generalised to one 6 mm chamfer per corner; the display stays as a dark plate because it is what tells the face from a blank cover; the knob is a plain 42 mm cylinder, 12 mm proud. Lamp, index, cone of the knob and every edge radius are merged.",
            "de": "Digitaler Maßstab ohne Zeichnung (das 0,45 mm große Objekt ist bei 1:200 nicht druckbar). Körper: Das Gehäuse ist ein Prisma von 90 mm und 15 mm Tiefe, dessen Eckradius von 8 mm zu je einer Fase von 6 mm verallgemeinert ist; die Anzeige bleibt als dunkle Platte, weil sie die Front von einer Blindabdeckung unterscheidet; der Knopf ist ein schlichter Zylinder von 42 mm, 12 mm vorstehend. Lampe, Zeiger, Konus des Knopfs und alle Kantenradien gehen darin auf."
        },
        {
            "en": "Thermal: the 11 mm air space of the casing is below the 30 mm feature, so the casing is the solid housing of the body with its wall face as mounting face; the display is its plate, the dial its cylinder with the face on the housing as mounting face, and the lamp keeps a flat 7 mm dome only to carry its region.",
            "de": "Thermik: Der 11 mm tiefe Luftraum des Gehäuses liegt unter dem Mindestmaß von 30 mm, das Gehäuse ist deshalb der massive Gehäusekörper mit der Wandseite als Montagefläche; die Anzeige ist ihre Platte, der Drehknopf sein Zylinder mit der Fläche am Gehäuse als Montagefläche, und die Lampe behält eine flache 7-mm-Kuppel nur, um ihre Region zu tragen."
        },
        {
            "en": "Structure: the base plate is reduced to its load path, a 20 mm strip at the wall from screw to screw; the two screws are square 14 mm blocks at their anchors, the dial hangs on one 10 mm shaft that reaches back to the strip. Joint and masses are merged into these members.",
            "de": "Tragwerk: Die Grundplatte ist auf ihren Lastpfad verkürzt, einen 20 mm breiten Streifen an der Wand von Schraube zu Schraube; die beiden Schrauben sind quadratische 14-mm-Blöcke an ihren Ankern, der Drehknopf hängt an einer 10-mm-Welle, die bis zum Streifen zurückreicht. Gelenk und Massen gehen in diesen Gliedern auf."
        },
    ]
    return {"status": "authored", "denominator": 200, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def scale_500():
    body = [
        K.box("housing", H, K.ABS, "body", 0.9, (0.09, 0.09, 0.015), (CU, 0, 0.006)),
        K.extrude("dial", D, K.ABS, "body", 0.45, K.circle(0.021, 12), K.W_INDEX, 0.0),
    ]
    thermal = [
        K.of("th-casing", "housing", region="casing", sectionFaces=BACK),
        K.box("th-display", H, K.LCD, "glazing", 0.12, (0.021, 0.036, 0.0005), (K.DISPLAY_U, 0, -0.00175), region="display", sectionFaces=REBATE),
        K.extrude("th-led", H, K.LED, "glazing", 0.35, K.circle(K.LED_R, 8, K.LED_AT[0], K.LED_AT[1]), -0.002, K.W_FACE, region="led", sectionFaces=BACK),
        K.of("th-dial", "dial", region="dial", sectionFaces=BACK),
    ]
    structure = [
        K.box("member-housing", H, K.ABS, "body", 0.5, (0.060, 0.012, 0.012), (CU, 0, 0.0075), **member(H, "member")),
        K.box("support-wall-screw-top", H, K.ABS, "detail", 0.25, (0.012, 0.018, 0.006), (K.SCREW_PITCH, 0, 0.0105), **member(H, "support", anchor="wall-screw-top")),
        K.box("support-wall-screw-bottom", H, K.ABS, "detail", 0.25, (0.012, 0.018, 0.006), (0, 0, 0.0105), **member(H, "support", anchor="wall-screw-bottom")),
        K.box("member-dial", D, K.ABS, "body", 0.5, (0.008, 0.008, 0.014), (0, 0, -0.0055), **member(D, "member")),
    ]
    decisions = [
        {
            "en": "Digital scale without a drawing; the model is minimal. Body: a square 90 mm slab, 15 mm deep, and the round knob as a twelve-sided 42 mm boss, 12 mm proud: square plate plus round knob is what still says thermostat at 0.18 mm. Corners, display, lamp and index are merged into the slab.",
            "de": "Digitaler Maßstab ohne Zeichnung; das Modell ist minimal. Körper: eine quadratische Platte von 90 mm und 15 mm Tiefe und der runde Knopf als zwölfeckiger Buckel von 42 mm, 12 mm vorstehend: quadratische Platte mit rundem Knopf ist das, was bei 0,18 mm noch Thermostat sagt. Ecken, Anzeige, Lampe und Zeiger gehen in der Platte auf."
        },
        {
            "en": "Thermal: the casing is the slab with its wall face as mounting face and the dial is the boss; display and lamp are flat patches on the face, 0.5 mm proud, that only carry their regions.",
            "de": "Thermik: Das Gehäuse ist die Platte mit der Wandseite als Montagefläche und der Drehknopf ist der Buckel; Anzeige und Lampe sind flache Flecken auf der Front, 0,5 mm vorstehend, die nur ihre Regionen tragen."
        },
        {
            "en": "Structure: one 12 mm spine from screw to screw inside the slab, the two screws as flat pads between spine and wall, and the dial on a square 8 mm stub that ends on the spine.",
            "de": "Tragwerk: ein 12 mm starker Steg von Schraube zu Schraube im Inneren der Platte, die beiden Schrauben als flache Auflager zwischen Steg und Wand und der Drehknopf auf einem quadratischen 8-mm-Stummel, der am Steg endet."
        },
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def volumes():
    return {"50": scale_50(), "100": scale_100(), "200": scale_200(), "500": scale_500()}


if __name__ == "__main__":
    path = os.path.join(HERE, "volumes", "appliances", "room-thermostat.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"volumes": volumes()}, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("wrote", path)
