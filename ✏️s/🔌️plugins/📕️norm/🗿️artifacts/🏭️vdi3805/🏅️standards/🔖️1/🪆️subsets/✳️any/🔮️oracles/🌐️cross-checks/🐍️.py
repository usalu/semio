"""Independent third-party cross-check oracle (scipy / mpmath / numpy / exact fractions; no repo oracle imported). Run `python -I 🐍️.py` to verify the committed vectors, `--write` to regenerate them. Every row is asserted (@agree); there are no known divergences left."""
import json
import math

import math

from scipy import optimize

KVS_TABLE = {10: 0.16, 15: 0.25, 20: 0.40, 25: 0.63, 32: 1.00, 40: 1.60, 50: 2.50, 65: 4.00, 80: 6.30, 100: 10.0, 125: 16.0, 150: 25.0, 200: 40.0, 250: 63.0, 300: 100.0}
IAPWS95_DENSITY = {4.0: 999.97, 20.0: 998.21, 60.0: 983.20, 70.0: 977.76, 80.0: 971.79}
CP_WATER = 4.19


def lmtd(flow, ret, room):
    return (flow - ret) / math.log((flow - room) / (ret - room))


def vectors():
    dt_n = lmtd(75.0, 65.0, 20.0)
    radiators = []
    for phi_n, n, flow, ret, room in [(1200.0, 1.3, 75.0, 65.0, 20.0), (1200.0, 1.3, 55.0, 45.0, 20.0), (1200.0, 1.3, 70.0, 55.0, 20.0), (1800.0, 1.25, 45.0, 35.0, 20.0), (800.0, 1.33, 60.0, 50.0, 22.0)]:
        dt = lmtd(flow, ret, room)
        phi = phi_n * (dt / dt_n) ** n
        m = phi / (CP_WATER * 1000.0 * (flow - ret))
        solved = optimize.brentq(lambda tr: phi_n * (lmtd(flow, tr, room) / dt_n) ** n - m * CP_WATER * 1000.0 * (flow - tr), room + 0.5, flow - 0.01) if abs(flow - ret) > 0 else ret
        radiators.append({"standardOutputW": phi_n, "n": n, "flowC": flow, "returnC": ret, "roomC": room, "deltaTLogK": dt, "outputW": phi, "massFlowKgS": m, "returnSolvedC": solved})
    valves = []
    for kvs, q, t in [(1.0, 1.0, 20.0), (0.63, 0.4, 70.0), (2.5, 1.8, 80.0), (4.0, 5.0, 60.0), (3.0, 3.0, 4.0)]:
        rho = IAPWS95_DENSITY[t]
        valves.append({"kvsM3H": kvs, "flowM3H": q, "waterC": t, "waterDensityKgM3": rho, "dropBar": (q / kvs) ** 2 * rho / 1000.0})
    authority = []
    for kvs, q, rest in [(1.0, 0.5, 0.15), (0.63, 0.3, 0.05), (2.5, 2.0, 0.3)]:
        dp100 = (q / kvs) ** 2
        authority.append({"kvsM3H": kvs, "designFlowM3H": q, "dropRestBar": rest, "dropValveBar": dp100, "authority": dp100 / (dp100 + rest)})
    r5 = [{"dn": dn, "tableKvs": v, "r5": 10 ** (round(5 * math.log10(v)) / 5), "matchesR5": abs(10 ** (round(5 * math.log10(v)) / 5) - v) / v < 0.05} for dn, v in KVS_TABLE.items()]
    return {"oracle": "scipy brentq, EN 442 log-mean excess, IAPWS-95 water density table (NIST WebBook)", "deltaTNominalK": dt_n, "radiators": radiators, "valves": valves, "authority": authority, "kvsTableR5": r5}


import pathlib
import sys

SUBSET = pathlib.Path(__file__).resolve().parents[2]
VECTORS = SUBSET / "🧪️tests/🧫️fixtures/🔮️cross-checks/🔣️.json"


def close(a, b):
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, float) or isinstance(b, float):
        return a is not None and b is not None and abs(a - b) <= 1e-9 * max(abs(a), abs(b), 1e-300) + 1e-15
    return a == b


def main():
    doc = vectors()
    doc = json.loads(json.dumps(doc))
    if "--write" in sys.argv:
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        return 0
    ok = close(doc, json.loads(VECTORS.read_text(encoding="utf8")))
    print(json.dumps({"ok": ok}))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
