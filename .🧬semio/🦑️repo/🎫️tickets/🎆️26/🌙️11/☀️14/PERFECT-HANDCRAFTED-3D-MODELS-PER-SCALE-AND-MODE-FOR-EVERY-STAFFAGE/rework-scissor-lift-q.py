"""🔎️ Print solids of the scissor-lift source compactly: rework-scissor-lift-q.py <scale> <list> <id regex> [exclude regex]."""
import json, re, sys
s = json.load(open("volumes/machinery/scissor-lift.json"))["volumes"]
scale, lst, rx = sys.argv[1], sys.argv[2], re.compile(sys.argv[3])
ex = re.compile(sys.argv[4]) if len(sys.argv) > 4 else None
def r(v):
    if isinstance(v, float): return round(v, 4)
    if isinstance(v, list): return [r(x) for x in v]
    if isinstance(v, dict): return {k: r(x) for k, x in v.items()}
    return v
for x in s[scale][lst]:
    if not rx.search(x["id"]) or (ex and ex.search(x["id"])): continue
    d = {k: v for k, v in x.items() if k not in ("material", "tone", "role", "faces")}
    if "vertices" in d: d["vertices"] = f"{len(d['vertices'])}v"
    if "transform" in d and list(d["transform"]) == ["translation"]: d["at"] = d.pop("transform")["translation"]
    t = json.dumps(r(d), ensure_ascii=False, separators=(",", ":")).replace('"', "")
    print(t[:420])
