p = "spiral_stair.py"
s = open(p, encoding="utf8").read()
a = s.index("def turn_bone")
b = s.index("    wx = next")
new = '''def turn_bone():
    return {"id": "walker-turn", "parent": COL, "title": {"en": "Walker azimuth round the column (massless)", "de": "Personenazimut um die Säule (masselos)"}, "x": 0.0, "y": 0.0, "angleDeg": 0, "lengthM": 0.0,
            "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": 360}}


def azimuth_offsets(theta, dy):
    return {"walker-turn": {"twistDeg": round(theta, 5)}, "walker-x": {"dy": -WALK_R}, "walker-y": {"dy": round(dy, 5)}}


def patch_walker(asset):
    bones = asset["rig"]["bones"]
    i = next(k for k, b in enumerate(bones) if b["id"] == "walker-x")
    bones.insert(i, turn_bone())
'''
s = s[:a] + new + s[b:]
s = s.replace('wx["parent"] = "walker-turn-b"', 'wx["parent"] = "walker-turn"')
open(p, "w", encoding="utf8").write(s)
