"""🕺️ The bones a pose or clip of an asset moves, and the ids of its volumes per scale: `python3 moves.py <asset.json>`."""
import json, sys
a = json.load(open(sys.argv[1]))["asset"]
for p in a["poses"]:
    print("pose", p["id"], json.dumps(p.get("bones", {})))
for c in a["clips"]:
    bones = {}
    for t in c.get("tracks", c.get("keys", [])):
        if isinstance(t, dict):
            for b, o in (t.get("bones") or {}).items():
                bones.setdefault(b, set()).update(o.keys())
    print("clip", c["id"], {b: sorted(v) for b, v in bones.items()} or list(c.keys()))
for k, v in a["volumes"].items():
    if v.get("status") == "authored":
        print("vol", k, len(v["body"]), " ".join(s["id"] for s in v["body"])[:600])
print("anchors", [(x["id"], x["bone"], x["kind"], x.get("x"), x.get("y"), x.get("z")) for x in a["anchors"]])
