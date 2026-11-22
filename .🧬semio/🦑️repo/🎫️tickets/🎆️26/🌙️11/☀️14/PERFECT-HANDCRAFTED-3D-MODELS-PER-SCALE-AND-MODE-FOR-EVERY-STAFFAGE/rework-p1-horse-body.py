"""🐎 Author pass 1 (second round), animals/horse: hand-chosen section tables of trunk, withers, breast, neck base, shoulder, arm and tail at every scale.

Usage: python3 rework-p1-horse-body.py
Tables are written in the world frame of the rest pose (X ahead of the hip joint, H above the ground, z lateral, near side z < 0) for the trunk masses
and in the bone frame (u along the bone, v across in the sagittal plane, z lateral) for limbs, neck and tail. a = sagittal half extent, b = lateral half extent.
Reference: warmblood, 1.61 m at the withers, croup 1.60 m, girth depth 0.74 m, barrel 0.61 m wide, points of the shoulder 0.43 m apart.
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "animals", "horse.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/horse/🔣️.json"
GROUND = 1.3123
LIMB = [0.5, 0.5, 0.5, 0.5]


def frames():
    with open(CATALOG, encoding="utf-8") as handle:
        bones = {bone["id"]: bone for bone in json.load(handle)["asset"]["rig"]["bones"]}
    done = {}

    def frame(ident):
        if ident in done:
            return done[ident]
        bone = bones[ident]
        px, py, pa = frame(bone["parent"]) if bone["parent"] else (0.0, 0.0, 0.0)
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        done[ident] = (px + c * bone["x"] - s * bone["y"], py + s * bone["x"] + c * bone["y"], pa + bone["angleDeg"])
        return done[ident]

    for ident in bones:
        frame(ident)
    return done


FRAMES = frames()


def upright(bone, rows, exponent):
    """🧱 Rows (X, top, bottom, z, b[, exponent]) with vertical section planes, given in the world frame, as loft sections in the frame of `bone`."""
    jx, jy, angle = FRAMES[bone]
    c, s = math.cos(math.radians(angle)), math.sin(math.radians(angle))
    half = math.radians(-angle) / 2
    ch, sh = math.cos(half), math.sin(half)
    rotation = [round((ch - sh) / 2, 5), round((ch + sh) / 2, 5), round((ch + sh) / 2, 5), round((ch - sh) / 2, 5)]
    out = []
    for row in rows:
        x, top, bottom, z, b = row[:5]
        dx, dy = x - jx, (top + bottom) / 2 - GROUND - jy
        out.append({"at": [round(c * dx + s * dy, 5), round(-s * dx + c * dy, 5), z], "rx": round((top - bottom) / 2, 5), "ry": b, "exponent": row[5] if len(row) > 5 else exponent, "rotation": rotation})
    return out


def along(rows, exponent, mirror=False):
    """🦴 Rows (u, v, z, a, b[, exponent]) in the bone frame with section planes square to the bone."""
    return [{"at": [row[0], row[1], -row[2] if mirror else row[2]], "rx": row[3], "ry": row[4], "exponent": row[5] if len(row) > 5 else exponent, "rotation": list(LIMB)} for row in rows]


def flip(rows):
    return [(row[0], row[1], row[2], -row[3]) + tuple(row[4:]) for row in rows]


TRUNK = {
    "50": [
        (-0.275, 1.44, 1.25, 0, 0.1, 2.6), (-0.262, 1.475, 1.19, 0, 0.15, 2.8), (-0.235, 1.5, 1.14, 0, 0.185, 2.8), (-0.19, 1.528, 1.085, 0, 0.215, 2.7), (-0.12, 1.556, 1.03, 0, 0.244, 2.6),
        (-0.03, 1.582, 1.0, 0, 0.262, 2.6), (0.08, 1.598, 0.995, 0, 0.278, 2.6), (0.19, 1.6, 1.005, 0, 0.283, 2.5), (0.3, 1.588, 1.015, 0, 0.282, 2.4),
        (0.42, 1.566, 1.005, 0, 0.28, 2.3), (0.54, 1.548, 0.975, 0, 0.287, 2.25), (0.66, 1.535, 0.943, 0, 0.297, 2.2), (0.78, 1.525, 0.917, 0, 0.304, 2.2),
        (0.9, 1.519, 0.897, 0, 0.303, 2.2), (1.0, 1.517, 0.884, 0, 0.294, 2.2), (1.09, 1.52, 0.876, 0, 0.279, 2.2), (1.17, 1.522, 0.878, 0, 0.26, 2.2),
        (1.24, 1.512, 0.893, 0, 0.238, 2.2), (1.3, 1.488, 0.92, 0, 0.214, 2.2), (1.35, 1.447, 0.958, 0, 0.19, 2.15), (1.395, 1.39, 1.005, 0, 0.162, 2.1),
        (1.43, 1.325, 1.055, 0, 0.128, 2.05), (1.455, 1.265, 1.1, 0, 0.088, 2.0), (1.468, 1.215, 1.14, 0, 0.04, 2.0),
    ],
    "100": [
        (-0.275, 1.45, 1.23, 0, 0.11, 2.6), (-0.245, 1.495, 1.15, 0, 0.18, 2.8), (-0.19, 1.528, 1.085, 0, 0.215, 2.7), (-0.1, 1.562, 1.02, 0, 0.25, 2.6), (0.05, 1.595, 0.995, 0, 0.274, 2.6),
        (0.2, 1.6, 1.005, 0, 0.283, 2.5), (0.38, 1.573, 1.01, 0, 0.28, 2.3), (0.58, 1.543, 0.965, 0, 0.29, 2.2), (0.8, 1.524, 0.913, 0, 0.304, 2.2),
        (1.0, 1.517, 0.884, 0, 0.294, 2.2), (1.15, 1.522, 0.877, 0, 0.265, 2.2), (1.27, 1.5, 0.905, 0, 0.226, 2.2), (1.36, 1.435, 0.968, 0, 0.184, 2.1),
        (1.425, 1.335, 1.048, 0, 0.132, 2.0), (1.465, 1.23, 1.13, 0, 0.05, 2.0),
    ],
    "200": [
        (-0.275, 1.46, 1.22, 0, 0.12, 2.6), (-0.22, 1.51, 1.12, 0, 0.2, 2.8), (-0.14, 1.548, 1.045, 0, 0.237, 2.6), (-0.02, 1.585, 1.0, 0, 0.264, 2.6), (0.2, 1.6, 1.005, 0, 0.283, 2.5),
        (0.45, 1.56, 1.0, 0, 0.281, 2.3), (0.78, 1.525, 0.917, 0, 0.304, 2.2), (1.05, 1.518, 0.879, 0, 0.287, 2.2), (1.25, 1.508, 0.897, 0, 0.234, 2.2),
        (1.38, 1.41, 0.988, 0, 0.172, 2.1), (1.46, 1.25, 1.11, 0, 0.07, 2.0),
    ],
    "500": [
        (-0.275, 1.47, 1.2, 0, 0.13, 2.6), (-0.2, 1.522, 1.1, 0, 0.21, 2.8), (-0.08, 1.568, 1.01, 0, 0.255, 2.6), (0.2, 1.6, 1.005, 0, 0.283, 2.5), (0.6, 1.54, 0.96, 0, 0.3, 2.2),
        (1.05, 1.518, 0.879, 0, 0.287, 2.2), (1.3, 1.488, 0.92, 0, 0.214, 2.2), (1.45, 1.28, 1.09, 0, 0.09, 2.0),
    ],
}

WITHERS = {
    "50": [
        (0.7, 1.5, 1.38, 0, 0.045), (0.8, 1.53, 1.3, 0, 0.085), (0.9, 1.556, 1.24, 0, 0.11), (1.0, 1.585, 1.2, 0, 0.125), (1.08, 1.606, 1.18, 0, 0.132),
        (1.15, 1.615, 1.17, 0, 0.135), (1.22, 1.608, 1.17, 0, 0.135), (1.29, 1.58, 1.18, 0, 0.13), (1.35, 1.53, 1.2, 0, 0.115), (1.4, 1.46, 1.24, 0, 0.08),
    ],
    "100": [(0.72, 1.505, 1.36, 0, 0.055), (0.86, 1.546, 1.26, 0, 0.1), (1.0, 1.585, 1.2, 0, 0.125), (1.12, 1.613, 1.17, 0, 0.134), (1.23, 1.605, 1.17, 0, 0.135), (1.32, 1.558, 1.19, 0, 0.122), (1.4, 1.46, 1.24, 0, 0.08)],
    "200": [(0.75, 1.51, 1.33, 0, 0.07), (0.95, 1.572, 1.21, 0, 0.12), (1.13, 1.614, 1.17, 0, 0.135), (1.27, 1.59, 1.17, 0, 0.132), (1.39, 1.475, 1.23, 0, 0.09)],
    "500": [(0.78, 1.52, 1.3, 0, 0.09), (1.0, 1.585, 1.2, 0, 0.125), (1.17, 1.615, 1.17, 0, 0.135), (1.36, 1.52, 1.2, 0, 0.11)],
}

BREAST = {
    "50": [(1.2, 0.975, 0.905, -0.066, 0.03), (1.25, 1.04, 0.897, -0.07, 0.052), (1.31, 1.09, 0.902, -0.073, 0.064), (1.37, 1.12, 0.93, -0.073, 0.066), (1.415, 1.125, 0.97, -0.07, 0.058), (1.445, 1.11, 1.01, -0.065, 0.042), (1.46, 1.09, 1.04, -0.06, 0.02)],
    "100": [(1.21, 0.99, 0.902, -0.067, 0.038), (1.29, 1.075, 0.9, -0.072, 0.06), (1.37, 1.12, 0.93, -0.073, 0.066), (1.43, 1.12, 0.99, -0.068, 0.05), (1.46, 1.09, 1.04, -0.06, 0.024)],
}

NECK_ROOT = {
    "50": [(1.26, 1.33, 1.02, 0, 0.1), (1.32, 1.37, 1.0, 0, 0.15), (1.38, 1.38, 1.02, 0, 0.165), (1.43, 1.36, 1.05, 0, 0.15), (1.47, 1.31, 1.09, 0, 0.115), (1.495, 1.26, 1.13, 0, 0.065)],
    "100": [(1.27, 1.34, 1.02, 0, 0.11), (1.36, 1.38, 1.01, 0, 0.162), (1.44, 1.35, 1.06, 0, 0.142), (1.49, 1.275, 1.12, 0, 0.075)],
    "200": [(1.28, 1.345, 1.02, 0, 0.12), (1.38, 1.38, 1.02, 0, 0.165), (1.485, 1.29, 1.11, 0, 0.085)],
    "500": [(1.28, 1.345, 1.02, 0, 0.15), (1.38, 1.38, 1.0, 0, 0.2), (1.45, 1.34, 1.05, 0, 0.19), (1.5, 1.28, 1.11, 0, 0.1)],
}

NECK_1 = {
    "50": [(-0.1, 0.05, 0, 0.13, 0.085), (-0.04, 0.066, 0, 0.19, 0.118), (0.04, 0.078, 0, 0.228, 0.136), (0.13, 0.08, 0, 0.236, 0.138), (0.22, 0.075, 0, 0.222, 0.13), (0.32, 0.064, 0, 0.198, 0.118), (0.42, 0.052, 0, 0.174, 0.108), (0.5, 0.042, 0, 0.158, 0.1), (0.57, 0.034, 0, 0.142, 0.092), (0.62, 0.028, 0, 0.112, 0.076)],
    "100": [(-0.09, 0.052, 0, 0.14, 0.09), (0.0, 0.074, 0, 0.214, 0.13), (0.13, 0.08, 0, 0.236, 0.138), (0.28, 0.068, 0, 0.207, 0.122), (0.44, 0.05, 0, 0.17, 0.106), (0.56, 0.035, 0, 0.144, 0.093), (0.62, 0.028, 0, 0.112, 0.076)],
    "200": [(-0.08, 0.055, 0, 0.15, 0.095), (0.06, 0.079, 0, 0.23, 0.137), (0.25, 0.072, 0, 0.214, 0.126), (0.46, 0.047, 0, 0.166, 0.104), (0.61, 0.03, 0, 0.118, 0.08)],
}

SHOULDER = {
    "50": [(-0.035, 0.0, -0.075, 0.02, 0.016), (-0.01, 0.0, -0.088, 0.062, 0.032), (0.04, 0.002, -0.108, 0.11, 0.046), (0.11, 0.0, -0.148, 0.152, 0.058), (0.2, -0.012, -0.178, 0.184, 0.066), (0.3, -0.038, -0.192, 0.2, 0.07), (0.38, -0.054, -0.195, 0.19, 0.07), (0.45, -0.05, -0.19, 0.15, 0.064), (0.5, -0.032, -0.178, 0.092, 0.05), (0.53, -0.02, -0.168, 0.03, 0.022)],
    "100": [(-0.03, 0.0, -0.078, 0.03, 0.022), (0.03, 0.002, -0.104, 0.1, 0.044), (0.13, -0.002, -0.156, 0.16, 0.06), (0.26, -0.028, -0.188, 0.196, 0.069), (0.4, -0.054, -0.194, 0.18, 0.069), (0.49, -0.036, -0.18, 0.105, 0.053), (0.53, -0.02, -0.168, 0.036, 0.026)],
    "200": [(-0.025, 0.0, -0.08, 0.04, 0.03), (0.08, 0.001, -0.14, 0.138, 0.054), (0.25, -0.026, -0.187, 0.195, 0.068), (0.42, -0.053, -0.193, 0.17, 0.067), (0.525, -0.022, -0.169, 0.05, 0.036)],
}

ARM = {
    "50": [(-0.088, 0.004, -0.152, 0.014, 0.012), (-0.078, 0.003, -0.154, 0.04, 0.034), (-0.055, 0.0, -0.158, 0.066, 0.052), (-0.02, -0.008, -0.163, 0.09, 0.066), (0.03, -0.03, -0.167, 0.116, 0.075), (0.1, -0.06, -0.169, 0.14, 0.08), (0.18, -0.08, -0.169, 0.145, 0.078), (0.25, -0.076, -0.167, 0.126, 0.072), (0.31, -0.06, -0.165, 0.1, 0.064), (0.36, -0.04, -0.163, 0.074, 0.055), (0.4, -0.022, -0.161, 0.042, 0.036)],
    "100": [(-0.086, 0.004, -0.153, 0.022, 0.018), (-0.06, 0.0, -0.157, 0.062, 0.05), (-0.01, -0.012, -0.164, 0.096, 0.069), (0.08, -0.052, -0.169, 0.134, 0.079), (0.18, -0.08, -0.169, 0.145, 0.078), (0.28, -0.069, -0.166, 0.113, 0.068), (0.36, -0.04, -0.163, 0.074, 0.055), (0.4, -0.022, -0.161, 0.042, 0.036)],
    "200": [(-0.082, 0.003, -0.154, 0.034, 0.03), (-0.03, -0.006, -0.162, 0.084, 0.063), (0.08, -0.052, -0.169, 0.134, 0.079), (0.2, -0.08, -0.169, 0.142, 0.077), (0.32, -0.056, -0.165, 0.095, 0.062), (0.4, -0.022, -0.161, 0.045, 0.038)],
}

ARM_500 = [(-0.082, 0.003, -0.154, 0.04, 0.0375), (-0.02, -0.008, -0.163, 0.09, 0.066), (0.1, -0.06, -0.169, 0.14, 0.08), (0.22, -0.08, -0.168, 0.136, 0.076), (0.34, -0.05, -0.164, 0.085, 0.058), (0.4, -0.022, -0.161, 0.045, 0.04)]

TAIL = {
    "tail-1": [(-0.04, 0.0, 0, 0.03, 0.03), (-0.01, -0.006, 0, 0.05, 0.052), (0.05, -0.014, 0, 0.062, 0.066), (0.12, -0.02, 0, 0.068, 0.074), (0.19, -0.024, 0, 0.073, 0.08), (0.25, -0.026, 0, 0.076, 0.084)],
    "tail-2": [(-0.04, -0.022, 0, 0.066, 0.072), (0.03, -0.027, 0, 0.076, 0.084), (0.11, -0.03, 0, 0.08, 0.088), (0.19, -0.028, 0, 0.082, 0.09), (0.26, -0.024, 0, 0.082, 0.09)],
    "tail-3": [(-0.04, -0.026, 0, 0.074, 0.082), (0.04, -0.024, 0, 0.082, 0.09), (0.12, -0.022, 0, 0.08, 0.088), (0.2, -0.018, 0, 0.074, 0.082), (0.28, -0.014, 0, 0.066, 0.074)],
    "tail-4": [(-0.04, -0.016, 0, 0.06, 0.068), (0.04, -0.012, 0, 0.066, 0.074), (0.11, -0.008, 0, 0.058, 0.064), (0.17, -0.004, 0, 0.044, 0.048), (0.21, 0.0, 0, 0.026, 0.028), (0.232, 0.0, 0, 0.01, 0.01)],
}


DECISIONS = {
    "50": {
        "en": "Rework of 2026-11-22: the trunk is one loft of 23 upright sections from the points of the buttock to the breastbone with the croup at 1.60 m, the back at 1.52 m and the barrel 0.61 m wide; the withers are a ridge that rises to 1.615 m and runs on under the crest of the neck, so the withers stand above the croup as on a warmblood; the breast is a mass between the points of the shoulder with the two pectoral muscles under it; the shoulder is the muscle sheet over the blade widened towards the triceps and stands 20 mm proud of the ribs; the upper arm starts as a dome at the point of the shoulder; the tail pieces share their end sections so no step shows. In the skeleton the seven cervical vertebrae follow the S of the equine neck, the tuber sacrale lie beside the midline on the sacrum, the dock continues the sacrum and the ear cartilages stand on the skull; a horse has no clavicle, the trunk hangs between the shoulder blades.",
        "de": "Überarbeitung vom 22.11.2026: Der Rumpf ist ein Loft aus 23 aufrechten Schnitten von den Sitzbeinhöckern bis zum Brustbein mit der Kruppe auf 1,60 m, dem Rücken auf 1,52 m und 0,61 m Rumpfbreite; der Widerrist ist ein Kamm, der auf 1,615 m steigt und unter dem Halskamm weiterläuft, sodass er wie beim Warmblut über der Kruppe steht; die Vorbrust ist eine Masse zwischen den Buggelenken mit den beiden Brustmuskeln darunter; die Schulter ist die Muskelplatte über dem Blatt, zum Trizeps hin verbreitert und 20 mm vor den Rippen; der Oberarm beginnt als Kuppe am Buggelenk; die Schweifstücke teilen ihre Endschnitte, sodass keine Stufe sichtbar ist. Im Skelett folgen die sieben Halswirbel dem S des Pferdehalses, die Kreuzhöcker liegen neben der Mitte auf dem Kreuzbein, die Schweifrübe setzt das Kreuzbein fort und die Ohrknorpel stehen auf dem Schädel; ein Pferd hat kein Schlüsselbein, der Rumpf hängt zwischen den Schulterblättern.",
    },
    "100": {
        "en": "Rework of 2026-11-22: the same horse cut for 15 mm: a trunk of 14 sections, a withers ridge of 7, breast, pectorals, shoulder and upper arm with their own shorter tables, the tail pieces joined without steps; the skeleton is joined at the pelvis by a pubis under the hip sockets.",
        "de": "Überarbeitung vom 22.11.2026: dasselbe Pferd für 15 mm geschnitten: ein Rumpf aus 14 Schnitten, ein Widerristkamm aus 7, Vorbrust, Brustmuskeln, Schulter und Oberarm mit eigenen kürzeren Tabellen, die Schweifstücke ohne Stufen verbunden; das Skelett ist am Becken durch ein Schambein unter den Hüftpfannen verbunden.",
    },
    "200": {
        "en": "Rework of 2026-11-22: for 30 mm the trunk has 10 sections, the withers 5, the breast 3; shoulder and upper arm keep their mass with 5 and 6 sections, the pectorals are left to the breast.",
        "de": "Überarbeitung vom 22.11.2026: für 30 mm hat der Rumpf 10 Schnitte, der Widerrist 4 bis 5, die Vorbrust 3; Schulter und Oberarm behalten ihre Masse mit 5 und 6 Schnitten, die Brustmuskeln gehen in der Vorbrust auf.",
    },
    "500": {
        "en": "Rework of 2026-11-22: for 75 mm the trunk has 7 sections with the full 0.60 m breadth, the withers 4; the upper foreleg is the arm with a domed point of the shoulder instead of a capped tube. No skeleton member is thinner than 75 mm, the hip bones are joined across the sacrum by the wings of the ilium and the shoulder blades lie on the ribs.",
        "de": "Überarbeitung vom 22.11.2026: für 75 mm hat der Rumpf 7 Schnitte mit der vollen Breite von 0,60 m, der Widerrist 4; das obere Vorderbein ist der Oberarm mit einer Kuppe am Buggelenk statt eines gekappten Rohrs. Kein Skelettstab ist dünner als 75 mm, die Hüftbeine sind über das Kreuzbein durch die Darmbeinschaufeln verbunden und die Schulterblätter liegen auf den Rippen.",
    },
}


def solid(listing, ident):
    for entry in listing:
        if entry.get("id") == ident:
            return entry
    return None


def reloft(listing, ident, sections):
    entry = solid(listing, ident)
    if entry is None:
        return
    for key in ("radii", "transform", "p0", "p1", "r0", "r1"):
        entry.pop(key, None)
    ordered = {}
    for key, value in entry.items():
        ordered[key] = value
        if key == "bone":
            ordered["kind"] = "loft"
    entry.clear()
    entry.update(ordered)
    entry["kind"] = "loft"
    entry["sections"] = sections
    entry["capStart"] = True
    entry["capEnd"] = True


def main():
    with open(SOURCE, encoding="utf-8") as handle:
        document = json.load(handle)
    for scale, volume in document["volumes"].items():
        body = volume["body"]
        reloft(body, "trunk", upright("lumbar", TRUNK[scale], 2.2))
        reloft(body, "withers", upright("thorax", WITHERS[scale], 2.0))
        if scale in BREAST:
            reloft(body, "near-breast", upright("thorax", BREAST[scale], 2.0))
            reloft(body, "far-breast", upright("thorax", [(r[0], r[1], r[2], -r[3], r[4]) for r in BREAST[scale]], 2.0))
        root = solid(body, "neck-root")
        if root is not None:
            root["bone"] = "thorax"
            root.pop("weights", None)
            reloft(body, "neck-root", upright("thorax", NECK_ROOT[scale], 2.0))
        if scale in NECK_1:
            reloft(body, "neck-1", along(NECK_1[scale], 2.2))
            reloft(body, "near-shoulder", along(SHOULDER[scale], 2.0))
            reloft(body, "far-shoulder", along(SHOULDER[scale], 2.0, True))
            reloft(body, "near-arm", along(ARM[scale], 2.1))
            reloft(body, "far-arm", along(ARM[scale], 2.1, True))
        if scale == "50":
            for ident, rows in TAIL.items():
                reloft(body, ident, along(rows, 2.1))
        else:
            previous = None
            for ident in ("tail-1", "tail-2", "tail-3", "tail-4"):
                sections = solid(body, ident)["sections"]
                if len(sections) > 2 and abs(sections[-1]["at"][0] - sections[-2]["at"][0]) < 0.006:
                    sections.pop()
                if previous is not None:
                    sections[0]["at"][0] = -0.03
                    sections[0]["rx"], sections[0]["ry"] = round(previous["rx"] * 0.97, 4), round(previous["ry"] * 0.97, 4)
                previous = sections[-1]
        if scale == "500":
            reloft(body, "neck-a", along(NECK_1["200"], 2.2))
            reloft(body, "near-fore-upper", along(ARM_500, 2.1))
            reloft(body, "far-fore-upper", along(ARM_500, 2.1, True))
        note = DECISIONS[scale]
        volume["decisions"] = [entry for entry in volume["decisions"] if not entry["en"].startswith("Rework of 2026-11-22")] + [note]
    with open(SOURCE, "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    print("[DEBUG] written", SOURCE)


main()
