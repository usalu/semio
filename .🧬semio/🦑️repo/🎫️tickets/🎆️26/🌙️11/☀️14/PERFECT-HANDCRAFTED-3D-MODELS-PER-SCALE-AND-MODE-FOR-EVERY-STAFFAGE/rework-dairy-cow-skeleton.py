"""💀 Dairy cow skeleton chisel: skull with zygomatic arches and cheek teeth, mandible halves with ramus, condyle and coronoid process, sternum through the ventral ends of the eight sternal ribs, glenoid ends of the scapulae.

Usage: python3 rework-dairy-cow-skeleton.py <asset document> (in place; after the girdle chisel)
Rows are (u, v, half depth, half breadth[, exponent]) in the bone frame; mandible rows are (u, v, w, half height, half thickness) of the far side (w > 0), the near side is the mirror.
Reference (Holstein-Friesian cow): skull 0.50 m, 0.21 m over the orbits, 0.15 m over the poll, 0.09 m over the incisive bone; mandible 0.39 m with a ramus 0.12 m high; sternum 0.55 m of seven sternebrae, 0.04 m broad at the manubrium and 0.09 m at the last sternebra, meeting ribs 1 to 8.
"""
import json
import sys

LIMB = [0.5, 0.5, 0.5, 0.5]
BONE = {"material": "bone", "role": "body", "tone": 0.82, "part": "member"}

SKULL = {
    "50": [(-0.02, 0.0, 0.050, 0.075, 3.0), (0.05, 0.004, 0.068, 0.098, 3.2), (0.13, 0.004, 0.066, 0.104, 3.2), (0.19, 0.0, 0.058, 0.098, 2.8), (0.26, -0.008, 0.050, 0.070, 2.4), (0.34, -0.016, 0.044, 0.052, 2.4), (0.42, -0.022, 0.036, 0.046, 2.4), (0.47, -0.028, 0.026, 0.046, 2.6), (0.505, -0.034, 0.014, 0.044, 2.6)],
    "100": [(-0.02, 0.0, 0.050, 0.075, 3.0), (0.08, 0.004, 0.068, 0.102, 3.2), (0.19, 0.0, 0.058, 0.098, 2.8), (0.28, -0.010, 0.048, 0.064, 2.4), (0.42, -0.022, 0.036, 0.046, 2.4), (0.505, -0.034, 0.016, 0.044, 2.6)],
}
MANDIBLE = [(-0.04, -0.035, 0.064, 0.060, 0.009), (0.02, -0.058, 0.062, 0.062, 0.010), (0.08, -0.058, 0.056, 0.040, 0.011), (0.16, -0.045, 0.044, 0.026, 0.011), (0.25, -0.032, 0.028, 0.019, 0.011), (0.31, -0.024, 0.016, 0.014, 0.012), (0.34, -0.020, 0.010, 0.011, 0.012)]
STERNUM = {
    "50": [(0.835, -0.340, 0.020, 0.018), (0.784, -0.377, 0.024, 0.024), (0.7225, -0.435, 0.024, 0.028), (0.653, -0.483, 0.022, 0.032), (0.587, -0.520, 0.019, 0.036), (0.517, -0.547, 0.017, 0.040), (0.445, -0.568, 0.016, 0.044), (0.372, -0.583, 0.015, 0.046), (0.297, -0.593, 0.013, 0.040), (0.235, -0.596, 0.008, 0.030)],
    "100": [(0.835, -0.340, 0.020, 0.018), (0.784, -0.377, 0.024, 0.024), (0.653, -0.483, 0.022, 0.032), (0.517, -0.547, 0.017, 0.040), (0.372, -0.583, 0.015, 0.046), (0.26, -0.595, 0.010, 0.034)],
    "200": [(0.80, -0.365, 0.024, 0.024), (0.7225, -0.435, 0.024, 0.028), (0.587, -0.520, 0.020, 0.036), (0.445, -0.568, 0.018, 0.044), (0.31, -0.592, 0.017, 0.036)],
}


def loft(name, bone, rows, member=None):
    sections = []
    for row in rows:
        entry = {"at": [row[0], row[1], row[2]], "rx": row[3], "ry": row[4], "rotation": list(LIMB)}
        if len(row) > 5:
            entry["exponent"] = row[5]
        sections.append(entry)
    return {"id": name, "kind": "loft", "bone": bone, **BONE, "sections": sections, "capStart": True, "capEnd": True, "member": member or bone}


def ellipsoid(name, bone, centre, radii):
    return {"id": name, "kind": "ellipsoid", "bone": bone, **BONE, "radii": list(radii), "transform": {"translation": list(centre)}, "member": bone}


def capsule(name, bone, p0, p1, radius):
    return {"id": name, "kind": "capsule", "bone": bone, **BONE, "p0": list(p0), "p1": list(p1), "r0": radius, "r1": radius, "member": bone}


def put(listing, solid, after):
    for index, entry in enumerate(listing):
        if entry["id"] == solid["id"]:
            listing[index] = solid
            return
    index = next(i for i, entry in enumerate(listing) if entry["id"] == after)
    listing.insert(index + 1, solid)


def main():
    path = sys.argv[1]
    document = json.load(open(path, encoding="utf-8"))
    volumes = document["asset"]["volumes"]
    for scale, rows in SKULL.items():
        put(volumes[scale]["structure"], loft("skull", "head", [(u, v, 0, a, b, e) for u, v, a, b, e in rows]), "skull")
    for scale, rows in STERNUM.items():
        put(volumes[scale]["structure"], loft("sternum", "thorax", [(u, v, 0, a, b, 4.0) for u, v, a, b in rows]), "sternum")
    fine = volumes["50"]["structure"]
    for prefix, sign in (("far-", 1), ("near-", -1)):
        put(fine, loft(prefix + "mandible", "jaw", [(u, v, sign * w, a, b, 3.0) for u, v, w, a, b in MANDIBLE]), prefix + "mandible")
        put(fine, ellipsoid(prefix + "condyle", "jaw", (-0.01, 0.0, sign * 0.062), (0.016, 0.012, 0.020)), prefix + "mandible")
        put(fine, capsule(prefix + "coronoid", "jaw", (0.02, -0.01, sign * 0.058), (0.035, 0.048, sign * 0.055), 0.008), prefix + "condyle")
        put(fine, capsule(prefix + "zygomatic", "head", (0.07, -0.045, sign * 0.088), (0.17, -0.035, sign * 0.094), 0.009), prefix + "orbit")
        put(fine, capsule(prefix + "cheek-teeth", "head", (0.22, -0.060, sign * 0.045), (0.33, -0.062, sign * 0.040), 0.012), prefix + "zygomatic")
        side = "f" if sign > 0 else "n"
        blade = next(entry["id"] for entry in fine if entry.get("bone") == "scapula-f" + side and entry.get("part") == "member")
        put(fine, ellipsoid(prefix + "glenoid", "scapula-f" + side, (0.435, 0.0, 0.0), (0.032, 0.028, 0.022)), blade)
    put(fine, ellipsoid("incisors", "jaw", (0.345, -0.018, 0.0), (0.012, 0.008, 0.028)), "near-mandible")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


main()
