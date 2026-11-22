"""🖐️ Stage 9 of the seated office worker (second round, pass 1): the hands get their real skeleton.

Per hand 15 bones as in the anatomy: `hand` (carpus and metacarpus), thumb (metacarpal, proximal, distal) and four fingers of three phalanges each, identical topology left and right, at rest in the relaxed curl of a typing hand. Every phalanx is its own hand-stated solid in body, thermal and structure at 1:50, the finger block follows the middle finger at 1:100. The typing clip taps with the index and middle fingers, the leaning-back pose lays the back on the backrest, the thigh sections lose their box corners.

Run: `python3 rework-seated-office-worker-9-fingers.py` writes the candidate document; `--apply` also writes the volumes source and the root members (rig, poses, clips) of the catalog file.
"""
import copy
import sys

from rework_03_lib import *

A = "humans/seated-office-worker"
Q = [0, -0.70711, 0, 0.70711]
FINGERS = {
    "index": {"mcp": (0.093, -0.002, -0.029), "lengths": (0.040, 0.023, 0.019), "r": 0.0095, "rest": (-16, -42, -26)},
    "middle": {"mcp": (0.096, -0.002, -0.0095), "lengths": (0.045, 0.027, 0.021), "r": 0.0099, "rest": (-16, -42, -26)},
    "ring": {"mcp": (0.092, -0.002, 0.0105), "lengths": (0.041, 0.026, 0.020), "r": 0.0093, "rest": (-17, -42, -26)},
    "little": {"mcp": (0.084, -0.003, 0.029), "lengths": (0.032, 0.018, 0.018), "r": 0.0083, "rest": (-20, -40, -24)},
}
TAPER = (1.0, 0.9, 0.82, 0.72)
THUMB = {"cmc": (0.018, -0.008, -0.022), "lengths": (0.046, 0.032, 0.027), "r": (0.0135, 0.012, 0.0105, 0.009), "rest": (-22, -6, -8), "spread": (-32, 18, 12)}
PARTS = ("proximal", "middle", "distal")
THUMB_PARTS = ("metacarpal", "proximal", "distal")
TITLES = {
    "index": ("index finger", "Zeigefinger"), "middle": ("middle finger", "Mittelfinger"), "ring": ("ring finger", "Ringfinger"), "little": ("little finger", "Kleinfinger"), "thumb": ("thumb", "Daumen"),
    "proximal": ("proximal phalanx", "Grundglied"), "middle-part": ("middle phalanx", "Mittelglied"), "distal": ("distal phalanx", "Endglied"), "metacarpal": ("metacarpal", "Mittelhandknochen"),
}
SIDES = {"l": (1, "Left", "links"), "r": (-1, "Right", "rechts")}
TAPS = {
    "index-proximal-r": (0, 9, 0, -3, 0, 7, 0, 0), "middle-proximal-r": (0, -3, 0, 6, 0, -2, 8, 0), "ring-proximal-r": (0, 0, 5, 0, 0, 0, -3, 0),
    "index-proximal-l": (0, 0, 9, -3, 7, 0, 0, 0), "middle-proximal-l": (0, 0, -3, 6, -2, 0, 7, 0), "ring-proximal-l": (0, 4, 0, 0, 0, 6, 0, 0),
    "thumb-proximal-r": (0, 0, 0, 5, 0, 0, 4, 0),
}


def bone(identity, parent, title, x, y, z, angle, length, low, high, spread=None):
    """🦴 One revolute rig bone with its lateral offset and rest spread."""
    out = {"id": identity, "parent": parent, "title": {"en": title[0], "de": title[1]}, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "joint": {"type": "revolute", "minDeg": low, "maxDeg": high}}
    if z:
        out["z"] = z
    if spread:
        out["abductionDeg"] = spread
    return out


