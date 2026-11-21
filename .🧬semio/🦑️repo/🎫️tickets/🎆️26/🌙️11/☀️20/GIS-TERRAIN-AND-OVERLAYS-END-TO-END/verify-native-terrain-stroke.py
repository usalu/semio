"""🧭 Check the portable terrain brush fixture against GDAL and SciPy."""

import json
import sys
from pathlib import Path

import numpy as np
from rasterio.warp import transform
from scipy.spatial import cKDTree


law = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
west, south, east, north = law["bounds"]
longitude, latitude = np.meshgrid(np.linspace(west, east, 33), np.linspace(south, north, 33))
x, y = transform("EPSG:4326", "EPSG:3857", longitude.ravel(), latitude.ravel())
oracle = cKDTree(np.column_stack([x, y]))
bounds = np.array([[min(x), min(y)], [max(x), max(y)]])
center = np.array(law["viewport"]) / 2
spacing = max(1, (law["radius"] + 1) // 2)
observations = {}
for case in law["cases"]:
    world = (np.array(case["points"]) - center) * np.array([1, -1])
    if not ((bounds[0] <= world) & (world <= bounds[1])).all():
        assert "start" not in case, case["id"]
        observations[case["id"]] = "outside the authored raster"
        continue
    _, indices = oracle.query(world)
    rows, columns = np.unravel_index(indices, (33, 33))
    cells = np.column_stack([columns, rows])
    cells = cells[np.r_[True, (np.diff(cells, axis=0) != 0).any(axis=1)]]
    steps = np.ceil(np.abs(np.diff(cells, axis=0)).max(axis=1) / spacing).sum() + 1
    if steps > 64:
        assert "start" not in case, case["id"]
        observations[case["id"]] = f"{int(steps)} stamps"
        continue
    if "beforeRelease" in case:
        assert "start" not in case, case["id"]
        continue
    assert cells[0].tolist() == case["start"], case["id"]
    assert cells.tolist() == case.get("stroke", [case["start"]]), case["id"]
    observations[case["id"]] = cells.tolist()

print(json.dumps(observations))
