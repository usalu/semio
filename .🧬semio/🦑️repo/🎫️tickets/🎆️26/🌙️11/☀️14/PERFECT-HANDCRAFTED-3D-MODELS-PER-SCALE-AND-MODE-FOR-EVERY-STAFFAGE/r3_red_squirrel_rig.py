"""🐿 Round 3 red squirrel: the corrected rig (bone lengths and joint positions of Sciurus vulgaris), the anchors and every pose, authored by hand as trunk angles and limb targets and solved for the joint angles.

Trunk bones run on the centre line of the trunk masses (the vertebral column lies dorsal of it and is modelled in `structure`); limb bones are the long bones, joint to joint. Angles are absolute sagittal angles (0 ahead, 90 up) here and turned into the offsets the asset stores.
"""
import math
from r3_red_squirrel_fk import chain, at, tip, solve2, sc

REST = {"spine-r": 0, "spine-f": 0, "neck": 15, "head": -25, "tail-1": 160, "tail-2": 0, "arm": -90, "forearm": 20, "hand": 40, "thigh": -60, "shank": -60, "foot": 80}
LEN = {"spine-r": 0.072, "spine-f": 0.062, "neck": 0.024, "head": 0.052, "tail-1": 0.085, "tail-2": 0.09, "arm": 0.04, "forearm": 0.04, "hand": 0.026, "thigh": 0.05, "shank": 0.058, "foot": 0.05}
SHOULDER = (0.052, -0.012, 0.015)
HIP = (0.016, 0.002, 0.013)
LIMITS = {"spine-r": (-90, 120), "spine-f": (-90, 90), "neck": (-80, 80), "head": (-90, 90), "tail-1": (-140, 140), "tail-2": (-150, 150)}


def bones():
    """The rig bones in file order with rest placement (titles, joints and masses are merged from the asset file by the build)."""
    out = [
        dict(id="spine-r", parent=None, x=0, y=0, angleDeg=0, lengthM=LEN["spine-r"]),
        dict(id="spine-f", parent="spine-r", x=LEN["spine-r"], y=0, angleDeg=0, lengthM=LEN["spine-f"]),
        dict(id="neck", parent="spine-f", x=LEN["spine-f"], y=0.006, angleDeg=15, lengthM=LEN["neck"]),
        dict(id="head", parent="neck", x=LEN["neck"], y=0, angleDeg=-25, lengthM=LEN["head"]),
        dict(id="tail-1", parent="spine-r", x=-0.004, y=0.012, angleDeg=160, lengthM=LEN["tail-1"]),
        dict(id="tail-2", parent="tail-1", x=LEN["tail-1"], y=0, angleDeg=0, lengthM=LEN["tail-2"]),
    ]
    for side, sign in (("n", -1), ("f", 1)):
        out += [
            dict(id=f"arm-{side}", parent="spine-f", x=SHOULDER[0], y=SHOULDER[1], z=sign * SHOULDER[2], angleDeg=-90, lengthM=LEN["arm"]),
            dict(id=f"forearm-{side}", parent=f"arm-{side}", x=LEN["arm"], y=0, angleDeg=20, lengthM=LEN["forearm"]),
            dict(id=f"hand-{side}", parent=f"forearm-{side}", x=LEN["forearm"], y=0, angleDeg=40, lengthM=LEN["hand"]),
            dict(id=f"thigh-{side}", parent="spine-r", x=HIP[0], y=HIP[1], z=sign * HIP[2], angleDeg=-60, lengthM=LEN["thigh"]),
            dict(id=f"shank-{side}", parent=f"thigh-{side}", x=LEN["thigh"], y=0, angleDeg=-60, lengthM=LEN["shank"]),
            dict(id=f"foot-{side}", parent=f"shank-{side}", x=LEN["shank"], y=0, angleDeg=80, lengthM=LEN["foot"]),
        ]
    order = ["spine-r", "spine-f", "neck", "head", "tail-1", "tail-2", "arm-n", "forearm-n", "hand-n", "thigh-n", "shank-n", "foot-n", "arm-f", "forearm-f", "hand-f", "thigh-f", "shank-f", "foot-f"]
    return sorted(out, key=lambda b: order.index(b["id"]))


