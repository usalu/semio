"""🥣 Pot thermal budget for 1 kg of water with a boiling phase change: argv JSON (waterW, baseW, wallW, areaM2, boilS, durationS, minutes, needKJ, gS)."""
import json
import sys
from pathlib import Path

F = Path(__file__).resolve().parents[8] / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/cooking-pot/🔣️.json"
P = json.loads(sys.argv[1])
doc = json.loads(F.read_text(encoding="utf-8"))
a = doc["asset"]
regs = {r["id"]: r for r in a["thermal"]["regions"]}
cap = round(1.0 * 4186 / P["areaM2"])

for mat in a["materials"]:
    if mat["id"] == "water":
        mat["phaseChange"] = {"transition": "boiling", "pointC": 100, "latentHeatJKg": 2257000}

w = regs["water"]
w["areaM2"] = P["areaM2"]
w["heatCapacityJM2K"] = cap
w["layers"][0]["heatCapacityJM2K"] = cap
w["operating"]["heatW"] = P["waterW"]
regs["base"]["operating"]["heatW"] = P["baseW"]
regs["wall"]["operating"]["heatW"] = P["wallW"]

a["thermal"]["operating"][0]["levels"] = [
    {"v": 0, "title": {"en": "Cold", "de": "Kalt"}},
    {"v": 1, "title": {"en": "Heating at 1.2 kW", "de": "Heizt mit 1,2 kW"}},
]
steam = a["thermal"]["emitters"][0]
steam["region"] = "water"
steam["vapourKgPerKg"] = 0.3

tb = P["boilS"]
T = P["durationS"]
m = P["minutes"]
clip = a["clips"][0]
clip["durationS"] = T
clip["description"] = {
    "en": f"One litre of cold water warms for about {m} minutes at 1.2 kW until it boils; at 100 degC the temperature stops and the burner heat leaves as steam.",
    "de": f"Ein Liter kaltes Wasser erwärmt sich bei 1,2 kW etwa {m} Minuten lang, bis es kocht; bei 100 °C bleibt die Temperatur stehen und die Brennerwärme verlässt den Topf als Dampf.",
}
clip["keys"] = [
    {"t": 0, "pose": "cold", "ease": "step"},
    {"t": 1, "pose": "heating", "ease": "linear"},
    {"t": tb, "pose": "boiling", "ease": "step"},
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
    {"target": "operating:heat", "ease": "linear", "keys": [{"t": 0, "v": 0}, {"t": 1, "v": 1}]},
    {"target": "emitter:steam-plume", "ease": "linear", "keys": [
        {"t": 0, "v": 0}, {"t": 1, "v": 0.05}, {"t": round(tb * 0.8), "v": 0.3}, {"t": tb, "v": 1}]},
]
for pose in a["poses"]:
    if pose["id"] in ("heating", "boiling"):
        pose["thermal"]["operating"]["heat"] = 1
        pose["thermal"]["emitters"]["steam"] = 1

en = a["provenance"]["notes"]["en"]
de = a["provenance"]["notes"]["de"]
a["provenance"]["notes"]["en"] = en.split(" The 1 kg of water")[0].split(" The 2 kg of water")[0] + f" The 1 kg of water needs 419 kJ from 0 to 100 degC and about {P['needKJ']} kJ for the 79 K from the room: the hob delivers 1.2 kW into the pot (gas-hob), so the open pot boils after about {m} minutes; at 100 degC the phase change of the water holds the temperature and turns the surplus heat into steam."
a["provenance"]["notes"]["de"] = de.split(" Das 1 kg Wasser")[0].split(" Die 2 kg Wasser")[0] + f" Das 1 kg Wasser braucht von 0 auf 100 °C 419 kJ und für die 79 K ab Raumtemperatur rund {P['needKJ']} kJ: das Kochfeld liefert 1,2 kW in den Topf (gas-hob), sodass der offene Topf nach etwa {m} Minuten kocht; bei 100 °C hält der Phasenwechsel des Wassers die Temperatur und macht die überschüssige Wärme zu Dampf."

a["thermal"]["assumptions"] = [
    {
        "en": f"The burner delivers about 2.5 kW of gas and about 1.2 kW of it enters the pot (about 50 percent efficiency): {P['waterW']} W reach the water through the base, {P['baseW']} W stay in the sandwich base and {P['wallW']} W warm the lower wall. The regions are lumped and do not conduct to each other, so each carries the heat it absorbs. The water region holds 1 kg (1 L) with the heat capacity of the water per surface area, 1 kg x 4186 J/(kg K) / {P['areaM2']} m2: about {P['needKJ']} kJ lift it by 79 K in about {m} minutes, the order of the 419 kJ of the physics lesson (1 kg, 100 K). At 100 degC the phase change of the water caps the temperature and the surplus power evaporates it at 2.257 MJ/kg; the steam emitter releases exactly this vapour rate.",
        "de": f"Der Brenner liefert etwa 2,5 kW Gas, davon gelangen etwa 1,2 kW in den Topf (rund 50 Prozent Wirkungsgrad): {P['waterW']} W erreichen das Wasser über den Boden, {P['baseW']} W bleiben im Sandwichboden und {P['wallW']} W erwärmen die untere Wand. Die Regionen sind konzentriert und leiten nicht untereinander, daher trägt jede die Wärme, die sie aufnimmt. Die Wasserregion fasst 1 kg (1 l) mit der Wärmekapazität des Wassers je Oberflächenfläche, 1 kg x 4186 J/(kg K) / {P['areaM2']} m2: rund {P['needKJ']} kJ heben es um 79 K in etwa {m} Minuten, in der Größenordnung der 419 kJ der Physiklektion (1 kg, 100 K). Bei 100 °C deckelt der Phasenwechsel des Wassers die Temperatur und die überschüssige Leistung verdampft es mit 2,257 MJ/kg; der Dampfemitter gibt genau diese Dampfrate ab.",
    }
]
txt = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
F.write_bytes(txt.replace("\n", "\r\n").encode("utf-8"))
