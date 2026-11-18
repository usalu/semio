"""🦶 Ops that set the lateral spans of the foot and toe slices (plan shapes `foot-X-n`, `toe-X-n`) from the loft sections of the shoe volume, and copy the slices to the front. python3 humansb-B-feetw.py <slug> <scale> <l|r|lr> <out.json> [--nofront]"""
import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/"
slug, scale, sides, out = sys.argv[1:5]
a = json.load(open(CAT + slug + "/🔣️.json"))["asset"]
g = a["representations"][scale]["geometry"]
body = a["volumes"][scale]["body"]
def interp(secs, u):
    secs = sorted(secs, key=lambda s: s["at"][0])
    if u <= secs[0]["at"][0]: return secs[0]
    for p, q in zip(secs, secs[1:]):
        if u <= q["at"][0]:
            t = (u - p["at"][0]) / ((q["at"][0] - p["at"][0]) or 1)
            return {"rx": p["rx"] + (q["rx"] - p["rx"]) * t, "at": [u, 0, p["at"][2] + (q["at"][2] - p["at"][2]) * t]}
    return secs[-1]
ops = []
for side in sides:
    for part, bone in (("foot", f"foot-{side}"), ("toe", f"toe-{side}")):
        solids = [s for s in body if s.get("bone") == bone and s["id"].startswith("shoe") and s.get("kind") == "loft"]
        slices = [s for s in g["plan"] if s["id"].startswith(f"{part}-{side}-")]
        if not solids or not slices: continue
        fixed = []
        for s in slices:
            xs = [p[0] for p in s["points"]]
            lo, hi = min(xs), max(xs)
            rxs = []
            for i in range(5):
                u = lo + (hi - lo) * i / 4
                best = None
                for so in solids:
                    secs = so["sections"]
                    if secs[0]["at"][0] - 1e-9 <= u <= secs[-1]["at"][0] + 1e-9 or True:
                        c = interp(secs, u)
                        if best is None or c["rx"] > best["rx"]: best = c
                rxs.append(best)
            rx = max(c["rx"] for c in rxs); zc = sum(c["at"][2] for c in rxs) / 5
            sol = dict(s["solid"]); sol["z0"] = round(zc - rx, 5); sol["z1"] = round(zc + rx, 5)
            s2 = dict(s); s2["solid"] = sol; fixed.append(s2)
            ops.append({"op": "shape-set", "scale": scale, "view": "plan", "id": s["id"], "set": {"solid": sol}})
        if "--nofront" in sys.argv: continue
        key = f"{part}-{side}"
        ops.append({"op": "shape", "scale": scale, "view": "front", "id": key, "shape": fixed[0]})
        prev = fixed[0]["id"]
        for s in fixed[1:]:
            ops.append({"op": "shape-add", "scale": scale, "view": "front", "after": prev, "shape": s}); prev = s["id"]
json.dump(ops, open(out, "w"), indent=1, ensure_ascii=False)
