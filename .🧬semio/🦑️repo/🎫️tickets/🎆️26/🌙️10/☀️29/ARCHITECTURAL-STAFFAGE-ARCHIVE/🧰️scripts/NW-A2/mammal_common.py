"""shared mammal helpers: three-segment limb IK against a partial pose, fur regions"""
import math
from nw import *


def attach(asset, offsets, root, bone_id):
    W = fk(asset, offsets, root)
    m = W[bone_id]
    return (m[4], m[5]), world_angle(m)


def limb(asset, offsets, root, names, lens, tip, end_deg, bend=1):
    """3-bone limb: bones names=[a,b,c]; its first bone origin is fixed by `offsets`/`root`; the last bone points at end_deg (world) and its tip reaches `tip`.
    Returns the offsets of the three bones."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    parent_deg = world_angle(fk(asset, offsets, root)[bones[names[0]]["parent"]])
    origin = (fk(asset, offsets, root)[names[0]][4], fk(asset, offsets, root)[names[0]][5])
    wrist = (tip[0] - lens[2]*math.cos(math.radians(end_deg)), tip[1] - lens[2]*math.sin(math.radians(end_deg)))
    t1, t2 = two_link(origin, wrist, lens[0], lens[1], bend)
    return offsets_from_world(asset, parent_deg, names, [t1, t2, end_deg])


def tip_world(asset, offsets, root, bone_id, length, local=(0.0, 0.0)):
    W = fk(asset, offsets, root)
    return apply(W[bone_id], (length + local[0], local[1]))


def fur_layers(prefix, R, th, cap, vref, ref="scholander-1950"):
    vr = lambda r: round(r / (0.0165 * 0.5), 2)
    return [layer(prefix + "-underfur", ("Underfur with enclosed air", "Unterfell mit eingeschlossener Luft"), "fur", vref, round(th*0.6, 4), round(R*0.65, 3), vr(R*0.65), round(cap*0.4), "authored-estimate", ref),
            layer(prefix + "-guard", ("Guard hair layer", "Deckhaarschicht"), "fur", vref, round(th*0.4, 4), round(R*0.35, 3), vr(R*0.35), round(cap*0.6), "authored-estimate", ref)]


def sumshare(regs):
    s1 = sum(r["body"]["metabolicShare"] for r in regs)
    regs[0]["body"]["metabolicShare"] = round(regs[0]["body"]["metabolicShare"] + (1 - s1), 4)
    s2 = sum(r["body"]["skinFraction"] for r in regs)
    regs[0]["body"]["skinFraction"] = round(regs[0]["body"]["skinFraction"] + (1 - s2), 5)
    return regs
