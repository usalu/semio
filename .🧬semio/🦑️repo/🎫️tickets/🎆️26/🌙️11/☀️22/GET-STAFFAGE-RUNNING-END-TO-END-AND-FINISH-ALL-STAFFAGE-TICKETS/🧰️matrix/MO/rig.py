"""🦴️ The rig, poses, anchors and clips of an asset document in short lines, and with a bone list the shapes of those bones per band and view: `python3 rig.py <asset.json> [bone,bone] [width]`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]
print("dims", a["dimensions"], "default", a["presentation"]["defaultPose"])
for b in a["rig"]["bones"]:
    print("bone", {k: v for k, v in b.items() if k not in ("title", "mass")})
for p in a["poses"]:
    print("pose", p["id"], json.dumps({k: p[k] for k in ("bones", "root", "ground", "contacts") if k in p}))
for n in a["anchors"]:
    print("anchor", json.dumps({k: v for k, v in n.items() if k not in ("title", "description")}))
for c in a.get("clips", []):
    print("clip", c["id"], sorted({t for f in c.get("tracks", c.get("keyframes", [])) for t in (f.get("bones", {}) if isinstance(f, dict) else {})}) or list(c.keys()))
if len(sys.argv) > 2:
    bones = set(sys.argv[2].split(","))
    w = int(sys.argv[3]) if len(sys.argv) > 3 else 260
    for k, r in a["representations"].items():
        for view, shapes in r.get("geometry", {}).items():
            for s in shapes:
                if s["bone"] in bones:
                    print(f"1:{k} {view}", json.dumps({x: y for x, y in s.items() if x not in ("material", "tone", "region", "regions", "role")}, ensure_ascii=False)[:w])
