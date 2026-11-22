"""🧰 Helpers of author 03 for hand-stated staffage solids: files, concise solid constructors, quaternions and the left/right mirror.

Import with `from rework_03_lib import *` (the file `rework-03-lib.py` only forwards to this module, because a hyphen cannot be imported).
Frames: a solid lives in the rest frame of its bone, u along the bone, v its normal in the sagittal plane, w lateral (left is +).
"""
import copy
import glob
import json
import math
import os

T = os.path.dirname(os.path.abspath(__file__))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
ACROSS = [0, -0.70711, 0, 0.70711]


def catalog_path(asset):
    """📍 The catalog file of `category/slug`."""
    slug = asset.split("/")[1]
    return glob.glob(f"{S}/🗂️catalog/*/{slug}/🔣️.json")[0]


def source_path(asset):
    """📍 The volumes source of `category/slug` in the ticket."""
    return f"{T}/volumes/{asset}.json"


def candidate_path(asset):
    """📍 The candidate document of `category/slug` (validated with `r3-validate.ts --file`)."""
    folder = f"{T}/🗑️generated/r3-{asset.split('/')[1]}"
    os.makedirs(folder, exist_ok=True)
    return f"{folder}/🔣️.json"


def read(path):
    """📖 Reads a JSON file."""
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def write(path, value):
    """💾 Writes a JSON file with two-space indentation and a final newline."""
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def rnd(value, digits=5):
    """🔢 Rounds numbers in nested lists and dicts."""
    if isinstance(value, float):
        return round(value, digits)
    if isinstance(value, list):
        return [rnd(item, digits) for item in value]
    if isinstance(value, dict):
        return {key: rnd(item, digits) for key, item in value.items()}
    return value


def quat(axis, deg):
    """🌀 Unit quaternion [x, y, z, w] of a rotation by `deg` about `axis` (u, v, w)."""
    n = math.sqrt(sum(a * a for a in axis))
    s = math.sin(math.radians(deg) / 2) / n
    return [round(axis[0] * s, 5), round(axis[1] * s, 5), round(axis[2] * s, 5), round(math.cos(math.radians(deg) / 2), 5)]


