"""🩻 Rig girdles and skeleton of `humans/person-with-white-cane` (woman, 1.71 m, 62 kg): clavicles and hip bones as laterally resting bones, limbs re-parented without moving, the structure lists of all four scales re-authored on them.

Every number is stated by hand in rest world coordinates [ahead, up, left] (metres) of the left side; the right side is its mirror. Writes the candidate document; `--publish` writes the catalog asset and the volumes source.

See ./rework-white-cane-lib.py, ./rig-lateral-rest.md and ./rework/assets/humans--person-with-white-cane.md.
"""
import copy
import importlib.util
import json
import os
import sys

_spec = importlib.util.spec_from_file_location("lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "rework-white-cane-lib.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

UP, BACK, LEFT = [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]
PELVIS = K.Frame([0.0, 0.9114, 0.0], UP, BACK, LEFT)
TORSO = K.Frame([0.0, 0.9954, 0.0], UP, BACK, LEFT)
ARM = K.Frame([0.0, 1.3938, 0.165], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], LEFT)
THIGH = K.Frame([0.0, 0.9114, 0.085], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], LEFT)

SC, AC = [0.05, 1.392, 0.025], [0.0, 1.422, 0.165]
SI, HIP = [-0.055, 0.985, 0.04], [0.0, 0.9114, 0.085]
CLAV = K.lateral(SC, AC)
ILIUM = K.lateral(SI, HIP)
F = {"pelvis": PELVIS, "torso": TORSO, "clavicle-l": CLAV, "clavicle-r": CLAV.mirrored(), "ilium-l": ILIUM, "ilium-r": ILIUM.mirrored()}

