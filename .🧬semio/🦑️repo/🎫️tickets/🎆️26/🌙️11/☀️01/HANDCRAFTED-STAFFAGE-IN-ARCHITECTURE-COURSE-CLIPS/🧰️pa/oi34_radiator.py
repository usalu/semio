"""♨️ pa: OI-34 - the panel radiator asset weighs 26.18 kg; the plastered wall strip stays as context geometry with zero asset mass (the schema has no context flag on a bone) - run once."""
import json

from jio import dumps, locate

path = locate("panel-radiator")
data = json.loads(path.read_text(encoding="utf8"))


def plain(node):
    if isinstance(node, dict):
        return {key: plain(value) for key, value in node.items()}
    if isinstance(node, list):
        return [plain(value) for value in node]
    if isinstance(node, float) and node.is_integer():
        return int(node)
    return node


data = plain(data)
a = data["asset"]
a["dimensions"]["massKg"] = 26.18
wall = next(b for b in a["rig"]["bones"] if b["id"] == "wall")
wall["mass"]["kg"] = 0

pairs = {
    "en": [
        ("The wall stub is a rigid cantilever fixed at the slab; the radiator hangs on two brackets and is carried as a rigid part of it.", "The wall stub is a rigid cantilever fixed at the slab and carries its own weight as part of the building; the radiator hangs on two brackets and is carried as a rigid part of it. The asset mass of 26.18 kg (radiator 26 kg, valve head 0.18 kg) excludes the wall: the bone has no mass because the schema has no context flag for a bone."),
    ],
    "de": [
        ("Das Wandstück ist ein starrer Kragarm, am Boden eingespannt; der Heizkörper hängt an zwei Konsolen und wird als starrer Teil getragen.", "Das Wandstück ist ein starrer Kragarm, am Boden eingespannt, und trägt sein Eigengewicht als Teil des Gebäudes; der Heizkörper hängt an zwei Konsolen und wird als starrer Teil getragen. Die Assetmasse von 26,18 kg (Heizkörper 26 kg, Ventilkopf 0,18 kg) schließt die Wand aus: der Knochen hat keine Masse, weil das Schema kein Kontext-Kennzeichen für einen Knochen kennt."),
    ],
}
for item in a["structure"]["assumptions"]:
    for lang, swaps in pairs.items():
        for old, new in swaps:
            if old in item[lang]:
                item[lang] = item[lang].replace(old, new)

with open(path, "r+b") as handle:
    handle.write(dumps(data).encode("utf8"))
    handle.truncate()
print("ok")
