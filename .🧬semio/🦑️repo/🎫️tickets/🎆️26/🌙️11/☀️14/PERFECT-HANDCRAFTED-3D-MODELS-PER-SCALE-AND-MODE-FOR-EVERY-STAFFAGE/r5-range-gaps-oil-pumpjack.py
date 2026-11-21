"""🛢 R01 repair of machinery/oil-pumpjack 1:500 (band 1:350…1:1000): the motor feet and the rod carrier are redrawn and remodelled 0.20 m thick so they print 0.2 mm at 1:1000, then the band is restored to 1:1000.

@see ./r5-range-gaps.md
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r5-range-gaps-lib.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)
A = "machinery/oil-pumpjack"


def feet_elevation(s):
    s["points"] = [[-0.32, -0.35], [0.32, -0.35], [0.32, -0.15], [-0.32, -0.15]]
    return s


def feet_front(s):
    s["h"] = 0.2
    return s


def carrier(s):
    s.update({"x": -0.1, "y": -0.1, "w": 0.2, "h": 0.2})
    return s


def feet_box(s):
    s["size"] = [0.64, 0.2, 0.68]
    s["transform"]["translation"] = [0.0, -0.25, -0.21]
    return s


def carrier_box(s):
    s["size"] = [0.2, 0.2, 0.56]
    return s


if __name__ == "__main__":
    for row in K.edit_shapes(A, [(500, "elevation", "motor-feet", feet_elevation), (500, "front", "motor-feet", feet_front), (500, "elevation", "carrier", carrier), (500, "front", "carrier", carrier), (500, "plan", "carrier", carrier)]):
        print("shape", row[:3])
    for row in K.edit_solids(A, 500, [("body", "motor-feet", feet_box), ("body", "carrier", carrier_box)]):
        print("solid", row[:3])
    print("range was", K.set_range(A, 500, 350, 1000))