BONES = bones()

ANCHORS = {
    "ground-foot": dict(bone="foot", x=0.04, y=-0.0049, footprintLengthM=0.05, footprintWidthM=0.013),
    "ground-hand": dict(bone="hand", x=0.016, y=-0.0035, footprintLengthM=0.022, footprintWidthM=0.01),
    "bark-foot": dict(bone="foot", x=0.052, y=-0.002, footprintLengthM=0.014, footprintWidthM=0.012),
    "bark-hand": dict(bone="hand", x=0.027, y=-0.0015, footprintLengthM=0.01, footprintWidthM=0.009),
    "nostrils": dict(bone="head", x=0.0548, y=0.0012),
    "snout-tip": dict(bone="head", x=0.0546, y=-0.001),
    "eye": dict(bone="head", x=0.0275, y=0.0098),
    "back-plume": dict(bone="spine-r", x=0.04, y=0.0329),
}


def anchor_list():
    """Anchors in the flat form the kinematics reads: id, bone, x, y."""
    out = []
    for key, a in ANCHORS.items():
        if a["bone"] in ("foot", "hand"):
            for side in ("n", "f"):
                out.append(dict(id=f"{key}-{side}", bone=f"{a['bone']}-{side}", x=a["x"], y=a["y"]))
        else:
            out.append(dict(id=key, bone=a["bone"], x=a["x"], y=a["y"]))
    return out


def ang(v):
    return math.degrees(math.atan2(v[1], v[0]))


def wrap(d):
    return (d + 180.0) % 360.0 - 180.0


def trunk(sr, sf, nk, hd, t1, t2):
    """Offsets of the six trunk bones from their absolute sagittal angles."""
    return {
        "spine-r": {"angleDeg": sr},
        "spine-f": {"angleDeg": sf - sr},
        "neck": {"angleDeg": wrap(nk - sf - 15)},
        "head": {"angleDeg": wrap(hd - nk + 25)},
        "tail-1": {"angleDeg": wrap(t1 - sr - 160)},
        "tail-2": {"angleDeg": wrap(t2 - t1)},
    }


def limb(off, side, kind, target, guess, end_abs, abd=(0, 0, 0), anchor=None, root=(0, 0, 0)):
    """Solves one limb: the two long bones so that the wrist or ankle (or, with `anchor`, that anchor of the hand or foot) reaches `target` in the side view, and the hand or foot so that it points along `end_abs`.

    `guess` are the absolute angles of the two long bones that pick the bend; `abd` the abduction of the three bones (positive towards the left, +z)."""
    names = [f"{n}-{side}" for n in (("arm", "forearm", "hand") if kind == "arm" else ("thigh", "shank", "foot"))]
    parent_abs = off["spine-f" if kind == "arm" else "spine-r"]["angleDeg"] + (off["spine-r"]["angleDeg"] if kind == "arm" else 0)
    rest = [REST[n.rsplit("-", 1)[0]] for n in names]
    for n, a in zip(names, abd):
        off[n] = {"angleDeg": 0.0}
        if a:
            off[n]["abductionDeg"] = a

    def end_dir():
        return ang(chain(BONES, off, root)[names[2]]["u"])

    def set_end():
        for _ in range(40):
            err = wrap(end_abs - end_dir())
            if abs(err) < 1e-6:
                break
            off[names[2]]["angleDeg"] += err

    def reach(a, b):
        off[names[0]]["angleDeg"], off[names[1]]["angleDeg"] = a, b
        f = chain(BONES, off, root)
        if anchor is None:
            p = tip(f[names[1]])
        else:
            set_end()
            f = chain(BONES, off, root)
            an = ANCHORS[anchor]
            p = at(f[names[2]], an["x"], an["y"])
        return p[0], p[1]

    g0 = guess[0] - parent_abs - rest[0]
    g1 = guess[1] - guess[0] - rest[1]
    a, b = solve2(reach, target, (g0, g1))
    off[names[0]]["angleDeg"], off[names[1]]["angleDeg"] = wrap(a), wrap(b)
    set_end()
    off[names[2]]["angleDeg"] = wrap(off[names[2]]["angleDeg"])
    got = reach(off[names[0]]["angleDeg"], off[names[1]]["angleDeg"])
    if abs(got[0] - target[0]) + abs(got[1] - target[1]) > 1e-5:
        raise SystemExit(f"limb {names[0]} does not reach {target}: {got}")
    for n in names:
        if abs(off[n]["angleDeg"]) > 170:
            raise SystemExit(f"{n} offset {off[n]['angleDeg']:.1f} beyond the joint limit")
    return off


