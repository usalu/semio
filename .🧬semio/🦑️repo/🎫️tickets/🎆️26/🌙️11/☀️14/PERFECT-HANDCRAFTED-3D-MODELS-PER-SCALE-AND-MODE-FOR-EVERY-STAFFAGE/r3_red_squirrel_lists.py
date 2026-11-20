"""🌡 Round 3 red squirrel: the thermal and the structural lists of both authored scales, stated by hand (millimetres in the bone frames).

Thermal: fur regions are the body surfaces by reference; at 1:50 every fur region carries its under-fur boundary as a nested shell (the body offset inwards by the guard-hair thickness). Bare skin is where the animal is bare: fingers, soles, the muzzle tip. Emitters are outlet patches on the surface.
Structure: the skeleton, never the body. 1:50: vertebral column dorsal in the trunk, rib hoops, sternum, innominate bones, scapulae, skull and mandible, long bones with their real shafts and joint ends, hand and foot plates, joints at the rig joints, masses at the centres of mass. 1:100: skull, rib cage and pelvis as masses, column and long bones as slender bars.
"""
import math
from r3_red_squirrel_geo import st, loft, ellipsoid, shrink, r5, MM

RED, WHITE, TAIL, SKIN, BONE = "fur-red", "fur-white", "tail-fur", "bare-skin", "bone"
SIDES = (("near", "n", -1), ("far", "f", 1))
GUARD = {"back": 5.6, "belly": 4.0, "head": 2.8, "ears": 1.6, "tail": 10.0, "arms": 2.0, "legs": 2.4}


def ref(sid, of, region, layer=None, **extra):
    out = {"id": sid, "of": of, "region": region}
    if layer:
        out["layer"] = layer
    out.update(extra)
    return out


def plume(coarse):
    """The body plume leaves from a saddle of the loin fur: a low patch 24 mm wide, 1.5 mm proud of the back."""
    s = [st(22, 4, 3, 28.6), st(27, 10, 7, 25.4), st(36, 12, 8, 24.9), st(46, 12, 8, 24.8), st(54, 10, 7, 25), st(59, 4, 3, 28)]
    if coarse:
        s = [st(22, 4, 3, 28.6), st(30, 11.4, 7.6, 25), st(50, 11.4, 7.6, 24.8), st(59, 4, 3, 28)]
    return loft("th-plume-back", "spine-r", RED, 0.8, s, sub=1 if coarse else 2, region="fur-back", tags=["emitter", "plume-back"])


def thermal50(body):
    by = {s["id"]: s for s in body}
    out = []

    def fur(ids, key, region, single=()):
        for sid in ids:
            if sid not in single:
                out.append(shrink(by[sid], GUARD[key], f"th-{sid}-underfur", tone=max(by[sid]["tone"] - 0.1, 0), region=region, layer=f"{key}-underfur"))
            out.append(ref(f"th-{sid}-guard", sid, region, f"{key}-guard"))

    fur(["rump", "chest"], "back", "fur-back")
    fur(["belly-patch", "belly-patch-rump", "throat-patch"], "belly", "fur-belly")
    fur(["neck", "head"], "head", "head-fur")
    out.append(ellipsoid("th-nose", "head", SKIN, 0.12, (50.6, 0, 0), (4.6, 8.3, 8.7), region="nose"))
    fur(["near-ear", "far-ear"], "ears", "ears")
    fur(["tail-1", "tail-2"], "tail", "tail-fur")
    fur(["near-arm", "near-forearm", "near-hand", "far-arm", "far-forearm", "far-hand"], "arms", "arms-fur", single=("near-hand", "far-hand"))
    out += [ref("th-near-fingers", "near-fingers", "hands-skin"), ref("th-far-fingers", "far-fingers", "hands-skin")]
    fur(["near-thigh", "near-shank", "near-foot", "far-thigh", "far-shank", "far-foot"], "legs", "legs-fur", single=("near-foot", "far-foot"))
    out += [ref("th-near-sole", "near-sole", "feet-skin", sectionFaces=["+v"]), ref("th-far-sole", "far-sole", "feet-skin", sectionFaces=["+v"])]
    out.append(ellipsoid("th-breath-nose", "head", SKIN, 0.2, (52.7, 0.8, 0), (2.7, 3, 3.8), region="nose", tags=["emitter", "breath-nose"]))
    out.append(plume(False))
    return out


