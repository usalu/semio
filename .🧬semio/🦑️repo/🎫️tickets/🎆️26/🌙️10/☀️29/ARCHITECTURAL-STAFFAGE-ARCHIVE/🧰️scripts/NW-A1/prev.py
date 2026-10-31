import sys, importlib
from sheep import *
import sheep_shapes as SS
sk = make_skel()
OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-A1/"
fr = sk.frames({})
for n in ("50", "100", "200", "500"):
    sh = getattr(SS, "shapes" + n)(sk)
    svg_pose(sk, sh.items, fr, OUT + "sheep%s.svg" % n, px_per_m=1000 if n in ("50", "100") else 1000)
    png(OUT + "sheep%s.svg" % n, OUT + "sheep%s.png" % n)
    print(n, [round(v, 3) for v in bounds(world_rings(sk, sh.items, fr))])
