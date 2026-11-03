import math, json
from lib import load
D = math.pi/180
def chain(bones, offs):
    fr = {}
    def solve(bid):
        if bid in fr: return fr[bid]
        b = bones[bid]; p = b["parent"]
        if p is None: par = ((0,0,0),(1,0,0),(0,1,0),(0,0,1))
        else: par = solve(p)
        o_, u, v, w = par
        e = offs.get(bid, {})
        ang = b["angleDeg"] + e.get("angleDeg", 0); ph = e.get("abductionDeg", 0)
        sh = tuple(u[i]*b["x"] + v[i]*b["y"] for i in range(3))
        o = tuple(o_[i]+sh[i] for i in range(3))
        ts, tc = math.sin(ang*D), math.cos(ang*D)
        u1 = tuple(u[i]*tc + v[i]*ts for i in range(3))
        v1 = tuple(-u[i]*ts + v[i]*tc for i in range(3))
        ps, pc = math.sin(ph*D), math.cos(ph*D)
        u2 = tuple(u1[i]*pc + w[i]*ps for i in range(3))
        w2 = tuple(-u1[i]*ps + w[i]*pc for i in range(3))
        tw = e.get("twistDeg", 0); ws, wc = math.sin(tw*D), math.cos(tw*D)
        v3 = tuple(v1[i]*wc - w2[i]*ws for i in range(3))
        w3 = tuple(v1[i]*ws + w2[i]*wc for i in range(3))
        fr[bid] = (o, u2, v3, w3)
        return fr[bid]
    for k in bones: solve(k)
    return {k: (f[0], tuple(f[0][i] + f[1][i]*bones[k]["lengthM"] for i in range(3))) for k, f in fr.items()}
def run(asset="agave", pose=None):
    d = load(asset)["asset"]
    bones = {b["id"]: b for b in d["rig"]["bones"]}
    for p in d["poses"]:
        if pose and p["id"] != pose: continue
        res = chain(bones, p["bones"])
        ys = {k: min(v[0][1], v[1][1]) for k, v in res.items() if k.startswith(("leaf", "petiole", "blade", "fr-"))}
        k = min(ys, key=ys.get)
        print(p["id"], "min bone y", round(ys[k], 3), k)
if __name__ == "__main__":
    run()
