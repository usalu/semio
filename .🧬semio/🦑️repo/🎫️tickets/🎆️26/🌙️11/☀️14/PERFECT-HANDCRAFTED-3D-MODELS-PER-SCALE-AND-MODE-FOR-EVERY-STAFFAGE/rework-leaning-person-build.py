"""🧍 Hand-authored rework of humans/leaning-person: rig (lumbar, clavicles, pelvis halves, finger bones) and skeleton per scale.

Every number below is chosen by hand for this one figure (slim man, 1.72 m barefoot, 66 kg). World coordinates are
[ahead, up, left] in metres with the origin at the hip-joint centre (the root joint) in the rest pose.
Usage: python3 rework-leaning-person-build.py [--publish]
See rig-lateral-rest.md (rest placement) and rework/assets/humans--leaning-person.md (reference table).
"""
import copy
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/leaning-person/🔣️.json"
SOURCE = os.path.join(T, "volumes/humans/leaning-person.json")
OUT = os.path.join(T, "🗑️generated/rework-leaning-person")
BASE = os.path.join(OUT, "base.json")
CAND = os.path.join(OUT, "cand/🔣️.json")
LUMBAR = 0.17
SCALES = ["50", "100", "200", "500"]
HALF = {"50": 0.00375, "100": 0.0075, "200": 0.015, "500": 0.0375}


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def dot(a, b):
    return sum(a[i] * b[i] for i in range(3))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(dot(a, a))
    return [x / n for x in a]


def cols(u, v, w):
    return [[u[i], v[i], w[i]] for i in range(3)]


