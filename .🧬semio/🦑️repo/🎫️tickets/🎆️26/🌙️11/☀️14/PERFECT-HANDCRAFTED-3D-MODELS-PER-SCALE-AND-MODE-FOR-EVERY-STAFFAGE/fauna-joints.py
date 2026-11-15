"""🦴️ Moves the generated joint solids of the listed bones laterally onto their limb (kit-structure puts every joint on the bone origin at z = 0, outside a body whose limb sits at z = +-offset).
Usage: python3 fauna-joints.py <asset> <scale> <bone>=<z>[,<bone>=<z>...]"""
import json, sys
asset, scale, spec = sys.argv[1], sys.argv[2], dict((a, float(b)) for a, b in (x.split("=") for x in sys.argv[3].split(",")))
f = f"volumes/{asset}.json"
d = json.load(open(f))
n = 0
for s in d["volumes"][scale]["structure"]:
    if s.get("part") == "joint" and s["bone"] in spec:
        tr = list(s.get("transform", {}).get("translation", [0, 0, 0]))
        tr[2] = spec[s["bone"]]
        s["transform"] = {**s.get("transform", {}), "translation": tr}
        n += 1
json.dump(d, open(f, "w"), separators=(",", ":"), ensure_ascii=False)
print("joints moved", n)
