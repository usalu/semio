import sys, importlib
name, sc, pose, zoom = sys.argv[1], int(sys.argv[2]), sys.argv[3], float(sys.argv[4])
rig = importlib.import_module(f"{name}_rig" if name != "skate" else "skate_poses")
art = importlib.import_module(f"{name}_art")
from lib import *
PENS = {50: dict(body=.25, detail=.18, texture=.13, glazing=.13, ground=.35), 100: dict(body=.2, detail=.13, texture=.13, glazing=.13, ground=.3), 200: dict(body=.18, detail=.13, texture=None, glazing=None, ground=.25), 500: dict(body=.15, detail=None, texture=None, glazing=None, ground=.2)}
P = rig.poses()
f = P[pose]
sh = art.all_shapes()[sc]
x0, y0, x1, y1 = bounds(f, sh)
png = render(svg(f, sh, PENS[sc], sc, zoom=zoom, ground_y=0.0), f"{name}-{sc}-{pose}-z{zoom}")
print(png, "bounds", [round(v, 3) for v in (x0, y0, x1, y1)], "w", round(x1 - x0, 3), "h", round(y1 - y0, 3))
