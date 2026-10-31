import sys, os, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from nw import *
mod = importlib.import_module(sys.argv[1])
a = mod.a
den = int(sys.argv[2])
pid = sys.argv[3]
ppm = float(sys.argv[4]) if len(sys.argv) > 4 else 60
pose = [p for p in a["poses"] if p["id"] == pid][0]
rp = a["representations"][str(den)]
f = OUT + "%s-%d-%s.png" % (a["id"], den, pid)
render(a, rp, pose, den, f, ppm=ppm, pad_m=0.15 * den / 100)
print(f)
