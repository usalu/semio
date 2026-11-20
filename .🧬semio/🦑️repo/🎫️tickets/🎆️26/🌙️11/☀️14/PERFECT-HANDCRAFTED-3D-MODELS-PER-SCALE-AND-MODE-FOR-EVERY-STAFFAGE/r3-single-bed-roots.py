"""🌱 Round 3, furniture/single-bed: the root corrections outside `rig`, `volumes` and the drawn shapes: the overall width of a bed for a 0.90 m mattress, the anchors at the real post positions, the materials of caps and bolts, the 8 mm slat, references and notes, the texts that name measures, the plan symbol and the decisions of the redrawn representations.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_single_bed_kit", os.path.join(HERE, "r3-single-bed-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

DEPTH = 1.01
AREAS = {"slats": 3.5}

REFERENCES = [
    {"id": "en-1334-1996", "citation": "EN 1334:1996 Domestic furniture - Beds and mattresses - Methods of measurement and recommended tolerances (mattress nominal sizes such as 900 x 2000 mm; the bed's inner length and width exceed the mattress by 10 to 20 mm)."},
    {"id": "ikea-tarva-90x200", "citation": "IKEA, TARVA bed frame, pine, 90x200 cm, article 302.612.70, product measurements: length 209 cm, width 98 cm, headboard height 92 cm, footboard height 32 cm, mattress 90 x 200 cm, slatted base up to 4 cm thick lies hidden in the frame (manufacturer data, read 2026-11-19).", "url": "https://www.ikea.com/lv/en/p/tarva-bed-frame-pine-30261270/"},
    {"id": "slatted-base-90x200", "citation": "Sprung slatted bases 90 x 200 cm with adjustable head section, retail data sheets (28 laminated birch or beech slats about 35-38 mm wide and 8 mm thick in rubber caps, frame rails about 25 x 45 mm, overall height with the slat camber about 80 mm, mass 9-11 kg; read 2026-11-19)."},
]

NOTES = {
    "en": "Pine bed for a 0.90 x 2.00 m mattress: four 60 mm posts (head 0.95 m, foot 0.55 m), frame-and-panel ends, 30 x 160 mm side rails with 25 mm ledgers, a cross beam on a centre leg; overall 2.12 x 1.01 x 0.95 m (clear 2.00 x 0.91 m). The sprung slatted base (base rails 25 x 45 mm, 28 slats 38 x 8 mm in rubber caps, crown 0.285 m) lies on the ledgers; its head frame lifts on two bolts through the base rails at x = 0.78 m, y = 0.23 m, which is the joint of the slat bones. The mattress is 0.20 m thick (top 0.485 m), 22 kg of cold foam in a quilted cover, drawn as three parts on the three slat sections. Masses from nominal volumes. Sleeper and sheets are not drawn.",
    "de": "Kiefernbett für eine Matratze 0,90 x 2,00 m: vier 60-mm-Pfosten (Kopf 0,95 m, Fuß 0,55 m), Kopf- und Fußteil als Rahmen mit Füllung, Seitenholme 30 x 160 mm mit 25-mm-Auflageleisten, ein Mittelsteg auf einem Mittelfuß; außen 2,12 x 1,01 x 0,95 m (licht 2,00 x 0,91 m). Der Federholzrahmen (Holme 25 x 45 mm, 28 Leisten 38 x 8 mm in Kautschukkappen, Scheitel 0,285 m) liegt auf den Auflageleisten; sein Kopfrahmen hebt sich auf zwei Bolzen durch die Holme bei x = 0,78 m, y = 0,23 m, dem Gelenk der Lattenknochen. Die Matratze ist 0,20 m dick (Oberkante 0,485 m), 22 kg Kaltschaum im Steppbezug, gezeichnet als drei Teile auf den drei Rostabschnitten. Massen aus Nennvolumen. Schläfer und Bettwäsche sind nicht gezeichnet.",
}

MATERIALS = [
    {"id": "cap-rubber", "title": {"en": "Slat caps, butadiene rubber", "de": "Leistenkappen, Butadien-Kautschuk"}, "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 980, "conductivityWmK": 0.25, "specificHeatJKgK": 1000, "source": "literature", "reference": "iso-10456-2007"},
    {"id": "steel-fitting", "title": {"en": "Steel bolts and fittings, zinc plated", "de": "Stahlbolzen und Beschläge, verzinkt"}, "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 450, "elasticModulusPa": 210000000000.0, "strengthPa": 235000000.0, "source": "literature", "reference": "iso-10456-2007"},
]

FRAME_ASSUMPTION = {
    "en": "The frame is a rigid body on four foot contacts (the two head posts as one contact across the bed, the centre leg under the cross beam and the left and right foot post at z = +-0.475 m); the slatted base is three sections, the head section on two bolts through the base rails at x = 0.78 m, y = 0.23 m (0 to 65 degrees); the foot section is fixed because a raised foot section would pass within a few centimetres of the footboard; each mattress part rides on its section and sinks along the section normal.",
    "de": "Der Rahmen ist ein starrer Körper auf vier Fußkontakten (die beiden Kopfpfosten als ein Kontakt quer zum Bett, der Mittelfuß unter dem Mittelsteg sowie linker und rechter Fußpfosten bei z = +-0,475 m); der Lattenrost besteht aus drei Teilen, das Kopfteil auf zwei Bolzen durch die Holme bei x = 0,78 m, y = 0,23 m (0 bis 65 Grad); das Fußteil ist fest, weil ein hochgestelltes Fußteil nur wenige Zentimeter am Fußbrett vorbeiliefe; jedes Matratzenteil sitzt auf seinem Teil und sinkt entlang der Teilnormalen ein.",
}

OCCUPANT_ASSUMPTION = {
    "en": "A lying person covers 35 % of the mattress surface (0.63 m2 of the 1.8 m2 top) with a clothed body at 30 degC (ISO 7730 clothing surface; a duvet is not modelled).",
    "de": "Eine liegende Person bedeckt 35 % der Matratzenoberfläche (0,63 m2 der 1,8 m2 Oberseite) mit einem bekleideten Körper bei 30 degC (Bekleidungsoberfläche nach ISO 7730; eine Bettdecke ist nicht modelliert).",
}

TRUE = ("True orthographic views of the {scale} model. ", "Wahre orthogonale Ansichten des Modells {scale}. ")

DECISIONS = {
    "50": {
        "silhouette": ("Bed 42.4 x 19 mm in the side elevation: chamfered posts, side rail, centre leg and the mattress as three parts with rounded free ends; the slat sections with their caps lie behind the rail and show when the head section is raised. Front elevation, seen from the foot end, 20.2 mm wide: foot posts, top rail, boarded panel and bottom rail in front of the head posts, head top rail and boarded head panel. Plan: posts, end rails, side rails and the mattress; ledgers and slats lie under it.", "Bett 42,4 x 19 mm in der Seitenansicht: gefaste Pfosten, Seitenholm, Mittelfuß und die Matratze als drei Teile mit gerundeten freien Enden; die Rostabschnitte mit ihren Kappen liegen hinter dem Holm und zeigen sich, wenn das Kopfteil hochgestellt ist. Vorderansicht vom Fußende, 20,2 mm breit: Fußpfosten, oberer Riegel, Brettfüllung und unterer Riegel vor den Kopfpfosten, dem oberen Kopfriegel und der Kopffüllung. Draufsicht: Pfosten, Querriegel, Seitenholme und die Matratze; Auflageleisten und Leisten liegen darunter."),
        "retained": ("Post chamfers, the V joints of the boarded panels, the slat caps as teeth on the rails, the piped seams of the mattress at top and bottom, grain hatching.", "Pfostenfasen, die V-Fugen der Brettfüllungen, die Leistenkappen als Zähne auf den Holmen, die Kedernähte der Matratze oben und unten, Maserungsschraffur."),
        "merged": ("Rail, caps and slat ends of a section are one outline; the far post, rail and seams coincide with the near ones in the side elevation.", "Holm, Kappen und Leistenenden eines Abschnitts sind ein Umriss; hinterer Pfosten, Holm und Nähte decken sich in der Seitenansicht mit den vorderen."),
        "omitted": ("Bolts, dowels, glides, the slat camber above the caps (5 mm), mattress handles and zip.", "Bolzen, Dübel, Gleiter, die Leistenwölbung über den Kappen (5 mm), Matratzengriffe und Reißverschluss."),
        "thinElements": ("Nothing is thickened: rails 30 mm (0.6 mm), panels 18 mm, caps 50 x 30 mm are filled shapes at their true size; seams and V joints are strokes.", "Nichts ist verdickt: Holme 30 mm (0,6 mm), Füllungen 18 mm, Kappen 50 x 30 mm sind gefüllte Formen in wahrer Größe; Nähte und V-Fugen sind Striche."),
        "hatching": ("Grain lines at 0.6 mm, pen 0.13: vertical on posts, panels and leg, horizontal on rails.", "Maserungslinien im Abstand 0,6 mm, Stift 0,13: senkrecht auf Pfosten, Füllungen und Fuß, waagerecht auf Holmen und Riegeln."),
        "styles": ("Mattress 0.85, slatted base 0.6, rails and panels 0.55, posts 0.5.", "Matratze 0,85, Lattenrost 0,6, Holme und Füllungen 0,55, Pfosten 0,5."),
        "overlays": ("Posts, rails, panels and leg = bed-frame; slat sections = slats (hidden); mattress parts = mattress.", "Pfosten, Holme, Füllungen und Fuß = bed-frame; Rostabschnitte = slats (verdeckt); Matratzenteile = mattress."),
    },
    "100": {
        "silhouette": ("Bed 21.2 x 9.5 mm: plain posts, side rail, centre leg, the mattress as three parts; each slat section is its rail with the slat deck on top, behind the side rail. Front elevation 10.1 mm wide: foot posts, top rail, panel and bottom rail in front of the head end. Plan: posts, end rails, side rails with their ledgers and the mattress.", "Bett 21,2 x 9,5 mm: glatte Pfosten, Seitenholm, Mittelfuß, die Matratze als drei Teile; jeder Rostabschnitt ist sein Holm mit dem Leistendeck darauf, hinter dem Seitenholm. Vorderansicht 10,1 mm breit: Fußpfosten, oberer Riegel, Füllung und unterer Riegel vor dem Kopfteil. Draufsicht: Pfosten, Querriegel, Seitenholme mit ihren Auflageleisten und die Matratze."),
        "retained": ("Frame-and-panel ends with their rails, the lifting head frame, the rounded mattress ends.", "Kopf- und Fußteil als Rahmen mit Füllung samt Riegeln, der aufstellbare Kopfrahmen, die gerundeten Matratzenenden."),
        "merged": ("Slats and caps of a section are one deck; rail and ledger one L section.", "Leisten und Kappen eines Abschnitts sind ein Deck; Holm und Auflageleiste ein L-Profil."),
        "omitted": ("Chamfers, V joints, piping, bolts (below 15 mm).", "Fasen, V-Fugen, Keder, Bolzen (unter 15 mm)."),
        "thinElements": ("Nothing is thickened: panels 18 mm and rails 30 mm are filled shapes at their true size.", "Nichts ist verdickt: Füllungen 18 mm und Holme 30 mm sind gefüllte Formen in wahrer Größe."),
        "hatching": ("None.", "Keine."),
        "styles": ("Mattress 0.85, slatted base 0.6, rails and panels 0.55, posts 0.5.", "Matratze 0,85, Lattenrost 0,6, Holme und Füllungen 0,55, Pfosten 0,5."),
        "overlays": ("Posts, rails, panels and leg = bed-frame; slat sections = slats (hidden); mattress parts = mattress.", "Pfosten, Holme, Füllungen und Fuß = bed-frame; Rostabschnitte = slats (verdeckt); Matratzenteile = mattress."),
    },
    "200": {
        "silhouette": ("Bed 10.6 x 4.8 mm: posts, side rail, centre leg and the mattress; each slat section is one slab behind the rail. Front elevation: foot posts and foot board in front of the head posts and head board. Plan: posts, end boards, side rails and the mattress.", "Bett 10,6 x 4,8 mm: Pfosten, Seitenholm, Mittelfuß und die Matratze; jeder Rostabschnitt ist eine Platte hinter dem Holm. Vorderansicht: Fußpfosten und Fußbrett vor den Kopfpfosten und dem Kopfbrett. Draufsicht: Pfosten, Querbretter, Seitenholme und die Matratze."),
        "retained": ("Posts at their real width of 0.06 m, the centre leg, the rounded mattress ends, the head section that follows its bone.", "Pfosten in Realbreite 0,06 m, der Mittelfuß, die gerundeten Matratzenenden, das Kopfteil, das seinem Knochen folgt."),
        "merged": ("Rails and panel of each end are one board; rails, caps and slats of a section one slab.", "Riegel und Füllung jedes Endes sind ein Brett; Holme, Kappen und Leisten eines Abschnitts eine Platte."),
        "omitted": ("Ledgers, cross beam (hidden), all fittings and seams.", "Auflageleisten, Mittelsteg (verdeckt), alle Beschläge und Nähte."),
        "thinElements": ("Nothing is thickened: side rails and end boards are 30 mm (0.15 mm) at their true size.", "Nichts ist verdickt: Seitenholme und Querbretter sind 30 mm (0,15 mm) in wahrer Größe."),
        "hatching": ("None.", "Keine."),
        "styles": ("Mattress 0.85, slatted base 0.6, rails and boards 0.55, posts 0.5.", "Matratze 0,85, Lattenrost 0,6, Holme und Bretter 0,55, Pfosten 0,5."),
        "overlays": ("Posts, rails, boards and leg = bed-frame; slat slabs = slats (hidden); mattress parts = mattress.", "Pfosten, Holme, Bretter und Fuß = bed-frame; Rostplatten = slats (verdeckt); Matratzenteile = mattress."),
    },
    "500": {
        "silhouette": ("Bed sign 4.2 x 1.9 mm: head panel 0.95 m, foot panel 0.55 m, the tray of rails and slatted base on the centre leg and the mattress in three parts, the head part movable. Front elevation: the foot panel in front of the head panel. Plan: the two panels, the tray and the mattress.", "Bettzeichen 4,2 x 1,9 mm: Kopfplatte 0,95 m, Fußplatte 0,55 m, die Wanne aus Holmen und Lattenrost auf dem Mittelfuß und die Matratze in drei Teilen, das Kopfteil beweglich. Vorderansicht: die Fußplatte vor der Kopfplatte. Draufsicht: die beiden Platten, die Wanne und die Matratze."),
        "retained": ("Head and foot heights, mattress top at 0.485 m, the open space under the bed with the centre leg.", "Kopf- und Fußhöhe, Matratzenoberkante 0,485 m, der offene Raum unter dem Bett mit dem Mittelfuß."),
        "merged": ("Posts, rails and panel of each end are one panel; side rails and slatted base one tray; the slats are bound to the mattress parts.", "Pfosten, Riegel und Füllung jedes Endes sind eine Platte; Seitenholme und Lattenrost eine Wanne; die Latten sind an die Matratzenteile gebunden."),
        "omitted": ("Everything below 75 mm: ledgers, slats, caps, seams, fittings.", "Alles unter 75 mm: Auflageleisten, Leisten, Kappen, Nähte, Beschläge."),
        "thinElements": ("Nothing is thickened: the end panels and the leg are 60 mm (0.12 mm) at their true size and read as their pen line.", "Nichts ist verdickt: die Endplatten und der Fuß sind 60 mm (0,12 mm) in wahrer Größe und lesen sich als ihre Stiftlinie."),
        "hatching": ("None.", "Keine."),
        "styles": ("Mattress 0.85, tray 0.55, panels and leg 0.5.", "Matratze 0,85, Wanne 0,55, Platten und Fuß 0,5."),
        "overlays": ("Panels, tray and leg = bed-frame; mattress parts = mattress and slats.", "Platten, Wanne und Fuß = bed-frame; Matratzenteile = mattress und slats."),
    },
}

SYMBOL_NOTE = ("0.21 m mattress", "0.20 m mattress", "0,21-m-Matratze", "0,20-m-Matratze")


def symbol(sym):
    """🔣 The plan symbol at the corrected true extent."""
    wide, half = DEPTH, DEPTH / 2
    by_id = {view: {s["id"]: s for s in shapes} for view, shapes in sym["geometry"].items()}
    e, f, p = by_id["elevation"], by_id["front"], by_id["plan"]
    e["leg-mid"]["h"] = K.RAIL_Y0
    e["slats"].update({"x": 0.065, "y": K.BASE_Y0, "w": 1.99, "h": 0.08})
    e["mattress"].update({"x": K.MAT_X0, "y": K.MAT_Y0, "w": 2.0, "h": K.MAT_T})
    e["cover-edge"]["points"] = [[0.7, 0.485], [0.7, K.MAT_Y0]]
    f["head"].update({"x": -half, "w": wide})
    f["foot-post-r"].update({"x": -half})
    f["foot-post-l"].update({"x": half - K.POST})
    f["foot-rail"].update({"x": -K.BOARD_Z, "y": 0.45, "w": 2 * K.BOARD_Z, "h": 0.08})
    f["rail-end"].update({"x": -K.BOARD_Z, "w": 2 * K.BOARD_Z})
    f["mattress"].update({"x": -K.MAT_Z, "y": K.MAT_Y0, "w": 2 * K.MAT_Z, "h": K.MAT_T})
    p["plan-frame"].update({"y": -half, "h": wide})
    p["plan-mattress"].update({"x": K.MAT_X0, "y": -K.MAT_Z, "w": 2.0, "h": 2 * K.MAT_Z})
    p["plan-turn"]["points"] = [[0.7, -K.MAT_Z], [0.7, K.MAT_Z]]
    sym["description"] = {"en": sym["description"]["en"].replace(SYMBOL_NOTE[0], SYMBOL_NOTE[1]), "de": sym["description"]["de"].replace(SYMBOL_NOTE[2], SYMBOL_NOTE[3])}


def apply(asset):
    """🧰 Applies every root correction to the asset in memory."""
    asset["dimensions"]["depthM"] = DEPTH
    for anchor in asset["anchors"]:
        if anchor["id"] == "foot-head":
            anchor["footprintWidthM"] = DEPTH
        if anchor["id"] == "foot-foot-l":
            anchor["z"] = K.POST_Z
        if anchor["id"] == "foot-foot-r":
            anchor["z"] = -K.POST_Z
    have = {m["id"] for m in asset["materials"]}
    asset["materials"] += [m for m in MATERIALS if m["id"] not in have]
    for material in asset["materials"]:
        if material["id"] == "birch-slat":
            material["title"] = {"en": "Birch laminated slats 8 mm", "de": "Birkenschichtholz-Federleisten 8 mm"}
    known = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in REFERENCES if r["id"] not in known]
    asset["provenance"]["notes"] = dict(NOTES)
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "slats":
            layer = region["layers"][0]
            layer["title"] = {"en": "Birch slat 8 mm", "de": "Birkenleiste 8 mm"}
            layer.update({"thicknessM": 0.008, "resistanceM2KW": 0.053333, "heatCapacityJM2K": 8320, "reference": "slatted-base-90x200"})
            region["heatCapacityJM2K"] = 8320
    asset["thermal"]["assumptions"][1] = dict(OCCUPANT_ASSUMPTION)
    asset["structure"]["assumptions"][0] = dict(FRAME_ASSUMPTION)
    for sym in asset["symbols"]:
        symbol(sym)
    for scale, texts in DECISIONS.items():
        asset["representations"][scale]["decisions"] = {key: {"en": TRUE[0].format(scale="1:" + scale) + en if key == "silhouette" else en, "de": TRUE[1].format(scale="1:" + scale) + de if key == "silhouette" else de} for key, (en, de) in texts.items()}
