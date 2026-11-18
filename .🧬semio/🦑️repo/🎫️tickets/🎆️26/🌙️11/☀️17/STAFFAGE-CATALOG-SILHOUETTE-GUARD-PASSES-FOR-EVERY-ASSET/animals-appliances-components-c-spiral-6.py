import math
from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/spiral-stair")
n = d.shape(50, "front", "handrail")
pts = d.obj(n)["points"]
h = 0.02125
def move(i, j):
    (x0, y0), (x1, y1) = pts[i], pts[j]
    l = math.hypot(x1 - x0, y1 - y0)
    return [round(x0 + (x1 - x0) / l * h, 5), round(y0 + (y1 - y0) / l * h, 5)]
a = move(0, 1); b = move(len(pts) - 1, len(pts) - 2)
L.sn(d, n, "points", "0", "0", value=a[0]); L.sn(d, n, "points", "0", "1", value=a[1])
L.sn(d, n, "points", str(len(pts) - 1), "0", value=b[0]); L.sn(d, n, "points", str(len(pts) - 1), "1", value=b[1])
d.commit()
