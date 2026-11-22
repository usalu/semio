"""🪔 Pass 1 chisel for `appliances/pendant-lamp`: hand-chosen numbers for the pendulum rig, the E27 lampholder, the coarse scales and their texts.

Usage: python3 rework-pendant-lamp-pass1.py [--candidate] [--root]
Always rewrites the volumes source. `--candidate` writes the whole candidate document to `🗑️generated/r3-pendant-lamp/🔣️.json`, `--root` replaces the members rig, poses, clips, structure and the two rig sentences of the notes in the catalog file (re-read immediately before the write).
"""
import copy
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes/appliances/pendant-lamp.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/pendant-lamp/🔣️.json"
CANDIDATE = os.path.join(T, "🗑️generated/r3-pendant-lamp/🔣️.json")

SWING_DEG = 8
SWING_LIMIT_DEG = 20
PERIOD_S = 1.68

SOCKET_50 = [
    [0.0306, 0.0], [0.0306, 0.012], [0.0318, 0.0158], [0.0340, 0.0183], [0.0372, 0.0195],
    [0.0545, 0.0195], [0.0556, 0.0212], [0.0606, 0.0212], [0.0617, 0.02],
    [0.0926, 0.02], [0.0938, 0.0212], [0.0950, 0.0212], [0.0956, 0.02],
    [0.0956, 0.0142], [0.074, 0.0142], [0.074, 0.0],
]
ROSE_100 = [[0.0, 0.0], [0.0, 0.05], [0.0165, 0.05], [0.024, 0.0425], [0.024, 0.0]]
CORD_100 = [[-0.002, 0.0], [-0.002, 0.0033], [0.602, 0.0033], [0.602, 0.0]]
SHADE_500 = [
    [-0.03, 0.0], [-0.03, 0.0155], [0.0, 0.0155], [0.0, 0.017],
    [0.0037, 0.04255], [0.01606, 0.06805], [0.03896, 0.0919], [0.07429, 0.11246], [0.12393, 0.12812], [0.1385, 0.133],
    [0.1385, 0.129], [0.1251, 0.1243], [0.0759, 0.1088], [0.0414, 0.0887], [0.0193, 0.0657], [0.0075, 0.0414],
    [0.004, 0.017], [0.004, 0.0],
]
PEAR_500 = [[0.026, 0.0138], [0.042, 0.0185], [0.058, 0.027], [0.073, 0.03], [0.088, 0.027], [0.099, 0.018], [0.1035, 0.0094], [0.105, 0.0]]
BULB_500 = [[-0.035, 0.0], [-0.035, 0.02], [0.026, 0.02]] + PEAR_500
TH_BULB_500 = [[0.026, 0.0]] + PEAR_500
TH_LAMPHOLDER_500 = [[-0.0005, 0.0], [-0.0005, 0.0165], [0.03, 0.0165], [0.03, 0.0155], [0.034, 0.0155], [0.034, 0.02], [0.0956, 0.02], [0.0956, 0.0]]
HANGER_100 = {
    "id": "member-rose-hanger", "kind": "capsule", "bone": "rose", "material": "brass-lacquered", "role": "body", "tone": 0.3,
    "p0": [0.0075, 0.0, 0.0], "p1": [0.0245, 0.0, 0.0], "r0": 0.0075, "r1": 0.0075, "member": "rose", "part": "member",
}

