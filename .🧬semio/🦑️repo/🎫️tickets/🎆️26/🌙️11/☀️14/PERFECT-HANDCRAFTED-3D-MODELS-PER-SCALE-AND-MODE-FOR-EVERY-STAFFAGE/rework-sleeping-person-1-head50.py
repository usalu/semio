"""🗿 Re-sculpts the 1:50 head of `humans/sleeping-person` by hand: one skull-and-face loft through the mid-sagittal profile, nose, closed lids, lips, auricles and a hair mass that stays on the head in every pose.

Head frame: u from the atlanto-occipital joint to the vertex, -v is the face, w is the left. Every number below is a hand-chosen station of a woman's head (chin-vertex 0.229, glabella-opisthocranion 0.187, breadth 0.148, bizygomatic 0.134, bigonial 0.10 m).
"""
import json
import math
from pathlib import Path

T = Path(__file__).resolve().parent
SRC = T / "volumes/humans/sleeping-person.json"
ROT = [0, -0.70711, 0, 0.70711]
SKIN, LIP, HAIR = 0.82, 0.6, 0.22

PROFILE = [
    (-0.094, -0.066, -0.040, 0.012, 2.0),
    (-0.089, -0.078, -0.022, 0.024, 2.0),
    (-0.080, -0.0855, 0.006, 0.034, 2.0),
    (-0.070, -0.0850, 0.022, 0.041, 2.0),
    (-0.061, -0.0825, 0.034, 0.046, 2.1),
    (-0.052, -0.0885, 0.044, 0.050, 2.1),
    (-0.045, -0.0880, 0.051, 0.053, 2.1),
    (-0.038, -0.0910, 0.057, 0.056, 2.1),
    (-0.027, -0.0875, 0.066, 0.060, 2.2),
    (-0.012, -0.0850, 0.076, 0.065, 2.2),
    (0.004, -0.0835, 0.084, 0.0675, 2.2),
    (0.020, -0.0820, 0.089, 0.070, 2.3),
    (0.031, -0.0870, 0.091, 0.0715, 2.3),
    (0.042, -0.0940, 0.093, 0.073, 2.3),
    (0.056, -0.0930, 0.0945, 0.074, 2.3),
    (0.075, -0.0860, 0.092, 0.073, 2.3),
    (0.098, -0.0680, 0.080, 0.064, 2.3),
    (0.117, -0.0460, 0.060, 0.048, 2.3),
    (0.130, -0.0210, 0.035, 0.026, 2.3),
    (0.135, -0.0010, 0.015, 0.008, 2.3),
]
NOSE = [
    (0.036, -0.0840, 0.0070, 0.0060),
    (0.024, -0.0870, 0.0075, 0.0070),
    (0.010, -0.0890, 0.0090, 0.0090),
    (-0.002, -0.0910, 0.0115, 0.0115),
    (-0.012, -0.0925, 0.0135, 0.0135),
    (-0.019, -0.0910, 0.0125, 0.0110),
    (-0.025, -0.0880, 0.0080, 0.0050),
]
HAIR_CAP = [
    (-0.030, 0.006, 0.098, 0.066),
    (0.000, 0.000, 0.100, 0.076),
    (0.025, -0.022, 0.102, 0.081),
    (0.050, -0.080, 0.104, 0.083),
    (0.075, -0.089, 0.101, 0.081),
    (0.098, -0.076, 0.089, 0.072),
    (0.117, -0.055, 0.069, 0.056),
    (0.132, -0.031, 0.045, 0.034),
    (0.142, -0.006, 0.020, 0.012),
]
HAIR_NAPE = [
    (0.040, 0.050, 0.100, 0.070),
    (0.000, 0.040, 0.100, 0.080),
    (-0.040, 0.034, 0.096, 0.082),
    (-0.080, 0.028, 0.090, 0.080),
    (-0.120, 0.026, 0.082, 0.072),
    (-0.155, 0.030, 0.074, 0.056),
    (-0.172, 0.040, 0.064, 0.030),
]


def section(u, front, back, rx, exponent=2.2):
    return {"at": [u, round((front + back) / 2, 5), 0], "rx": rx, "ry": round((back - front) / 2, 5), "exponent": exponent, "rotation": ROT}


