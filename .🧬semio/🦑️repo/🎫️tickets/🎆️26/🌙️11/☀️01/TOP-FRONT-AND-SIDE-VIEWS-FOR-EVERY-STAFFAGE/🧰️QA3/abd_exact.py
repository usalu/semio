import math, json
from lib import save, rest
D = math.pi / 180


def mat_cols(u, v, w):
    return (u, v, w)


def dotv(a, b):
    return sum(x * y for x, y in zip(a, b))


def az_rot(az, vec):
    c, s = math.cos(az * D), math.sin(az * D)
    x, y, z = vec
    return (x * c - z * s, y, x * s + z * c)


def parent_angle(bones, offs, pid):
    ang = 0.0
    cur = pid
    while cur is not None:
        b = bones[cur]
        ang += b["angleDeg"] + offs.get(cur, {}).get("angleDeg", 0)
        cur = b["parent"]
    return ang


def solve_offsets(P_ang, rel_total, az):
    ps, pc = math.sin(P_ang * D), math.cos(P_ang * D)
    Pu, Pv, Pw = (pc, ps, 0), (-ps, pc, 0), (0, 0, 1)
    t, tc = math.sin(rel_total * D), math.cos(rel_total * D)
    u1 = tuple(Pu[i] * tc + Pv[i] * t for i in range(3))
    v1 = tuple(-Pu[i] * t + Pv[i] * tc for i in range(3))
    w = Pw
    ut, vt = az_rot(az, u1), az_rot(az, v1)
    phi = math.asin(max(-1, min(1, dotv(ut, w))))
    delta = math.atan2(dotv(ut, Pv), dotv(ut, Pu))
    d_, ds = math.cos(delta), math.sin(delta)
    v1p = tuple(-Pu[i] * ds + Pv[i] * d_ for i in range(3))
    u1p = tuple(Pu[i] * d_ + Pv[i] * ds for i in range(3))
    w2 = tuple(-u1p[i] * math.sin(phi) + w[i] * math.cos(phi) for i in range(3))
    tau = math.atan2(-dotv(vt, w2), dotv(vt, v1p))
    return math.degrees(delta), math.degrees(phi), math.degrees(tau)


def run(asset, before, chains, edit=None, margin=5):
    doc = json.load(open(before, encoding="utf8"))
    a = doc["asset"]
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    big = {}
    def put(d):
        for n, (ia, ib, az) in chains.items():
            e = d.get(ia, {})
            rel_total = bones[ia]["angleDeg"] + e.get("angleDeg", 0)
            P = parent_angle(bones, d, bones[ia]["parent"])
            abs0 = (parent_angle(bones, {}, bones[ia]["parent"]) + bones[ia]["angleDeg"]) % 360
            rot = az if (abs0 < 90 or abs0 > 270) else az - 180
            dl, ph, ta = solve_offsets(P, rel_total, rot)
            off = dl - bones[ia]["angleDeg"]
            off = (off + 180) % 360 - 180
            ne = dict(e)
            ne["angleDeg"] = round(off, 4)
            ne["abductionDeg"] = round(ph, 4)
            ne["twistDeg"] = round(ta, 4)
            d[ia] = ne
            big[ia] = max(big.get(ia, 0), abs(off))
    if edit:
        edit(a, bones)
    for p in a["poses"]:
        put(p["bones"])
    for c in a["clips"]:
        for k in c["keys"]:
            put(k.setdefault("bones", {}))
    for ia, m in big.items():
        lim = max(bones[ia]["joint"].get("maxDeg", 0), math.ceil((m + margin) / 5) * 5)
        bones[ia]["joint"]["minDeg"] = -lim
        bones[ia]["joint"]["maxDeg"] = lim
    save(asset, doc)
    print("applied", asset, {k: v for k, v in big.items()})
