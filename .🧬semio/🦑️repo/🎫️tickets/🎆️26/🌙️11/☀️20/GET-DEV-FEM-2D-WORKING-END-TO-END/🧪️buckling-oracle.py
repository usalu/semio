"""🧮️ Verifies the compact Euler–Bernoulli pencil and continuum symmetry with NumPy."""
import json
from pathlib import Path

import numpy as np


def geometric_cell(left, right, stress):
    x, y = left
    u, v = right
    xx, xy, yy = stress
    return x * xx * u + x * xy * v + y * xy * u + y * yy * v


fixture = json.loads(Path(__file__).with_name("📐️buckling-roundoff.json").read_text(encoding="utf-8"))
a, b = fixture["gradients"]
stress = fixture["stress"]
forward = geometric_cell(a, b, stress)
transpose = geometric_cell(b, a, stress)
symmetry_tolerance = 8 * np.finfo(float).eps * max(abs(forward), abs(transpose))
assert abs(forward - transpose) > symmetry_tolerance
recovered = 0.5 * (forward + transpose)
gradients = np.asarray([a, b])
xx, xy, yy = stress
reference = gradients @ np.asarray([[xx, xy], [xy, yy]]) @ gradients.T
np.testing.assert_allclose(recovered, reference[0, 1], rtol=8 * np.finfo(float).eps)
print(f"[DEBUG] continuum transpose gap={abs(forward - transpose):.4e} tolerance={symmetry_tolerance:.4e}")

length, modulus, inertia, compression = 6.0, 210e9, 8.356e-5, 10000.0
elastic = np.asarray([[4, 2], [2, 4]]) * modulus * inertia / length
prestress = np.asarray([[4, -1], [-1, 4]]) * compression * length / 30
factor = np.linalg.cholesky(prestress)
left_reduced = np.linalg.solve(factor, elastic)
factors = np.linalg.eigvalsh(np.linalg.solve(factor, left_reduced.T).T)
expected = np.asarray([12, 60]) * modulus * inertia / (compression * length**2)
np.testing.assert_allclose(factors, expected, rtol=1e-13)
print(f"[DEBUG] compressed pinned beam load factors={factors.tolist()}")
