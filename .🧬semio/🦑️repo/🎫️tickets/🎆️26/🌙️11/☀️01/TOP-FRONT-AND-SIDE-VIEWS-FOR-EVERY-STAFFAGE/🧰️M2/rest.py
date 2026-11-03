import sys, math, os
sys.path.insert(0, os.path.dirname(__file__))
from lib import load


def fk(doc, pose=None):
    bones = {b["id"]: b for b in doc["asset"]["rig"]["bones"]}
    pb = (pose or {})
    out = {}

    def solve(i):
        if i in out:
            return out[i]
        b = bones[i]
        p = pb.get(i, {})
        ang = b["angleDeg"] + p.get("angleDeg", 0)
        x, y = b["x"], b["y"]
        if b["joint"]["type"] == "prismatic":
            ax = math.radians(b["joint"].get("axisDeg", 0))
            d = p.get("dx", 0)
            x += d * math.cos(ax)
            y += d * math.sin(ax)
        if b["parent"] is None:
            o = (x, y)
            a = ang
        else:
            po, pa = solve(b["parent"])
            c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
            o = (po[0] + c * x - s * y, po[1] + s * x + c * y)
            a = pa + ang
        out[i] = (o, a)
        return out[i]

    for i in bones:
        solve(i)
    res = {}
    for i, (o, a) in out.items():
        L = bones[i]["lengthM"]
        t = (o[0] + L * math.cos(math.radians(a)), o[1] + L * math.sin(math.radians(a)))
        res[i] = (o, a, t)
    return res


if __name__ == "__main__":
    doc = load(sys.argv[1])
    poses = {p["id"]: p.get("bones", {}) for p in doc["asset"]["poses"]}
    names = sys.argv[2:] or [None]
    for nm in names:
        print("POSE", nm)
        for i, (o, a, t) in fk(doc, poses.get(nm) if nm else None).items():
            print(f"  {i:14s} o=({o[0]:7.3f},{o[1]:7.3f}) ang={a:8.2f} tip=({t[0]:7.3f},{t[1]:7.3f})")
