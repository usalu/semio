"""🪚 Dining table, second round pass 1: the telescopic slide gets its real section depth at every scale.

Hand-chosen numbers (metres). World height of the slide: top 0.688, underside 0.6245 (63.5 mm flat on edge, 8 mm thick,
in a 67.5 x 12 mm slotted C-rail of 2 mm sheet screwed to the inner face of each long apron).
Usage: python3 rework-dining-table-slide.py [--area <slide-rails areaM2>]
"""
import json
import re
import sys
from pathlib import Path

T = Path(__file__).resolve().parent
SOURCE = T / "volumes/furniture/dining-table.json"
CATALOG = Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/dining-table/🔣️.json")

RUNNER = [[-0.0635, 0.363], [-0.1125, 0.363], [-0.1125, 0.375], [-0.045, 0.375], [-0.045, 0.363], [-0.0515, 0.363],
          [-0.0515, 0.365], [-0.047, 0.365], [-0.047, 0.373], [-0.1105, 0.373], [-0.1105, 0.365], [-0.0635, 0.365]]
BAR = {"50": (0.0635, -0.02125, 0.008, 0.369), "100": (0.065, -0.023, 0.018, 0.366), "200": (0.07, -0.0225, 0.03, 0.355), "500": (0.066, -0.0225, 0.018, 0.366)}
LINK = {"50": ("shoe-", 0.0635, -0.07875), "100": ("slide-link-", 0.065, -0.0805), "200": ("slide-link-", 0.07, -0.08)}
MASS = {"frame": (12.75746, 0.5, -0.02351), "top-left": (24.312, 0.36532, -0.06976), "top-right": (24.312, 0.43468, -0.06976)}
TOTAL = 12.75746 + 2 * 24.312 + 11.85871
RAIL_2D = {"50": (-0.053, 0.0635), "100": (-0.0555, 0.065), "200": (-0.0575, 0.07), "500": (-0.0555, 0.066)}


def solids(volume):
    for name in ("body", "thermal", "structure"):
        yield from volume[name]


def edit_source():
    document = json.loads(SOURCE.read_text())
    for scale, volume in document["volumes"].items():
        height, centre, width, z = BAR[scale]
        for solid in solids(volume):
            identifier = solid["id"]
            if identifier in ("centre-bar", "slide") and solid.get("kind") == "box":
                solid["size"] = [1.0, height, width]
                solid["transform"]["translation"] = [0.5, centre, z]
                solid.pop("slender", None)
            if identifier.startswith("runner-") and solid.get("kind") == "extrude":
                sign = -1 if identifier.endswith("-r") else 1
                solid["profile"] = [[y, sign * zz] for y, zz in RUNNER]
            if scale in LINK and identifier.startswith(LINK[scale][0]) and solid.get("kind") == "box":
                solid["size"][1] = LINK[scale][1]
                solid["transform"]["translation"][1] = LINK[scale][2]
            if solid.get("part") == "mass" and solid.get("member") in MASS and solid.get("kind") == "ellipsoid":
                _, x, y = MASS[solid["member"]]
                solid["transform"]["translation"] = [x, y, 0.0]
    SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n")


def replace_once(text, pattern, replacement, count=1):
    result, found = re.subn(pattern, replacement, text, flags=re.S)
    if found != count:
        raise SystemExit(f"[DEBUG] pattern {pattern!r} matched {found} times, expected {count}")
    return result


def edit_catalog(area):
    text = CATALOG.read_text()
    for bone, (kg, x, y) in MASS.items():
        text = replace_once(text, r'("id": "' + bone + r'",.*?"mass": \{\s*"kg": )[-0-9.]+(,\s*"comX": )[-0-9.]+(,\s*"comY": )[-0-9.]+',
                            lambda m, kg=kg, x=x, y=y: f"{m.group(1)}{kg}{m.group(2)}{x}{m.group(3)}{y}")
    text = replace_once(text, r'"massKg": [0-9.]+', f'"massKg": {round(TOTAL, 5)}')
    order = iter(["500", "200", "100", "50"])

    def rail(match):
        y, h = RAIL_2D[next(order)]
        return f"{match.group(1)}{y}{match.group(2)}{h}"

    text = replace_once(text, r'("id": "(?:rail|slide-bar)",\s*"bone": "frame",[^{}]*?"y": )[-0-9.]+(,\s*"w": 1(?:\.0)?,\s*"h": )[0-9.]+', rail, 4)
    if area:
        text = replace_once(text, r'("id": "slide-rails",.*?"areaM2": )[0-9.]+', lambda m: m.group(1) + area)
    CATALOG.write_text(text)
    document = json.loads(text)
    return document


if __name__ == "__main__":
    edit_source()
    edit_catalog(sys.argv[sys.argv.index("--area") + 1] if "--area" in sys.argv else None)
    print("[DEBUG] slide rebuilt in source and catalog masses updated")
