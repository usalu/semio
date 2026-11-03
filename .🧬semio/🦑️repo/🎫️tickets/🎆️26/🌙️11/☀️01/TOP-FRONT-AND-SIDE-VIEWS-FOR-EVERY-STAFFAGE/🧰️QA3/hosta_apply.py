from abd_exact import run
from tipy import chain
AZ = {"R1": -12, "R2": 84, "R3": -52, "R4": 38, "C1": 0, "L1": 168, "L2": -96, "L3": 128, "L4": 205}
chains = {n: (f"petiole-{n}", f"blade-{n}", az) for n, az in AZ.items()}


def edit(a, bones):
    a["dimensions"]["depthM"] = 0.905
    floor = min(min(v[0][1], v[1][1]) for k, v in chain(bones, {}).items() if k.startswith(("petiole", "blade")))
    for p in a["poses"]:
        if p["id"] not in ("rain-bowed", "autumn-collapse"):
            continue
        for n, (ia, ib, az) in chains.items():
            ids = [ia, ib]
            full = {b: p["bones"][b]["angleDeg"] for b in ids}
            for g in [i / 20 for i in range(20, -1, -1)]:
                for b in ids:
                    p["bones"][b]["angleDeg"] = round(full[b] * g, 3)
                res = chain(bones, p["bones"])
                if min(min(res[b][0][1], res[b][1][1]) for b in ids) >= floor - 1e-9:
                    break


    def clamp(d):
        for n, (ia, ib, az) in chains.items():
            ids = [ia, ib]
            full = {b: d.get(b, {}).get("angleDeg", 0) for b in ids}
            for g in [i / 20 for i in range(20, -1, -1)]:
                for b in ids:
                    d.setdefault(b, {})["angleDeg"] = round(full[b] * g, 3)
                res = chain(bones, d)
                if min(min(res[b][0][1], res[b][1][1]) for b in ids) >= floor - 1e-9:
                    break
    for c in a["clips"]:
        for k in c["keys"]:
            clamp(k.setdefault("bones", {}))


run("hosta-clump", "hosta.before.json", chains, edit=edit)
