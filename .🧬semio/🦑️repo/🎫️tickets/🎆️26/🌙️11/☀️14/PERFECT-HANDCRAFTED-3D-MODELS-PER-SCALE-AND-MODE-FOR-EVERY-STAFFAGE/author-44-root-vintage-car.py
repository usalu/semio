import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from author_14_root import patch, add_shape

S = "vintage-car-1929"


def frame_plan(o):
    o["x0"] = -1.95 if o["x0"] < 0 else 2.05
    o["x1"] = 2.05 if o["x0"] < 0 else -1.95


def plan_frame(o):
    o["x0"], o["x1"] = -1.95, 2.05


def front_frame(o):
    o["x"], o["w"] = -0.41, 0.82


def rb_l(o):
    o["x"], o["w"] = 0.62, 0.18


def rb_r(o):
    o["x"], o["w"] = -0.8, 0.18


def roof_front(o):
    o["x"], o["w"] = -0.6, 1.2


def rad_h(o):
    o["h"] = 0.7


def louvre_l(o):
    o["y"], o["h"] = 0.30, 0.04


def louvre_r(o):
    o["y"], o["h"] = -0.34, 0.04


def tailpipe_plan(o):
    o["x0"] = -1.7575


def fender_top(o):
    pts = o["points"]
    for p in pts:
        if abs(abs(p[0]) - 0.68) < 1e-9 and p[1] == 0.86:
            p[1] = 0.85


e = []
for sc in ("50", "100"):
    e += [(sc, "plan", "frame-l", plan_frame), (sc, "plan", "frame-r", plan_frame), (sc, "front", "running-board-l", rb_l), (sc, "front", "running-board-r", rb_r), (sc, "front", "roof-slab", roof_front)]
for sc in ("50", "100", "200"):
    e += [(sc, "front", "frame", front_frame), (sc, "plan", "tail-pipe", tailpipe_plan), (sc, "front", "fender-l", fender_top), (sc, "front", "fender-r", fender_top)]
e += [("100", "front", "radiator-shell", rad_h), ("50", "plan", "louvre-l", louvre_l), ("50", "plan", "louvre-r", louvre_r)]
print(patch(S, e))
for sc in ("50", "100"):
    add_shape(S, sc, "front", "frame", {"kind": "rect", "x": -0.58, "y": 0.34, "w": 1.16, "h": 0.06, "id": "axle-f", "bone": "axle-f", "role": "detail", "z": 1.2, "filled": True, "tone": 0.3, "material": "cast-iron"})
