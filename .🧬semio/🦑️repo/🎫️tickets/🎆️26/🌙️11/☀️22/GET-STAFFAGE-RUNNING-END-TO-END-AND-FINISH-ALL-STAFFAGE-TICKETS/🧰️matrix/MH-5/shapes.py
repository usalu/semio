"""🗒️ Lists, per bone of an asset at a scale, the shapes of the three views with kind and solid: `python3 shapes.py <file.json> <scale> [bone-substring]`."""
import json, sys
d = json.load(open(sys.argv[1]))["asset"]
g = d["representations"][sys.argv[2]]["geometry"]
flt = sys.argv[3] if len(sys.argv) > 3 else ""
bones = [b["id"] for b in d["rig"]["bones"]]
def tag(s):
    so = s.get("solid")
    t = s["kind"] + ("" if not so else ":" + so["kind"] + (("-" + so.get("sweep", "")) if so["kind"] == "extruded" else "")) + (":tip" if s.get("followTip") else "")
    return f'{s["id"]}({t}{",ground" if s.get("role") == "ground" else ""})'
for b in bones:
    if flt not in b: continue
    row = {v: [tag(s) for s in g[v] if s["bone"] == b] for v in ("elevation", "front", "plan")}
    if any(row.values()): print(b, "\n   E", " ".join(row["elevation"]), "\n   F", " ".join(row["front"]), "\n   P", " ".join(row["plan"]))
