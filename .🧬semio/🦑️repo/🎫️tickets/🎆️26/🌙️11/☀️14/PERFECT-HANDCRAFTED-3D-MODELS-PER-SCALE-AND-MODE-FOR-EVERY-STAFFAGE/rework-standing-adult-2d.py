"""✏️ standing-adult, second round: the filled body outlines of the 1:50 and 1:100 drawings (side elevation first, then front and plan, which are carried at the depth of their elevation twin) follow the rebuilt 3D body (round upper back, lumbar hollow, buttock, calf, hair on the skull).

The outlines are the orthographic outlines of each bone's body solids as `r3-project2d.ts --scale N --view V --candidate` rules them (quarter-feature simplification); every stroke, detail, finger capsule and the members of each shape (role, z, tone, region, hatch, pen) stay as drawn.

Usage: python3 rework-standing-adult-2d.py [--scale 50|100] [--view elevation|front|plan] [--apply]
"""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
rig = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rig)
SCALE = sys.argv[sys.argv.index("--scale") + 1] if "--scale" in sys.argv else "50"
VIEW = sys.argv[sys.argv.index("--view") + 1] if "--view" in sys.argv else "elevation"
RULED = os.path.join(T, "🗑️generated", "r3-project2d", "humans", "standing-adult.s%s.%s" % (SCALE, VIEW), "🔣️.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-adult", "draw", "🔣️.json")


def convert(asset):
    ruled = {(s["id"], s["bone"]): s for s in json.load(open(RULED, encoding="utf8"))["asset"]["representations"][SCALE]["geometry"][VIEW]}
    changed = []
    for shape in asset["representations"][SCALE]["geometry"][VIEW]:
        other = ruled.get((shape["id"], shape["bone"]))
        if other and shape.get("kind") == "poly" and other.get("kind") == "poly" and shape.get("filled") and shape["points"] != other["points"] and "solid" not in shape:
            shape["points"] = other["points"]
            changed.append(shape["id"])
    return changed


def main():
    original = text = open(rig.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"]["representations"])
    print(convert(document["asset"]))
    if "--apply" not in sys.argv:
        document["asset"]["volumes"] = json.load(open(rig.SOURCE, encoding="utf8"))["volumes"]
        os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
        json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
        print(CANDIDATE)
        return
    text = rig.splice(text, "representations", before, document["asset"]["representations"])
    if json.loads(text) != document or open(rig.CATALOG, encoding="utf8").read() != original:
        raise SystemExit("not written: splice did not round-trip or the file changed")
    open(rig.CATALOG, "w", encoding="utf8").write(text)
    print("applied to", rig.CATALOG)


if __name__ == "__main__":
    main()
