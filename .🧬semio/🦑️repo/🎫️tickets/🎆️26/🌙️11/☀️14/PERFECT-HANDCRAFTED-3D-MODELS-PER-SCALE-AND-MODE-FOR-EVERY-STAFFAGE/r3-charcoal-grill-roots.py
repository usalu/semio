"""🔥 Round 3 charcoal kettle grill: root fixes outside `volumes` (references, dimensions, rig, anchors, poses, clips, structure supports, thermal areas) stated by hand."""
import importlib.util
import os

_spec = importlib.util.spec_from_file_location("kit", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-charcoal-grill-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def text(en, de):
    return {"en": en, "de": de}


REFERENCES = [
    {"id": "weber-original-kettle-57", "citation": "Weber-Stephen Products, Original Kettle charcoal grill 57 cm (22 in) product data (weber.com, retrieved 2026-11-19): lid closed 100.3 cm high, 57.2 cm bowl, 63.5 cm deep with the lid handle, 14.5-15.6 kg; E-5710 and Premium versions 106-107 cm with 5 cm longer legs; three aluminium legs (one on a foot, two carrying the wheel axle), triangle leg support, two 6 in (152 mm) wheels (part 65930), porcelain-enamelled bowl and lid, aluminium dampers, 54.6 cm plated cooking grate."},
]

NOTES = text(
    "Side elevation of a 57 cm kettle grill of the Weber Original Kettle class (lid closed 1.00 m high, 0.585 m bowl, 0.64 m deep with the lid handle, 0.67 m wide over the two side handles; grill 14.5 kg plus a 1.5 kg charcoal charge). The bowl stands on a tripod: one front leg on a foot under the lid handle, two rear legs carrying the axle of two 152 mm wheels, all three braced by a wire triangle; the ash pan hangs from the legs below the bottom damper. The lid is lifted off its rim hook in reality; the rig hinges it at the rear rim to pose open and closed. The vent dial and the bottom damper turn about the vertical (twist of their bones). The fire is 8 kW fuel (about 1 kg charcoal per hour at 29 MJ/kg): 3.6 kW released into the glowing bed, 1.8 kW into the bowl shell, 1.2 kW into the lid shell (closed), 1.4 kW sensible heat in the flue gas (0.0086 m3/s at 220 K), the rest to the food. Bed and grate lie inside the bowl and are hidden in the side elevation.",
    "Seitenansicht eines 57-cm-Kugelgrills der Klasse Weber Original Kettle (Deckel geschlossen 1,00 m hoch, Kessel 0,585 m, 0,64 m tief mit Deckelgriff, 0,67 m breit über die beiden Seitengriffe; Grill 14,5 kg plus 1,5 kg Holzkohle). Der Kessel steht auf einem Dreibein: ein vorderes Bein auf einem Fuß unter dem Deckelgriff, zwei hintere Beine, die die Achse zweier Räder von 152 mm tragen, alle drei durch ein Drahtdreieck ausgesteift; die Aschewanne hängt unter dem Bodenschieber an den Beinen. Der Deckel wird in Wirklichkeit vom Randhaken gehoben; das Rig lenkt ihn am hinteren Rand an, um offen und geschlossen darzustellen. Lüftungsscheibe und Bodenschieber drehen um die Senkrechte (Verdrehung ihrer Knochen). Das Feuer hat 8 kW Brennstoff (etwa 1 kg Holzkohle je Stunde bei 29 MJ/kg): 3,6 kW in die Glut, 1,8 kW in die Kesselschale, 1,2 kW in die Deckelschale (geschlossen), 1,4 kW fühlbare Wärme im Abgas (0,0086 m3/s bei 220 K), der Rest ins Grillgut. Bett und Rost liegen im Kessel und sind in der Seitenansicht verdeckt.",
)

BONES = {
    "bowl": {"lengthM": 0.74, "mass": {"kg": 10.0, "comX": -0.017, "comY": 0.485}},
    "grate": {"x": -K.GRATE_R, "y": K.GRATE_Y, "lengthM": 2 * K.GRATE_R, "mass": {"kg": 1.3, "comX": K.GRATE_R, "comY": 0}},
    "charcoal": {"x": -K.BED_A, "y": K.BED_Y, "lengthM": 2 * K.BED_A, "mass": {"kg": 1.5, "comX": K.BED_A, "comY": 0.018}},
    "lid": {"x": K.LID_X0, "y": K.LID_Y0, "lengthM": 0.585, "mass": {"kg": 3.0, "comX": 0.331, "comY": 0.11}},
    "lid-vent": {"x": 0.2925, "y": round(K.VENT_V, 4), "angleDeg": 90, "lengthM": K.FIN_U1, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 90}, "mass": {"kg": 0.1, "comX": 0.004, "comY": 0}},
    "ash-damper": {"x": 0, "y": round(K.BOWL_BOTTOM, 4), "angleDeg": -90, "lengthM": 0.0045, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 90}, "mass": {"kg": 0.1, "comX": 0.003, "comY": 0.03}},
}

