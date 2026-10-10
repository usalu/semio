#!/usr/bin/env python3
"""🧫️ Writes the BIM inputs of the energy export cases from the committed inference cases of `🌡️energy-envelope`: `room` is the box and `pair` the zoning model, each with the flat roof the originals lack (so every
surface has thermal data), `stack` is the cellar, ground floor and upper floor as committed.

    python make-fixtures.py <sibling inference fixtures> <energy export fixtures>
"""
import json
import os
import sys
from pathlib import Path


def roof_for(snapshot, storey):
    slab = next(iter(snapshot["slabs"].values()))
    xs = [vertex["point"]["x"] for vertex in slab["boundary"]]
    ys = [vertex["point"]["y"] for vertex in slab["boundary"]]
    low, high = (min(xs) - 0.2, min(ys) - 0.2), (max(xs) + 0.2, max(ys) + 0.2)
    ring = [(low[0], low[1]), (high[0], low[1]), (high[0], high[1]), (low[0], high[1])]
    return {"storey": storey, "roof_type": "rt", "footprint": [{"point": {"x": x, "y": y}, "bulge": 0} for x, y in ring], "shape": "Flat", "overhang": 0, "base_offset": 0, "phase": "New", "name": "Roof"}


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    os.replace(temporary, path)


def main(source, target):
    source, target = Path(source), Path(target)
    for case, original, needs_roof in (("🏠️room", "🏠️box", True), ("🏘️pair", "🏘️zoning", True), ("🧱️stack", "🧱️stack", False)):
        snapshot = json.loads((source / original / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
        if needs_roof:
            snapshot["roofs"] = {"rf": roof_for(snapshot, next(iter(snapshot["storeys"])))}
        write(target / case / "📸️snapshot" / "🔣️.json", snapshot)
        print("wrote", case)


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
