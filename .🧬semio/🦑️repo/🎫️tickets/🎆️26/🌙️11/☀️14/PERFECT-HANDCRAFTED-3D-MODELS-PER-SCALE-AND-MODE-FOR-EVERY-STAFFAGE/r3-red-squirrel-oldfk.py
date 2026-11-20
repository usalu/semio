"""🔎 Round 3 red squirrel: prints joint, tip and anchor positions of every pose of the catalog asset (audit of the rig as found)."""
import json, sys, glob
sys.path.insert(0, ".")
from r3_red_squirrel_fk import chain, grounded, at, tip
f = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/red-squirrel/*.json")[0]
a = json.load(open(f))["asset"]
bones, anchors = a["rig"]["bones"], a["anchors"]
only = sys.argv[1:] 
for pose in [dict(id="REST", bones={}, ground="x")] + a["poses"]:
    if only and pose["id"] not in only: continue
    fr = grounded(bones, anchors, pose)
    print("==", pose["id"], pose.get("root"), pose.get("contacts"))
    for b in bones:
        o, t = fr[b["id"]]["o"], tip(fr[b["id"]])
        print(f"  {b['id']:10s} o {o[0]:+.4f} {o[1]:+.4f} {o[2]:+.4f} tip {t[0]:+.4f} {t[1]:+.4f} {t[2]:+.4f}")
    for an in anchors:
        p = at(fr[an["bone"]], an["x"], an["y"], an.get("z", 0))
        print(f"  @{an['id']:14s} {p[0]:+.4f} {p[1]:+.4f}")
