import json, sys
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW")
from atomic_json import write_json_atomic

P = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-hob/🔣️.json"
doc = json.load(open(P, encoding="utf-8"))
for s in doc["asset"]["representations"]["200"]["geometry"]["elevation"]:
    if s["id"] == "burner":
        s["points"] = [[-0.056, 0], [0.056, 0], [0.046, 0.012], [0.022, 0.012], [0.018, 0.03], [-0.018, 0.03], [-0.022, 0.012], [-0.046, 0.012]]
write_json_atomic(P, doc)
