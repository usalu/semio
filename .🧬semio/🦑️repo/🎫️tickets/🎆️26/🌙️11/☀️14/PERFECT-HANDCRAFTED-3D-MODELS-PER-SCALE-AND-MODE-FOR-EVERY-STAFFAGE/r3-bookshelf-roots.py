"""🌱 Round 3, furniture/bookshelf: the root corrections outside `volumes`: the steel of the fittings, the rig (shelf heights on the 32 mm raster, bone lengths, carcass centre), the floor contacts under the side panels, areas by hand calculation, texts, and the 2D drawings of every scale and the symbol redrawn as the true views of the bookcase (ruling E1).

Paper frames: elevation = the open front seen from -z (x across the width, shapes bone-local); front = section A-A at mid width seen from +x (x = world z, y = rest height); plan from above (x = world x, y = world z).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_bookshelf_kit", os.path.join(HERE, "r3-bookshelf-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

SHELVES = range(1, 6)
RENAME = {"plinth-left": "foot-left", "plinth-right": "foot-right"}

STEEL = {
    "id": K.STEEL,
    "title": {"en": "Zinc-plated steel fittings (shelf pins, cam fittings)", "de": "Verzinkte Stahlbeschläge (Bodenträger, Exzenterverbinder)"},
    "emissivity": 0.25,
    "solarAbsorptivity": 0.5,
    "densityKgM3": 7800,
    "conductivityWmK": 50,
    "specificHeatJKgK": 450,
    "elasticModulusPa": 210000000000.0,
    "strengthPa": 235000000.0,
    "source": "literature",
    "reference": "iso-10456-2007",
}

REFERENCES = [
    {"id": "ikea-billy-80", "citation": "IKEA BILLY bookcase 80 x 28 x 202 cm and 80 x 28 x 106 cm (product data and assembly instructions): side panels down to the floor, fixed top, bottom and middle shelf on cam-and-bolt fittings, kick board under the bottom, fibreboard back in grooves, adjustable shelves 76 x 26 cm on four steel pins, maximum load 30 kg per shelf; taken as the typical flat-pack construction."},
    {"id": "haefele-fittings", "citation": "Häfele furniture fittings catalogue: Minifix 15 cam connector (15 mm cam, drilling distance 34 mm, connecting bolt 7 mm for 5 mm holes) and plug-in shelf supports of zinc-plated steel for 5 mm holes with collar."},
]

NOTES = {
    "en": "Flat-pack bookcase 0.80 x 0.30 x 1.80 m of 18 mm melamine-faced chipboard: the two side panels stand on the floor; top and bottom (0.06 m up) lie between them on eight cam-and-bolt fittings; a kick board closes the plinth 30 mm behind the front; the 3 mm hardboard back sits in 4 x 8 mm grooves 10 mm from the rear edge. Five shelves 762 x 280 mm lie on four 5 mm steel pins each in System 32 hole rows (37 mm behind the front edge, 224 mm apart), shelf pitch 288 mm = 9 x 32 mm, undersides at 0.352 to 1.504 m. Shelf deflection 5.8 mm computed from beam theory (see structure). Books are loads and not drawn so that the empty and loaded states differ only physically. Masses from the board volumes x 650 kg/m3 (chipboard) and 900 kg/m3 (hardboard).",
    "de": "Zerlegtes Bücherregal 0,80 x 0,30 x 1,80 m aus 18 mm melaminbeschichteter Spanplatte: die zwei Seiten stehen auf dem Boden; Deckel und Boden (0,06 m hoch) liegen dazwischen auf acht Exzenterverbindern; eine Sockelblende schließt den Sockel 30 mm hinter der Front; die 3-mm-Hartfaserrückwand sitzt in 4 x 8 mm Nuten 10 mm vor der Hinterkante. Fünf Fachböden 762 x 280 mm liegen auf je vier 5-mm-Stahlbodenträgern in Lochreihen System 32 (37 mm hinter der Vorderkante, 224 mm Abstand), Bodenabstand 288 mm = 9 x 32 mm, Unterseiten auf 0,352 bis 1,504 m. Fachbodendurchbiegung 5.8 mm aus der Balkentheorie (siehe Tragwerk). Bücher sind Lasten und nicht gezeichnet, damit sich leerer und beladener Zustand nur physikalisch unterscheiden. Massen aus den Plattenvolumen x 650 kg/m3 (Spanplatte) und 900 kg/m3 (Hartfaser).",
}

ASSUMPTION = {
    "en": "The carcass is a rigid body standing on the floor edges of its two side panels (18 x 300 mm each); the hardboard back braces it against racking; EN 14749 shelf tests (uniformly distributed shelf loads) are the scope of the load case.",
    "de": "Der Korpus ist ein starrer Körper, der auf den Bodenkanten seiner zwei Seiten steht (je 18 x 300 mm); die Hartfaserrückwand steift ihn gegen Verschieben aus; die Fachbodenprüfungen nach EN 14749 (gleichmäßig verteilte Last) sind der Rahmen des Lastfalls.",
}

AREAS = {"carcass": 3.26, "shelves": 2.32, "plinth": 0.121}


def renamed(value):
    """🔁 A deep copy with the anchor ids renamed wherever they stand as a whole string."""
    if isinstance(value, dict):
        return {key: renamed(item) for key, item in value.items()}
    if isinstance(value, list):
        return [renamed(item) for item in value]
    if isinstance(value, str):
        for old, new in RENAME.items():
            if value == old or value.endswith(":" + old):
                return value.replace(old, new)
    return value


def rig(old):
    """🦴 The corrected rig: the floor reference carries no length, the carcass bone runs across the 0.80 m width, the shelf carriers sit on the 32 mm raster and travel six holes either way."""
    m = K.masses()
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        parts = bone["id"].split("-")
        if bone["id"] == "ground":
            bone["lengthM"] = 0
        elif bone["id"] == "frame":
            bone["lengthM"] = K.W
            bone["mass"] = dict(bone["mass"], kg=m["frame"][0], comY=m["frame"][1])
        elif len(parts) == 2:
            bone["y"] = K.SHELF_Y[int(parts[1]) - 1]
            bone["joint"] = dict(bone["joint"], minM=-6 * K.PITCH, maxM=6 * K.PITCH)
        bones.append(bone)
    return dict(old, bones=bones)


def anchors():
    """⚓ The two floor contacts: the bottom edge of each side panel."""
    out = []
    for aid, x, en, de in (("foot-left", K.T / 2, "Left side panel on the floor", "Linke Seite auf dem Boden"), ("foot-right", K.W - K.T / 2, "Right side panel on the floor", "Rechte Seite auf dem Boden")):
        out.append({"id": aid, "bone": "frame", "x": x, "y": 0.0, "kind": "contact", "title": {"en": en, "de": de}, "support": "contact", "normalDeg": 90, "footprintLengthM": K.T, "footprintWidthM": K.D, "frictionCoefficient": 0.5})
    return out


def shape(sid, bone, role, z, filled, tone, geometry, **more):
    """✏ A 2D shape in the member order of the catalog file."""
    return dict({"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}, **geometry, **more)


def rect(x0, y0, x1, y1):
    """▭ A rectangle between two corners."""
    return {"kind": "rect", "x": K.r6(x0), "y": K.r6(y0), "w": K.r6(x1 - x0), "h": K.r6(y1 - y0)}


def line(points):
    """➖ An open stroke."""
    return {"kind": "poly", "points": [[K.r6(x), K.r6(y)] for x, y in points], "closed": False}


def board(sid, bone, z, geometry, region, tone=0.7, pen=0.25, material=K.BOARD):
    """🟫 A filled board."""
    more = {"region": region, "material": material}
    if pen:
        more["penMm"] = pen
    return shape(sid, bone, "body", z, True, tone, geometry, **more)


def boards(t, pen, pins):
    """📐 The drawings of a scale that shows every board (1:50, 1:100): boards `t` thick on paper, measured inwards from the true outer faces."""
    elevation = [
        board("back", "frame", 1, rect(t, K.BOTTOM_Y0 + t, K.W - t, K.H - t), "back-panel", 0.9, None, K.HARDBOARD),
        board("side-l", "frame", 6, rect(0, 0, t, K.H), "carcass", pen=pen),
        board("side-r", "frame", 6, rect(K.W - t, 0, K.W, K.H), "carcass", pen=pen),
        board("top", "frame", 6, rect(t, K.H - t, K.W - t, K.H), "carcass", pen=pen),
        board("bottom", "frame", 6, rect(t, K.BOTTOM_Y0, K.W - t, K.BOTTOM_Y0 + t), "carcass", pen=pen),
        board("plinth", "frame", 2, rect(t, 0, K.W - t, K.BOTTOM_Y0), "plinth", 0.4, None),
    ]
    front = [
        board("side", "frame", 1, rect(K.ZF, 0, K.ZB, K.H), "carcass", 0.85, pen),
        board("back", "frame", 2, rect(K.BACK_Z0, K.BOTTOM_Y0 + t - K.GROOVE_DEPTH, K.BACK_Z0 + K.BACK_T, K.H - t + K.GROOVE_DEPTH), "back-panel", 0.9, None, K.HARDBOARD),
        board("top", "frame", 6, rect(K.ZF, K.H - t, K.ZB, K.H), "carcass", pen=pen),
        board("bottom", "frame", 6, rect(K.ZF, K.BOTTOM_Y0, K.ZB, K.BOTTOM_Y0 + t), "carcass", pen=pen),
        board("plinth", "frame", 2, rect(K.KICK_Z0, 0, K.KICK_Z0 + t, K.BOTTOM_Y0), "plinth", 0.4, None),
    ]
    play = K.SHELF_PLAY if pins else 0.0
    x0 = t - K.T + play
    for n in SHELVES:
        y = K.SHELF_Y[n - 1]
        elevation.append(board(f"shelf-{n}-l", f"shelf-{n}-l", 5, rect(x0, 0, K.HALF, t), "shelves", pen=pen))
        elevation.append(board(f"shelf-{n}-r", f"shelf-{n}-r", 5, {"kind": "poly", "points": [[K.HALF, 0.0], [K.r6(x0), 0.0], [K.r6(x0), -t], [K.HALF, -t]], "closed": True}, "shelves", pen=pen))
        front.append(board(f"shelf-{n}", f"shelf-{n}-l", 5, rect(K.SHELF_Z0, y, K.SHELF_Z1, y + t), "shelves", pen=pen))
        if pins:
            steel = {"material": K.STEEL, "penMm": 0.13}
            elevation.append(shape(f"pin-{n}-l", f"shelf-{n}", "detail", 5.5, True, 0.3, rect(0, -2 * K.PIN_R, K.PIN_OUT, 0), **steel))
            elevation.append(shape(f"pin-{n}-r", f"shelf-{n}", "detail", 5.5, True, 0.3, rect(K.SPAN - K.PIN_OUT, -2 * K.PIN_R, K.SPAN, 0), **steel))
            for name, z in (("front", K.PIN_Z0), ("back", K.PIN_Z0 + K.PIN_ROWS)):
                front.append(shape(f"pin-{n}-{name}", f"shelf-{n}", "detail", 5.5, True, 0.3, {"kind": "ellipse", "cx": K.r6(z), "cy": K.r6(y - K.PIN_R), "rx": K.PIN_R, "ry": K.PIN_R}, **steel))
    plan = [
        board("top", "frame", 6, rect(t, K.ZF, K.W - t, K.ZB), "carcass", pen=pen),
        board("side-l", "frame", 6, rect(0, K.ZF, t, K.ZB), "carcass", pen=pen),
        board("side-r", "frame", 6, rect(K.W - t, K.ZF, K.W, K.ZB), "carcass", pen=pen),
    ]
    return {"elevation": elevation, "front": front, "plan": plan}


def sign(t, kick_h, tone, role, pen):
    """🔲 The drawings of a coarse scale (1:200, 1:500): the carcass block, the kick board between the feet and one stroke per shelf on its carrier bone."""
    block = {"regions": ["carcass", "back-panel"], "material": K.BOARD}
    kick = {"region": "plinth", "material": K.BOARD}
    stroke = {"region": "shelves"}
    if pen:
        stroke["penMm"] = pen
    elevation = [shape("body", "frame", "body", 3, True, tone, rect(0, 0, K.W, K.H), **block), shape("plinth", "frame", "body", 4, True, 0.4, rect(t, 0, K.W - t, kick_h), **kick)]
    front = [shape("body", "frame", "body", 3, True, tone, rect(K.ZF, 0, K.ZB, K.H), **block), shape("plinth", "frame", "body", 4, True, 0.4, rect(K.KICK_Z0, 0, K.KICK_Z0 + t, kick_h), **kick)]
    for n in SHELVES:
        y = K.SHELF_Y[n - 1]
        elevation.append(shape(f"shelf-{n}", f"shelf-{n}", role, 5, False, 0.3, line([(t - K.T, t / 2), (K.SPAN - t + K.T, t / 2)]), **stroke))
        front.append(shape(f"shelf-{n}", f"shelf-{n}", role, 5, False, 0.3, line([(K.SHELF_Z0, y + t / 2), (K.ZB - t, y + t / 2)]), **stroke))
    plan = [shape("body", "frame", "body", 3, True, tone, rect(0, K.ZF, K.W, K.ZB), **block)]
    return {"elevation": elevation, "front": front, "plan": plan}


def text(en, de):
    """🗣 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": {
        "silhouette": text("Bookcase 16 x 36 mm with real 18 mm boards: the two side panels stand on the floor, top and bottom lie between them, the kick board closes the plinth, five shelves in two halves each rest on steel pins. Front view: section A-A at mid width looking at the left side panel: the pale side with the cut top, bottom, back panel, kick board and the five shelves, and the two pins of each shelf beyond. Plan: the top panel between the upper ends of the two sides.", "Regal 16 x 36 mm mit echten 18-mm-Platten: die zwei Seiten stehen auf dem Boden, Deckel und Boden liegen dazwischen, die Sockelblende schließt den Sockel, fünf Böden in je zwei Hälften liegen auf Stahlbodenträgern. Vorderansicht: Schnitt A-A in Regalmitte mit Blick auf die linke Seite: die helle Seite mit angeschnittenem Deckel, Boden, Rückwand, Sockelblende und den fünf Fachböden, dahinter die zwei Bodenträger jedes Bodens. Grundriss: der Deckel zwischen den oberen Enden der zwei Seiten."),
        "retained": text("Real board thickness, the 3 mm back 10 mm in front of the rear edge, the kick board 30 mm behind the front, the 288 mm shelf pitch, the pin under each board end that moves with the board when it is re-pinned.", "Echte Plattenstärke, die 3-mm-Rückwand 10 mm vor der Hinterkante, die Sockelblende 30 mm hinter der Front, der Bodenabstand von 288 mm, der Bodenträger unter jedem Brettende, der beim Umstecken mit dem Brett wandert."),
        "merged": text("Edge banding is merged into the boards; the 1 mm play of the shelves is kept.", "Kanten gehen in den Platten auf; die 1 mm Luft der Fachböden bleibt."),
        "omitted": text("The 32 mm hole rows (5 mm holes), the cam fittings, the grooves of the back panel.", "Die 32-mm-Lochreihen (5-mm-Löcher), die Exzenterverbinder, die Nuten der Rückwand."),
        "thinElements": text("Pins 9 x 5 mm (0.18 x 0.1 mm) and the 3 mm back in the section (0.06 mm) are drawn at true size and read through their outline.", "Bodenträger 9 x 5 mm (0,18 x 0,1 mm) und die 3-mm-Rückwand im Schnitt (0,06 mm) sind in wahrer Größe gezeichnet und wirken durch ihre Kontur."),
        "hatching": text("None; laminate is shown by tone.", "Keine; Laminat wird durch Ton gezeigt."),
        "styles": text("Boards 0.7 with 0.25 mm outlines, back 0.9, kick board 0.4, steel pins 0.3.", "Platten 0,7 mit Konturen 0,25 mm, Rückwand 0,9, Sockelblende 0,4, Stahlbodenträger 0,3."),
        "overlays": text("One shape per board, board half and pin; the boards carry carcass, shelves, back-panel and plinth.", "Eine Form je Platte, Bretthälfte und Bodenträger; die Platten tragen carcass, shelves, back-panel und plinth."),
    },
    "100": {
        "silhouette": text("Bookcase 8 x 18 mm: side panels down to the floor, top and bottom between them, back panel, kick board and ten shelf halves. Front view: section A-A at mid width: the pale side panel with the cut top, bottom, back panel and kick board, and the five shelves as boards that ride up and down with their bones. Plan: the top panel between the two sides.", "Regal 8 x 18 mm: Seiten bis zum Boden, Deckel und Boden dazwischen, Rückwand, Sockelblende und zehn Fachbodenhälften. Vorderansicht: Schnitt A-A in Regalmitte: die helle Seite mit angeschnittenem Deckel, Boden, Rückwand und Sockelblende und den fünf Fachböden als Bretter, die mit ihren Knochen auf und ab fahren. Grundriss: der Deckel zwischen den zwei Seiten."),
        "retained": text("Every board as its own filled shape; the halves meet at mid span, where the sag keeps them closed.", "Jedes Brett als eigene gefüllte Form; die Hälften stoßen in Feldmitte, wo die Durchbiegung sie geschlossen hält."),
        "merged": text("Edge banding and the play of the shelves are merged into the boards.", "Kanten und die Luft der Fachböden gehen in den Platten auf."),
        "omitted": text("Shelf pins, cam fittings, hole rows.", "Bodenträger, Exzenterverbinder, Lochreihen."),
        "thinElements": text("Boards of 0.018 m (0.18 mm) and the 3 mm back (0.03 mm) are drawn at true size and read through their 0.25 mm outline.", "Platten von 0,018 m (0,18 mm) und die 3-mm-Rückwand (0,03 mm) sind in wahrer Größe gezeichnet und wirken durch ihre 0,25-mm-Kontur."),
        "hatching": text("None.", "Keine."),
        "styles": text("Boards 0.7, back 0.9, kick board 0.4; board outlines 0.25 mm.", "Platten 0,7, Rückwand 0,9, Sockelblende 0,4; Plattenkonturen 0,25 mm."),
        "overlays": text("One shape per board; shelf halves carry shelves.", "Eine Form je Platte; Fachbodenhälften tragen shelves."),
    },
    "200": {
        "silhouette": text("Bookcase 4 x 9 mm: the carcass block down to the floor with the kick board between its feet and one stroke per shelf. Front view: section A-A as the carcass block with the kick board behind the front and the five shelves as lines at their pin heights. Plan: the carcass as one plate.", "Regal 4 x 9 mm: der Korpusblock bis zum Boden mit der Sockelblende zwischen seinen Füßen und einem Strich je Fachboden. Vorderansicht: Schnitt A-A als Korpusblock mit der Sockelblende hinter der Front und den fünf Fachböden als Linien auf ihren Stifthöhen. Grundriss: der Korpus als eine Fläche."),
        "retained": text("Five boards, each one stroke on its carrier bone that follows the re-pinning.", "Fünf Böden, jeder ein Strich auf seinem Trägerknochen, der dem Umstecken folgt."),
        "merged": text("Carcass boards and back panel are one block; the two halves of a shelf are one stroke (the 6 mm sag is 0.03 mm).", "Korpusplatten und Rückwand sind ein Block; die zwei Hälften eines Bodens sind ein Strich (die 6 mm Durchbiegung sind 0,03 mm)."),
        "omitted": text("Board thickness, pins, fittings.", "Plattenstärke, Bodenträger, Beschläge."),
        "thinElements": text("Boards (0.09 mm) are 0.13 mm detail lines.", "Böden (0,09 mm) sind 0,13-mm-Detaillinien."),
        "hatching": text("None.", "Keine."),
        "styles": text("Block 0.85, kick board 0.4.", "Block 0,85, Sockelblende 0,4."),
        "overlays": text("The block merges carcass and back-panel; strokes carry shelves.", "Der Block führt carcass und back-panel zusammen; Striche tragen shelves."),
    },
    "500": {
        "silhouette": text("Shelf sign 1.6 x 3.6 mm: one carcass block down to the floor, the kick board between its feet and five shelf strokes. Front view: section A-A as one filled block with the kick board behind the front and the five shelves as lines that move with their bone. Plan: the carcass as one plate.", "Regalzeichen 1,6 x 3,6 mm: ein Korpusblock bis zum Boden, die Sockelblende zwischen seinen Füßen und fünf Fachbodenstriche. Vorderansicht: Schnitt A-A als ein gefüllter Block mit der Sockelblende hinter der Front und den fünf Böden als Linien, die mit ihrem Knochen wandern. Grundriss: der Korpus als eine Fläche."),
        "retained": text("Overall height 1.80 m, the rhythm of the five shelves, the kick board.", "Gesamthöhe 1,80 m, der Rhythmus der fünf Böden, die Sockelblende."),
        "merged": text("Sides, top, bottom and back panel are one filled block; each shelf is one stroke.", "Seiten, Deckel, Boden und Rückwand sind ein gefüllter Block; jeder Boden ist ein Strich."),
        "omitted": text("Shelf pins, board thickness, edge banding.", "Bodenträger, Plattenstärke, Kanten."),
        "thinElements": text("The 0.018 m shelves (0.036 mm) are 0.13 mm strokes (0.065 m real) and the kick board is drawn 0.075 m high so that both stay readable.", "Die 0,018 m starken Böden (0,036 mm) sind 0,13-mm-Striche (0,065 m real), und die Sockelblende ist 0,075 m hoch gezeichnet, damit beide lesbar bleiben."),
        "hatching": text("None.", "Keine."),
        "styles": text("Block tone 0.8, kick board 0.4.", "Blockton 0,8, Sockelblende 0,4."),
        "overlays": text("The block lists carcass and back-panel as merged; strokes carry shelves; the sag (0.9 deg, 6 mm) is far below 0.1 mm and invisible.", "Der Block führt carcass und back-panel zusammen; Striche tragen shelves; die Durchbiegung (0,9 Grad, 6 mm) liegt weit unter 0,1 mm und ist unsichtbar."),
    },
}


