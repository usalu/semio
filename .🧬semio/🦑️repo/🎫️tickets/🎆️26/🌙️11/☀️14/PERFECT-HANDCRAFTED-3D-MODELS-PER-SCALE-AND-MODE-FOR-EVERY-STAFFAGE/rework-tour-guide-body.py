"""🧥 tour-guide: trunk, seat and legs re-modelled by hand at every scale (softshell jacket with chest, shoulder blades, lumbar hollow and seat; chinos with thigh and calf form), the nested thermal layers under them and the jacket details re-seated on the new surface.

Subject: a man of 1.75 m and 78.6 kg. Every number is metres in the world of the rest pose (x ahead, y up from the hip joint, z left) or along a limb bone from its joint.
Usage: python3 rework-tour-guide-body.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("rework_01_lib", os.path.join(T, "rework-01-lib.py"))
lib = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(lib)

SOURCE = os.path.join(T, "volumes", "humans", "tour-guide.json")
CATALOG = lib.catalog_path("👤️humans", "tour-guide")

JACKET = [
    (-0.090, 0.1145, -0.128, 0.186, 2.35), (-0.060, 0.115, -0.135, 0.188, 2.35), (-0.025, 0.114, -0.138, 0.188, 2.35), (0.020, 0.112, -0.130, 0.184, 2.3),
    (0.070, 0.112, -0.114, 0.176, 2.3), (0.120, 0.114, -0.100, 0.168, 2.25), (0.175, 0.116, -0.093, 0.164, 2.25), (0.230, 0.121, -0.098, 0.166, 2.25),
    (0.290, 0.128, -0.112, 0.171, 2.3), (0.340, 0.134, -0.126, 0.176, 2.35), (0.397, 0.134, -0.135, 0.182, 2.4), (0.440, 0.127, -0.136, 0.190, 2.45),
    (0.470, 0.117, -0.130, 0.196, 2.5), (0.497, 0.104, -0.118, 0.200, 2.5), (0.513, 0.093, -0.108, 0.197, 2.45), (0.527, 0.084, -0.097, 0.190, 2.4),
    (0.538, 0.075, -0.088, 0.180, 2.4), (0.547, 0.068, -0.081, 0.165, 2.4), (0.556, 0.063, -0.076, 0.141, 2.3), (0.564, 0.060, -0.072, 0.115, 2.2), (0.574, 0.056, -0.068, 0.080, 2.0),
]
SEAT = [(0.030, 0.100, -0.116, 0.168, 2.3), (-0.030, 0.102, -0.128, 0.176, 2.3), (-0.075, 0.094, -0.122, 0.172, 2.3), (-0.105, 0.070, -0.098, 0.135, 2.2), (-0.130, 0.035, -0.050, 0.060, 2.0)]
THIGH = [(-0.020, 0.088, 0.100, 0.080, 0.092), (0.040, 0.096, 0.104, 0.084, 0.094), (0.100, 0.098, 0.098, 0.083, 0.092), (0.170, 0.094, 0.088, 0.078, 0.086), (0.240, 0.086, 0.079, 0.071, 0.079), (0.310, 0.077, 0.071, 0.066, 0.072), (0.370, 0.071, 0.066, 0.063, 0.067), (0.411, 0.069, 0.065, 0.062, 0.065)]
SHANK = [(-0.010, 0.068, 0.066, 0.062, 0.065), (0.050, 0.062, 0.070, 0.060, 0.063), (0.120, 0.056, 0.078, 0.060, 0.062), (0.180, 0.054, 0.076, 0.058, 0.060), (0.260, 0.054, 0.068, 0.056, 0.058), (0.340, 0.056, 0.064, 0.056, 0.058), (0.400, 0.060, 0.064, 0.057, 0.059), (0.418, 0.060, 0.064, 0.057, 0.059)]
SKIRT_TOP, JACKET_BOTTOM, TUCK, HEM_OVER = 0.142, 0.107, 0.004, 0.006
PICK = {
    "50": {"jacket": None, "skirt": None, "hem": (-0.092, -0.074, -0.056), "seat": (0, 1, 2, 3, 4), "thigh": None, "shank": None, "n": 2, "steps": 3},
    "100": {"jacket": (0.107, 0.175, 0.290, 0.397, 0.470, 0.527, 0.556, 0.574), "skirt": (-0.088, -0.025, 0.070, 0.142), "hem": (-0.092, -0.056), "seat": (0, 1, 2, 4), "thigh": (0, 1, 3, 5, 7), "shank": (0, 2, 4, 6, 7), "n": 1, "steps": 2},
    "200": {"jacket": (0.107, 0.230, 0.397, 0.497, 0.547, 0.574), "skirt": (-0.088, -0.025, 0.142), "hem": None, "seat": (0, 1, 4), "thigh": (0, 1, 4, 7), "shank": (0, 2, 5, 7), "n": 1, "steps": 2},
    "500": {"jacket": (-0.090, -0.025, 0.120, 0.230, 0.397, 0.497, 0.547, 0.574), "skirt": None, "hem": None, "seat": (0, 1, 4), "thigh": (1, 7), "shank": (0, 2, 7), "n": 1, "steps": 1},
}
RESEAT = ("jacket-zip", "jacket-zip-skirt", "jacket-pocket-l", "jacket-pocket-r", "jacket-chest-pocket", "lanyard-l", "lanyard-r", "badge", "badge-clip", "th-badge")


def at_y(rows, y):
    """📏 The master row at height y (linear between the stated rows, clamped at the ends)."""
    rows = sorted(rows)
    if y <= rows[0][0]:
        return (y,) + tuple(rows[0][1:])
    for a, b in zip(rows, rows[1:]):
        if y <= b[0]:
            t = (y - a[0]) / (b[0] - a[0])
            return (y,) + tuple(a[i] + (b[i] - a[i]) * t for i in range(1, len(a)))
    return (y,) + tuple(rows[-1][1:])


def span(rows, y0, y1):
    """✂️ The stated rows between two heights with the cut rows at both ends."""
    return [at_y(rows, y0)] + [r for r in rows if y0 + 0.004 < r[0] < y1 - 0.004] + [at_y(rows, y1)]


def surface(row, z, sign):
    """🧭 World x of the front (sign +1) or back (sign -1) of a horizontal superellipse row at lateral z."""
    y, front, back, hw, e = row
    t = min(abs(z) / hw, 0.98)
    return (front + back) / 2 + sign * ((front - back) / 2) * (1 - t ** e) ** (1 / e)


def old_rows(frames, solid):
    """🕰️ Rows (y, front, back, half width, exponent) of an existing horizontal loft."""
    out = []
    for s in solid["sections"]:
        x, y, _ = frames.world(solid["bone"], s["at"])
        out.append((y, x + s["ry"], x - s["ry"], s["rx"], s.get("exponent", 2)))
    return out


def stations(rows, d=0.0):
    return [lib.H(r[0], r[1] - d, r[2] + d, r[3] - d, 0.0, r[4]) for r in rows]


def replace(solid, made):
    solid.pop("transform", None)
    solid["sections"] = made["sections"]


def run():
    source = lib.read(SOURCE)
    asset = lib.read(CATALOG)["asset"]
    frames = lib.Frames(asset["rig"])
    kit = lib.Kit(frames)
    for scale, pick in PICK.items():
        volume = source["volumes"][scale]
        body = {s["id"]: s for s in volume["body"]}
        every = volume["body"] + volume["thermal"] + volume["structure"]
        before = {"torso": old_rows(frames, body["jacket"]), "pelvis": old_rows(frames, body.get("jacket-skirt", body["jacket"]))}
        jacket_rows = span(JACKET, JACKET_BOTTOM, 0.574) if pick["jacket"] is None else [at_y(JACKET, y) for y in pick["jacket"]]
        skirt_rows = None
        if "jacket-skirt" in body:
            ys = [r[0] for r in span(JACKET, -0.088, SKIRT_TOP)] if pick["skirt"] is None else pick["skirt"]
            skirt_rows = [at_y(JACKET, y) for y in ys]
            skirt_rows = [(r[0], r[1] - TUCK * (r[0] > 0.1), r[2] + TUCK * (r[0] > 0.1), r[3] - TUCK * (r[0] > 0.1), r[4]) for r in skirt_rows]
        seat_rows = [SEAT[i] for i in pick["seat"]]
        made = {"jacket": (stations, jacket_rows, "torso"), "trousers-seat": (stations, seat_rows, "pelvis")}
        if skirt_rows:
            made["jacket-skirt"] = (stations, skirt_rows, "pelvis")
        if pick["hem"] and "jacket-hem" in body:
            made["jacket-hem"] = (stations, [(lambda r: (r[0], r[1] + HEM_OVER, r[2] - HEM_OVER, r[3] + HEM_OVER, r[4]))(at_y(JACKET, y)) for y in pick["hem"]], "pelvis")
        nests = {"th-jacket-air": "jacket", "th-jacket-shirt": "jacket", "th-jacket-skirt-air": "jacket-skirt", "th-jacket-skirt-shirt": "jacket-skirt", "th-seat-air": "trousers-seat"}
        old_first = {i: body[i]["sections"][0]["rx"] for i in body if "sections" in body[i]}
        for solid in every:
            sid = solid["id"]
            if sid in made and "sections" in solid:
                fn, rows, bone = made[sid]
                replace(solid, kit.loft(sid, bone, "", 0, fn(rows), n=pick["n"]))
            elif sid in nests and "sections" in solid and nests[sid] in made:
                outer = nests[sid]
                d = min(0.02, max(0.001, old_first[outer] - solid["sections"][0]["rx"]))
                fn, rows, bone = made[outer]
                print(f"[DEBUG] {scale} {sid}: {d * 1000:.1f} mm inside {outer}")
                replace(solid, kit.loft(sid, bone, "", 0, fn(rows, d), n=pick["n"]))
        for side, s in ((1, "l"), (-1, "r")):
            for part, rows_all, ends in (("thigh", THIGH, {"end": 0.035}), ("shank", SHANK, {"start": 0.030})):
                rows = rows_all if pick[part] is None else [rows_all[i] for i in pick[part]]
                for solid in every:
                    if solid["id"] == f"{part}-{s}" and "sections" in solid and solid.get("material") != "bone":
                        replace(solid, kit.loft("", f"{part}-{s}", "", 0, kit.limb(f"{part}-{s}", side, rows), n=pick["n"], steps=pick["steps"], **ends))
                    if solid["id"] == f"th-{part}-{s}-air" and "sections" in solid:
                        d = 0.0012
                        inner = [(r[0], r[1] - d, r[2] - d, r[3] - d, r[4] - d) for r in rows]
                        replace(solid, kit.loft("", f"{part}-{s}", "", 0, kit.limb(f"{part}-{s}", side, inner), n=pick["n"], steps=pick["steps"], **ends))
        after = {"torso": JACKET, "pelvis": JACKET}
        for solid in every:
            if solid["id"] not in RESEAT or solid.get("bone") not in before:
                continue
            bone = solid["bone"]

            def moved(q):
                p = frames.world(bone, q)
                old, new = at_y(before[bone], p[1]), at_y(after[bone], p[1])
                sign = 1 if p[0] >= (old[1] + old[2]) / 2 else -1
                p[0] += surface(new, p[2], sign) - surface(old, p[2], sign)
                return [lib.r5(c) for c in frames.local(bone, p)]

            if "sections" in solid:
                for section in solid["sections"]:
                    section["at"] = moved(section["at"])
            elif "transform" in solid and "translation" in solid["transform"]:
                solid["transform"]["translation"] = moved(solid["transform"]["translation"])
    with open(SOURCE, "w", encoding="utf8") as handle:
        json.dump(source, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


run()
