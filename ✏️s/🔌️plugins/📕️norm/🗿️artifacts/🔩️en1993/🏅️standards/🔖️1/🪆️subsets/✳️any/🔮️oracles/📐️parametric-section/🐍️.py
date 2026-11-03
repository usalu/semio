"""📐️ Independent oracle of the parametric section properties: numeric integration over the section polygon (shapely-free Green's theorem) for the welded I of the showcase row. Run with `python3 -I`."""
import json
import math


def polygon_props(points):
    area = sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1])) / 2.0
    cx = sum((x0 + x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1])) / (6.0 * area)
    cy = sum((y0 + y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1])) / (6.0 * area)
    ix = sum((y0 * y0 + y0 * y1 + y1 * y1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1])) / 12.0 - area * cy * cy
    iy = sum((x0 * x0 + x0 * x1 + x1 * x1) * (x0 * y1 - x1 * y0) for (x0, y0), (x1, y1) in zip(points, points[1:] + points[:1])) / 12.0 - area * cx * cx
    return abs(area), abs(ix), abs(iy)


def welded_i_polygon(h, b, tw, tf):
    return [(-b / 2, -h / 2), (b / 2, -h / 2), (b / 2, -h / 2 + tf), (tw / 2, -h / 2 + tf), (tw / 2, h / 2 - tf), (b / 2, h / 2 - tf), (b / 2, h / 2), (-b / 2, h / 2), (-b / 2, h / 2 - tf), (-tw / 2, h / 2 - tf), (-tw / 2, -h / 2 + tf), (-b / 2, -h / 2 + tf)]


def main():
    area, iy, iz = polygon_props(welded_i_polygon(240.0, 240.0, 10.0, 17.0))
    ref = {"area": 10600.0, "iy": 11260e4, "iz": 3923e4, "w_el_y": 938e3, "w_pl_y": 1053e3}
    w_el = 2 * iy / 240.0
    hw = 240.0 - 34.0
    w_pl = 240.0 * 17.0 * 223.0 + 10.0 * hw * hw / 4.0
    dev = max(abs(area - ref["area"]) / ref["area"], abs(iy - ref["iy"]) / ref["iy"], abs(iz - ref["iz"]) / ref["iz"], abs(w_el - ref["w_el_y"]) / ref["w_el_y"], abs(w_pl - ref["w_pl_y"]) / ref["w_pl_y"])
    print(json.dumps({"area": area, "iy": iy, "iz": iz, "w_el_y": w_el, "w_pl_y": w_pl, "deviation": dev, "utilisation": dev / 0.05}))


main()
