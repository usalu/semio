import json, sys
sys.stdout.reconfigure(encoding="utf8")
S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/"


def sub(path, old, new):
    s = open(S + path, encoding="utf8", newline="").read()
    assert s.count(old) == 1, (path, old[:70], s.count(old))
    open(S + path, "w", encoding="utf8", newline="").write(s.replace(old, new))


FX = S + "🧫️fixtures/🧊️axial-twist/🔣️.json"
raw = open(FX, encoding="utf8", newline="").read()
d = json.loads(raw)
d["poses"]["turn-270-full"] = {"bones": {"torso": {"twistDeg": 270}, "arm-l": {"angleDeg": 90}}}
d["poses"]["turn-360-full"] = {"bones": {"torso": {"twistDeg": 360}, "arm-l": {"angleDeg": 90}}}
d["frames"] += [
    {"name": "arm at torso twist 270 (a joint with whole turns)", "pose": "turn-270-full", "bone": "arm-l", "originWorld": {"x": 0, "y": 1.48}, "tipWorld": {"x": 0, "y": 1.48}, "originZ": 0, "tipZ": -0.6},
    {"name": "arm at torso twist 360 (a joint with whole turns)", "pose": "turn-360-full", "bone": "arm-l", "originWorld": {"x": 0, "y": 1.48}, "tipWorld": {"x": 0.6, "y": 1.48}, "originZ": 0, "tipZ": 0},
]
d["limits"] += [
    {"name": "a twist of 300 degrees is allowed on a joint with twistMaxDeg 360", "bone": "torso", "joint": "revolute", "twistMaxDeg": 360, "twistDeg": 300, "refused": False},
    {"name": "a twist above twistMaxDeg is refused", "bone": "torso", "joint": "revolute", "twistMaxDeg": 360, "twistDeg": 370, "refused": True},
    {"name": "a twist of -300 degrees is allowed on a joint with twistMinDeg -360", "bone": "torso", "joint": "revolute", "twistMinDeg": -360, "twistDeg": -300, "refused": False},
    {"name": "a twist of -190 degrees is refused when twistMinDeg is not given", "bone": "torso", "joint": "revolute", "twistDeg": -190, "refused": True},
]
d["fullTurnJoint"] = {"bone": "torso", "twistMinDeg": -360, "twistMaxDeg": 360}
json.dump(d, open(FX, "w", encoding="utf8", newline=""), ensure_ascii=False, indent=2)
open(FX, "a", encoding="utf8", newline="").write("\n" if raw.endswith("\n") else "")

T = "🧪️tests/🧊️axial-twist/🟦️.ts"
sub(T, 'limits: { name: string; bone: string; joint: "revolute" | "fixed" | "prismatic"; twistDeg: number; refused: boolean }[];', 'limits: { name: string; bone: string; joint: "revolute" | "fixed" | "prismatic"; twistMinDeg?: number; twistMaxDeg?: number; twistDeg: number; refused: boolean }[];\n  fullTurnJoint: { bone: string; twistMinDeg: number; twistMaxDeg: number };')
sub(T, "  return { ...sample, poses, representations:", "  const rig = { ...sample.rig, bones: sample.rig.bones.map((bone) => (bone.id === vectors.fullTurnJoint.bone ? { ...bone, joint: { ...bone.joint, twistMinDeg: vectors.fullTurnJoint.twistMinDeg, twistMaxDeg: vectors.fullTurnJoint.twistMaxDeg } } : bone)) };\n  return { ...sample, rig, poses, representations:")
sub(T, 'joint: limit.joint === "revolute" ? bone.joint :', 'joint: limit.joint === "revolute" ? { ...bone.joint, ...(limit.twistMinDeg === undefined ? {} : { twistMinDeg: limit.twistMinDeg }), ...(limit.twistMaxDeg === undefined ? {} : { twistMaxDeg: limit.twistMaxDeg }) } :')
F = "🧪️tests/🧊️axial-twist/🥒️.feature"
sub(F, "    Then a revolute joint accepts up to 180 degrees and fixed and prismatic joints and larger angles are refused\n", "    Then a revolute joint accepts up to 180 degrees unless it declares twistMinDeg and twistMaxDeg, and fixed and prismatic joints and larger angles are refused\n\n  Scenario: A joint may declare whole turns\n    Given a revolute joint with twistMinDeg -360 and twistMaxDeg 360 and a torso twisted 270 and 360 degrees with the arm raised forward\n    When the frames are solved\n    Then the arm reaches 0.6 m to the right at 270 degrees and 0.6 m ahead at 360 degrees\n")
print("ok")
