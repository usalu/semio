"""💪 Stage 4 of humans/leaning-person: the 1:50 forearm sleeve leaves the elbow at the girth of the upper sleeve.

The upper sleeve ends 92 x 97 mm at the elbow, the forearm sleeve began 84 x 88 mm, which read as a rolled edge on the hanging arm.
Half breadth and half depth of the four sections round the elbow are set by hand; the air layer below stays 4 mm inside.
Loaded by rework-leaning-person-build.py after stage 3.
"""

ELBOW = {2: (0.043, 0.045), 3: (0.0455, 0.048), 4: (0.0455, 0.0475), 5: (0.044, 0.0455)}
AIR = 0.004


def apply(a, m):
    for s in a["volumes"]["50"]["body"] + a["volumes"]["50"]["thermal"]:
        inset = AIR if s.get("id", "").startswith("th-sleeve-air-fore-") else 0 if s.get("id") in ("forearm-l", "forearm-r") else None
        if inset is None:
            continue
        for index, (rx, ry) in ELBOW.items():
            s["sections"][index]["rx"] = round(rx - inset, 5)
            s["sections"][index]["ry"] = round(ry - inset, 5)