TITLES = {
    "lid-vent": text("Lid vent dial (turns about the vertical)", "Deckel-Lüftungsscheibe (dreht um die Senkrechte)"),
    "ash-damper": text("Bottom air damper (turns about the vertical)", "Unterluft-Schieber (dreht um die Senkrechte)"),
}


def anchors():
    ax = round(K.axle_x(), 4)
    return [
        {"id": "leg-front", "bone": "bowl", "x": K.FOOT_X, "y": 0, "kind": "contact", "title": text("Front foot", "Vorderer Fuß"), "support": "contact", "footprintLengthM": 0.035, "footprintWidthM": 0.035, "frictionCoefficient": 0.5},
        {"id": "wheel-left", "bone": "bowl", "x": ax, "y": 0, "kind": "contact", "title": text("Left wheel", "Linkes Rad"), "support": "contact", "footprintLengthM": 0.03, "footprintWidthM": 0.03, "frictionCoefficient": 0.5, "z": K.WHEEL_Z},
        {"id": "wheel-right", "bone": "bowl", "x": ax, "y": 0, "kind": "contact", "title": text("Right wheel", "Rechtes Rad"), "support": "contact", "footprintLengthM": 0.03, "footprintWidthM": 0.03, "frictionCoefficient": 0.5, "z": -K.WHEEL_Z},
        {"id": "grate-point", "bone": "grate", "x": K.GRATE_R, "y": 0, "kind": "load", "title": text("Food on the grate", "Grillgut auf dem Rost")},
        {"id": "vent-top", "bone": "lid-vent", "x": K.FIN_U1, "y": 0, "kind": "emitter", "title": text("Lid vent opening", "Deckelöffnung")},
        {"id": "bowl-mouth", "bone": "bowl", "x": 0, "y": 0.74, "kind": "emitter", "title": text("Open bowl mouth", "Offene Kesselmündung")},
    ]


CONTACTS = ["leg-front", "wheel-left", "wheel-right"]

STRUCTURE_ASSUMPTIONS = [
    text("A rigid kettle on a tripod with three ground contacts: the front foot at x = +0.30 m and the two wheels at x = -0.143 m, z = +-0.282 m; the raised lid shifts the centre of mass by less than the base triangle.",
         "Ein starrer Kessel auf einem Dreibein mit drei Bodenkontakten: dem vorderen Fuß bei x = +0,30 m und den beiden Rädern bei x = -0,143 m, z = +-0,282 m; der angehobene Deckel verschiebt den Schwerpunkt weniger als das Standdreieck."),
    text("Food load 5 kg on the grate is an authored estimate.", "Die Grillgutlast von 5 kg ist eine Schätzung."),
]

REGION_TITLES = {"legs-steel": text("Tripod legs, sockets, triangle, axle and wheels", "Dreibein, Beinhalter, Dreieck, Achse und Räder")}

AREAS = {}


def apply(asset, areas=None):
    refs = asset["provenance"]["references"]
    for reference in REFERENCES:
        if all(r["id"] != reference["id"] for r in refs):
            refs.append(reference)
    asset["provenance"]["notes"] = NOTES
    asset["dimensions"].update({"heightM": 1.0, "widthM": 0.64, "depthM": 0.67, "massKg": 16.0})
    for bone in asset["rig"]["bones"]:
        fix = BONES.get(bone["id"])
        if fix is None:
            continue
        for key, value in fix.items():
            if key == "mass":
                bone["mass"].update(value)
            else:
                bone[key] = value
        if bone["id"] in TITLES:
            bone["title"] = TITLES[bone["id"]]
        if bone["id"] == "bowl":
            bone["mass"]["reference"] = "weber-original-kettle-57"
    asset["anchors"] = anchors()
    for pose in asset["poses"]:
        pose["contacts"] = list(CONTACTS)
        for bid in ("lid-vent", "ash-damper"):
            if bid in pose["bones"]:
                pose["bones"][bid] = {"twistDeg": pose["bones"][bid]["angleDeg"]} if "angleDeg" in pose["bones"][bid] else pose["bones"][bid]
    for clip in asset["clips"]:
        span = (0, clip["durationS"])
        clip["contacts"] = [{"anchor": a, "fromS": span[0], "toS": span[1]} for a in CONTACTS]
    asset["structure"]["supports"] = list(CONTACTS)
    asset["structure"]["assumptions"] = STRUCTURE_ASSUMPTIONS
    for region in asset["thermal"]["regions"]:
        if region["id"] in REGION_TITLES:
            region["title"] = REGION_TITLES[region["id"]]
        if areas and region["id"] in areas:
            region["areaM2"] = areas[region["id"]]
    return asset