def symbol(old):
    """🔣 The true-extent symbol redrawn to the construction."""
    g = lambda sid, role, z, filled, tone, geom: shape(sid, "ground", role, z, filled, tone, geom)
    elevation = [
        g("plinth", "body", 1, True, 0.85, rect(K.T, 0, K.W - K.T, K.BOTTOM_Y0)),
        g("side-l", "body", 2, True, 0.85, rect(0, 0, K.T, K.H)),
        g("side-r", "body", 2, True, 0.85, rect(K.W - K.T, 0, K.W, K.H)),
        g("top", "body", 2, True, 0.85, rect(K.T, K.TOP_Y0, K.W - K.T, K.H)),
        g("bottom", "body", 2, True, 0.85, rect(K.T, K.BOTTOM_Y0, K.W - K.T, K.BOTTOM_Y1)),
        g("back", "detail", 1.5, False, 0.5, rect(K.T, K.BOTTOM_Y1, K.W - K.T, K.TOP_Y0)),
    ]
    front = [
        g("side", "body", 2, False, 0.5, rect(K.ZF, 0, K.ZB, K.H)),
        g("plinth", "body", 1, True, 0.85, rect(K.KICK_Z0, 0, K.KICK_Z1, K.BOTTOM_Y0)),
        g("back", "detail", 2.5, True, 0.6, rect(K.BACK_Z0, K.BOTTOM_Y1 - K.GROOVE_DEPTH, K.BACK_Z0 + K.BACK_T, K.TOP_Y0 + K.GROOVE_DEPTH)),
        g("top", "detail", 2.2, False, 0.5, rect(K.ZF, K.TOP_Y0, K.ZB, K.H)),
        g("bottom", "detail", 2.2, False, 0.5, rect(K.ZF, K.BOTTOM_Y0, K.ZB, K.BOTTOM_Y1)),
    ]
    for n in SHELVES:
        y = K.SHELF_Y[n - 1]
        elevation.append(g(f"shelf-{n}", "body", 2.5, True, 0.85, rect(K.T + K.SHELF_PLAY, y, K.W - K.T - K.SHELF_PLAY, y + K.T)))
        front.append(g(f"shelf-{n}", "detail", 2.2, False, 0.5, rect(K.SHELF_Z0, y, K.SHELF_Z1, y + K.T)))
    plan = [
        g("plan-body", "body", 1, True, 0.85, rect(0, K.ZF, K.W, K.ZB)),
        g("plan-joint-l", "detail", 2, False, 0.5, line([(K.T, K.ZF), (K.T, K.ZB)])),
        g("plan-joint-r", "detail", 2, False, 0.5, line([(K.W - K.T, K.ZF), (K.W - K.T, K.ZB)])),
    ]
    out = dict(old)
    out["description"] = text("The bookcase in all three views: side panels 0.30 x 1.80 m down to the floor, top and bottom between them, the kick board 30 mm behind the front, five shelves on a pitch of 0.288 m with their undersides at 0.352 to 1.504 m, the back panel on the left side 10 mm in front of the rear edge.", "Das Regal in allen drei Ansichten: Seiten 0,30 x 1,80 m bis zum Boden, Deckel und Boden dazwischen, die Sockelblende 30 mm hinter der Front, fünf Böden im Abstand von 0,288 m mit den Unterseiten auf 0,352 bis 1,504 m, die Rückwand auf der linken Seite 10 mm vor der Hinterkante.")
    out["geometry"] = {"elevation": elevation, "front": front, "plan": plan}
    return out