def loft(id, material, tone, sections):
    return {"id": id, "kind": "loft", "bone": "head", "material": material, "role": "body", "tone": tone, "sections": sections, "capStart": True, "capEnd": True}


def ellipsoid(id, tone, radii, at, rotation=None):
    transform = {"translation": at} if rotation is None else {"rotation": rotation, "translation": at}
    return {"id": id, "kind": "ellipsoid", "bone": "head", "material": "skin", "role": "body", "tone": tone, "radii": radii, "transform": transform}


def quat(axis, deg):
    s = math.sin(math.radians(deg) / 2)
    return [axis[0] * s, axis[1] * s, axis[2] * s, math.cos(math.radians(deg) / 2)]


def mul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [round(n, 5) for n in (aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz)]


def head_solids():
    out = [loft("head", "skin", SKIN, [section(u, f, b, rx, e) for u, f, b, rx, e in PROFILE])]
    out.append(loft("nose", "skin", SKIN, [{"at": [u, v, 0], "rx": rx, "ry": ry, "exponent": 2, "rotation": ROT} for u, v, rx, ry in NOSE]))
    for side, sign in (("l", 1), ("r", -1)):
        out.append(ellipsoid(f"ala-{side}", SKIN, [0.0075, 0.0065, 0.006], [-0.0165, -0.0885, sign * 0.0115]))
        out.append(ellipsoid(f"eyelid-{side}", SKIN, [0.0085, 0.006, 0.0145], [0.0205, -0.0725, sign * 0.031]))
        flare = mul(quat([1, 0, 0], sign * 22), quat([0, 0, 1], 14))
        out.append(ellipsoid(f"ear-{side}", SKIN, [0.031, 0.017, 0.0055], [0.019, 0.021, sign * 0.0745], flare))
        out.append(ellipsoid(f"ear-lobe-{side}", SKIN, [0.009, 0.007, 0.0045], [-0.009, 0.010, sign * 0.0700]))
    out.append(ellipsoid("lips", LIP, [0.0048, 0.0050, 0.021], [-0.0395, -0.0885, 0]))
    out.append(ellipsoid("lips-lower", LIP, [0.0052, 0.0052, 0.018], [-0.0505, -0.0868, 0]))
    out.append(loft("hair", "hair", HAIR, [section(u, f, b, rx) for u, f, b, rx in HAIR_CAP]))
    out.append(loft("hair-nape", "hair", HAIR, [section(u, f, b, rx, 2.4) for u, f, b, rx in HAIR_NAPE]))
    return out


def face_region():
    rows = [(u, f, min(b, 0.008), round(rx * 0.93, 5), e) for u, f, b, rx, e in PROFILE if u <= 0.075]
    rows.append((0.082, -0.070, -0.030, 0.030, 2.3))
    face = loft("th-face", "skin", SKIN, [section(u, f, b, rx, e) for u, f, b, rx, e in rows])
    face["region"] = "face-skin"
    return face


def main():
    doc = json.loads(SRC.read_text())
    volume = doc["volumes"]["50"]
    body = volume["body"]
    old = [s for s in body if s["bone"] == "head"]
    gone = {s["id"] for s in old}
    new = head_solids()
    kept = {s["id"] for s in new}
    first = body.index(old[0])
    volume["body"] = body[:first] + new + [s for s in body[first:] if s["bone"] != "head"]
    thermal = []
    for s in volume["thermal"]:
        if s.get("of") in gone - kept:
            print("[DEBUG] thermal reference dropped", s["id"], "of", s["of"], s.get("region"))
            continue
        thermal.append(face_region() if s["id"] == "th-face" else s)
    have = {s.get("of") for s in thermal}
    at = next(i for i, s in enumerate(thermal) if s["id"] == "th-face")
    extra = [{"id": f"th-{s['id']}", "of": s["id"], "region": "hair" if s["material"] == "hair" else "face-skin"} for s in new if s["id"] not in have and s["id"] not in ("head",)]
    print("[DEBUG] thermal references added", [e["id"] for e in extra])
    volume["thermal"] = thermal[:at] + extra + thermal[at:]
    SRC.write_text(json.dumps(doc, ensure_ascii=False))
    print("head solids", len(new), "thermal", len(volume["thermal"]))


main()
