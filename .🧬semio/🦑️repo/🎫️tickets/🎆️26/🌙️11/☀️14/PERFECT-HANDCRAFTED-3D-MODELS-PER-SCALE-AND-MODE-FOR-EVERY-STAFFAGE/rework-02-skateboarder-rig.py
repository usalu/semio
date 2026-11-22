"""🦴️ Girdles of `humans/skateboarder` on the lateral rest placement: clavicles from the sternoclavicular to the acromioclavicular joint,
hip bones from the sacroiliac to the hip joint, arms and thighs re-parented without moving, the former sagittal stub kept only as the
fixed frame of hood and collar (`hood-yoke`). Every point below is a hand-chosen world point of the rest pose [ahead, up, left] in metres,
taken against the reference table of `rework/assets/humans--skateboarder.md`.

Usage: python3 rework-02-skateboarder-rig.py [--publish]   (without the flag only the candidate document is written)
"""
import copy
import glob
import importlib.util
import json
import math
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib02", os.path.join(T, "rework-02-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
ASSET = "humans/skateboarder"
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
CANDIDATE = os.path.join(T, "🗑️generated", "rework-skateboarder", "cand", "🔣️.json")
BONE = {"material": "bone", "tone": 0.9}
MM = 0.001
NAME = {"l": ("Left", "Linkes"), "r": ("Right", "Rechtes")}

STERNOCLAVICULAR = (0.016, 1.192, 0.022)
ACROMIOCLAVICULAR = (-0.012, 1.245, 0.176)
SACROILIAC = (-0.058, 0.760, 0.040)
HIP = (0.0, 0.685, 0.087)
ACROMION = ((-0.046, 1.236, 0.150), (-0.010, 1.247, 0.190))
CORACOID = ((-0.002, 1.236, 0.158), (0.030, 1.224, 0.150))
GLENOID = (-0.004, 1.2154, 0.168)


def tool(*arguments):
    """🔧️ Output lines of `rig-rest.ts`."""
    return subprocess.run([BUN, os.path.join(T, "rig-rest.ts"), *arguments], check=True, capture_output=True, text=True, cwd=T).stdout.strip().splitlines()


def vector(text):
    return [float(c) for c in text.strip(" []").split(",")]


def frames(path):
    """🧭️ Rest frame of every bone: origin and the columns u, v, w."""
    out = {}
    for line in tool("--file", path):
        if " o [" not in line:
            continue
        head, rest = line.split(": o [", 1)
        parts = rest.replace("] tip [", "|").replace("] u [", "|").replace("] v [", "|").replace("] w [", "|").rstrip("]").split("|")
        out[head.split(" (")[0]] = {"o": vector(parts[0]), "u": vector(parts[2]), "v": vector(parts[3]), "w": vector(parts[4])}
    return out


def solve(path, bone, parent, at=None, u=None, w=None):
    """🧮️ Paste-ready rest placement of a bone."""
    arguments = ["--file", path, "--solve", bone, "--parent", parent]
    for flag, value in (("--at", at), ("--u", u), ("--w", w)):
        if value is not None:
            arguments += [flag, ",".join(str(c) for c in value)]
    return json.loads(tool(*arguments)[-1])


def mirrored(fields):
    """🪞️ Right twin of a left rest placement."""
    out = dict(fields)
    out["parent"] = out["parent"][:-2] + "-r" if out["parent"].endswith("-l") else out["parent"]
    for key in ("z", "abductionDeg", "twistDeg"):
        if key in out:
            out[key] = -out[key]
    return out


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def local(frame, point):
    """📍️ A world rest point in the frame of a bone."""
    d = [p - o for p, o in zip(point, frame["o"])]
    return [lib.r5(dot(d, frame[axis])) for axis in "uvw"]


def world(frame, point):
    return [frame["o"][i] + sum(frame[axis][i] * point[k] for k, axis in enumerate("uvw")) for i in range(3)]