def apply(asset):
    """🌱 Writes the root corrections into the asset in memory."""
    if not any(m["id"] == K.STEEL for m in asset["materials"]):
        asset["materials"] = asset["materials"] + [STEEL]
    asset["rig"] = rig(asset["rig"])
    asset["anchors"] = anchors()
    asset["poses"] = renamed(asset["poses"])
    asset["clips"] = renamed(asset["clips"])
    asset["structure"] = renamed(asset["structure"])
    asset["structure"]["assumptions"][2] = ASSUMPTION
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "plinth":
            region["title"] = text("Kick board of the plinth", "Sockelblende")
    prov = asset["provenance"]
    known = {r["id"] for r in prov["references"]}
    prov["references"] = prov["references"] + [r for r in REFERENCES if r["id"] not in known]
    prov["notes"] = NOTES
    reps = asset["representations"]
    drawn = {"50": boards(K.T, 0.25, True), "100": boards(K.T, 0.25, False), "200": sign(0.03, K.BOTTOM_Y0, 0.85, "detail", None), "500": sign(0.075, 0.075, 0.8, "body", 0.13)}
    for scale, geometry in drawn.items():
        reps[scale]["geometry"] = geometry
        reps[scale]["decisions"] = DECISIONS[scale]
    asset["symbols"] = [symbol(asset["symbols"][0])] + asset["symbols"][1:]
