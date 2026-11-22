"""🔬️ Writes `probe/🔣️.json`: the candidate with one group of 1:50 plan shapes removed (argument: a substring of the shape ids to drop), to bisect a silhouette finding."""
import json, sys
d = json.load(open("🔣️.json"))
g = d["asset"]["representations"]["50"]["geometry"]
g["plan"] = [s for s in g["plan"] if not any(k in s["id"] for k in sys.argv[1].split(","))]
json.dump(d, open("probe/🔣️.json", "w"), ensure_ascii=False)
print(len(g["plan"]))
