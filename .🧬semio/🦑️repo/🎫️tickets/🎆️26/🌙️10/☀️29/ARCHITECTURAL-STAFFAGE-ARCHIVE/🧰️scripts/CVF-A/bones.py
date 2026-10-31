import sys
from cva import *
aid, pose = sys.argv[1], sys.argv[2]
a = load_v1(aid); w = pose_world(a, pose)
for b in a["rig"]["bones"]:
    p = app(w[b["id"]], 0, 0); q = app(w[b["id"]], b["lengthM"], 0)
    print(f"{b['id']:14s} ({p[0]:.3f},{p[1]:.3f}) -> ({q[0]:.3f},{q[1]:.3f})")
for an in a["anchors"]:
    p = app(w[an["bone"]], an["x"], an["y"]); print(f"  anchor {an['id']:16s} ({p[0]:.3f},{p[1]:.3f}) on {an['bone']}")
