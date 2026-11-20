"""🏘️ Integrate the shipped House solids and solve the dead-load system with NumPy."""
import argparse
import os
from pathlib import Path
import shutil
import re
import subprocess
import numpy as np


TICKET = Path(__file__).resolve().parent
ARTIFACT = TICKET.parents[6] / "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
MATERIALS = {
    "concrete": (33e9, 0.2, 2400),
    "masonry": (5e9, 0.25, 1800),
    "timber": (11e9, 0.3, 420),
}


def footprint_meshes(shapes, build_dir):
    generated = TICKET / "🗑️generated"
    libraries = generated / "rdeps"
    libraries.mkdir(parents=True, exist_ok=True)
    if build_dir is not None:
        for artifact in (build_dir / "debug" / "build").glob("*/*/out/*"):
            if artifact.suffix not in {".rlib", ".rmeta", ".dll", ".so", ".dylib"}:
                continue
            link = libraries / artifact.name
            if link.exists():
                continue
            try:
                os.link(artifact, link)
            except OSError:
                shutil.copy2(artifact, link)
    mesh = next(path for path in libraries.glob("libsemio_s_fem_mesh-*.rmeta")
                if path.with_suffix(".rlib").is_file())

    def rust_points(coords):
        return "vec!" + repr(coords)

    specifications = ",\n".join(
        f"({rust_points(outline)}, vec![{', '.join(map(rust_points, holes))}], {size})"
        for _, outline, holes, size, *_ in shapes
    )
    source = """
fn main() {
    use semio_s_fem_mesh::{triangulate, PlanarDomain, MeshOpts};
    for (outer, holes, max_edge) in [SHAPES] {
        let mesh = triangulate(&PlanarDomain {outer,holes},&MeshOpts {max_edge,min_angle_deg:20.0}).unwrap();
        println!(r#"{{"points": {:?}, "triangles": {:?}}}"#, mesh.points, mesh.tris);
    }
}
""".replace("SHAPES", specifications)
    output = generated / "house-footprints.exe"
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                    "--edition=2024", "-L", f"dependency={libraries}",
                    "--extern", f"semio_s_fem_mesh={mesh}",
                    "--extern", f"semio_s_fem_mesh={mesh.with_suffix('.rlib')}",
                    "-o", str(output), "-"], input=source.encode(), check=True)
    import json
    return [json.loads(row) for row in subprocess.check_output([str(output)], encoding="utf-8").splitlines()]


def house_mesh(text, build_dir):
    def xy_pairs(sequence):
        return [[float(v) for v in pair.split(",")] for pair in sequence.split()]

    shapes = []
    solid_rows = text.split("solids [", 1)[1].split("\n}", 1)[0].splitlines()[1:]
    for line in solid_rows:
        row = re.fullmatch(r'\s*(\S+) "[^"]+" \[ (.*?) \] \[ (.*?)\] (\S+) (\S+) (\d+) (\S+) (\S+) (\w)', line)
        name, outline, holes, base, height, layers, size, material, axis = row.groups()
        shapes.append((name, xy_pairs(outline), list(map(xy_pairs, re.findall(r"\[ (.*?) \]", holes))),
                       float(size), float(base), float(height), int(layers), material, axis))
    vertices, indices, tets, materials, top_triangles = [], {}, [], [], []

    def index(position):
        key = tuple(np.rint(np.asarray(position) * 1e6).astype(np.int64))
        if key not in indices:
            indices[key] = len(vertices)
            vertices.append(position)
        return indices[key]

    for x in range(7):
        for y in range(9):
            index([x, y, -0.5])
    for shape, mesh in zip(shapes, footprint_meshes(shapes, build_dir), strict=True):
        name, _, _, _, base, height, layers, material, axis = shape
        placement = {
            "x": lambda u, v, z: [z, u, v],
            "y": lambda u, v, z: [u, z, v],
            "z": lambda u, v, z: [u, v, z],
        }[axis]
        points = [[index(placement(u, v, base + height * layer / layers)) for u, v in mesh["points"]]
                  for layer in range(layers + 1)]
        print(f"{name}: {len(mesh['points'])} footprint vertices, {len(mesh['triangles'])} triangles, {layers} layers, {len(vertices)} merged nodes")
        for triangle in mesh["triangles"]:
            a, b, c = sorted(triangle)
            for lo, hi in zip(points, points[1:]):
                tets.extend([[lo[a], lo[b], lo[c], hi[c]], [lo[a], lo[b], hi[b], hi[c]],
                             [lo[a], hi[a], hi[b], hi[c]]])
                materials.extend([MATERIALS[material]] * 3)
            if name == "attic_slab":
                top_triangles.append([points[-1][i] for i in triangle])
    return np.array(vertices), np.array(tets), np.array(materials), np.array(top_triangles)


