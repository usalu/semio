import json
from abd_exact import run
from tipy import chain
AZ = {"R1": -10, "R4": 40, "R5": 62, "R2": 85, "L5": 108, "L3": 128, "L1": 168, "L4": 220, "L2": 272, "R3": 308}
chains = {n: (f"leaf-{n}" if n in ("R5", "L5") else f"leaf-{n}-a", None if n in ("R5", "L5") else f"leaf-{n}-b", az) for n, az in AZ.items()}
FLOOR = 0.14


def edit(a, bones):
    for bid, b in bones.items():
        if bid.endswith("-b") and bid.startswith("leaf-"):
            b["joint"]["minDeg"], b["joint"]["maxDeg"] = -30, 30
    for p in a["poses"]:
        if p["id"] not in ("drought-infolded", "frost-sag"):
            continue
        for bid, e in p["bones"].items():
            if bid == "leaf-S" or "angleDeg" not in e:
                continue
            sgn = 1 if bid.startswith("leaf-R") else -1
            isb = bid.endswith("-b")
            e["angleDeg"] = sgn * ((22.0 if isb else 18.0) if p["id"] == "drought-infolded" else (-14.0 if isb else -9.0))
    for p in a["poses"]:
        if p["id"] != "frost-sag":
            continue
        for n, (ia, ib, az) in chains.items():
            ids = [x for x in (ia, ib) if x]
            full = {b: p["bones"][b]["angleDeg"] for b in ids}
            for g in [i / 20 for i in range(20, -1, -1)]:
                for b in ids:
                    p["bones"][b]["angleDeg"] = round(full[b] * g, 3)
                res = chain(bones, p["bones"])
                if min(min(res[b][0][1], res[b][1][1]) for b in ids) >= FLOOR - 1e-9:
                    break


run("agave", "agave.before.json", chains, edit=edit)
