"""🐴 Horse girdles on the lateral rest placement: scapulae and hip bones at their true breadths, limbs through their joints (see rig-lateral-rest.md)."""
import copy
import json
import math
import sys

T = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"
SOURCE = f"{T}/volumes/animals/horse.json"
CANDIDATE = f"{T}/🗑️generated/rework-horse/cand/🔣️.json"

FORE = ["scapula", "humerus", "radius", "cannon", "pastern", "hoof"]
HIND = ["thigh", "shank", "cannon", "pastern", "hoof"]
FORE_Z = [0.09, 0.19, 0.186, 0.156, 0.144, 0.142, 0.142]
HIND_Z = [0.15, 0.19, 0.155, 0.147, 0.146, 0.146]
SACROILIAC = [0.132, 0.153, 0.06]
TUBER_SACRALE = [0.15, 0.225, 0.04]
TUBER_COXAE = [0.27, 0.085, 0.225]
TUBER_ISCHII = [-0.25, -0.014, 0.10]
SIDES = (("n", -1), ("f", 1))
SIDE_NAME = {"n": ("near", "Near", "Nahes"), "f": ("far", "Far", "Fernes")}


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def tr(a):
    return [[a[j][i] for j in range(3)] for i in range(3)]


def app(a, p):
    return [sum(a[i][k] * p[k] for k in range(3)) for i in range(3)]