def finger_bones(side):
    """🖐️ The fourteen finger bones of one hand."""
    sign, en, de = SIDES[side]
    out = []
    for i, part in enumerate(THUMB_PARTS):
        title = (f"{en} {TITLES['thumb'][0]}, {TITLES[part][0]}", f"{TITLES['thumb'][1]} {de}, {TITLES[part][1]}")
        x, y, z = THUMB["cmc"] if i == 0 else (THUMB["lengths"][i - 1], 0, 0)
        parent = f"hand-{side}" if i == 0 else f"thumb-{THUMB_PARTS[i - 1]}-{side}"
        out.append(bone(f"thumb-{part}-{side}", parent, title, x, y, sign * z, THUMB["rest"][i], THUMB["lengths"][i], -50 if i else -30, 30, sign * THUMB["spread"][i]))
    for name, f in FINGERS.items():
        for i, part in enumerate(PARTS):
            title = (f"{en} {TITLES[name][0]}, {TITLES['middle-part' if part == 'middle' else part][0]}", f"{TITLES[name][1]} {de}, {TITLES['middle-part' if part == 'middle' else part][1]}")
            x, y, z = f["mcp"] if i == 0 else (f["lengths"][i - 1], 0, 0)
            parent = f"hand-{side}" if i == 0 else f"{name}-{PARTS[i - 1]}-{side}"
            out.append(bone(f"{name}-{part}-{side}", parent, title, x, y, sign * z, f["rest"][i], f["lengths"][i], (-75, -70, -55)[i], (35, 5, 10)[i]))
    return out


def loft(identity, bone_id, material, tone, rows, exponent=None, w=0, **more):
    """🧵 A loft along the bone from rows of (u, half width, half thickness)."""
    sections = []
    for u, rx, ry in rows:
        section = {"at": [round(u, 5), 0, round(w, 5)], "rx": round(rx, 5), "ry": round(ry, 5), "rotation": Q}
        if exponent:
            section["exponent"] = exponent
        sections.append(section)
    return {"id": identity, "kind": "loft", "bone": bone_id, "material": material, "role": "body", "tone": tone, "sections": sections, "capStart": True, "capEnd": True, **more}


def phalanx(identity, bone_id, length, r0, r1, tip, tone, flat=0.92):
    """☝️ The flesh of one phalanx: knuckle ball over the joint, waist, and either the next knuckle or the finger pad."""
    rows = [(-0.7 * r0, 0.55 * r0), (-0.35 * r0, 0.88 * r0), (0, 1.04 * r0)]
    if tip:
        rows += [(0.45 * length, 0.96 * r0), (length - 0.9 * r1, r1), (length - 0.4 * r1, 0.85 * r1), (length - 0.12 * r1, 0.55 * r1), (length, 0.25 * r1)]
    else:
        rows += [(0.4 * length, 0.6 * r0 + 0.4 * r1), (0.8 * length, r1), (length, 1.02 * r1), (length + 0.4 * r1, 0.8 * r1), (length + 0.7 * r1, 0.45 * r1)]
    return loft(identity, bone_id, "skin", tone, [(u, k, flat * k) for u, k in rows])


def ossicle(identity, bone_id, length, r, tip):
    """🦴 One phalanx bone: base, shaft and head (or the tuft of a distal phalanx)."""
    rows = [(0, 1.25 * r), (0.18 * length, r), ((0.6 if tip else 0.8) * length, r), (length - 0.004 if tip else length, 1.2 * r if not tip else 1.1 * r)]
    return loft(identity, bone_id, "bone", 0.62, [(u, k, k) for u, k in rows], member=bone_id, part="member")


def hand_50(side):
    """🖐️ Body and structure solids of the digits of one hand at 1:50."""
    body, structure = [], []
    radii = THUMB["r"]
    for i, part in enumerate(THUMB_PARTS):
        b = f"thumb-{part}-{side}"
        body.append(phalanx(b, b, THUMB["lengths"][i], radii[i], radii[i + 1], i == 2, 0.74, 0.9))
        structure.append(ossicle(f"phalanx-{b}" if i else f"metacarpal-{b}", b, THUMB["lengths"][i], 0.0046, i == 2))
    for n, (name, f) in enumerate(FINGERS.items()):
        for i, part in enumerate(PARTS):
            b = f"{name}-{part}-{side}"
            body.append(phalanx(b, b, f["lengths"][i], f["r"] * TAPER[i], f["r"] * TAPER[i + 1], i == 2, 0.74 if n % 2 == 0 else 0.72))
            structure.append(ossicle(f"phalanx-{b}", b, f["lengths"][i], 0.004, i == 2))
    return body, structure


