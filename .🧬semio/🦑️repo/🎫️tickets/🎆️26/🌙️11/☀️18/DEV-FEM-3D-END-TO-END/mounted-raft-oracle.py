import json
import sys
from pathlib import Path

import numpy as np


law = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
outline = np.asarray(law["outer"])
lower = np.min(outline, axis=0)
upper = np.max(outline, axis=0)
divisions = np.ceil((upper - lower) * np.sqrt(2) / law["maximumEdge"]).astype(int)
xs, ys = (np.linspace(lo, hi, count + 1) for lo, hi, count in zip(lower, upper, divisions))
vertices = np.stack(np.meshgrid(xs, ys, indexing="ij"), axis=-1)
origins = vertices[:-1, :-1]
horizontal = vertices[1:, :-1] - origins
vertical = vertices[:-1, 1:] - origins
diagonal = vertices[1:, 1:] - origins
first = np.linalg.det(np.stack([horizontal, diagonal], axis=-1))
second = np.linalg.det(np.stack([diagonal, vertical], axis=-1))
result = {
    "points": int(vertices.shape[0] * vertices.shape[1]),
    "triangles": int(first.size + second.size),
    "area": float(np.sum(first + second) / 2),
}
for key in ("points", "triangles"):
    assert result[key] == law["expected"][key], result
assert np.isclose(result["area"], law["expected"]["area"]), result
print(json.dumps(result))
