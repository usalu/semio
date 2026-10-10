#!/usr/bin/env python3
"""🧨️ Third-party ORACLE for the `s.bim.model@1` inference `🧨️clash-sets`.

The subject (Rust, `semio-s-artifact-bim-model`) tests the solids of the elements two selectors pick against each other with a bounding-volume hierarchy and a triangle/triangle test and reports hard
clashes (interpenetration beyond the tolerance) and soft clashes (closer than the clearance). This file re-derives the table from the SAME committed inputs without sharing a line of code with the
subject: the authored snapshot (clash sets, selectors) and the committed world meshes of the solids (`🧊️meshes`, written by the subject's own test from the inferred solids, which the solid oracles
audit separately). It uses only `numpy`, a library that has never seen this repository:

* the meshes of the fixture are boxes (columns, beams, walls and slabs of an orthogonal frame): the file verifies it (the volume of a mesh by the divergence theorem equals the volume of its box),
  and for boxes the clash is analytic: the overlap per axis, a hard clash when every overlap exceeds `max(tolerance, 1e-9)` with the penetration `-min(overlap)` and the contact point at the centre of
  the overlap box, touching (an overlap of zero on an axis) or an overlap within the tolerance is no clash, a soft clash when the boxes are separated by a Euclidean gap in `(0, clearance)`;
* independently of that, a brute-force triangle/triangle test (the Moller interval test written with numpy) decides for every pair of the meshes whether their surfaces cross, and the file asserts that
  the boxes overlap with a positive volume exactly when the surfaces cross or one lies inside the other.

The committed expectation under `🧫️fixtures/💡️inferences/🧨️clash-sets/<case>/💡️inference/🧨️clashes/🔣️.json` is WRITTEN by this file (`write`), never by hand.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/🧨️clash-sets>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/🧨️clash-sets>
"""

# region 🔖️Imports
import json
import sys
from pathlib import Path

import numpy as np

# endregion 🔖️Imports

EXACT = 1e-9
CLASSES = {"walls": "Wall", "curtain_walls": "CurtainWall", "columns": "Column", "beams": "Beam", "slabs": "Slab", "ceilings": "Ceiling", "roofs": "Roof", "stairs": "Stair", "ramps": "Ramp", "railings": "Railing", "wall_sweeps": "WallSweep", "components": "Component", "mep_elements": "Mep"}


# region 🔖️Elements
def class_of(snapshot, element):
    """🧩️ The class of an element of the authored snapshot, none for an id that has no solid."""
    for collection, name in CLASSES.items():
        if element in snapshot.get(collection, {}):
            return name
    opening = snapshot.get("openings", {}).get(element)
    if opening:
        return {"Window": "Window", "Door": "Door"}.get(next(iter(opening["kind"])))
    return None


def storey_of(snapshot, element):
    """🪜️ The storey an element stands on, the storey of the host for an opening or a wall sweep."""
    for collection in CLASSES:
        row = snapshot.get(collection, {}).get(element)
        if row and "storey" in row:
            return row["storey"]
    host = (snapshot.get("openings", {}).get(element) or snapshot.get("wall_sweeps", {}).get(element) or {}).get("host")
    for collection in ("walls", "curtain_walls"):
        if host in snapshot.get(collection, {}):
            return snapshot[collection][host]["storey"]
    return None


def phase_of(snapshot, element):
    """🕰️ The effective phase: its own, the host's for an opening or sweep, New otherwise."""
    for collection in ("walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings"):
        row = snapshot.get(collection, {}).get(element)
        if row:
            return row.get("phase", "New")
    host = (snapshot.get("openings", {}).get(element) or snapshot.get("wall_sweeps", {}).get(element) or {}).get("host")
    for collection in ("walls", "curtain_walls"):
        if host in snapshot.get(collection, {}):
            return snapshot[collection][host].get("phase", "New")
    return "New"


def picks(snapshot, selector, element):
    """🎯️ Whether a selector picks an element: every restriction that is not empty must hold."""
    cls = class_of(snapshot, element)
    return (
        (not selector["classes"] or cls in selector["classes"])
        and (not selector["ids"] or element in selector["ids"])
        and (not selector["storeys"] or storey_of(snapshot, element) in selector["storeys"])
        and (not selector["phases"] or phase_of(snapshot, element) in selector["phases"])
    )