def hand_100(side):
    """🧤 The finger block and the thumb of one hand at 1:100 (15 mm features), on the middle finger and thumb bones."""
    sign = SIDES[side][0]
    f = FINGERS["middle"]
    body, structure = [], []
    widths = ((0.041, 0.0095), (0.040, 0.009), (0.038, 0.0085), (0.034, 0.008))
    for i, part in enumerate(PARTS):
        b = f"middle-{part}-{side}"
        length = f["lengths"][i]
        (w0, t0), (w1, t1) = widths[i], widths[i + 1]
        rows = [(-0.008, w0, t0), (0, w0, t0), (length, w1, t1), (length + 0.006, w1, t1)] if i < 2 else [(-0.008, w0, t0), (0, w0, t0), (length - 0.006, w1, t1), (length, 0.8 * w1, t1)]
        body.append(loft(f"fingers-{part}-{side}", b, "skin", 0.74, rows, 2.8, sign * 0.0095))
        structure.append(loft(f"finger-bones-{part}-{side}", b, "bone", 0.62, [(0, 0.033, 0.008), (length - 0.003, 0.031, 0.008)], 3, sign * 0.0095, member=b, part="member"))
    lengths = THUMB["lengths"]
    body.append(loft(f"thumb-{side}", f"thumb-metacarpal-{side}", "skin", 0.74, [(-0.008, 0.009, 0.009), (0, 0.0128, 0.0125), (lengths[0], 0.0115, 0.011), (lengths[0] + 0.007, 0.008, 0.008)]))
    body.append(loft(f"thumb-tip-{side}", f"thumb-proximal-{side}", "skin", 0.74, [(-0.006, 0.009, 0.009), (0, 0.011, 0.0105), (0.03, 0.01, 0.0095), (0.05, 0.0092, 0.0085), (0.057, 0.0075, 0.0075)]))
    structure.append(loft(f"thumb-bones-{side}", f"thumb-metacarpal-{side}", "bone", 0.62, [(0, 0.0078, 0.0078), (lengths[0], 0.0076, 0.0076)], member=f"thumb-metacarpal-{side}", part="member"))
    structure.append(loft(f"thumb-tip-bones-{side}", f"thumb-proximal-{side}", "bone", 0.62, [(0, 0.0076, 0.0076), (0.05, 0.0075, 0.0075)], member=f"thumb-proximal-{side}", part="member"))
    return body, structure


def replace(items, drop, after, fresh):
    """🔁 Removes the solids named in `drop` and every earlier copy of `fresh`, then inserts `fresh` after the solid `after`."""
    ids = set(drop) | {x["id"] for x in fresh}
    kept = [x for x in items if x["id"] not in ids]
    at = next(i for i, x in enumerate(kept) if x["id"] == after) + 1
    return kept[:at] + fresh + kept[at:]


doc = read(catalog_path(A))
a = doc["asset"]
a["volumes"] = read(source_path(A))["volumes"]
v = a["volumes"]

bones = [b for b in a["rig"]["bones"] if not any(b["id"].startswith(p) for p in ("thumb-", "index-", "middle-", "ring-", "little-"))]
for side in SIDES:
    at = next(i for i, b in enumerate(bones) if b["id"] == f"hand-{side}") + 1
    bones[at:at] = finger_bones(side)
    hand = bones[at - 1]
    hand["title"] = {"en": f"{SIDES[side][1]} hand (carpus and metacarpus; the bone runs to the tip of the straight middle finger)", "de": f"Hand {SIDES[side][2]} (Handwurzel und Mittelhand; der Knochen reicht bis zur Spitze des gestreckten Mittelfingers)"}
a["rig"]["bones"] = bones

