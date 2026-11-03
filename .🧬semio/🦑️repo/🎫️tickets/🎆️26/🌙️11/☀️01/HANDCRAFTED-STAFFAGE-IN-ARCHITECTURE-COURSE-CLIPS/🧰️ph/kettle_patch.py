"""🫖 ph: bind the phase change of water to the electric kettle, drop the power throttle and let the steam vent follow the evaporation - run once."""
import json
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
PATH = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json"
data = json.loads(PATH.read_text(encoding="utf8"))
a = data["asset"]
th = a["thermal"]

for m in a["materials"]:
    if m["id"] == "water":
        m["phaseChange"] = {"transition": "boiling", "pointC": 100, "latentHeatJKg": 2257000}

th["operating"][0]["levels"] = [lv for lv in th["operating"][0]["levels"] if lv["v"] in (0, 1)]

for e in th["emitters"]:
    if e["id"] == "steam":
        e.pop("volumeFlowM3S", None)
        e["region"] = "water"
        e["title"] = {"en": "Steam leaving the spout", "de": "Dampf am Ausguss"}

for pose in a["poses"]:
    if pose["id"] in ("heating", "boiling"):
        pose["thermal"]["operating"]["power"] = 1
        pose["thermal"]["emitters"]["steam"] = 1

for clip in a["clips"]:
    if clip["id"] == "boil":
        for tr in clip["tracks"]:
            if tr["target"] == "operating:power":
                tr["keys"] = [{"t": 0, "v": 0}, {"t": 1, "v": 1}]
        clip["tracks"] = [tr for tr in clip["tracks"] if tr["target"] != "emitter:steam"]

PATH.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf8", newline="\n")
print("ok")
