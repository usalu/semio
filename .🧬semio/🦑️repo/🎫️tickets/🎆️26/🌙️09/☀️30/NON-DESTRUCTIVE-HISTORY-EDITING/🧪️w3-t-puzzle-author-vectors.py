#!/usr/bin/env python3
"""🧫️ W3-T-PUZZLE authoring tool: writes the selection-transform fixture quintets of puzzle 3d and puzzle 5d and
their Rust leaf tests. The transform arithmetic mirrors the Rust leaves operation for operation (IEEE doubles, same
evaluation order), so the committed after-snapshots are exactly what the Rust diff builders must reproduce. Re-running
it reproduces every committed file byte for byte. Usage: `🧪️w3-t-puzzle-author-vectors.py 3d|5d`."""
import copy
import json
import math
import os
import sys

PUZZLE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts"
SUBSET = {"3d": PUZZLE + "/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any", "5d": PUZZLE + "/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any"}
IDENTITY = [0.0, 0.0, 0.0, 1.0]
QUARTER = math.pi / 2
FLAT_TO_WORLD = 1.0 / 48.0


def quat_mul(a, b):
    return [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]


def quat_from_axis_angle(ax, ay, az, angle):
    length = math.sqrt(ax * ax + ay * ay + az * az)
    if length < 1e-8:
        return list(IDENTITY)
    half = angle * 0.5
    s = math.sin(half)
    return [ax / length * s, ay / length * s, az / length * s, math.cos(half)]


def scaled(scale, factors):
    current = [1.0, 1.0, 1.0] if scale is None else ([float(scale)] * 3 if isinstance(scale, float) else list(scale))
    return [current[0] * factors[0], current[1] * factors[1], current[2] * factors[2]]


# region 🔖️AttractionPose
def vec3_sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def vec3_add(a, b):
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def vec3_scale(a, s):
    return [a[0] * s, a[1] * s, a[2] * s]


def vec3_cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def vec3_dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def vec3_len(a):
    return math.sqrt(vec3_dot(a, a))


def vec3_normalize(a):
    length = vec3_len(a)
    return a if length < 1e-12 else vec3_scale(a, 1.0 / length)


def quat_conjugate(q):
    return [-q[0], -q[1], -q[2], q[3]]


def quat_normalize(q):
    length = math.sqrt(q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
    return list(IDENTITY) if length < 1e-12 else [q[0] / length, q[1] / length, q[2] / length, q[3] / length]


def quat_rotate_vector(quat, vector):
    x, y, z, w = quat
    vx, vy, vz = vector
    ix = w * vx + y * vz - z * vy
    iy = w * vy + z * vx - x * vz
    iz = w * vz + x * vy - y * vx
    iw = -x * vx - y * vy - z * vz
    return [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]


def quaternion_from_unit_vectors(source, target):
    r = vec3_dot(source, target) + 1.0
    if r < 0.000001:
        quat = [-source[1], source[0], 0.0, 0.0] if abs(source[0]) > abs(source[2]) else [0.0, -source[2], source[1], 0.0]
    else:
        c = vec3_cross(source, target)
        quat = [c[0], c[1], c[2], r]
    return quat_normalize(quat)


def attraction_align_quat(parent_dir, child_dir):
    reverse_child = vec3_scale(child_dir, -1.0)
    if vec3_len(vec3_cross(parent_dir, reverse_child)) < 0.01:
        if abs(parent_dir[2]) < 0.01:
            return quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        axis = vec3_cross([0.0, 0.0, 1.0], parent_dir)
        if vec3_len(axis) < 1e-9:
            axis = vec3_cross([1.0, 0.0, 0.0], parent_dir)
        axis = vec3_normalize(axis)
        half = math.pi / 2
        return quat_normalize([axis[0] * math.sin(half), axis[1] * math.sin(half), axis[2] * math.sin(half), math.cos(half)])
    return quaternion_from_unit_vectors(reverse_child, parent_dir)


def derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b):
    """🧮️ The six connection parameters reproducing the attracted pose — operation for operation the Rust kernel."""
    parent_dir = vec3_normalize(d_a)
    child_dir = vec3_normalize(d_b)
    align_q = attraction_align_quat(parent_dir, child_dir)
    pq = quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir)
    gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0])
    shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0])
    raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0])
    orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)))
    offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b))
    diff = vec3_sub(vec3_sub(offset, t_a), p_a)
    gap, shift, rise = vec3_dot(diff, gap_dir), vec3_dot(diff, shift_dir), vec3_dot(diff, raise_dir)
    m = quat_mul(quat_mul(quat_conjugate(pq), quat_mul(align_q, orientation_local)), pq)
    col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0])
    col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0])
    tilt_rad = -math.asin(max(-1.0, min(1.0, col_y[2])))
    if abs(abs(col_y[2]) - 1.0) < 1e-6:
        rotation_rad, turn_rad = math.atan2(col_x[1], col_x[0]), 0.0
    else:
        col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0])
        rotation_rad, turn_rad = math.atan2(-col_x[2], col_z[2]), math.atan2(col_y[0], col_y[1])
    degrees = lambda rad: rad * 180.0 / math.pi
    return gap, shift, rise, degrees(rotation_rad), degrees(turn_rad), degrees(tilt_rad)


def full_id(object_id, vortex_id):
    return vortex_id if ":" in vortex_id else "%s:%s" % (object_id, vortex_id)


def attraction_child_pose(t_a, q_a, p_a, d_a, p_b, d_b, gap, shift, rise, rotation_deg, turn_deg, tilt_deg):
    """📐️ The attracted pose the six parameters place against the attracting pose — `puzzle3d_attraction_child_pose`."""
    parent_dir = vec3_normalize(d_a)
    child_dir = vec3_normalize(d_b)
    align_q = attraction_align_quat(parent_dir, child_dir)
    pq = quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir)
    gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0])
    shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0])
    raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0])
    radians = lambda deg: deg * math.pi / 180.0
    rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -radians(rotation_deg))
    turn_axis = quat_rotate_vector(rotate_q, raise_dir)
    tilt_axis = quat_rotate_vector(rotate_q, shift_dir)
    turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], radians(turn_deg))
    tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], radians(tilt_deg))
    local = quat_conjugate(align_q)
    local = quat_mul(local, quat_conjugate(rotate_q))
    local = quat_mul(local, quat_conjugate(turn_q))
    local = quat_mul(local, quat_conjugate(tilt_q))
    local = quat_normalize(local)
    offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)))
    return vec3_sub(quat_rotate_vector(local, offset), p_b), quat_normalize(quat_mul(local, q_a))


