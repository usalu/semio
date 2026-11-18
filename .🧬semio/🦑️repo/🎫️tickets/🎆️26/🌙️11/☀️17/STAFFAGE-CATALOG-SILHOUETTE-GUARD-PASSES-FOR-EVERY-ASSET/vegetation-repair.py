"""Repairs shapes that the rewrites left with members their kind does not define: a path with followTip becomes a poly of its largest ring, a non-ellipse with rotationDeg loses it. python3 vegetation-repair.py <slug>..."""
import sys, json, re, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
def area(r): return abs(sum(r[i][0] * r[(i + 1) % len(r)][1] - r[(i + 1) % len(r)][0] * r[i][1] for i in range(len(r)))) / 2
for slug in sys.argv[1:]:
    d = mv.D("🌳️vegetation/" + slug); a = d.val("asset"); n = 0
    for sc, rep in a["representations"].items():
        for view, geo in rep.get("geometry", {}).items():
            for s in geo:
                o = dict(s); changed = False
                if o["kind"] == "path" and "followTip" in o:
                    rings = []
                    for sub in re.split(r"M", o["d"])[1:]:
                        nums = [float(x) for x in re.findall(r"-?\d+\.?\d*(?:e-?\d+)?", sub)]
                        rings.append(list(zip(nums[0::2], nums[1::2])))
                    r = max(rings, key=area)
                    o["kind"] = "poly"; o["points"] = [[round(x, 4), round(y, 4)] for x, y in r]; o["closed"] = True; del o["d"]; changed = True
                if o["kind"] not in ("ellipse",) and "rotationDeg" in o: del o["rotationDeg"]; changed = True
                if changed:
                    order = [k for k in s if k in o] + [k for k in o if k not in s]
                    d.replace_obj(d.shape(sc, view, s["id"]), {k: o[k] for k in order}); n += 1
    d.commit(); print(slug, "repaired", n)