def fk(off, root=(0, 0, 0)):
    return chain(BONES, off, root)


def joint(off, bone, root=(0, 0, 0)):
    return fk(off, root)[bone]["o"]


def add(p, d):
    return (p[0] + d[0], p[1] + d[1])


def sitting(head=-22.0, near_hand=62.0, far_hand=50.0):
    """Sitting upright on the haunches, both hands at the mouth: hind feet flat ahead of the hips, knees up beside the belly and spread, elbows low, the tail up along the back with the tip curling away."""
    off = trunk(42, 80, 52, head, 70, 104)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        limb(off, side, "leg", add(hip, (0.014, -0.0245)), (32, -118), -4.57, abd=(s * 12, 0, 0))
        mouth = at(fk(trunk(42, 80, 52, -22.0, 70, 104))["head"], 0.045, -0.0105)
        hand_abs = near_hand if side == "n" else far_hand
        tipp = add(mouth, (0.001, -0.0035) if side == "n" else (-0.0015, -0.006))
        c, s_ = math.cos(math.radians(hand_abs)), math.sin(math.radians(hand_abs))
        wrist = (tipp[0] - 0.0245 * c, tipp[1] - 0.0245 * s_)
        limb(off, side, "arm", wrist, (-112, 28), hand_abs, abd=(s * 6, -s * 12, s * 10))
    return off


def gather():
    """The gather of the bound: hind feet planted flat under the chest, the back arched, the fore limbs reaching ahead, the tail streaming behind."""
    off = trunk(30, -8, 12, -8, 162, 152)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        d = (0.0, 0.0) if side == "n" else (-0.006, 0.0)
        limb(off, side, "leg", add(add(hip, (0.0247, -0.0613)), d), (-10, -115), -4.57, abd=(s * 15, 0, 0))
        sh = joint(off, f"arm-{side}")
        limb(off, side, "arm", add(sh, (0.071, 0.004) if side == "n" else (0.066, -0.006)), (12, -14), -28, abd=(s * 6, -s * 6, 0))
    return off


def push():
    """Push-off: the hind legs extend behind the hips, heels raised, the body stretches forward and up, the hands reach ahead."""
    off = trunk(10, -3, 6, 0, 172, 184)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        d = (0.0, 0.0) if side == "n" else (-0.003, 0.0)
        limb(off, side, "leg", add(add(hip, (-0.034, -0.09)), d), (-100, -135), -15, abd=(s * 8, -s * 8, 0), anchor="ground-foot")
        sh = joint(off, f"arm-{side}")
        limb(off, side, "arm", add(sh, (0.072, 0.018) if side == "n" else (0.068, 0.01)), (27, 2), -10, abd=(s * 5, -s * 5, 0))
    return off


