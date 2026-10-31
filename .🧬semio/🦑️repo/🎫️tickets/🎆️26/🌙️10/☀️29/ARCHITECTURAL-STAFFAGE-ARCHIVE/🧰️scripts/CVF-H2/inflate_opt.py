"""coordinate descent over per-shape outward offsets (paper mm) at one scale; objective = weighted violations (gaps, slivers, features) from the daemon probe"""
import sys, copy, collections, json
from io_ import *
from opt import probe_doc, shapes_of
import geom
def obj(res, sc):
    c = collections.Counter(v["type"] for v in res if v["scale"] == sc)
    return c["gap"] + 3 * c["sliver"] + 2 * c["feature"]
def run(name, sc, steps=(-0.06, 0.06, 0.12, -0.12, 0.2), sweeps=3, ids=None):
    doc = load(name); res = probe_doc(name, doc, [sc]); best = obj(res, sc); k = sc/1000
    print("start", best, flush=True)
    base = {s["id"]: copy.deepcopy(s) for s in shapes_of(doc, sc).values()}
    delta = {i: 0.0 for i in base}
    for sw in range(sweeps):
        for sid in (ids or list(base)):
            for st in steps:
                cand = copy.deepcopy(doc); sh = shapes_of(cand, sc)[sid]
                nd = delta[sid] + st
                if abs(nd) > 0.35: continue
                sh.clear(); sh.update(copy.deepcopy(base[sid])); geom.inflate(sh, nd*k, 0.05*k, 0.01*k)
                r = probe_doc(name, cand, [sc]); o = obj(r, sc)
                if o < best:
                    best, doc, delta[sid] = o, cand, nd; print(f"  {sid} {nd:+.2f}: {best}", flush=True)
        print("sweep", sw, best, flush=True)
    save(name, doc); return best
if __name__ == "__main__":
    run(sys.argv[1], int(sys.argv[2]))