def rz(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def ry(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rx(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def rest_matrix(angle, abduction, twist):
    return mul(mul(rz(angle), ry(-abduction)), rx(-twist))


def rest_angles(r):
    return (math.degrees(math.atan2(r[1][0], r[0][0])), math.degrees(math.asin(max(-1, min(1, r[2][0])))), -math.degrees(math.atan2(r[2][1], r[2][2])))


def quat(m):
    t = m[0][0] + m[1][1] + m[2][2]
    if t > 0:
        s = math.sqrt(t + 1) * 2
        q = [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    elif m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        q = [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    elif m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        q = [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    else:
        s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
        q = [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]
    return [round(x, 5) for x in q]


def r5(p):
    return [round(x, 5) for x in p]


def frames(bones):
    out = {}
    for b in bones:
        r = rest_matrix(b["angleDeg"], b.get("abductionDeg", 0), b.get("twistDeg", 0))
        if b.get("parent"):
            o, m = out[b["parent"]]
            out[b["id"]] = (add(o, app(m, [b.get("x", 0), b.get("y", 0), b.get("z", 0)])), mul(m, r))
        else:
            out[b["id"]] = ([b.get("x", 0), b.get("y", 0), b.get("z", 0)], r)
    return out


def place(bone, parent_frame, origin, m):
    po, pm = parent_frame
    loc = app(tr(pm), sub(origin, po))
    a, ab, tw = rest_angles(mul(tr(pm), m))
    bone["x"], bone["y"], bone["z"] = round(loc[0], 5), round(loc[1], 5), round(loc[2], 5)
    bone["angleDeg"] = round(a, 4)
    for key, val in (("abductionDeg", ab), ("twistDeg", tw)):
        if abs(val) > 1e-6:
            bone[key] = round(val, 4)
        else:
            bone.pop(key, None)


def mirror(p):
    return [p[0], p[1], -p[2]]


def side(points, s):
    return points if s > 0 else [mirror(p) for p in points]


class Kit:
    """🦴 Builds structure solids from world rest coordinates into the frame of their bone."""

    def __init__(self, fr, half):
        self.fr = fr
        self.half = half

    def base(self, sid, bone, kind, part="member", **flags):
        s = {"id": sid, "kind": kind, "bone": bone, "material": "bone", "role": "body", "tone": 0.92 if part == "member" else 0.6}
        s.update(flags)
        return s, {"member": bone, "part": part}

    def loc(self, bone, p):
        o, m = self.fr[bone]
        return r5(app(tr(m), sub(p, o)))

    def rod(self, sid, bone, pts, radii, **flags):
        s, tail = self.base(sid, bone, "loft", **flags)
        s["sections"] = [{"at": self.loc(bone, p), "rx": max(r, self.half) if not flags.get("slender") else r, "ry": max(r, self.half) if not flags.get("slender") else r} for p, r in zip(pts, radii)]
        s["capStart"] = s["capEnd"] = True
        s.update(tail)
        return s

    def plate(self, sid, bone, pts, widths, thick, hint, **flags):
        s, tail = self.base(sid, bone, "loft", **flags)
        _, m = self.fr[bone]
        secs = []
        for i, p in enumerate(pts):
            n = unit(sub(pts[min(i + 1, len(pts) - 1)], pts[max(i - 1, 0)]))
            ax = unit(sub(hint, [dot(hint, n) * c for c in n]))
            ay = cross(n, ax)
            t = thick[i] if isinstance(thick, list) else thick
            secs.append({"at": self.loc(bone, p), "rx": widths[i], "ry": max(t, self.half), "rotation": quat(mul(tr(m), cols(ax, ay, n)))})
        s["sections"] = secs
        s["capStart"] = s["capEnd"] = True
        s.update(tail)
        return s

    def ball(self, sid, bone, centre, radii, part="member", **flags):
        s, tail = self.base(sid, bone, "ellipsoid", part, **flags)
        _, m = self.fr[bone]
        s["radii"] = [max(r, self.half) for r in radii]
        s["transform"] = {"translation": self.loc(bone, centre), "rotation": quat(tr(m))}
        s.update(tail)
        return s

    def capsule(self, sid, bone, p0, p1, r0, r1, **flags):
        s, tail = self.base(sid, bone, "capsule", **flags)
        keep = bool(flags.get("slender"))
        s.update({"p0": self.loc(bone, p0), "p1": self.loc(bone, p1), "r0": r0 if keep else max(r0, self.half), "r1": r1 if keep else max(r1, self.half)})
        s.update(tail)
        return s


def local_rod(sid, bone, secs, **flags):
    s = {"id": sid, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.92}
    s.update(flags)
    s["sections"] = [{"at": [x, y, z], "rx": r, "ry": r} for x, y, z, r in secs]
    s["capStart"] = s["capEnd"] = True
    s.update({"member": bone, "part": "member"})
    return s


def local_ball(sid, bone, centre, radii, part="member", tone=0.92):
    return {"id": sid, "kind": "ellipsoid", "bone": bone, "material": "bone", "role": "body", "tone": tone, "radii": radii, "transform": {"translation": centre}, "member": bone, "part": part}


def barrel(sid, bone, secs, **extra):
    s = {"id": sid, "kind": "loft", "bone": bone, "material": "bone", "role": "body", "tone": 0.92}
    s["sections"] = [{"at": [x, y, 0], "rx": lat, "ry": ap, "exponent": 2.2, "rotation": [0, -0.70711, 0, 0.70711]} for x, y, lat, ap in secs]
    s["capStart"] = s["capEnd"] = True
    s.update({"member": bone, "part": "member"})
    s.update(extra)
    return s


SPINE_BACK = [(0.0, 0.042), (0.05, 0.031), (0.10, 0.030), (0.15, 0.038), (0.20, 0.052), (0.25, 0.064), (0.30, 0.070), (0.35, 0.069), (0.40, 0.062), (0.45, 0.049), (0.4755, 0.040)]


def spine_back(x):
    for (x0, y0), (x1, y1) in zip(SPINE_BACK, SPINE_BACK[1:]):
        if x <= x1:
            t = (x - x0) / (x1 - x0)
            t = t * t * (3 - 2 * t)
            return y0 + (y1 - y0) * t
    return SPINE_BACK[-1][1]


SC = [0.072, 0.5097, 0.022]
AC = [-0.004, 0.546, 0.165]
SI = [-0.055, 0.085, 0.035]
HIP = [0.0, 0.0, 0.09]
FINGERS = ["index", "middle", "ring", "little"]
PHALANX = ["proximal", "middle", "distal"]
THUMB = ["metacarpal", "proximal", "distal"]
TITLE = {
    "lumbar": ("Lumbar spine and abdomen", "Lendenwirbelsäule und Bauch"),
    "clavicle": ("{S} clavicle and shoulder blade", "{S}s Schlüsselbein mit Schulterblatt"),
    "ilium": ("{S} hip bone", "{S}s Hüftbein"),
    "thumb-metacarpal": ("{S} thumb, metacarpal", "{S}s Mittelhandbein des Daumens"),
    "thumb-proximal": ("{S} thumb, proximal phalanx", "{S}s Grundglied des Daumens"),
    "thumb-distal": ("{S} thumb, distal phalanx", "{S}s Endglied des Daumens"),
}
FINGER_NAME = {"index": ("index finger", "Zeigefingers"), "middle": ("middle finger", "Mittelfingers"), "ring": ("ring finger", "Ringfingers"), "little": ("little finger", "kleinen Fingers")}
PHALANX_NAME = {"proximal": ("proximal phalanx", "Grundglied"), "middle": ("middle phalanx", "Mittelglied"), "distal": ("distal phalanx", "Endglied")}


def title(kind, s):
    en, de = ("Left", "Linke") if s == "l" else ("Right", "Rechte")
    if kind in TITLE:
        a, b = TITLE[kind]
        return {"en": a.replace("{S}", en), "de": b.replace("{S}", de)}
    f, p = kind.split("-")
    return {"en": f"{en} {FINGER_NAME[f][0]}, {PHALANX_NAME[p][0]}", "de": f"{de}s {PHALANX_NAME[p][1]} des {FINGER_NAME[f][1]}"}


def shift_solid(s, dx):
    for key in ("p0", "p1"):
        if key in s:
            s[key] = [round(s[key][0] + dx, 5), s[key][1], s[key][2]]
    for sec in s.get("sections", []):
        sec["at"] = [round(sec["at"][0] + dx, 5), sec["at"][1], sec["at"][2]]
    if "kind" in s and s["kind"] in ("ellipsoid", "box", "revolve", "extrude"):
        t = s.setdefault("transform", {}).setdefault("translation", [0, 0, 0])
        s["transform"]["translation"] = [round(t[0] + dx, 5), t[1], t[2]]


def build_rig(a):
    bones = a["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    old = frames(bones)
    lumbar = {"id": "lumbar", "parent": "pelvis", "title": {"en": TITLE["lumbar"][0], "de": TITLE["lumbar"][1]}, "x": by["torso"]["x"], "y": 0, "angleDeg": 0, "lengthM": LUMBAR, "joint": {"type": "revolute", "minDeg": -25, "maxDeg": 15}, "mass": {"kg": 10.778, "comX": 0.085, "comY": 0, "distribution": "point", "source": "literature", "reference": "de-leva-1996"}}
    torso = by["torso"]
    torso.update({"parent": "lumbar", "x": LUMBAR, "lengthM": round(torso["lengthM"] - LUMBAR, 4), "title": {"en": "Thorax", "de": "Brustkorb"}, "joint": {"type": "revolute", "minDeg": -15, "maxDeg": 10}})
    torso["mass"].update({"kg": 10.533, "comX": 0.1823})
    by["neck"]["x"] = torso["lengthM"]
    order = []
    for b in bones:
        if b["id"] == "torso":
            order.append(lumbar)
        order.append(b)
    bones[:] = order
    fr = frames([b for b in bones if b["id"] in ("pelvis", "lumbar", "torso")])
    new = []
    for s, sg in (("l", 1), ("r", -1)):
        sc, ac, si, hip = (p if sg > 0 else mirror(p) for p in (SC, AC, SI, HIP))
        for name, parent, p0, p1, limits in (("clavicle", "torso", sc, ac, (-10, 35)), ("ilium", "pelvis", si, hip, (-5, 5))):
            u = unit(sub(p1, p0))
            w = [-1.0, 0.0, 0.0]
            w = unit(sub(w, [dot(w, u) * c for c in u]))
            if sg < 0:
                w = [-c for c in w]
            v = cross(w, u)
            bone = {"id": f"{name}-{s}", "parent": parent, "title": title(name, s)}
            place(bone, fr[parent], p0, cols(u, v, w))
            bone["lengthM"] = round(math.sqrt(dot(sub(p1, p0), sub(p1, p0))), 5)
            bone["joint"] = {"type": "revolute", "minDeg": limits[0], "maxDeg": limits[1]}
            new.append(bone)
            fr[bone["id"]] = (p0, cols(u, v, w))
            limb = by[("upper-arm-" if name == "clavicle" else "thigh-") + s]
            limb["parent"] = bone["id"]
            place(limb, fr[bone["id"]], *old[limb["id"]])
    at = next(i for i, b in enumerate(bones) if b["id"] == "upper-arm-l")
    bones[at:at] = [b for b in new if b["id"].startswith("clavicle")]
    at = next(i for i, b in enumerate(bones) if b["id"] == "thigh-l")
    bones[at:at] = [b for b in new if b["id"].startswith("ilium")]
    return old


def finger_bones(a, old):
    bones = a["rig"]["bones"]
    by = {b["id"]: b for b in bones}
    body = {s["id"]: s for s in a["volumes"]["50"]["body"]}
    chains = {}
    for s, sg in (("l", 1), ("r", -1)):
        hand = by[f"hand-{s}"]
        fr = {hand["id"]: ([0, 0, 0], [[1, 0, 0], [0, 1, 0], [0, 0, 1]])}
        added = []
        for finger in ["thumb"] + FINGERS:
            parent = hand["id"]
            for n, seg in enumerate(THUMB if finger == "thumb" else PHALANX):
                cap = body[f"hand-{s}-{finger}-{n + 1}"]
                u = unit(sub(cap["p1"], cap["p0"]))
                w = [0.0, float(sg), 0.0]
                w = unit(sub(w, [dot(w, u) * c for c in u]))
                v = cross(w, u)
                bone = {"id": f"{finger}-{seg}-{s}", "parent": parent, "title": title(f"{finger}-{seg}", s)}
                place(bone, fr[parent], cap["p0"], cols(u, v, w))
                length = math.sqrt(dot(sub(cap["p1"], cap["p0"]), sub(cap["p1"], cap["p0"])))
                bone["lengthM"] = round(length, 5)
                bone["joint"] = {"type": "revolute", "minDeg": -20 if n == 0 else -5, "maxDeg": 90 if n == 0 else 100}
                fr[bone["id"]] = (cap["p0"], cols(u, v, w))
                chains[cap["id"]] = (bone["id"], hand["id"], fr[bone["id"]], length)
                added.append(bone)
                parent = bone["id"]
        at = next(i for i, b in enumerate(bones) if b["id"] == hand["id"]) + 1
        bones[at:at] = added
        hand["lengthM"] = round(body[f"hand-{s}-middle-1"]["p0"][0], 4)
        hand["mass"]["comX"] = round(0.79 * hand["lengthM"], 4)
    return chains


def rebind_fingers(a, chains):
    for scale in SCALES:
        vol = a["volumes"][scale]
        for name in ("body", "thermal", "structure"):
            for s in vol[name]:
                if s.get("id") in chains and s.get("kind") == "capsule":
                    bone, _, (o, m), _ = chains[s["id"]]
                    s["bone"] = bone
                    s["p0"] = r5(app(tr(m), sub(s["p0"], o)))
                    s["p1"] = r5(app(tr(m), sub(s["p1"], o)))


def split_spine(a):
    for scale in SCALES:
        vol = a["volumes"][scale]
        for name in ("body", "thermal", "structure"):
            extra = []
            for s in vol[name]:
                if s.get("bone") != "torso":
                    continue
                if name == "structure" and (s["id"].startswith("spinous-") and s["p0"][0] < LUMBAR or s["id"] == "joint-lumbosacral"):
                    s["bone"] = "lumbar"
                    s["member"] = "lumbar"
                    continue
                shift_solid(s, -LUMBAR)
                if s["id"] == "mass-torso":
                    s["transform"]["translation"] = [0.1823, 0, 0]
                    twin = copy.deepcopy(s)
                    twin.update({"id": "mass-lumbar", "bone": "lumbar", "member": "lumbar"})
                    twin["transform"]["translation"] = [0.085, 0, 0]
                    extra.append(twin)
            vol[name].extend(extra)
    for rep in a["representations"].values():
        for shape in rep["geometry"]["elevation"]:
            if shape["bone"] != "torso":
                continue
            if shape["kind"] != "poly":
                raise SystemExit(f"torso elevation shape {shape['id']} is {shape['kind']}")
            shape["points"] = [[round(p[0] - LUMBAR, 5), p[1]] for p in shape["points"]]
    for anchor in a["anchors"]:
        if anchor["bone"] == "torso":
            anchor["x"] = round(anchor["x"] - LUMBAR, 5)

    def spread(bones):
        if bones and "torso" in bones and bones["torso"].get("angleDeg"):
            angle = bones["torso"]["angleDeg"]
            bones["lumbar"] = {"angleDeg": round(0.4 * angle, 3)}
            bones["torso"]["angleDeg"] = round(0.6 * angle, 3)

    for pose in a["poses"]:
        spread(pose.get("bones"))
    for clip in a["clips"]:
        for key in clip["keys"]:
            spread(key.get("bones"))


def pelvis_solids(k, scale):
    out = []
    fine = scale in ("50", "100")
    if scale == "500":
        return [barrel("pelvic-ring", "pelvis", [(-0.07, 0.02, 0.07, 0.045), (0.0, 0.02, 0.115, 0.07), (0.08, 0.03, 0.135, 0.08), (0.145, 0.035, 0.12, 0.06)], members=["ilium-l", "ilium-r"])]
    out.append(k.plate("sacrum", "pelvis", [[-0.05, 0.1, 0], [-0.08, 0.062, 0], [-0.097, 0.022, 0], [-0.088, -0.018, 0]], [0.056, 0.046, 0.028, 0.011], [0.02, 0.016, 0.012, 0.0075], [0, 0, 1]))
    for s, sg in (("l", 1), ("r", -1)):
        b = f"ilium-{s}"
        wing = side([[-0.002, 0.03, 0.086], [-0.008, 0.072, 0.104], [-0.015, 0.113, 0.121], [-0.02, 0.143, 0.128]], sg)
        out.append(k.plate(f"ilium-{s}", b, wing, [0.03, 0.05, 0.068, 0.06], [0.016, 0.009, 0.006, 0.006], [1, 0, 0]))
        if fine:
            out.append(k.rod(f"iliac-crest-{s}", b, side([[0.05, 0.095, 0.118], [0.028, 0.133, 0.132], [-0.02, 0.15, 0.13], [-0.062, 0.135, 0.095], [-0.085, 0.105, 0.045]], sg), [0.008, 0.007, 0.007, 0.007, 0.009]))
        out.append(k.capsule(f"ilium-auricular-{s}", b, *side([[-0.058, 0.108, 0.08], [-0.057, 0.085, 0.04]], sg), 0.014, 0.014))
        out.append(k.ball(f"acetabulum-{s}", b, mirror([-0.002, 0.004, 0.074]) if sg < 0 else [-0.002, 0.004, 0.074], [0.033, 0.033, 0.016]))
        out.append(k.rod(f"pubis-{s}", b, side([[0.016, -0.004, 0.07], [0.042, -0.012, 0.038], [0.056, -0.022, 0.008]], sg), [0.011, 0.0095, 0.012]))
        out.append(k.rod(f"ischium-{s}", b, side([[-0.012, -0.022, 0.076], [-0.03, -0.052, 0.066], [-0.045, -0.076, 0.055]], sg), [0.015, 0.013, 0.016]))
        out.append(k.rod(f"ischiopubic-ramus-{s}", b, side([[-0.04, -0.078, 0.052], [0.002, -0.07, 0.03], [0.05, -0.042, 0.008]], sg), [0.009, 0.008, 0.009]))
        if fine:
            out.append(k.ball(f"joint-sacroiliac-{s}", b, side([SI], sg)[0], [0.012, 0.012, 0.012], part="joint"))
    return out


def shoulder_solids(k, scale):
    out = []
    for s, sg in (("l", 1), ("r", -1)):
        b = f"clavicle-{s}"
        thin = scale in ("200", "500")
        flags = {"slender": True} if thin else {}
        out.append(k.rod(f"clavicle-{s}", b, side([SC, [0.083, 0.52, 0.066], [0.05, 0.535, 0.122], AC], sg), [0.0075, 0.0075, 0.0075, 0.0075] if thin else [0.0105, 0.0075, 0.0075, 0.009], **flags))
        if scale == "500":
            continue
        blade = side([[-0.106, 0.392, 0.098], [-0.098, 0.43, 0.105], [-0.088, 0.47, 0.11], [-0.08, 0.505, 0.112], [-0.078, 0.528, 0.105]], sg)
        out.append(k.plate(f"scapula-{s}", b, blade, [0.012, 0.034, 0.048, 0.052, 0.04], 0.0045, [0.557, 0, 0.83 * sg]))
        spine = side([[-0.108, 0.495, 0.072], [-0.03, 0.543, 0.166]], sg)
        out.append(k.capsule(f"scapular-spine-{s}", b, *spine, 0.007, 0.007, **flags))
        out.append(k.ball(f"acromion-{s}", b, side([[-0.012, 0.548, 0.176]], sg)[0], [0.024, 0.007, 0.017]))
        neck = side([[-0.05, 0.508, 0.15], [-0.02, 0.512, 0.153]], sg)
        out.append(k.capsule(f"scapular-neck-{s}", b, *neck, 0.011, 0.012))
        if scale in ("50", "100"):
            out.append(k.ball(f"glenoid-{s}", b, side([[-0.018, 0.512, 0.153]], sg)[0], [0.01, 0.019, 0.012]))
            out.append(k.capsule(f"coracoid-{s}", b, *side([[-0.014, 0.52, 0.142], [0.03, 0.505, 0.135]], sg), 0.0075, 0.0075))
            out.append(k.ball(f"joint-sternoclavicular-{s}", b, side([SC], sg)[0], [0.011, 0.011, 0.011], part="joint"))
    return out


def spine_solids(scale):
    out = []
    if scale == "500":
        pts = [(x, spine_back(x)) for x in (0.0, 0.085, 0.17, 0.25, 0.33, 0.41, 0.4755)]
        s = local_rod("spine", "torso", [(round(x - LUMBAR, 4), round(y, 4), 0, 0.019) for x, y in pts] + [(0.36, 0.03, 0, 0.014), (0.415, 0.012, 0, 0.012)], slender=True)
        s["members"] = ["lumbar", "neck"]
        return [s]
    if scale == "200":
        out.append(local_rod("lumbar-spine", "lumbar", [(round(x, 4), round(spine_back(x), 4), 0, 0.02) for x in (-0.004, 0.045, 0.09, 0.135, 0.174)]))
        out.append(local_rod("thoracic-spine", "torso", [(round(x - LUMBAR, 4), round(spine_back(x), 4), 0, 0.016) for x in (0.166, 0.23, 0.3, 0.37, 0.43, 0.478)]))
        return out
    for i in range(5):
        x = 0.017 + 0.034 * i
        out.append(local_ball(f"vertebra-l{5 - i}", "lumbar", [round(x, 4), round(spine_back(x), 4), 0], [0.0178, 0.0175, 0.0235]))
    for i in range(12):
        x = LUMBAR + 0.0127 + 0.0255 * i
        grow = 1 - i / 11
        out.append(local_ball(f"vertebra-t{12 - i}", "torso", [round(x - LUMBAR, 4), round(spine_back(x), 4), 0], [0.0135, round(0.0115 + 0.004 * grow, 4), round(0.02 + 0.007 * grow, 4)]))
    out.append(local_rod("vertebral-arch-lumbar", "lumbar", [(round(x, 4), round(spine_back(x) + 0.018, 4), 0, 0.01) for x in (0.0, 0.045, 0.09, 0.135, 0.172)]))
    out.append(local_rod("vertebral-arch-thoracic", "torso", [(round(x - LUMBAR, 4), round(spine_back(x) + 0.015, 4), 0, 0.009) for x in (0.168, 0.23, 0.3, 0.37, 0.43, 0.4755)]))
    out.append(local_ball("joint-thoracolumbar", "torso", [0, round(spine_back(LUMBAR), 4), 0], [0.012, 0.022, 0.022], part="joint", tone=0.6))
    return out


def long_bones(scale):
    out = []
    for s, sg in (("l", 1), ("r", -1)):
        thin = {"slender": True}
        out.append(local_rod(f"femur-{s}", f"thigh-{s}", [(0, 0, 0, 0.022), (0.03, -0.005, 0.04 * sg, 0.016), (0.06, -0.006, 0.056 * sg, 0.017), (0.2, -0.002, 0.03 * sg, 0.0145), (0.37, 0.002, 0.004 * sg, 0.017), (0.415, 0, 0, 0.03)], **(thin if scale == "500" else {})))
        out.append(local_rod(f"tibia-{s}", f"shank-{s}", [(0, 0, 0, 0.032), (0.04, 0.006, -0.004 * sg, 0.019), (0.2, 0.005, -0.006 * sg, 0.0145), (0.37, 0.004, -0.008 * sg, 0.015), (0.41, 0, 0, 0.02)], **thin))
        out.append(local_rod(f"humerus-{s}", f"upper-arm-{s}", [(0, 0, -0.004 * sg, 0.022), (0.03, 0.003, 0.004 * sg, 0.0135), (0.16, 0, 0.002 * sg, 0.011), (0.285, -0.004, 0, 0.012), (0.318, -0.004, 0, 0.016)], **thin))
        out.append(local_rod(f"ulna-{s}", f"forearm-{s}", [(-0.022, -0.012, -0.012 * sg, 0.011), (0.02, -0.008, -0.013 * sg, 0.009), (0.13, -0.005, -0.015 * sg, 0.0075), (0.245, -0.004, -0.017 * sg, 0.0075)], **thin))
        out.append(local_rod(f"radius-{s}", f"forearm-{s}", [(0.01, 0.004, 0.012 * sg, 0.008), (0.12, 0.006, 0.012 * sg, 0.0075), (0.24, 0.006, 0.014 * sg, 0.011)], **thin))
        if scale == "200":
            out.append(local_rod(f"fibula-{s}", f"shank-{s}", [(0.035, -0.012, 0.033 * sg, 0.009), (0.2, -0.01, 0.031 * sg, 0.0075), (0.405, -0.008, 0.028 * sg, 0.01)], **thin))
    return out


def hand_skeleton(a, chains):
    out = []
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    for s, sg in (("l", 1), ("r", -1)):
        hand = f"hand-{s}"
        for finger in FINGERS:
            first = bones[f"{finger}-proximal-{s}"]
            out.append({"id": f"metacarpal-{finger}-{s}", "kind": "capsule", "bone": hand, "material": "bone", "role": "body", "tone": 0.92, "p0": [0.03, round(first["y"] * 0.6, 4), round(-0.002 * sg, 4)], "p1": [first["x"], first["y"], first["z"]], "r0": 0.0048, "r1": 0.0045, "member": hand, "part": "member"})
            for seg, r in zip(PHALANX, (0.0044, 0.004, 0.0038)):
                bone = f"{finger}-{seg}-{s}"
                out.append({"id": f"phalanx-{finger}-{seg}-{s}", "kind": "capsule", "bone": bone, "material": "bone", "role": "body", "tone": 0.92, "p0": [0, 0, 0], "p1": [bones[bone]["lengthM"], 0, 0], "r0": r, "r1": r - 0.0002 if seg != "distal" else r, "member": bone, "part": "member"})
        for seg, r in zip(THUMB, (0.005, 0.0044, 0.004)):
            bone = f"thumb-{seg}-{s}"
            out.append({"id": f"thumb-bone-{seg}-{s}", "kind": "capsule", "bone": bone, "material": "bone", "role": "body", "tone": 0.92, "p0": [0, 0, 0], "p1": [bones[bone]["lengthM"], 0, 0], "r0": r, "r1": r, "member": bone, "part": "member"})
    return out


DROP = {
    "50": ("spine", "clavicle-", "scapula-", "scapular-spine-", "sacrum", "ilium-", "pubis-", "ischium-", "hand-bones-", "thumb-bones-"),
    "100": ("spine", "shoulder-girdle-", "hip-bone-", "sacrum", "femur-", "tibia-", "humerus-", "forearm-bones-", "fibula-"),
    "200": ("spine", "shoulder-girdle-", "hip-bone-", "sacrum", "femur-", "tibia-", "humerus-", "forearm-bones-", "fibula-", "occipital-condyle-", "clavicle-", "scapula-"),
    "500": ("spine", "shoulder-girdle-", "hip-bone-", "sacrum", "femur-", "tibia-", "humerus-", "forearm-bones-", "fibula-", "occipital-condyle-", "clavicle-", "scapula-"),
}
FINE_LONG = ("femur-neck-", "femur-head-", "trochanter-", "femur-l", "femur-r", "femur-condyles-", "tibia-plateau-", "tibia-l", "tibia-r", "malleolus-medial-", "fibula-", "humerus-head-", "humerus-l", "humerus-r", "humerus-condyles-", "ulna-", "radius-")
RIBS_100 = ("rib-1-", "rib-3-", "rib-5-", "rib-7-", "rib-9-", "rib-11-")


def structure(a, base, chains):
    fr = frames(a["rig"]["bones"])
    fine = {s["id"]: s for s in a["volumes"]["50"]["structure"]}
    for scale in SCALES:
        k = Kit(fr, HALF[scale])
        lst = a["volumes"][scale]["structure"]
        kept = [s for s in lst if not s["id"].startswith(DROP[scale])]
        dropped = [s["id"] for s in lst if s["id"].startswith(DROP[scale])]
        new = spine_solids(scale) + pelvis_solids(k, scale) + shoulder_solids(k, scale)
        if scale == "50":
            new += hand_skeleton(a, chains)
        elif scale == "100":
            for s in fine.values():
                if s["id"].startswith(FINE_LONG):
                    new.append(copy.deepcopy(s))
                if s["id"].startswith(RIBS_100):
                    rib = copy.deepcopy(s)
                    for sec in rib["sections"]:
                        sec["rx"] = sec["ry"] = 0.0075
                    new.append(rib)
        else:
            new += long_bones(scale)
            if not any(s["id"] == "rib-cage" for s in kept):
                new.append(barrel("rib-cage", "torso", [(0.02, 0.005, 0.115, 0.085), (0.09, 0.005, 0.125, 0.1), (0.17, 0.005, 0.12, 0.098), (0.25, 0.003, 0.095, 0.08), (0.285, 0.0, 0.06, 0.055)]))
        ids = {s["id"] for s in new}
        a["volumes"][scale]["structure"] = [s for s in kept if s["id"] not in ids] + new
        print(f"[DEBUG] 1:{scale} structure dropped {len(dropped)}: {' '.join(sorted(set(d.rstrip('lr').rstrip('-') for d in dropped)))}; now {len(a['volumes'][scale]['structure'])}")


def nowall(doc, path):
    twin = copy.deepcopy(doc)
    for vol in twin["asset"]["volumes"].values():
        for name in ("body", "thermal", "structure"):
            vol[name] = [s for s in vol[name] if "wall" not in s["id"] and s.get("bone") != "wall"]
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(twin, f, ensure_ascii=False)


def main():
    os.makedirs(os.path.dirname(CAND), exist_ok=True)
    if not os.path.exists(BASE):
        with open(CATALOG) as f:
            doc = json.load(f)
        with open(BASE, "w") as f:
            json.dump(doc, f, ensure_ascii=False)
    with open(BASE) as f:
        doc = json.load(f)
    a = doc["asset"]
    old = build_rig(a)
    split_spine(a)
    chains = finger_bones(a, old)
    rebind_fingers(a, chains)
    structure(a, doc, chains)
    for name in sorted(n for n in os.listdir(T) if n.startswith("rework-leaning-person-stage") and n.endswith(".py")):
        scope = {"__file__": os.path.join(T, name)}
        with open(os.path.join(T, name)) as f:
            exec(compile(f.read(), name, "exec"), scope)
        scope["apply"](a, sys.modules[__name__])
        if "reach" in scope:
            scope["reach"](a, sys.modules[__name__])
    new = frames(a["rig"]["bones"])
    worst = max(max(abs(new[b][0][i] - old[b][0][i]) for i in range(3)) for b in old if b not in ("torso",))
    print(f"[DEBUG] bones {len(a['rig']['bones'])}; largest rest move of a pre-existing bone origin {worst * 1000:.4f} mm")
    with open(CAND, "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
    nowall(doc, os.path.join(OUT, "nowall/🔣️.json"))
    if "--publish" in sys.argv:
        with open(SOURCE, "w") as f:
            json.dump({"volumes": a["volumes"]}, f, ensure_ascii=False, indent=1)
        with open(CATALOG) as f:
            live = json.load(f)
        for key in ("rig", "anchors", "poses", "clips", "volumes", "representations", "structure", "thermal", "dimensions"):
            live["asset"][key] = a[key]
        with open(CATALOG, "w") as f:
            json.dump(live, f, ensure_ascii=False, indent=2)
        print("[DEBUG] published source and catalog")


if __name__ == "__main__":
    main()
