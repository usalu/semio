"""💻 Laptop: the two heat-source slabs of the thermal list are replaced by the real internals in the 6 mm cavity between bottom cover and deck: logic board with the shielded processor, the flat heat pipe to two fin stacks at the exhaust, two blower fans over the intake slots, three battery cells under the palm rest; body, rig, structure and the 2D drawings were judged right and are left as they are."""
import json
import subprocess
import sys

from rework_16a_lib import FACES, T, _INSERT, box, revolve, catalog_path, source_path

SLUG = "laptop"
doc = json.load(open(source_path(SLUG), encoding="utf-8"))
Y = [0, 1, 0]


def inner(solid, region):
    """🔥 Marks an internal part: every face shows in a section."""
    solid.update(region=region, sectionFaces=FACES)
    return solid


def internals_50():
    return [
        inner(box("th-logic-board", "base", "keycap-plastic", [0.070, 0.002, 0.190], [0.062, 0.0062, 0.0], tone=0.3), "deck"),
        inner(box("th-processor", "base", "hinge-steel", [0.030, 0.003, 0.030], [0.058, 0.0087, 0.0], tone=0.75), "deck"),
        inner(box("th-heat-pipe", "base", "hinge-steel", [0.008, 0.003, 0.180], [0.034, 0.0087, 0.0], tone=0.7), "deck"),
        inner(box("th-pipe-bridge", "base", "hinge-steel", [0.022, 0.003, 0.012], [0.047, 0.0087, 0.0], tone=0.7), "deck"),
        inner(box("th-fin-stack", "base", "hinge-steel", [0.010, 0.005, 0.050], [0.018, 0.0075, -0.075], tone=0.6, repeat={"count": 2, "translation": [0.0, 0.0, 0.150]}), "underside"),
        inner(revolve("th-fan", "base", "keycap-plastic", [[0.005, 0.0], [0.005, 0.024], [0.010, 0.024], [0.010, 0.0]], Y, [0.054, 0.0, -0.075], tone=0.2, repeat={"count": 2, "translation": [0.0, 0.0, 0.150]}), "underside"),
        inner(box("th-battery", "base", "keycap-plastic", [0.078, 0.005, 0.086], [0.168, 0.0075, -0.095], tone=0.4, repeat={"count": 3, "translation": [0.0, 0.0, 0.095]}), "underside"),
    ]


def internals_100():
    return [
        inner(box("th-logic-board", "base", "keycap-plastic", [0.070, 0.004, 0.190], [0.062, 0.0075, 0.0], tone=0.3), "deck"),
        inner(revolve("th-fan", "base", "keycap-plastic", [[0.005, 0.0], [0.005, 0.024], [0.010, 0.024], [0.010, 0.0]], Y, [0.054, 0.0, -0.075], tone=0.2, repeat={"count": 2, "translation": [0.0, 0.0, 0.150]}), "underside"),
        inner(box("th-battery", "base", "keycap-plastic", [0.078, 0.005, 0.276], [0.168, 0.0075, 0.0], tone=0.4), "underside"),
    ]


TEXT = {
    "50": ("Thermal internals: in the 6 mm cavity between bottom cover and deck lie the logic board (190 x 70 mm) with the shielded processor, the flat heat pipe (8 x 3 mm) running from it to two fin stacks at the exhaust gap, two 48 mm blower fans over the intake slots and three battery cells of 86 x 78 x 5 mm under the palm rest; board, processor and pipe heat the deck, fans, fins and cells the underside.", "Thermik innen: Im 6 mm hohen Hohlraum zwischen Bodenschale und Deck liegen die Hauptplatine (190 x 70 mm) mit dem abgeschirmten Prozessor, das flache Wärmerohr (8 x 3 mm) von dort zu zwei Lamellenpaketen am Abluftspalt, zwei 48-mm-Radiallüfter über den Ansaugschlitzen und drei Akkuzellen von 86 x 78 x 5 mm unter der Handauflage; Platine, Prozessor und Rohr heizen das Deck, Lüfter, Lamellen und Zellen die Unterseite."),
    "100": ("Thermal internals: the logic board, the two fans over the intake slots and the battery as one pack lie inside the chassis; processor, heat pipe and fin stacks are below what this scale can carry.", "Thermik innen: Hauptplatine, die zwei Lüfter über den Ansaugschlitzen und der Akku als ein Paket liegen im Gehäuse; Prozessor, Wärmerohr und Lamellenpakete liegen unter dem, was dieser Maßstab tragen kann."),
}
for scale, parts in (("50", internals_50()), ("100", internals_100())):
    volume = doc["volumes"][scale]
    old = [s["id"] for s in parts] + ["th-logic-board", "th-battery"]
    keep = [s for s in volume["thermal"] if s["id"] not in old]
    at = next((i for i, s in enumerate(keep) if s["id"] in ("th-deck", "th-underside")), len(keep) - 1) + 1
    volume["thermal"] = keep[:at] + parts + keep[at:]
    volume["decisions"] = [d for d in volume["decisions"] if not d["en"].startswith(("Thermal internals:", "Thermal: the heat sources", "Thermal: at 1:100"))]
    volume["decisions"].append({"en": TEXT[scale][0], "de": TEXT[scale][1]})
with open(source_path(SLUG), "w", encoding="utf-8") as handle:
    handle.write(_INSERT.dump(doc, 0) + "\n")
if "--publish" in sys.argv:
    print(subprocess.run([sys.executable, f"{T}/volumes-insert.py", source_path(SLUG), catalog_path(SLUG)], capture_output=True, text=True).stdout.strip())
