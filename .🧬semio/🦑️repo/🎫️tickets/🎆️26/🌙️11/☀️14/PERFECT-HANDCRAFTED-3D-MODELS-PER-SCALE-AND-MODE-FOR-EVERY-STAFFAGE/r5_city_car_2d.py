"""✏️ Round 5 drawings of the city-car: the round 3 drawings of every scale with the headlamps redrawn as the true views of the wrap-around lamp lofts (1:50 to 1:200) and the mirrors as the views of the lofted housings."""
from r3_city_car_2d import drawing as r3_drawing, ellipse, poly

LAMP_SIDE = [(1.64, 0.749), (1.72, 0.76), (1.85, 0.76), (1.905, 0.749), (1.912, 0.715), (1.905, 0.681), (1.85, 0.67), (1.72, 0.67), (1.64, 0.681), (1.632, 0.715)]
LAMP_FRONT = [(0.575, 0.715), (0.585, 0.75), (0.68, 0.76), (0.79, 0.755), (0.805, 0.715), (0.79, 0.675), (0.68, 0.67), (0.585, 0.68)]
LAMP_PLAN = [(1.64, 0.81), (1.72, 0.80), (1.79, 0.758), (1.85, 0.695), (1.885, 0.64), (1.91, 0.59), (1.905, 0.565), (1.875, 0.605), (1.84, 0.66), (1.78, 0.73), (1.71, 0.774), (1.64, 0.788)]


def lamp(shape, points):
    return poly(shape["id"], shape["bone"], shape["role"], shape["z"], shape["tone"], points, shape.get("region"), shape.get("material"))


def drawing(scale):
    out = r3_drawing(scale)
    for view, shapes in out.items():
        for k, shape in enumerate(shapes):
            if not shape["id"].startswith("headlamp"):
                continue
            s = -1 if shape["id"].endswith("-r") else 1
            if view == "elevation":
                shapes[k] = lamp(shape, LAMP_SIDE)
            elif view == "front":
                shapes[k] = lamp(shape, [(s * z, y) for z, y in LAMP_FRONT])
            else:
                shapes[k] = lamp(shape, [(x, s * z) for x, z in LAMP_PLAN])
    return out
