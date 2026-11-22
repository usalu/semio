"""🦴 Courier with hand truck: shoulder and pelvic girdle on the lateral rest placement (clavicles from the sternoclavicular to the acromioclavicular joint, hip bones from the sacroiliac joint to the hip joint), arms and thighs hung from them without moving, and the girdle bones of the skeleton modelled by hand in world rest coordinates [ahead, up, left] for every scale.

Usage: python3 rework-courier-with-hand-truck-girdles.py            writes the candidate document 🗑️generated/rework-courier-with-hand-truck/cand/🔣️.json
       python3 rework-courier-with-hand-truck-girdles.py --apply    re-reads and writes the volumes source and the `rig` of the catalog file
"""
import copy
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(T, "rig-lateral-standing-adult.py")).read().split("def convert(")[0].split('"""', 2)[2])
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "courier-with-hand-truck", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "courier-with-hand-truck.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-courier-with-hand-truck", "cand", "🔣️.json")

STERNOCLAVICULAR = (0.076, 1.478, 0.022)
ACROMIOCLAVICULAR = (-0.008, 1.513, 0.170)
SACROILIAC = (-0.056, 1.030, 0.045)
HIP = (0.0, 0.95745, 0.095)
BONE = {"material": "bone", "role": "body", "tone": 0.9}


def side(p, sign):
    return [p[0], p[1], sign * p[2]]


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def local(frame, p):
    return r5(apply(transpose(frame[1]), [p[i] - frame[0][i] for i in range(3)]))


def section_rotation(frame, ex, ey):
    ex, ey = unit(ex), unit(ey)
    n = cross(ex, ey)
    world = [[ex[i], ey[i], n[i]] for i in range(3)]
    return r5(quaternion(mul(transpose(frame[1]), world)))


def capsule(frame, bone, sid, sign, p0, p1, r0, r1=None, part="member", **more):
    return {"id": sid, "kind": "capsule", "bone": bone, **BONE, "p0": local(frame, side(p0, sign)), "p1": local(frame, side(p1, sign)), "r0": r0, "r1": r0 if r1 is None else r1, "member": bone, "part": part, **more}


def ball(frame, bone, sid, sign, at, radius, part="member"):
    return {"id": sid, "kind": "ellipsoid", "bone": bone, **BONE, "radii": [radius, radius, radius], "transform": {"translation": local(frame, side(at, sign))}, "member": bone, "part": part}