def flight():
    """Flight: the body fully stretched, the fore limbs reaching down ahead for the landing, the hind legs trailing."""
    off = trunk(3, -3, 4, -2, 170, 178)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        limb(off, side, "leg", add(hip, (-0.097, -0.008) if side == "n" else (-0.094, -0.014)), (-172, -176), -158, abd=(s * 6, -s * 6, 0))
        sh = joint(off, f"arm-{side}")
        limb(off, side, "arm", add(sh, (0.064, -0.035) if side == "n" else (0.06, -0.041)), (-22, -36), -42, abd=(s * 5, -s * 5, 0))
    return off


def extension():
    """Extension: the hands touch down ahead, the chest low, the hind feet still trailing off the ground."""
    off = trunk(-3, -11, 14, -2, 166, 174)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        limb(off, side, "leg", add(hip, (-0.095, -0.02) if side == "n" else (-0.092, -0.027)), (-150, -166), -140, abd=(s * 6, -s * 6, 0))
        sh = joint(off, f"arm-{side}")
        d = (0.0, 0.0) if side == "n" else (-0.005, 0.0)
        limb(off, side, "arm", add(add(sh, (0.047, -0.0725)), d), (-55, -80), -6.6, abd=(s * 5, -s * 5, 0), anchor="ground-hand")
    return off


def support():
    """Mid fore-stance: the hands planted under the shoulders carry the body, the hind legs pass forward tightly flexed with the toes pointing down."""
    off = trunk(8, -13, 11, -3, 162, 171)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        limb(off, side, "leg", add(hip, (-0.02, -0.037) if side == "n" else (-0.023, -0.04)), (-40, -175), -90, abd=(s * 8, -s * 4, 0))
        sh = joint(off, f"arm-{side}")
        d = (0.0, 0.0) if side == "n" else (-0.005, 0.0)
        limb(off, side, "arm", (0.118 + d[0], sh[1] - 0.074), (-78, -100), -6.6, abd=(s * 5, -s * 5, 0), anchor="ground-hand")
    return off


def swing():
    """Swing: airborne after the hands pushed off behind the shoulders; the back arches and the hind legs swing forward, tucked."""
    off = trunk(20, -15, 8, -5, 158, 168)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        limb(off, side, "leg", add(hip, (0.034, -0.04) if side == "n" else (0.028, -0.043)), (-12, -108), -28, abd=(s * 12, 0, 0))
        sh = joint(off, f"arm-{side}")
        limb(off, side, "arm", (0.068 if side == "n" else 0.064, sh[1] - (0.064 if side == "n" else 0.061)), (-125, -100), -50, abd=(s * 4, -s * 4, 0), anchor="ground-hand")
    return off


def reach():
    """Mid air after the hands pushed off: the elbows fold and the forearms swing forward under the chest, the hind feet reach forward for the landing."""
    off = trunk(25, -12, 10, -6, 160, 160)
    for side, s in (("n", -1), ("f", 1)):
        hip = joint(off, f"thigh-{side}")
        limb(off, side, "leg", add(hip, (0.032, -0.05) if side == "n" else (0.027, -0.052)), (-10, -110), -15, abd=(s * 13, 0, 0))
        sh = joint(off, f"arm-{side}")
        limb(off, side, "arm", add(sh, (0.03, -0.022) if side == "n" else (0.026, -0.027)), (-62, 28), -12, abd=(s * 5, -s * 5, 0))
    return off


BARK_X = 0.038


def climb(rise, near_hand=0.0, far_hand=0.0, far_off=0.0):
    """On a vertical trunk whose bark is the plane x = BARK_X: the body upright along it, limbs spread, the claws of hands and hind feet in the bark; `rise` lifts the body over the planted hind feet, `near_hand` and `far_hand` move the grips up, `far_off` holds the far hand off the bark."""
    off = trunk(84, 91, 90, 98, -106, -93)
    root = (0.0, rise, 0.0)
    for side, s in (("n", -1), ("f", 1)):
        d = 0.0 if side == "n" else 0.012
        limb(off, side, "leg", (BARK_X, 0.016 + 0.046 + d), (48 - rise * 500, -62 - rise * 300), 86, abd=(s * 38, s * 18, -s * 16), anchor="bark-foot", root=root)
        up = (near_hand if side == "n" else far_hand) + (0.0 if side == "n" else -0.014)
        away = 0.0 if side == "n" else far_off
        limb(off, side, "arm", (BARK_X - away, 0.016 + 0.19 + up), (128, 40), 86 + away * 900, abd=(s * 30, s * 6, 0), anchor="bark-hand", root=root)
    return off