FEATURE = {"50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075}


def rig(asset):
    bones = [b for b in asset["rig"]["bones"] if b["id"] not in ("clavicle-l", "clavicle-r", "ilium-l", "ilium-r")]
    by = {b["id"]: b for b in bones}
    new = []
    for side, s in (("l", 1), ("r", -1)):
        def sided(p):
            return {**p, "z": p["z"] * s + 0.0, "abductionDeg": p["abductionDeg"] * s + 0.0, "twistDeg": p["twistDeg"] * s + 0.0}
        new.append({"id": f"clavicle-{side}", "parent": "torso", "title": {"en": ("Left" if s > 0 else "Right") + " clavicle and shoulder blade", "de": ("Linkes" if s > 0 else "Rechtes") + " Schlüsselbein mit Schulterblatt"}, **sided(K.placement(TORSO, CLAV)), "lengthM": round(K.math.dist(SC, AC), 5), "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40 if s > 0 else -5, "twistMaxDeg": 5 if s > 0 else 40}})
        new.append({"id": f"ilium-{side}", "parent": "pelvis", "title": {"en": ("Left" if s > 0 else "Right") + " hip bone", "de": ("Linkes" if s > 0 else "Rechtes") + " Hüftbein"}, **sided(K.placement(PELVIS, ILIUM)), "lengthM": round(K.math.dist(SI, HIP), 5), "joint": {"type": "revolute", "minDeg": -2, "maxDeg": 2}})
        for limb, parent, frame, girdle in ((f"arm-{side}", f"clavicle-{side}", ARM, CLAV), (f"thigh-{side}", f"ilium-{side}", THIGH, ILIUM)):
            b = by[limb]
            p = sided(K.placement(girdle, frame))
            b["parent"] = parent
            for k, v in p.items():
                if k in ("abductionDeg", "twistDeg") and abs(v) < 1e-4:
                    b.pop(k, None)
                else:
                    b[k] = v
    out = []
    for b in bones:
        if b["id"] == "arm-l":
            out += [n for n in new if n["id"].startswith("clavicle")]
        if b["id"] == "thigh-l":
            out += [n for n in new if n["id"].startswith("ilium")]
        out.append(b)
    asset["rig"]["bones"] = out
    for pose in asset["poses"]:
        for n in new:
            pose["bones"].setdefault(n["id"], {"angleDeg": 0})


def pairs(make):
    """👯 The left solids of `make(side, frame of a bone, mirror)` and their right twins."""
    out = []
    for side in ("l", "r"):
        m = (lambda s: s) if side == "l" else K.flip
        p = (lambda q: q) if side == "l" else K.mz
        out += make(side, m, p)
    return out


def shoulder(scale):
    f = FEATURE[scale] / 2
    r = lambda x: max(x, f)

    def make(side, m, p):
        bone = f"clavicle-{side}"
        fr = F[bone]
        mem = K.member(bone)
        tb = lambda sid, path, radii, extra=None: K.loft(fr, sid, bone, [m(s) for s in K.tube(path, radii)], {**mem, **(extra or {})})
        if scale in ("200", "500"):
            return [K.capsule(fr, f"clavicle-{side}", bone, p(SC), p(AC), 0.006, 0.0055, {**mem, "slender": True})]
        out = [tb(f"clavicle-{side}", [SC, [0.062, 1.399, 0.06], [0.052, 1.408, 0.1], [0.022, 1.416, 0.135], [0.004, 1.421, 0.156], [0.0, 1.422, 0.167]],
                  [(r(0.011), r(0.01)), (r(0.007), r(0.0065)), (r(0.006), r(0.0055)), (r(0.0065), r(0.005)), (r(0.009), r(0.0045)), (r(0.009), r(0.0045))])]
        h = r(0.004)
        blade = K.plate([(1.258, (-0.082, 0.082), (-0.076, 0.094), h), (1.29, (-0.086, 0.072), (-0.062, 0.105), h), (1.33, (-0.088, 0.066), (-0.042, 0.125), h),
                         (1.365, (-0.088, 0.066), (-0.026, 0.138), h), (1.385, (-0.085, 0.068), (-0.016, 0.146), h), (1.405, (-0.079, 0.074), (-0.05, 0.11), h)])
        out.append(K.loft(fr, f"scapula-{side}", bone, [m(s) for s in blade], mem))
        out.append(tb(f"scapular-spine-{side}", [[-0.09, 1.375, 0.07], [-0.08, 1.392, 0.105], [-0.058, 1.408, 0.14], [-0.036, 1.418, 0.166], [-0.008, 1.423, 0.178], [0.002, 1.422, 0.166]],
                      [(r(0.004), r(0.005)), (r(0.005), r(0.008)), (r(0.006), r(0.008)), (r(0.01), r(0.0045)), (r(0.011), r(0.0045)), (r(0.008), r(0.0045))]))
        if scale == "50":
            out.append(tb(f"coracoid-{side}", [[-0.008, 1.4, 0.143], [-0.004, 1.412, 0.132], [0.022, 1.398, 0.14]], [0.0065, 0.0055, 0.0045]))
            out.append(K.ellipsoid(fr, f"glenoid-{side}", bone, [0.012, 0.017, 0.006], p([-0.006, 1.392, 0.146]), mem))
        out.append(K.ellipsoid(fr, f"joint-clavicle-{side}", bone, [r(0.009)] * 3, p(SC), K.member(bone, "joint"), {**K.BONE, "tone": 0.4}))
        return out
    return pairs(make)


def hip(scale):
    f = FEATURE[scale] / 2
    r = lambda x: max(x, f)

    def make(side, m, p):
        bone = f"ilium-{side}"
        fr = F[bone]
        mem = K.member(bone)
        tb = lambda sid, path, radii, extra=None: K.loft(fr, sid, bone, [m(s) for s in K.tube(path, radii)], {**mem, **(extra or {})})
        if scale == "500":
            return []
        rows = [(0.925, (-0.03, 0.07), (-0.008, 0.08), (0.012, 0.085), 0.012), (0.945, (-0.045, 0.07), (-0.02, 0.098), (0.015, 0.102), 0.009),
                (0.97, (-0.068, 0.048), (-0.03, 0.108), (0.032, 0.115), 0.006), (0.995, (-0.082, 0.04), (-0.035, 0.12), (0.045, 0.125), 0.005),
                (1.02, (-0.085, 0.045), (-0.035, 0.132), (0.04, 0.133), 0.005), (1.036, (-0.072, 0.062), (-0.032, 0.134), (0.022, 0.139), 0.006), (1.046, (-0.05, 0.098), (-0.03, 0.13), (-0.004, 0.138), 0.005)]
        out = []
        if scale == "200":
            out.append(K.loft(fr, f"ilium-{side}", bone, [m(s) for s in K.plate([(y, a, c, r(h)) for y, a, b, c, h in rows[::2]])], mem))
        else:
            out.append(K.loft(fr, f"ilium-{side}", bone, [m(s) for s in K.plate([(y, b, c, r(h)) for y, a, b, c, h in rows])], mem))
            out.append(K.loft(fr, f"ilium-auricular-{side}", bone, [m(s) for s in K.plate([(y, a, b, r(h)) for y, a, b, c, h in rows])], mem))
            out.append(K.ellipsoid(fr, f"acetabulum-{side}", bone, [0.028, 0.028, r(0.014)], p([0.0, 0.9114, 0.072]), mem))
            out.append(tb(f"ischium-{side}", [[-0.012, 0.895, 0.07], [-0.028, 0.86, 0.062], [-0.032, 0.835, 0.055]], [r(0.012), r(0.012), r(0.014)]))
            out.append(tb(f"ischiopubic-ramus-{side}", [[-0.03, 0.838, 0.052], [0.01, 0.845, 0.03], [0.05, 0.868, 0.012]], [r(0.008), r(0.006), r(0.007)]))
            out.append(K.ellipsoid(fr, f"joint-ilium-{side}", bone, [r(0.008)] * 3, p(SI), K.member(bone, "joint"), {**K.BONE, "tone": 0.4}))
        out.append(tb(f"pubis-{side}", [[0.015, 0.915, 0.068], [0.04, 0.905, 0.04], [0.057, 0.89, 0.012]], [r(0.008), r(0.007), (r(0.009), r(0.014))]))
        return out
    out = pairs(make)
    mem = K.member("pelvis")
    if scale == "500":
        out.append(K.capsule(PELVIS, "pelvic-girdle", "pelvis", [-0.02, 0.95, -0.095], [-0.02, 0.95, 0.095], 0.043, 0.043, {**mem, "members": ["ilium-l", "ilium-r"]}))
        return out
    sac = [K.st(c, LEFT, ey, max(rx, f), max(ry, f), 2.4) for c, ey, rx, ry in (
        ([-0.05, 1.0, 0.0], [1.0, 0.35, 0.0], 0.055, 0.024), ([-0.07, 0.967, 0.0], [1.0, 0.45, 0.0], 0.044, 0.016), ([-0.082, 0.932, 0.0], [1.0, 0.1, 0.0], 0.027, 0.011),
        ([-0.076, 0.902, 0.0], [1.0, -0.5, 0.0], 0.011, 0.006), ([-0.064, 0.886, 0.0], [1.0, -0.8, 0.0], 0.006, 0.0045))][: 5 if scale == "50" else 3]
    out.append(K.loft(PELVIS, "sacrum", "pelvis", sac, mem))
    if scale != "200":
        out.append(K.ellipsoid(PELVIS, "pubic-symphysis", "pelvis", [max(0.008, f), 0.016, max(0.012, f)], [0.057, 0.885, 0.0], mem, {**K.BONE, "tone": 0.9}))
    return out


SPINE = [(0.006, 0.032), (0.052, 0.013), (0.102, 0.006), (0.167, 0.019), (0.241, 0.044), (0.316, 0.058), (0.381, 0.053), (0.436, 0.038), (0.4705, 0.028)]
VERTEBRAE = [("l5", 0.006, 0.042, 0.024, 0.017, 0.034, 0.038), ("l4", 0.042, 0.078, 0.024, 0.017, 0.036, 0.04), ("l3", 0.078, 0.114, 0.023, 0.0165, 0.036, 0.042), ("l2", 0.114, 0.15, 0.022, 0.016, 0.034, 0.04),
             ("l1", 0.15, 0.186, 0.021, 0.0155, 0.032, 0.038), ("t12", 0.186, 0.214, 0.02, 0.015, 0.03, 0.03), ("t11", 0.214, 0.241, 0.019, 0.015, 0.03, 0.028), ("t10", 0.241, 0.267, 0.0185, 0.0145, 0.032, 0.03),
             ("t9", 0.267, 0.292, 0.018, 0.014, 0.034, 0.03), ("t8", 0.292, 0.316, 0.0175, 0.0135, 0.036, 0.03), ("t7", 0.316, 0.339, 0.017, 0.013, 0.036, 0.03), ("t6", 0.339, 0.361, 0.0165, 0.0125, 0.036, 0.03),
             ("t5", 0.361, 0.382, 0.016, 0.012, 0.034, 0.03), ("t4", 0.382, 0.402, 0.0155, 0.0115, 0.032, 0.03), ("t3", 0.402, 0.421, 0.015, 0.011, 0.03, 0.03), ("t2", 0.421, 0.439, 0.0145, 0.0105, 0.024, 0.03),
             ("t1", 0.439, 0.4575, 0.014, 0.01, 0.02, 0.032)]


def centre(x):
    for (x0, y0), (x1, y1) in zip(SPINE, SPINE[1:]):
        if x <= x1 or (x1, y1) == SPINE[-1]:
            t = (x - x0) / (x1 - x0)
            n = K.unit([x1 - x0, y1 - y0, 0.0])
            return [x, y0 + (y1 - y0) * t, 0.0], n
    raise ValueError(x)


def vertebrae():
    """🦴 The seventeen thoracic and lumbar vertebrae in the frame of the torso bone (u up, v back, w left): body, spinous process, transverse processes."""
    out = []
    mem = K.member("torso")
    for name, lo, hi, rw, rv, spinous, transverse in VERTEBRAE:
        gap = 0.0035
        secs = []
        for x, k in ((lo + gap, 1.0), ((lo + hi) / 2, 0.9), (hi - gap, 1.0)):
            c, t = centre(x)
            secs.append({"at": [K.r5(v) for v in c], "rx": K.r5(rw * k), "ry": K.r5(rv * k), "exponent": 2.0, "rotation": K.quat([0, 0, 1], [-t[1], t[0], 0])})
        out.append({"id": f"vertebra-{name}", "kind": "loft", "bone": "torso", **K.BONE, "sections": secs, "capStart": True, "capEnd": True, **mem})
        c, t = centre((lo + hi) / 2)
        back = [-t[1], t[0], 0.0]
        root = K.add(c, back, rv + 0.003)
        drop = 0.5 if name.startswith("t") and name not in ("t12", "t11", "t1") else 0.12
        tip = K.add(K.add(root, back, spinous), t, -spinous * drop)
        out.append({"id": f"spinous-{name}", "kind": "capsule", "bone": "torso", **K.BONE, "p0": [K.r5(v) for v in root], "p1": [K.r5(v) for v in tip], "r0": 0.005, "r1": 0.004, **mem})
        out.append({"id": f"transverse-{name}", "kind": "capsule", "bone": "torso", **K.BONE, "p0": [K.r5(root[0]), K.r5(root[1]), -transverse], "p1": [K.r5(root[0]), K.r5(root[1]), transverse], "r0": 0.0045, "r1": 0.0045, **mem})
    return out


CERVICAL = [("c7", 0.0, 0.016, 0.012, 0.009, 0.03), ("c6", 0.016, 0.03, 0.0115, 0.0085, 0.022), ("c5", 0.03, 0.044, 0.011, 0.008, 0.018), ("c4", 0.044, 0.058, 0.011, 0.008, 0.017), ("c3", 0.058, 0.072, 0.011, 0.008, 0.017), ("c2", 0.072, 0.09, 0.0115, 0.0085, 0.02)]


def cervical():
    """🦒 The cervical vertebrae C2 to C7 in the frame of the neck bone along the line of the existing cervical column."""
    a, b = [-0.0016, 0.0282, 0.0], [0.0844, 0.0021, 0.0]
    t = K.unit(K.sub(b, a))
    back = [-t[1], t[0], 0.0]
    n = K.math.dist(a, b)
    out = []
    mem = K.member("neck")
    for name, lo, hi, rw, rv, spinous in CERVICAL:
        secs = [{"at": [K.r5(v) for v in K.add(a, t, n * x / 0.09)], "rx": rw, "ry": rv, "exponent": 2.0, "rotation": K.quat([0, 0, 1], back)} for x in (lo + 0.002, hi - 0.002)]
        out.append({"id": f"vertebra-{name}", "kind": "loft", "bone": "neck", **K.BONE, "sections": secs, "capStart": True, "capEnd": True, **mem})
        root = K.add(K.add(a, t, n * (lo + hi) / 2 / 0.09), back, rv + 0.002)
        tip = K.add(K.add(root, back, spinous), t, -spinous * 0.3)
        out.append({"id": f"spinous-{name}", "kind": "capsule", "bone": "neck", **K.BONE, "p0": [K.r5(v) for v in root], "p1": [K.r5(v) for v in tip], "r0": 0.0045, "r1": 0.004, **mem})
    return out


def face():
    """💀 Facial skeleton in the frame of the head bone (u up, v back, w left): maxilla, cheek bones with their arches, orbital rims, nasal bones."""
    mem = K.member("head")
    sec = lambda u, v, rw, rv: {"at": [u, v, 0.0], "rx": rw, "ry": rv, "exponent": 2.6, "rotation": K.quat([0, 0, 1], [0, 1, 0])}
    out = [{"id": "maxilla", "kind": "loft", "bone": "head", **K.BONE, "sections": [sec(-0.024, -0.066, 0.024, 0.021), sec(-0.006, -0.07, 0.027, 0.02), sec(0.02, -0.066, 0.031, 0.018), sec(0.03, -0.062, 0.03, 0.016)], "capStart": True, "capEnd": True, **mem},
           {"id": "nasal-bones", "kind": "capsule", "bone": "head", **K.BONE, "p0": [0.056, -0.088, 0.0], "p1": [0.034, -0.097, 0.0], "r0": 0.0065, "r1": 0.005, **mem}]
    for side, s in (("l", 1), ("r", -1)):
        out.append({"id": f"zygomatic-{side}", "kind": "ellipsoid", "bone": "head", **K.BONE, "radii": [0.012, 0.012, 0.012], "transform": {"translation": [0.024, -0.06, 0.04 * s]}, **mem})
        out.append({"id": f"zygomatic-arch-{side}", "kind": "capsule", "bone": "head", **K.BONE, "p0": [0.024, -0.054, 0.052 * s], "p1": [0.024, -0.02, 0.054 * s], "r0": 0.0045, "r1": 0.004, **mem})
        out.append({"id": f"orbital-rim-{side}", "kind": "capsule", "bone": "head", **K.BONE, "p0": [0.03, -0.064, 0.049 * s], "p1": [0.066, -0.068, 0.042 * s], "r0": 0.005, "r1": 0.0055, **mem})
    return out


LONG = ("femur", "tibia", "fibula", "humerus", "ulna", "radius")
DROP = {"50": {"clavicle-l", "clavicle-r", "scapula-l", "scapula-r", "ilium-l", "ilium-r", "pubis-l", "pubis-r", "ischium-l", "ischium-r", "sacrum", "face-bones"},
        "100": {"clavicle-l", "clavicle-r", "scapula-l", "scapula-r", "ilium-l", "ilium-r", "ischiopubic-l", "ischiopubic-r", "sacrum"},
        "200": {"shoulder-girdle", "pelvic-ring", "patella-l", "patella-r", "malleolus-l", "malleolus-r", "humerus-condyles-l", "humerus-condyles-r"},
        "500": {"shoulder-girdle", "pelvic-girdle"}}


def coarse(scale, solids):
    """🔭 The limb skeleton of a coarse scale: the long bones keep their true shaft (declared slender), compact ends below the feature are left out or grown to the feature, joints are balls of one feature."""
    f = FEATURE[scale] / 2
    out = []
    for s in solids:
        stem = s["id"].rsplit("-", 1)[0]
        if stem in LONG and s["id"].rsplit("-", 1)[-1] in ("l", "r"):
            if scale != "100" or stem == "fibula":
                s["slender"] = True
            if scale == "500" and stem == "femur":
                first = copy.deepcopy(s["sections"][0])
                first["at"] = [0.0, 0.0, 0.0]
                second = copy.deepcopy(first)
                second["at"] = [0.02, -0.004, 0.036 * (1 if s["id"].endswith("-l") else -1)]
                if s["sections"][0]["at"][0] > 0.01:
                    s["sections"] = [first, second] + s["sections"]
        elif scale in ("200", "500") and s.get("part") == "joint" and s.get("kind") == "ellipsoid":
            if scale == "500" and not s["id"].startswith("joint-arm"):
                continue
            s["radii"] = [max(c, f) for c in s["radii"]]
        elif scale in ("200", "500") and s.get("kind") == "ellipsoid" and s.get("part") == "member" and min(s["radii"]) < f:
            if scale == "500" and s["bone"].split("-")[0] in ("thigh", "shank", "arm", "forearm"):
                continue
            s["radii"] = [max(c, f) for c in s["radii"]]
        elif scale == "500" and s.get("kind") == "capsule" and s["id"].startswith("femur-neck"):
            continue
        out.append(s)
    return out


def joints(scale, solids):
    """🔩 Where the limb bones meet: the fibula reaches the tibial plateau and ends in the lateral malleolus, the talus sits in the ankle mortise, the patella lies on the condyles, the manubrium carries the clavicles, the grip of the cane lies in the hand."""
    out = []
    by = {s["id"]: s for s in solids}
    for side, k in (("l", 1), ("r", -1)):
        fib, pat, tib, fem = by.get(f"fibula-{side}"), by.get(f"patella-{side}"), by.get(f"tibia-{side}"), by.get(f"femur-{side}")
        if fib:
            fib["p0"] = [0.028, -0.012, 0.024 * k]
        if pat:
            pat["transform"]["translation"][1] = 0.032
        if scale in ("50", "100"):
            out.append({"id": f"malleolus-lateral-{side}", "kind": "ellipsoid", "bone": f"shank-{side}", **K.BONE, "radii": [0.014, 0.011, 0.009], "transform": {"translation": [0.418, -0.008, 0.026 * k]}, **K.member(f"shank-{side}")})
        if scale != "500":
            out.append({"id": f"talus-{side}", "kind": "ellipsoid", "bone": f"foot-{side}", **K.BONE, "radii": [0.018, 0.018, 0.018], "transform": {"translation": [0.0, 0.0, 0.0]}, **K.member(f"foot-{side}")})
        if scale == "500" and tib["sections"][0]["at"][0] > 0.01:
            head = copy.deepcopy(tib["sections"][0])
            head["at"], head["rx"], head["ry"] = [0.004, 0.0, -0.005 * k], 0.019, 0.019
            tib["sections"].insert(0, head)
            end = copy.deepcopy(fem["sections"][-1])
            end["at"], end["rx"], end["ry"] = [0.409, 0.0, -0.004 * k], 0.019, 0.019
            fem["sections"].append(end)
    if scale in ("200", "500"):
        for q in by["sternum"]["sections"]:
            q["rx"], q["ry"] = 0.012, 0.006
    if scale == "500" and by["spine"]["sections"][0]["at"][0] > 0:
        low = copy.deepcopy(by["spine"]["sections"][0])
        low["at"] = [-0.035, 0.022, 0.0]
        by["spine"]["sections"].insert(0, low)
    if scale == "200":
        out.append(K.ellipsoid(TORSO, "manubrium", "torso", [0.015, 0.022, 0.03], [0.045, 1.385, 0.0], K.member("torso")))
    if scale == "100":
        like = by["spine"]["sections"][0]
        by["spine"]["sections"] = [{**like, "at": [x, y, 0.0], "rx": r, "ry": r} for (x, y), r in zip(SPINE, SPINE_RADII)]
    if scale == "100" and "member-cane-grip" not in by:
        out.append({"id": "member-cane-grip", "of": "cane-grip", "member": "cane", "part": "member"})
    fresh = {o["id"] for o in out}
    return [s for s in solids if s["id"] not in fresh] + out


def body(asset):
    """🕶️ Parts of the body that did not meet what carries them: the temples start at the hinge of the frame front, the tip section of the cane reaches the roller."""
    for s in asset["volumes"]["50"]["body"]:
        if s["id"] in ("temple-l", "temple-r"):
            s["p0"] = [0.0612, -0.0945, 0.064 * (1 if s["id"].endswith("-l") else -1)]
    for s in asset["volumes"]["100"]["body"]:
        if s["id"] == "cane-band":
            s["p1"] = [1.186, 0, 0]


METATARSALS = [(1, -0.02, -0.03, 0.15, 0.0065), (2, -0.01, -0.014, 0.151, 0.0045), (3, 0.0, 0.002, 0.147, 0.0045), (4, 0.01, 0.016, 0.141, 0.0045), (5, 0.02, 0.03, 0.134, 0.0045)]
TOES = [(1, -0.03, -0.027, 0.056, 0.0075, 0.006), (2, -0.014, -0.013, 0.05, 0.0048, 0.004), (3, 0.002, 0.002, 0.045, 0.0046, 0.004), (4, 0.016, 0.015, 0.038, 0.0045, 0.004), (5, 0.03, 0.027, 0.03, 0.0045, 0.004)]


def feet(solids):
    """🦶 The foot of 1:50 in the frames of the foot bone (u from the ankle to the ball, v dorsal, w left) and the toe bone: tarsus, five metatarsals fanning to the ball, five toes; the first ray is the medial one."""
    by = {s["id"]: s for s in solids}
    out = []
    for side, k in (("l", 1), ("r", -1)):
        mid = by.get(f"midfoot-{side}")
        if mid is None or len(mid["sections"]) != 4:
            continue
        mid["sections"] = mid["sections"][:2]
        end = copy.deepcopy(mid["sections"][1])
        end["at"], end["rx"], end["ry"] = [0.078, 0.004, 0.0], 0.031, 0.015
        mid["sections"].append(end)
        ph = by[f"phalanges-{side}"]
        solids.remove(ph)
        for n, w0, w1, x1, r in METATARSALS:
            out.append({"id": f"metatarsal-{n}-{side}", "kind": "capsule", "bone": f"foot-{side}", **K.BONE, "p0": [0.07, 0.003, w0 * k], "p1": [x1, 0.0196, w1 * k], "r0": r + 0.001, "r1": r, **K.member(f"foot-{side}")})
        for n, w0, w1, x1, r0, r1 in TOES:
            out.append({"id": f"toe-{n}-{side}", "kind": "capsule", "bone": f"toe-{side}", **K.BONE, "p0": [-0.0078, 0.0255, w0 * k], "p1": [x1, 0.0205, w1 * k], "r0": r0, "r1": r1, **K.member(f"toe-{side}")})
    return solids + out


def structure(asset):
    for scale, v in asset["volumes"].items():
        mine = shoulder(scale) + hip(scale)
        if scale == "50":
            mine += vertebrae() + cervical() + face()
        ids = {s["id"] for s in mine} | DROP[scale]
        kept = [s for s in v["structure"] if s["id"] not in ids]
        if scale == "50":
            for s in kept:
                if s["id"] in ("spine", "cervical-spine") and s["tone"] != 0.9:
                    s["tone"] = 0.9
                    for q in s["sections"]:
                        q["rx"], q["ry"] = K.r5(q["rx"] * 0.72), K.r5(q["ry"] * 0.72)
        else:
            kept = coarse(scale, kept)
        v["structure"] = joints(scale, kept + mine)
        if scale == "50":
            v["structure"] = feet(v["structure"])


STALE = ("Skeleton at 1:50 on the girdle", "Girdles at", "Structure at 1:200:", "Structure at 1:500:")


def texts(asset):
    """📝 Decisions of the structure lists, English first and German second, from ./rework-white-cane-texts.json."""
    with open(os.path.join(K.T, "rework-white-cane-texts.json"), encoding="utf8") as h:
        mine = json.load(h)
    for scale, v in asset["volumes"].items():
        v["decisions"] = [d for d in v["decisions"] if not d["en"].startswith(STALE)] + [mine[scale]]


SPINE_RADII = [0.019, 0.0185, 0.018, 0.0165, 0.015, 0.014, 0.013, 0.012, 0.0115]


def build():
    doc = K.load()
    asset = doc["asset"]
    rig(asset)
    structure(asset)
    body(asset)
    texts(asset)
    return doc


if __name__ == "__main__":
    doc = build()
    K.save(K.CANDIDATE, doc)
    print("[DEBUG] candidate", K.CANDIDATE)
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"].split("-")[0] in ("clavicle", "ilium", "arm", "thigh"):
            print("[DEBUG]", json.dumps({k: v for k, v in b.items() if k not in ("title", "mass", "joint")}))
    if "--publish" in sys.argv:
        live = K.load()
        for k in ("rig", "poses", "volumes"):
            live["asset"][k] = doc["asset"][k]
        K.save(K.CATALOG, live, like=K.CATALOG)
        with open(K.SOURCE, encoding="utf8") as h:
            src = json.load(h)
        src["volumes"] = doc["asset"]["volumes"]
        K.save(K.SOURCE, src, like=K.SOURCE)
        print("[DEBUG] published")
