"""🚶️ The redraw of the walking commuter's moving limbs in plan and front view as the solids their side elevation already draws.

Front and plan limbs were flat outlines of the standing figure that a bone only carries along as a card; a swung limb is then a sliver in the view that looks along it. Every such shape takes the outline of its side-elevation twin (same id, same bone, same scale) as profile and a `solid`:

- round limbs (sleeve, forearm, cuff, wrist, thigh, shank): a body of revolution about the bone, the half profile is the half width of the side outline along the bone, the axis keeps the lateral middle of the old outline;
- flat parts (hand, shoes): the side outline swept laterally over the lateral range of the old outline; at the scales whose 3D model has fingers (1:50, 1:100) the hand is the palm up to the knuckles and one body of revolution per finger and thumb bone, in all three views (the side elevation drew the fingers straight, the model curls them inwards), see `flat_part`;
- the toe box in plan: a capsule from the ball to the tip that follows both ends of the toe bone (the rounded footprint the plan-projection guard measures against its own kinematics).

`redraw(asset, rest, groups)` returns `{(scale, view, id): shape}`; an id the view does not have yet is inserted before the shape it belongs to (`FAR`); `rest` are the rest frames of the bones (`rest-frames.ts`).
"""

ROUND = {"arm-l", "forearm-l", "thigh-l", "shank-l", "thigh-r", "shank-r"}
FLAT = {"hand-l", "foot-l", "toe-l", "foot-r", "toe-r"}
GROUPS = {"arm": {"arm-l", "forearm-l", "hand-l"}, "feet": {"foot-l", "toe-l", "foot-r", "toe-r"}, "legs": {"thigh-l", "shank-l", "thigh-r", "shank-r"}}
SCALES = (50, 100, 200, 500)
FAR = {}
SAMPLES = {50: 28, 100: 18, 200: 10, 500: 6}
SIMPLIFY = {50: 0.0006, 100: 0.0012, 200: 0.0024, 500: 0.005}


def crossings(points, axis, value):
    """✂️ The sorted coordinates at which a closed outline crosses the line `axis = value` (the other coordinate)."""
    other = 1 - axis
    out = []
    for i, a in enumerate(points):
        b = points[(i + 1) % len(points)]
        if (a[axis] - value) * (b[axis] - value) < 0:
            t = (value - a[axis]) / (b[axis] - a[axis])
            out.append(a[other] + t * (b[other] - a[other]))
    return sorted(out)


def span(points, axis, value):
    """↔️ The range of the other coordinate of an outline at `axis = value`, clamped into the outline."""
    low = min(p[axis] for p in points)
    high = max(p[axis] for p in points)
    inset = 0.02 * (high - low)
    cut = crossings(points, axis, min(max(value, low + inset), high - inset))
    return (cut[0], cut[-1])


def simplify(points, tolerance):
    """🪒️ Drops the vertices of an open line that lie within `tolerance` of the chord of their neighbours (Ramer-Douglas-Peucker)."""
    if len(points) < 3:
        return points
    (ax, ay), (bx, by) = points[0], points[-1]
    length = ((bx - ax) ** 2 + (by - ay) ** 2) ** 0.5 or 1e-12
    far, index = 0.0, 0
    for i in range(1, len(points) - 1):
        d = abs((bx - ax) * (points[i][1] - ay) - (by - ay) * (points[i][0] - ax)) / length
        if d > far:
            far, index = d, i
    if far <= tolerance:
        return [points[0], points[-1]]
    return simplify(points[: index + 1], tolerance)[:-1] + simplify(points[index:], tolerance)


