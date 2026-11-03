"""📈️ Third-party oracle of the closed-form moment line: numpy solves the slope-deflection (stiffness) equations of the same beams, independent of the three-moment solve of the Rust sources.

Reads one beam per line from stdin: `id kind clamped spans udl points xs` (`kind` is `pinned` or `cantilever`, `clamped` 0/1, comma separated lists, points as `span:at:force` or `-`) and prints `id.m<k>` (moment at the k-th abscissa), `id.s<k>` (moment over the k-th support, sagging positive) and `id.r<k>` (reaction) lines in N and m.
"""
import sys

import numpy as np


def parse(line):
    ident, kind, clamped, spans, udl, points, xs = line.split()
    nums = lambda text: np.array([float(v) for v in text.split(",")])
    loads = [[] for _ in spans.split(",")]
    if points != "-":
        for item in points.split(","):
            j, a, p = item.split(":")
            loads[int(j)].append((float(a), float(p)))
    return ident, kind, clamped == "1", nums(spans), nums(udl), loads, nums(xs)


def fixed_end(length, w, points):
    left = -w * length**2 / 12 - sum(p * a * (length - a) ** 2 / length**2 for a, p in points)
    right = w * length**2 / 12 + sum(p * a**2 * (length - a) / length**2 for a, p in points)
    return left, right


def solve(kind, clamped, spans, udl, loads):
    n = len(spans)
    if kind == "cantilever":
        length, w, points = spans[0], udl[0], loads[0]
        moment = -(w * length**2 / 2 + sum(p * a for a, p in points))
        return np.array([moment]), np.array([w * length + sum(p for _, p in points)])
    stiffness = np.zeros((n + 1, n + 1))
    force = np.zeros(n + 1)
    fem = [fixed_end(spans[j], udl[j], loads[j]) for j in range(n)]
    for j in range(n):
        k = 2.0 / spans[j]
        stiffness[j : j + 2, j : j + 2] += k * np.array([[2.0, 1.0], [1.0, 2.0]])
        force[j] += fem[j][0]
        force[j + 1] += fem[j][1]
    free = list(range(1 if clamped else 0, n + 1))
    theta = np.zeros(n + 1)
    theta[free] = np.linalg.solve(stiffness[np.ix_(free, free)], -force[free])
    left = np.array([2.0 / spans[j] * (2 * theta[j] + theta[j + 1]) + fem[j][0] for j in range(n)])
    right = np.array([-(2.0 / spans[j] * (2 * theta[j + 1] + theta[j]) + fem[j][1]) for j in range(n)])
    shear = np.array([(right[j] - left[j]) / spans[j] + udl[j] * spans[j] / 2 + sum(p * (spans[j] - a) / spans[j] for a, p in loads[j]) for j in range(n)])
    return left, shear


def main():
    for line in sys.stdin.read().splitlines():
        if not line.strip():
            continue
        ident, kind, clamped, spans, udl, loads, xs = parse(line)
        n = len(spans)
        start, shear = solve(kind, clamped, spans, udl, loads)
        offsets = np.concatenate([[0.0], np.cumsum(spans)])

        def local(j, x):
            pts = loads[j]
            return start[j] + shear[j] * x - udl[j] * x**2 / 2 - sum(p * max(x - a, 0.0) for a, p in pts)

        def end_shear(j):
            return shear[j] - udl[j] * spans[j] - sum(p for _, p in loads[j])

        for k, x in enumerate(xs):
            j = min(int(np.searchsorted(offsets[1:], x - 1e-12)), n - 1)
            print(f"{ident}.m{k} {float(local(j, min(max(x - offsets[j], 0.0), spans[j])))!r}")
        ends = [float(local(n - 1, spans[-1]))]
        for j in range(n):
            print(f"{ident}.s{j} {float(start[j])!r}")
        print(f"{ident}.s{n} {ends[0]!r}")
        for j in range(n + 1):
            reaction = (shear[j] if j < n else 0.0) - (end_shear(j - 1) if j > 0 else 0.0)
            print(f"{ident}.r{j} {float(reaction)!r}")


main()
