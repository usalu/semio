"""🐐 Chisel of the goat author: the trunk of `animals/goat` re-cut by hand at every scale.

Tables are world stations of the rest pose (x ahead of the hip joint, y above it, metres):
(x, top line, under line, half breadth, exponent). Each scale reads the same animal at its own stations.
"""
import json
import math
import sys

SOURCE = "volumes/animals/goat.json"
BONES = {"pelvis": (0.0, 0.0, 33.6901), "lumbar": (0.108, 0.072, 0.0), "thorax": (0.375, 0.072, -9.9476)}

RUMP = [
    (-0.112, 0.036, -0.020, 0.030, 2.0),
    (-0.100, 0.054, -0.060, 0.052, 2.0),
    (-0.075, 0.070, -0.100, 0.070, 2.1),
    (-0.040, 0.086, -0.128, 0.084, 2.1),
    (0.000, 0.097, -0.152, 0.096, 2.1),
    (0.045, 0.106, -0.178, 0.110, 2.1),
    (0.090, 0.111, -0.200, 0.124, 2.1),
    (0.135, 0.111, -0.220, 0.136, 2.1),
]
LOIN = [
    (0.090, 0.111, -0.200, 0.124, 2.1),
    (0.135, 0.111, -0.220, 0.136, 2.1),
    (0.185, 0.110, -0.234, 0.143, 2.1),
    (0.245, 0.108, -0.246, 0.148, 2.1),
    (0.305, 0.107, -0.252, 0.150, 2.1),
    (0.370, 0.105, -0.252, 0.149, 2.15),
    (0.425, 0.104, -0.252, 0.146, 2.15),
]
CHEST = [
    (0.370, 0.105, -0.252, 0.149, 2.15),
    (0.425, 0.104, -0.252, 0.146, 2.15),
    (0.470, 0.103, -0.252, 0.141, 2.15),
    (0.525, 0.105, -0.250, 0.133, 2.1),
    (0.575, 0.112, -0.246, 0.124, 2.05),
    (0.625, 0.121, -0.242, 0.113, 2.0),
    (0.670, 0.119, -0.236, 0.102, 2.0),
    (0.705, 0.106, -0.228, 0.093, 2.0),
    (0.732, 0.090, -0.214, 0.083, 2.0),
    (0.752, 0.070, -0.196, 0.071, 2.0),
    (0.767, 0.046, -0.172, 0.057, 2.0),
    (0.777, 0.020, -0.142, 0.040, 2.0),
    (0.782, -0.004, -0.108, 0.020, 2.0),
]
BELLY = [
    (0.120, -0.020, -0.196, 0.090, 2.0),
    (0.160, 0.030, -0.228, 0.136, 2.05),
    (0.210, 0.052, -0.248, 0.151, 2.1),
    (0.265, 0.060, -0.261, 0.156, 2.1),
    (0.335, 0.062, -0.266, 0.158, 2.1),
    (0.405, 0.058, -0.264, 0.156, 2.1),
    (0.470, 0.045, -0.259, 0.148, 2.1),
    (0.525, 0.010, -0.253, 0.132, 2.05),
    (0.565, -0.060, -0.244, 0.095, 2.0),
]
PICK = {
    "50": {"rump": range(8), "loin": range(7), "chest": range(13), "belly": range(9)},
    "100": {"rump": [0, 1, 2, 4, 5, 7], "loin": [0, 2, 3, 4, 6], "chest": [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], "belly": [0, 1, 2, 4, 5, 6, 7, 8]},
    "200": {"rump": [0, 1, 2, 4, 6, 7], "loin": [0, 2, 4, 6], "chest": [0, 2, 4, 5, 6, 8, 9, 11, 12], "belly": [1, 2, 4, 6, 7]},
    "500": {"rump": [0, 2, 4, 5, 7], "loin": [0, 3, 6], "chest": [0, 3, 5, 6, 8, 10, 11], "belly": [1, 3, 5, 7]},
}
PART = {"rump": ("pelvic-barrel", "pelvis", RUMP), "loin": ("abdominal-barrel", "lumbar", LOIN), "chest": ("costal-barrel", "thorax", CHEST), "belly": ("belly", "lumbar", BELLY)}


def sections(bone, rows):
    jx, jy, angle = BONES[bone]
    a = math.radians(angle)
    s, c, h = math.sin(a / 2), math.cos(a / 2), math.sqrt(0.5)
    rotation = [round(s * h, 8), round(c * h, 8), round(-s * h, 8), round(c * h, 8)]
    out = []
    for x, top, bottom, half, exponent in rows:
        dx, dy = x - jx, (top + bottom) / 2 - jy
        u = dx * math.cos(a) + dy * math.sin(a)
        v = -dx * math.sin(a) + dy * math.cos(a)
        out.append({"at": [round(u, 5), round(v, 5), 0], "rx": half, "ry": round((top - bottom) / 2, 5), "exponent": exponent, "rotation": rotation})
    return out


def find(listing, ident):
    for index, entry in enumerate(listing):
        if entry.get("id") == ident:
            return index
    return -1


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale, picks in PICK.items():
        volume = document["volumes"][scale]
        body, thermal = volume["body"], volume["thermal"]
        for part, (ident, bone, table) in PART.items():
            rows = [table[i] for i in picks[part]]
            index = find(body, ident)
            if index < 0:
                loin = body[find(body, "abdominal-barrel")]
                entry = {k: loin[k] for k in loin if k != "sections"}
                entry["id"] = ident
                body.insert(find(body, "abdominal-barrel") + 1, entry)
                index = find(body, ident)
                coat = thermal[find(thermal, "th-abdominal-barrel")]
                twin = dict(coat)
                twin["id"], twin["of"] = "th-" + ident, ident
                thermal.insert(find(thermal, "th-abdominal-barrel") + 1, twin)
            entry = body[index]
            entry["bone"] = bone
            entry["sections"] = sections(bone, rows)
            entry["capStart"] = True
            entry["capEnd"] = True
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] trunk written", file=sys.stderr)


main()
