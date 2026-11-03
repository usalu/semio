"""🧭️ Dry run of the `mechanics.*` checks of the sectionproperties adapter: validates the geometry of a case and plans the mesh without importing the library or solving anything, so a case file can be checked on a machine that has no solver installed.

The refusals mirror the Rust kernel `semio_s_plugin_norm_mechanics::{analyse_section, analyse_thin_walled}`: a ring needs three finite vertices and an area, every hole lies inside the outer ring, thin walls have a positive thickness and form one connected open tree (closed cells and disconnected walls are refused). The returned closed forms (net area, sum of l t, sum of l t^3 / 3) are what a solver result is sanity-checked against.
"""

import math

CHECK_SECTION = "mechanics.analyse-section"
CHECK_THIN = "mechanics.analyse-thin-walled"
DEFAULT_CELLS = 2500


def number(value, label):
    """🔢️ A finite float or a `ValueError` naming what was wrong."""
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        raise ValueError(f"{label} must be a finite number, got {value!r}")
    return float(value)


def ring_of(points, label):
    """⭕️ A ring of at least three finite (y, z) vertices with a non-zero area, as a tuple list."""
    if not isinstance(points, list) or len(points) < 3:
        raise ValueError(f"{label} needs at least three vertices")
    ring = []
    for k, vertex in enumerate(points):
        if not isinstance(vertex, (list, tuple)) or len(vertex) != 2:
            raise ValueError(f"{label} vertex {k} must be a [y, z] pair")
        ring.append((number(vertex[0], f"{label} vertex {k} y"), number(vertex[1], f"{label} vertex {k} z")))
    if abs(signed_area(ring)) <= 1e-18:
        raise ValueError(f"{label} encloses no area")
    return ring


def signed_area(ring):
    """📐️ Shoelace area, positive for counter-clockwise rings."""
    return 0.5 * sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(ring, ring[1:] + ring[:1]))


def inside(point, ring):
    """📍️ Even-odd point-in-polygon test."""
    y, z = point
    hit = False
    for a, b in zip(ring, ring[1:] + ring[:1]):
        if (a[1] > z) != (b[1] > z) and y < a[0] + (z - a[1]) * (b[0] - a[0]) / (b[1] - a[1]):
            hit = not hit
    return hit


def centroid(ring):
    """🎯️ Vertex mean of a ring, enough to locate a hole in the outer ring."""
    return (sum(p[0] for p in ring) / len(ring), sum(p[1] for p in ring) / len(ring))


def cells_of(inputs):
    """🧮️ The number of mesh cells the case asks for, a positive integer."""
    cells = inputs.get("cells", DEFAULT_CELLS)
    if isinstance(cells, bool) or not isinstance(cells, int) or cells < 16:
        raise ValueError(f"cells must be an integer of at least 16, got {cells!r}")
    return cells


def poisson_of(inputs):
    """🧪️ Poisson's ratio of the shear functions, 0 up to below 0.5."""
    nu = number(inputs.get("nu", 0.0), "nu")
    if not 0.0 <= nu < 0.5:
        raise ValueError(f"nu must be in [0, 0.5), got {nu}")
    return nu


def rings_of(inputs):
    """⭕️ The outer ring and the hole rings of a polygon case, validated; every hole must lie inside the outer ring."""
    outer = ring_of(inputs.get("outer"), "outer")
    holes = [ring_of(hole, f"hole {k}") for k, hole in enumerate(inputs.get("holes", []))]
    for k, hole in enumerate(holes):
        if not inside(centroid(hole), outer):
            raise ValueError(f"hole {k} does not lie inside the outer ring")
    return outer, holes


def plan_analyse_section(inputs):
    """🔲️ Plan of `mechanics.analyse-section`: net area, bounds, ring and vertex counts, Poisson's ratio and the mesh cell area."""
    outer, holes = rings_of(inputs)
    net = abs(signed_area(outer)) - sum(abs(signed_area(hole)) for hole in holes)
    if net <= 0.0:
        raise ValueError("the holes remove the whole outer ring")
    cells = cells_of(inputs)
    ys, zs = [p[0] for p in outer], [p[1] for p in outer]
    return {"check": CHECK_SECTION, "area": net, "bounds": [min(ys), min(zs), max(ys), max(zs)], "rings": 1 + len(holes), "vertices": len(outer) + sum(len(h) for h in holes), "nu": poisson_of(inputs), "cells": cells, "cell_area": net / cells}


def walls_of(inputs):
    """🪶️ The centreline walls as (y1, z1, y2, z2, t) tuples, validated: finite, positive thickness, non-zero length."""
    walls = inputs.get("walls")
    if not isinstance(walls, list) or not walls:
        raise ValueError("a thin-walled section needs at least one wall")
    out = []
    for k, wall in enumerate(walls):
        if not isinstance(wall, dict):
            raise ValueError(f"wall {k} must be an object with y1, z1, y2, z2 and thickness")
        y1, z1, y2, z2, t = (number(wall.get(key), f"wall {k} {key}") for key in ("y1", "z1", "y2", "z2", "thickness"))
        if t <= 0.0:
            raise ValueError(f"wall {k} thickness must be positive")
        if math.hypot(y2 - y1, z2 - z1) <= 0.0:
            raise ValueError(f"wall {k} has zero length")
        out.append((y1, z1, y2, z2, t))
    return out


def topology(walls):
    """🕸️ Node count after merging endpoints at 1e-9 of the longest wall; raises for disconnected walls and closed cells, like the Rust kernel."""
    scale = max(math.hypot(w[2] - w[0], w[3] - w[1]) for w in walls)
    tol = 1e-9 * scale
    nodes = []

    def node(point):
        for k, q in enumerate(nodes):
            if abs(point[0] - q[0]) < tol and abs(point[1] - q[1]) < tol:
                return k
        nodes.append(point)
        return len(nodes) - 1

    edges = [(node((w[0], w[1])), node((w[2], w[3]))) for w in walls]
    parent = list(range(len(nodes)))

    def find(k):
        while parent[k] != k:
            parent[k] = parent[parent[k]]
            k = parent[k]
        return k

    for a, b in edges:
        if find(a) == find(b):
            raise ValueError("closed cells are refused: use mechanics.analyse-section for them")
        parent[find(a)] = find(b)
    if len({find(k) for k in range(len(nodes))}) != 1:
        raise ValueError("the walls are not connected")
    return len(nodes)


def plan_analyse_thin_walled(inputs):
    """🪶️ Plan of `mechanics.analyse-thin-walled`: wall and node counts, total centreline length, A = sum l t and J = sum l t^3 / 3, the closed forms the Rust kernel reproduces and the solid-strip solver approaches for thin walls."""
    walls = walls_of(inputs)
    nodes = topology(walls)
    lengths = [math.hypot(w[2] - w[0], w[3] - w[1]) for w in walls]
    return {"check": CHECK_THIN, "walls": len(walls), "nodes": nodes, "length": sum(lengths), "area": sum(l * w[4] for l, w in zip(lengths, walls)), "j": sum(l * w[4] ** 3 / 3.0 for l, w in zip(lengths, walls)), "nu": poisson_of(inputs), "cells": cells_of(inputs)}


DRY_RUN = {CHECK_SECTION: plan_analyse_section, CHECK_THIN: plan_analyse_thin_walled}
