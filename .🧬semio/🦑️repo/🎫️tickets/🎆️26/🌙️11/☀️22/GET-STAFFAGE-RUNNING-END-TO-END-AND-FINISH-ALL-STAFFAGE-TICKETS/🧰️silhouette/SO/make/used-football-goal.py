"""⚽️ Writes the repair ops of `used-football-goal`: at 1:50 the side elevation draws the sagging back mesh by the outline of its volume (per row of the mesh polyhedron the foremost and the rearmost point) instead of a band with two kinks that stood 24 mm behind the mesh in its upper half."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, solid, write
V = solid(asset("used-football-goal"), 50, "back-mesh")["vertices"]
COLS = 25
layer = len(V) // 2
rows = [[(V[k * layer + r * COLS + c][0], V[k * layer + r * COLS + c][1]) for k in (0, 1) for c in range(COLS)] for r in range(layer // COLS)]
front = [max(row) for row in rows]
back = [min(row) for row in rows]
points = [[round(x, 5), round(y, 5)] for x, y in front + back[::-1]]
write("used-football-goal", [{"op": "shape-set", "scale": "50", "view": "elevation", "id": "back-mesh", "set": {"points": points}}])