def equilibrium(vertices, tets, materials, top):
    coordinates = vertices[tets]
    interpolation = np.concatenate((np.ones((*tets.shape, 1)), coordinates), axis=-1)
    gradients = np.linalg.inv(interpolation)[:, 1:, :].transpose(0, 2, 1)
    volume = np.abs(np.linalg.det(interpolation)) / 6
    E, nu, density = materials.T
    G = E / (2 * (1 + nu))
    lam = 2 * G * nu / (1 - 2 * nu)
    D = 2 * G[:, None, None] * np.eye(9) + lam[:, None, None] * np.outer(np.eye(3).ravel(), np.eye(3).ravel())
    derivative = np.einsum("ab,mvk->mavbk", np.eye(3), gradients)
    strain = ((derivative + derivative.transpose(0, 4, 2, 3, 1)) / 2).transpose(0, 1, 4, 2, 3).reshape(-1, 9, 12)
    local = volume[:, None, None] * (strain.transpose(0, 2, 1) @ D @ strain)
    dofs = (3 * tets[..., None] + np.arange(3)).reshape(-1, 12)
    n = 3 * len(vertices)
    keys, inverse = np.unique((dofs[..., None] * n + dofs[:, None, :]).ravel(), return_inverse=True)
    values = np.bincount(inverse, local.ravel())
    rows, columns = np.divmod(keys, n)
    fixed = 63 * 3
    force = np.bincount(dofs[:, 2::3].ravel(), np.repeat(-density * 9.81 * volume / 4, 4), minlength=n)
    a, b, c = vertices[top].transpose(1, 0, 2)
    pressure = -1000 * np.abs(np.cross(b - a, c - a)[:, 2]) / 6
    force += np.bincount((3 * top + 2).ravel(), np.repeat(pressure, 3), minlength=n)
    free = (rows >= fixed) & (columns >= fixed)
    row, column, data = rows[free] - fixed, columns[free] - fixed, values[free]
    b = force[fixed:]
    diagonal = np.bincount(row[row == column], data[row == column], minlength=len(b))

    def multiply(x):
        return np.bincount(row, data * x[column], minlength=len(x))

    x = np.zeros_like(b)
    r = b.copy()
    direction = r / diagonal
    rho = r @ direction
    scale = np.linalg.norm(b)
    for iteration in range(1, 8193):
        ap = multiply(direction)
        alpha = rho / (direction @ ap)
        x += alpha * direction
        r -= alpha * ap
        residual = np.linalg.norm(r) / scale
        if iteration == 512:
            print(f"relative residual at 512 iterations: {residual:.9g}")
        if residual <= 1e-8:
            print(f"converged after {iteration} iterations, residual {residual:.9g}")
            break
        z = r / diagonal
        next_rho = r @ z
        direction = z + next_rho / rho * direction
        rho = next_rho
    else:
        raise RuntimeError("House CG failed to converge")
    reaction = np.bincount(rows, values * np.r_[np.zeros(fixed), x][columns], minlength=n) - force
    return reaction[2:fixed:3].sum()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--build-dir", type=Path)
    args = parser.parse_args()
    dsl = next(ARTIFACT.glob("**/🖼️assets/🏠️house/*.semio")).read_text(encoding="utf-8")
    mesh = house_mesh(dsl, args.build_dir)
    print(f"House has {len(mesh[0])} analysis vertices and {len(mesh[1])} tetrahedra")
    print(f"House vertical reaction: {equilibrium(*mesh):.9f} N")


