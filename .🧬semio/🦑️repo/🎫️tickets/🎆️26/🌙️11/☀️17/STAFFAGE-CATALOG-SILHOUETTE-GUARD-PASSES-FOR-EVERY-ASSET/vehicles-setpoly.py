"""Sets the points of one drawn polygon from the outline of volume solids: python3 vehicles-setpoly.py <category/asset> <scale> <view> <shape-id> <solid-ids> [tolerance]"""
import sys, json, subprocess, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
slug, scale, view, sid, ids = sys.argv[1:6]
tol = sys.argv[6] if len(sys.argv) > 6 else "0.004"
off = [float(v) for v in sys.argv[7].split(",")] if len(sys.argv) > 7 else [0.0, 0.0, 0.0]
import math
ang = math.radians(off[2] if len(off) > 2 else 0.0)
name = slug.split("/")[-1]
out = subprocess.run(["./vehicles-run.sh", "outline", name, scale, view, ids, tol], capture_output=True, text=True).stdout
def loc(x, y):
    dx, dy = x - off[0], y - off[1]
    return [round(dx * math.cos(ang) + dy * math.sin(ang), 4), round(-dx * math.sin(ang) + dy * math.cos(ang), 4)]
pts = [loc(x, y) for x, y in json.loads(out.strip().splitlines()[-1])]
d = m.V(slug)
d.drawn_array(d.shape(int(scale), view, sid), "points", pts)
d.commit()
print(len(pts), "points")
