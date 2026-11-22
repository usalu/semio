"""🪜 Spiral stair 1:500: the infill of 52 rods (26 mm clear at the 75 mm feature) is one guard panel per tread under the helical rail and one under each landing rail; the thermal list keeps the 52 real rods inside the panels."""
import json
import math

PATH = "volumes/components/spiral-stair.json"
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/spiral-stair/🔣️.json"
RISE = 0.18667
STEP = math.radians(22.5)
R_PANEL = 0.7525
HALF_T = 0.0375
HALF_CHORD = 0.155
RAIL_MID = 1.0
SINK = 0.02
NEWEL_Z = -0.0991
LANDING = 2.8
LANDING_RAIL = 3.70667
HALF_TURN = [0, round(-math.sin(STEP / 4), 5), 0, round(math.cos(STEP / 4), 5)]
SECOND_TURN = [0, round(-math.sin(3 * STEP / 4), 5), 0, round(math.cos(3 * STEP / 4), 5)]
BASE = {"bone": "balustrade", "material": "steel-bright", "role": "detail", "tone": 0.35}


def rail(z):
    return round(RISE + RAIL_MID + RISE * z / (2 * R_PANEL * math.tan(STEP / 2)), 5)


def tread_panel(name, z0, extra):
    profile = [[RISE - SINK, z0], [RISE - SINK, HALF_CHORD], [rail(HALF_CHORD), HALF_CHORD], [rail(z0), z0]]
    return {"id": name, "kind": "extrude", **BASE, "profile": profile, "plane": "yz", "from": R_PANEL - HALF_T, "to": R_PANEL + HALF_T, "transform": {"rotation": HALF_TURN}, **extra}


def landing_panel(name, profile):
    return {"id": name, "kind": "extrude", **BASE, "profile": profile, "plane": "zx", "from": LANDING - SINK, "to": LANDING_RAIL}


PANELS = [
    tread_panel("infill-start", NEWEL_Z, {}),
    tread_panel("infill", -HALF_CHORD, {"transform": {"rotation": SECOND_TURN, "translation": [0, RISE, 0]}, "repeat": {"count": 13, "rotationAxis": [0, 1, 0], "rotationRad": -0.3927, "translation": [0, RISE, 0]}}),
    landing_panel("infill-outer", [[-0.5745, 0.5745], [-0.0265, 1.1225], [0.0265, 1.0695], [-0.5215, 0.5215]]),
    landing_panel("infill-inner", [[0.0125, 0.1009], [0.5038, 0.5922], [0.5568, 0.5392], [0.0655, 0.0479]]),
]

source = json.load(open(PATH))
scale = source["volumes"]["500"]
published = json.load(open(CATALOG))["asset"]["volumes"]["500"]
rods = [solid for solid in scale["thermal"] + published["thermal"] if solid["id"].startswith("th-baluster-")][:5]
assert sum(solid.get("repeat", {}).get("count", 1) for solid in rods) == 52, "the catalog no longer holds the 52 real rods"


def swapped(solids, prefix, new):
    old = (prefix, prefix.replace("baluster-", "infill"))
    at = next(i for i, solid in enumerate(solids) if solid["id"].startswith(old))
    kept = [solid for solid in solids if not solid["id"].startswith(old)]
    return kept[:at] + new + kept[at:]


scale["body"] = swapped(scale["body"], "baluster-", PANELS)
scale["thermal"] = swapped(scale["thermal"], "th-baluster-", rods)
scale["structure"] = swapped(scale["structure"], "st-baluster-", [{"id": "st-" + panel["id"], "of": panel["id"], "member": "balustrade", "part": "member"} for panel in PANELS])
json.dump(source, open(PATH, "w"), ensure_ascii=False, indent=1)
print("[DEBUG] 1:500", len(scale["body"]), len(scale["thermal"]), len(scale["structure"]))