def hosting(snapshot):
    """🔗️ The (element, host) relations: an opening and its wall, a wall sweep and its wall, a railing and its host."""
    rows = {}
    for collection in ("openings", "wall_sweeps"):
        rows.update({key: row["host"] for key, row in snapshot.get(collection, {}).items()})
    rows.update({key: row["host"]["element"] for key, row in snapshot.get("railings", {}).items() if row.get("host")})
    return rows


# endregion 🔖️Elements


# region 🔖️Geometry
def volume(mesh):
    """📦️ The enclosed volume by the divergence theorem."""
    p, t = np.array(mesh["positions"], float), np.array(mesh["indices"], int)
    a, b, c = p[t[:, 0]], p[t[:, 1]], p[t[:, 2]]
    return float(np.einsum("ij,ij->i", a, np.cross(b, c)).sum() / 6.0)


def box_of(mesh):
    p = np.array(mesh["positions"], float)
    return p.min(axis=0), p.max(axis=0)


def is_box(mesh):
    lo, hi = box_of(mesh)
    return abs(volume(mesh) - float(np.prod(hi - lo))) < 1e-9 * max(1.0, float(np.prod(hi - lo)))


def segment_triangle(p, q, tri):
    """✂️ The point where the segment pq crosses the triangle (including its border), or none."""
    a, b, c = tri
    n = np.cross(b - a, c - a)
    d = q - p
    denom = float(n @ d)
    if abs(denom) < 1e-14:
        return None
    t = float(n @ (a - p)) / denom
    if t < -1e-10 or t > 1 + 1e-10:
        return None
    x = p + t * d
    for u, v in ((a, b), (b, c), (c, a)):
        if float(n @ np.cross(v - u, x - u)) < -1e-10 * max(1.0, float(n @ n) ** 0.5):
            return None
    return x


def surfaces_cross(first, second):
    """🔀️ Whether any triangle of one mesh touches any triangle of the other (edge/triangle test both ways)."""
    pa, ta = np.array(first["positions"], float), np.array(first["indices"], int)
    pb, tb = np.array(second["positions"], float), np.array(second["indices"], int)
    for x in ta:
        tri_x = pa[x]
        for y in tb:
            tri_y = pb[y]
            for i in range(3):
                if segment_triangle(tri_x[i], tri_x[(i + 1) % 3], tri_y) is not None or segment_triangle(tri_y[i], tri_y[(i + 1) % 3], tri_x) is not None:
                    return True
    return False


# endregion 🔖️Geometry


# region 🔖️Clash
def clash(first, second, tolerance, clearance):
    """💥️ The analytic clash of two boxes, none when they do not clash. Returns kind, distance and the contact point (hard clashes only)."""
    (alo, ahi), (blo, bhi) = first, second
    overlap = np.minimum(ahi, bhi) - np.maximum(alo, blo)
    if (overlap > EXACT).all():
        extent = float(overlap.min())
        if extent > max(tolerance, EXACT):
            lo, hi = np.maximum(alo, blo), np.minimum(ahi, bhi)
            return {"kind": "Hard", "distance": -extent, "point": [float(v) for v in (lo + hi) / 2.0]}
        return None
    gap = np.maximum(-overlap, 0.0)
    distance = float(np.linalg.norm(gap))
    if clearance > 0 and EXACT < distance < clearance:
        return {"kind": "Clearance", "distance": distance, "point": None}
    return None


def table(snapshot, meshes):
    """🧨️ The table of every clash set of the snapshot over the committed meshes."""
    host = hosting(snapshot)
    out = {}
    for set_id, record in sorted(snapshot.get("clash_sets", {}).items()):
        side_a = sorted(e for e in meshes if picks(snapshot, record["a"], e))
        side_b = sorted(e for e in meshes if picks(snapshot, record["b"], e))
        clashes, seen, tested = [], set(), 0
        for x in side_a:
            for y in side_b:
                pair = tuple(sorted((x, y)))
                if x == y or pair in seen or host.get(x) == y or host.get(y) == x:
                    continue
                seen.add(pair)
                (ax, ay) = (box_of(meshes[pair[0]]), box_of(meshes[pair[1]]))
                grown = np.maximum(ax[0], ay[0]) - np.minimum(ax[1], ay[1])
                if (grown > record["clearance"] + 1e-12).any():
                    continue
                tested += 1
                found = clash(ax, ay, record["tolerance"], record["clearance"])
                if found:
                    clashes.append({"first": pair[0], "second": pair[1], **found})
        clashes.sort(key=lambda row: (row["first"], row["second"]))
        out[set_id] = {"elements_a": len(side_a), "elements_b": len(side_b), "tested": tested, "clashes": clashes}
    return out