def half_profile(side, scale):
    """🌗️ The half profile of a round limb: the half width of its side outline along the bone about the middle line `v0` of the outline."""
    us = [p[0] for p in side]
    low, high = min(us), max(us)
    n = SAMPLES[scale]
    edge = 0.0005
    stations = [low + edge] + [low + (high - low) * k / n for k in range(1, n)] + [high - edge]
    rows = []
    for u in stations:
        cut = crossings(side, 0, u)
        if len(cut) >= 2:
            rows.append((u, (cut[-1] - cut[0]) / 2, (cut[-1] + cut[0]) / 2))
    weight = sum(r for _, r, _ in rows)
    v0 = sum(r * c for _, r, c in rows) / weight
    line = simplify([(low, 0.0)] + [(u, r) for u, r, _ in rows] + [(high, 0.0)], SIMPLIFY[scale])
    return [[round(u, 4), round(v0 + r, 4)] for u, r in line], round(v0, 4)


def lateral_range(old, view):
    """📏️ The lateral range of an old front (x) or plan (y) outline in rest coordinates of the asset."""
    values = [p[0 if view == "front" else 1] for p in old["points"]]
    return min(values), max(values)


def carried(shape, keep):
    """🧳️ The members of the old shape that the redraw keeps (id, bone, role, z, fill, tone, hatch)."""
    return {key: value for key, value in shape.items() if key in keep}


KEEP = ("id", "bone", "role", "z", "filled", "tone", "hatch", "region", "material")


def round_limb(old_plan, side, bone_z, scale):
    """🪵️ A round limb as a body of revolution about its bone."""
    points, v0 = half_profile(side["points"], scale)
    low, high = lateral_range(old_plan, "plan")
    return {**carried(old_plan, KEEP), "kind": "poly", "points": points, "closed": True, "solid": {"kind": "revolved", "axisBone": old_plan["bone"], "axis": "u", "axisX": v0, "axisZ": round((low + high) / 2 - bone_z, 4), "mirror": True}}


def clip(points, axis, low, high):
    """✂️ The part of a closed outline between two values of one coordinate (Sutherland-Hodgman against both lines)."""
    for sign, bound in ((1, low), (-1, high)):
        out = []
        for i, a in enumerate(points):
            b = points[(i + 1) % len(points)]
            ina, inb = sign * (a[axis] - bound) >= 0, sign * (b[axis] - bound) >= 0
            if ina != inb:
                t = (bound - a[axis]) / (b[axis] - a[axis])
                cut = [round(a[0] + t * (b[0] - a[0]), 4), round(a[1] + t * (b[1] - a[1]), 4)]
                out += [cut, b] if inb else [cut]
            elif inb:
                out.append(b)
        points = out
    return points


def prism(keep, points, low, high):
    """🧊️ A side outline swept laterally from `low` to `high` (rest coordinates of the asset)."""
    return {**keep, "kind": "poly", "points": points, "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": round(low, 4), "z1": round(high, 4)}}


def finger_tubes(asset, scale, keep):
    """🖐️ The finger and thumb capsules of the 3D model of a scale as bodies of revolution about their own bones, the same shape for the three views."""
    parents = {b["id"]: b["parent"] for b in asset["rig"]["bones"]}
    out = {}
    for solid in asset["volumes"][str(scale)]["body"]:
        if solid["kind"] != "capsule" or parents.get(solid.get("bone")) is None:
            continue
        chain, bone = [], solid["bone"]
        while bone is not None:
            chain.append(bone)
            bone = parents[bone]
        if keep["bone"] not in chain[1:]:
            continue
        length, r0, r1 = solid["p1"][0], solid["r0"], solid["r1"]
        k = 0.7071
        points = [[-r0, 0], [-k * r0, k * r0], [0, r0], [length, r1], [length + k * r1, k * r1], [length + r1, 0]]
        out[solid["id"]] = {**keep, "id": solid["id"], "bone": solid["bone"], "kind": "poly", "points": [[round(x, 4), round(y, 4)] for x, y in points], "closed": True, "solid": {"kind": "revolved", "axisBone": solid["bone"], "axis": "u", "axisX": 0, "axisZ": 0, "mirror": True}}
    return out


