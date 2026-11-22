"""💪 standing-adult, pass 1 second round: the 2D sleeves, forearms and cuffs of the side and front elevation were prisms (`extruded`), so an upper arm turned about its own axis (arms crossed, hands warming, shiver) drew as a hatched slab 0.13 m wide. An arm is round: side and front elevation now state the same body of revolution about the bone as the plan (half profile along the bone, `revolved` about `u`), which keeps its outline under every twist.

Usage: python3 rework-standing-adult-sleeves.py [--apply]   (without --apply: candidate `🗑️generated/rework-standing-adult/sleeves/🔣️.json` with the volumes source merged)
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
CANDIDATE = os.path.join(T, "🗑️generated", "rework-standing-adult", "sleeves", "🔣️.json")
BONES = ("arm-l", "arm-r", "forearm-l", "forearm-r")


def convert(asset):
    changed = []
    for scale, representation in asset["representations"].items():
        geometry = representation["geometry"]
        plan = {(s["id"], s["bone"]): s for s in geometry["plan"] if s.get("solid", {}).get("kind") == "revolved"}
        for view in ("elevation", "front"):
            for shape in geometry[view]:
                twin = plan.get((shape["id"], shape["bone"]))
                if twin and shape["bone"] in BONES and shape.get("solid", {}).get("kind") == "extruded" and shape.get("kind") == "poly":
                    shape["points"] = copy.deepcopy(twin["points"])
                    shape["solid"] = copy.deepcopy(twin["solid"])
                    changed.append("%s %s %s" % (scale, view, shape["id"]))
    return changed


def main():
    original = text = open(rig.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    before = copy.deepcopy(document["asset"]["representations"])
    changed = convert(document["asset"])
    print(len(changed), changed[:8])
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
