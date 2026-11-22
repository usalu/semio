"""🐖 pig: shoulder and pelvic girdle on the full 3D rest placement of the rig (see `rig-lateral-rest.md`).

Scapulae: from the dorsal border 0.05 m beside the plane down, forward and outward to the shoulder joint 0.095 m beside it (rest `abductionDeg`), so the humerus leaves from the tip of the blade instead of jumping 45 mm sideways. Hip bones: new bones `ilium-hn/-hf` from the sacroiliac joint 0.04 m beside the plane back, down and outward to the hip joint 0.085 m beside it; the femora hang from their tips. Humeri and femora keep their world rest frames. Solids bound to a scapula, and the ilium / ischium solids that move from `pelvis` to the hip bones, are re-expressed in the new bone frames (same world geometry). Poses with a scapula swing keep the shoulder joint and the humerus where they were.

Usage: python3 rework-pig-rig.py            writes the candidate `🗑️generated/rework-pig/pig/🔣️.json`
       python3 rework-pig-rig.py --apply    writes `rig`, `poses`, `representations`, `volumes` of the catalog asset and the volumes source
"""
import copy
import importlib.util
import json
import math
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "animals", "pig.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-pig", "pig", "🔣️.json")
DEG = 180 / math.pi

SHOULDER_JOINT_Z = 0.095
SACROILIAC = {"alongPelvis": 0.125, "z": 0.04}
HIP_BONE_SOLIDS = ("ilium", "ischium", "hip-bone")
SIDES = {"n": -1.0, "f": 1.0}

spec = importlib.util.spec_from_file_location("pig_io", os.path.join(T, "r3-pig-io.py"))
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, p):
    return [sum(m[i][k] * p[k] for k in range(3)) for i in range(3)]


def transpose(m):
    return [[m[j][i] for j in range(3)] for i in range(3)]


def turn(angle, abduction, twist):
    a, b, t = angle / DEG, abduction / DEG, twist / DEG
    rw = [[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]]
    rv = [[math.cos(b), 0, -math.sin(b)], [0, 1, 0], [math.sin(b), 0, math.cos(b)]]
    ru = [[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]]
    return mul(mul(rw, rv), ru)


def angles(local):
    u = [local[i][0] for i in range(3)]
    v = [local[i][1] for i in range(3)]
    b = math.asin(max(-1.0, min(1.0, u[2])))
    a = 0.0 if math.hypot(u[0], u[1]) < 1e-9 else math.atan2(u[1], u[0])
    v1 = [-math.sin(a), math.cos(a), 0.0]
    w2 = [-math.cos(a) * math.sin(b), -math.sin(a) * math.sin(b), math.cos(b)]
    t = math.atan2(-sum(v[i] * w2[i] for i in range(3)), sum(v[i] * v1[i] for i in range(3)))
    return a * DEG, b * DEG, t * DEG


def frames(bones, offsets=None):
    offsets = offsets or {}
    by_id = {bone["id"]: bone for bone in bones}
    out = {}

    def solve(bone_id):
        if bone_id in out:
            return out[bone_id]
        bone = by_id[bone_id]
        offset = offsets.get(bone_id, {})
        origin, basis = ([0.0, 0.0, 0.0], turn(0, 0, 0)) if bone["parent"] is None else solve(bone["parent"])
        here = [origin[i] + apply(basis, [bone["x"], bone["y"], bone.get("z", 0.0)])[i] for i in range(3)]
        rest = turn(bone["angleDeg"], bone.get("abductionDeg", 0.0), bone.get("twistDeg", 0.0))
        moved = turn(offset.get("angleDeg", 0.0), offset.get("abductionDeg", 0.0), offset.get("twistDeg", 0.0))
        out[bone_id] = (here, mul(mul(basis, rest), moved))
        return out[bone_id]

    for bone in bones:
        solve(bone["id"])
    return out