def follow_3d(scene, targets, survivors, transform, follow):
    """🌲️ `puzzle3d_selection_follow`: targets first (payload order), then a breadth-first re-placement of every
    unlocked attracted object from its moved parent, then every other attraction touching a moved object re-derived."""
    objects = scene["objects"]
    moved = [None] * len(objects)
    queue, followers = [], []
    for identifier in targets:
        if identifier not in survivors:
            continue
        at = next((index for index, entry in enumerate(objects) if entry["id"] == identifier), None)
        if at is not None and moved[at] is None:
            moved[at] = transform(objects[at])
            queue.append(at)
    if not follow:
        return moved, followers, []
    ports = {full_id(entry["id"], vortex["id"]): (at, port) for at, entry in enumerate(objects) for port, vortex in enumerate(entry["vortices"])}
    ends = []
    for attraction in scene["attractions"]:
        source, target = ports.get(attraction["attracting"]), ports.get(attraction["attracted"])
        ends.append((source, target) if source is not None and target is not None and source[0] != target[0] else None)
    placing = [False] * len(ends)
    while queue:
        parent = queue.pop(0)
        for index, end in enumerate(ends):
            if end is None or end[0][0] != parent or moved[end[1][0]] is not None or objects[end[1][0]]["locked"]:
                continue
            (_, port_a), (child, port_b) = end
            attracting, attraction, attracted = moved[parent], scene["attractions"][index], objects[child]
            vortex_a, vortex_b = attracting["vortices"][port_a], attracted["vortices"][port_b]
            origin, orientation = attraction_child_pose(attracting["origin"], attracting.get("orientation") or IDENTITY, vortex_a["position"], vortex_a.get("direction") or [0.0, 0.0, -1.0], vortex_b["position"], vortex_b.get("direction") or [0.0, 0.0, -1.0], attraction["gap"], attraction["shift"], attraction["rise"], attraction["rotation"], attraction["turn"], attraction["tilt"])
            moved[child] = dict(attracted, origin=origin, orientation=orientation)
            followers.append(attracted["id"])
            placing[index] = True
            queue.append(child)
    pose = lambda at: objects[at] if moved[at] is None else moved[at]
    rederived = []
    for index, (attraction, end) in enumerate(zip(scene["attractions"], ends)):
        if end is None or placing[index]:
            continue
        (a, port_a), (b, port_b) = end
        if moved[a] is None and moved[b] is None:
            continue
        attracting, attracted = pose(a), pose(b)
        vortex_a, vortex_b = attracting["vortices"][port_a], attracted["vortices"][port_b]
        gap, shift, rise, rotation, turn, tilt = derive_attraction_params(attracting["origin"], attracting.get("orientation") or IDENTITY, vortex_a["position"], vortex_a.get("direction") or [0.0, 0.0, -1.0], vortex_b["position"], vortex_b.get("direction") or [0.0, 0.0, -1.0], attracted["origin"], attracted.get("orientation") or IDENTITY)
        updated = dict(attraction, gap=gap, shift=shift, rise=rise, rotation=rotation, turn=turn, tilt=tilt)
        if updated != attraction:
            rederived.append(updated)
    return moved, followers, rederived
# endregion 🔖️AttractionPose


