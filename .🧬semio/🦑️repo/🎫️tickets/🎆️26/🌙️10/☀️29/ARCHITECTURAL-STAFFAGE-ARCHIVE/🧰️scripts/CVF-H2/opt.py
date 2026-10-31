"""Greedy accept/revert optimiser of print-suitability violations (gaps, slivers) of one asset at given scales.
Candidates are written to a scratch file, probed by the daemon, accepted only when the weighted violation score drops."""
import sys, json, math, copy, collections, os, time, random
from io_ import *
from job import submit
import geom
CAND = f"{T}/🗑️generated/CVF-H2/cand"; os.makedirs(CAND, exist_ok=True)
def probe_doc(name, doc, scales):
    f = f"{CAND}/{name}.json"
    json.dump(doc, open(f, "w"))
    sfile = f"{CAND}/{name}.struct.json"
    submit("probe", name, file=f, json=sfile, scales=scales, limit=1)
    return json.load(open(sfile))
def score(viol, weights=None):
    s = collections.Counter()
    for v in viol:
        if v["type"] == "gap": s["gap"] += 1 + (0.15 - v["gapMm"]) * 2
        elif v["type"] == "sliver": s["sliver"] += 3
        elif v["type"] == "feature": s["feature"] += 1
    return s
def total(s, kinds=("gap", "sliver")): return sum(s[k] for k in kinds)
def shapes_of(doc, sc): return {s["id"]: s for s in doc["asset"]["representations"][str(sc)]["geometry"]["elevation"]}
def actions(v, k):
    """candidate edits for a gap violation: (shape key 'a'|'b', vec in that shape's local frame, at, rho)"""
    out = []
    for who in ("lower", "upper"):
        lowerA = v["zA"] <= v["zB"]
        isA = (who == "lower") == lowerA
        p0 = v["localA"] if isA else v["localB"]; p1 = v["localAtoB_A"] if isA else v["localBtoA_B"]
        dx, dy = p1["x"] - p0["x"], p1["y"] - p0["y"]; L = math.hypot(dx, dy)
        if L < 1e-9: continue
        ux, uy = dx / L, dy / L
        for kind, amt in (("bridge", v["centreMm"] + 0.07), ("clear", -(0.16 - v["gapMm"] + 0.04))):
            for rho in (0.3, 0.7):
                out.append(("a" if isA else "b", (p0["x"], p0["y"]), (ux * amt * k, uy * amt * k), rho * k, f"{who}-{kind}-{rho}"))
    return out
def optimise(name, scales, max_trials=400, verbose=True):
    doc = load(name)
    cur = probe_doc(name, doc, scales); sc0 = score(cur)
    trials = 0; tried = set()
    log = []
    while trials < max_trials:
        gaps = sorted([v for v in cur if v["type"] == "gap"], key=lambda v: v["gapMm"])
        gaps = [v for v in gaps if (v["scale"], v["pose"], v["a"], v["b"]) not in tried]
        if not gaps: break
        v = gaps[0]; sc = v["scale"]; k = sc / 1000.0
        tried.add((sc, v["pose"], v["a"], v["b"]))
        base = total(score(cur))
        accepted = False
        for who, at, vec, rho, label in actions(v, k):
            cand = copy.deepcopy(doc)
            shp = shapes_of(cand, sc)
            sh = shp[v["a"] if who == "a" else v["b"]]
            geom.deform(sh, [(at, vec, rho)], step=0.06*k, tol=0.012*k, max_move=0.5*k)
            res = probe_doc(name, cand, scales); trials += 1
            if total(score(res)) < base - 0.5:
                doc, cur = cand, res; accepted = True
                log.append((sc, v["pose"], v["a"], v["b"], label, base, total(score(res))))
                if verbose: print(f"  ok 1:{sc} {v['pose']} {v['a']}+{v['b']} {label}: {base:.1f} -> {total(score(res)):.1f}", flush=True)
                break
        if not accepted and verbose: print(f"  -- 1:{sc} {v['pose']} {v['a']}+{v['b']} unresolved", flush=True)
    save(name, doc)
    return cur, log
if __name__ == "__main__":
    name = sys.argv[1]; scales = [int(x) for x in sys.argv[2].split(",")]; mt = int(sys.argv[3]) if len(sys.argv) > 3 else 400
    cur, log = optimise(name, scales, mt)
    c = collections.Counter((v["scale"], v["type"]) for v in cur)
    print(dict(c))