def tube(frame, bone, sid, sign, points, radii, **more):
    """A round bar through world points: every section stands across the path."""
    points = [side(p, sign) for p in points]
    sections = []
    for i, p in enumerate(points):
        a, b = points[max(i - 1, 0)], points[min(i + 1, len(points) - 1)]
        t = unit([b[k] - a[k] for k in range(3)])
        ex = unit(cross(t, [0.0, 1.0, 0.0] if abs(t[1]) < 0.9 else [1.0, 0.0, 0.0]))
        ey = cross(ex, t)
        sections.append({"at": local(frame, p), "rx": radii[i], "ry": radii[i], "exponent": 2, "rotation": section_rotation(frame, ex, ey)})
    return {"id": sid, "kind": "loft", "bone": bone, **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member", **more}


def blade(frame, bone, sid, sign, rows, **more):
    """A flat bone plate: rows of (edge a, edge b, half thickness) in world points, lofted from the first row to the last."""
    sections = []
    rows = [(side(a, sign), side(b, sign), t) for a, b, t in rows]
    for i, (a, b, t) in enumerate(rows):
        mid = [(a[k] + b[k]) / 2 for k in range(3)]
        ex = [b[k] - a[k] for k in range(3)]
        half = math.sqrt(sum(c * c for c in ex)) / 2
        m0 = [(rows[max(i - 1, 0)][0][k] + rows[max(i - 1, 0)][1][k]) / 2 for k in range(3)]
        m1 = [(rows[min(i + 1, len(rows) - 1)][0][k] + rows[min(i + 1, len(rows) - 1)][1][k]) / 2 for k in range(3)]
        path = unit([m1[k] - m0[k] for k in range(3)])
        ex = unit(ex)
        ey = unit(cross(path, ex))
        ex = cross(ey, path)
        sections.append({"at": local(frame, mid), "rx": round(half, 5), "ry": t, "exponent": 2.6, "rotation": section_rotation(frame, ex, ey)})
    return {"id": sid, "kind": "loft", "bone": bone, **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": bone, "part": "member", **more}


def shoulder(frames_, s, sign, scale):
    """Clavicle and scapula of one side in world rest coordinates."""
    bone = f"clavicle-{s}"
    f = frames_[bone]
    sc, ac = STERNOCLAVICULAR, ACROMIOCLAVICULAR
    curve = [sc, (0.074, 1.489, 0.080), (0.036, 1.501, 0.132), ac]
    medial = [(-0.060, 1.507, 0.082), (-0.062, 1.495, 0.078), (-0.066, 1.470, 0.077), (-0.071, 1.440, 0.078), (-0.078, 1.400, 0.080), (-0.084, 1.365, 0.083), (-0.087, 1.345, 0.086)]
    lateral = [(-0.046, 1.507, 0.120), (-0.040, 1.495, 0.140), (-0.038, 1.470, 0.150), (-0.047, 1.440, 0.138), (-0.061, 1.400, 0.120), (-0.074, 1.365, 0.104), (-0.082, 1.345, 0.096)]
    if scale == "50":
        return [tube(f, bone, f"clavicle-{s}", sign, curve, [0.0095, 0.0065, 0.0065, 0.0085]),
                blade(f, bone, f"scapula-{s}", sign, [(a, b, 0.004) for a, b in zip(medial, lateral)]),
                capsule(f, bone, f"scapular-spine-{s}", sign, (-0.074, 1.468, 0.080), (-0.048, 1.502, 0.160), 0.005, 0.007),
                capsule(f, bone, f"acromion-{s}", sign, (-0.048, 1.502, 0.160), (0.002, 1.5165, 0.181), 0.007, 0.0065),
                capsule(f, bone, f"scapular-neck-{s}", sign, (-0.038, 1.474, 0.146), (-0.010, 1.489, 0.158), 0.009, 0.014),
                capsule(f, bone, f"coracoid-{s}", sign, (-0.010, 1.499, 0.152), (0.030, 1.486, 0.150), 0.006, 0.005)]
    if scale == "100":
        return [tube(f, bone, f"clavicle-{s}", sign, curve, [0.0095, 0.0075, 0.0075, 0.0085]),
                blade(f, bone, f"scapula-{s}", sign, [(medial[i], lateral[i], 0.0075) for i in (0, 2, 4, 6)]),
                capsule(f, bone, f"acromion-{s}", sign, (-0.072, 1.468, 0.082), (0.000, 1.515, 0.180), 0.0075, 0.008),
                capsule(f, bone, f"scapular-neck-{s}", sign, (-0.038, 1.474, 0.146), (-0.010, 1.489, 0.158), 0.010, 0.014)]
    if scale == "200":
        return [capsule(f, bone, f"clavicle-{s}", sign, sc, ac, 0.0065, slender=True),
                blade(f, bone, f"scapula-{s}", sign, [(medial[i], lateral[i], 0.015) for i in (1, 3, 6)]),
                capsule(f, bone, f"acromion-{s}", sign, (-0.045, 1.490, 0.130), (-0.004, 1.508, 0.176), 0.015)]
    return [capsule(f, bone, f"clavicle-{s}", sign, sc, ac, 0.0065, slender=True)]


def hip(frames_, s, sign, scale):
    """Hip bone of one side: ala of the ilium, acetabulum, pubis and ischium around the obturator opening."""
    bone = f"ilium-{s}"
    f = frames_[bone]
    front = [(0.030, 0.985, 0.090), (0.052, 1.010, 0.106), (0.066, 1.040, 0.119), (0.050, 1.062, 0.133), (0.022, 1.073, 0.138)]
    rear = [(-0.040, 0.985, 0.082), (-0.052, 1.010, 0.096), (-0.062, 1.040, 0.114), (-0.060, 1.062, 0.126), (-0.050, 1.072, 0.124)]
    sacral = [((-0.066, 1.000, 0.034), (-0.050, 1.000, 0.092)), ((-0.062, 1.030, 0.040), (-0.062, 1.030, 0.112)), ((-0.072, 1.055, 0.050), (-0.060, 1.055, 0.124)), ((-0.078, 1.066, 0.062), (-0.052, 1.070, 0.122))]
    tuber, symphysis = (-0.040, 0.878, 0.058), (0.068, 0.944, 0.009)
    if scale == "50":
        return [blade(f, bone, f"ilium-{s}", sign, [(a, b, t) for a, b, t in zip(rear, front, (0.011, 0.007, 0.0055, 0.0055, 0.006))]),
                blade(f, bone, f"iliac-auricular-{s}", sign, [(a, b, t) for (a, b), t in zip(sacral, (0.008, 0.007, 0.006, 0.006))]),
                {"id": f"acetabulum-{s}", "kind": "ellipsoid", "bone": bone, **BONE, "radii": [0.031, 0.031, 0.017], "transform": {"translation": local(f, side((-0.004, 0.966, 0.079), sign))}, "member": bone, "part": "member"},
                capsule(f, bone, f"pubic-ramus-{s}", sign, (0.022, 0.962, 0.068), symphysis, 0.008, 0.010),
                capsule(f, bone, f"ischium-{s}", sign, (-0.012, 0.945, 0.076), tuber, 0.012, 0.015),
                capsule(f, bone, f"ischiopubic-{s}", sign, (-0.034, 0.878, 0.055), (0.062, 0.926, 0.012), 0.007)]
    if scale == "100":
        return [blade(f, bone, f"ilium-{s}", sign, [(rear[i], front[i], t) for i, t in ((0, 0.012), (2, 0.0075), (4, 0.0075))]),
                blade(f, bone, f"iliac-auricular-{s}", sign, [(sacral[i][0], sacral[i][1], 0.0075) for i in (0, 1, 3)]),
                ball(f, bone, f"acetabulum-{s}", sign, (-0.004, 0.966, 0.074), 0.026),
                capsule(f, bone, f"pubic-ramus-{s}", sign, (0.022, 0.962, 0.068), symphysis, 0.008, 0.010),
                capsule(f, bone, f"ischium-{s}", sign, (-0.012, 0.945, 0.076), tuber, 0.012, 0.015),
                capsule(f, bone, f"ischiopubic-{s}", sign, (-0.034, 0.878, 0.055), (0.062, 0.926, 0.012), 0.0075)]
    if scale == "200":
        return [blade(f, bone, f"ilium-{s}", sign, [((-0.060, 0.985, 0.060), (0.030, 0.985, 0.088), 0.017), (rear[2], front[2], 0.015), (rear[4], front[4], 0.015)]),
                capsule(f, bone, f"iliac-auricular-{s}", sign, (-0.060, 1.040, 0.105), (-0.062, 1.025, 0.035), 0.015),
                capsule(f, bone, f"ischium-{s}", sign, (-0.008, 0.955, 0.076), tuber, 0.017, 0.015),
                capsule(f, bone, f"pubic-ramus-{s}", sign, (0.015, 0.958, 0.068), (0.062, 0.940, 0.016), 0.015)]
    return [capsule(f, bone, f"hip-bone-{s}", sign, (-0.050, 1.035, 0.050), (-0.004, 0.962, 0.090), 0.0375)]


def skull(fr, st):
    """Mandible of 1:50 as plates (ramus, body, chin) hinged at the temporal bone; points in the head frame (up, back, left)."""
    f = fr["head"]

    def world(p):
        moved = apply(f[1], list(p))
        return tuple(f[0][i] + moved[i] for i in range(3))

    st[:] = [o for o in st if not re.match(r"^(mandible|temporal|temporomandibular|mental|occipital)", o["id"])]
    new = [{"id": "occipital-base", "kind": "ellipsoid", "bone": "head", **BONE, "radii": [0.014, 0.042, 0.034], "transform": {"translation": [0.022, 0.012, 0]}, "member": "head", "part": "member"}]
    for s, sign in (("l", 1), ("r", -1)):
        ramus = [((0.026, -0.006, 0.056), (0.026, -0.036, 0.055)), ((0.0, -0.012, 0.053), (0.0, -0.040, 0.052)), ((-0.024, -0.020, 0.048), (-0.022, -0.046, 0.047))]
        body = [((-0.031, -0.030, 0.047), (-0.004, -0.032, 0.047)), ((-0.041, -0.060, 0.034), (-0.012, -0.060, 0.034)), ((-0.050, -0.080, 0.014), (-0.018, -0.082, 0.014))]
        new += [{"id": f"temporal-{s}", "kind": "ellipsoid", "bone": "head", **BONE, "radii": [0.022, 0.030, 0.012], "transform": {"translation": [0.044, 0.004, sign * 0.056]}, "member": "head", "part": "member"},
                {"id": f"temporomandibular-{s}", "kind": "ellipsoid", "bone": "head", **BONE, "radii": [0.008, 0.008, 0.008], "transform": {"translation": [0.031, -0.010, sign * 0.058]}, "member": "head", "part": "joint"},
                blade(f, "head", f"mandible-ramus-{s}", sign, [(world(a), world(b), 0.004) for a, b in ramus]),
                blade(f, "head", f"mandible-{s}", sign, [(world(a), world(b), 0.005) for a, b in body])]
    new.append({"id": "mental-protuberance", "kind": "ellipsoid", "bone": "head", **BONE, "radii": [0.017, 0.008, 0.02], "transform": {"translation": [-0.034, -0.082, 0]}, "member": "head", "part": "member"})
    at = next(i for i, o in enumerate(st) if o["id"] == "atlas")
    st[at:at] = new


GIRDLE = re.compile(r"^(clavicle|scapula|scapular|acromion|coracoid|ilium|iliac|hip-bone|acetabulum|ischium|pubic|ischiopubic|shoulder-girdle|pelvic-girdle)(-|$)")


def bones(asset):
    rig = asset["rig"]["bones"]
    by_id = {b["id"]: b for b in rig}
    if "clavicle-l" in by_id:
        return
    old = frames(rig)
    title = {"clavicle": {"en": "%s clavicle with scapula", "de": "%s Schlüsselbein mit Schulterblatt"}, "ilium": {"en": "%s hip bone", "de": "%s Hüftbein"}}
    word = {"l": {"en": "Left", "de": "Linkes"}, "r": {"en": "Right", "de": "Rechtes"}}
    for s, sign in (("l", 1), ("r", -1)):
        for name, parent, a, b, limb, joint in (("clavicle", "torso", STERNOCLAVICULAR, ACROMIOCLAVICULAR, "arm", {"type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40 if s == "l" else -5, "twistMaxDeg": 5 if s == "l" else 40}), ("ilium", "pelvis", SACROILIAC, HIP, "thigh", {"type": "revolute", "minDeg": -4, "maxDeg": 4})):
            origin, tip = side(a, sign), side(b, sign)
            u = [tip[i] - origin[i] for i in range(3)]
            new = {"id": f"{name}-{s}", "parent": parent, "title": {k: v % word[s][k] for k, v in title[name].items()}, **placed(old[parent], origin, u, (-1.0, 0.0, 0.0) if s == "l" else (1.0, 0.0, 0.0)), "lengthM": round(math.sqrt(sum(c * c for c in u)), 5), "joint": joint}
            rig.insert(rig.index(by_id[f"{limb}-{s}"]), new)
            child = by_id[f"{limb}-{s}"]
            now = frames(rig)
            placement = fields(now[new["id"]], old[child["id"]][0], old[child["id"]][1])
            child.pop("z", None)
            child["parent"] = new["id"]
            child.update({k: v for k, v in placement.items() if k not in ("abductionDeg", "twistDeg", "z") or abs(v) > 1e-9})
    new = frames(rig)
    for bone_id, (o, m) in old.items():
        drift = max(abs(new[bone_id][0][i] - o[i]) for i in range(3))
        turned = max(abs(new[bone_id][1][i][j] - m[i][j]) for i in range(3) for j in range(3))
        if drift > 3e-5 or turned > 3e-5:
            raise SystemExit(f"{bone_id} moved by {drift} m / {turned}")


def find(items, sid):
    return next((o for o in items if o["id"] == sid), None)


def girdles(asset, volumes):
    fr = frames(asset["rig"]["bones"])
    for scale, volume in volumes.items():
        st = volume["structure"]
        at = min(i for i, o in enumerate(st) if GIRDLE.match(o["id"]))
        if os.environ.get("SHOW"):
            for o in st:
                if GIRDLE.match(o["id"]):
                    print("[DEBUG] ", scale, json.dumps(o)[:330])
        st[:] = [o for o in st if not GIRDLE.match(o["id"])]
        new = []
        for s, sign in (("l", 1), ("r", -1)):
            new += shoulder(fr, s, sign, scale)
        for s, sign in (("l", 1), ("r", -1)):
            new += hip(fr, s, sign, scale)
        if scale in ("50", "100"):
            new.append({"id": "pubic-symphysis", "kind": "ellipsoid", "bone": "pelvis", **BONE, "radii": [0.019, 0.010, 0.011], "transform": {"translation": local(fr["pelvis"], (0.068, 0.940, 0.0))}, "member": "pelvis", "part": "joint"})
        if scale == "500":
            new.append({"id": "sacrum", "kind": "capsule", "bone": "pelvis", **BONE, "p0": [0.0975, 0.05, 0], "p1": [0.03, 0.07, 0], "r0": 0.0375, "r1": 0.0375, "member": "pelvis", "part": "member"})
        st[at:at] = new
        if scale == "50":
            skull(fr, st)


def details(volumes):
    """Small corrections outside the girdles, each stated by hand."""
    for scale, volume in volumes.items():
        for name in ("body", "thermal"):
            for o in volume[name]:
                if o.get("kind") == "loft" and o.get("bone", "").startswith("arm-"):
                    for sec in o["sections"]:
                        if sec["at"][0] < -0.02:
                            sec["at"][0] = round(sec["at"][0] + 0.010, 5)
    for s in "lr":
        find(volumes["50"]["structure"], f"patella-{s}")["transform"]["translation"][1] = 0.039
    for name, sid in (("body", "snap-ring"), ("thermal", "th-snap-ring"), ("structure", "st-snap-ring")):
        volumes["100"][name][:] = [o for o in volumes["100"][name] if o["id"] != sid]
    for s in "lr":
        find(volumes["200"]["body"], f"forearm-{s}")["sections"].append({"at": [0.282, 0.0, 0], "rx": 0.034, "ry": 0.03, "exponent": 2.4, "rotation": [0, -0.70711, 0, 0.70711]})
        shafts = {"200": {f"humerus-{s}": (0.011, 0.3366), f"forearm-bones-{s}": (0.009, 0.2652), f"femur-{s}": (0.014, None), f"tibia-{s}": (0.0135, 0.4335)},
                  "500": {f"arm-bar-{s}": (0.011, 0.3366), f"forearm-bar-{s}": (0.009, 0.2652), f"thigh-bar-{s}": (0.014, 0.4437), f"shank-bar-{s}": (0.0135, 0.4335)}}
        for scale, rows in shafts.items():
            for sid, (radius, length) in rows.items():
                bar = find(volumes[scale]["structure"], sid)
                bar.update({"r0": radius, "r1": radius, "slender": True})
                if length:
                    bar["p0"], bar["p1"] = [0, 0, 0], [length, 0, 0]
        sign = 1 if s == "l" else -1
        body = volumes["500"]["body"]
        body.insert(body.index(find(body, "handle")) + 1, {"id": f"strut-{s}", "kind": "capsule", "bone": "hand-truck", "material": "steel-tube", "role": "body", "tone": 0.35, "p0": [0.085, -0.13, sign * 0.150], "p1": [0, 0, sign * 0.165], "r0": 0.0375, "r1": 0.0375})
        volumes["500"]["thermal"].append({"id": f"th-strut-{s}", "of": f"strut-{s}", "region": "hand-truck"})
        st = volumes["500"]["structure"]
        st.insert(st.index(find(st, "st-handle")) + 1, {"id": f"st-strut-{s}", "of": f"strut-{s}", "member": "hand-truck", "part": "member"})


def main():
    document = json.load(open(CATALOG))
    source = json.load(open(SOURCE))
    asset = document["asset"]
    bones(asset)
    volumes = source["volumes"]
    if not find(volumes["50"]["structure"], "acromion-l"):
        details(volumes)
    girdles(asset, volumes)
    if "--apply" in sys.argv:
        text = open(CATALOG).read()
        a, b = text.index('\n    "rig": ') + 12, text.index(',\n    "anchors": ')
        rig = json.dumps(asset["rig"], ensure_ascii=False, indent=2).replace("\n", "\n    ")
        open(CATALOG, "w").write(text[:a] + rig + text[b:])
        json.dump(source, open(SOURCE, "w"), ensure_ascii=False, separators=(",", ":"))
        print("applied")
        return
    candidate = copy.deepcopy(document)
    candidate["asset"]["volumes"] = volumes
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(candidate, open(CANDIDATE, "w"), ensure_ascii=False, indent=1)
    for b in asset["rig"]["bones"]:
        if re.match(r"(clavicle|ilium|arm|thigh)-", b["id"]):
            print({k: v for k, v in b.items() if k not in ("title", "mass", "joint")})
    print(CANDIDATE)


main()
