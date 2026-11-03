import json
import os
from pathlib import Path

os.chdir("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
P = Path("🔌️appliances/electric-kettle/🔣️.json")
d = json.loads(P.read_text(encoding="utf-8"))
clip = next(c for c in d["asset"]["clips"] if c["id"] == "boil")
BOIL, END, WARM, SOFT = 184.0, 200.0, 171.0, 141.0
clip["durationS"] = END
clip["keys"][2]["t"] = WARM
clip["keys"][3]["t"] = BOIL
clip["keys"][4]["t"] = END
clip["contacts"][0]["toS"] = END
heat = next(p for p in clip["phases"] if p["id"] == "heat")
boil = next(p for p in clip["phases"] if p["id"] == "boil")
heat["toS"] = BOIL
boil["fromS"] = BOIL
boil["toS"] = END
steam = next(t for t in clip["tracks"] if t["target"] == "emitter:steam-plume")
steam["keys"][2]["t"] = SOFT
steam["keys"][3]["t"] = BOIL
P.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="")
print("ok")
d = json.loads(P.read_text(encoding="utf-8"))
for pose in d["asset"]["poses"]:
    pose["thermal"]["regions"]["water"]["exposedFraction"] = 0.05
P.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="")
print("poses")
