"""🫖 ph: retime the kettle boil clip to the solved boil at constant 2 kW (water reaches 100 degC at about 184 s) and describe the phase change - run once."""
import json
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json"
data = json.loads(PATH.read_text(encoding="utf8"))
a = data["asset"]
th = a["thermal"]
BOIL = 184.0

a["provenance"]["notes"]["en"] = a["provenance"]["notes"]["en"].replace(
    "Once the water boils, the heat only evaporates it: the steam emitters show the vapour.",
    "The water carries its phase change (boiling at 100 degC, 2.257 MJ/kg): at 100 degC the temperature holds and the surplus power evaporates the water at about 0.85 g/s, which the steam emitter follows.",
)
a["provenance"]["notes"]["de"] = a["provenance"]["notes"]["de"].replace(
    "Sobald das Wasser kocht, verdampft die Wärme es nur noch: die Dampfemitter zeigen den Dampf.",
    "Das Wasser trägt seine Phasenumwandlung (Sieden bei 100 °C, 2,257 MJ/kg): bei 100 °C bleibt die Temperatur stehen und die überschüssige Leistung verdampft das Wasser mit etwa 0,85 g/s, dem der Dampfemitter folgt.",
)
th["assumptions"][0] = {
    "en": "The 2 kW element heats the water region directly at a constant 1930 W (plate 40, shell 15, lid 5): 1 kg of water (1 L, 4186 J/(kg K)) rises by 85 K in about 3 minutes (356 kJ). The region heat capacity is the water's per gauge area. At 100 degC the water's phase change caps the temperature and the surplus power evaporates it at 2.257 MJ/kg (about 0.85 g/s); the steam emitter releases exactly this vapour rate.",
    "de": "Das 2-kW-Heizelement heizt die Wasserregion direkt mit konstant 1930 W (Platte 40, Hülle 15, Deckel 5): 1 kg Wasser (1 l, 4186 J/(kg K)) steigt in etwa 3 Minuten um 85 K (356 kJ). Die Wärmekapazität der Region ist die des Wassers je Wasserstandsfläche. Bei 100 °C deckelt die Phasenumwandlung des Wassers die Temperatur und die überschüssige Leistung verdampft es mit 2,257 MJ/kg (etwa 0,85 g/s); der Dampfemitter gibt genau diese Dampfrate ab.",
}

for clip in a["clips"]:
    if clip["id"] != "boil":
        continue
    clip["description"] = {
        "en": "The kettle heats 1 L of water from about 15 degC to the boil in about three minutes at a constant 2 kW, then holds 100 degC and evaporates the water with steam at the spout.",
        "de": "Der Wasserkocher erhitzt 1 l Wasser mit konstant 2 kW in etwa drei Minuten von etwa 15 °C bis zum Sieden, hält dann 100 °C und verdampft das Wasser mit Dampf am Ausguss.",
    }
    clip["keys"] = [
        {"t": 0, "pose": "off", "ease": "step"},
        {"t": 1, "pose": "heating", "ease": "linear"},
        {"t": 171.0, "pose": "heating", "ease": "linear"},
        {"t": BOIL, "pose": "boiling", "ease": "step"},
        {"t": clip["durationS"], "pose": "boiling", "ease": "step"},
    ]
    for ph in clip["phases"]:
        if ph["id"] == "heat":
            ph["toS"] = BOIL
        if ph["id"] == "boil":
            ph["fromS"] = BOIL
    for tr in clip["tracks"]:
        if tr["target"] == "emitter:steam-plume":
            tr["keys"] = [{"t": 0, "v": 0}, {"t": 1, "v": 0.2}, {"t": 141.0, "v": 0.4}, {"t": BOIL, "v": 1}]

for pose in a["poses"]:
    if pose["id"] == "heating":
        pose["description"] = {
            "en": "The 2 kW element heats the water directly; the shell warms through and the vapour rises with the water temperature.",
            "de": "Das 2-kW-Heizelement erwärmt das Wasser direkt; die Hülle erwärmt sich durch und der Dampf nimmt mit der Wassertemperatur zu.",
        }

PATH.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8", newline="\n")
print("ok")
