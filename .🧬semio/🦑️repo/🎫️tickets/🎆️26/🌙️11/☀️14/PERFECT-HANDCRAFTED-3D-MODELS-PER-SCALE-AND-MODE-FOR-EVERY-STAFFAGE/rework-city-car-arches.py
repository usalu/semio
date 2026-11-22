"""🛞 City car 1:200: the wheel arches become six 30 degree chords of the 0.365 m arch circle (centre 0.30 m above the ground on the axle) instead of four 45 degree chords, whose sagitta of 28 mm was one whole feature; run once after rework-city-car-bay.py."""
from rework_09_lib import Source

ROT = [0.70711, 0, 0.70711, 0]
src = Source("vehicles/city-car")


def station(x, y, z, rx, ry, exponent):
    return {"at": [x, y, z], "rx": rx, "ry": ry, "exponent": exponent, "rotation": ROT}


for side, sign in (("l", 1), ("r", -1)):
    wall = src.get(200, "body", "side-wall-" + side)
    keep = [s for s in wall["sections"] if s["at"][0] not in (-1.55809, -1.04191) and abs(abs(s["at"][0] + 1.3) - 0.258) > 0.002]
    arch = [station(-1.6161, 0.74125, sign * 0.80134, 0.03, 0.25875, 8), station(-1.4825, 0.80805, sign * 0.805, 0.03, 0.19195, 8), station(-1.1175, 0.80805, sign * 0.81, 0.03, 0.19195, 8), station(-0.9839, 0.74125, sign * 0.81, 0.03, 0.25875, 8)]
    wall["sections"] = sorted(keep + arch, key=lambda s: s["at"][0])
    assert len(wall["sections"]) == 16, len(wall["sections"])
    wing = src.get(200, "body", "wing-" + side)
    keep = [s for s in wing["sections"] if abs(s["at"][0] - 1.408) > 0.002]
    arch = [station(1.3325, 0.75726, sign * 0.6925, 0.14, 0.14116, 6), station(1.4661, 0.67796, sign * 0.68701, 0.14, 0.19546, 6)]
    wing["sections"] = sorted(keep + arch, key=lambda s: s["at"][0])
    assert len(wing["sections"]) == 6, len(wing["sections"])
scuttle = src.get(200, "body", "scuttle")
keep = [s for s in scuttle["sections"] if abs(s["at"][0] - 0.892) > 0.002]
scuttle["sections"] = sorted(keep + [station(0.8339, 0.74125, 0, 0.84, 0.25875, 12), station(0.9675, 0.79141, 0, 0.84, 0.17531, 12)], key=lambda s: s["at"][0])
assert len(scuttle["sections"]) == 7, len(scuttle["sections"])
src.decide(200, "The wheel arches are six chords of 30 degrees on the arch circle of 0.365 m, so the largest gap between chord and circle is 12 mm, under half a feature; four chords left a whole feature.", "Die Radläufe sind sechs Sehnen von 30 Grad auf dem Radlaufkreis von 0,365 m, sodass die größte Abweichung zwischen Sehne und Kreis 12 mm beträgt, unter einem halben Merkmal; vier Sehnen ließen ein ganzes Merkmal stehen.")
src.save()
