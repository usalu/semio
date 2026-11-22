"""📷️ Write an asymmetric camera fixture with LAPACK least-squares triangulation."""

import csv
import ctypes as c
import ctypes.util
import math
import pathlib
import random
import sys


def camera_point(world, angle, translation):
    """↪️ Rotate about Y, then translate into camera coordinates."""
    x, y, z = world
    ct, st = math.cos(angle), math.sin(angle)
    return (ct * x + st * z + translation[0], y + translation[1], -st * x + ct * z + translation[2])


def pixel(ray, model, linear, distortion):
    """🖼️ Apply the lens model and calibrated pixel map."""
    x, y = ray
    k1, k2, k3, d4, d5 = distortion
    radius = math.hypot(x, y)
    r2 = radius * radius
    if model == "brownConrady":
        scale = 1 + r2 * (k1 + r2 * (k2 + r2 * k3))
        x, y = x * scale + 2 * d4 * x * y + d5 * (r2 + 2 * x * x), y * scale + d4 * (r2 + 2 * y * y) + 2 * d5 * x * y
    elif model == "fisheye":
        theta = math.atan(radius)
        t2 = theta * theta
        scale = theta * (1 + t2 * (k1 + t2 * (k2 + t2 * (k3 + t2 * d4)))) / radius
        x, y = x * scale, y * scale
    fx, fy, cx, cy, skew = linear
    return (fx * x + skew * y + cx, fy * y + cy)


def triangulate(rays, cameras, lapack):
    """🔺 Solve the unnormalised multi-view ray equations using LAPACK DGELS."""
    rows, rhs = [], []
    for (u, v), (_, _, _, _, angle, (tx, ty, tz)) in zip(rays, cameras):
        ct, st = math.cos(angle), math.sin(angle)
        rows.extend(((ct + u * st, 0, st - u * ct), (v * st, 1, -v * ct)))
        rhs.extend((u * tz - tx, v * tz - ty))
    m, n, nrhs, lwork, info = map(c.c_int, (len(rows), 3, 1, 64, 0))
    a = (c.c_double * (m.value * n.value))(*(row[k] for k in range(n.value) for row in rows))
    b = (c.c_double * m.value)(*rhs)
    work = (c.c_double * lwork.value)()
    lapack.dgels_(c.c_char_p(b"N"), c.byref(m), c.byref(n), c.byref(nrhs), a, c.byref(m), b, c.byref(m), work, c.byref(lwork), c.byref(info), c.c_size_t(1))
    assert info.value == 0
    return tuple(b[:3])


def main(output):
    """🧪 Generate known poses, pixel observations and independent reconstructed points."""
    lapack = c.CDLL(ctypes.util.find_library("lapack") or "")
    cameras = (
        ("brownConrady", (1024, 768), (760, 920, 479, 415, 21), (-0.125, 0.03125, 0.0078125, 0.015625, -0.0078125), 0.0, (0.0, 0.0, 0.0)),
        ("fisheye", (640, 480), (365, 510, 287, 261, -17), (0.0625, -0.015625, 0.0078125, 0.00390625, 0), 0.12, (-1.0, 0.0, 0.0)),
        ("pinhole", (1200, 900), (980, 665, 673, 396, 33), (0, 0, 0, 0, 0), -0.08, (0.36, -0.41, 0.24)),
    )
    rand = random.Random(913)
    with pathlib.Path(output).open("w", newline="", encoding="utf-8") as file:
        writer = csv.writer(file, lineterminator="\n")
        writer.writerow(["# Camera, model, width, height, fx, fy, cx, cy, skew, distortion x5, Y rotation, XYZ translation"])
        for model, dimensions, linear, distortion, angle, translation in cameras:
            writer.writerow(("camera", model, *dimensions, *linear, *distortion, angle, *translation))
        writer.writerow(["# Point, LAPACK triangulated X Y Z, pixel pairs in camera order"])
        for _ in range(28):
            world = (rand.uniform(-0.9, 0.9), rand.uniform(-0.7, 0.7), rand.uniform(3.4, 5.7))
            local = [camera_point(world, angle, translation) for _, _, _, _, angle, translation in cameras]
            rays = [(x / z, y / z) for x, y, z in local]
            point = triangulate(rays, cameras, lapack)
            assert math.dist(world, point) < 1e-12
            pixels = [pixel(ray, model, linear, distortion) for ray, (model, _, linear, distortion, _, _) in zip(rays, cameras)]
            writer.writerow(("point", *(f"{value:.12f}" for value in (*point, *(v for pair in pixels for v in pair)))))


if __name__ == "__main__":
    main(sys.argv[1])
