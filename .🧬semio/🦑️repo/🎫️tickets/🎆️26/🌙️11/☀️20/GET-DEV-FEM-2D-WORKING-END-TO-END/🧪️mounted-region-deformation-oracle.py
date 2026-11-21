"""🧭️ Verify the retained FEM 2D deformation edges independently with NumPy."""

import json
from pathlib import Path
import sys

import numpy as np


fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
point_indices = {point_id: index for index, (point_id, _, _) in enumerate(fixture["points"])}
positions = np.array([position for _, position, _ in fixture["points"]])
published = np.array([visible for _, _, visible in fixture["points"]])
triangles = np.array([[point_indices[point_id] for point_id in row] for row in fixture["triangles"]])
pairs = np.stack((triangles, np.roll(triangles, -1, axis=1)), axis=-1).reshape(-1, 2)
pairs = np.sort(pairs[published[pairs].all(axis=1)], axis=1)
_, first_occurrences = np.unique(pairs, axis=0, return_index=True)
expected = np.array(fixture["edges"])

np.testing.assert_array_equal(positions[pairs[np.sort(first_occurrences)]], expected)
print("[DEBUG] NumPy reproduced the three ordered deformation edges")