TEXTS = {
    ("50", 0): (
        " The lampholder is turned as the real E27 holder: a domed cap on the nipple, the seam bead where cap and skirt screw together and a rolled mouth around the 28.4 mm bore.",
        " Die Fassung ist als echte E27-Fassung gedreht: eine gewölbte Kappe auf dem Nippel, der Nahtwulst, an dem Kappe und Mantel verschraubt sind, und eine gerollte Mündung um die Bohrung von 28,4 mm.",
    ),
    ("50", 1): (
        " The cord joint under the rose is the pivot of the pendulum: the whole chain below it swings as one rigid body.",
        " Das Kabelgelenk unter der Rosette ist der Drehpunkt des Pendels: die ganze Kette darunter schwingt als ein starrer Körper.",
    ),
    ("100", 0): (
        " The cord enters the rose by 2 mm so that it never hangs free under it, and the rose edge is one 7.5 mm chamfer.",
        " Das Kabel greift 2 mm in die Rosette, damit es nie frei darunter hängt, und die Rosettenkante ist eine Fase von 7,5 mm.",
    ),
    ("100", 1): (
        " A 15 mm hanger joins the pin to the cord joint through the rose, so the load path is closed from the hook to the bulb.",
        " Ein 15-mm-Gehänge verbindet den Bolzen durch die Rosette mit dem Kabelgelenk, sodass der Lastweg vom Haken bis zur Lampe geschlossen ist.",
    ),
}
BODY_500 = {
    "en": "Body at 75 mm: a crisp massing of the lamp that keeps its silhouette. The rose is a 100 mm disc with a chamfer, the cord stays at its true 6.6 mm (slender), the shade is the dome of the 1:50 curve at seven stations as a 4 mm shell of even thickness down to the true rim at 0.266 m, without the rolled bead and with its 31 mm neck, so it stays a dome and never turns into a cone; the A60 bulb hangs inside it on a 40 mm lampholder stub, a pear with a round crown whose tip shows 7.5 mm below the rim as at 1:50.",
    "de": "Körper bei 75 mm: eine klare Massenform der Leuchte, die ihren Umriss behält. Die Rosette ist eine Scheibe von 100 mm mit Fase, das Kabel bleibt in seinen wahren 6,6 mm (schlank), der Schirm ist die Kuppel der 1:50-Kurve an sieben Stationen als gleichmäßig 4 mm starke Schale bis zum wahren Rand bei 0,266 m, ohne den gerollten Wulst und mit ihrem 31-mm-Hals, damit er eine Kuppel bleibt und nie ein Kegel wird; die A60-Lampe hängt darin an einem 40-mm-Fassungsstumpf, eine Birne mit runder Kuppe, deren Spitze wie bei 1:50 7,5 mm unter dem Rand hervorschaut.",
}
THERMAL_500 = {
    "en": "Thermal at 75 mm: rose, cord and shade reference their bodies, the lampholder is collar and holder as one turned stub that stands 1 mm proud of the neck and steps back where the shade sheet passes, the bulb glass its simplified envelope, the plume emitter the outlet patch on the shade top around the neck (D2).",
    "de": "Thermik bei 75 mm: Rosette, Kabel und Schirm verweisen auf ihre Körper, die Fassung ist Hülse und Halter als ein gedrehter Stumpf, der 1 mm über den Hals vorsteht und dort zurückspringt, wo das Schirmblech durchläuft, das Lampenglas seine vereinfachte Hülle, der Fahnenemitter das Auslassfeld oben auf dem Schirm um den Hals (D2).",
}

STRUCTURE_ASSUMPTION = {
    "en": "A rigid chain hanging from a pin at the ceiling hook; the 0.62 kg weight is carried in tension by the cord. The cord grip under the rose is the only pivot: pushed, the lamp swings about it as a pendulum of about 1.7 s period, up to 20 degrees to either side; the forces are evaluated quasi-statically in each drawn position.",
    "de": "Eine starre Kette, die an einem Festlager am Deckenhaken hängt; das Gewicht von 0,62 kg trägt das Kabel auf Zug. Die Zugentlastung unter der Rosette ist der einzige Drehpunkt: angestoßen schwingt die Leuchte um ihn als Pendel mit etwa 1,7 s Periode, bis 20 Grad nach jeder Seite; die Kräfte werden in jeder gezeichneten Lage quasistatisch ausgewertet.",
}
NOTE_EN = ("and from the lampholder the shade and the bulb.", " The cord grip under the rose is the pivot about which the lamp swings as a pendulum, up to 20 degrees to either side.")
NOTE_DE = ("und an der Fassung Schirm und Lampe.", " Die Zugentlastung unter der Rosette ist der Drehpunkt, um den die Leuchte als Pendel schwingt, bis 20 Grad nach jeder Seite.")


