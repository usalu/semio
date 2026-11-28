#!/usr/bin/env python3
"""Independent oracle of the H93 host cache budget allocator (written from the spec in
design-H93-host-cache-budget.md, not from allocator.ts). Usage: oracle.py gen N SEED > corpus.json"""
import json, random, sys
from fractions import Fraction

GIB = 2 ** 30
AGES = [300000, 2100000, 12600000, 54000000, 345600000, 1209600000]


def commit(u, q, pol):
    raw = u + (u * pol["headroomNum"]) // pol["headroomDen"]
    a = q["alignBytes"]
    v = max(q["floorBytes"], raw)
    return -(-v // a) * a


def plan(case):
    host, pol, ps = case["host"], case["policy"], case["purposes"]
    phys = sum(i["physicalBytes"] for q in ps for i in q["images"])
    reserve = max(pol["reserveFloorBytes"], host["totalBytes"] * pol["reserveFractionPercent"] // 100)
    budget = max(0, min(host["totalBytes"] * pol["maxSharePercent"] // 100, phys + host["freeBytes"] - reserve))
    used = {q["name"]: sum(q["buckets"]) for q in ps}
    before = dict(used)
    byname = {q["name"]: q for q in ps}
    left = {(q["name"], b): q["buckets"][b] for q in ps for b in range(1, 6)}

    def total():
        return sum(commit(used[n], byname[n], pol) for n in byname) if ps else 0

    order = sorted(left, key=lambda k: (-Fraction(AGES[k[1]], byname[k[0]]["costMsPerGiB"]), k[0], -k[1]))
    evict = []
    hn, hd = pol["headroomNum"], pol["headroomDen"]
    for key in order:
        name, b = key
        while total() > budget and left[key] > 0:
            deficit = total() - budget
            e = min(left[key], max(byname[name]["alignBytes"], -(-(deficit * hd) // (hd + hn))))
            left[key] -= e
            used[name] -= e
            if evict and evict[-1][0] == name and evict[-1][1] == b:
                evict[-1][2] += e
            else:
                evict.append([name, b, e])
    after = total()
    commits = {n: commit(used[n], byname[n], pol) for n in byname}
    extra = {n: max(0, commit(max(byname[n]["peakBytes"], used[n]), byname[n], pol) - commits[n]) for n in byname}
    se = sum(extra.values())
    spare = max(0, budget - after)
    target = {}
    for n in byname:
        if se == 0 or spare == 0:
            target[n] = commits[n]
        else:
            share = extra[n] if se <= spare else (spare * extra[n]) // se
            a = byname[n]["alignBytes"]
            target[n] = commits[n] + (min(share, extra[n]) // a) * a
    actions, outp = [], []
    for q in ps:
        n = q["name"]
        if q["imageless"]:
            actions.append(["quota", n, "-", before[n], target[n]])
            outp.append((n, before[n], used[n], commits[n], target[n], []))
            continue
        a = q["alignBytes"]
        scap = (pol["imageCapBytes"] * pol["shardFillNum"] // pol["shardFillDen"]) // a * a
        k = max(1, -(-target[n] // scap))
        units = target[n] // a
        sizes = [(units // k + (1 if j < units % k else 0)) * a for j in range(k)]
        outp.append((n, before[n], used[n], commits[n], target[n], sizes))
        imgs = sorted(q["images"], key=lambda i: i["id"])
        if any(i["capacityBytes"] > pol["imageCapBytes"] for i in imgs):
            actions += [["retire", n, i["id"], i["capacityBytes"], 0] for i in imgs]
            actions += [["birth", n, "%s#%d" % (n, j), 0, s] for j, s in enumerate(sizes)]
            continue
        for j, s in enumerate(sizes):
            if j >= len(imgs):
                actions.append(["birth", n, "%s#%d" % (n, j), 0, s])
                continue
            im = imgs[j]
            c = im["capacityBytes"]
            if s > c:
                actions.append(["grow", n, im["id"], c, s])
            elif im["unmappable"] and c * 2 > s * 3 and c - s >= 2 * a:
                actions.append(["shrink", n, im["id"], c, s])
            else:
                actions.append(["keep", n, im["id"], c, c])
        actions += [["retire", n, i["id"], i["capacityBytes"], 0] for i in imgs[len(sizes):]]
    return {
        "budgetBytes": budget, "reserveBytes": reserve, "physicalNowBytes": phys,
        "unmet": after > budget, "deficitBytes": max(0, after - budget),
        "evictions": [{"purpose": e[0], "bucket": e[1], "bytes": e[2]} for e in evict],
        "purposes": [{"name": o[0], "usedBeforeBytes": o[1], "usedAfterBytes": o[2], "commitBytes": o[3], "targetBytes": o[4], "shards": o[5]} for o in outp],
        "actions": [{"kind": x[0], "purpose": x[1], "image": x[2], "fromBytes": x[3], "toBytes": x[4]} for x in actions],
    }


def policy(cap=32 * GIB):
    return {"reserveFloorBytes": 10 * GIB, "reserveFractionPercent": 8, "maxSharePercent": 60, "headroomNum": 1,
            "headroomDen": 4, "imageCapBytes": cap, "shardFillNum": 7, "shardFillDen": 8}


def gen(n, seed):
    rnd = random.Random(seed)
    names = ["r", "d", "g", "l", "n", "o", "p", "t", "cas"]
    cases = []
    for c in range(n):
        cap = rnd.choice([32, 64, 128]) * GIB
        total = rnd.choice([64, 128, 228, 460, 926, 2000]) * GIB
        ps = []
        for name in rnd.sample(names, rnd.randint(1, 9)):
            align = rnd.choice([64 * 2 ** 20, 256 * 2 ** 20, GIB])
            scale = rnd.choice([GIB // 8, GIB, 4 * GIB, 20 * GIB])
            buckets = [rnd.randint(0, 3) * rnd.randint(0, 10) * scale // 10 // 4096 * 4096 for _ in range(6)]
            if rnd.random() < 0.15:
                buckets = [0] * 6
            imageless = name == "cas" or rnd.random() < 0.03
            imgs = []
            if not imageless:
                for k in range(rnd.randint(0, 3)):
                    capimg = rnd.randint(1, 120) * GIB
                    u = rnd.randint(0, capimg // 2)
                    imgs.append({"id": "%s%d" % (name, k), "capacityBytes": capimg, "usedBytes": u,
                                 "physicalBytes": u + rnd.randint(0, u // 3 + 1), "unmappable": capimg <= cap and rnd.random() < 0.8})
            ps.append({"name": name, "floorBytes": rnd.choice([0, 64 * 2 ** 20, GIB, 4 * GIB]), "alignBytes": align,
                       "costMsPerGiB": rnd.choice([1, 10, 1000, 30000, 120000, 600000, 1000000]),
                       "peakBytes": sum(buckets) * rnd.choice([1, 1, 2]), "imageless": imageless, "buckets": buckets, "images": imgs})
        free = rnd.choice([0, 3 * GIB, 12 * GIB, 40 * GIB, total // 3, total // 2])
        cases.append({"host": {"totalBytes": total, "freeBytes": free}, "policy": policy(cap), "purposes": ps})
    return [{"input": c, "expected": plan(c)} for c in cases]


if __name__ == "__main__":
    if sys.argv[1] == "gen":
        json.dump(gen(int(sys.argv[2]), int(sys.argv[3])), sys.stdout, separators=(",", ":"))