def quaternion(m):
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1) * 2
        return [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, s / 4]
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1 + m[0][0] - m[1][1] - m[2][2]) * 2
        return [s / 4, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    if m[1][1] > m[2][2]:
        s = math.sqrt(1 + m[1][1] - m[0][0] - m[2][2]) * 2
        return [(m[0][1] + m[1][0]) / s, s / 4, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    s = math.sqrt(1 + m[2][2] - m[0][0] - m[1][1]) * 2
    return [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, s / 4, (m[1][0] - m[0][1]) / s]


def product(a, b):
    return [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]


def r5(values):
    return [round(x, 5) + 0.0 for x in values]


def r4(x):
    return round(x, 4) + 0.0


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def norm(a):
    return math.sqrt(sum(c * c for c in a))


def place(bone, parent_frame, origin, basis):
    """Writes the parent-frame fields that put `bone` at the world `origin` with the world `basis`."""
    po, pb = parent_frame
    local = apply(transpose(pb), sub(origin, po))
    a, b, t = angles(mul(transpose(pb), basis))
    bone["x"], bone["y"] = r5(local)[0], r5(local)[1]
    if abs(local[2]) > 1e-6:
        bone["z"] = r5(local)[2]
    else:
        bone.pop("z", None)
    bone["angleDeg"] = r4(a)
    for key, value in (("abductionDeg", b), ("twistDeg", t)):
        if abs(value) > 1e-4:
            bone[key] = r4(value)
        else:
            bone.pop(key, None)


def ordered(bone):
    keys = ["id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint", "mass"]
    return {key: bone[key] for key in keys if key in bone} | {key: bone[key] for key in bone if key not in keys}


def new_rig(old_bones):
    bones = copy.deepcopy(old_bones)
    old = frames(old_bones)
    by_id = {bone["id"]: bone for bone in bones}
    for side, sign in SIDES.items():
        scapula, humerus = by_id["scapula-f" + side], by_id["humerus-f" + side]
        origin = old["scapula-f" + side][0]
        joint = old["humerus-f" + side][0]
        assert abs(abs(joint[2]) - SHOULDER_JOINT_Z) < 1e-4
        axis = sub(joint, origin)
        length = norm(axis)
        scapula["lengthM"] = round(length, 4)
        scapula["abductionDeg"] = r4(math.asin(sign * (SHOULDER_JOINT_Z - abs(origin[2])) / length) * DEG)
        scapula["mass"]["comX"] = round(length / 2, 4)
        mid = frames(bones)
        place(humerus, mid["scapula-f" + side], joint, old["humerus-f" + side][1])
        pelvis_o, pelvis_b = old["pelvis"]
        sacroiliac = [pelvis_o[i] + apply(pelvis_b, [SACROILIAC["alongPelvis"], 0.0, sign * SACROILIAC["z"]])[i] for i in range(3)]
        hip = old["thigh-h" + side][0]
        span = norm(sub(hip, sacroiliac))
        lateral = apply(transpose(pelvis_b), sub(hip, sacroiliac))
        ilium = {
            "id": "ilium-h" + side,
            "parent": "pelvis",
            "title": {"en": ("Near" if side == "n" else "Far") + " hip bone (ilium, ischium, pubis)", "de": ("Nahes" if side == "n" else "Fernes") + " Hüftbein (Darmbein, Sitzbein, Schambein)"},
            "x": SACROILIAC["alongPelvis"],
            "y": 0.0,
            "z": sign * SACROILIAC["z"],
            "angleDeg": 180.0,
            "abductionDeg": r4(math.asin(lateral[2] / span) * DEG),
            "lengthM": round(span, 4),
            "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3},
        }
        index = bones.index(by_id["thigh-h" + side])
        bones.insert(index, ilium)
        by_id[ilium["id"]] = ilium
        thigh = by_id["thigh-h" + side]
        thigh["parent"] = ilium["id"]
        mid = frames(bones)
        place(thigh, mid[ilium["id"]], hip, old["thigh-h" + side][1])
    return [ordered(bone) for bone in bones]


def rebind(solid, change):
    """Re-expresses a solid in a new bone frame: `change` = (rotation Bnewᵀ·Bold, translation Bnewᵀ·(Oold − Onew))."""
    m, shift = change
    q = quaternion(m)

    def point(p):
        moved = apply(m, p)
        return r5([moved[i] + shift[i] for i in range(3)])

    def rotation(r):
        return r5(product(q, r if r is not None else [0.0, 0.0, 0.0, 1.0]))

    if solid["kind"] == "loft":
        for section in solid["sections"]:
            if "rotation" not in section:
                raise SystemExit("loft section without rotation: " + solid["id"])
            section["at"] = point(section["at"])
            section["rotation"] = rotation(section["rotation"])
    elif solid["kind"] == "ellipsoid":
        transform = solid.setdefault("transform", {})
        transform["translation"] = point(transform.get("translation", [0.0, 0.0, 0.0]))
        transform["rotation"] = rotation(transform.get("rotation"))
    elif solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    else:
        raise SystemExit("rebind: unsupported kind " + solid["kind"] + " of " + solid["id"])


def change_of(old, new, old_bone, new_bone):
    oo, ob = old[old_bone]
    no, nb = new[new_bone]
    return mul(transpose(nb), ob), apply(transpose(nb), sub(oo, no))


def convert_volumes(volumes, old, new):
    count = 0
    for scale in ("50", "100", "200", "500"):
        for name in ("body", "thermal", "structure"):
            for solid in volumes[scale][name]:
                bone = solid.get("bone", "")
                if bone.startswith("scapula-f"):
                    rebind(solid, change_of(old, new, bone, bone))
                    count += 1
                elif bone == "pelvis" and solid["id"].startswith(("near-", "far-")) and solid["id"].endswith(HIP_BONE_SOLIDS):
                    target = "ilium-hn" if solid["id"].startswith("near-") else "ilium-hf"
                    rebind(solid, change_of(old, new, "pelvis", target))
                    solid["bone"] = target
                    if "member" in solid:
                        solid["member"] = target
                    count += 1
        structure = volumes[scale]["structure"]
        if scale == "50" and not any(s["id"] == "joint-ilium-hn" for s in structure):
            anchor = next(i for i, s in enumerate(structure) if s["id"] == "joint-pelvis")
            for k, side in enumerate(SIDES):
                structure.insert(anchor + 1 + k, {"id": "joint-ilium-h" + side, "kind": "ellipsoid", "bone": "ilium-h" + side, "material": "bone", "role": "body", "tone": 0.5, "radii": [0.012, 0.012, 0.012], "transform": {"translation": [0, 0, 0], "rotation": [0, 0, 0, 1]}, "member": "ilium-h" + side, "part": "joint"})
    return count


def convert_poses(poses, old_bones, bones):
    """Keeps every fore hoof where the pose had it: the scapula swings about its own (now tilted) joint axis, and the flexions of humerus, forearm and metacarpus are re-solved so that fetlock and sole pitch stay in the side elevation. Flexion only: the limb keeps no abduction or twist offset."""
    lengths = {b["id"]: b["lengthM"] for b in bones}
    old_lengths = {b["id"]: b["lengthM"] for b in old_bones}
    worst, lateral = 0.0, 0.0
    for pose in poses:
        offsets = pose.get("bones", {})
        if not any(key.startswith("scapula-f") for key in offsets):
            continue
        was = frames(old_bones, offsets)
        moved = copy.deepcopy(offsets)
        for side in SIDES:
            scapula = "scapula-f" + side
            if scapula not in offsets:
                continue
            chain = ["humerus-f" + side, "forearm-f" + side, "metacarpus-f" + side]
            hoof = "hoof-f" + side
            swing = offsets[scapula]["angleDeg"] / DEG
            moved[scapula] = {"angleDeg": r4(math.asin(old_lengths[scapula] / lengths[scapula] * math.sin(swing)) * DEG)}
            target = was[hoof]

            def residual(values):
                trial = dict(moved)
                for bone, value in zip(chain, values):
                    trial[bone] = {"angleDeg": value}
                o, b = frames(bones, trial)[hoof]
                pitch = math.atan2(b[1][0], b[0][0]) - math.atan2(target[1][1][0], target[1][0][0])
                return [o[0] - target[0][0], o[1] - target[0][1], math.atan2(math.sin(pitch), math.cos(pitch)) * 0.1]

            values = [offsets.get(bone, {}).get("angleDeg", 0.0) for bone in chain]
            for _ in range(30):
                f = residual(values)
                jac = []
                for k in range(3):
                    probe = list(values)
                    probe[k] += 1e-3
                    g = residual(probe)
                    jac.append([(g[i] - f[i]) / 1e-3 for i in range(3)])
                m = transpose(jac)
                det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
                step = []
                for k in range(3):
                    c = [row[:] for row in m]
                    for i in range(3):
                        c[i][k] = -f[i]
                    step.append((c[0][0] * (c[1][1] * c[2][2] - c[1][2] * c[2][1]) - c[0][1] * (c[1][0] * c[2][2] - c[1][2] * c[2][0]) + c[0][2] * (c[1][0] * c[2][1] - c[1][1] * c[2][0])) / det)
                values = [values[k] + step[k] for k in range(3)]
                if max(abs(x) for x in step) < 1e-7:
                    break
            for bone, value in zip(chain, values):
                moved[bone] = {"angleDeg": r4(value)}
        now = frames(bones, moved)
        for side in SIDES:
            tip_now = [now["hoof-f" + side][0][i] + now["hoof-f" + side][1][i][0] * lengths["hoof-f" + side] for i in range(3)]
            tip_was = [was["hoof-f" + side][0][i] + was["hoof-f" + side][1][i][0] * lengths["hoof-f" + side] for i in range(3)]
            worst = max(worst, math.hypot(tip_now[0] - tip_was[0], tip_now[1] - tip_was[1]))
            lateral = max(lateral, abs(tip_now[2] - tip_was[2]))
        pose["bones"] = {key: moved[key] for key in list(offsets) + [k for k in moved if k not in offsets] if key in moved}
    print("largest lateral move of a fore toe in a pose: %.2f mm" % (lateral * 1000))
    return worst


def convert_shapes(representations, bones, old):
    """Elevation shapes on a scapula: the 1:50 shoulder tone patch moves to the thorax (same place on the paper; a flat shape on a bone that leaves the paper plane is ambiguous), the small 1:500 blades are stretched by the foreshortening of the tilted bone."""
    by_id = {b["id"]: b for b in bones}
    count = 0
    for representation in representations.values():
        for shape in representation["geometry"].get("elevation", []):
            bone = shape.get("bone", "")
            if not bone.startswith("scapula-f"):
                continue
            if shape.get("kind") != "poly":
                raise SystemExit("scapula elevation shape is not a poly: " + shape["id"])
            if shape["id"] == "shoulder-n":
                (so, sb), (to, tb) = old[bone], old["thorax"]
                points = []
                for x, y in shape["points"]:
                    world = [so[i] + sb[i][0] * x + sb[i][1] * y for i in range(3)]
                    local = apply(transpose(tb), sub(world, to))
                    points.append([round(local[0], 5), round(local[1], 5)])
                shape["points"], shape["bone"] = points, "thorax"
            else:
                stretch = 1 / math.cos(by_id[bone]["abductionDeg"] / DEG)
                shape["points"] = [[round(p[0] * stretch, 5), p[1]] for p in shape["points"]]
            count += 1
    return count


def main():
    asset = io.load()
    source = json.load(open(SOURCE, encoding="utf-8"))
    if any(b["id"] == "ilium-hn" for b in asset["rig"]["bones"]):
        raise SystemExit("already converted")
    old_bones = asset["rig"]["bones"]
    bones = new_rig(old_bones)
    old, new = frames(old_bones), frames(bones)
    for bone in old:
        if not bone.startswith("scapula-f"):
            drift = max(norm(sub(old[bone][0], new[bone][0])), max(abs(old[bone][1][i][j] - new[bone][1][i][j]) for i in range(3) for j in range(3)))
            assert drift < 2e-4, (bone, drift)
    rig = dict(asset["rig"], bones=bones)
    volumes, src = copy.deepcopy(asset["volumes"]), copy.deepcopy(source)
    print("solids re-bound:", convert_volumes(volumes, old, new), "catalog,", convert_volumes(src["volumes"], old, new), "source")
    poses = copy.deepcopy(asset["poses"])
    print("largest sagittal move of a fore toe in a pose: %.2f mm" % (convert_poses(poses, old_bones, bones) * 1000))
    representations = copy.deepcopy(asset["representations"])
    print("scapula elevation shapes stretched:", convert_shapes(representations, bones, old))
    for bone in bones:
        if bone["id"].startswith(("scapula-f", "humerus-f", "ilium-h", "thigh-h")):
            print(json.dumps({k: v for k, v in bone.items() if k not in ("title", "mass", "joint")}))
    if "--apply" in sys.argv:
        io.put({"rig": rig, "poses": poses, "representations": representations, "volumes": volumes})
        open(SOURCE, "w", encoding="utf-8").write(json.dumps(src, ensure_ascii=False, separators=(",", ":")))
        print("applied")
        return
    document = json.load(open(io.FILE, encoding="utf-8"))
    document["asset"].update({"rig": rig, "poses": poses, "representations": representations, "volumes": src["volumes"]})
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    open(CANDIDATE, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
    print(CANDIDATE)


main()