def thermal100(body):
    out = [ref("th-rump", "rump", "fur-back"), ref("th-chest", "chest", "fur-back"), ref("th-belly", "belly-patch", "fur-belly"), ref("th-belly-rump", "belly-patch-rump", "fur-belly"), ref("th-neck", "neck", "head-fur"), ref("th-head", "head", "head-fur")]
    out.append(ellipsoid("th-nose", "head", SKIN, 0.12, (50.8, 0, 0), (4.4, 7.6, 8), region="nose"))
    out += [ref("th-near-ear", "near-ear", "ears"), ref("th-far-ear", "far-ear", "ears"), ref("th-tail-1", "tail-1", "tail-fur"), ref("th-tail-2", "tail-2", "tail-fur")]
    for name, side, sign in SIDES:
        out += [ref(f"th-{name}-arm", f"{name}-arm", "arms-fur"), ref(f"th-{name}-forearm", f"{name}-forearm", "arms-fur"), ref(f"th-{name}-hand", f"{name}-hand", "hands-skin")]
        out += [ref(f"th-{name}-thigh", f"{name}-thigh", "legs-fur"), ref(f"th-{name}-shank", f"{name}-shank", "legs-fur"), ref(f"th-{name}-foot", f"{name}-foot", "legs-fur")]
        out.append(ref(f"th-{name}-sole", f"{name}-sole", "feet-skin", sectionFaces=["+v"]))
    out.append(ellipsoid("th-breath-nose", "head", SKIN, 0.2, (52.4, 0.6, 0), (2.7, 2.7, 3.4), region="nose", tags=["emitter", "breath-nose"]))
    out.append(plume(True))
    return out


BONE_TONE, JOINT_TONE, MASS_TONE = 0.85, 0.55, 0.3


def member(sid, bone, stations, sub=2, frame=None, **extra):
    return loft(sid, bone, BONE, BONE_TONE, stations, sub=sub, frame=frame, member=bone, part="member", **extra)


def joint(bone, at, r):
    return ellipsoid(f"j-{bone}", bone, BONE, JOINT_TONE, at, (r, r, r), member=bone, part="joint")


def rib(i, u, radius, vc, tube=0.9):
    """One pair of ribs as a hoop about the long axis of the chest, slightly narrower than deep, hung under the column."""
    n = 8
    profile = [[r5((u + tube * math.cos(2 * math.pi * k / n)) * MM), r5((radius + tube * math.sin(2 * math.pi * k / n)) * MM)] for k in range(n)]
    profile.append(profile[0])
    return {"id": f"rib-{i}", "kind": "revolve", "bone": "spine-f", "material": BONE, "role": "body", "tone": BONE_TONE, "profile": profile, "axis": [1, 0, 0], "origin": [0, r5(vc * MM), 0], "lateralScale": 0.94, "slender": True, "member": "spine-f", "part": "member"}


