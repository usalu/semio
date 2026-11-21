"""🎼️ Integrate the free tip's consistent mass independently with NumPy Gauss quadrature."""

import json
from pathlib import Path

import numpy as np


root = Path(__file__).resolve().parents[7]
fixture = root / (
    "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/"
    "🧪️tests/🧫️fixtures/🎼️secondary-matrix/🔣️.json"
)
law = json.loads(fixture.read_text(encoding="utf-8"))
length = law["length"]
points, weights = np.polynomial.legendre.leggauss(4)
station = (points + 1) / 2
shapes = np.zeros((len(station), 2, 3))
shapes[:, 0, 0] = station
shapes[:, 1, 1] = station**2 * (3 - 2 * station)
shapes[:, 1, 2] = station**2 * (station - 1) * length
matrix = law["density"] * law["area"] * length / 2 * np.einsum("k,kai,kaj->ij", weights, shapes, shapes)
np.testing.assert_allclose(matrix, law["expectedMass"], rtol=0, atol=law["tolerance"])
print(matrix.tolist())

stiffness = np.zeros((3, 3))
stiffness[0, 0] = law["youngsModulus"] * law["area"] / length
stiffness[1:, 1:] = law["youngsModulus"] * law["inertia"] / length**3 * np.array(
    [[12, -6 * length], [-6 * length, 4 * length**2]]
)
weight = law["compression"]
signed_geometric = np.zeros((3, 3))
signed_geometric[1:, 1:] = weight / (30 * length) * np.array(
    [[36, -3 * length], [-3 * length, 4 * length**2]]
)
def symmetric_eigenvalues(left, right):
    lower = np.linalg.cholesky(right)
    transformed = np.linalg.solve(lower, left)
    transformed = np.linalg.solve(lower, transformed.T).T
    return np.linalg.eigvalsh(transformed)

modal = np.sqrt(symmetric_eigenvalues(stiffness, matrix)[0]) / (2 * np.pi)
buckling = 1 / symmetric_eigenvalues(signed_geometric, stiffness)[-1]
np.testing.assert_allclose([modal, buckling], [law["modalHz"], law["bucklingFactor"]], rtol=1e-10)
print({"modalHz": float(modal), "bucklingFactor": float(buckling)})
