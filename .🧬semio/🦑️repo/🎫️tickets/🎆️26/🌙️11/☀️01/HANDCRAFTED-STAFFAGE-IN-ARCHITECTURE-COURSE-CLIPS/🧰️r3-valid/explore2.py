import json, sys
from collections import Counter
sys.stdout.reconfigure(encoding="utf-8")
S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/"
m = json.load(open(S + "🗂️catalog/🔣️.json", encoding="utf-8"))
print([k for k in m]); 
rows = m["entries"]
print(len(rows)); print(rows[0])
print(Counter(tuple(r.get("analytics", [])) for r in rows))
cast = json.load(open("C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json", encoding="utf-8"))["casts"]
by = {r["id"]: r for r in rows}
bad = []
for cid, c in cast.items():
    an = by[c["asset"]].get("analytics", [])
    if c["view"] == "thermal" and "thermal" not in an: bad.append((cid, "thermal"))
    if c.get("transport") and "transport" not in an: bad.append((cid, "transport"))
print(bad)
