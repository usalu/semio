"""🧮 Recompute the native FEM 3D demonstration reference with NumPy."""
from pathlib import Path

import numpy as np


ROOT = Path(__file__).resolve().parents[7]
ARTIFACT = ROOT / "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"


def frame_equilibrium(demo):
    nodes, members = {}, []
    for line in demo.splitlines():
        fields = line.split()
        if len(fields) == 4 and fields[0].startswith("n"):
            nodes[fields[0]] = np.array(fields[1:], dtype=float)
        elif fields and fields[0] == "frame":
            member = dict(field.split("=", 1) for field in fields[1:])
            members.append((member["start"], member["end"]))

    elastic, shear = 210e9, 80.77e9
    area, iy, iz, torsion = 0.00538, 0.0000369, 0.0000133, 0.0000006
    gravity = np.array([0.0, 0.0, -9.81])
    density = 7850
    stiffness = np.zeros((len(nodes) * 6, len(nodes) * 6))
    forces = np.zeros(len(stiffness))
    dofs = {name: np.arange(i * 6, (i + 1) * 6) for i, name in enumerate(nodes)}
    stations, weights = np.polynomial.legendre.leggauss(3)
    for start, end in members:
        offset = nodes[end] - nodes[start]
        length = np.linalg.norm(offset)
        along = offset / length
        up = [1.0, 0.0, 0.0] if along[2] else [0.0, 0.0, 1.0]
        side = np.cross(up, along)
        basis = np.array([along, side, np.cross(along, side)])
        into_local = np.kron(np.eye(4), basis)

        local_stiffness = np.zeros((12, 12))
        local_forces = np.zeros(12)
        for station, weight in zip((1 + stations) / 2, weights * length / 2):
            s = station
            linear = np.array([1 - s, s])
            derivative = np.array([-1, 1]) / length
            hermite = np.array([1 - 3*s**2 + 2*s**3, length*(s - 2*s**2 + s**3),
                                3*s**2 - 2*s**3, length*(-s**2 + s**3)])
            curvature = np.array([-6 + 12*s, length*(-4 + 6*s),
                                  6 - 12*s, length*(-2 + 6*s)]) / length**2
            strain = np.zeros((4, 12))
            displacement = np.zeros((3, 12))
            strain[0, [0, 6]] = derivative
            strain[1, [3, 9]] = derivative
            strain[2, [1, 5, 7, 11]] = curvature
            strain[3, [2, 4, 8, 10]] = curvature * [1, -1, 1, -1]
            displacement[0, [0, 6]] = linear
            displacement[1, [1, 5, 7, 11]] = hermite
            displacement[2, [2, 4, 8, 10]] = hermite * [1, -1, 1, -1]
            local_stiffness += weight * strain.T @ np.diag(
                [elastic * area, shear * torsion, elastic * iz, elastic * iy]
            ) @ strain
            local_forces += weight * displacement.T @ (basis @ gravity) * density * area

        indices = np.r_[dofs[start], dofs[end]]
        stiffness[np.ix_(indices, indices)] += into_local.T @ local_stiffness @ into_local
        forces[indices] += into_local.T @ local_forces

    free = np.concatenate([indices for name, indices in dofs.items() if not name.endswith("_g")])
    displacement = np.zeros(len(stiffness))
    displacement[free] = np.linalg.solve(stiffness[np.ix_(free, free)], forces[free])
    reactions = stiffness @ displacement - forces
    return {
        "storeyVerticalDisplacements": [displacement[dofs[name][2]].item() for name in ("n00_l1", "n00_l2")],
        "rootVerticalReaction": reactions[dofs["n00_g"][2]].item(),
        "firstStoreyCornerLateralDisplacement": displacement[dofs["n00_l1"][:2]].tolist(),
    }


def slab_equilibrium(footprint, triangles):
    footprint = np.asarray(footprint, dtype=float)
    count = len(footprint)
    points = np.c_[np.tile(footprint, (2, 1)), np.repeat([0.0, 0.5], count)]
    cells = []
    for triangle in triangles:
        a, b, c = sorted(triangle)
        cells.extend([
            [a, b, c, c + count],
            [a, b, b + count, c + count],
            [a, a + count, b + count, c + count],
        ])
    elastic, poisson, density = 33e9, 0.2, 2400
    shear = elastic / (2 * (1 + poisson))
    lam = 2 * shear * poisson / (1 - 2 * poisson)
    constitutive = 2 * shear * np.eye(9) + lam * np.outer(np.eye(3).ravel(), np.eye(3).ravel())
    stiffness = np.zeros((len(points) * 3, len(points) * 3))
    forces = np.zeros(len(stiffness))
    recovery = []
    for cell in cells:
        coordinates = points[cell]
        interpolation = np.c_[np.ones(4), coordinates]
        gradients = np.linalg.inv(interpolation)[1:, :].T
        volume = abs(np.linalg.det(interpolation)) / 6
        strain = np.zeros((9, 12))
        for vertex, gradient in enumerate(gradients):
            derivatives = np.einsum("ij,k->ijk", np.eye(3), gradient)
            strain[:, vertex*3:vertex*3+3] = (
                (derivatives + derivatives.swapaxes(0, 2)) / 2
            ).transpose(0, 2, 1).reshape(9, 3)
        indices = (3 * np.asarray(cell)[:, None] + np.arange(3)).ravel()
        stiffness[np.ix_(indices, indices)] += volume * strain.T @ constitutive @ strain
        forces[indices[2::3]] -= density * 9.81 * volume / 4
        recovery.append((cell, indices, strain))
    for triangle in triangles:
        a, b, c = footprint[triangle]
        projected_area = abs(np.linalg.det([b - a, c - a])) / 2
        forces[3 * (np.asarray(triangle) + count) + 2] -= 800 * projected_area / 3
    corners = [np.flatnonzero(np.all(footprint == point, axis=1))[0]
               for point in ([10, 0], [12, 0], [12, 2], [10, 2])]
    fixed = (3 * np.asarray(corners)[:, None] + np.arange(3)).ravel()
    free = np.setdiff1d(np.arange(len(stiffness)), fixed)
    displacement = np.zeros(len(stiffness))
    displacement[free] = np.linalg.solve(stiffness[np.ix_(free, free)], forces[free])
    reactions = stiffness @ displacement - forces
    stresses = []
    for cell, indices, strain in recovery:
        if corners[0] in cell:
            stress = (constitutive @ strain @ displacement[indices]).reshape(3, 3)
            deviator = stress - np.trace(stress) / 3 * np.eye(3)
            stresses.append(np.sqrt(1.5 * np.sum(deviator**2)))
    return {
        "verticalReactionSum": reactions[fixed[2::3]].sum().item(),
        "cornerVonMisesStress": np.mean(stresses).item(),
    }





if __name__ == "__main__":
    import json

    demo = next(ARTIFACT.glob("**/🖼️assets/🎬️demo/🗣️.dsl.semio"))
    footprint = [[10 + 2*i/3, 2*j/3] for i in range(4) for j in range(4)]
    triangles = [
        [0, 1, 5], [0, 4, 5], [1, 2, 6], [1, 5, 6], [2, 3, 6], [3, 6, 7],
        [4, 5, 8], [5, 6, 9], [5, 8, 9], [6, 7, 11], [6, 9, 10], [6, 10, 11],
        [8, 9, 13], [8, 12, 13], [9, 10, 14], [9, 13, 14], [10, 11, 14], [11, 14, 15],
    ]
    print(json.dumps({
        "frame": frame_equilibrium(demo.read_text(encoding="utf-8")),
        "slab": slab_equilibrium(footprint, triangles),
    }, indent=2))



