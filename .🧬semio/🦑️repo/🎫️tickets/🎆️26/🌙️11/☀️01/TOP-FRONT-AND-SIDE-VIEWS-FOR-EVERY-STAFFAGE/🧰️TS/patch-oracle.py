import os

p = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️lateral-spread/🐍️.py"
s = open(p, encoding="utf-8", newline="").read().replace("\r\n", "\n")


def sub(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)


sub('np.array([bone["x"], bone["y"], 0.0])', 'np.array([bone["x"] + offset.get("dx", 0.0), bone["y"] + offset.get("dy", 0.0), bone.get("z", 0.0)])')

i = s.index("def positions(case, rig):")
j = s.index("def main():")
NEW = '''def positions(case, rig):
    spread = solve(rig, case["offsets"], case["root"], True)
    return {id_: {"origin": spread[id_][0].tolist(), "tip": spread[id_][2].tolist(), "rotation": spread[id_][1].tolist()} for id_ in spread}


def affected(rig, offsets):
    result = {}
    for bone in rig:
        offset = offsets.get(bone["id"], {})
        own = offset.get("abductionDeg", 0.0) != 0.0 or offset.get("twistDeg", 0.0) != 0.0
        result[bone["id"]] = own or (bool(bone["parent"]) and result[bone["parent"]])
    return result


def carry(rest, now, point):
    local = np.array(rest["rotation"]).T @ (np.array(point) - np.array(rest["origin"]))
    return (np.array(now["origin"]) + np.array(now["rotation"]) @ local).tolist()


def lateral_fraction(points, outward):
    low, high = min(points), max(points)
    return lambda value: 0.0 if outward == 0 else 1.0 if high == low else (value - low) / (high - low) if outward > 0 else (high - value) / (high - low)


def moved(view, shape, bone, rest, now, is_affected):
    corners = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"]), (shape["x"], shape["y"] + shape["h"])]
    front = view == "front"
    stretches = shape.get("followTip") is True
    span = (rest["tip"][1] - rest["origin"][1]) if front else (rest["tip"][0] - rest["origin"][0])
    by_axis = abs(span) >= 0.5 * bone["lengthM"]
    if is_affected and (not stretches or by_axis):
        if front:
            carried = [carry(rest, now, [rest["origin"][0], y, x]) for x, y in corners]
            return MultiPoint([(q[2], q[1]) for q in carried]).convex_hull
        carried = [carry(rest, now, [x, rest["origin"][1], y]) for x, y in corners]
        return MultiPoint([(q[0], q[2]) for q in carried]).convex_hull
    dx0, dy0 = now["origin"][0] - rest["origin"][0], now["origin"][1] - rest["origin"][1]
    dxt, dyt = now["tip"][0] - rest["tip"][0], now["tip"][1] - rest["tip"][1]
    z0, z1 = now["origin"][2] - rest["origin"][2], now["tip"][2] - rest["tip"][2]
    outward = np.sign(z1 - z0)
    across = lateral_fraction([c[0] if front else c[1] for c in corners], outward)
    result = []
    for x, y in corners:
        if not stretches:
            u = 0.0
        elif by_axis:
            u = min(1.0, max(0.0, ((y - rest["origin"][1]) if front else (x - rest["origin"][0])) / span))
        else:
            u = across(x if front else y)
        z = z0 + (z1 - z0) * u
        if front:
            rest_span = rest["tip"][1] - rest["origin"][1]
            uy = min(1.0, max(0.0, (y - rest["origin"][1]) / rest_span)) if stretches and rest_span != 0 else 0.0
            result.append((x + z, y + dy0 + (dyt - dy0) * uy))
        else:
            result.append((x + dx0 + ((dxt - dx0) * u if stretches and by_axis else 0.0), y + z))
    return MultiPoint(result).convex_hull


'''
s = s[:i] + NEW + s[j:]

old_start = s.index("    rest = solve(rig")
old_end = s.index("        output[name] =")
BLOCK = '''    rest_frames = solve(rig, {}, {"x": 0.0, "y": 0.0, "angleDeg": 0.0}, False)
    rest = {id_: {"origin": f[0].tolist(), "tip": f[2].tolist(), "rotation": f[1].tolist()} for id_, f in rest_frames.items()}
    output = {}
    for name, case in document["cases"].items():
        now = positions(case, rig)
        flags = affected(rig, case["offsets"])
        extents = {}
        for view, shapes in case["shapes"].items():
            for shape in shapes:
                bone = next(b for b in rig if b["id"] == shape["bone"])
                geometry = moved(view, shape, bone, rest[bone["id"]], now[bone["id"]], flags[bone["id"]])
                minx, miny, maxx, maxy = geometry.bounds
                extents[f"{view}/{shape['id']}"] = {"minX": minx, "maxX": maxx, "minY": miny, "maxY": maxy}
'''
s = s[:old_start] + BLOCK + s[old_end:]
s = s.replace('"bones": now, "extents"', '"bones": {k: {"origin": v["origin"], "tip": v["tip"]} for k, v in now.items()}, "extents"')
open(p + ".tmp", "w", encoding="utf-8", newline="").write(s)
os.replace(p + ".tmp", p)