for side in SIDES:
    sign = SIDES[side][0]
    body, structure = hand_50(side)
    old = [f"finger-{n}-{side}" for n in (1, 2, 3, 4)] + [f"thumb-{side}"]
    v["50"]["body"] = replace(v["50"]["body"], old, f"thenar-{side}", body)
    v["50"]["thermal"] = replace(v["50"]["thermal"], [f"th-{x}" for x in old], f"th-palm-{side}", [{"id": f"th-{x['id']}", "of": x["id"], "region": "hand-skin"} for x in body])
    v["50"]["structure"] = replace(v["50"]["structure"], [f"phalanges-{n}-{side}" for n in (1, 2, 3, 4)] + [f"thumb-bones-{side}"], f"metacarpal-4-{side}", structure)
    for n, f in enumerate(FINGERS.values(), 1):
        ray = next(x for x in v["50"]["structure"] if x["id"] == f"metacarpal-{n}-{side}")
        ray["sections"][-1]["at"] = [f["mcp"][0], f["mcp"][1], sign * f["mcp"][2]]
    for thigh in (x for x in v["50"]["body"] if x["id"] == f"thigh-{side}"):
        for section in thigh["sections"]:
            if section.get("exponent", 2) > 2.3:
                section["exponent"] = 2.3

    body, structure = hand_100(side)
    palm = next(x for x in v["100"]["body"] if x["id"] == f"hand-{side}")
    palm["sections"] = [s for s in palm["sections"] if s["at"][0] <= 0.096] + [{"at": [0.104, -0.003, 0], "rx": 0.041, "ry": 0.0095, "exponent": 2.6, "rotation": Q}]
    v["100"]["body"] = replace(v["100"]["body"], [f"thumb-{side}"], f"hand-{side}", body)
    v["100"]["thermal"] = replace(v["100"]["thermal"], [f"th-thumb-{side}"], f"th-hand-{side}", [{"id": f"th-{x['id']}", "of": x["id"], "region": "hand-skin"} for x in body])
    v["100"]["structure"] = replace(v["100"]["structure"], [], f"hand-bones-{side}", structure)

next(x for x in v["200"]["structure"] if x["id"] == "mass-desk-top")["radii"][1] = 0.0125

pose = next(p for p in a["poses"] if p["id"] == "leaning-back")["bones"]
LEAN = float(sys.argv[sys.argv.index("--lean") + 1]) if "--lean" in sys.argv else 5
pose["pelvis"]["angleDeg"] = LEAN
pose["lumbar"]["angleDeg"] = 8
pose["torso"]["angleDeg"] = 8
pose["hip-shift"]["dx"] = -0.085
for side in SIDES:
    pose[f"thigh-{side}"]["angleDeg"] = round(-0.873 - LEAN, 3)
pose["arm-r"]["angleDeg"] = round(-23.1 - LEAN - 4, 3)

clip = next(c for c in a["clips"] if c["id"] == "typing")
for i, key in enumerate(clip["keys"]):
    for b, taps in TAPS.items():
        key["bones"][b] = {"angleDeg": taps[i % len(taps)]}

for d in v["50"]["decisions"]:
    d["en"] = d["en"].replace("a 780 x 600 x 720 mm desk", "a 1200 x 800 x 720 mm desk").replace("four three-part fingers curled onto the home row and the thumb over the space bar", "four fingers of three phalanges and a thumb of metacarpal and two phalanges, every segment a solid on its own bone, curled onto the home row and over the space bar")
    d["de"] = d["de"].replace("ein Schreibtisch 780 x 600 x 720 mm", "ein Schreibtisch 1200 x 800 x 720 mm").replace("vier dreigliedrigen, auf der Grundreihe gekrümmten Fingern und dem Daumen über der Leertaste", "vier Fingern aus je drei Gliedern und einem Daumen aus Mittelhandknochen und zwei Gliedern, jedes Glied ein Körper auf seinem eigenen Knochen, gekrümmt auf der Grundreihe und über der Leertaste")

write(candidate_path(A), doc)
print("candidate", candidate_path(A), {k: {l: len(v[k][l]) for l in ("body", "thermal", "structure")} for k in v}, len(bones), "bones")
if "--apply" in sys.argv:
    write(source_path(A), {"volumes": v})
    live = read(catalog_path(A))
    for key in ("rig", "poses", "clips"):
        live["asset"][key] = a[key]
    write(catalog_path(A), live)
    print("applied")