def flat_part(asset, scale, old_plan, old_front, side, frame, length):
    """🧱️ A flat part as its side outline swept laterally over the lateral range of the old outline. Where the 3D model of the scale has fingers of their own (capsules on the finger bones), the hand is the palm up to the knuckles, swept over the lateral range of the old front outline at the wrist, and the fingers and the thumb as the bodies of revolution of {@link finger_tubes}; the second value is the palm outline for the side elevation and the third the finger shapes."""
    reference, view = (old_plan, "plan") if old_plan is not None else (old_front, "front")
    low, high = lateral_range(reference, view)
    keep = carried(reference, KEEP)
    points = side["points"]
    tubes = finger_tubes(asset, scale, keep) if old_plan is not None and old_front is not None else {}
    if not tubes:
        return prism(keep, points, low, high), None, {}
    wrist = span(old_front["points"], 1, frame["o"][1])
    edge = span(points, 0, 0.2 * length)[1]
    palm = clip(clip(points, 0, min(p[0] for p in points) - 1, length + 0.003), 1, min(p[1] for p in points) - 1, edge + 0.005)
    return prism(keep, palm, *wrist), palm, tubes


def toe_capsule(old_plan, frame, length):
    """👟️ The toe box in plan as a capsule from the ball to the tip of the toe bone, following both ends."""
    xs = [p[0] for p in old_plan["points"]]
    ball = span(old_plan["points"], 0, frame["o"][0])
    r0 = round((ball[1] - ball[0]) / 2, 4)
    y = round((ball[0] + ball[1]) / 2, 4)
    front = max(xs)
    near = span(old_plan["points"], 0, front - 0.012)
    r1 = round(max((near[1] - near[0]) / 2, 0.02), 4)
    return {**carried(old_plan, KEEP), "kind": "capsule", "x0": round(frame["o"][0], 4), "y0": y, "x1": round(front - r1, 4), "y1": round((near[0] + near[1]) / 2, 4), "r0": r0, "r1": r1, "followTip": True}


def redraw(asset, rest, groups, toe="capsule"):
    """🖍️ The new plan and front shapes of the chosen groups, keyed by (scale, view, id)."""
    bones = set().union(*(GROUPS[g] for g in groups))
    lengths = {b["id"]: b["lengthM"] for b in asset["rig"]["bones"]}
    out = {}
    for scale in SCALES:
        geometry = asset["representations"][str(scale)]["geometry"]
        side = {s["id"]: s for s in geometry["elevation"]}
        plan = {s["id"]: s for s in geometry["plan"]}
        front = {s["id"]: s for s in geometry["front"]}
        for shape_id in sorted(set(plan) | set(front)):
            old_plan, old_front = plan.get(shape_id), front.get(shape_id)
            old = old_plan or old_front
            bone = old["bone"]
            if bone not in bones or shape_id not in side or side[shape_id]["bone"] != bone:
                continue
            frame = rest[bone]["frame"]
            if bone in ROUND:
                new = round_limb(old_plan or {**old_front, "points": [[0, p[0]] for p in old_front["points"]]}, side[shape_id], frame["o"][2], scale)
                for view, there in (("plan", old_plan), ("front", old_front)):
                    if there is not None:
                        out[(scale, view, shape_id)] = {**new, "z": there["z"]}
            else:
                first, palm, more = flat_part(asset, scale, old_plan, old_front, side[shape_id], frame, lengths[bone])
                views = [("plan", old_plan), ("front", old_front)]
                if palm is not None:
                    out[(scale, "elevation", shape_id)] = {**side[shape_id], "points": palm}
                    views.append(("elevation", side[shape_id]))
                for view, there in views:
                    if there is None:
                        continue
                    if view != "elevation":
                        out[(scale, view, shape_id)] = {**first, "z": there["z"]}
                    for far_id, part in more.items():
                        out[(scale, view, far_id)] = {**part, "z": round(there["z"] - 0.1, 2)}
                        FAR[far_id] = shape_id
                if toe == "capsule" and bone.startswith("toe-") and old_plan is not None:
                    out[(scale, "plan", shape_id)] = toe_capsule(old_plan, frame, lengths[bone])
    return out