def quaternion(m):
    """🌀️ Unit quaternion [x, y, z, w] of a rotation matrix."""
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        return [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    i = max(range(3), key=lambda k: m[k][k])
    j, k = (i + 1) % 3, (i + 2) % 3
    s = math.sqrt(1 + m[i][i] - m[j][j] - m[k][k]) * 2
    q = [0, 0, 0, 0]
    q[i], q[j], q[k], q[3] = s / 4, (m[j][i] + m[i][j]) / s, (m[k][i] + m[i][k]) / s, (m[k][j] - m[j][k]) / s
    return q


def product(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def rebind(solid, old, new, bone):
    """🔁️ The same world solid written in the frame of another bone."""
    turn = [[dot(new[a], old[b]) for b in "uvw"] for a in "uvw"]
    q = quaternion(turn)
    move = lambda p: local(new, world(old, p))
    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = move(solid["p0"]), move(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            if "rotation" not in section:
                raise SystemExit(f"loft {solid['id']} has a section without rotation")
            section["at"] = move(section["at"])
            section["rotation"] = [round(c, 7) for c in product(q, section["rotation"])]
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = move(transform["translation"])
        transform["rotation"] = [round(c, 7) for c in product(q, transform.get("rotation", [0, 0, 0, 1]))]
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} is not converted")
    solid["bone"] = bone
    if "member" in solid:
        solid["member"] = bone


def rename(node, old, new):
    """🏷️ A bone id replaced wherever a document names it."""
    if isinstance(node, dict):
        for key in list(node):
            if key == old:
                node[new] = node.pop(key)
        for key, value in node.items():
            if value == old and key in ("bone", "member", "parent"):
                node[key] = new
            else:
                rename(value, old, new)
    elif isinstance(node, list):
        for item in node:
            rename(item, old, new)


def clavicle(side, scale, frame):
    """🦴️ Clavicle: round sternal end, shaft convex ahead over the medial two thirds and concave ahead laterally, flat acromial end.
    Keys along the bone in mm: (u, up, back on the left / ahead on the right, half height, half depth)."""
    sign = 1 if side == "l" else -1
    length = 1000 * math.dist(STERNOCLAVICULAR, ACROMIOCLAVICULAR)
    if scale == 50:
        keys = [(0, 0, 0, 11, 11), (28, 1, -8, 8, 8.5), (60, 2, -12, 6.5, 7), (100, 2, -3, 6, 7), (135, 1, 7, 5.5, 9), (length, 0, 0, 5, 11)]
        return lib.loft(f"clavicle-{side}", f"clavicle-{side}", BONE["material"], BONE["tone"], [(u * MM, cv * MM, sign * cw * MM, rv * MM, rw * MM) for u, cv, cw, rv, rw in keys], sub=2, round0=5 * MM, round1=4 * MM, member=f"clavicle-{side}", part="member")
    if scale == 100:
        keys = [(0, 0, 0, 11, 11), (60, 2, -12, 7.5, 7.5), (120, 1, 4, 7.5, 8.5), (length, 0, 0, 7.5, 11)]
        return lib.loft(f"clavicle-{side}", f"clavicle-{side}", BONE["material"], BONE["tone"], [(u * MM, cv * MM, sign * cw * MM, rv * MM, rw * MM) for u, cv, cw, rv, rw in keys], sub=1, round0=5 * MM, round1=4 * MM, member=f"clavicle-{side}", part="member")
    return lib.capsule(f"clavicle-{side}", f"clavicle-{side}", BONE["material"], BONE["tone"], [0, 0, 0], [length * MM, 0, 0], 0.0065, member=f"clavicle-{side}", part="member", slender=True)


def flipped(point, side):
    return [point[0], point[1], point[2] if side == "l" else -point[2]]


def girdle(volume, scale, old, new):
    """🦴️ The structure list of one scale on the new bones."""
    solids = volume["structure"]
    by = {s["id"]: s for s in solids}
    out = [s for s in solids if s["id"] != "shoulder-girdle" and not s["id"].startswith("clavicle-")]
    first = next((i for i, s in enumerate(out) if s["id"].startswith("scapula-") or s["id"].startswith("humerus-")), len(out))
    added = []
    for side in "lr":
        bone = f"clavicle-{side}"
        added.append(clavicle(side, scale, new[bone]))
        scapula = by.get(f"scapula-{side}")
        if scapula is not None:
            rebind(scapula, old["shoulder-girdle"], new[bone], bone)
        if scale in (50, 100):
            radius = 0.007 if scale == 50 else 0.0078
            added.append(lib.capsule(f"acromion-{side}", bone, BONE["material"], BONE["tone"], local(new[bone], flipped(ACROMION[0], side)), local(new[bone], flipped(ACROMION[1], side)), radius + 0.001, radius, member=bone, part="member"))
            added.append(lib.ellipsoid(f"joint-clavicle-{side}", bone, BONE["material"], 0.5, [0, 0, 0], [0.012, 0.012, 0.012], member=bone, part="joint"))
        if scale == 50:
            added.append(lib.capsule(f"coracoid-{side}", bone, BONE["material"], BONE["tone"], local(new[bone], flipped(CORACOID[0], side)), local(new[bone], flipped(CORACOID[1], side)), 0.0065, 0.005, member=bone, part="member"))
            added.append(lib.ellipsoid(f"glenoid-{side}", bone, BONE["material"], BONE["tone"], local(new[bone], flipped(GLENOID, side)), [0.007, 0.019, 0.013], member=bone, part="member"))
        hip = f"ilium-{side}"
        for name in ("ilium", "ilium-auricular", "pubis", "ischium", "ischiopubic-ramus"):
            solid = by.get(f"{name}-{side}")
            if solid is not None:
                rebind(solid, old["pelvis"], new[hip], hip)
        if scale in (50, 100):
            added.append(lib.ellipsoid(f"joint-ilium-{side}", hip, BONE["material"], 0.5, [0, 0, 0], [0.013, 0.013, 0.013], member=hip, part="joint"))
    if "pelvis-bone" in by:
        by["pelvis-bone"]["members"] = ["ilium-l", "ilium-r"]
    volume["structure"] = out[:first] + added + out[first:]
    for name in ("body", "thermal"):
        rename(volume[name], "shoulder-girdle", "hood-yoke")


def main():
    publish = "--publish" in sys.argv
    document = lib.load_catalog(ASSET)
    asset = document["asset"]
    source = lib.load_source(ASSET)
    bones = asset["rig"]["bones"]
    if any(bone["id"] == "clavicle-l" for bone in bones):
        raise SystemExit("already converted")
    old = frames(lib.catalog_path(ASSET))
    asset["volumes"] = source["volumes"]
    yoke = next(bone for bone in bones if bone["id"] == "shoulder-girdle")
    yoke["id"], yoke["lengthM"] = "hood-yoke", 0.06
    volumes = asset.pop("volumes")
    rename(asset, "shoulder-girdle", "hood-yoke")
    asset["volumes"] = volumes
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)

    def write():
        json.dump(document, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=2)

    def insert(before, bone):
        bones.insert(next(i for i, b in enumerate(bones) if b["id"] == before), bone)

    write()
    span = lambda a, b: [q - p for p, q in zip(a, b)]
    clavicle_l = solve(CANDIDATE, "clavicle-l", "torso", STERNOCLAVICULAR, span(STERNOCLAVICULAR, ACROMIOCLAVICULAR), (-1, 0, 0))
    ilium_l = solve(CANDIDATE, "ilium-l", "pelvis", SACROILIAC, span(SACROILIAC, HIP), (-1, 0, 0))
    for side, pick in (("l", dict), ("r", mirrored)):
        insert("upper-arm-l" if side == "l" else "upper-arm-r", {"id": f"clavicle-{side}", "title": {"en": f"{NAME[side][0]} clavicle", "de": f"{NAME[side][1]} Schlüsselbein"}, **pick(clavicle_l), "lengthM": round(math.dist(STERNOCLAVICULAR, ACROMIOCLAVICULAR), 5), "joint": {"type": "revolute", "minDeg": -8, "maxDeg": 35}})
        insert("thigh-l" if side == "l" else "thigh-r", {"id": f"ilium-{side}", "title": {"en": f"{NAME[side][0]} hip bone", "de": f"{NAME[side][1]} Hüftbein"}, **pick(ilium_l), "lengthM": round(math.dist(SACROILIAC, HIP), 5), "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3}})
    write()
    for limb, parent in (("upper-arm", "clavicle"), ("thigh", "ilium")):
        placement = solve(CANDIDATE, f"{limb}-l", f"{parent}-l")
        for side, pick in (("l", dict), ("r", mirrored)):
            bone = next(b for b in bones if b["id"] == f"{limb}-{side}")
            fields = {k: v for k, v in pick(placement).items() if k not in ("abductionDeg", "twistDeg") or abs(v) > 1e-9}
            rest = {k: v for k, v in bone.items() if k not in ("parent", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "id")}
            bone.clear()
            bone.update({"id": f"{limb}-{side}", **fields, **rest})
    write()
    new = frames(CANDIDATE)
    for bone, frame in old.items():
        if bone == "shoulder-girdle":
            continue
        drift = max(abs(a - b) for key in "ouvw" for a, b in zip(frame[key], new[bone][key]))
        if drift > 2e-4:
            raise SystemExit(f"{bone} moved by {drift}")
    for scale in (50, 100, 200, 500):
        girdle(asset["volumes"][str(scale)], scale, old, new)
    for stage in ("coarse", "skeleton", "draw"):
        spec = importlib.util.spec_from_file_location(stage, os.path.join(T, f"rework-02-skateboarder-{stage}.py"))
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        module.apply(asset)
    write()
    print("[DEBUG] candidate", CANDIDATE)
    for bone in ("clavicle-l", "clavicle-r", "ilium-l", "ilium-r", "upper-arm-l", "upper-arm-r", "thigh-l", "thigh-r"):
        print("[DEBUG]", json.dumps({k: v for k, v in next(b for b in bones if b["id"] == bone).items() if k not in ("mass", "section", "title")}))
    if publish:
        lib.save_source(ASSET, {"volumes": asset["volumes"]})
        fresh = lib.load_catalog(ASSET)
        for key in ("rig", "poses", "clips", "anchors", "representations", "thermal", "structure"):
            fresh["asset"][key] = asset[key]
        lib.save_catalog(ASSET, fresh)
        print("[DEBUG] published")


main()
