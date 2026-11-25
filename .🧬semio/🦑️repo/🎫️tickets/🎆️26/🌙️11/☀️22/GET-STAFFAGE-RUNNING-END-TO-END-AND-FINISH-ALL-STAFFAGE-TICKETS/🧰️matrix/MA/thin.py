"""📐 Slims the thinnest loft of the bones that `volumes-feature` names: thin.py <id>… reads the copy block of `$GEN/loft/<id>.measure.log` and sets `slim` (the thinness of the named solid less 0.1 mm, in metres) on the one loft op of that bone and scale in `loft/<id>.json` whose loft in the trial copy `$GEN/loft/<id>.json` has the least dimension (a thicker part of the same bone keeps its size), and on the same op of the mirrored bone."""
import json, re, sys, pathlib
W = pathlib.Path(__file__).resolve().parent
GEN = W.parent.parent / "🗑️generated" / "matrix" / "MA"
for a in sys.argv[1:]:
    asset = json.load(open(GEN / "backup" / f"{a}.json"))["asset"]
    path = W / "loft" / f"{a}.json"
    ops = json.load(open(path))
    text = (GEN / "loft" / f"{a}.measure.log").read_text()
    trial = json.load(open(GEN / "loft" / f"{a}.json"))["asset"]

    def least(op):
        for view in ("front", "plan", "elevation"):
            for shape in trial["representations"][op["scale"]]["geometry"][view]:
                if shape["id"] == (op.get(view) or op["id"]) and shape["bone"] == op["bone"] and shape.get("kind") == "loft":
                    return min(2 * max(q["rx"] for q in shape["sections"]), 2 * max(q["ry"] for q in shape["sections"]))
        return 9.0

    n = 0
    for scale, kind, index, value in re.findall(r"volumes-feature volumes\.(\d+)\.(\w+)\[(\d+)\].*? is ([\d.]+) mm thin", text):
        bone = asset["volumes"][scale][kind][int(index)]["bone"]
        other = {"n": "f", "f": "n"}.get(bone[-1])
        for twin in {bone, bone[:-1] + other if other else bone}:
            found = sorted((op for op in ops if op["op"] == "loft" and op["scale"] == scale and op["bone"] == twin), key=least)
            target = round(float(value) / 1000 - 0.0001, 5)
            if found and least(found[0]) < 9 and found[0].get("slim", 9) > target:
                found[0]["slim"] = target
                n += 1
    path.write_text("[\n" + ",\n".join(" " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
    print(a, n, "ops slimmed")
