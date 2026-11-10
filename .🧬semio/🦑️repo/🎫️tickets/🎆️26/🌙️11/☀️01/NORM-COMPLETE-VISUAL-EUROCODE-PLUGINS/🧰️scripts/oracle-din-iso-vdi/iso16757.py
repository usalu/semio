"""ISO 16757-2 independent cross-checks: solid volumes by scipy ConvexHull and closed forms, the rotated axis-aligned extent by numpy, against the Rust bounding-box evaluation transcribed from `part-libraries/🦀️.rs` (rotation ignored, every boolean operator the union hull)."""
import math

import numpy as np
from scipy.spatial import ConvexHull


def box_pts(w, h, d):
    return np.array([[x, y, z] for x in (0, w) for y in (0, h) for z in (0, d)], dtype=float)


def cylinder_pts(r, h, n=2048):
    a = np.linspace(0, 2 * math.pi, n, endpoint=False)
    ring = np.stack([r + r * np.cos(a), np.zeros(n), r + r * np.sin(a)], axis=1)
    return np.vstack([ring, ring + [0, h, 0]])


def sphere_pts(r, n=20000):
    i = np.arange(n) + 0.5
    phi = np.arccos(1 - 2 * i / n)
    th = math.pi * (1 + 5 ** 0.5) * i
    return np.stack([r + r * np.cos(th) * np.sin(phi), r + r * np.sin(th) * np.sin(phi), r + r * np.cos(phi)], axis=1)


def rot_z(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[c, -s, 0], [s, c, 0], [0, 0, 1]])


def aabb_volume(pts):
    ext = pts.max(axis=0) - pts.min(axis=0)
    return float(np.prod(ext)), ext.tolist()


def vectors():
    rows = []
    def add(name, solid, hull_pts, rust_bbox, note):
        v_hull = float(ConvexHull(hull_pts).volume) if hull_pts is not None else None
        rows.append({"id": name, "solidVolumeM3": solid, "convexHullVolumeM3": v_hull, "rustBoundingBoxVolumeM3": rust_bbox, "rustOverSolid": rust_bbox / solid, "note": note})
    add("box-1x2x3", 6.0, box_pts(1, 2, 3), 6.0, "agree")
    add("cylinder-r1-h2", math.pi * 1.0 ** 2 * 2.0, cylinder_pts(1.0, 2.0), 2.0 * 2.0 * 2.0, "bounding box of a cylinder")
    add("sphere-r1.5", 4.0 / 3.0 * math.pi * 1.5 ** 3, sphere_pts(1.5), 27.0, "bounding box of a sphere")
    rotated = box_pts(2, 1, 1) @ rot_z(45).T
    true_aabb, ext = aabb_volume(rotated)
    add("box-2x1x1-rotated-45-about-z", 2.0, rotated, 2.0 * 1.0 * 1.0, f"Rust ignores rotationDeg; the true axis-aligned extent is {ext} (volume {true_aabb:.6f}), the Rust box 2x1x1 does not contain the solid")
    add("union-of-two-unit-boxes-5m-apart", 2.0, None, 6.0 ** 3, "Boolean union takes the hull box of the children")
    a, b = box_pts(1, 1, 1), box_pts(1, 1, 1) + 0.5
    inter = float(np.prod(np.minimum(a.max(0), b.max(0)) - np.maximum(a.min(0), b.min(0))))
    add("intersection-of-two-unit-boxes-offset-0.5", inter, None, 1.5 ** 3, "Boolean intersection also returns the union hull")
    add("difference-unit-box-minus-half-box", 1.0 - 0.5, None, 1.0, "Boolean difference also returns the union hull")
    for r in rows:
        r["agrees"] = abs(r["rustBoundingBoxVolumeM3"] - r["solidVolumeM3"]) <= 1e-9 * r["solidVolumeM3"]
        if r["convexHullVolumeM3"] is not None:
            r["hullAgreesWithClosedForm"] = abs(r["convexHullVolumeM3"] - r["solidVolumeM3"]) / r["solidVolumeM3"] < (2e-3 if "sphere" in r["id"] else 1e-4)
    for r in rows:
        if r["id"].startswith("box-2x1x1-rotated"):
            r["agrees"] = False
    return {"oracle": "scipy.spatial.ConvexHull, numpy rotation, closed forms; ISO 16757-2 7.1 solid volume", "volumes": rows}


def divergences(doc):
    return [{"id": "iso16757." + r["id"], "clause": "ISO 16757-2 7.1 (solid volume)", "inputs": r["id"], "rust": r["rustBoundingBoxVolumeM3"], "reference": r["solidVolumeM3"], "relative": r["rustOverSolid"] - 1, "note": r["note"]} for r in doc["volumes"] if not r["agrees"]]
