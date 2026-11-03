import sys
sys.path.insert(0, "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️FW")
from atomic_json import write_json_atomic
import json
P = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"
raw = open(P, encoding="utf8", newline="").read()
i = raw.index('"id": "tied-canopy"')
j = raw.index('"heightM": 3.14', i)
assert j - i < 400
raw = raw[:j] + '"heightM": 5.35' + raw[j + len('"heightM": 3.14'):]
open(P + ".tmp", "w", encoding="utf8", newline="").write(raw)
import os
json.loads(raw)
os.replace(P + ".tmp", P)
print("ok")
