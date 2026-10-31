from sheep import *
import sheep_shapes as SS
sk = make_skel()
OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-A1/"
offs = standing_offsets(sk)
print(offs)
for b in sk.bones:
    v = offs.get(b["id"], 0)
    lo, hi = sk.lim[b["id"]]
    if not lo <= v <= hi: print("LIMIT", b["id"], v, lo, hi)
fr = sk.frames(offs)
sh = SS.shapes50(sk)
svg_pose(sk, sh.items, fr, OUT + "sheepS.svg", px_per_m=1000, show_anchors=True)
png(OUT + "sheepS.svg", OUT + "sheepS.png")
print([round(v, 3) for v in bounds(world_rings(sk, sh.items, fr))])
