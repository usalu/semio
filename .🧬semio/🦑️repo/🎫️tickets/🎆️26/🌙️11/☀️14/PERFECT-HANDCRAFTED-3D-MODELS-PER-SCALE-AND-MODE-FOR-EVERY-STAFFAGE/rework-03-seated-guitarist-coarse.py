"""🎚️ Seated guitarist: the coarse scales (1:100, 1:200, 1:500) modelled for their own feature size: guitar neck with fretboard as one solid, tuner rows, no fret wires, hand skeleton merged.

Usage: python3 rework-03-seated-guitarist-coarse.py   edits the candidate document of rework-03-seated-guitarist-rig.py in place (run once after it);
       rework-03-seated-guitarist-rig.py --apply then writes the candidate into catalog and source.
Also closes the load path of the skeleton at every scale (temporal bones, acetabula, sacrum to the sacroiliac joints, ischia on the seat) and merges the fingers at 1:200 and 1:500.
"""
import importlib.util
import json
import math
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(T, "volumes", "humans", "seated-guitarist.json")
CANDIDATE = os.path.join(T, "🗑️generated", "rework-seated-guitarist", "cand", "🔣️.json")
LIMB = [0, -0.70711, 0, 0.70711]
NECK = [(0.134, 0.02983), (0.39, 0.02517), (0.5797, 0.0218)]
ROW_OFFSET = 0.043
HEAD_AXIS_Z = -0.1365


def neck():
    return {"id": "guitar-neck", "kind": "loft", "bone": "guitar", "material": "mahogany", "role": "body", "tone": 0.4,
            "sections": [{"at": [x, -0.04835, HEAD_AXIS_Z], "rx": rx, "ry": 0.01335, "exponent": 4, "rotation": LIMB} for x, rx in NECK], "capStart": True, "capEnd": True}


def tuner_row(name, sign):
    z = round(HEAD_AXIS_Z + sign * ROW_OFFSET, 5)
    return {"id": f"guitar-tuners-{name}", "bone": "guitar", "kind": "capsule", "p0": [0.607, -0.0352, z], "p1": [0.693, -0.0137, z], "r0": 0.0078, "r1": 0.0078, "material": "steel", "role": "detail", "tone": 0.7}


def hand_bones(side):
    return {"id": f"hand-bones-{side}", "kind": "ellipsoid", "bone": f"hand-{side}", "material": "bone", "role": "body", "tone": 0.62, "radii": [0.05, 0.0185, 0.04], "transform": {"translation": [0.05, 0, 0]}, "member": f"hand-{side}", "part": "member"}


def coarse(volumes):
    for scale in ("100", "200", "500"):
        volume = volumes[scale]
        body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
        if any(s["id"] == "guitar-tuners-b" for s in body):
            continue
        drop = re.compile(r"guitar-(fretboard|nut|tuner-.*)$|fret-wire-\d+$" + ("" if scale == "100" else r"|guitar-string-\d$") + ("" if scale == "100" else r"|thenar-[lr]$"))
        gone = {s["id"] for s in body if drop.match(s["id"])}
        at = next(i for i, s in enumerate(body) if s["id"] == "guitar-neck")
        body[at] = neck()
        body[:] = [s for s in body if s["id"] not in gone] + [tuner_row("b", -1), tuner_row("t", 1)]
        thermal[:] = [s for s in thermal if s.get("of") not in gone] + [{"id": f"th-guitar-tuners-{n}", "of": f"guitar-tuners-{n}", "region": "guitar"} for n in ("b", "t")]
        structure[:] = [s for s in structure if s.get("of") not in gone]
        for solid in structure:
            key = solid["id"]
            if re.match(r"metacarpal-\d-[lr]$", key) and scale in ("100", "200"):
                solid["slender"] = True
            if scale != "100" and re.match(r"joint-hand-[lr]$", key):
                r = 0.015 if scale == "200" else 0.0185
                solid["radii"] = [r, r, r]
            if scale == "500" and key == "joint-head":
                solid["radii"] = [0.0375, 0.0375, 0.0375]
        if scale != "100":
            structure[:] = [s for s in structure if not re.match(r"occipital-condyle-[lr]$|carpals-[lr]$", s["id"])]
        if scale == "500":
            structure[:] = [s for s in structure if not re.match(r"metacarpal-\d-[lr]$", s["id"])] + [hand_bones("l"), hand_bones("r")]


