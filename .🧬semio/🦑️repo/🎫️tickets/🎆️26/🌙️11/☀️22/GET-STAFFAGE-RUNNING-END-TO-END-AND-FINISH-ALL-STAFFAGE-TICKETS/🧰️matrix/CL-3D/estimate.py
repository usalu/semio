"""🧮️ A light estimate of the triangles of every scene of every catalog asset, read from the asset files alone (no engine run): the segment rule of `qualityOf` and `circleSegments` and the grid sizes of the kernel tessellators (exact for ellipsoid, capsule, loft, box and polyhedron; a revolve and an extrusion are approximated by their profile length). Prints the assets whose estimate breaks a triangle budget; the engine tool `budgets.ts` gives the real numbers. estimate.py [ids…]"""
import glob, json, math, os, sys

CATALOG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")


def segments(radius, quality):
    radius = max(radius, 1e-9)
    ratio = min(1, quality["chord"] / radius)
    n = quality["min"] if ratio >= 1 else math.ceil(math.pi / math.acos(1 - ratio))
    if radius > quality["feature"]:
        n = max(n, 24)
    return max(quality["min"], min(quality["max"], n))


def quality_of(largest, scale):
    feature = 0.00015 * scale
    return {"chord": min(largest / 3200, 0.02 * feature), "min": 12, "max": 96 if scale <= 20 else 64 if scale <= 50 else 48 if scale <= 100 else 40 if scale <= 200 else 32, "feature": feature}


def triangles(solid, quality):
    kind = solid["kind"]
    copies = solid.get("repeat", {}).get("count", 1)
    if kind == "ellipsoid":
        n = segments(max(solid["radii"]), quality)
        return copies * (2 * n * max(2, math.ceil(n / 2)) - 2 * n)
    if kind == "capsule":
        n = segments(max(solid["r0"], solid["r1"]) * max(1, solid.get("lateralScale", 1)), quality)
        return copies * (2 * n * (2 * max(2, math.ceil(n / 4)) + 1) - 2 * n)
    if kind == "loft":
        n = max(4, segments(max(max(s["rx"], s["ry"]) for s in solid["sections"]), quality))
        return copies * (2 * n * (len(solid["sections"]) - 1) + n * (bool(solid.get("capStart")) + bool(solid.get("capEnd"))))
    if kind == "revolve":
        n = segments(max(p[1] for p in solid["profile"]) * max(1, solid.get("lateralScale", 1)), quality)
        return copies * 2 * n * len(solid["profile"])
    if kind == "extrude":
        return copies * 4 * len(solid["profile"])
    if kind == "polyhedron":
        return copies * len(solid["triangles"])
    return copies * 12


rows = []
for path in sorted(glob.glob(os.path.join(CATALOG, "*", "*", "🔣️.json"))):
    name = os.path.basename(os.path.dirname(path))
    if len(sys.argv) > 1 and name not in sys.argv[1:]:
        continue
    asset = json.load(open(path, encoding="utf-8"))["asset"]
    largest = max(asset["dimensions"][k] for k in ("heightM", "widthM", "depthM"))
    counts = {}
    for key, volume in asset["volumes"].items():
        if volume.get("status") != "authored":
            continue
        quality = quality_of(largest, int(key))
        body = {s["id"]: s for s in volume["body"]}
        for mode in ("body", "thermal", "structure"):
            counts[(int(key), mode)] = sum(triangles(s if "kind" in s else body[s["of"]], quality) for s in volume[mode])
    findings = []
    for mode in ("body", "thermal", "structure"):
        row = sorted((scale, n) for (scale, m), n in counts.items() if m == mode)
        findings += [f"{mode} grows 1:{a[0]} {a[1]} -> 1:{b[0]} {b[1]}" for a, b in zip(row, row[1:]) if b[1] > a[1]]
        if row and row[0][1] > 400_000:
            findings.append(f"{row[0][1]} {mode} at 1:{row[0][0]}")
    total = sum(counts.values())
    rows.append((asset["category"], name, counts.get((50, "body"), 0), counts.get((50, "thermal"), 0), total, findings))
for category, name, body, thermal, total, findings in rows:
    if findings or total > 600_000 or len(sys.argv) > 1:
        print(f"{category:12} {name:44} 1:50 body {body:7} thermal+ {thermal:7} all scenes {total:8}  {'; '.join(findings)}")
print(len(rows), "assets")