def poses(only=None):
    """Every pose of the asset: id -> (offsets, root, ground, contacts)."""
    feet = ["ground-foot-n", "ground-foot-f"]
    hands = ["ground-hand-n", "ground-hand-f"]
    bark = ["bark-hand-n", "bark-hand-f", "bark-foot-n", "bark-foot-f"]
    table = {
        "sitting-eating": (sitting, None, "auto", feet),
        "running": (gather, None, "auto", feet),
        "climbing": (lambda: climb(0.0), 0.2, "authored", bark),
        "nibble-b": (lambda: sitting(head=-28.0, near_hand=74.0, far_hand=40.0), None, "auto", feet),
        "bound-extension": (extension, None, "auto", hands),
        "bound-support": (support, None, "auto", hands),
        "bound-swing": (swing, None, "auto", hands),
        "bound-reach": (reach, "air", "authored", []),
        "bound-gather": (gather, None, "auto", feet),
        "bound-push": (push, None, "auto", feet),
        "bound-flight": (flight, "air", "authored", []),
        "climb-a": (lambda: climb(0.0), 0.2, "authored", bark),
        "climb-b": (lambda: climb(0.024, near_hand=0.0, far_hand=0.04, far_off=0.008), 0.2, "authored", bark),
        "climb-c": (lambda: climb(0.048, near_hand=0.048, far_hand=0.048), 0.2, "authored", bark),
    }
    return {k: (v[0](), v[1], v[2], v[3]) for k, v in table.items() if only is None or k in only}


AIR = {"bound-reach": ("ground-foot", 0.014), "bound-flight": ("ground-hand", 0.066)}


def lowest(off, names):
    f = fk(off)
    by = {a["id"]: a for a in anchor_list()}
    return min(at(f[by[n]["bone"]], by[n]["x"], by[n]["y"])[1] for n in names)


def root_y(pid, off, root, ground, contacts):
    """The root height of a pose: grounded on its contacts, the authored trunk height for the climb, or for the two airborne keys of the bound the height that holds the named anchor at its clearance."""
    if pid.startswith("climb"):
        return root
    if root == "air":
        key, clearance = AIR[pid]
        return clearance - lowest(off, [f"{key}-n", f"{key}-f"])
    return -lowest(off, contacts)


if __name__ == "__main__":
    import sys
    for pid, (off, root, ground, contacts) in poses(sys.argv[1:] or None).items():
        y = root_y(pid, off, root, ground, contacts)
        rise = 0.0
        if pid.startswith("climb-"):
            rise = {"a": 0.0, "b": 0.024, "c": 0.048}[pid[-1]]
        f = fk(off, (0.0, y + rise, 0.0))
        print("==", pid, "root y %.4f" % (y + rise))
        for b in BONES:
            o, t = f[b["id"]]["o"], tip(f[b["id"]])
            print(f"  {b['id']:10s} o {o[0]:+.4f} {o[1]:+.4f} {o[2]:+.4f} tip {t[0]:+.4f} {t[1]:+.4f} {t[2]:+.4f}  off {off[b['id']].get('angleDeg', 0):+8.2f} abd {off[b['id']].get('abductionDeg', 0):+6.1f}")
        for a in anchor_list():
            p = at(f[a["bone"]], a["x"], a["y"])
            print(f"  @{a['id']:14s} {p[0]:+.4f} {p[1]:+.4f} {p[2]:+.4f}")
