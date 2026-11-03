import os

p = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts"
s = open(p, encoding="utf-8", newline="").read()
nl = "\r\n" if "\r\n" in s else "\n"
s = s.replace("\r\n", "\n")


def sub(a, b):
    global s
    assert a in s, a[:90]
    s = s.replace(a, b, 1)


sub('import { apply, effectiveOffset, rotationRad, sinCosDeg, solveLateral, solveRig } from "../🦴️rig/🟦️.ts";\nimport type { BoneSpread } from "../🦴️rig/🟦️.ts";',
    'import { apply, carry, effectiveOffset, restFrames, rotationRad, sinCosDeg, solveFrames, solveRig } from "../🦴️rig/🟦️.ts";\nimport type { BoneSpread, Frame3, SolvedFrame } from "../🦴️rig/🟦️.ts";')

NEW = open("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🧰️TS/eval-new.ts.txt", encoding="utf-8").read()
marker = "/** 🪽️ A front or plan shape moved sideways by the lateral spread of its bone."
sub(marker, NEW + marker)

sub("identity: boolean, lateral: Readonly<Record<Slug, BoneSpread>> | undefined = undefined): EvaluatedShape {\n  const spread = lateral?.[shape.bone];",
    "identity: boolean, lateral: Readonly<Record<Slug, BoneSpread>> | undefined = undefined, kin: Chain3 | undefined = undefined): EvaluatedShape {\n  const spread = lateral?.[shape.bone];")
sub("  if (viewpoint !== \"elevation\" && shape.solid !== undefined) return",
    "  const carried = viewpoint === \"elevation\" || kin === undefined ? undefined : kin.now[shape.bone];\n  if (carried?.affected === true && kin !== undefined) {\n    if (shape.solid !== undefined) return { shape, rings: solid3(shape.solid, flat.rings, kin, shape.bone, viewpoint), closed: true };\n    if (!(\"followTip\" in shape && shape.followTip === true)) return { shape, rings: flat.rings.map((ring) => ring.map((p) => carryPoint(kin, shape.bone, viewpoint, p))), closed: flat.closed };\n  }\n  if (viewpoint !== \"elevation\" && shape.solid !== undefined) return")
sub('placeShape(asset, shape, world, "front", geometry.paper, spread ?? undefined)', 'placeShape(asset, shape, world, "front", geometry.paper, spread ?? undefined, kin)')
sub('placeShape(asset, shape, world, "plan", geometry.paper, spread ?? undefined)', 'placeShape(asset, shape, world, "plan", geometry.paper, spread ?? undefined, kin)')
sub("  const spread = solveLateral(asset.rig, sample.offsets, sample.root);",
    "  const frames = solveFrames(asset.rig, sample.offsets, sample.root);\n  const spread = frames === null ? null : lateralOf(frames);\n  const kin: Chain3 | undefined = frames === null ? undefined : { now: frames, rest: restOf(asset) };")
open(p + ".tmp", "w", encoding="utf-8", newline="").write(s.replace("\n", nl))
os.replace(p + ".tmp", p)
