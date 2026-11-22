"""🙅 Stage 5 of humans/leaning-person: in the folded-arms pose the left hand closes round the right upper arm.

The open hand reached 60 mm past the opposite elbow. The fingers now flex at all three joints (knuckle, middle and end joint,
degrees on top of the rest curl of 14 / 22 / 22) and the thumb closes against them. Loaded by rework-leaning-person-build.py after stage 4.
"""

CLASP = {"proximal": 46, "middle": 48, "distal": 26}
LITTLE = {"proximal": 52, "middle": 52, "distal": 28}
THUMB = {"thumb-proximal-l": 22, "thumb-distal-l": 28}


def apply(a, m):
    pose = next(p for p in a["poses"] if p["id"] == "leaning-arms-folded")
    for finger in ("index", "middle", "ring", "little"):
        for seg, angle in (LITTLE if finger == "little" else CLASP).items():
            pose["bones"][f"{finger}-{seg}-l"] = {"angleDeg": angle}
    for bone, angle in THUMB.items():
        pose["bones"][bone] = {"angleDeg": angle}
