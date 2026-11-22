"""🦴 Skeleton of the domestic cat: ribs close onto spine and sternum, the hip bones meet the sacrum and each other, the ear cartilage sits on the skull."""
import json, math

SOURCE = "volumes/animals/domestic-cat.json"
THORAX_A_LENGTH, THORAX_B_ANGLE = 0.0677, math.radians(-16.528)
STERNAL = {"50": ["rib-1", "rib-2", "rib-3", "rib-4", "rib-5", "rib-6", "rib-7", "rib-8"], "100": ["rib-1", "rib-2", "rib-3"], "200": ["rib-3", "rib-7"], "500": ["rib-4", "rib-9"]}
ASTERNAL = {"100": {"rib-4": [(0.036, -0.093, 0.0255)]}, "200": {"rib-11": [(0.026, -0.086, 0.0255), (0.052, -0.094, 0.0235)]}}
HEAD_Z = {"50": 0.006, "100": 0.007, "200": 0.006, "500": 0.006}


def rotate(q, v):
    x, y, z, w = q
    tx, ty, tz = 2 * (y * v[2] - z * v[1]), 2 * (z * v[0] - x * v[2]), 2 * (x * v[1] - y * v[0])
    return [v[0] + w * tx + (y * tz - z * ty), v[1] + w * ty + (z * tx - x * tz), v[2] + w * tz + (x * ty - y * tx)]


