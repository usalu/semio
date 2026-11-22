"""🧵 Tower crane 1:50 to 1:200: the two rope falls lie in the sheave grooves (0.204 m off the axis instead of 0.200 m, where they hung 2.8 mm clear of the trolley sheaves)."""
import json
import os
import re

SOURCE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "volumes", "machinery", "tower-crane.json")
src = json.load(open(SOURCE, encoding="utf-8"))
for sc in ("50", "100", "200"):
    for s in src["volumes"][sc]["body"]:
        m = re.match(r"^rope-\d-(a|b)$", s["id"])
        if m:
            s["origin"][0] = -0.204 if m.group(1) == "a" else 0.204
json.dump(src, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
