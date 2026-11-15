"""✂️ Author 64: drops body solids of one scale (the drawing of that scale does not draw them), converts thermal and structure references to them into own copies so thermal and structure keep their coverage, replaces the decisions.
Usage: python3 author-64-dropbody.py <category/slug> <scale> <id,id,...> <decisions.json> [--force-age]
decisions.json = [{"en","de"},...]"""
import copy, json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
key, scale, ids, dec = sys.argv[1], sys.argv[2], set(sys.argv[3].split(",")), json.load(open(sys.argv[4]))
path = f"{T}/volumes/{key}.json"
if (time.time() - os.path.getmtime(path)) / 60 < 20 and "--force-age" not in sys.argv:
    print("SKIP", key); sys.exit()
raw = open(path).read()
src = json.loads(raw)
v = src["volumes"][scale]
dropped = {x["id"]: x for x in v["body"] if x["id"] in ids}
v["body"] = [x for x in v["body"] if x["id"] not in ids]
for lst in ("thermal", "structure"):
    out = []
    for t in v[lst]:
        if t.get("of") in dropped:
            solid = copy.deepcopy(dropped[t["of"]])
            solid.pop("repeat", None) if False else None
            solid["id"] = t["id"]
            for k in t:
                if k not in ("of", "id"):
                    solid[k] = t[k]
            out.append(solid)
        else:
            out.append(t)
    v[lst] = out
v["decisions"] = dec
out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
assert open(path).read() == raw
open(path, "w").write(out)
print("WROTE", key, scale, len(dropped), "body solids dropped")
