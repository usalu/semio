"""🐈 domestic-cat: the two walk keys at full retraction (`walk-phase-3` far limb, `walk-phase-7` near limb) whose limb could not reach with half the scapular swing keep the largest part of the reduction that still reaches (tried in the order of `KEEPS`). One-shot after `rework-domestic-cat-gait.py`: `python3 rework-domestic-cat-gait-reach.py --apply`."""
import copy
import importlib.util
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("gait", os.path.join(T, "rework-domestic-cat-gait.py"))
A = importlib.util.module_from_spec(spec)
spec.loader.exec_module(A)
KEEPS = (0.6, 0.7, 0.8, 0.9)
CASES = (("walk-phase-3", "f"), ("walk-phase-7", "n"))


def main():
    original = text = open(A.G.CATALOG, encoding="utf8").read()
    document = json.loads(text)
    asset = document["asset"]
    before = copy.deepcopy(asset["poses"])
    bones = asset["rig"]["bones"]
    limits = {bone["id"]: bone["joint"] for bone in bones}
    for pose_id, side in CASES:
        pose = next(pose for pose in asset["poses"] if pose["id"] == pose_id)
        if abs(pose["bones"][f"scapula-f{side}"]["angleDeg"]) < 15:
            raise SystemExit("already applied")
        for keep in KEEPS:
            A.KEEP = keep
            solved, miss, z0, z1 = A.resolve(bones, pose["bones"], side)
            inside = all(limits[bone]["minDeg"] <= angle <= limits[bone]["maxDeg"] for bone, angle in solved.items())
            print("[DEBUG]", pose_id, side, "keep", keep, "miss mm", round(miss * 1000, 2), "paw z mm before", round(z0 * 1000, 1), "now", round(z1 * 1000, 1), solved)
            if miss <= 0.0005 and inside:
                for bone, angle in solved.items():
                    pose["bones"][bone] = {"angleDeg": angle}
                break
    if "--apply" not in sys.argv:
        return
    text = A.K.splice(text, "poses", before, asset["poses"])
    if json.loads(text) != document or open(A.G.CATALOG, encoding="utf8").read() != original:
        raise SystemExit("not written")
    open(A.G.CATALOG, "w", encoding="utf8").write(text)
    print("applied")


if __name__ == "__main__":
    main()
