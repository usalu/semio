"""🍲 Pot thermal budget: parameters from argv as JSON (water W, base W, wall W, boil level, boil time, vapour ratio)."""
import json
import sys
from pathlib import Path

F = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json"
P = json.loads(sys.argv[1])
doc = json.loads(F.read_text(encoding="utf-8"))
a = doc["asset"]
regs = {r["id"]: r for r in a["thermal"]["regions"]}
cap = round(P["massKg"] * 4186 / P["areaM2"])

w = regs["water"]
w["areaM2"] = P["areaM2"]
w["heatCapacityJM2K"] = cap
w["layers"][0]["heatCapacityJM2K"] = cap
w["layers"][0]["thicknessM"] = 0.057
w["layers"][0]["source"] = "authored-estimate"
w["source"] = "authored-estimate"
w["operating"]["heatW"] = P["waterW"]
regs["base"]["operating"]["heatW"] = P["baseW"]
regs["wall"]["operating"]["heatW"] = P["wallW"]

lv = a["thermal"]["operating"][0]["levels"]
lv[:] = [
    {"v": 0, "title": {"en": "Cold", "de": "Kalt"}},
    {"v": P["boilLevel"], "title": {"en": "Boiling (evaporation)", "de": "Sieden (Verdunstung)"}},
    {"v": 1, "title": {"en": "Heating at 1.2 kW", "de": "Heizt mit 1,2 kW"}},
]
a["thermal"]["emitters"][0]["vapourKgPerKg"] = P["vapour"]

tb = P["boilS"]
T = P["durationS"]
clip = a["clips"][0]
clip["durationS"] = T
clip["description"] = {
    "en": f"The cold water warms for about {round(tb / 60)} minutes at 1.2 kW until it boils; then the burner heat leaves as steam.",
    "de": f"Das kalte Wasser erwärmt sich bei 1,2 kW etwa {round(tb / 60)} Minuten lang, bis es kocht; dann verlässt die Brennerwärme als Dampf den Topf.",
}
clip["keys"] = [
    {"t": 0, "pose": "cold", "ease": "step"},
    {"t": 1, "pose": "heating", "ease": "linear"},
    {"t": tb, "pose": "heating", "ease": "linear"},
    {"t": tb + P["rampS"], "pose": "boiling", "ease": "step"},
    {"t": T, "pose": "boiling", "ease": "step"},
]
clip["contacts"] = [{"anchor": "base-contact", "fromS": 0, "toS": T}]
clip["phases"] = [
    {"id": "off", "fromS": 0, "toS": 1, "title": {"en": "Switched off", "de": "Aus"}},
    {"id": "warm", "fromS": 1, "toS": round(tb * 0.5), "title": {"en": "Warming", "de": "Erwärmen"}},
    {"id": "near", "fromS": round(tb * 0.5), "toS": tb, "title": {"en": "Nearly boiling", "de": "Fast kochend"}},
    {"id": "boil", "fromS": tb, "toS": T, "title": {"en": "Boiling", "de": "Kochen"}},
]
clip["events"] = [
    {"t": 1, "kind": "ignition", "title": {"en": "Burner lit", "de": "Brenner an"}, "target": "heat"},
    {"t": tb, "kind": "operating", "title": {"en": "Boiling starts", "de": "Sieden beginnt"}, "target": "heat"},
]
clip["tracks"] = [
    {"target": "operating:heat", "ease": "linear", "keys": [
        {"t": 0, "v": 0}, {"t": 1, "v": 1}, {"t": tb, "v": 1}, {"t": tb + P["rampS"], "v": P["boilLevel"]}]},
    {"target": "emitter:steam", "ease": "linear", "keys": [
        {"t": 0, "v": 0}, {"t": 1, "v": 0.02}, {"t": round(tb * 0.8), "v": 0.2}, {"t": tb + P["rampS"], "v": 1}]},
    {"target": "emitter:steam-plume", "ease": "linear", "keys": [
        {"t": 0, "v": 0}, {"t": 1, "v": 0.05}, {"t": round(tb * 0.8), "v": 0.3}, {"t": tb + P["rampS"], "v": 1}]},
]
for pose in a["poses"]:
    if pose["id"] == "boiling":
        pose["thermal"]["operating"]["heat"] = P["boilLevel"]
    if pose["id"] == "heating":
        pose["thermal"]["operating"]["heat"] = 1
    if pose["id"] in ("heating", "boiling"):
        pose["thermal"]["regions"]["base"]["contact"]["tempC"] = P["trivetC"]
for lvl in a["thermal"]["operating"][0]["levels"]:
    pass
a["thermal"]["assumptions"] = [
    {
        "en": f"The burner delivers about 2.5 kW of gas and about 1.2 kW of it enters the pot (about 50 percent efficiency): {P['waterW']} W reach the water through the base, {P['baseW']} W stay in the sandwich base and {P['wallW']} W warm the lower wall. The regions are lumped and do not conduct to each other, so each carries the heat it absorbs. The water region holds 2 kg (2 L) with the heat capacity of the water per surface area, 2 kg x 4186 J/(kg K) / {P['areaM2']} m2: about 660 kJ lift it by 79 K in about {round(tb / 60)} minutes. At the boil the level drops to {P['boilLevel']}, the evaporation of the open surface: the heat leaves as latent heat with the steam emitter ({P['vapour']} kg vapour per kg of air at 0.004 m3/s, 2257 kJ/kg).",
        "de": f"Der Brenner liefert etwa 2,5 kW Gas, davon gelangen etwa 1,2 kW in den Topf (rund 50 Prozent Wirkungsgrad): {P['waterW']} W erreichen das Wasser über den Boden, {P['baseW']} W bleiben im Sandwichboden und {P['wallW']} W erwärmen die untere Wand. Die Regionen sind konzentriert und leiten nicht untereinander, daher trägt jede die Wärme, die sie aufnimmt. Die Wasserregion fasst 2 kg (2 l) mit der Wärmekapazität des Wassers je Oberflächenfläche, 2 kg x 4186 J/(kg K) / {P['areaM2']} m2: rund 660 kJ heben es um 79 K in etwa {round(tb / 60)} Minuten. Beim Sieden sinkt der Pegel auf {P['boilLevel']}, die Verdunstung der offenen Oberfläche: die Wärme verlässt den Topf als latente Wärme mit dem Dampfemitter ({P['vapour']} kg Dampf je kg Luft bei 0,004 m3/s, 2257 kJ/kg).",
    }
]
txt = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
F.write_bytes(txt.replace("\n", "\r\n").encode("utf-8"))
