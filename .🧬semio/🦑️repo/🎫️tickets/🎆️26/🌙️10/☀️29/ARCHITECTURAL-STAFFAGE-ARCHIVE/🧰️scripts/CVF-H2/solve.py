"""Round-based solver: all violations of a round are turned into displacement fields (bridge = merge, clear = separate) on the lower-z shape,
applied to dense polygons, probed; kept when the score drops, else the step is damped."""
import sys, json, math, copy, collections
from io_ import *
from opt import probe_doc, score, total, shapes_of
import geom
def fields_for(viol, doc, scales, bridge_below=0.075, damp=1.0, only=None):
    per = collections.defaultdict(list)
    for v in viol:
        if v["type"] != "gap": continue
        if only and v["scale"] not in only: continue
        sc = v["scale"]; k = sc / 1000.0
        lowerA = v["zA"] <= v["zB"]
        who = "a" if lowerA else "b"
        sid = v["a"] if lowerA else v["b"]
        p0 = v["localA"] if lowerA else v["localB"]; p1 = v["localAtoB_A"] if lowerA else v["localBtoA_B"]
        dx, dy = p1["x"] - p0["x"], p1["y"] - p0["y"]; L = math.hypot(dx, dy)
        if L < 1e-9: continue
        ux, uy = dx / L, dy / L; g = v["gapMm"]
        amt = (v["centreMm"] + 0.05) if g < bridge_below else -(0.15 - g + 0.03)
        amt *= damp
        rho = max(0.3, 1.6 * abs(amt)) * k
        per[(sc, sid)].append(((p0["x"], p0["y"]), (ux * amt * k, uy * amt * k), rho))
    return per
def apply_fields(doc, per):
    for (sc, sid), fl in per.items():
        sh = shapes_of(doc, sc)[sid]; k = sc / 1000.0
        geom.deform(sh, fl, step=0.06 * k, tol=0.012 * k, max_move=0.45 * k)
def solve(name, scales, rounds=12, verbose=True, bridge_below=0.075):
    doc = load(name); cur = probe_doc(name, doc, scales)
    best = total(score(cur))
    if verbose: print(f"start {best:.1f}", dict(collections.Counter((v['scale'], v['type']) for v in cur)), flush=True)
    for r in range(rounds):
        improved = False
        for damp in (1.0, 0.6, 0.35):
            per = fields_for(cur, doc, scales, bridge_below, damp)
            if not per: break
            cand = copy.deepcopy(doc); apply_fields(cand, per)
            res = probe_doc(name, cand, scales); sc = total(score(res))
            if verbose: print(f"  round {r} damp {damp}: {best:.1f} -> {sc:.1f}", flush=True)
            if sc < best - 0.5:
                doc, cur, best = cand, res, sc; improved = True; break
        if not improved or best <= 0: break
    save(name, doc)
    c = collections.Counter((v["scale"], v["type"]) for v in cur)
    print("final", dict(c)); return cur
if __name__ == "__main__":
    name = sys.argv[1]; scales = [int(x) for x in sys.argv[2].split(",")]; rounds = int(sys.argv[3]) if len(sys.argv) > 3 else 12
    solve(name, scales, rounds)
