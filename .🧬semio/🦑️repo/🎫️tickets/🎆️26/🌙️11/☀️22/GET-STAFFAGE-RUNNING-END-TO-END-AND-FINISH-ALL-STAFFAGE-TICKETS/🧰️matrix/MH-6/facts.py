"""🔬️ Facts of an asset document for the fixer: `python3 facts.py <file.json> [scale]` prints the authored scales, the bones with length and parent, the anchors, the poses and clips with the bones they turn, and per authored scale and bone the shapes of each view with their construction (flat, sweep v, sweep w, revolved, followTip)."""
import json, sys, collections
a = json.load(open(sys.argv[1]))["asset"]
only = sys.argv[2] if len(sys.argv) > 2 else None
print("id", a["id"], "dims", a["dimensions"], "default", a["presentation"]["defaultPose"])
print("scales", {k: v["status"] for k, v in a["representations"].items()})
print("bones", " ".join(f'{b["id"]}<{b.get("parent","")}:{b.get("lengthM")}' for b in a["rig"]["bones"]))
for an in a.get("anchors", []):
    print("anchor", an["id"], an.get("kind"), an.get("bone"), an.get("x"), an.get("y"), an.get("z"))
def turned(node, acc):
    if isinstance(node, list):
        for n in node: turned(n, acc)
    elif isinstance(node, dict):
        for k, v in node.items():
            if k == "bones" and isinstance(v, dict):
                for b, o in v.items():
                    if isinstance(o, dict):
                        for f in ("angleDeg", "abductionDeg", "twistDeg"):
                            if abs(o.get(f) or 0) >= 1: acc[b].add(f[:2])
            turned(v, acc)
for name in ("poses", "clips"):
    items = a.get(name) or []
    for p in (items if isinstance(items, list) else items.values()):
        acc = collections.defaultdict(set); turned(p, acc)
        print(name[:-1], p.get("id"), " ".join(f"{b}:{''.join(sorted(s))}" for b, s in acc.items()))
def kind(s):
    so = s.get("solid")
    k = s["kind"] + ("+tip" if s.get("followTip") else "")
    if not so: return k + ":flat"
    return k + ":" + (so["kind"][:3] + (so.get("sweep") or so.get("axis") or ""))
for sc, rep in a["representations"].items():
    if rep["status"] != "authored" or (only and sc != only): continue
    by = collections.defaultdict(lambda: collections.defaultdict(list))
    for view, shapes in rep["geometry"].items():
        for s in shapes:
            by[s.get("bone")][view].append(f'{s["id"]}[{kind(s)}]')
    print("== 1:" + sc, {v: len(s) for v, s in rep["geometry"].items()})
    for bone, views in by.items():
        print(" ", bone)
        for v in ("elevation", "front", "plan"):
            print("    ", v[:2], " ".join(views.get(v, [])))