# region 🔖️Puzzle3d
SCENE_3D = {
    "schema": "puzzle.3d",
    "domain": "architecture",
    "meta": {"kindCompatibility": [{"source": "vortex-kind-a", "target": "vortex-kind-b", "bidirectional": True, "important": False, "specificity": "vortex"}]},
    "objects": [
        {"id": "object-a", "label": "Alpha", "objectKind": "object-kind-a", "anchor": "fixed", "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0], "scale": 1.0, "meshUrl": "mesh://alpha", "vortices": [{"id": "vortex-1", "vortexKind": "vortex-kind-a", "position": [1.0, 0.0, 0.0], "hidden": False, "locked": False}], "hidden": False, "locked": False},
        {"id": "object-b", "objectKind": "object-kind-b", "anchor": "fixed", "origin": [4.0, 0.0, 0.0], "vortices": [{"id": "vortex-2", "vortexKind": "vortex-kind-b", "position": [-1.0, 0.0, 0.0], "hidden": False, "locked": False}], "hidden": False, "locked": False},
        {"id": "object-c", "objectKind": "object-kind-b", "anchor": "fixed", "origin": [8.0, 0.0, 0.0], "vortices": [], "hidden": False, "locked": True},
    ],
    "attractions": [{"id": "attraction-1", "attracting": "object-a:vortex-1", "attracted": "object-b:vortex-2", "gap": 1.0, "shift": 0.0, "rise": 0.0, "rotation": 0.0, "turn": 0.0, "tilt": 0.0, "x": 0.0, "y": 0.0}],
    "targetVolumes": [
        {"id": "volume-1", "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0], "scale": [2.0, 2.0, 2.0], "hidden": False, "locked": False},
        {"id": "volume-2", "origin": [5.0, 0.0, 0.0], "scale": 1.5, "hidden": False, "locked": True},
    ],
    "references": [{"id": "reference-1", "source": {"url": "asset://plan.png", "mediaKind": "image"}, "origin": [0.0, 0.0, 0.0], "widthWorld": 10.0, "locked": False, "hidden": False}],
}


def transform_3d(payload):
    """🧮️ `(object transform, volume transform, identity, follow)` — whether the leaf moves poses, so attracted
    objects follow (a scaling does not)."""
    kind = payload["mutation"]
    if kind == "dragSelection":
        dx, dy, dz = payload["offset"]
        move = lambda record: dict(record, origin=[record["origin"][0] + dx, record["origin"][1] + dy, record["origin"][2] + dz])
        return move, move, payload["offset"] == [0.0, 0.0, 0.0], True
    if kind == "rotateSelection":
        turn = quat_from_axis_angle(payload["axis"][0], payload["axis"][1], payload["axis"][2], payload["angle"])
        rotate = lambda record: dict(record, orientation=quat_mul(turn, record.get("orientation") or IDENTITY))
        return rotate, rotate, payload["angle"] == 0.0 or turn == IDENTITY, True
    factors = payload["factors"]
    grow = lambda record: dict(record, scale=scaled(record.get("scale"), factors))
    return grow, grow, factors == [1.0, 1.0, 1.0], False


def outcome_3d(scene, payload):
    """🎯️ The same classification, attraction re-solve and outcome rules as `puzzle3d_selection_diff`."""
    move_object, move_volume, identity, follow = transform_3d(payload)
    targets = payload["targets"]
    missing, locked, survivors = [], [], []
    for identifier in targets:
        record = next((entry for entry in scene["objects"] if entry["id"] == identifier), None) or next((entry for entry in scene["targetVolumes"] if entry["id"] == identifier), None)
        if record is None:
            missing.append(identifier)
        elif record["locked"]:
            locked.append(identifier)
        else:
            survivors.append(identifier)
    if not survivors:
        return None, {"status": "rejected", "code": "mutation.target-missing", "path": list(targets)}
    messages = [{"code": "mutation.partial", "level": "warning", "target": ids} for ids in (missing, locked) if ids]
    after = copy.deepcopy(scene)
    patched = {"objects": [], "targetVolumes": [], "attractions": []}
    followers = []
    if not identity:
        moved, followers, rederived = follow_3d(scene, targets, set(survivors), move_object, follow)
        for at, record in enumerate(scene["objects"]):
            if moved[at] is not None and moved[at] != record:
                after["objects"][at] = moved[at]
                patched["objects"].append({"id": record["id"], "patch": {"replacement": moved[at]}})
        for at, record in enumerate(scene["targetVolumes"]):
            if record["id"] in survivors:
                next_record = move_volume(record)
                if next_record != record:
                    after["targetVolumes"][at] = next_record
                    patched["targetVolumes"].append({"id": record["id"], "patch": {"replacement": next_record}})
        for updated in rederived:
            after["attractions"] = [updated if entry["id"] == updated["id"] else entry for entry in after["attractions"]]
            patched["attractions"].append({"id": updated["id"], "patch": {"replacement": updated}})
    delta = lambda entries: {"added": [], "patched": entries, "removed": [], "reordered": None} if entries else None
    diff = {"artifact": None, "attractions": delta(patched["attractions"]), "domain": None, "meta": None, "objects": delta(patched["objects"]), "references": None, "schema": None, "targetVolumes": delta(patched["targetVolumes"])}
    if not patched["objects"] and not patched["targetVolumes"] and not patched["attractions"]:
        return (after, diff), {"status": "no-op", "messages": messages + [{"code": "mutation.no-op", "level": "warning", "target": list(targets)}]}
    if followers or patched["attractions"]:
        messages.append({"code": "mutation.cascade", "level": "info", "target": followers + [entry["id"] for entry in patched["attractions"]]})
    return (after, diff), dict({"status": "applied"}, **({"messages": messages} if messages else {}))


def chain_scene_3d():
    """⛓️ A resolved attraction chain for the re-solve vectors: `object-a` attracts `object-b`, which attracts the
    LOCKED `object-c` and the free `object-d`; every attraction's six parameters are derived from the committed
    poses, so the base is exactly what resolving it reproduces."""
    scene = copy.deepcopy(SCENE_3D)
    vortex = lambda identifier, position, direction: {"id": identifier, "vortexKind": "vortex-kind-a", "position": position, "direction": direction, "hidden": False, "locked": False}
    scene["objects"] = [
        {"id": "object-a", "objectKind": "object-kind-a", "anchor": "fixed", "origin": [0.0, 0.0, 0.0], "orientation": list(IDENTITY), "vortices": [vortex("vortex-1", [1.0, 0.0, 0.0], [1.0, 0.0, 0.0])], "hidden": False, "locked": False},
        {"id": "object-b", "objectKind": "object-kind-b", "anchor": "fixed", "origin": [2.0, 0.0, 0.0], "vortices": [vortex("vortex-2", [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]), vortex("vortex-3", [1.0, 0.0, 0.0], [1.0, 0.0, 0.0])], "hidden": False, "locked": False},
        {"id": "object-c", "objectKind": "object-kind-b", "anchor": "fixed", "origin": [4.0, 0.0, 0.0], "vortices": [vortex("vortex-4", [-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0])], "hidden": False, "locked": True},
        {"id": "object-d", "objectKind": "object-kind-b", "anchor": "fixed", "origin": [2.0, 2.0, 0.0], "vortices": [vortex("vortex-5", [0.0, -1.0, 0.0], [0.0, -1.0, 0.0])], "hidden": False, "locked": False},
    ]
    pairs = [("attraction-ab", "object-a", 0, "object-b", 0), ("attraction-bc", "object-b", 1, "object-c", 0), ("attraction-bd", "object-b", 1, "object-d", 0)]
    by_id = {entry["id"]: entry for entry in scene["objects"]}
    scene["attractions"] = []
    for identifier, source, source_port, target, target_port in pairs:
        a, b = by_id[source], by_id[target]
        va, vb = a["vortices"][source_port], b["vortices"][target_port]
        gap, shift, rise, rotation, turn, tilt = derive_attraction_params(a["origin"], a.get("orientation") or IDENTITY, va["position"], va["direction"], vb["position"], vb["direction"], b["origin"], b.get("orientation") or IDENTITY)
        scene["attractions"].append({"id": identifier, "attracting": "%s:%s" % (source, va["id"]), "attracted": "%s:%s" % (target, vb["id"]), "gap": gap, "shift": shift, "rise": rise, "rotation": rotation, "turn": turn, "tilt": tilt, "x": 0.0, "y": 0.0})
    return scene


CASES_3D = [
    ("✋️drag-selection", "✋️drags-two-objects", {"mutation": "dragSelection", "targets": ["object-a", "object-b"], "offset": [1.5, -2.0, 0.5]}, "Drags `object-a` and `object-b` by (1.5, -2, 0.5): both origins move by the one offset, read off the base, and `attraction-1`, which touches both, is re-derived from the moved poses (`mutation.cascade`)."),
    ("✋️drag-selection", "🎯️drags", {"mutation": "dragSelection", "targets": ["object-a", "volume-1"], "offset": [0.0, 0.0, 3.0]}, "One drag over an object AND a target volume (ids classified by document membership): `object-a` and `volume-1` both lift by 3, and `object-b`, which `object-a` attracts, is re-placed from the lifted `object-a` (Info-level `mutation.cascade`)."),
    ("✋️drag-selection", "⚠️skips-locked-ghost", {"mutation": "dragSelection", "targets": ["object-b", "object-c", "volume-2", "object-ghost"], "offset": [1.0, 1.0, 0.0]}, "`object-b` moves by (1, 1, 0) away from `object-a`, so `attraction-1` is re-derived from the moved pose (`mutation.cascade`); the absent `object-ghost` and the locked `object-c` and `volume-2` are skipped with one Warning-level `mutation.partial` per reason."),
    ("✋️drag-selection", "🚫️rejects-ghosts", {"mutation": "dragSelection", "targets": ["object-ghost", "volume-ghost"], "offset": [1.0, 0.0, 0.0]}, "Every target is absent: Error-level `mutation.target-missing`, nothing moves."),
    ("✋️drag-selection", "⏸️keeps-a-zero-offset", {"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 0.0]}, "A zero offset is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🔄️rotate-selection", "🔄️turns-two-objects", {"mutation": "rotateSelection", "targets": ["object-a", "object-b"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, "A quarter turn about +z: `object-a` turns from the identity and `object-b`, which carries no orientation, gains one; neither origin moves, and `attraction-1`, which touches both, is re-derived from the turned poses (`mutation.cascade`)."),
    ("🔄️rotate-selection", "🎯️turns", {"mutation": "rotateSelection", "targets": ["object-a", "volume-1"], "axis": [1.0, 0.0, 0.0], "angle": math.pi}, "A half turn about +x over an object AND a target volume, each about its own origin; `object-b`, which `object-a` attracts, is re-placed from the turned `object-a` (`mutation.cascade`)."),
    ("🔄️rotate-selection", "⚠️skips-locked-ghost", {"mutation": "rotateSelection", "targets": ["object-b", "volume-2", "object-ghost"], "axis": [0.0, 1.0, 0.0], "angle": QUARTER}, "`object-b` turns about +y while `object-a` stays, so `attraction-1` is re-derived (`mutation.cascade`); the absent `object-ghost` and the locked `volume-2` are skipped as `mutation.partial`."),
    ("🔄️rotate-selection", "🚫️rejects-ghosts", {"mutation": "rotateSelection", "targets": ["object-ghost"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, "The only target is absent: Error-level `mutation.target-missing`, nothing turns."),
    ("🔄️rotate-selection", "⏸️keeps-a-zero-angle", {"mutation": "rotateSelection", "targets": ["object-a"], "axis": [0.0, 0.0, 1.0], "angle": 0.0}, "A zero angle is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🔍️scale-selection", "🔍️scales-two-objects", {"mutation": "scaleSelection", "targets": ["object-a", "object-b"], "factors": [2.0, 1.0, 0.5]}, "Factors (2, 1, 0.5): `object-a`'s uniform scale becomes a per-axis triple, and `object-b`, which carries no scale, reads as one."),
    ("🔍️scale-selection", "🎯️scales", {"mutation": "scaleSelection", "targets": ["object-a", "volume-1"], "factors": [3.0, 3.0, 3.0]}, "A uniform factor 3 over an object AND a target volume, each about its own origin."),
    ("🔍️scale-selection", "⚠️skips-locked-ghost", {"mutation": "scaleSelection", "targets": ["volume-1", "object-c", "object-ghost"], "factors": [0.5, 0.5, 0.5]}, "`volume-1` halves; the absent `object-ghost` and the locked `object-c` are skipped with one `mutation.partial` per reason."),
    ("🔍️scale-selection", "🚫️rejects-ghosts", {"mutation": "scaleSelection", "targets": ["object-ghost", "volume-ghost"], "factors": [2.0, 2.0, 2.0]}, "Every target is absent: Error-level `mutation.target-missing`, nothing scales."),
    ("🔍️scale-selection", "⏸️keeps-unit-factors", {"mutation": "scaleSelection", "targets": ["object-a"], "factors": [1.0, 1.0, 1.0]}, "Unit factors are a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
]

CHAIN_CASES_3D = [
    ("✋️drag-selection", "⛓️follow", {"mutation": "dragSelection", "targets": ["object-a"], "offset": [0.0, 0.0, 1.0]}, "Drags the chain root `object-a` up by 1: `object-b` and `object-d`, which hang off it, are re-placed from their moved parents with unchanged attractions; the LOCKED `object-c` stays, so `attraction-bc` is re-derived — one Info-level `mutation.cascade` names both followers and the re-derived attraction."),
    ("✋️drag-selection", "🪢️cross", {"mutation": "dragSelection", "targets": ["object-b"], "offset": [0.0, 1.0, 0.0]}, "Drags the middle link `object-b` by (0, 1, 0): `object-d` is re-placed from it, while `attraction-ab` (from the unmoved `object-a`) and `attraction-bc` (to the locked `object-c`) are re-derived from the moved pose."),
    ("🔄️rotate-selection", "⛓️orbits", {"mutation": "rotateSelection", "targets": ["object-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, "A quarter turn of the chain root `object-a` about +z: `object-b` and `object-d` are re-placed from the turned root by the placement kernel, the locked `object-c` stays and `attraction-bc` is re-derived."),
    ("🔍️scale-selection", "⛓️stays", {"mutation": "scaleSelection", "targets": ["object-a"], "factors": [2.0, 2.0, 2.0]}, "Scaling moves no pose: only `object-a`'s scale doubles, nothing follows and no attraction is re-derived."),
]

INVARIANTS_3D = [
    ("✋️drag-selection", "🧱️no-targets", {"mutation": "dragSelection", "targets": [], "offset": [1.0, 0.0, 0.0]}, [], "An empty target set is what the schema's `minItems: 1` forbids: a Fatal `mutation.invariant`, nothing moves."),
    ("🔄️rotate-selection", "🧱️repeated-targets", {"mutation": "rotateSelection", "targets": ["object-a", "object-b", "object-a"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, ["object-a", "object-b", "object-a"], "A target named twice is what the schema's `uniqueItems` forbids: a Fatal `mutation.invariant`, nothing turns."),
    ("🔍️scale-selection", "🧱️zero-factor", {"mutation": "scaleSelection", "targets": ["object-a"], "factors": [1.0, 0.0, 1.0]}, ["object-a"], "A zero factor would flatten the object; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`."),
    ("🔍️scale-selection", "⛔️negative-factor", {"mutation": "scaleSelection", "targets": ["object-a"], "factors": [-1.0, 1.0, 1.0]}, ["object-a"], "A negative factor would mirror the object; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`."),
]
# endregion 🔖️Puzzle3d


# region 🔖️Puzzle5d
def scene_5d():
    """🧱️ The committed `move-target-volume/⬆️lifts-volume-1` base, with a uniform world scale 2 on `part-a`, a third
    part `part-c` locked on the board and `volume-2` locked — the synthetic selection scene of every 5d vector."""
    path = SUBSET["5d"] + "/🧫️fixtures/🧬️mutations/🚀move-target-volume/⬆️lifts-volume-1/📸️snapshot/⬅️before/🔣️.json"
    scene = json.load(open(path, encoding="utf-8"))
    scene["parts"][0]["3d"]["scale"] = 2.0
    scene["parts"].append({"id": "part-c", "partKind": "part-kind-b", "2d": {"x": 80.0, "y": 0.0, "locked": True}, "3d": {"origin": [8.0, 0.0, 0.0]}, "grips": []})
    scene["targetVolumes"][1]["locked"] = True
    return scene


def transform_5d(payload):
    kind = payload["mutation"]
    if kind == "dragSelection2d":
        dx, dy = payload["dx"], payload["dy"]
        return (lambda part: dict(part, **{"2d": dict(part["2d"], x=part["2d"]["x"] + dx, y=part["2d"]["y"] + dy)})), None, dx == 0.0 and dy == 0.0
    if kind == "dragSelection3d":
        ox, oy, oz = payload["offset"]
        move = lambda origin: [origin[0] + ox, origin[1] + oy, origin[2] + oz]
        flat = lambda part: dict(part["2d"], x=part["2d"]["x"] + ox / FLAT_TO_WORLD, y=part["2d"]["y"] - oy / FLAT_TO_WORLD)
        return (lambda part: dict(part, **{"2d": flat(part), "3d": dict(part["3d"], origin=move(part["3d"]["origin"]))})), (lambda volume: dict(volume, origin=move(volume["origin"]))), payload["offset"] == [0.0, 0.0, 0.0]
    if kind == "rotateSelection3d":
        turn = quat_from_axis_angle(payload["axis"][0], payload["axis"][1], payload["axis"][2], payload["angle"])
        return (lambda part: dict(part, **{"3d": dict(part["3d"], orientation=quat_mul(turn, part["3d"].get("orientation") or IDENTITY))})), (lambda volume: dict(volume, orientation=quat_mul(turn, volume.get("orientation") or IDENTITY))), payload["angle"] == 0.0 or turn == IDENTITY
    factors = payload["factors"]
    return (lambda part: dict(part, **{"3d": dict(part["3d"], scale=scaled(part["3d"].get("scale"), factors))})), (lambda volume: dict(volume, scale=scaled(volume.get("scale"), factors))), factors == [1.0, 1.0, 1.0]


def outcome_5d(scene, payload):
    """🎯️ The same classification and outcome rules as `puzzle5d_selection_diff`."""
    move_part, move_volume, identity = transform_5d(payload)
    targets = payload["targets"]
    missing, locked, unreached, survivors = [], [], [], []
    for identifier in targets:
        part = next((entry for entry in scene["parts"] if entry["id"] == identifier), None)
        volume = next((entry for entry in scene.get("targetVolumes", []) if entry["id"] == identifier), None)
        if part is not None:
            (locked if part["2d"].get("locked") is True else survivors).append(identifier)
        elif volume is not None:
            if volume["locked"]:
                locked.append(identifier)
            elif move_volume is None:
                unreached.append(identifier)
            else:
                survivors.append(identifier)
        else:
            missing.append(identifier)
    if not survivors:
        return None, {"status": "rejected", "code": "mutation.target-missing", "path": list(targets)}
    messages = [{"code": "mutation.partial", "level": "warning", "target": ids} for ids in (missing, locked, unreached) if ids]
    after = copy.deepcopy(scene)
    patched = {"parts": [], "targetVolumes": []}
    if not identity:
        for member, transform in (("parts", move_part), ("targetVolumes", move_volume)):
            if transform is None:
                continue
            for at, record in enumerate(after.get(member, [])):
                if record["id"] in survivors:
                    moved = transform(record)
                    if moved != record:
                        after[member][at] = moved
                        patched[member].append({"id": record["id"], "patch": {"replacement": moved}})
    delta = lambda entries: {"added": [], "patched": entries, "removed": [], "reordered": None} if entries else None
    diff = {"artifact": None, "domain": None, "fasteners": None, "kindCatalogs": None, "kindCatalogsExtra": None, "kindCompatibility": None, "label": None, "meta": None, "parts": delta(patched["parts"]), "schema": None, "targetVolumes": delta(patched["targetVolumes"])}
    if not patched["parts"] and not patched["targetVolumes"]:
        return (after, diff), {"status": "no-op", "messages": messages + [{"code": "mutation.no-op", "level": "warning", "target": list(targets)}]}
    return (after, diff), dict({"status": "applied"}, **({"messages": messages} if messages else {}))


CASES_5D = [
    ("✋️drag-selection2d", "✋️drags-two-parts", {"mutation": "dragSelection2d", "targets": ["part-a", "part-b"], "dx": 5.0, "dy": -2.5}, "Drags the board projections of `part-a` and `part-b` by (5, -2.5); their world poses stay."),
    ("✋️drag-selection2d", "⚠️skips", {"mutation": "dragSelection2d", "targets": ["part-a", "part-c", "volume-1", "part-ghost"], "dx": 1.0, "dy": 1.0}, "`part-a` moves on the board; the absent `part-ghost`, the locked `part-c` and `volume-1`, which the board does not paint, are skipped with one `mutation.partial` per reason."),
    ("✋️drag-selection2d", "🚫️rejects-ghosts", {"mutation": "dragSelection2d", "targets": ["part-ghost"], "dx": 1.0, "dy": 0.0}, "The only target is absent: Error-level `mutation.target-missing`, nothing moves."),
    ("✋️drag-selection2d", "⏸️keeps-a-zero-offset", {"mutation": "dragSelection2d", "targets": ["part-a"], "dx": 0.0, "dy": 0.0}, "A zero offset is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🚚️drag-selection3d", "🚚️drags", {"mutation": "dragSelection3d", "targets": ["part-a", "volume-1"], "offset": [1.5, -2.0, 0.5]}, "Drags `part-a`'s world origin and `volume-1` by (1.5, -2, 0.5); the board projection stays."),
    ("🚚️drag-selection3d", "⚠️skips-locked-ghost", {"mutation": "dragSelection3d", "targets": ["part-b", "part-c", "volume-2", "part-ghost"], "offset": [0.0, 0.0, 3.0]}, "`part-b` lifts by 3; the absent `part-ghost` and the locked `part-c` and `volume-2` are skipped as `mutation.partial`."),
    ("🚚️drag-selection3d", "🚫️rejects-ghosts", {"mutation": "dragSelection3d", "targets": ["part-ghost", "volume-ghost"], "offset": [1.0, 0.0, 0.0]}, "Every target is absent: Error-level `mutation.target-missing`, nothing moves."),
    ("🚚️drag-selection3d", "⏸️keeps-a-zero-offset", {"mutation": "dragSelection3d", "targets": ["part-a"], "offset": [0.0, 0.0, 0.0]}, "A zero offset is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🔄️rotate-selection3d", "🔄️turns", {"mutation": "rotateSelection3d", "targets": ["part-a", "part-b", "volume-1"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, "A quarter turn about +z: `part-a` turns from the identity, `part-b`, which carries no orientation, gains one, and `volume-1` turns about its own origin."),
    ("🔄️rotate-selection3d", "⚠️skips-locked-ghost", {"mutation": "rotateSelection3d", "targets": ["part-b", "volume-2", "part-ghost"], "axis": [1.0, 0.0, 0.0], "angle": math.pi}, "A half turn of `part-b` about +x; the absent `part-ghost` and the locked `volume-2` are skipped as `mutation.partial`."),
    ("🔄️rotate-selection3d", "🚫️rejects-ghosts", {"mutation": "rotateSelection3d", "targets": ["part-ghost"], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, "The only target is absent: Error-level `mutation.target-missing`, nothing turns."),
    ("🔄️rotate-selection3d", "⏸️keeps-a-zero-angle", {"mutation": "rotateSelection3d", "targets": ["part-a"], "axis": [0.0, 0.0, 1.0], "angle": 0.0}, "A zero angle is a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
    ("🔍️scale-selection3d", "🔍️scales", {"mutation": "scaleSelection3d", "targets": ["part-a", "part-b", "volume-1"], "factors": [2.0, 1.0, 0.5]}, "Factors (2, 1, 0.5): `part-a`'s uniform scale becomes a per-axis triple, `part-b`, which carries no scale, reads as one, and `volume-1` scales its triple."),
    ("🔍️scale-selection3d", "⚠️skips-locked-ghost", {"mutation": "scaleSelection3d", "targets": ["part-b", "part-c", "part-ghost"], "factors": [3.0, 3.0, 3.0]}, "`part-b` triples; the absent `part-ghost` and the locked `part-c` are skipped with one `mutation.partial` per reason."),
    ("🔍️scale-selection3d", "🚫️rejects-ghosts", {"mutation": "scaleSelection3d", "targets": ["part-ghost"], "factors": [2.0, 2.0, 2.0]}, "The only target is absent: Error-level `mutation.target-missing`, nothing scales."),
    ("🔍️scale-selection3d", "⏸️keeps-unit-factors", {"mutation": "scaleSelection3d", "targets": ["part-a"], "factors": [1.0, 1.0, 1.0]}, "Unit factors are a Warning-level `mutation.no-op`: the default diff, nothing to undo."),
]

INVARIANTS_5D = [
    ("✋️drag-selection2d", "🧱️no-targets", {"mutation": "dragSelection2d", "targets": [], "dx": 1.0, "dy": 0.0}, [], "An empty target set is what the schema's `minItems: 1` forbids: a Fatal `mutation.invariant`, nothing moves."),
    ("🚚️drag-selection3d", "🧱️repeated-targets", {"mutation": "dragSelection3d", "targets": ["part-a", "part-a"], "offset": [1.0, 0.0, 0.0]}, ["part-a", "part-a"], "A target named twice is what the schema's `uniqueItems` forbids: a Fatal `mutation.invariant`, nothing moves."),
    ("🔄️rotate-selection3d", "🧱️no-targets", {"mutation": "rotateSelection3d", "targets": [], "axis": [0.0, 0.0, 1.0], "angle": QUARTER}, [], "An empty target set is what the schema's `minItems: 1` forbids: a Fatal `mutation.invariant`, nothing turns."),
    ("🔍️scale-selection3d", "🧱️zero-factor", {"mutation": "scaleSelection3d", "targets": ["part-a"], "factors": [1.0, 1.0, 0.0]}, ["part-a"], "A zero factor would flatten the part; the schema's `exclusiveMinimum: 0` forbids it: a Fatal `mutation.invariant`."),
]
# endregion 🔖️Puzzle5d


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        if value is None:
            return
        handle.write(json.dumps(value, indent=2, ensure_ascii=False, sort_keys=True) + "\n")


def kind_of(leaf):
    return next(leaf[at:] for at, character in enumerate(leaf) if character.isascii() and character.isalpha())


def slug_of(case):
    return case.split("️", 1)[1]


def module_of(case):
    return "tests_" + slug_of(case).replace("-", "_")


RUST_HEADER = '''//! 🧪️ `{kind}` fixture — `{case}`.
//!
//! {description}
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/{leaf}/{case}/`
//! (contract D1); the scene is the synthetic selection scene shared by every selection-transform vector.

use crate::standards::v1::subsets::any::schema::diff::{Diff};
use crate::standards::v1::subsets::any::schema::mutations::{Mutation};
use crate::standards::v1::subsets::any::schema::mutations::{{{apply}, {inverse}}};
use crate::{Snapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/🦠️mutation/🔣️.json");
{diff_const}
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{case}/🎯️outcome/🔣️.json");

fn before() -> {Snapshot} {{
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}}
fn expected_after() -> {Snapshot} {{
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}}
fn mutation() -> {Mutation} {{
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}}
fn outcome() -> serde_json::Value {{
    serde_json::from_str(OUTCOME).expect("outcome decodes")
}}

/// 🗣️ `(level, code, target)` of every message `{kind}` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {{
    let produced = <{Mutation} as protocol::Mutation<{Snapshot}>>::diff(&mutation(), &before());
    produced.messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}}

/// 🔣️ Both committed snapshots and the committed `{kind}` payload are already canonical.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: {Snapshot} = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{kind}/{slug}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{kind}/{slug}: committed mutation JSON is not canonical");
}}
'''

DECLARED_MESSAGES = '''
/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {{
    let level = |text: &str| match text {{
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("{kind}/{slug}: unknown message level {{other:?}}"),
    }};
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    outcome().get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {{
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    }})
}}

/// 🔺️ The sparse delta `{kind}` produces is exactly the committed diff: WHICH records it patches, and
/// every patched record whole.
#[test]
fn produces_committed_diff() {{
    let outcome = <{Mutation} as protocol::Mutation<{Snapshot}>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{kind}/{slug}: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!({untouched}, "{kind}/{slug}: a selection transform touches {untouched_text}");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: {Diff} = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = <{Diff} as protocol::MutationDiff<{Snapshot}>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{kind}/{slug}: committed diff did not carry before to after");
}}
'''

APPLIED = '''
/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    {apply}(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, before(), "{kind}/{slug}: an applied vector must move the scene");
}}

/// ↩️ Applying the payload then the inverse it derives from `before` restores `before` EXACTLY — the
/// inverse is absolute setters read off the base, never a negated parameter.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = {inverse}(&base, &mutation);
    assert!(!inverse.is_empty(), "{kind}/{slug}: a moving vector must have something to undo");
    let mut snapshot = base.clone();
    {apply}(&mut snapshot, &mutation).expect("forward applies");
    for step in inverse.iter().rev() {{
        {apply}(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{slug}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome — `applied`, with exactly the declared warnings — is what `{kind}` emits.
#[test]
fn declared_outcome_holds() {{
    assert_eq!(outcome()["status"].as_str(), Some("applied"), "{kind}/{slug} declares an applied outcome");
    assert_eq!(produced_messages(), declared_messages(), "{kind}/{slug}: the produced messages differ from the declared ones");
}}

/// 🧾️ The committed diff is itself canonical and decodes to `{Diff}`.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: {Diff} = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{kind}/{slug}: committed diff JSON is not canonical");
}}
'''

NO_OP = '''
/// ⏸️ A no-op still applies cleanly and leaves the scene byte-identical.
#[test]
fn applies_to_committed_after() {{
    let mut snapshot = before();
    {apply}(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a no-op vector's two committed snapshots must be identical");
}}

/// 🎯️ The declared no-op is exactly what `{kind}` emits: a Warning-level `mutation.no-op` and the default diff.
#[test]
fn declared_outcome_holds() {{
    assert_eq!(outcome()["status"].as_str(), Some("no-op"), "{kind}/{slug} declares a no-op outcome");
    assert_eq!(produced_messages(), declared_messages(), "{kind}/{slug}: the produced messages differ from the declared ones");
    let produced = <{Mutation} as protocol::Mutation<{Snapshot}>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &{Diff}::default(), "{kind}/{slug}: a no-op answers the default diff");
}}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_is_empty() {{
    assert!({inverse}(&before(), &mutation()).is_empty(), "{kind}/{slug}: a no-op must yield no inverse step");
}}
'''

INVARIANT = '''
/// ▶️ A refused `{kind}` still applies cleanly — its diff is the default one — and leaves the scene at
/// the committed `after`, which is the committed `before`.
#[test]
fn refusal_leaves_the_document_at_the_committed_after() {{
    let mut snapshot = before();
    {apply}(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a rejected vector's two committed snapshots must be identical");
}}

/// 🧱️ The payload breaks a hard bound of its own schema, so the refusal is exactly one Fatal
/// `mutation.invariant` addressing the declared path, with the default diff.
#[test]
fn the_invariant_is_the_declared_refusal() {{
    assert!(DIFF_ABSENT.is_empty(), "{kind}/{slug}: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <{Mutation} as protocol::Mutation<{Snapshot}>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &{Diff}::default(), "{kind}/{slug}: a Fatal outcome carries the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "{kind}/{slug} declares a rejected outcome");
    assert_eq!(outcome["code"].as_str(), Some("mutation.invariant"), "{kind}/{slug} declares the invariant refusal");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant".to_string(), path)], "{kind}/{slug}: the refusal differs from the declared one");
}}

/// 🌐️ The refusal does not depend on the scene: the empty scene refuses the same payload the same way.
#[test]
fn the_invariant_is_independent_of_the_base() {{
    let produced = <{Mutation} as protocol::Mutation<{Snapshot}>>::diff(&mutation(), &{Snapshot}::default());
    assert_eq!(produced.messages().iter().map(|message| (message.level, message.code.0.as_str())).collect::<Vec<_>>(), vec![(semio_framework_diagnostic::Severity::Fatal, "mutation.invariant")], "{kind}/{slug}: an invariant is a property of the payload alone");
}}
'''

REJECTED = '''
/// ▶️ A refused `{kind}` still applies cleanly — its diff is the default one — and leaves the scene at
/// the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {{
    let mut snapshot = before();
    {apply}(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{kind}/{slug}: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "{kind}/{slug}: a rejected vector's two committed snapshots must be identical");
}}

/// 🚨️ The refusal is exactly the declared one: default diff, one Error-level message, its code and
/// every target the payload named.
#[test]
fn the_refusal_is_the_declared_one() {{
    assert!(DIFF_ABSENT.is_empty(), "{kind}/{slug}: the D6 sentinel 🔺️diff/🚫️.absent must stay empty");
    let produced = <{Mutation} as protocol::Mutation<{Snapshot}>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &{Diff}::default(), "{kind}/{slug}: a refusing diff builder answers the default diff");
    let outcome = outcome();
    assert_eq!(outcome["status"].as_str(), Some("rejected"), "{kind}/{slug} declares a rejected outcome");
    let path: Vec<String> = outcome["path"].as_array().expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(produced_messages(), vec![(semio_framework_diagnostic::Severity::Error, outcome["code"].as_str().expect("a code").to_string(), path)], "{kind}/{slug}: the refusal differs from the declared one");
}}

/// ↩️ Nothing moved, so nothing is undone.
#[test]
fn inverse_of_a_refusal_is_empty() {{
    assert!({inverse}(&before(), &mutation()).is_empty(), "{kind}/{slug}: a refusal must yield no inverse step");
}}
'''

NAMES = {
    "3d": {"Diff": "Puzzle3dDiff", "Mutation": "Puzzle3dMutation", "Snapshot": "Puzzle3dSnapshot", "apply": "apply_puzzle3d_mutation", "inverse": "inverse_puzzle3d_mutation", "untouched": 'committed["meta"].is_null() && committed["references"].is_null()', "untouched_text": "no document meta and no reference"},
    "5d": {"Diff": "Puzzle5dDiff", "Mutation": "Puzzle5dMutation", "Snapshot": "Puzzle5dSnapshot", "apply": "apply_puzzle5d_mutation", "inverse": "inverse_puzzle5d_mutation", "untouched": 'committed["fasteners"].is_null() && committed["meta"].is_null()', "untouched_text": "no relation and no document meta"},
}


def rust_test(artifact, leaf, case, description, status, invariant=False):
    fields = dict(NAMES[artifact], kind=kind_of(leaf), slug=slug_of(case), leaf=leaf, case=case, description=description)
    if invariant or status == "rejected":
        fields["diff_const"] = 'const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/%s/%s/🔺️diff/🚫️.absent");' % (leaf, case)
        text = RUST_HEADER.format(**fields) + (INVARIANT if invariant else REJECTED).format(**fields)
        if invariant:
            text = text.replace("mutations::{%s, %s};" % (fields["apply"], fields["inverse"]), "mutations::%s;" % fields["apply"])
        return text
    fields["diff_const"] = 'const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/%s/%s/🔺️diff/🔣️.json");' % (leaf, case)
    return RUST_HEADER.format(**fields) + DECLARED_MESSAGES.format(**fields) + (APPLIED if status == "applied" else NO_OP).format(**fields)


def write_case(artifact, scene, outcome_fn, leaf, case, payload, description):
    subset = SUBSET[artifact]
    directory = os.path.join(subset, "🧫️fixtures", "🧬️mutations", leaf, case)
    result, outcome = outcome_fn(copy.deepcopy(scene), payload)
    dump(os.path.join(directory, "📸️snapshot", "⬅️before", "🔣️.json"), scene)
    dump(os.path.join(directory, "🦠️mutation", "🔣️.json"), payload)
    dump(os.path.join(directory, "🎯️outcome", "🔣️.json"), outcome)
    if result is None:
        dump(os.path.join(directory, "📸️snapshot", "➡️after", "🔣️.json"), scene)
        dump(os.path.join(directory, "🔺️diff", "🚫️.absent"), None)
    else:
        after, diff = result
        dump(os.path.join(directory, "📸️snapshot", "➡️after", "🔣️.json"), after)
        dump(os.path.join(directory, "🔺️diff", "🔣️.json"), diff)
    write_test(artifact, leaf, case, rust_test(artifact, leaf, case, description, outcome["status"]))
    return outcome["status"]


def write_invariant(artifact, scene, leaf, case, payload, path, description):
    directory = os.path.join(SUBSET[artifact], "🧫️fixtures", "🧬️mutations", leaf, case)
    for side in ("⬅️before", "➡️after"):
        dump(os.path.join(directory, "📸️snapshot", side, "🔣️.json"), scene)
    dump(os.path.join(directory, "🦠️mutation", "🔣️.json"), payload)
    dump(os.path.join(directory, "🎯️outcome", "🔣️.json"), {"status": "rejected", "code": "mutation.invariant", "path": path})
    dump(os.path.join(directory, "🔺️diff", "🚫️.absent"), None)
    write_test(artifact, leaf, case, rust_test(artifact, leaf, case, description, "rejected", invariant=True))


def write_test(artifact, leaf, case, text):
    path = os.path.join(SUBSET[artifact], "🧬️schema", "🧬️mutations", leaf, "🧪️tests", case, "🦀️.rs")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def mount(artifact, cases, invariants):
    """🌳️ The crate mount-tree rows of every selection leaf's test modules, printed for the crate root."""
    by_leaf = {}
    for leaf, case, *_ in list(cases) + list(invariants):
        by_leaf.setdefault(leaf, []).append(case)
    for leaf, leaf_cases in by_leaf.items():
        print("---", leaf)
        for case in leaf_cases:
            print('#[cfg(test)]\n#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/%s/🧪️tests/%s/🦀️.rs"]\nmod %s;' % (leaf, case, module_of(case)))


def main():
    artifact = sys.argv[1]
    if artifact == "3d":
        for leaf, case, payload, description in CASES_3D:
            print(leaf, case, write_case("3d", SCENE_3D, outcome_3d, leaf, case, payload, description))
        for leaf, case, payload, description in CHAIN_CASES_3D:
            print(leaf, case, write_case("3d", chain_scene_3d(), outcome_3d, leaf, case, payload, description))
        for leaf, case, payload, path, description in INVARIANTS_3D:
            write_invariant("3d", SCENE_3D, leaf, case, payload, path, description)
            print(leaf, case, "invariant")
        mount("3d", CASES_3D + CHAIN_CASES_3D, INVARIANTS_3D)
    else:
        scene = scene_5d()
        for leaf, case, payload, description in CASES_5D:
            print(leaf, case, write_case("5d", scene, outcome_5d, leaf, case, payload, description))
        for leaf, case, payload, path, description in INVARIANTS_5D:
            write_invariant("5d", scene, leaf, case, payload, path, description)
            print(leaf, case, "invariant")
        mount("5d", CASES_5D, INVARIANTS_5D)


if __name__ == "__main__":
    main()
