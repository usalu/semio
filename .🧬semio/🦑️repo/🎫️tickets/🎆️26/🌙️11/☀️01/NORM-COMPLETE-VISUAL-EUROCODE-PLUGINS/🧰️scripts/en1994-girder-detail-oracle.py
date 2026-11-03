"""Independent oracle for the EN 1994 bridge girder detail checks and the crack-width band.

Run: /home/deck/semio/.venv/bin/python -I en1994-girder-detail-oracle.py
Third-party: structuralcodes (f_cd), PyNite (support moment of the cracked two-span beam, through en1994-continuous-oracle.py); numpy flexibility method and the closed forms of EN 1992-1-1 6.7 (partially loaded area) for the rest.
Prints the reference values hard-coded in the Rust tests of the catalogue module `bridge` and the scene module `fields`.
"""
import importlib.util, math, os
import numpy as np
from structuralcodes.codes import ec2_2004 as ec2

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("continuous", os.path.join(HERE, "en1994-continuous-oracle.py"))
cont = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cont)


def end_plate_bearing(fcd, w, h, slab_b, slab_h):
    a0 = w * h
    a1 = max(min(3 * w, slab_b), w) * max(min(3 * h, slab_h), h)
    return min(a0 * fcd * math.sqrt(a1 / a0), 3 * fcd * a0)


def band(spans, ei1, ei2, frac, n=25):
    q = [1.0] * len(spans)
    sup = cont.solve(spans, q, ei1, ei2, frac)
    l = spans[0]

    def m(x):
        return q[0] * x * (l - x) / 2 + sup[0] * (1 - x / l) + sup[1] * x / l

    xs = np.linspace(0, l, 40001)
    ms = np.array([m(x) for x in xs])
    x0 = xs[np.where(ms[:-1] * ms[1:] <= 0)[0][-1]]
    half = l - x0
    pts = [x0 + 2 * half * k / (n - 1) for k in range(n)]
    right = spans[1]

    def m2(x):
        xl = x - l
        return q[1] * xl * (right - xl) / 2 + sup[1] * (1 - xl / right) + sup[2] * xl / right

    vals = [abs(m(x)) if x <= l else abs(m2(x)) for x in pts]
    peak = max(vals)
    return sup, x0, [round(v / peak, 6) for v in vals]


if __name__ == "__main__":
    fcd = ec2.fcd(30e6, 0.85, 1.5)
    print("fcd(30 MPa) =", fcd)
    for w, h, sb, sh in [(0.2, 0.1, 1.0, 0.25), (0.1, 0.1, 5.0, 5.0), (0.1, 0.1, 0.1, 0.1)]:
        print("end plate", (w, h, sb, sh), end_plate_bearing(fcd, w, h, sb, sh))
    E = 210e9
    ei1 = E * 6e-4
    sup, x0, vals = band([8.0, 8.0], ei1, 0.5 * ei1, 0.15)
    print("support moments (unit load)", [round(s, 4) for s in sup], "contraflexure x0 =", round(x0, 4))
    print("band profile k=0,6,12,18,24:", [vals[k] for k in (0, 6, 12, 18, 24)])
    fe, nodes = cont.pynite([8.0, 8.0], [1.0, 1.0], ei1, 0.5 * ei1, 0.15)
    mem = fe.members["E0_%d" % max(int(n.split("_")[1]) for n in fe.members if n.startswith("E0_"))]
    print("PyNite support moment", mem.moment("Mz", mem.L(), "Combo 1"))