def skeleton50():
    out = [member("m-spine-r", "spine-r", [st(-4, 3.2, 3.2, 11.8), st(10, 3.8, 3.5, 14.4), st(30, 4, 3.85, 16), st(52, 4, 3.85, 16), st(72, 3.8, 3.4, 14.6), st(77, 3.4, 3.3, 14.2)])]
    for name, side, sign in SIDES:
        A, Y, X = (-0.94, -0.341, 0.0), (-0.341, 0.94, 0.0), (0.0, 0.0, 1.0)
        s = [st(0, 1.4, 2.6), st(8, 1.6, 4.6, 0, sign * 0.5), st(18, 2.4, 4.2, 0, sign * 1), st(25.5, 3.8, 5.2, 0, sign * 1.5), st(33, 2.2, 5.4, 0, sign * 1.8), st(42, 1.8, 5, 0, sign * 2), st(47, 1.4, 3, 0, sign * 2)]
        out.append(loft(f"pelvis-{side}", "spine-r", BONE, BONE_TONE, s, frame=((40.0, 10.7, sign * 9.0), A, X, Y), member="spine-r", part="member"))
    out.append(member("m-spine-f", "spine-f", [st(-4.5, 3.4, 3.3, 14.2), st(0, 3.9, 3.85, 14.6), st(20, 3.9, 3.85, 14.8), st(40, 3.6, 3.3, 12.4), st(56, 3.5, 3.3, 8.8), st(62, 3.5, 3.3, 6.4), st(66, 3.2, 3.2, 5.4)]))
    out += [rib(i + 1, u, radius, vc) for i, (u, radius, vc) in enumerate([(5, 11.4, -0.4), (11, 13.6, -2.2), (17, 15.4, -3.8), (23, 16.2, -4.4), (29, 16.2, -4.2), (35, 15.4, -3.4), (41, 14, -2.2), (47, 12.2, -0.6), (53, 10, 1.2)])]
    out.append(member("sternum", "spine-f", [st(16, 1, 1, -21.2), st(26, 1, 1, -22), st(35, 1, 1, -19.8), st(44, 1, 1, -15.6), st(53, 1, 1, -10.2), st(59, 1, 1, -6.6)], slender=True))
    for name, side, sign in SIDES:
        A, Y, X = (-0.694, 0.72, 0.0), (0.72, 0.694, 0.0), (0.0, 0.0, 1.0)
        s = [st(0, 3.8, 3), st(5, 1.6, 3.4, 0, -sign * 1), st(14, 1, 6, 0, -sign * 2.4), st(22, 0.9, 7.6, 0, -sign * 3.4), st(29, 0.8, 7.2, 0, -sign * 4), st(31, 0.6, 3, 0, -sign * 4.2)]
        out.append(loft(f"scapula-{side}", "spine-f", BONE, BONE_TONE, s, frame=((52.0, -12.0, sign * 15.0), A, X, Y), member="spine-f", part="member"))
    out.append(member("m-neck", "neck", [st(-3, 3.6, 3.6, 0.6), st(8, 3.8, 3.8, 2), st(16, 3.8, 3.8, 1.6), st(24, 3.6, 3.6, 0.6), st(27, 2.6, 2.6, 0.4)]))
    out.append(member("m-head", "head", [st(-1, 4, 4, 3), st(2, 9.5, 8.2, 4.6), st(10, 12, 9.6, 5.6), st(19, 12.6, 9.2, 5.4), st(26, 15, 7.4, 3.6), st(32, 9, 6.6, 3.2), st(40, 5.6, 5.2, 2.2), st(48, 4, 3.8, 1.2), st(51.5, 2, 2.2, 0.6)]))
    out.append(loft("mandible", "head", BONE, BONE_TONE - 0.1, [st(5, 9.5, 2.6, -5), st(14, 10.5, 3.6, -6.8), st(26, 8, 3, -7.4), st(36, 4.6, 2.6, -6.6), st(45, 2.4, 2, -5.2)], member="head", part="member"))
    out.append(member("m-tail-1", "tail-1", [st(-4, 3.8, 3.8), st(6, 3.2, 3.2), st(20, 2.6, 2.6, -0.6), st(50, 2.2, 2.2, -1.4), st(85, 1.9, 1.9), st(88, 1.4, 1.4)]))
    out.append(member("m-tail-2", "tail-2", [st(-3, 1.9, 1.9), st(30, 1.7, 1.7, 1), st(60, 1.4, 1.4, 3.2), st(90, 0.9, 0.9, 6.6)], slender=True))
    rods = {
        "arm": [st(-3, 3.2, 3.2), st(0, 3.8, 3.9), st(6, 2.6, 2.8), st(20, 1.6, 1.7), st(33, 2.6, 1.8), st(39, 4.6, 2.4), st(42, 3, 2)],
        "forearm": [st(-5.5, 1.4, 2.2, -1.8), st(-1, 2.2, 3.8, -0.8), st(4, 2.6, 3), st(18, 2.2, 1.9), st(32, 2.6, 1.9), st(39, 3.8, 2.4), st(41.5, 2.6, 1.8)],
        "hand": [st(-1.5, 2.8, 2.7), st(4, 3.6, 2, -0.2), st(12, 4.2, 1.5, -0.5), st(20, 3.4, 1.1, -1.3), st(26, 2, 0.8, -2.2)],
        "thigh": [st(-3.5, 3, 3), st(0, 4.6, 4), st(6, 2.8, 2.8), st(25, 1.8, 1.8), st(42, 2.8, 2.8), st(49, 4.4, 3.9), st(52.5, 3, 2.8)],
        "shank": [st(-3, 3, 2.8), st(0, 4.4, 3.9), st(8, 2.8, 3), st(30, 1.7, 1.7), st(52, 1.8, 1.8), st(57.5, 3, 2.6), st(60, 2, 1.8)],
        "foot": [st(-7.4, 1.6, 2, -3.2), st(-3, 2.4, 2.8, -2.4), st(0, 3.2, 3, -1.4), st(12, 3.4, 1.8, -2.2), st(32, 4.8, 1.5, -1.4), st(44, 4.2, 1.2, -1), st(50, 2.4, 0.9, -1.2)],
    }
    for name, side, sign in SIDES:
        for key, stations in rods.items():
            out.append(member(f"m-{key}-{side}", f"{key}-{side}", stations))
    out += [joint("spine-f", (0, 14.6, 0), 4.1), joint("neck", (0, 0.8, 0), 4.2), joint("head", (0, 1.6, 0), 4.2), joint("tail-1", (0, 0, 0), 3.9)]
    for name, side, sign in SIDES:
        out += [joint(f"arm-{side}", (0, 0, 0), 3.9), joint(f"forearm-{side}", (0, 0, 0), 3.9), joint(f"hand-{side}", (0, 0, 0), 2.9), joint(f"thigh-{side}", (0, 0, 0), 4.1), joint(f"shank-{side}", (0, 0, 0), 4.3), joint(f"foot-{side}", (0, 0, 0), 3.4)]
    out += [ellipsoid("mass-spine-f", "spine-f", BONE, MASS_TONE, (31, 0, 0), (7.8, 7.8, 7.8), member="spine-f", part="mass"), ellipsoid("mass-head", "head", BONE, MASS_TONE, (24, 2, 0), (6.1, 6.1, 6.1), member="head", part="mass")]
    return out


