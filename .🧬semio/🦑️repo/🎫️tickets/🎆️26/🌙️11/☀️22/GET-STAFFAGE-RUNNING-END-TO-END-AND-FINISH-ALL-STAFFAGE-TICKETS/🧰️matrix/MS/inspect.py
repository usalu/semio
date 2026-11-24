"""🔬️ Prints the structure of one asset document: dimensions, bones, poses, anchors and per representation and view the shapes with their kind, bone, role and bounding box."""
import json, sys, glob
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
def pts(o, acc):
    if isinstance(o, dict):
        if "x" in o and "y" in o and isinstance(o["x"], (int, float)) and isinstance(o["y"], (int, float)): acc.append((o["x"], o["y"]))
        for v in o.values(): pts(v, acc)
    elif isinstance(o, list):
        for v in o: pts(v, acc)
for a in sys.argv[1].split(","):
    f = glob.glob(f"{S}/*/{a}/🔣️.json")[0]
    d = json.load(open(f))["asset"]
    print("=====", a, list(d.keys()))
    print("dims", d.get("dimensions"), "symmetry" , d.get("symmetry"))
    print("bones", [(b["id"], b.get("parent"), b.get("origin") or b.get("at")) for b in d["rig"]["bones"]][:40], len(d["rig"]["bones"]))
    print("poses", [(p["id"], json.dumps(p)[:200]) for p in d["poses"]])
    print("clips", [c["id"] for c in d["clips"]])
    print("anchors", len(d.get("anchors", [])))
    if len(sys.argv) > 2 and "a" in sys.argv[2]:
        for an in d.get("anchors", []): print("   ", json.dumps(an, ensure_ascii=False)[:260])
    for k, r in d["representations"].items():
        g = r.get("geometry")
        print("rep", k, [x for x in r.keys()], r.get("validRange"), json.dumps(r, ensure_ascii=False)[:150] if g is None else "")
        if g is None: continue
        for v in ("elevation", "front", "plan"):
            sh = g.get(v, [])
            print("  ", v, len(sh))
            lim = int(sys.argv[3]) if len(sys.argv) > 3 else 12
            for s in sh[:lim]:
                acc = []; pts(s, acc)
                bb = (round(min(p[0] for p in acc), 3), round(max(p[0] for p in acc), 3), round(min(p[1] for p in acc), 3), round(max(p[1] for p in acc), 3)) if acc else None
                print("     ", s.get("id"), s.get("kind"), "bone", s.get("bone"), "role", s.get("role"), "solid" if "solid" in s else "", [x for x in s.keys() if x not in ("id", "kind", "bone", "role")], bb)
