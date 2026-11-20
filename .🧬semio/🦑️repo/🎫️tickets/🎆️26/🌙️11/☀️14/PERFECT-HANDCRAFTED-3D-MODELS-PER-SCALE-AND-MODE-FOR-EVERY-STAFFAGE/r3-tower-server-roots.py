"""🖥 Round 3, appliances/tower-server: the root corrections outside `volumes`: the upright rig with its anchors and emitter directions, the references, the corrected areas and texts, and the 2D shapes of every drawn scale as the true orthographic views of the handcrafted body (ruling E1). Elevation shapes are bone-local (x along the upright bone = world y, y = -world x), front shapes are (world z, world y), plan shapes (world x, world z)."""
import importlib.util
import os

_spec = importlib.util.spec_from_file_location("r3_tower_server_kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-tower-server-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
r6 = K.r6

AREAS = {"front-bezel": 0.125}
REFERENCES = [
    {"id": "hpe-ml110-gen10-2020", "citation": "Hewlett Packard Enterprise (2020). HPE ProLiant ML110 Gen10 server, digital data sheet a00021851enw (tower of 44 x 19.5 x 48.05 cm, from 13.5 kg)."},
    {"id": "dell-t150-2021", "citation": "Dell Technologies (2021). Dell EMC PowerEdge T150 Technical Specifications (chassis 360 x 175 x 453.75 mm with bezel, at most 11.68 kg)."},
    {"id": "intel-atx-2-2", "citation": "Intel Corporation (2004). ATX Specification, version 2.2, and ATX12V Power Supply Design Guide, version 2.2 (board 305 x 244 mm, port field 158.75 x 44.45 mm, slot pitch 20.32 mm, power supply 150 x 86 x 140 mm, 120 mm fans)."},
]
NOTE = {
    "en": " Construction (round 3): a riveted chassis of 0.8 mm sheet steel with the mainboard on the fixed wall, a removable cover with a honeycomb field over the tower cooler, an ATX power supply of 150 x 86 x 140 mm at the rear top that draws air through a field in the top panel, a 120 mm fan in the rear panel beside the port field and seven expansion slots, and a clipped ABS bezel with mesh, one 5.25 inch bay, power button and USB sockets (sizes after the HPE ProLiant ML110 Gen10 and Dell PowerEdge T150 class and the ATX specification). The centre of mass follows the component table of the model: chassis 4.9 kg, bezel 0.4 kg, power supply 1.7 kg, mainboard with processor and cooler 1.6 kg, two disk drives 1.5 kg, optical drive 0.7 kg, fans 0.2 kg.",
    "de": " Aufbau (Runde 3): ein genietetes Chassis aus 0,8 mm Stahlblech mit dem Mainboard an der festen Wand, ein abnehmbarer Deckel mit Wabenfeld über dem Turmkühler, ein ATX-Netzteil von 150 x 86 x 140 mm hinten oben, das durch ein Feld im Deckblech ansaugt, ein 120-mm-Lüfter in der Rückwand neben dem Anschlussfeld und sieben Steckplätzen sowie eine geclipste ABS-Blende mit Gitter, einem 5,25-Zoll-Schacht, Taster und USB-Buchsen (Maße nach der Klasse HPE ProLiant ML110 Gen10 und Dell PowerEdge T150 und der ATX-Spezifikation). Der Schwerpunkt folgt der Bauteiltabelle des Modells: Chassis 4,9 kg, Blende 0,4 kg, Netzteil 1,7 kg, Mainboard mit Prozessor und Kühler 1,6 kg, zwei Festplatten 1,5 kg, optisches Laufwerk 0,7 kg, Lüfter 0,2 kg.",
}


def shape(sid, role, z, filled, tone, geometry, **more):
    """🖊 A shape with its common members first."""
    out = {"id": sid, "bone": "case", "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geometry)
    out.update(more)
    return out


def erect(x0, x1, y0, y1):
    """⬜ A world rectangle of the side elevation in the frame of the upright bone."""
    return {"kind": "rect", "x": r6(y0), "y": r6(-x1), "w": r6(y1 - y0), "h": r6(x1 - x0)}


def eline(points):
    """📏 An open run of world (x, y) points of the side elevation."""
    return {"kind": "path", "d": "M " + " L ".join(f"{r6(y):g} {r6(-x):g}" for x, y in points)}


def rect(a0, a1, b0, b1, r=0.0):
    """⬜ A rectangle of the front or the plan."""
    out = {"kind": "rect", "x": r6(a0), "y": r6(b0), "w": r6(a1 - a0), "h": r6(b1 - b0)}
    if r:
        out["r"] = r
    return out


def geometry(scale):
    """🖼 The three views of a scale."""
    steel, absm, rubber = {"material": K.STEEL}, {"material": K.ABS}, {"material": K.RUBBER}
    gx0, gx1, gy0, gy1 = K.GRILLE
    lx0, lx1, ly0, ly1 = K.LATCH
    px0, px1, pz0, pz1 = K.PSU_TOP
    mz0, mz1, my0, my1 = K.MESH
    bz0, bz1, by0, by1 = K.BAY
    if scale == "200":
        merged = ["side-panel", "rear-panel", "front-bezel", "psu-vent", "cpu-grille"]
        return {
            "elevation": [
                shape("skid-front", "body", 3, True, 0.15, erect(0.160, 0.200, 0.0, K.FOOT), region="feet", **rubber),
                shape("skid-rear", "body", 3, True, 0.15, erect(-0.200, -0.160, 0.0, K.FOOT), region="feet", **rubber),
                shape("case", "body", 3.1, True, 0.5, erect(K.REAR, K.FRONT, K.FOOT, K.TOP), regions=merged, **steel),
            ],
            "front": [
                shape("skid", "body", 3, True, 0.15, rect(-0.090, 0.090, 0.0, K.FOOT), region="feet", **rubber),
                shape("case", "body", 3.1, True, 0.4, rect(-K.HALF, K.HALF, K.FOOT, K.TOP), regions=["front-bezel", "side-panel"], **absm),
            ],
            "plan": [shape("top", "body", 3.1, True, 0.5, rect(K.REAR, K.FRONT, -K.HALF, K.HALF), regions=merged, **steel)],
        }
    fine = scale == "50"
    rear = K.PAN if fine else -0.210
    elevation = [
        shape("foot-front", "body", 3, True, 0.15, erect(0.160, 0.200, 0.0, K.FOOT), region="feet", **rubber),
        shape("foot-rear", "body", 3, True, 0.15, erect(-0.200, -0.160, 0.0, K.FOOT), region="feet", **rubber),
        shape("case-side", "body", 3.1, True, 0.55, erect(rear, K.CHASSIS_FRONT, K.FOOT, K.TOP), region="side-panel", **steel),
        shape("rear-panel", "body", 3.2, True, 0.5, erect(K.REAR, rear, K.FOOT, K.TOP), regions=["rear-panel", "psu-vent"], **steel),
        shape("front-bezel", "body", 3.3, True, 0.18, erect(K.CHASSIS_FRONT, K.FRONT, K.FOOT, K.TOP), region="front-bezel", **absm),
        shape("cpu-grille", "body", 3.4, True, 0.38, erect(gx0, gx1, gy0, gy1), region="cpu-grille", **steel),
        shape("latch", "body", 3.5, True, 0.3, erect(lx0, lx1, ly0, ly1), region="side-panel", **steel),
    ]
    if fine:
        elevation.append(shape("top-seam", "detail", 3.6, False, 0.3, eline([(rear, K.TOP_UNDER), (K.CHASSIS_FRONT, K.TOP_UNDER)]), region="side-panel"))
    front = [
        shape("foot-left", "body", 3, True, 0.15, rect(0.060, 0.090, 0.0, K.FOOT), region="feet", **rubber),
        shape("foot-right", "body", 3, True, 0.15, rect(-0.090, -0.060, 0.0, K.FOOT), region="feet", **rubber),
        shape("front-bezel", "body", 3.1, True, 0.18, rect(-K.HALF, K.HALF, K.FOOT, K.TOP, 0.008), region="front-bezel", **absm),
        shape("intake-mesh", "body", 3.3, True, 0.3, rect(mz0, mz1, my0, my1, 0.004 if fine else 0.0), region="front-bezel", **steel),
        shape("drive-bay", "body", 3.4, True, 0.12, rect(bz0, bz1, by0, by1), region="front-bezel", **absm),
    ]
    if fine:
        front += [
            shape("drive-tray", "detail", 3.5, True, 0.3, rect(-0.064, 0.064, 0.398, 0.414), region="front-bezel", **absm),
            shape("power-button", "detail", 3.5, True, 0.7, {"kind": "ellipse", "cx": K.BUTTON[0], "cy": K.BUTTON[1], "rx": 0.007, "ry": 0.007}, region="front-bezel", **absm),
        ]
    plan = [
        shape("top-sheet", "body", 3.1, True, 0.55, rect(rear, K.CHASSIS_FRONT, -K.HALF, K.HALF), region="side-panel", **steel),
        shape("rear-panel", "body", 3.2, True, 0.5, rect(K.REAR, rear, -K.HALF, K.HALF), region="rear-panel", **steel),
        shape("front-bezel", "body", 3.3, True, 0.18, rect(K.CHASSIS_FRONT, K.FRONT, -K.HALF, K.HALF), region="front-bezel", **absm),
        shape("psu-vent", "body", 3.4, True, 0.38, rect(px0, px1, pz0, pz1), region="psu-vent", **steel),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


TEXTS = {
    "50": {
        "silhouette": ("A 0.45 m side view, 9 mm tall on paper, of the side with the removable cover: a steel case on two visible rubber feet, the dark bezel in front (right) and the rim of the rear panel behind (left).", "Eine Seitenansicht von 0,45 m, auf Papier 9 mm hoch, auf die Seite mit dem abnehmbaren Deckel: ein Stahlgehäuse auf zwei sichtbaren Gummifüßen, vorn (rechts) die dunkle Blende und hinten (links) der Rand der Rückwand."),
        "retained": ("Height 0.45 m, depth 0.45 m, the 12 mm feet, the 25 mm bezel, the 12 mm rim of the rear panel, the grille field over the processor cooler, the release latch of the cover and the joint under the top panel; in the front view mesh, drive bay with tray and power button, in the plan the intake field of the power supply.", "Höhe 0,45 m, Tiefe 0,45 m, die 12 mm hohen Füße, die 25 mm tiefe Blende, der 12 mm breite Rand der Rückwand, das Gitterfeld über dem Prozessorkühler, die Entriegelung des Deckels und die Fuge unter dem Deckblech; in der Vorderansicht Gitter, Laufwerksschacht mit Lade und Taster, in der Draufsicht das Ansaugfeld des Netzteils."),
        "merged": ("The two feet at each end are one side-view foot each; the rim of the rear panel stands for the rear panel and for the power supply behind it, which no side view can show.", "Die je zwei Füße an einem Ende sind in der Seitenansicht ein Fuß; der Rand der Rückwand steht für die Rückwand und für das Netzteil dahinter, das keine Seitenansicht zeigen kann."),
        "omitted": ("The honeycomb of the grilles (8 mm cells are 0.16 mm on paper), the USB sockets, and everything on the rear face: fan guard, ports, slots, inlet and switch.", "Die Waben der Gitter (8-mm-Zellen sind auf Papier 0,16 mm), die USB-Buchsen und alles an der Rückseite: Lüftergitter, Anschlüsse, Steckplätze, Netzbuchse und Schalter."),
        "thinElements": ("The joint under the top panel is one detail stroke of 0.13 mm.", "Die Fuge unter dem Deckblech ist ein Detailstrich von 0,13 mm."),
        "styles": ("Case 0.55, rear rim 0.5, bezel 0.18, grille fields 0.38, latch and mesh 0.3; one body pen of 0.18 mm and one detail pen of 0.13 mm.", "Gehäuse 0,55, Rückwandrand 0,5, Blende 0,18, Gitterfelder 0,38, Entriegelung und Gitter 0,3; ein Körperstift von 0,18 mm und ein Detailstift von 0,13 mm."),
        "overlays": ("Side panels, grille, bezel and feet have their own shapes; rear panel and power supply share the rear rim in the side view and have their own shapes in the plan.", "Seitenwände, Gitter, Blende und Füße haben eigene Formen; Rückwand und Netzteil teilen sich in der Seitenansicht den Rückwandrand und haben in der Draufsicht eigene Formen."),
    },
    "100": {
        "silhouette": ("A 0.45 m side view, 4.5 mm tall on paper: the case with the dark bezel in front, the rim of the rear panel and the grille field.", "Eine Seitenansicht von 0,45 m, auf Papier 4,5 mm hoch: das Gehäuse mit der dunklen Blende vorn, dem Rand der Rückwand und dem Gitterfeld."),
        "retained": ("Depth, height, feet, the bezel in its true 25 mm, the rear rim as 15 mm, the grille field and the latch; in the front view mesh and drive bay, in the plan the intake field of the power supply.", "Tiefe, Höhe, Füße, die Blende in ihren wahren 25 mm, der Rückwandrand mit 15 mm, das Gitterfeld und die Entriegelung; in der Vorderansicht Gitter und Laufwerksschacht, in der Draufsicht das Ansaugfeld des Netzteils."),
        "merged": ("Both feet at each end are one block each; the rear rim stands for rear panel and power supply.", "Je zwei Füße an einem Ende sind ein Block; der Rückwandrand steht für Rückwand und Netzteil."),
        "omitted": ("The joint under the top panel, the drive tray, the power button, the USB sockets and all perforation.", "Die Fuge unter dem Deckblech, die Laufwerkslade, der Taster, die USB-Buchsen und alle Lochungen."),
        "thinElements": ("The rear rim is drawn 15 mm wide, the plate thickness of this scale's model.", "Der Rückwandrand ist 15 mm breit gezeichnet, die Plattendicke des Modells dieses Maßstabs."),
        "styles": ("Case 0.55, rear rim 0.5, bezel 0.18, grille fields 0.38, latch and mesh 0.3.", "Gehäuse 0,55, Rückwandrand 0,5, Blende 0,18, Gitterfelder 0,38, Entriegelung und Gitter 0,3."),
        "overlays": ("As at 1:50.", "Wie bei 1:50."),
    },
    "200": {
        "silhouette": ("A block 2.25 mm wide and 2.25 mm high on paper: a case on two low cross skids.", "Ein Block von 2,25 mm Breite und 2,25 mm Höhe auf Papier: ein Gehäuse auf zwei niedrigen Querkufen."),
        "retained": ("Height 0.45 m, depth 0.45 m, width 0.2 m and the feet at their true 12 mm.", "Höhe 0,45 m, Tiefe 0,45 m, Breite 0,2 m und die Füße in ihren wahren 12 mm."),
        "merged": ("Case sheet, bezel, rear panel, power supply and processor grille are one case shape listing all five regions; the two feet of each end are one cross skid.", "Gehäuseblech, Blende, Rückwand, Netzteil und Prozessorgitter sind eine Gehäuseform mit allen fünf Regionen; die zwei Füße jedes Endes sind eine Querkufe."),
        "omitted": ("Bezel step, grille fields, latch and every opening as separate shapes.", "Blendenabsatz, Gitterfelder, Entriegelung und alle Öffnungen als eigene Formen."),
        "thinElements": ("The skids are 12 mm high, 0.06 mm on paper, and are carried by the body pen.", "Die Kufen sind 12 mm hoch, auf Papier 0,06 mm, und werden vom Körperstift getragen."),
        "styles": ("Case 0.5, skids 0.15; one body pen of 0.18 mm.", "Gehäuse 0,5, Kufen 0,15; ein Körperstift von 0,18 mm."),
        "overlays": ("The case shape lists the five case regions as merged regions; the feet are their own region.", "Die Gehäuseform führt die fünf Gehäuseregionen zusammengeführt; die Füße sind eine eigene Region."),
    },
}


def apply(asset):
    """🌱 Writes every root correction into the asset (in memory)."""
    asset["rig"] = K.rig(asset["rig"])
    total, cx, cy = K.centre_of_mass()
    for anchor in asset["anchors"]:
        if anchor["id"] == "rear-outlet":
            anchor.update(x=K.REAR_FAN[1], y=-K.REAR, z=K.REAR_FAN[2])
        if anchor["id"] == "case-top":
            anchor.update(x=K.TOP, y=0.1)
    for emitter in asset["thermal"]["emitters"]:
        emitter["directionDeg"] = {"rear-exhaust": 90, "case-plume": 0}[emitter["id"]]
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    behind = f"{abs(cx):g}".replace(".", ",")
    asset["structure"]["assumptions"] = [{
        "en": f"A rigid {total:g} kg body on four rubber feet; its centre of mass lies {abs(cx):g} m behind the case centre and {cy:g} m above the floor (component table of the model), inside the 0.4 m by 0.17 m footprint.",
        "de": f"Ein starrer Körper von {total:g} kg auf vier Gummifüßen; sein Schwerpunkt liegt {behind} m hinter der Gehäusemitte und {cy:g} m über dem Boden (Bauteiltabelle des Modells), innerhalb der Standfläche von 0,4 m mal 0,17 m.".replace(f"{cy:g} m über", f"{cy:g}".replace(".", ",") + " m über"),
    }]
    have = {reference["id"] for reference in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [reference for reference in REFERENCES if reference["id"] not in have]
    for language in ("en", "de"):
        if NOTE[language] not in asset["provenance"]["notes"][language]:
            asset["provenance"]["notes"][language] += NOTE[language]
    for scale in ("50", "100", "200"):
        rep = asset["representations"][scale]
        rep["geometry"] = geometry(scale)
        for key, (en, de) in TEXTS[scale].items():
            rep["decisions"][key] = {"en": en, "de": de}
