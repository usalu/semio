"""🧷 R01 part h1: geometry helpers for widening thin 2D shapes about their own centre line (poly points, rect), shared by the r5-range-gaps-<human> repairs.

@see ./r5-range-gaps-lib.py
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("lib", os.path.join(T, "r5-range-gaps-lib.py"))
LIB = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(LIB)


def widen(axis, width, centre=None):
    """↔️ A shape change that stretches the poly points (or rect) along `axis` (0 = x, 1 = y) about their mid extent (or `centre`) so the extent along it is at least `width` m."""

    def change(shape):
        if shape["kind"] == "rect":
            key, size = ("x", "w") if axis == 0 else ("y", "h")
            if shape[size] < width:
                mid = shape[key] + shape[size] / 2 if centre is None else centre
                shape[key], shape[size] = round(mid - width / 2, 4), width
            return shape
        values = [p[axis] for p in shape["points"]]
        low, high = min(values), max(values)
        if high - low >= width:
            return shape
        mid = (low + high) / 2 if centre is None else centre
        factor = width / (high - low)
        for p in shape["points"]:
            p[axis] = round(mid + (p[axis] - mid) * factor, 4)
        return shape

    return change


def sections(scale_rx=1.0, scale_ry=1.0, floor_rx=None, floor_ry=None):
    """🫱 A loft change that scales (or floors) the radii of every section."""

    def change(solid):
        for s in solid["sections"]:
            s["rx"] = round(max(s["rx"] * scale_rx, floor_rx or 0), 4)
            s["ry"] = round(max(s["ry"] * scale_ry, floor_ry or 0), 4)
        return solid

    return change


def grow(factor):
    """🔍 A poly change that scales the points about their bounding-box centre by `factor`."""

    def change(shape):
        xs, ys = [p[0] for p in shape["points"]], [p[1] for p in shape["points"]]
        cx, cy = (min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2
        shape["points"] = [[round(cx + (x - cx) * factor, 4), round(cy + (y - cy) * factor, 4)] for x, y in shape["points"]]
        return shape

    return change


def radii(factor):
    """🥚 An ellipsoid change that scales its radii by `factor`."""

    def change(solid):
        solid["radii"] = [round(r * factor, 4) for r in solid["radii"]]
        return solid

    return change
