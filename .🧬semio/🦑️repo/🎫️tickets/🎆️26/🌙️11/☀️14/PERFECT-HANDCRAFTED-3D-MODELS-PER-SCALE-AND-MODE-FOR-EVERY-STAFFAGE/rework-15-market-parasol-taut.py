"""⛱️ Rework 15 market parasol: the open canopy is taut. The gores were cut 0.8 degrees wider than the open frame spans (44.13 against 43.33 degrees) because a gust key lifted the runner and the ribs 4 degrees above their open position, which the stretched cloth of a real parasol forbids; open, every gore therefore hung in six zigzag pleats. The gust now sways the mast in its tilt joint, and the gores are cut for the open frame.

Hand-stated numbers: rib pitch 15.28033 degrees below the horizontal, eight ribs at 45 degrees in plan, so two neighbouring ribs enclose acos(cos^2 p cos 45 + sin^2 p) = 43.3317 degrees; cut = that + 0.06 degrees of ease, six facets per gore; cloth 1.3476 m along the rib, hem sag 12 mm.
Usage: python3 rework-15-market-parasol-taut.py [--apply]
"""
import math
import re
import sys

from rework_15_kit import Kit, SCALES

kit = Kit("objects/market-parasol")
SIDES = ["right", "b", "c", "d", "left", "f", "g", "h"]
FACETS, LENGTH, PITCH, EASE = 6, 1.3476, 15.28033, 0.06
INTERVALS = {"50": 3, "100": 2, "200": 1, "500": 1}
p = math.radians(PITCH)
OPEN = math.acos(math.cos(p) ** 2 * math.cos(math.radians(45)) + math.sin(p) ** 2)
STEP = (OPEN + math.radians(EASE)) / FACETS


def profile(step, i, scale):
    full = step * FACETS

    def at(j):
        phi, f = step * j, j / FACETS
        sag = 0.012 * (1 - abs(2 * f - 1)) if scale == "500" else 0.012 * 4 * f * (1 - f)
        r = LENGTH * math.cos(full / 2) / math.cos(phi - full / 2) - sag
        return [round(-r * math.sin(phi - i * step), 8), round(r * math.cos(phi - i * step), 8)]

    n = INTERVALS[scale]
    return [[0, 0]] + [at(i + j / n) for j in range(n + 1)]


def facet(id):
    found = re.fullmatch(r"(?:th-panel|th-underside|panel)-(%s)(?:-(\d))?" % "|".join(SIDES), id)
    return (found.group(1), int(found.group(2) or 1) - 1) if found else None


old = math.radians(kit.asset["rig"]["radialFolds"][0]["cutAngleDeg"]) / FACETS
probe = kit.solid("50", "panel-right-3")
print("check old profile reproduced:", max(abs(a - b) for row, ref in zip(profile(old, 2, "50"), probe["profile"]) for a, b in zip(row, ref)) < 2e-7 or "already retaut")
for fold in kit.asset["rig"]["radialFolds"]:
    fold["cutAngleDeg"] = round(math.degrees(STEP) * FACETS, 7)
count = 0
for scale in SCALES:
    for name in ("body", "thermal"):
        for s in kit.solids(scale, name):
            which = facet(s["id"])
            if which and "profile" in s:
                s["profile"] = profile(STEP, which[1], scale)
                count += 1
    for view in ("elevation", "front", "plan"):
        for s in kit.shapes(scale, view):
            which = facet(s["id"])
            if which and "points" in s and "solid" in s:
                s["points"] = [[x, z] for z, x in profile(STEP, which[1], scale)]
                count += 1
print("facets rewritten:", count)

for clip in kit.asset["clips"]:
    if clip["id"] == "wind-flap":
        for key in clip["keys"]:
            if key.get("bones"):
                key["bones"] = {"mast": {"angleDeg": -9.5}}
        clip["description"] = {"en": "A gust builds on the tilted canopy and sways the mast 1.5 degrees further in its tilt joint; the stretched cloth holds ribs and runner in their open position, then the wind releases.",
                               "de": "Eine Böe baut sich auf dem geneigten Schirm auf und drückt den Mast im Knickgelenk um weitere 1,5 Grad; das gespannte Tuch hält Streben und Schieber in der offenen Stellung, dann lässt der Wind nach."}
for scale in SCALES:
    kit.decide(scale, "Open, the cloth is taut: every gore is cut for the angle its two ribs enclose when open (43.33 degrees plus 0.06 of ease), so the six facets lie in one plane and only fold when the runner comes down.",
               "Offen ist das Tuch gespannt: jede Bahn ist auf den Winkel zugeschnitten, den ihre beiden Streben offen einschließen (43,33 Grad plus 0,06 Zugabe); die sechs Flächen liegen in einer Ebene und falten erst, wenn der Schieber herunterkommt.")
kit.finish(sys.argv)
