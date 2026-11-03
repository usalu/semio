"""🧱 ap: OI-34 - remove the wall from the panel-radiator asset (bone, stub geometry, region, mass); the brackets stay and the radiator becomes the root. Run once."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "🧰️pa"))
from jio import dumps, load

path, data = load("panel-radiator")
a = data["asset"]
DU, DV = 0.11, -0.035


def conv(point):
    return [round(point[0] - DU, 6), round(point[1] - DV, 6)]


bones = a["rig"]["bones"]
wall = next(b for b in bones if b["id"] == "wall")
radiator = next(b for b in bones if b["id"] == "radiator")
valve = next(b for b in bones if b["id"] == "valve-head")
radiator.update(parent=None, x=0.035, y=0.11, angleDeg=90)
radiator["title"] = {"en": "Type 22 radiator body on its wall brackets (fixed root)", "de": "Kompaktheizkörper Typ 22 an den Wandkonsolen (feste Wurzel)"}
valve.update(parent="radiator", x=round(valve["x"] - DU, 6), y=round(valve["y"] - DV, 6))
a["rig"]["root"] = "radiator"
a["rig"]["bones"] = [radiator, valve]

for anchor in list(a["anchors"]):
    if anchor["id"] == "wall-foot":
        a["anchors"].remove(anchor)
    elif anchor["id"] == "bracket-top":
        x, y = conv([anchor["x"], anchor["y"]])
        anchor.update(bone="radiator", x=x, y=y, kind="support", support="fixed")
        anchor["title"] = {"en": "Upper bracket, fixed to the wall of the scene", "de": "Obere Konsole, an der Wand der Szene befestigt"}

for rep in a["representations"].values():
    geometry = rep.get("geometry")
    if not geometry:
        continue
    elevation = []
    for shape in geometry["elevation"]:
        if shape["id"] in ("wall-stub", "wall-break"):
            continue
        if shape["bone"] == "wall":
            shape["bone"] = "radiator"
            shape["points"] = [conv(p) for p in shape["points"]]
        elevation.append(shape)
    geometry["elevation"] = elevation
    for shape in geometry["front"]:
        if shape["bone"] == "wall":
            shape["bone"] = "radiator"

for rep in a["representations"].values():
    if rep.get("hatches"):
        rep["hatches"] = []
a["materials"] = [m for m in a["materials"] if m["id"] != "masonry"]

for pose in a["poses"]:
    pose["thermal"]["regions"].pop("wall-stub", None)
for clip in a["clips"]:
    if "tracks" in clip:
        clip["tracks"] = [t for t in clip["tracks"] if "wall-stub" not in t["target"]]

a["thermal"]["regions"] = [r for r in a["thermal"]["regions"] if r["id"] != "wall-stub"]
a["thermal"]["assumptions"] = [x for x in a["thermal"]["assumptions"] if not x["en"].startswith("The wall stub is the plastered")]
a["thermal"]["assumptions"].append({
    "en": "The wall is not part of the asset: the scene draws the wall section behind the brackets, and the rear panel exchanges radiation with whatever surface the scene places there.",
    "de": "Die Wand gehört nicht zum Asset: die Szene zeichnet den Wandschnitt hinter den Konsolen, und die Rückplatte tauscht Strahlung mit der Fläche aus, die die Szene dort setzt.",
})

a["structure"]["supports"] = ["bracket-top"]
a["structure"]["assumptions"] = [
    {
        "en": "The wall is not part of the asset: the upper bracket is the fixed support where the asset meets the wall of the scene, and the radiator hangs on its two brackets as one rigid part. The asset mass of 26.18 kg is the radiator 26 kg and the valve head 0.18 kg.",
        "de": "Die Wand gehört nicht zum Asset: die obere Konsole ist das feste Auflager, wo das Asset auf die Wand der Szene trifft, und der Heizkörper hängt als ein starrer Teil an seinen zwei Konsolen. Die Assetmasse von 26,18 kg besteht aus Heizkörper 26 kg und Ventilkopf 0,18 kg.",
    },
    a["structure"]["assumptions"][1],
]
a["structure"]["assumptions"][1] = {
    "en": "The leaning load is an authored estimate, not a normative load.",
    "de": "Die Anlehnlast ist ein Schätzwert, keine Normlast.",
}

SWAPS = [
    ("on two wall brackets 35 mm off a plastered wall", "on two wall brackets that stand 35 mm off the wall of the scene"),
    ("an zwei Wandkonsolen 35 mm vor einer verputzten Wand", "an zwei Wandkonsolen 35 mm vor der Wand der Szene"),
    ("A wall stub and the radiator as one filled panel mass beside it, with the valve head as a dot below.", "The radiator as one filled panel mass with the valve head as a dot below."),
    ("Ein Wandstück und der Heizkörper als eine gefüllte Paneelmasse daneben, darunter der Ventilkopf als Punkt.", "Der Heizkörper als eine gefüllte Paneelmasse, darunter der Ventilkopf als Punkt."),
    ("Wall stub 0.15 m, radiator 0.10 m deep", "Radiator 0.10 m deep"),
    ("Wandstück 0,15 m, Heizkörper 0,10 m tief", "Heizkörper 0,10 m tief"),
    ("Front: fin slits, end seams, pipes (0.1 mm), rear panel, brackets, wall.", "Front: fin slits, end seams, pipes (0.1 mm), rear panel, brackets."),
    ("Vorderansicht: Lamellenschlitze, Stirnnähte, Rohre (0,1 mm), Rückplatte, Konsolen, Wand.", "Vorderansicht: Lamellenschlitze, Stirnnähte, Rohre (0,1 mm), Rückplatte, Konsolen."),
    ("Fin corrugation, grille slots, supply pipe, valve index line, wall break line.", "Fin corrugation, grille slots, supply pipe, valve index line."),
    ("Lamellenwellung, Gitterschlitze, Zuleitung, Ventilmarke, Bruchlinie.", "Lamellenwellung, Gitterschlitze, Zuleitung, Ventilmarke."),
    ("No hatch: the wall stub is a tone fill.", "No hatch: the radiator is a tone fill."),
    ("Keine Schraffur: das Wandstück ist Tonfüllung.", "Keine Schraffur: der Heizkörper ist Tonfüllung."),
    ("Thermal cells: wall stub, radiator mass", "Thermal cells: radiator mass"),
    ("Thermische Zellen: Wandstück, Heizkörpermasse", "Thermische Zellen: Heizkörpermasse"),
    ("The cut wall stub, the radiator as one panel body", "The radiator as one panel body"),
    ("Das geschnittene Wandstück, der Heizkörper als ein Paneelkörper", "Der Heizkörper als ein Paneelkörper"),
    ("Wall stub, panel body, centre split", "Panel body, centre split"),
    ("Wandstück, Paneelkörper, Mittelteilung", "Paneelkörper, Mittelteilung"),
    ("Front: rear panel, brackets, wall.", "Front: rear panel, brackets."),
    ("Vorderansicht: Rückplatte, Konsolen, Wand.", "Vorderansicht: Rückplatte, Konsolen."),
    ("Cut masonry as 45 degree lines at 0.5 mm.", "No hatch: no cut material is drawn."),
    ("Geschnittenes Mauerwerk als 45-Grad-Linien mit 0,5 mm.", "Keine Schraffur: kein geschnittenes Material gezeichnet."),
    ("Wall stub, radiator body (merged panels and convector)", "Radiator body (merged panels and convector)"),
    ("Wandstück, Heizkörperkörper (zusammengefasste Platten und Konvektor)", "Heizkörperkörper (zusammengefasste Platten und Konvektor)"),
    ("hung on two brackets 0.035 m off a cut wall", "hung on two brackets 0.035 m off the wall of the scene"),
    ("an zwei Konsolen 0,035 m vor einer geschnittenen Wand", "an zwei Konsolen 0,035 m vor der Wand der Szene"),
    ("supply pipe, wall break line.", "supply pipe."),
    ("Zuleitung, Wandbruchlinie.", "Zuleitung."),
    ("Front: rear panel, brackets, wall and the fins behind the plate", "Front: rear panel, brackets and the fins behind the plate"),
    ("Vorderansicht: Rückplatte, Konsolen, Wand und die Lamellen hinter der Platte", "Vorderansicht: Rückplatte, Konsolen und die Lamellen hinter der Platte"),
    ("Cut masonry as 45 degree lines at 0.6 mm.", "No hatch: no cut material is drawn."),
    ("Geschnittenes Mauerwerk als 45-Grad-Linien mit 0,6 mm.", "Keine Schraffur: kein geschnittenes Material gezeichnet."),
    ("Outline 0.35 mm for the wall, 0.18 mm for the radiator, 0.1 mm for fins.", "Outline 0.18 mm for the radiator, 0.1 mm for fins."),
    ("Kontur 0,35 mm für die Wand, 0,18 mm für den Heizkörper, 0,1 mm für Lamellen.", "Kontur 0,18 mm für den Heizkörper, 0,1 mm für Lamellen."),
    ("valve, wall stub.", "valve."),
    ("Ventil, Wandstück.", "Ventil."),
]


def walk(node):
    if isinstance(node, dict):
        return {k: walk(v) for k, v in node.items()}
    if isinstance(node, list):
        return [walk(v) for v in node]
    if isinstance(node, str):
        for old, new in SWAPS:
            node = node.replace(old, new)
    return node


data["asset"] = walk(a)
data["asset"]["dimensions"].update(heightM=0.71, widthM=0.135, massKg=26.18)
with open(path, "r+b") as handle:
    handle.write(dumps(data).encode("utf8"))
    handle.truncate()
print("ok")
