#!/usr/bin/env python3
"""🧰️ Independent oracle of the projection-kit geometry: yaw rotation, the right-handed member frame and the base-centre box anchor from the textbook formulas, cross-checked against numpy when it is installed. Writes one JSON answer. Usage: python3 -I 🐍️.py"""
import json
import math


def rotate_yaw(d, yaw):
    c, s = math.cos(yaw), math.sin(yaw)
    return [d[0] * c - d[1] * s, d[0] * s + d[1] * c, d[2]]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = math.sqrt(sum(v * v for v in a))
    return [v / n for v in a]


def member_frame(axis):
    x = unit(axis)
    ref = [1.0, 0.0, 0.0] if math.sqrt(sum(v * v for v in cross(x, [0, 0, 1]))) < 1e-9 else [0.0, 0.0, 1.0]
    y = unit(cross(ref, x))
    return {"x": x, "y": y, "z": cross(x, y)}


def box_centre(corner, x, y, size):
    return [corner[i] + size[0] / 2 * x[i] + size[1] / 2 * y[i] for i in range(3)]


def interp(stations, t):
    xs, ys = [a for a, _ in stations], [v for _, v in stations]
    if t <= xs[0]:
        return ys[0]
    if t >= xs[-1]:
        return ys[-1]
    for (a, va), (b, vb) in zip(stations, stations[1:]):
        if a <= t <= b:
            return va + (vb - va) * (t - a) / (b - a)


def segments(stations, n):
    out = []
    for i in range(n):
        a, b = i / n, (i + 1) / n
        out.append(max([interp(stations, a), interp(stations, b)] + [v for x, v in stations if a < x < b]))
    return out


def idw(stations, p):
    num = den = 0.0
    for at, v in stations:
        d2 = sum((x - y) ** 2 for x, y in zip(at, p))
        if d2 < 1e-24:
            return v
        num += v / d2
        den += 1 / d2
    return num / den


MEMBER = [(0.0, 0.2), (0.25, 0.6), (0.5, 0.9), (1.0, 0.4)]
PLATE = [((0.0, 0.0), 0.2), ((1.0, 0.0), 0.6), ((0.0, 1.0), 0.4)]
PARAMS = [-0.1, 0.0, 0.125, 0.5, 0.75, 1.0, 1.2]

answer = {
    "field_along": [interp(MEMBER, t) for t in PARAMS],
    "segments": segments(MEMBER, 4),
    "idw": [idw(PLATE, p) for p in [(0.0, 0.0), (0.5, 0.5), (1.0, 1.0)]],
    "yaw_quarter": rotate_yaw([1.0, 0.0, 0.0], math.pi / 2),
    "frame_horizontal": member_frame([2.0, 0.0, 0.0]),
    "frame_vertical": member_frame([0.0, 0.0, 3.0]),
    "box_centre": box_centre([1.0, 2.0, 0.5], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [4.0, 2.0, 1.0]),
}
try:
    import numpy as np

    assert np.allclose(np.cross([0, 0, 1], [1, 0, 0]), answer["frame_horizontal"]["y"])
    assert np.allclose(answer["field_along"], np.interp(PARAMS, [a for a, _ in MEMBER], [v for _, v in MEMBER]))
    assert np.allclose(np.cross([1, 0, 0], [0, 0, 1]), [0, -1, 0])
except ImportError:
    pass
print(json.dumps(answer, sort_keys=True))
