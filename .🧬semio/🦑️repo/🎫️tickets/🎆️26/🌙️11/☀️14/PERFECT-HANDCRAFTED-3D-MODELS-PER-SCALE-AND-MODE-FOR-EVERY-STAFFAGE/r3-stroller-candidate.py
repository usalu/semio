"""🧪 Writes the full candidate document of humans/parent-with-stroller for `r3-validate.ts --file` and `r3-2d.ts --file`: the roots candidate of `r3-stroller-roots.py` with the `volumes` of the ticket source put in.

Usage: python3 r3-stroller-candidate.py   (output 🗑️generated/r3-parent-with-stroller/full/🔣️.json)
"""
import json, os

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "r3-parent-with-stroller")
doc = json.load(open(os.path.join(G, "🔣️.json"), encoding="utf-8"))
doc["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes", "humans", "parent-with-stroller.json"), encoding="utf-8"))["volumes"]
os.makedirs(os.path.join(G, "full"), exist_ok=True)
out = os.path.join(G, "full", "🔣️.json")
json.dump(doc, open(out, "w", encoding="utf-8"), ensure_ascii=False)
print(out)
