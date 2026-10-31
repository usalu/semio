import sys
from skate_poses import *
from skate_art import all_shapes
sc = int(sys.argv[1]) if len(sys.argv) > 1 else 50
pose = sys.argv[2] if len(sys.argv) > 2 else "cruising"
zoom = float(sys.argv[3]) if len(sys.argv) > 3 else 1.0
PENS = {50: dict(body=.25, detail=.18, texture=.13, glazing=.13, ground=.35), 100: dict(body=.2, detail=.13, texture=.13, glazing=.13, ground=.3), 200: dict(body=.18, detail=.13, texture=None, glazing=None, ground=.25), 500: dict(body=.15, detail=None, texture=None, glazing=None, ground=.2)}
P = poses()
sh = all_shapes()[sc]
f = P[pose]
# ground via lowest contact: shift
W = f.world_all()
png = render(svg(f, sh, PENS[sc], sc, zoom=zoom, ground_y=0.0), f"skate-{sc}-{pose}-z{zoom}", None)
x0,y0,x1,y1 = bounds(f, sh)
print(png, "bounds", round(x0,3), round(y0,3), round(x1,3), round(y1,3), "w", round(x1-x0,3), "h", round(y1-y0,3))