def audit(snapshot, meshes):
    """🔎️ The independent checks: the meshes are boxes, and a positive overlap volume holds exactly when the surfaces cross or one box lies inside the other."""
    problems = []
    for element, mesh in sorted(meshes.items()):
        if not is_box(mesh):
            problems.append("%s: the mesh is not a box" % element)
    names = sorted(meshes)
    for i, x in enumerate(names):
        for y in names[i + 1 :]:
            (alo, ahi), (blo, bhi) = box_of(meshes[x]), box_of(meshes[y])
            overlap = np.minimum(ahi, bhi) - np.maximum(alo, blo)
            if (overlap < -1e-9).any():
                continue
            positive = bool((overlap > EXACT).all())
            inside = bool((alo >= blo - EXACT).all() and (ahi <= bhi + EXACT).all()) or bool((blo >= alo - EXACT).all() and (bhi <= ahi + EXACT).all())
            crossing = surfaces_cross(meshes[x], meshes[y])
            if positive and not (crossing or inside):
                problems.append("%s/%s: overlapping boxes without crossing surfaces" % (x, y))
    return problems


def compare(expected, got):
    """⚖️ The differences between two tables: structure and ids exactly, numbers within a nanometre."""
    problems = []
    if sorted(expected) != sorted(got):
        return ["sets differ: %s vs %s" % (sorted(expected), sorted(got))]
    for set_id in expected:
        a, b = expected[set_id], got[set_id]
        for key in ("elements_a", "elements_b", "tested"):
            if a[key] != b[key]:
                problems.append("%s.%s: %s vs %s" % (set_id, key, a[key], b[key]))
        ids_a = [(c["first"], c["second"], c["kind"]) for c in a["clashes"]]
        ids_b = [(c["first"], c["second"], c["kind"]) for c in b["clashes"]]
        if ids_a != ids_b:
            problems.append("%s: clashes %s vs %s" % (set_id, ids_a, ids_b))
            continue
        for x, y in zip(a["clashes"], b["clashes"]):
            if abs(x["distance"] - y["distance"]) > EXACT:
                problems.append("%s %s/%s: distance %s vs %s" % (set_id, x["first"], x["second"], x["distance"], y["distance"]))
            if x["point"] is not None and y["point"] is not None and max(abs(p - q) for p, q in zip(x["point"], y["point"])) > EXACT:
                problems.append("%s %s/%s: point %s vs %s" % (set_id, x["first"], x["second"], x["point"], y["point"]))
    return problems


# endregion 🔖️Clash


# region 🔖️Handlers
def clashes_handler(ctx):
    """🧭️ Oracle handler: the table of the committed inputs of a scenario (registered in the ORACLE role only)."""
    from semio_repo_test import Outcome

    uris = ctx.step_input_uris()
    snapshot = json.loads(ctx.input_bytes(next(uri for uri in uris if "snapshot" in uri)))
    meshes = json.loads(ctx.input_bytes(next(uri for uri in uris if "meshes" in uri)))
    return Outcome(table(snapshot, meshes))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario id."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("clashes-frame", clashes_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        meshes = json.loads((case / "🧊️meshes" / "🔣️.json").read_text(encoding="utf-8"))
        failures += ["%s: %s" % (case.name, problem) for problem in audit(snapshot, meshes)]
        computed = table(snapshot, meshes)
        target = case / "💡️inference" / "🧨️clashes" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed)]
        print("%s: numpy %s, %d meshes, %d sets, %d clashes" % (case.name, np.__version__, len(meshes), len(computed), sum(len(v["clashes"]) for v in computed.values())))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