def qmul(a, b):
    """✖️ Quaternion product a·b (apply b first, then a)."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [round(aw * bx + ax * bw + ay * bz - az * by, 5), round(aw * by - ax * bz + ay * bw + az * bx, 5), round(aw * bz + ax * by - ay * bx + az * bw, 5), round(aw * bw - ax * bx - ay * by - az * bz, 5)]


def base(id, kind, bone, material, tone, role="body", **more):
    """🧱 The members every solid states, followed by its own fields."""
    return {"id": id, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone, **more}


def loft(id, bone, material, tone, sections, role="body", across=True, caps=True, **more):
    """🧵 A loft from rows (u, v, w, rx, ry[, exponent[, rotation]]): rx is the half width along w, ry along v for a path along u; `across` states the section plane perpendicular to u, otherwise the kernel takes the plane perpendicular to the path."""
    rows = []
    for row in sections:
        u, v, w, rx, ry = row[:5]
        section = {"at": [u, v, w], "rx": rx, "ry": ry}
        if len(row) > 5 and row[5] != 2:
            section["exponent"] = row[5]
        if len(row) > 6:
            section["rotation"] = row[6]
        elif across:
            section["rotation"] = ACROSS
        rows.append(section)
    return base(id, "loft", bone, material, tone, role, sections=rows, capStart=caps, capEnd=caps, **more)


def ell(id, bone, material, tone, radii, at, rotation=None, role="body", **more):
    """🥚 An ellipsoid with radii (u, v, w) centred at `at`."""
    transform = {"translation": list(at)}
    if rotation:
        transform["rotation"] = rotation
    return base(id, "ellipsoid", bone, material, tone, role, radii=list(radii), transform=transform, **more)


def box(id, bone, material, tone, size, at, radius=0.002, rotation=None, role="body", **more):
    """📦 A box with edges (u, v, w) centred at `at`."""
    transform = {"translation": list(at)}
    if rotation:
        transform["rotation"] = rotation
    return base(id, "box", bone, material, tone, role, size=list(size), cornerRadius=radius, transform=transform, **more)


def cap(id, bone, material, tone, p0, p1, r0, r1=None, role="body", **more):
    """💊 A capsule from p0 to p1."""
    return base(id, "capsule", bone, material, tone, role, p0=list(p0), p1=list(p1), r0=r0, r1=r0 if r1 is None else r1, **more)


def rev(id, bone, material, tone, profile, axis, origin, role="body", **more):
    """🏺 A revolve of (axial, radial) points about `axis` through `origin`; a first point equal to the last is a closed ring."""
    return base(id, "revolve", bone, material, tone, role, profile=[list(p) for p in profile], axis=list(axis), origin=list(origin), **more)


def ext(id, bone, material, tone, profile, plane, start, end, role="body", **more):
    """🧇 An extrusion of a simple polygon: plane xy = (u, v) along w, yz = (v, w) along u, zx = (w, u) along v."""
    return base(id, "extrude", bone, material, tone, role, profile=[list(p) for p in profile], plane=plane, **{"from": start, "to": end}, **more)


def _mq(q):
    return [-q[0], -q[1], q[2], q[3]]


def mirror(solid, old="-l", new="-r"):
    """🪞 The mirror image of a solid across the sagittal plane (w to -w), with `old` replaced by `new` in id and bone."""
    s = copy.deepcopy(solid)
    s["id"] = s["id"].replace(old, new)
    s["bone"] = s["bone"].replace(old, new)
    for key in ("of", "member"):
        if key in s:
            s[key] = s[key].replace(old, new)
    kind = s.get("kind")
    if "transform" in s:
        t = s["transform"]
        if "translation" in t:
            t["translation"] = [t["translation"][0], t["translation"][1], -t["translation"][2]]
        if "rotation" in t:
            t["rotation"] = _mq(t["rotation"])
    if kind == "loft":
        for sec in s["sections"]:
            sec["at"] = [sec["at"][0], sec["at"][1], -sec["at"][2]]
            if "rotation" in sec and sec["rotation"] != ACROSS:
                sec["rotation"] = _mq(sec["rotation"])
    elif kind == "capsule":
        s["p0"] = [s["p0"][0], s["p0"][1], -s["p0"][2]]
        s["p1"] = [s["p1"][0], s["p1"][1], -s["p1"][2]]
    elif kind == "revolve":
        s["axis"] = [s["axis"][0], s["axis"][1], -s["axis"][2]]
        s["origin"] = [s["origin"][0], s["origin"][1], -s["origin"][2]]
    elif kind == "extrude":
        if s["plane"] == "xy":
            s["from"], s["to"] = -s["to"], -s["from"]
        elif s["plane"] == "yz":
            s["profile"] = [[p[0], -p[1]] for p in reversed(s["profile"])]
        else:
            s["profile"] = [[-p[0], p[1]] for p in reversed(s["profile"])]
    return s


def both(solid):
    """👐 A left solid and its right mirror."""
    return [solid, mirror(solid)]


def by_id(solids):
    """🗂️ Solids of a list by id."""
    return {s["id"]: s for s in solids}


def bun():
    """🥟 The bun runtime of the repo cache."""
    return sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]


def ik(file, pose, variables, targets, fixed=None):
    """🎯 Solves pose values with `rework-03-ik.ts`: `variables` [(bone, field)], `targets` [(bone, local point, world point[, weight])], `fixed` {bone: {field: value}} written into the pose first; returns {bone: {field: value}}."""
    import subprocess

    job = {"file": file, "pose": pose, "vars": [list(v) for v in variables], "set": fixed or {}, "targets": [{"bone": t[0], "p": list(t[1]), "to": list(t[2]), **({"weight": list(t[3])} if len(t) > 3 else {})} for t in targets]}
    path = os.path.join(os.path.dirname(file), "ik-job.json")
    write(path, job)
    run = subprocess.run([bun(), os.path.join(T, "rework-03-ik.ts"), path], capture_output=True, text=True, cwd=T)
    os.remove(path)
    if run.returncode != 0:
        raise RuntimeError(run.stderr[-800:])
    worst = max((abs(float(x)) for line in run.stderr.splitlines() if "residual" in line for x in line.split("mm")[1].split()), default=0)
    return json.loads(run.stdout.strip().splitlines()[-1]), worst


def fk(file, points, pose=None, extra=()):
    """🦴 World positions of bone-local points with `rework-03-fk.ts`: `points` [(bone, (u, v, w))]; returns [(x, y, z)]."""
    import re
    import subprocess

    spec = ";".join(f"{b}:{p[0]},{p[1]},{p[2]}" for b, p in points)
    args = [bun(), os.path.join(T, "rework-03-fk.ts"), "--file", file, "--points", spec, *(["--pose", pose] if pose else []), *extra]
    run = subprocess.run(args, capture_output=True, text=True, cwd=T)
    return [tuple(float(x) for x in m.split(",")) for m in re.findall(r"\(([-0-9.,]+)\)", run.stdout)]
