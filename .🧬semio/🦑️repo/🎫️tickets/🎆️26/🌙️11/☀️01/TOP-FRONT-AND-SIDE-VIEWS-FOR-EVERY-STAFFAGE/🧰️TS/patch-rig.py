import os
S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/"


def rw(p, f):
    p = S + p
    s = open(p, encoding="utf-8", newline="").read()
    nl = "\r\n" if "\r\n" in s else "\n"
    s = f(s.replace("\r\n", "\n"))
    open(p + ".tmp", "w", encoding="utf-8", newline="").write(s.replace("\n", nl))
    os.replace(p + ".tmp", p)


def schema_ts(s):
    a = "  readonly x: number;\n  readonly y: number;\n  readonly angleDeg: number;\n  readonly lengthM: number;\n  readonly joint: Joint;"
    assert a in s
    s = s.replace(a, "  readonly x: number;\n  readonly y: number;\n  readonly z?: number;\n  readonly angleDeg: number;\n  readonly lengthM: number;\n  readonly joint: Joint;", 1)
    old = "its local x axis runs along it. */\nexport type Bone"
    assert old in s
    return s.replace(old, "its local x axis runs along it. `z` is the lateral rest offset of the origin in the parent frame (left is +, default 0): the pivot of a spread or twist of a bone beside the symmetry plane (a wing root, a shoulder). */\nexport type Bone", 1)


rw("🧬️schema/🟦️.ts", schema_ts)


def schema_json(s):
    i = s.index('    "Bone": {')
    j = s.index('"angleDeg": {', i)
    return s[:j] + '"z": {\n          "type": "number"\n        },\n        ' + s[j:]


rw("🧬️schema/🔣️.json", schema_json)

TAIL = open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️TS/rig-tail.ts.txt", encoding="utf-8").read()


def rig(s):
    i = s.index("/** 🪽️ The lateral (left is +z) position of the origin")
    return s[:i] + TAIL


rw("🔨️modules/🦴️rig/🟦️.ts", rig)
