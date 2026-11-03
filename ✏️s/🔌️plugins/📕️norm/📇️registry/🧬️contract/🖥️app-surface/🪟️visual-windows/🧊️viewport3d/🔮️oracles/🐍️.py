"""🔮️ Independent oracle for the viewport prism meshes: section areas from a closed form and from shapely when importable.

Run with `python3 -I 🐍️.py` to print the golden JSON the Rust tests compare against (`../🧪️tests/🧫️fixtures/🔣️prisms.json`).
"""
import json

def i_shape(h, b, tw, tf):
    return 2 * b * tf + (h - 2 * tf) * tw

def rhs(h, b, t):
    return h * b - (h - 2 * t) * (b - 2 * t)

def chs(d, t, n=48):
    import math
    ring = lambda r: 0.5 * n * r * r * math.sin(2 * math.pi / n)
    return ring(d / 2) - ring(d / 2 - t)

def shapely_area(kind, *p):
    try:
        from shapely.geometry import Polygon
    except ImportError:
        return None
    if kind == "rhs":
        h, b, t = p
        return Polygon([(-b/2, -h/2), (b/2, -h/2), (b/2, h/2), (-b/2, h/2)], [[(-b/2+t, -h/2+t), (b/2-t, -h/2+t), (b/2-t, h/2-t), (-b/2+t, h/2-t)]]).area
    if kind == "ishape":
        h, b, tw, tf = p
        pts = [(0, 0), (b, 0), (b, tf), ((b+tw)/2, tf), ((b+tw)/2, h-tf), (b, h-tf), (b, h), (0, h), (0, h-tf), ((b-tw)/2, h-tf), ((b-tw)/2, tf), (0, tf)]
        return Polygon(pts).area

golden = {
    "ishape": {"params": [0.24, 0.24, 0.01, 0.017], "area": i_shape(0.24, 0.24, 0.01, 0.017), "shapely": shapely_area("ishape", 0.24, 0.24, 0.01, 0.017)},
    "rhs": {"params": [0.2, 0.1, 0.01], "area": rhs(0.2, 0.1, 0.01), "shapely": shapely_area("rhs", 0.2, 0.1, 0.01)},
    "chs": {"params": [0.1, 0.005], "area": chs(0.1, 0.005), "shapely": None},
}

if __name__ == "__main__":
    print(json.dumps(golden, indent=2))
