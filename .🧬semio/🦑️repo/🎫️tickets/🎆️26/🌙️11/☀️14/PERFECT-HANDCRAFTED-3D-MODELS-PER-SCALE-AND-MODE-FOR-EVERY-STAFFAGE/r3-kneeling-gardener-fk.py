"""🦿 Sagittal forward kinematics of `humans/kneeling-gardener` for the root fixes: world joint positions and bone directions of a pose from the rig as written in an asset dictionary.

Usage: python3 r3-kneeling-gardener-fk.py [pose]   (prints wrists, hands, trowel and thigh frames of every pose or of one)
"""
import json
import math
import sys

ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/kneeling-gardener/🔣️.json"


def frames(asset, pose):
    """World origin (x, y) and absolute angle in degrees of every bone in a pose."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    offsets = pose["bones"]
    out = {}

    def solve(bone_id):
        if bone_id in out:
            return out[bone_id]
        bone = bones[bone_id]
        delta = offsets.get(bone_id, {})
        if bone["parent"] is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = solve(bone["parent"])
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        x = px + bone["x"] * c - bone["y"] * s
        y = py + bone["x"] * s + bone["y"] * c
        joint = bone["joint"]
        if joint["type"] == "prismatic":
            axis = math.radians(pa + joint["axisDeg"])
            travel = delta.get("dx", 0.0)
            x += travel * math.cos(axis)
            y += travel * math.sin(axis)
        angle = pa + bone["angleDeg"] + delta.get("angleDeg", 0.0)
        out[bone_id] = (x, y, angle)
        return out[bone_id]

    for bone_id in bones:
        solve(bone_id)
    return out


def point(frame, u, v):
    x, y, a = frame
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (x + u * c - v * s, y + u * s + v * c)


if __name__ == "__main__":
    asset = json.load(open(ASSET, encoding="utf-8"))["asset"]
    for pose in asset["poses"]:
        if len(sys.argv) > 1 and pose["id"] != sys.argv[1]:
            continue
        f = frames(asset, pose)
        print(pose["id"])
        for bone_id in ("torso", "head", "upper-arm-l", "forearm-l", "hand-l", "upper-arm-r", "forearm-r", "hand-r", "trowel", "thigh-l", "shank-l"):
            x, y, a = f[bone_id]
            print("  %-12s o %.4f %.4f angle %.2f" % (bone_id, x, y, (a + 180) % 360 - 180))
        print("  trowel tip %.4f %.4f  palm-l %.4f %.4f" % (*point(f["trowel"], 0.3, 0), *point(f["hand-l"], 0.09, -0.03)))
