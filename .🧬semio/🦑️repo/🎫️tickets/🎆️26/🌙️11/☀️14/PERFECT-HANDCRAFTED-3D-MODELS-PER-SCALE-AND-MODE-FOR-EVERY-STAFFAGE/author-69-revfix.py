"""python3 author-69-revfix.py <slug> <id,id,...>: the named open revolve profiles (off the axis at both ends, which the kernel closes through the axis) get the two axis points written out, so the solid disc / plug that was meant is explicit and the trap is gone. Geometry is unchanged."""
import sys
sys.path.insert(0, ".")
from author_69_lib import *
slug, ids = sys.argv[1], set(sys.argv[2].split(","))
p = f"{T}/volumes/humans/{slug}.json"
d = json.load(open(p)); n = 0
for sc, v in d["volumes"].items():
    if v.get("status") != "authored": continue
    for l in ("body", "thermal", "structure"):
        for s in v[l]:
            if s.get("kind") == "revolve" and s["id"] in ids and s["profile"][0][1] != 0 and s["profile"][-1][1] != 0 and s["profile"][0] != s["profile"][-1]:
                s["profile"] = [[s["profile"][0][0], 0]] + s["profile"] + [[s["profile"][-1][0], 0]]; n += 1
json.dump(d, open(p, "w"), ensure_ascii=False, separators=(",", ":"))
print(n, "profiles")
