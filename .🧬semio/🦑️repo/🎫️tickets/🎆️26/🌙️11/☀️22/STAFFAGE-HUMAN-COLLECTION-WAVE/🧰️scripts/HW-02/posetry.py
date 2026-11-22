"""🤸️ Pose experiment: applies bone offsets (JSON) to the relaxed pose of a test file and prints the frames of chosen bones (palm normal of hand-l = -w, hand-r = +w)."""
import json, sys, lib
src, offsets, show = sys.argv[1], json.loads(sys.argv[2]), sys.argv[3].split(",")
d = json.load(open(src)); a = d["asset"]
p = json.loads(json.dumps(next(x for x in a["poses"] if x["id"] == a["presentation"]["defaultPose"])))
p["id"] = "try"; p["bones"].update(offsets); a["poses"].append(p)
path = lib.write_tmp(d, "t-pose.json")
f = lib.frames(path, "try")
for b in show:
    fr = f[b]; fmt = lambda v: "(" + ", ".join(f"{v[k]:.3f}" for k in "xyz") + ")"
    print(f"{b:12s} o{fmt(fr['o'])} tip{fmt(fr['tip'])} u{fmt(fr['u'])} v{fmt(fr['v'])} w{fmt(fr['w'])}")
