#!/usr/bin/env python3
"""🛝️ Writes the authored snapshot of the `ramp-runs` inference fixture of `s.bim.model@1` (the ONLY input of an inference oracle).

The expected table is written by the oracle `🛝️infer-bim-1-ramps/🐍️.py`; never by hand.

    python r11-w09-ramps-fixtures.py <path to the subset 🧫️fixtures/💡️inferences>
"""

import json
import math
import sys
from pathlib import Path

COLLECTIONS = ["materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "ramps", "spaces", "properties", "classifications"]


def snapshot(name, **parts):
    return {"schema": "s.bim.model@1", "project": {"name": name, "description": "", "author": "", "organization": "", "phase_names": []}, **{key: parts.get(key, {}) for key in COLLECTIONS}}


def vertex(x, y, bulge=0.0):
    return {"point": {"x": float(x), "y": float(y)}, "bulge": float(bulge)}


def ramp(storey, path, rise, **over):
    top = over.pop("top", {"Unconnected": {"height": float(rise)}})
    return {
        "storey": storey,
        "path": [vertex(*point) for point in path],
        "width": 1.2,
        "landing_start": 1.5,
        "landing_end": 1.5,
        "landing_turn": 1.5,
        "max_slope": 1.0 / 12.0,
        "thickness": 0.2,
        "material": "m-concrete",
        "base_offset": 0.0,
        "top": top,
        "railing_left": False,
        "railing_right": False,
        "name": "Ramp",
        **over,
    }


QUARTER = math.tan(math.pi / 8.0)

RAMPS = {
    "r-straight": ramp("st-ground", [(0, 0), (10, 0)], 0.5, railing_left=True),
    "r-at-limit": ramp("st-ground", [(0, 3), (10, 3)], 7.0 / 12.0),
    "r-steep": ramp("st-ground", [(0, 6), (10, 6)], 0.7),
    "r-bent": ramp("st-ground", [(0, 9), (6, 9), (6, 14)], 0.6),
    "r-zigzag": ramp("st-ground", [(12, 0), (17, 0), (17, 5), (22, 5)], 0.7),
    "r-smooth-join": ramp("st-ground", [(12, 9), (16, 9), (20, 9)], 0.4),
    "r-curved": ramp("st-ground", [(12, 12, QUARTER), (17, 17)], 0.3),
    "r-to-first": ramp("st-ground", [(30, 0), (70, 0)], 0.0, top={"Storey": {"storey": "st-first", "offset": 0.0}}),
    "r-to-first-short": ramp("st-ground", [(30, 3), (40, 3)], 0.0, top={"Storey": {"storey": "st-first", "offset": 0.0}}),
    "r-storey-top": ramp("st-first", [(30, 6), (70, 6)], 0.0, top={"StoreyTop": {"offset": 0.0}}),
    "r-down": ramp("st-ground", [(30, 9), (40, 9)], -0.5, base_offset=0.5),
    "r-no-run": ramp("st-ground", [(30, 12), (40, 12)], 0.3, landing_start=6.0, landing_end=6.0),
    "r-flat": ramp("st-ground", [(30, 15), (40, 15)], 0.0),
    "r-merged": ramp("st-ground", [(30, 18), (36, 18), (36, 23)], 0.0, landing_start=6.0, landing_turn=3.0, landing_end=20.0),
    "r-degenerate": ramp("st-ground", [(1, 1), (1, 1)], 0.3),
    "r-wide": ramp("st-first", [(50, 9), (60, 9)], 0.4, width=2.4, max_slope=0.1, railing_left=True, railing_right=True),
    "r-orphan": ramp("st-nowhere", [(0, 0), (10, 0)], 0.5),
}

STOREYS = {
    "st-base": {"building": "bldg-1", "name": "Basement", "level": -1, "height": 2.6},
    "st-ground": {"building": "bldg-1", "name": "Ground", "level": 0, "height": 3.0},
    "st-first": {"building": "bldg-1", "name": "First", "level": 1, "height": 2.8},
}

BUILDINGS = {"bldg-1": {"site": "site-1", "name": "House", "origin": {"x": 0.0, "y": 0.0}, "rotation": 0.0, "elevation": 0.0}}
SITES = {"site-1": {"name": "Plot", "latitude": 47.0, "longitude": 8.0, "elevation": 100.0, "true_north": 0.0, "boundary": []}}
MATERIALS = {"m-concrete": {"name": "Concrete", "category": "Concrete", "color": {"r": 0.6, "g": 0.6, "b": 0.6}, "density": 2400.0, "conductivity": 1.8, "specific_heat": 1000.0}}


def main(arguments):
    target = Path(arguments[0]) / "🛝️ramp-runs" / "🏞️ramps" / "📸️snapshot" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    document = snapshot("Ramps", sites=SITES, buildings=BUILDINGS, storeys=STOREYS, materials=MATERIALS, ramps=RAMPS)
    target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("wrote %s (%d ramps)" % (target, len(RAMPS)))


if __name__ == "__main__":
    main(sys.argv[1:])
