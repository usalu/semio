import os

root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/"
T = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️TS/"
p = root + "🔨️modules/🧮️evaluation/🟦️.ts"
s = open(p, encoding="utf-8", newline="").read()
nl = "\r\n" if "\r\n" in s else "\n"
s = s.replace("\r\n", "\n")


def cut(start_key, end_key, end_is_code=False):
    global s
    i = s.rfind("/**", 0, s.index(start_key))
    j = s.index(end_key, i) if end_is_code else s.rfind("/**", 0, s.index(end_key, i))
    s = s[:i] + s[j:]


cut("The vertical placement of a front shape", "const CIRCLE_SEGMENTS", True)
cut("The front or plan rings of a {@link Solid}", "The current and the rest 3D frames")
cut("A front or plan shape moved sideways by the lateral spread", "function placeShape(", True)
i = s.index("function placeShape(")
j = s.index("function anchorOf(")
s = s[:i] + open(T + "place-new.ts.txt", encoding="utf-8").read() + s[j:]


def sub(a, b):
    global s
    assert a in s, a[:90]
    s = s.replace(a, b, 1)


sub('placeShape(asset, shape, world, "front", geometry.paper, spread ?? undefined, kin)', 'placeShape(asset, shape, world, "front", geometry.paper, kin)')
sub('placeShape(asset, shape, world, "plan", geometry.paper, spread ?? undefined, kin)', 'placeShape(asset, shape, world, "plan", geometry.paper, kin)')
i = s.index("  const frames = solveFrames(asset.rig, sample.offsets, sample.root);")
j = s.index("\n", s.index("const kin: Chain3", i))
s = s[:i] + "  const kin: Chain3 = { now: solveChain(asset.rig, sample.offsets, sample.root), rest: restOf(asset) };" + s[j:]
sub("import { apply, carry, effectiveOffset, restFrames, rotationRad, sinCosDeg, solveFrames, solveRig } from", "import { apply, carry, effectiveOffset, restFrames, rotationRad, sinCosDeg, solveChain, solveRig } from")
sub("import type { BoneSpread, Frame3, SolvedFrame } from", "import type { Frame3, SolvedFrame } from")
open(p + ".tmp", "w", encoding="utf-8", newline="").write(s.replace("\n", nl))
os.replace(p + ".tmp", p)

q = root + "🔨️modules/🦴️rig/🟦️.ts"
r = open(q, encoding="utf-8", newline="").read()
rnl = "\r\n" if "\r\n" in r else "\n"
r = r.replace("\r\n", "\n")
a = "/** 🧊️ The 3D frames of the rest pose with the root at the origin"
assert a in r
r = r.replace(a, "/** 🧊️ The three-dimensional chain of every rig, spread or not: the reference of the single front and plan placement. */\nexport function solveChain(rig: Rig, offsets: Readonly<Record<Slug, BoneOffset>>, root: { readonly x: number; readonly y: number; readonly angleDeg: number }): Record<Slug, SolvedFrame> {\n  return chain(rig, offsets, root);\n}\n\n" + a, 1)
open(q + ".tmp", "w", encoding="utf-8", newline="").write(r.replace("\n", rnl))
os.replace(q + ".tmp", q)
