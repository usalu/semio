import sys
sys.stdout.reconfigure(encoding="utf8")
S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/"


def sub(path, old, new):
    s = open(S + path, encoding="utf8", newline="").read()
    assert s.count(old) == 1, (path, old[:60], s.count(old))
    open(S + path, "w", encoding="utf8", newline="").write(s.replace(old, new))


sub("🔨️modules/🦴️rig/🟦️.ts", "clamp(offset?.twistDeg ?? 0, -180, 180), dx: 0, dy: 0 };", "clamp(offset?.twistDeg ?? 0, joint.twistMinDeg ?? -180, joint.twistMaxDeg ?? 180), dx: 0, dy: 0 };")
sub("🔨️modules/🦴️rig/🟦️.ts", "`abductionDeg` and `twistDeg` to ±180°;", "`abductionDeg` to ±180° and `twistDeg` to `twistMinDeg..twistMaxDeg` (default ±180°, whole turns where the joint says so);")
sub("🔨️modules/🗂️catalog/🟦️.ts", 'else if (joint.type === "revolute" && Math.abs(offset.twistDeg ?? 0) > 180) bad(`Twist ${offset.twistDeg} deg of bone "${id}" leaves -180..180 deg.`, `Verdrehung ${offset.twistDeg} Grad von Knochen "${id}" verlässt -180..180 Grad.`);',
    'else if (joint.type === "revolute" && Math.abs((offset.twistDeg ?? 0) - effective.twistDeg) > ANGLE_EPS) bad(`Twist ${offset.twistDeg} deg of bone "${id}" leaves ${joint.twistMinDeg ?? -180}..${joint.twistMaxDeg ?? 180} deg.`, `Verdrehung ${offset.twistDeg} Grad von Knochen "${id}" verlässt ${joint.twistMinDeg ?? -180}..${joint.twistMaxDeg ?? 180} Grad.`);')
sub("🧬️schema/🟦️.ts", "  readonly maxDeg?: number;\n  readonly axisDeg?: number;", "  readonly maxDeg?: number;\n  readonly twistMinDeg?: number;\n  readonly twistMaxDeg?: number;\n  readonly axisDeg?: number;")
sub("🧬️schema/🔣️.json", '        "maxDeg": {\n          "type": "number"\n        },\n        "axisDeg": {', '        "maxDeg": {\n          "type": "number"\n        },\n        "twistMinDeg": {\n          "type": "number",\n          "minimum": -720,\n          "maximum": 0\n        },\n        "twistMaxDeg": {\n          "type": "number",\n          "minimum": 0,\n          "maximum": 720\n        },\n        "axisDeg": {')
sub("🧬️schema/🔣️.json", '        "twistDeg": {\n          "type": "number",\n          "minimum": -180,\n          "maximum": 180\n        },', '        "twistDeg": {\n          "type": "number",\n          "minimum": -720,\n          "maximum": 720\n        },')
print("ok")
