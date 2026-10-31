"""CVF-VMC scratch: sweep shape parameters in memory and report which values pass the prefilter. usage in python: see bottom."""
import json, copy, itertools, sys
import prefilter as P
def load(f): return json.load(open(f))["asset"]
def sweep(f, den, edits, only=None):
    """edits: list of (shape id, key, [values]); evaluates the cartesian product, prints combos with fewest gaps."""
    a = load(f)
    res = []
    for combo in itertools.product(*[e[2] for e in edits]):
        b = copy.deepcopy(a)
        for (sid, key, _), v in zip(edits, combo):
            for s in b["representations"][str(den)]["geometry"]["elevation"]:
                if s["id"] == sid:
                    if isinstance(key, tuple):
                        s[key[0]][key[1]] = v
                    else:
                        s[key] = v
        r = P.measure(b, den, True, False)
        gaps = r["gap"]
        res.append((len(gaps), min([g[0] for g in gaps.values()] or [9]), combo, list(gaps)[:3]))
    res.sort(key=lambda t: (t[0], -t[1]))
    for t in res[:8]: print(t)