def rig_module():
    spec = importlib.util.spec_from_file_location("rig_lateral", os.path.join(T, "rig-lateral-standing-adult.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def member(solid, bone):
    solid.update({"material": "bone", "role": "body", "tone": 0.62, "bone": bone, "member": bone, "part": "member"})
    return solid


def sacrum(scale):
    stations = {"50": [(0.1, 0.04, 0.05, 0.02), (0.07, 0.055, 0.048, 0.018), (0.04, 0.072, 0.03, 0.014), (0.005, 0.082, 0.012, 0.01)],
                "100": [(0.1, 0.04, 0.05, 0.02), (0.07, 0.055, 0.048, 0.018), (0.04, 0.072, 0.03, 0.014), (0.005, 0.082, 0.012, 0.01)],
                "200": [(0.1, 0.04, 0.05, 0.02), (0.06, 0.06, 0.046, 0.016), (0.005, 0.082, 0.0152, 0.0152)]}[scale]
    return member({"id": "sacrum", "kind": "loft", "sections": [{"at": [x, y, 0], "rx": rx, "ry": ry, "exponent": 2, "rotation": LIMB} for x, y, rx, ry in stations], "capStart": True, "capEnd": True}, "pelvis")


def temporal(side):
    s = 1 if side == "l" else -1
    return member({"id": f"temporal-{side}", "kind": "ellipsoid", "radii": [0.018, 0.022, 0.012], "transform": {"translation": [0.012, -0.022, s * 0.05]}}, "head")


def ellipse(cx, cy, rx, ry, count=12):
    return [[round(cx + rx * math.cos(2 * math.pi * i / count), 4), round(cy + ry * math.sin(2 * math.pi * i / count), 4)] for i in range(count)]


FINGERS = (0.118, 0.022, 0.032, 0.03, 0.04)


def fit(asset):
    R = rig_module()
    rest = R.frames(asset["rig"]["bones"])
    pelvis = rest["pelvis"]
    length = next(b["lengthM"] for b in asset["rig"]["bones"] if b["id"] == "hip-bone-l")
    finger = re.compile(r"(pad|shaft|joint)-(index|middle|ring|little|thumb)-")
    for scale, volume in asset["volumes"].items():
        body, thermal, structure = volume["body"], volume["thermal"], volume["structure"]
        if any(s["id"] in ("acetabulum-l", "finger-bones-l") for s in structure):
            continue
        if scale == "50":
            structure[:] = [s for s in structure if not re.match(r"st-fret-wire-\d+$", s["id"])]
        if scale in ("50", "100"):
            structure += [temporal("l"), temporal("r")]
            for side in ("l", "r"):
                structure.append(member({"id": f"acetabulum-{side}", "kind": "ellipsoid", "radii": [0.032, 0.032, 0.032], "transform": {"translation": [length, 0, 0]}}, f"hip-bone-{side}"))
        if scale != "500":
            structure[next(i for i, s in enumerate(structure) if s["id"] == "sacrum")] = sacrum(scale)
        if scale == "200":
            structure[:] = [s for s in structure if s["id"] not in ("hip-bar-l", "hip-bar-r")]
            for side, s in (("l", 1), ("r", -1)):
                frame = rest[f"hip-bone-{side}"]
                local = lambda p: R.r5(R.apply(R.transpose(frame[1]), [a - b for a, b in zip([pelvis[0][i] + R.apply(pelvis[1], p)[i] for i in range(3)], frame[0])]))
                structure.append(member({"id": f"hip-bar-{side}", "kind": "capsule", "p0": [0, 0, 0], "p1": [length, 0, 0], "r0": 0.017, "r1": 0.017}, f"hip-bone-{side}"))
                structure.append(member({"id": f"ischium-{side}", "kind": "capsule", "p0": local([0.0, 0.0, s * 0.088]), "p1": local([-0.07, 0.02, s * 0.06]), "r0": 0.017, "r1": 0.017}, f"hip-bone-{side}"))
        if scale in ("200", "500"):
            gone = {s["id"] for s in body if finger.match(s["id"])}
            body[:] = [s for s in body if s["id"] not in gone]
            thermal[:] = [s for s in thermal if s.get("of") not in gone]
            structure[:] = [s for s in structure if not finger.match(s["id"]) and not re.match(r"(metacarpal-\d|carpals|hand-bones)-[lr]$", s["id"])]
            cx, cy, ru, rv, rw = FINGERS
            for side in ("l", "r"):
                body.append({"id": f"fingers-{side}", "kind": "ellipsoid", "bone": f"hand-{side}", "material": "skin", "role": "body", "tone": 0.74, "radii": [ru, rv, rw], "transform": {"translation": [cx, cy, 0]}})
                thermal.append({"id": f"th-fingers-{side}", "of": f"fingers-{side}", "region": "hands"})
                structure.append(hand_bones(side))
                structure.append(member({"id": f"finger-bones-{side}", "kind": "ellipsoid", "radii": [0.025, 0.02, 0.035], "transform": {"translation": [cx, cy, 0]}}, f"hand-{side}"))
            for view in ("elevation", "front", "plan"):
                shapes = asset["representations"][scale]["geometry"][view]
                shapes[:] = [s for s in shapes if not finger.match(s["id"])]
                for side in ("l", "r"):
                    palm = next(s for s in shapes if s["id"] == f"palm-{side}")
                    twin = json.loads(json.dumps(palm))
                    twin.update({"id": f"fingers-{side}", "points": ellipse(cx, cy, ru, rv), "z": round(palm["z"] + 0.01, 3)})
                    shapes.append(twin)


def main():
    document = json.load(open(CANDIDATE, encoding="utf8"))
    coarse(document["asset"]["volumes"])
    fit(document["asset"])
    json.dump(document, open(CANDIDATE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("written", CANDIDATE)


if __name__ == "__main__":
    main()
