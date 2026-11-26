"""🍗️ Draws the limb roots the trunk outline drew: llimb.py <id>… puts `body` ops of `loft.ts` at the head of `loft/<id>.json` for the plain loft solids of the bones BONES (regular expression, default thigh, scapula and humerus of both sides) that have no shape in at least one view, per authored scale. The paint is the trunk shape the part lies on (the first filled body shape that `lumbar` has in all three views for a thigh, `thorax` for scapula and humerus), the old body shapes of the bone leave the views, and the paint order is set per view: just before the trunk in the side elevation on the near side, just behind it on the far side, and behind it in the front and the plan (the paint order of a side elevation in a front elevation draws a ham over the face). The `loft` ops of those bones leave the list, and their entries leave `loft/<id>.lofts.json`."""
import json, os, pathlib, re, sys
W = pathlib.Path(__file__).resolve().parent
GEN = W.parent.parent / "🗑️generated" / "matrix" / "MA"
BONES = os.environ.get("BONES", r"(thigh-h.|scapula-f.|humerus-f.)")
VIEWS = ("elevation", "front", "plan")
for a in sys.argv[1:]:
    asset = json.load(open(GEN / "backup" / f"{a}.json"))["asset"]
    path = W / "loft" / f"{a}.json"
    ops = [op for op in json.load(open(path)) if not (op.get("op") == "body" and op.get("limb"))]
    made, taken = [], set()
    for scale, rep in asset["representations"].items():
        volume = asset.get("volumes", {}).get(scale, {})
        if rep.get("status") != "authored" or volume.get("status") != "authored":
            continue
        everywhere = lambda sid, bone: all(any(s["id"] == sid and s["bone"] == bone for s in rep["geometry"][v]) for v in VIEWS)
        trunk = lambda bone: next((s["id"] for s in rep["geometry"]["elevation"] if s["bone"] == bone and s.get("role") == "body" and s.get("filled") and everywhere(s["id"], bone)), None)
        for solid in volume["body"]:
            bone = solid.get("bone") or ""
            if not re.fullmatch(BONES, bone) or solid["kind"] != "loft" or solid.get("weights") or solid.get("repeat") or solid.get("transform"):
                continue
            if all(any(s["bone"] == bone for s in rep["geometry"][v]) for v in VIEWS):
                continue
            paint = trunk("lumbar" if bone.startswith("thigh") else "thorax") or trunk("lumbar") or trunk("thorax")
            if paint is None:
                continue
            old = sorted({s["id"] for v in VIEWS for s in rep["geometry"][v] if s["bone"] == bone and s.get("role") == "body" and s.get("filled")})
            near = bone.endswith("n")
            made.append({"op": "body", "limb": True, "scale": scale, "solid": solid["id"], "paint": paint, **({"replace": old} if old else {}), "z": {"elevation": 0.01 if near else -0.01, "front": -0.01, "plan": -0.01}})
            taken.add((scale, bone))
    ops = [op for op in ops if (op.get("scale"), op.get("bone")) not in taken]
    path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in ops + made) + "\n]\n")
    lofts = W / "loft" / f"{a}.lofts.json"
    if lofts.exists():
        entries = [e for e in json.load(open(lofts)) if (e["scale"], e["bone"]) not in taken]
        lofts.write_text("[\n" + ",\n".join(" " + json.dumps(e, ensure_ascii=False) for e in entries) + "\n]\n")
    print(a, len(made), "limb bodies;", " ".join(sorted({f"{s}:{b}" for s, b in taken}))[:200])