def skeleton100():
    bar = lambda sid, bone, stations: member(sid, bone, stations, sub=1, slender=True)
    out = [
        bar("m-spine-r", "spine-r", [st(-4, 1.9, 1.9, 11.8), st(30, 1.9, 1.9, 15.6), st(76, 1.9, 1.9, 14.2)]),
        member("pelvis", "spine-r", [st(-6, 9, 4, -2.6), st(8, 13, 7.5, 0.5), st(22, 13.4, 7.5, 3.5), st(40, 9.6, 3.4, 10.4)], sub=1),
        bar("m-spine-f", "spine-f", [st(-4, 1.7, 1.7, 14.2), st(20, 1.7, 1.7, 14.8), st(42, 1.7, 1.7, 12), st(66, 1.7, 1.7, 6.2)]),
        rib(1, 8, 12.8, -1), rib(2, 20, 16, -3.6), rib(3, 33, 15.8, -3.4), rib(4, 46, 12.6, -0.4),
        bar("m-neck", "neck", [st(-3, 0.7, 0.7, 0.6), st(12, 0.7, 0.7, 1.9), st(27, 0.7, 0.7, 0.5)]),
        member("m-head", "head", [st(-1, 4, 4, 3), st(4, 10.6, 8.6, 4.8), st(18, 12.8, 9.6, 5.4), st(27, 14.6, 7.8, 3.6), st(38, 6.2, 5.6, 2.4), st(51.5, 2, 2.2, 0.6)], sub=1),
        bar("m-tail-1", "tail-1", [st(-4, 2.2, 2.2), st(40, 2.2, 2.2, -1.2), st(88, 2, 2)]),
        bar("m-tail-2", "tail-2", [st(-3, 1.9, 1.9), st(45, 1.6, 1.6, 2), st(90, 0.9, 0.9, 6.6)]),
    ]
    rods = {
        "arm": [st(-2, 1, 1), st(42, 1, 1)],
        "forearm": [st(-5, 1, 1, -1.6), st(4, 1, 1), st(41.5, 1, 1)],
        "hand": [st(-1.5, 0.65, 0.65), st(13, 0.65, 0.65, -0.5), st(26, 0.65, 0.65, -2.2)],
        "thigh": [st(-3, 1.35, 1.35), st(53, 1.35, 1.35)],
        "shank": [st(-3, 1.5, 1.5), st(60, 1.5, 1.5)],
        "foot": [st(-7, 1.35, 1.35, -3), st(0, 1.35, 1.35, -1.6), st(50, 1.35, 1.35, -1.2)],
    }
    for name, side, sign in SIDES:
        for key, stations in rods.items():
            out.append(bar(f"m-{key}-{side}", f"{key}-{side}", stations))
    out += [ellipsoid("mass-spine-f", "spine-f", BONE, MASS_TONE, (31, 0, 0), (7.8, 7.8, 7.8), member="spine-f", part="mass"), ellipsoid("mass-head", "head", BONE, MASS_TONE, (22, 2.6, 0), (7.5, 7.5, 7.5), member="head", part="mass")]
    return out
