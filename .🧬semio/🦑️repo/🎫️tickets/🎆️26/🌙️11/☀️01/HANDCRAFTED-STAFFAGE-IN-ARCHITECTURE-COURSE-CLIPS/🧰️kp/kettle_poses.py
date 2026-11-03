"""🫖 kp: rewire poses and the boil clip of the electric kettle to the water heating: kettle_poses.py <boil_level> <t_boil_s> <duration_s>"""
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json"
a = json.loads(PATH.read_text(encoding="utf8"))
asset = a["asset"]
boil, tb, dur = float(sys.argv[1]), float(sys.argv[2]), float(sys.argv[3])
poses = {p["id"]: p for p in asset["poses"]}
poses["heating"]["thermal"]["operating"]["power"] = 1
poses["heating"]["thermal"]["emitters"] = {"steam": 0.3, "steam-plume": 0.4}
poses["heating"]["description"] = {"en": "The 2 kW element heats the water directly; the shell warms through and a little vapour appears.", "de": "Das 2-kW-Heizelement erwärmt das Wasser direkt; die Hülle erwärmt sich durch und etwas Dampf tritt aus."}
poses["boiling"]["thermal"]["operating"]["power"] = boil
clip = next(c for c in asset["clips"] if c["id"] == "boil")
clip["durationS"] = dur
clip["description"] = {"en": "The kettle heats 1 L of water from about 15 degC to the boil in about three minutes, then boils with steam at the spout.", "de": "Der Wasserkocher erhitzt 1 l Wasser in etwa drei Minuten von etwa 15 °C bis zum Sieden und kocht dann mit Dampf am Ausguss."}
ramp = tb - 30.0
clip["keys"] = [
    {"t": 0, "pose": "off", "ease": "step"},
    {"t": 1, "pose": "heating", "ease": "linear"},
    {"t": tb, "pose": "heating", "ease": "linear"},
    {"t": tb + 16, "pose": "boiling", "ease": "step"},
    {"t": dur, "pose": "boiling", "ease": "step"},
]
clip["contacts"] = [{"anchor": "base-contact", "fromS": 0, "toS": dur}]
clip["phases"][1]["toS"] = tb
clip["phases"][2]["fromS"] = tb
clip["phases"][2]["toS"] = dur
tracks = {
    "operating:power": [(0, 0), (1, 1), (tb, 1), (tb + 16, boil)],
    "emitter:steam": [(0, 0), (1, 0.1), (ramp, 0.3), (tb + 16, 1)],
    "emitter:steam-plume": [(0, 0), (1, 0.2), (ramp, 0.4), (tb + 16, 1)],
}
for tr in clip["tracks"]:
    tr["keys"] = [{"t": t, "v": v} for t, v in tracks[tr["target"]]]
for lv in asset["thermal"]["operating"][0]["levels"]:
    if lv["title"]["en"].startswith("Boiling"):
        lv["v"] = boil
asset["thermal"]["operating"][0]["levels"].sort(key=lambda lv: lv["v"])
PATH.write_text(json.dumps(a, ensure_ascii=False, indent=2) + "\n", encoding="utf8", newline="\n")
print("ok")
