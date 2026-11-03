import math, json
from lib import load, save, rest
from tipy import chain
D = math.pi / 180


def dirv(e, az):
    return (math.cos(e * D) * math.cos(az * D), math.cos(e * D) * math.sin(az * D), math.sin(e * D))


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def run(asset, before, chains, edit_poses=None, floor_poses=(), floor=None):
    doc = json.load(open(before, encoding="utf8"))
    a = doc["asset"]
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    r, _ = rest(doc)
    stat, flip = {}, set()
    for n, (ia, ib, az) in chains.items():
        par = bones[ia]["parent"]
        pabs = r[par][2]
        al0 = r[ia][2] % 360
        side0 = 1 if al0 < 90 or al0 > 270 else -1
        ea = al0 if side0 > 0 else 180 - al0
        if ea > 180: ea -= 360
        da = dirv(ea, az)
        alpha = math.degrees(math.atan2(da[2], da[0]))
        phi = math.degrees(math.asin(max(-1, min(1, da[1]))))
        rel = alpha - pabs
        bones[ia]["angleDeg"] = round(rel, 4)
        stat[ia] = round(phi, 4)
        newside = 1 if alpha < 90 else -1
        if newside != (1 if (al0 - 0) % 360 < 90 or al0 % 360 > 270 else -1) and abs(alpha - 90) > 0.01:
            flip.add(ia)
        if ib:
            alb = r[ib][2] % 360
            eb = alb if side0 > 0 else 180 - alb
            if eb > 180: eb -= 360
            db = dirv(eb, az)
            ca, sa, cp, sp = math.cos(alpha * D), math.sin(alpha * D), math.cos(phi * D), math.sin(phi * D)
            ua, va, wa = (cp * ca, sp, cp * sa), (-sa, 0, ca), (-sp * ca, cp, -sp * sa)
            bones[ib]["angleDeg"] = round(math.degrees(math.atan2(dot(db, va), dot(db, ua))), 4)
            stat[ib] = round(math.degrees(math.asin(max(-1, min(1, dot(db, wa))))), 4)
            if ia in flip: flip.add(ib)
    def put(d):
        for bid, phi in stat.items():
            e = d.setdefault(bid, {})
            e["abductionDeg"] = phi
            if bid in flip and "angleDeg" in e:
                e["angleDeg"] = -e["angleDeg"]
    for p in a["poses"]:
        put(p["bones"])
    for c in a["clips"]:
        for k in c["keys"]:
            put(k.setdefault("bones", {}))
    if edit_poses:
        edit_poses(a, bones, stat, flip)
    if floor_poses:
        fl = floor if floor is not None else min(min(v[0][1], v[1][1]) for k, v in chain(bones, {b: {"abductionDeg": s} for b, s in stat.items()}).items() if k in stat)
        for p in a["poses"]:
            if p["id"] not in floor_poses:
                continue
            for n, (ia, ib, az) in chains.items():
                ids = [x for x in (ia, ib) if x]
                full = {b: p["bones"][b].get("angleDeg", 0) for b in ids}
                for g in [i / 20 for i in range(20, -1, -1)]:
                    for b in ids:
                        p["bones"][b]["angleDeg"] = round(full[b] * g, 3)
                    res = chain(bones, p["bones"])
                    if min(min(res[b][0][1], res[b][1][1]) for b in ids) >= fl - 1e-9:
                        break
    save(asset, doc)
    print("applied", asset, len(stat), "flipped", sorted(flip))