def solid(items, identifier):
    return next(item for item in items if item["id"] == identifier)


def edit_volumes(volumes):
    solid(volumes["50"]["body"], "socket")["profile"] = SOCKET_50
    solid(volumes["100"]["body"], "rose")["profile"] = ROSE_100
    solid(volumes["100"]["body"], "cord")["profile"] = CORD_100
    structure = volumes["100"]["structure"]
    if not any(item["id"] == "member-rose-hanger" for item in structure):
        structure.insert(1, HANGER_100)
    solid(volumes["500"]["body"], "shade")["profile"] = SHADE_500
    solid(volumes["500"]["body"], "bulb")["profile"] = BULB_500
    solid(volumes["500"]["thermal"], "th-bulb")["profile"] = TH_BULB_500
    solid(volumes["500"]["thermal"], "th-lampholder")["profile"] = TH_LAMPHOLDER_500
    for (scale, index), (en, de) in TEXTS.items():
        text = volumes[scale]["decisions"][index]
        if not text["en"].endswith(en):
            text["en"] += en
            text["de"] += de
    volumes["500"]["decisions"][0] = BODY_500
    volumes["500"]["decisions"][1] = THERMAL_500


def edit_root(asset):
    cord = solid(asset["rig"]["bones"], "cord")
    cord["joint"] = {"type": "revolute", "minDeg": -SWING_LIMIT_DEG, "maxDeg": SWING_LIMIT_DEG}
    lit = solid(asset["poses"], "on")
    poses = [pose for pose in asset["poses"] if pose["id"] not in ("swinging", "swinging-back")]
    for identifier, angle, title, description, key in (
        ("swinging", SWING_DEG, {"en": "Swinging", "de": "Schwingend"}, {
            "en": "The lit lamp at the turning point of a gentle swing, 8 degrees off the plumb line about the cord grip under the rose: cord, collar, shade and bulb stay in one line.",
            "de": "Die brennende Leuchte im Umkehrpunkt einer sanften Schwingung, 8 Grad aus dem Lot um die Zugentlastung unter der Rosette: Kabel, Hülse, Schirm und Lampe bleiben in einer Linie.",
        }, False),
        ("swinging-back", -SWING_DEG, {"en": "Swinging back", "de": "Zurückschwingend"}, {
            "en": "The opposite turning point of the swing, 8 degrees to the other side.",
            "de": "Der gegenüberliegende Umkehrpunkt der Schwingung, 8 Grad zur anderen Seite.",
        }, True),
    ):
        pose = copy.deepcopy(lit)
        pose.update({"id": identifier, "title": title, "description": description, "bones": {"cord": {"angleDeg": angle}}})
        if key:
            pose = {"id": pose["id"], "title": pose["title"], "description": pose["description"], "key": True, **{name: value for name, value in pose.items() if name not in ("id", "title", "description")}}
        poses.append(pose)
    asset["poses"] = poses
    half = PERIOD_S / 2
    clip = {
        "id": "pendulum-swing",
        "title": {"en": "Pendulum swing", "de": "Pendelschwingung"},
        "description": {
            "en": "The lit lamp swings 8 degrees to either side of the plumb line about the cord grip; one full swing of the 0.7 m pendulum takes about 1.7 s.",
            "de": "Die brennende Leuchte schwingt 8 Grad zu jeder Seite des Lots um die Zugentlastung; eine volle Schwingung des 0,7-m-Pendels dauert etwa 1,7 s.",
        },
        "durationS": PERIOD_S, "fps": 25, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "swinging", "ease": "smooth"}, {"t": half, "pose": "swinging-back", "ease": "smooth"}],
        "contacts": [{"anchor": "ceiling-hook", "fromS": 0, "toS": PERIOD_S}],
        "phases": [
            {"id": "forth", "fromS": 0, "toS": half, "title": {"en": "Swinging forth", "de": "Schwingt hin"}},
            {"id": "back", "fromS": half, "toS": PERIOD_S, "title": {"en": "Swinging back", "de": "Schwingt zurück"}},
        ],
        "events": [
            {"t": half / 2, "kind": "motion", "title": {"en": "Through the plumb line", "de": "Durch das Lot"}, "target": "cord"},
            {"t": half * 1.5, "kind": "motion", "title": {"en": "Through the plumb line again", "de": "Wieder durch das Lot"}, "target": "cord"},
        ],
        "tracks": [
            {"target": "operating:power", "ease": "step", "keys": [{"t": 0, "v": 1}]},
            {"target": "emitter:shade-plume", "ease": "step", "keys": [{"t": 0, "v": 1}]},
        ],
    }
    asset["clips"] = [item for item in asset["clips"] if item["id"] != "pendulum-swing"] + [clip]
    asset["structure"]["assumptions"] = [STRUCTURE_ASSUMPTION]
    notes = asset["provenance"]["notes"]
    for language, (anchor, addition) in (("en", NOTE_EN), ("de", NOTE_DE)):
        if addition not in notes[language]:
            assert anchor in notes[language], anchor
            notes[language] = notes[language].replace(anchor, anchor + addition)


