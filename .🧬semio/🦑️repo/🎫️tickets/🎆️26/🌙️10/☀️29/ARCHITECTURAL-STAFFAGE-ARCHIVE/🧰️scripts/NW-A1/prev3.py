import sys
from sheep import *
import sheep_shapes as SS
sk = make_skel()
OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-A1/"
sh = SS.shapes50(sk)
for name, fn in (("graze", grazing_offsets), ("lie", lying_offsets)):
    offs, root = fn(sk)
    for b in sk.bones:
        v = offs.get(b["id"], 0); lo, hi = sk.lim[b["id"]]
        if not lo - 1e-6 <= v <= hi + 1e-6: print("LIMIT", name, b["id"], v, lo, hi)
    fr = sk.frames(offs, root)
    svg_pose(sk, sh.items, fr, OUT + "sheep_%s.svg" % name, px_per_m=800, show_anchors=True)
    png(OUT + "sheep_%s.svg" % name, OUT + "sheep_%s.png" % name)
    print(name, [round(v, 3) for v in bounds(world_rings(sk, sh.items, fr))])
