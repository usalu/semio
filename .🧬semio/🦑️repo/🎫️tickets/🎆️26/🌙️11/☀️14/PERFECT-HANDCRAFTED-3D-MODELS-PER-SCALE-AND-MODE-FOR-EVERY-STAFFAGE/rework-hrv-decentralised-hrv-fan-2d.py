"""📐 Hand-redrawn 2D shapes and rig lengths of appliances/decentralised-hrv-fan that the reworked 1:100 and 1:200 models invalidate (rework pass 1).

Candidate: python3 rework-hrv-decentralised-hrv-fan-2d.py            (writes 🗑️generated/r3-decentralised-hrv-fan/🔣️.json)
Catalog:   python3 rework-hrv-decentralised-hrv-fan-2d.py --write    (re-reads the catalog file immediately before writing)
"""
import importlib.util
import json
import math
import pathlib
import sys

HERE = pathlib.Path(__file__).parent
CATALOG = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/decentralised-hrv-fan/🔣️.json")
spec = importlib.util.spec_from_file_location("volumes_insert", HERE / "volumes-insert.py")
insert = importlib.util.module_from_spec(spec)
sys.argv, argv = [sys.argv[0]], sys.argv
try:
    spec.loader.exec_module(insert)
except SystemExit:
    pass
sys.argv = argv

text = CATALOG.read_text()
document = json.loads(text)
ROUND_TRIP = insert.dump(document, 0) + "\n" == text
print("round trip byte-identical:", ROUND_TRIP)
asset = document["asset"]

RADIAL_LENGTHS = {"plate": 0.13, "ring": 0.098, "core": 0.08}
for bone in asset["rig"]["bones"]:
    if bone["id"] in RADIAL_LENGTHS:
        bone["lengthM"] = RADIAL_LENGTHS[bone["id"]]


def shapes(scale, view):
    """🗂️ The shape list of a view."""
    return asset["representations"][scale]["geometry"][view]


def shape(scale, view, identifier):
    """🔎 One shape by id."""
    return next(item for item in shapes(scale, view) if item["id"] == identifier)


def remove(scale, view, identifier):
    """🗑️ Drop a shape that the model of this scale no longer has."""
    shapes(scale, view)[:] = [item for item in shapes(scale, view) if item["id"] != identifier]


def strut(angle_deg, width, inner, outer):
    """➖ Radial strut as a quad about the axis."""
    c, s = math.cos(math.radians(angle_deg)), math.sin(math.radians(angle_deg))
    half = width / 2
    corners = [(inner, -half), (outer, -half), (outer, half), (inner, half)]
    return [[round(u * c - v * s, 6), round(u * s + v * c, 6)] for u, v in corners]


def hexagons(pitch, across_flats):
    """⬡ Path of the 19 cells of the 1:100 ceramic: centre, ring of six, ring of twelve."""
    centres = [(0.0, 0.0)]
    centres += [(pitch * math.cos(math.radians(60 * k)), pitch * math.sin(math.radians(60 * k))) for k in range(6)]
    centres += [(2 * pitch * math.cos(math.radians(60 * k)), 2 * pitch * math.sin(math.radians(60 * k))) for k in range(6)]
    centres += [(math.sqrt(3) * pitch * math.cos(math.radians(30 + 60 * k)), math.sqrt(3) * pitch * math.sin(math.radians(30 + 60 * k))) for k in range(6)]
    radius = across_flats / math.sqrt(3)
    parts = []
    for cx, cy in centres:
        points = [(cx + radius * math.cos(math.radians(30 + 60 * k)), cy + radius * math.sin(math.radians(30 + 60 * k))) for k in range(6)]
        parts.append("M " + " L ".join(f"{x:.6f} {y:.6f}" for x, y in points) + " Z")
    return " ".join(parts)


# 1:100: struts of 15 mm from the 40 mm boss to the 176 mm throat, 19 cells of 25 mm at 31 mm pitch, 15 mm shaft, outer collar 15 mm long
for identifier, angle in (("strut-1", 90), ("strut-2", 210), ("strut-3", 330)):
    shape("100", "elevation", identifier)["points"] = strut(angle, 0.015, 0.02, 0.088)
shape("100", "elevation", "core-cells")["d"] = hexagons(0.031, 0.025)
for view, along, across in (("front", "x", "y"), ("plan", "y", "x")):
    rim = shape("100", view, "rim")
    rim[along], rim["w" if along == "x" else "h"] = 0.105, 0.015
    shaft = shape("100", view, "shaft-section")
    shaft[along], shaft["w" if along == "x" else "h"] = -0.0845, 0.0305
    shaft[across], shaft["h" if along == "x" else "w"] = -0.0075, 0.015

# 1:200: one bridge bar of 30 mm, the tray as one slab of 20 mm, the can instead of the 8 mm shaft
shape("200", "elevation", "strut-1")["points"] = [[0.015, -0.088], [0.015, 0.088], [-0.015, 0.088], [-0.015, -0.088]]
remove("200", "elevation", "strut-2")
remove("200", "elevation", "strut-3")
for view, along, across in (("front", "x", "y"), ("plan", "y", "x")):
    plate = shape("200", view, "plate")
    plate[along], plate["w" if along == "x" else "h"] = -0.12, 0.02
    remove("200", view, "tray-rim")
    can = shape("200", view, "shaft-section")
    can[along], can["w" if along == "x" else "h"] = -0.103, 0.03
    can[across], can["h" if along == "x" else "w"] = -0.028, 0.056

REPLACEMENTS = {
    "100": [
        ("19 channels of 30 mm", "19 channels of 25 mm at a pitch of 31 mm"), ("19 Kanälen von 30 mm", "19 Kanälen von 25 mm im Raster von 31 mm"),
        ("the 60 mm hub", "the 56 mm hub"), ("die Nabe von 60 mm", "die Nabe von 56 mm"),
        ("the bearing shaft", "the 15 mm shaft"), ("die Lagerwelle", "die Welle von 15 mm"),
    ],
    "200": [
        ("three broad radial blades and the 80 mm hub", "five radial blades and the 56 mm hub"), ("drei breite radiale Flügel und die Nabe von 80 mm", "fünf radiale Flügel und die Nabe von 56 mm"),
        ("five blades become three and the three struts one bridge bar", "the three struts become one bridge bar, tray floor and rim one slab, and shaft and bearing disappear in the can"),
        ("and the bearing shaft", "and the motor can"), ("und die Lagerwelle", "und den Motorbecher"),
        ("aus fünf Flügeln werden drei und aus den drei Streben ein Brückensteg", "aus den drei Streben wird ein Brückensteg, aus Wannenboden und Rand eine Platte, und Welle und Lager verschwinden im Becher"),
    ],
}
for scale, pairs in REPLACEMENTS.items():
    decisions = asset["representations"][scale]["decisions"]
    for old, new in pairs:
        hits = 0
        for key, entry in decisions.items():
            for language in entry:
                if old in entry[language]:
                    entry[language] = entry[language].replace(old, new)
                    hits += 1
        print(scale, "replaced" if hits else "NOT FOUND", repr(old))
print("1:200 merged de:", asset["representations"]["200"]["decisions"]["merged"]["de"])

output = insert.dump(document, 0) + "\n" if ROUND_TRIP else json.dumps(document, ensure_ascii=False, indent=2) + "\n"
if "--write" in sys.argv:
    if CATALOG.read_text() != text:
        sys.exit("catalog file changed while editing: run again")
    CATALOG.write_text(output)
    print("written", CATALOG)
else:
    target = HERE / "🗑️generated" / "r3-decentralised-hrv-fan" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(output)
    print("candidate", target)