def replace_member(text, name, value):
    start = text.index('\n    "%s": ' % name) + 1
    following = re.compile(r',?\n    "[A-Za-z]+": |\n  \}').search(text, start + 8)
    block = json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n    ")
    return text[:start] + '    "%s": ' % name + block + text[following.start():]


source = json.load(open(SOURCE))
edit_volumes(source["volumes"])
open(SOURCE, "w").write(json.dumps(source, ensure_ascii=False, indent=1) + "\n")
print("source written")

if "--candidate" in sys.argv:
    document = json.load(open(CATALOG))
    edit_root(document["asset"])
    document["asset"]["volumes"] = source["volumes"]
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w").write(json.dumps(document, ensure_ascii=False, indent=2) + "\n")
    print("candidate", CANDIDATE)

if "--root" in sys.argv:
    text = open(CATALOG).read()
    asset = json.loads(text)["asset"]
    edit_root(asset)
    for name in ("rig", "poses", "clips", "structure"):
        text = replace_member(text, name, asset[name])
    for anchor, addition in (NOTE_EN, NOTE_DE):
        if addition not in text:
            assert text.count(anchor) == 1, anchor
            text = text.replace(anchor, anchor + addition)
    check = json.loads(text)["asset"]
    assert all(check[name] == asset[name] for name in ("rig", "poses", "clips", "structure", "provenance")), "root write would differ"
    open(CATALOG, "w").write(text)
    print("catalog root written")

ROSE_2D_100 = (
    ("M 0 0.05 L 0.015 0.05 L 0.0215 0.047 L 0.024 0.04 L 0.024 -0.04 L 0.0215 -0.047 L 0.015 -0.05 L 0 -0.05 Z", "M 0 0.05 L 0.0165 0.05 L 0.024 0.0425 L 0.024 -0.0425 L 0.0165 -0.05 L 0 -0.05 Z"),
    ("M 0.05 0.8 L 0.05 0.785 L 0.047 0.7785 L 0.04 0.776 L -0.04 0.776 L -0.047 0.7785 L -0.05 0.785 L -0.05 0.8 Z", "M 0.05 0.8 L 0.05 0.7835 L 0.0425 0.776 L -0.0425 0.776 L -0.05 0.7835 L -0.05 0.8 Z"),
)

if "--draw" in sys.argv:
    text = open(CATALOG).read()
    for old, new in ROSE_2D_100:
        if new not in text:
            assert text.count(old) == 1, old
            text = text.replace(old, new)
    json.loads(text)
    open(CATALOG, "w").write(text)
    print("catalog 2D rose 1:100 redrawn")