def multiply(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def turned(q, direction):
    """🧭 Rotation of the previous section turned by the shortest arc so its normal follows the new direction."""
    normal = rotate(q, [0, 0, 1])
    length = math.sqrt(sum(c * c for c in direction))
    d = [c / length for c in direction]
    if sum(a * b for a, b in zip(normal, d)) < 0:
        d = [-c for c in d]
    axis = [normal[1] * d[2] - normal[2] * d[1], normal[2] * d[0] - normal[0] * d[2], normal[0] * d[1] - normal[1] * d[0]]
    arc = axis + [1 + sum(a * b for a, b in zip(normal, d))]
    size = math.sqrt(sum(c * c for c in arc))
    return [round(c, 5) for c in multiply([c / size for c in arc], q)]


def closest(polyline, point):
    best = None
    for (ax, ay), (bx, by) in zip(polyline, polyline[1:]):
        u = max(0, min(1, ((point[0] - ax) * (bx - ax) + (point[1] - ay) * (by - ay)) / ((bx - ax) ** 2 + (by - ay) ** 2)))
        candidate = (ax + u * (bx - ax), ay + u * (by - ay))
        distance = math.hypot(candidate[0] - point[0], candidate[1] - point[1])
        if best is None or distance < best[0]:
            best = (distance, candidate)
    return best[1]


def extend(rib, target):
    last = rib["sections"][-1]
    at = [round(c, 5) for c in target]
    rib["sections"].append({**last, "at": at, "rotation": turned(last["rotation"], [at[i] - last["at"][i] for i in range(3)])})


def loft(template, identifier, rows):
    return {**template, "id": identifier, "sections": [{"at": list(at), "rx": rx, "ry": ry, "exponent": 2, "rotation": rotation} for at, rx, ry, rotation in rows]}


document = json.load(open(SOURCE))
volumes = document["volumes"]
for scale in ("50", "100", "200", "500"):
    structure = {solid["id"]: solid for solid in volumes[scale]["structure"]}
    sternum_b = [(s["at"][0], s["at"][1]) for s in structure["sternum"]["sections"]]
    cos, sin = math.cos(THORAX_B_ANGLE), math.sin(THORAX_B_ANGLE)
    sternum_a = [(THORAX_A_LENGTH + x * cos - y * sin, x * sin + y * cos) for x, y in sternum_b]
    half = structure["sternum"]["sections"][0]["rx"] * 0.5
    for side in ("near", "far"):
        for identifier, solid in structure.items():
            if not identifier.startswith(side + "-rib-"):
                continue
            head, name = solid["sections"][0], identifier[len(side) + 1:]
            sign = 1 if solid["sections"][-1]["at"][2] > 0 else -1
            head["at"][2] = sign * min(abs(head["at"][2]), HEAD_Z[scale])
            if name in STERNAL[scale]:
                end = solid["sections"][-1]["at"]
                x, y = closest(sternum_b if solid["bone"] == "thorax-b" else sternum_a, end)
                extend(solid, (x, y, sign * half))
            for x, y, z in ASTERNAL.get(scale, {}).get(name, []):
                extend(solid, (x, y, sign * z))
            print(scale, identifier, len(solid["sections"]))

fine = {solid["id"]: solid for solid in volumes["50"]["structure"]}
fine["sacrum"]["sections"][0]["rx"], fine["sacrum"]["sections"][1]["rx"] = 0.019, 0.017
for side, sign in (("near", -1), ("far", 1)):
    for section, z in zip(fine[side + "-ilium"]["sections"], (0.023, 0.022, 0.026, 0.030)):
        section["at"][2] = sign * z
    ear, root = fine[side + "-ear-cartilage"], fine[side + "-ear-cartilage"]["sections"][0]
    ear["sections"].insert(0, {**root, "at": [-0.009, 0.0015, -sign * 0.0035], "rx": 0.009})
pubis = fine["pubis"]
first, last = pubis["sections"][0], pubis["sections"][-1]
pubis["sections"] = [{**first, "at": [0.0003, -0.008, -0.027]}] + pubis["sections"] + [{**last, "at": [0.0003, -0.008, 0.027]}]

medium = volumes["100"]["structure"]
index = next(i for i, solid in enumerate(medium) if solid["id"] == "pelvis")
plate = medium[index]
sacrum_rotation, pubis_rotations = fine["sacrum"]["sections"][0]["rotation"], [s["rotation"] for s in pubis["sections"]]
replacement = [loft(plate, "sacrum", [((0.0493, -0.0007, 0), 0.019, 0.008, sacrum_rotation), ((0.0336, 0.0061, 0), 0.017, 0.008, sacrum_rotation), ((0.0167, 0.0122, 0), 0.009, 0.0075, sacrum_rotation), ((-0.0236, 0.0241, 0), 0.0077, 0.0077, sacrum_rotation)])]
for side, sign in (("near", -1), ("far", 1)):
    ilium, ischium = fine[side + "-ilium"]["sections"], fine[side + "-ischium"]["sections"]
    replacement.append(loft(plate, side + "-ilium", [((0.0572, 0.0072, sign * 0.024), 0.0075, 0.012, ilium[0]["rotation"]), ((0.0389, 0.0059, sign * 0.024), 0.0075, 0.011, ilium[1]["rotation"]), ((0.0177, 0.0024, sign * 0.0265), 0.0075, 0.008, ilium[2]["rotation"]), ((0, -0.0001, sign * 0.029), 0.008, 0.009, ilium[3]["rotation"])]))
    replacement.append(loft(plate, side + "-ischium", [((0, -0.0001, sign * 0.029), 0.008, 0.009, ischium[0]["rotation"]), ((-0.0203, 0.0024, sign * 0.0275), 0.0075, 0.0075, ischium[1]["rotation"]), ((-0.0364, 0.0046, sign * 0.0265), 0.0075, 0.008, ischium[2]["rotation"])]))
replacement.append(loft(plate, "pubis", [((0.0003, -0.008, -0.027), 0.0075, 0.0075, pubis_rotations[0]), ((0.0006, -0.0172, -0.019), 0.0075, 0.0075, pubis_rotations[1]), ((-0.0054, -0.0226, 0), 0.0075, 0.0075, pubis_rotations[2]), ((0.0006, -0.0172, 0.019), 0.0075, 0.0075, pubis_rotations[3]), ((0.0003, -0.008, 0.027), 0.0075, 0.0075, pubis_rotations[4])]))
medium[index:index + 1] = replacement
json.dump(document, open(SOURCE, "w"), ensure_ascii=False, indent=1)