def rz(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def ry(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rx(d):
    c, s = math.cos(math.radians(d)), math.sin(math.radians(d))
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def euler(angle, abduction=0.0, twist=0.0):
    return mul(mul(rz(angle), ry(-abduction)), rx(-twist))


def rest(bone):
    return euler(bone["angleDeg"], bone.get("abductionDeg", 0.0), bone.get("twistDeg", 0.0))


def frames(bones):
    out = {}
    by_id = {b["id"]: b for b in bones}

    def solve(bone_id):
        if bone_id in out:
            return out[bone_id]
        bone = by_id[bone_id]
        if bone.get("parent"):
            origin_p, rot_p = solve(bone["parent"])
        else:
            origin_p, rot_p = [0.0, 0.0, 0.0], [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
        local = [bone.get("x", 0.0), bone.get("y", 0.0), bone.get("z", 0.0)]
        moved = app(rot_p, local)
        out[bone_id] = ([origin_p[i] + moved[i] for i in range(3)], mul(rot_p, rest(bone)))
        return out[bone_id]

    for b in bones:
        solve(b["id"])
    return out


def norm(v):
    length = math.sqrt(sum(c * c for c in v))
    return [c / length for c in v]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def triad(u, w_hint=(0.0, 0.0, 1.0)):
    u = norm(u)
    d = sum(u[i] * w_hint[i] for i in range(3))
    w = norm([w_hint[i] - d * u[i] for i in range(3)])
    v = cross(w, u)
    return [[u[i], v[i], w[i]] for i in range(3)]


def decompose(m):
    b = math.degrees(math.asin(max(-1.0, min(1.0, m[2][0]))))
    a = math.degrees(math.atan2(m[1][0], m[0][0]))
    rest_twist = mul(mul(ry(b), rz(-a)), m)
    t = math.degrees(math.atan2(-rest_twist[2][1], rest_twist[1][1]))
    return a, b, t


def quaternion(m):
    trace = m[0][0] + m[1][1] + m[2][2]
    if trace > 0:
        s = math.sqrt(trace + 1.0) * 2
        return [(m[2][1] - m[1][2]) / s, (m[0][2] - m[2][0]) / s, (m[1][0] - m[0][1]) / s, 0.25 * s]
    if m[0][0] > m[1][1] and m[0][0] > m[2][2]:
        s = math.sqrt(1.0 + m[0][0] - m[1][1] - m[2][2]) * 2
        return [0.25 * s, (m[0][1] + m[1][0]) / s, (m[0][2] + m[2][0]) / s, (m[2][1] - m[1][2]) / s]
    if m[1][1] > m[2][2]:
        s = math.sqrt(1.0 + m[1][1] - m[0][0] - m[2][2]) * 2
        return [(m[0][1] + m[1][0]) / s, 0.25 * s, (m[1][2] + m[2][1]) / s, (m[0][2] - m[2][0]) / s]
    s = math.sqrt(1.0 + m[2][2] - m[0][0] - m[1][1]) * 2
    return [(m[0][2] + m[2][0]) / s, (m[1][2] + m[2][1]) / s, 0.25 * s, (m[1][0] - m[0][1]) / s]


def product(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def r5(values):
    return [round(v, 5) + 0.0 for v in values]


def rebind(solid, old, new):
    turn = mul(tr(new[1]), old[1])
    shift = app(tr(new[1]), [old[0][i] - new[0][i] for i in range(3)])
    q = quaternion(turn)

    def point(p):
        moved = app(turn, p)
        return r5([moved[i] + shift[i] for i in range(3)])

    if solid["kind"] == "capsule":
        solid["p0"], solid["p1"] = point(solid["p0"]), point(solid["p1"])
    elif solid["kind"] == "loft":
        for section in solid["sections"]:
            section["at"] = point(section["at"])
            section["rotation"] = r5(product(q, section["rotation"]))
    elif solid["kind"] == "ellipsoid":
        transform = solid["transform"]
        transform["translation"] = point(transform["translation"])
        transform["rotation"] = r5(product(q, transform.get("rotation", [0, 0, 0, 1])))
    else:
        raise SystemExit(f"solid {solid['id']} of kind {solid['kind']} on {solid['bone']} is not converted")


def place(bone, parent_frame, origin, tip):
    rot = triad([tip[i] - origin[i] for i in range(3)])
    local = app(tr(parent_frame[1]), [origin[i] - parent_frame[0][i] for i in range(3)])
    a, b, t = decompose(mul(tr(parent_frame[1]), rot))
    fields = {"x": round(local[0], 5), "y": round(local[1], 5), "z": round(local[2], 5), "angleDeg": round(a, 5), "abductionDeg": round(b, 5), "twistDeg": round(t, 5), "lengthM": round(math.dist(origin, tip), 5)}
    for key in ("z", "abductionDeg", "twistDeg"):
        if abs(fields[key]) < 1e-5:
            fields.pop(key)
            bone.pop(key, None)
    bone.update(fields)
    ordered = {key: bone[key] for key in ("id", "parent", "title", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM", "joint") if key in bone}
    ordered.update({key: value for key, value in bone.items() if key not in ordered})
    bone.clear()
    bone.update(ordered)


def world_loft(frame, stations):
    sections = []
    for at, ex, ez, half_x, half_y, exponent in stations:
        ez = norm(ez)
        d = sum(ex[i] * ez[i] for i in range(3))
        ex = norm([ex[i] - d * ez[i] for i in range(3)])
        ey = cross(ez, ex)
        local = mul(tr(frame[1]), [[ex[i], ey[i], ez[i]] for i in range(3)])
        point = app(tr(frame[1]), [at[i] - frame[0][i] for i in range(3)])
        sections.append({"at": r5(point), "rx": half_x, "ry": half_y, "exponent": exponent, "rotation": r5(quaternion(local))})
    return sections


def side_point(p, sign):
    return [p[0], p[1], sign * p[2]]


def pelvis_half(frame, side, sign, scale):
    name = SIDE_NAME[side][0]
    bone = f"hip-bone-h{side}"
    base = {"bone": bone, "material": "short-coat", "role": "body", "tone": 0.85}
    tail = {"member": bone, "part": "member"}
    crest = [TUBER_COXAE[i] - TUBER_SACRALE[i] for i in range(3)]
    crest = side_point(crest, sign)
    run = side_point([-0.2, -0.14, 0.01], sign)
    lateral = [0.0, 0.0, float(sign)]
    back = side_point([-1.0, -0.02, -0.17], sign)
    thin = {"50": 0.011, "100": 0.012, "200": 0.016}[scale]
    wing = [
        (side_point([0.21, 0.155, 0.1325], sign), crest, run, 0.125, thin, 2.6),
        (side_point([0.15, 0.105, 0.135], sign), crest, run, 0.085, thin + 0.002, 2.4),
        (side_point([0.075, 0.05, 0.146], sign), crest, run, 0.036, 0.02, 2.0),
        (side_point([0.012, 0.008, 0.15], sign), crest, run, 0.045, 0.036, 2.0),
        (side_point([-0.022, -0.014, 0.15], sign), crest, run, 0.04, 0.032, 2.0),
    ]
    plate = [
        (side_point([-0.015, -0.01, 0.15], sign), lateral, back, 0.034, 0.028, 2.0),
        (side_point([-0.1, -0.026, 0.125], sign), lateral, back, 0.046, thin + 0.002, 2.4),
        (side_point([-0.19, -0.03, 0.107], sign), lateral, back, 0.052, thin + 0.002, 2.4),
        (side_point(TUBER_ISCHII, sign), lateral, back, 0.04, 0.022, 2.0),
    ]
    if scale != "50":
        wing = [wing[0], wing[1], wing[3], wing[4]]
        plate = [plate[0], plate[2], plate[3]]
    to_local = lambda p: r5(app(tr(frame[1]), [p[i] - frame[0][i] for i in range(3)]))
    joint = [] if scale == "200" else [{"id": f"j-{bone}", "kind": "ellipsoid", **{**base, "tone": 0.5}, "radii": [0.02, 0.02, 0.02], "transform": {"translation": [0.0, 0.0, 0.0], "rotation": [0.0, 0.0, 0.0, 1.0]}, "member": bone, "part": "joint"}]
    girth = 0.03 if scale == "200" else 0.022
    return joint + [
        {"id": f"{name}-sacroiliac", "kind": "capsule", **base, "p0": to_local(side_point([0.132, 0.15, 0.045], sign)), "p1": to_local(side_point([0.16, 0.125, 0.125], sign)), "r0": girth, "r1": girth, **tail},
        {"id": f"{name}-ilium", "kind": "loft", **base, "sections": world_loft(frame, wing), "capStart": True, "capEnd": True, **tail},
        {"id": f"{name}-iliac-crest", "kind": "capsule", **base, "p0": to_local(side_point(TUBER_SACRALE, sign)), "p1": to_local(side_point(TUBER_COXAE, sign)), "r0": 0.018, "r1": 0.024, **tail},
        {"id": f"{name}-ischium", "kind": "loft", **base, "sections": world_loft(frame, plate), "capStart": True, "capEnd": True, **tail},
    ]


def chain_bones(side):
    return [f"{b}-f{side}" for b in FORE], [f"{b}-h{side}" for b in HIND]


def sagittal_angle(rot):
    return math.atan2(rot[1][0], rot[0][0])


def resolve_offsets(old_by, new_by, parents_frame_new, chain, offsets):
    """Offsets for the new chain that give every bone the sagittal direction it had with the old planar chain."""
    result = {}
    old_sum = 0.0
    rot_parent = parents_frame_new
    for bone_id in chain:
        theta_old = offsets.get(bone_id, {}).get("angleDeg", 0.0)
        old_sum += old_by[bone_id]["angleDeg"] + theta_old
        base = mul(rot_parent, rest(new_by[bone_id]))

        def miss(theta):
            delta = sagittal_angle(mul(base, rz(theta))) - math.radians(old_sum)
            return math.degrees(math.atan2(math.sin(delta), math.cos(delta)))

        theta = theta_old
        for _ in range(40):
            f0 = miss(theta)
            if abs(f0) < 1e-9:
                break
            slope = (miss(theta + 1e-3) - f0) / 1e-3
            theta -= f0 / slope
        result[bone_id] = round(theta, 5)
        rot_parent = mul(base, rz(theta))
    return result


def main():
    write = "--write" in sys.argv
    document = json.load(open(CATALOG))
    asset = document["asset"]
    bones = asset["rig"]["bones"]
    if any(b["id"].startswith("hip-bone") for b in bones):
        raise SystemExit("already converted")
    old_bones = copy.deepcopy(bones)
    old_by = {b["id"]: b for b in old_bones}
    old = frames(old_bones)
    by_id = {b["id"]: b for b in bones}
    changed = []
    for side, sign in SIDES:
        fore, hind = chain_bones(side)
        hip = {
            "id": f"hip-bone-h{side}", "parent": "pelvis",
            "title": {"en": f"{SIDE_NAME[side][1]} hip bone (ilium, ischium, pubis)", "de": f"{SIDE_NAME[side][2]} Hüftbein (Darmbein, Sitzbein, Schambein)"},
            "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0,
            "joint": {"type": "revolute", "minDeg": -3, "maxDeg": 3},
        }
        bones.insert(bones.index(by_id[hind[0]]), hip)
        by_id[hip["id"]] = hip
        place(hip, old["pelvis"], side_point(SACROILIAC, sign), [0.0, 0.0, sign * HIND_Z[0]])
        by_id[hind[0]]["parent"] = hip["id"]
        for chain, breadth in ((fore, FORE_Z), (hind, HIND_Z)):
            for index, bone_id in enumerate(chain):
                now = frames(bones)
                origin_old, rot_old = old[bone_id]
                tip_old = [origin_old[i] + rot_old[i][0] * old_by[bone_id]["lengthM"] for i in range(3)]
                origin = [origin_old[0], origin_old[1], sign * breadth[index]]
                tip = [tip_old[0], tip_old[1], sign * breadth[index + 1]]
                place(by_id[bone_id], now[by_id[bone_id]["parent"]], origin, tip)
                changed.append(bone_id)
    new = frames(bones)
    cosines = {}
    for bone_id in changed:
        cosines[bone_id] = old_by[bone_id]["lengthM"] / by_id[bone_id]["lengthM"]
        mass = by_id[bone_id].get("mass")
        if mass and "comX" in mass:
            mass["comX"] = round(mass["comX"] / cosines[bone_id], 5)
    for anchor in asset["anchors"]:
        if anchor["bone"] in cosines:
            anchor["x"] = round(anchor["x"] / cosines[anchor["bone"]], 5)
    for representation in asset["representations"].values():
        for shape in representation["geometry"]["elevation"]:
            if shape.get("bone") in cosines and abs(cosines[shape["bone"]] - 1) > 1e-9:
                if shape["kind"] != "poly":
                    raise SystemExit(f"elevation shape {shape['id']} of kind {shape['kind']}")
                shape["points"] = [[round(p[0] / cosines[shape["bone"]], 5), p[1]] for p in shape["points"]]
    hip_ids = ("ilium", "iliac-crest", "ischium", "hip-bone")
    for scale, lists in asset["volumes"].items():
        for name in ("body", "thermal", "structure"):
            kept = []
            for solid in lists[name]:
                bone = solid.get("bone")
                if "kind" in solid and bone in cosines:
                    rebind(solid, old[bone], new[bone])
                if "kind" in solid and bone == "pelvis" and name == "structure":
                    side = {"near": "n", "far": "f"}.get(solid["id"].split("-")[0])
                    if side and solid["id"].split("-", 1)[1] in hip_ids:
                        if scale in ("50", "100", "200"):
                            continue
                        hip = f"hip-bone-h{side}"
                        rebind(solid, old["pelvis"], new[hip])
                        solid["bone"] = hip
                        solid["member"] = hip
                kept.append(solid)
            if name == "structure" and scale in ("50", "100", "200"):
                index = next(i for i, s in enumerate(kept) if s["id"] == "sacrum") + 1
                added = []
                for side, sign in SIDES:
                    added += pelvis_half(new[f"hip-bone-h{side}"], side, sign, scale)
                kept[index:index] = added
            lists[name] = kept
    largest = 0.0
    for side, sign in SIDES:
        fore, hind = chain_bones(side)
        for chain, parent in ((fore, "thorax"), (hind, f"hip-bone-h{side}")):
            start = new[parent][1] if parent != "thorax" else new["thorax"][1]
            start_old_frame = old["thorax"][1] if parent == "thorax" else old["pelvis"][1]
            lead = math.degrees(sagittal_angle(start_old_frame))
            shifted_old = dict(old_by)

            def solve(offsets):
                first = dict(old_by[chain[0]])
                first["angleDeg"] = first["angleDeg"] + lead
                local_old = {**{b: old_by[b] for b in chain}, chain[0]: first}
                return resolve_offsets(local_old, by_id, start, chain, offsets)

            pose_new = {}
            for pose in asset["poses"]:
                offsets = pose.get("bones", {})
                solved = solve(offsets)
                pose_new[pose["id"]] = solved
                for bone_id, theta in solved.items():
                    before = offsets.get(bone_id, {}).get("angleDeg", 0.0)
                    largest = max(largest, abs(theta - before))
                    if bone_id in offsets or abs(theta) > 1e-4:
                        offsets.setdefault(bone_id, {})["angleDeg"] = theta
                        joint = by_id[bone_id]["joint"]
                        if not joint["minDeg"] - 1e-6 <= theta <= joint["maxDeg"] + 1e-6:
                            print("[DEBUG] limit", pose["id"], bone_id, theta, joint["minDeg"], joint["maxDeg"])
                pose["bones"] = offsets
            for clip in asset["clips"]:
                for key in clip["keys"]:
                    if "bones" not in key:
                        continue
                    base_old = {}
                    if key.get("pose"):
                        base_old = {b: {"angleDeg": v} for b, v in old_pose_offsets[key["pose"]].items()}
                    effective = {**base_old, **{b: o for b, o in key["bones"].items()}}
                    solved = solve(effective)
                    inherited = pose_new.get(key.get("pose"), {})
                    for bone_id, theta in solved.items():
                        before = effective.get(bone_id, {}).get("angleDeg", 0.0)
                        largest = max(largest, abs(theta - before))
                        if bone_id in key["bones"] or abs(theta - inherited.get(bone_id, 0.0)) > 1e-4:
                            key["bones"].setdefault(bone_id, {})["angleDeg"] = theta
    print("[DEBUG] largest offset change (deg)", round(largest, 4))
    for bone_id in ["hip-bone-hn", "hip-bone-hf"] + changed[:11]:
        print("[DEBUG]", json.dumps({k: v for k, v in by_id[bone_id].items() if k in ("id", "parent", "x", "y", "z", "angleDeg", "abductionDeg", "twistDeg", "lengthM")}))
    import os
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(document, open(CANDIDATE, "w"), ensure_ascii=False, indent=2)
    if write:
        fresh = json.load(open(CATALOG))
        if fresh["asset"]["rig"] != {"kind": asset["rig"]["kind"], "root": asset["rig"]["root"], "bones": old_bones} and fresh["asset"]["rig"]["bones"] != old_bones:
            raise SystemExit("catalog rig changed under the script")
        json.dump(document, open(CATALOG, "w"), ensure_ascii=False, indent=2)
        json.dump({"volumes": asset["volumes"]}, open(SOURCE, "w"), ensure_ascii=False, indent=1)
        print("written")


old_pose_offsets = {}
if __name__ == "__main__":
    for pose in json.load(open(CATALOG))["asset"]["poses"]:
        old_pose_offsets[pose["id"]] = {b: o.get("angleDeg", 0.0) for b, o in pose.get("bones", {}).items()}
    main()
