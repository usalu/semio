# [scratch] dairy-cow pose solvers (IK) shared by the emit script; run directly for diagnostics
import sys, math, importlib.util
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import kin
F = sys.argv[1] if len(sys.argv) > 1 else None
a = kin.load(F, "dairy-cow") if F else None
def chains(s): return [f"thigh-h{s}", f"shank-h{s}", f"metatarsus-h{s}", f"hoof-h{s}"], [f"scapula-f{s}", f"humerus-f{s}", f"forearm-f{s}", f"cannon-f{s}", f"hoof-f{s}"]
def hind(s, x, y, base, root, m, p, aid=None):
    h, _ = chains(s)
    return a.solve_limb(base, h, aid or f"contact-h{s}", (x, y), {h[0]:("solve",), h[1]:("solve",), h[2]:("abs",m), h[3]:("abs",p)}, root=root)
def fore(s, x, y, base, root, m, p, sc=0, aid=None):
    _, f = chains(s)
    return a.solve_limb(base, f, aid or f"contact-f{s}", (x, y), {f[0]:("off",sc), f[1]:("solve",), f[2]:("solve",), f[3]:("abs",m), f[4]:("abs",p)}, root=root)
def fit(base, root, free, targets, prior=None, w=1e-4, iters=400):
    """coordinate descent on named bone offsets; targets: list of (anchor, 'x'|'y'|'xy', value(s)); returns offsets and rms error."""
    v = {k: min(max(prior.get(k, 0) if prior else 0, a.limits(k)[0]), a.limits(k)[1]) for k in free}
    def obj(v):
        w_ = a.fk({**base, **v}, root); e = 0
        for aid, ax, val in targets:
            p = a.anchor(w_, aid)
            if ax == "x": e += (p[0]-val)**2
            elif ax == "y": e += (p[1]-val)**2
            else: e += (p[0]-val[0])**2 + (p[1]-val[1])**2
        return e + w*sum((v[k]-(prior or {}).get(k, 0))**2 for k in free)
    best = obj(v); step = 8.0
    for _ in range(iters):
        imp = False
        for k in free:
            for d in (step, -step):
                n = dict(v); n[k] = min(max(v[k]+d, a.limits(k)[0]), a.limits(k)[1]); o = obj(n)
                if o < best - 1e-14: best, v, imp = o, n, True
        if not imp:
            step /= 2
            if step < 1e-4: break
    w_ = a.fk({**base, **v}, root)
    return v, {aid: tuple(round(c, 4) for c in a.anchor(w_, aid)) for aid, _, _ in targets}
