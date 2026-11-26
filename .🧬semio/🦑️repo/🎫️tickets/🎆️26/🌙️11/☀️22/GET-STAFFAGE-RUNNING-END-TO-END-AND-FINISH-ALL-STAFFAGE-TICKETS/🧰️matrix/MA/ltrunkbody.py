"""🐄️ Draws a trunk as the bodies of its volume: ltrunkbody.py <id>… puts, per authored scale, one `body` op of `loft.ts` per plain loft solid of the trunk bones (TRUNK, default "pelvis lumbar thorax"; skinned solids included, colour patches `patch-*` left out) at the head of `loft/<id>.json`; the first op of a scale replaces the filled body shapes the trunk bones had (not the patches, and only those of the thermal region of the first, so that a region keeps its shape) and lends their paint, and the `loft` ops of those shapes leave the list. For a trunk whose volume is skinned over two bones (`dairy-cow`, `horse`) or made of several solids per bone."""
import json, os, pathlib, sys
W = pathlib.Path(__file__).resolve().parent
GEN = W.parent.parent / "🗑️generated" / "matrix" / "MA"
TRUNK = os.environ.get("TRUNK", "pelvis lumbar thorax").split()
for a in sys.argv[1:]:
    asset = json.load(open(GEN / "backup" / f"{a}.json"))["asset"]
    path = W / "loft" / f"{a}.json"
    ops = [op for op in json.load(open(path)) if op.get("op") != "body"]
    made = []
    for scale, rep in asset["representations"].items():
        volume = asset.get("volumes", {}).get(scale, {})
        if rep.get("status") != "authored" or volume.get("status") != "authored":
            continue
        solids = [s for s in volume["body"] if s.get("bone") in TRUNK and s["kind"] == "loft" and "transform" not in s and "repeat" not in s and not s["id"].startswith("patch-")]
        old = []
        for view in ("elevation", "front", "plan"):
            for shape in rep["geometry"][view]:
                if shape["bone"] in TRUNK and shape.get("role") == "body" and shape.get("filled") and not shape["id"].startswith("patch-") and shape["id"] not in old:
                    old.append(shape["id"])
        side = [s["id"] for s in rep["geometry"]["elevation"] if s["id"] in old]
        region = next((s.get("region") for s in rep["geometry"]["elevation"] if side and s["id"] == side[0]), None)
        old = [i for i in old if all(s.get("region") == region for v in ("elevation", "front", "plan") for s in rep["geometry"][v] if s["id"] == i and s["bone"] in TRUNK)]
        if not solids or not side:
            continue
        for k, solid in enumerate(solids):
            made.append({"op": "body", "scale": scale, "solid": solid["id"], "paint": side[0] if k == 0 else solids[0]["id"], **({"replace": old} if k == 0 else {})})
        ops = [op for op in ops if not (op.get("scale") == scale and op.get("bone") in TRUNK and op.get("id") in old)]
    path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in made + ops) + "\n]\n")
    print(a, len(made), "body ops:", ", ".join(f"1:{op['scale']} {op['solid']}" for op in made)[:300])
