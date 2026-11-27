def segments(radius, quality):
    radius = max(radius, 1e-9)
    ratio = min(1.0, quality["chordM"] / radius)
    chord = quality["minSegments"] if ratio >= 1 else math.ceil(math.pi / math.acos(1 - ratio))
    floor = quality["roundSegments"] if radius > quality["featureM"] else 0
    return min(quality["maxSegments"], max(quality["minSegments"], chord, floor))


def nominal(solid, quality):
    kind = solid["kind"]
    copies = solid.get("repeat", {}).get("count", 1)
    if kind == "ellipsoid":
        n = segments(max(solid["radii"]), quality)
        return copies * 2 * n * (max(2, math.ceil(n / 2)) - 1)
    if kind == "capsule":
        n = segments(max(solid["r0"], solid["r1"]) * max(1, solid.get("lateralScale", 1)), quality)
        return copies * 4 * n * max(2, math.ceil(n / 4))
    if kind == "loft":
        n = max(4, segments(max(max(s["rx"], s["ry"]) for s in solid["sections"]), quality))
        return copies * n * (2 * (len(solid["sections"]) - 1) + bool(solid["capStart"]) + bool(solid["capEnd"]))
    if kind == "revolve":
        n = segments(max(p[1] for p in solid["profile"]) * max(1, solid.get("lateralScale", 1)), quality)
        return copies * 2 * max(1, math.ceil(n * min(solid.get("sweepRad", 2 * math.pi), 2 * math.pi) / (2 * math.pi))) * len(solid["profile"])
    if kind == "box":
        radius = min(solid.get("cornerRadius", 0), *(v / 2 for v in solid["size"]))
        return copies * (12 * (2 * max(2, math.ceil(segments(radius, quality) / 4)) + 1) ** 2 if radius > 0 else 12)
    if kind == "extrude":
        return copies * 4 * (len(solid["profile"]) + sum(len(hole) for hole in solid.get("holes", [])))
    return copies * len(solid["triangles"])


def budget(model, rule):
    lists = ("body", "thermal", "structure")
    scales = sorted(model["volumes"], key=int)
    standard = lambda scale: next(cap for limit, cap in sorted((int(k), v) for k, v in rule["maxSegments"].items()) if scale <= limit)
    cap = lambda scale, level: max(rule["roundSegments"], standard(scale) * level // rule["fullLevel"]) if level >= rule["roundSegments"] else min(standard(scale), level)

    def quality(key, level):
        feature = rule["paperFeatureMm"] * int(key) / 1000
        return {"chordM": min(model["largestM"] / rule["largestPerChord"], rule["chordPerFeature"] * feature) * (rule["fullLevel"] / level) ** 2, "featureM": feature, "minSegments": rule["minSegments"], "roundSegments": rule["roundSegments"], "maxSegments": cap(int(key), level)}

    def counted(key, level):
        body = {solid["id"]: solid for solid in model["volumes"][key]["body"]}
        q = quality(key, level)
        return [sum(nominal(solid if "kind" in solid else body[solid["of"]], q) for solid in model["volumes"][key][name]) for name in lists]

    def own(key, level):
        q = quality(key, level)
        return sum(nominal(solid, q) for name in lists for solid in model["volumes"][key][name] if "kind" in solid)

    whole = rule["fullLevel"]
    while whole > rule["minSegments"] and sum(own(key, whole) for key in scales) > rule["modelTriangles"]:
        whole -= 1
    levels = {}
    for index, key in enumerate(scales):
        level = whole
        if index > 0:
            finer = counted(scales[index - 1], levels[scales[index - 1]])
            while level > rule["minSegments"] and any(a > b for a, b in zip(counted(key, level), finer)):
                level -= 1
        levels[key] = level
    return {"model": whole, "levels": levels, "segments": {key: cap(int(key), levels[key]) for key in scales}, "chordM": {key: quality(key, levels[key])["chordM"] for key in scales}, "triangles": {key: own(key, levels[key]) for key in scales}, "fullTriangles": sum(own(key, rule["fullLevel"]) for key in scales)}
