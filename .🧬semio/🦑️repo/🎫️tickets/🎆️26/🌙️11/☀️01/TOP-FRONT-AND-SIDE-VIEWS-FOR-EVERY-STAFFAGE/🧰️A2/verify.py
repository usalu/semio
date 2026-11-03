import json, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import lib
from view import rng


def verify(aid, scale, show_all=False):
    A = lib.Asset(aid)
    ev = json.load(open(A.evalfile(scale), encoding="utf-8"))
    dims = A.a["dimensions"]
    tol = max(0.02, 0.03 * max(dims["widthM"], dims["depthM"], dims["heightM"]))
    bad = 0
    for q in ev:
        if "error" in q:
            print("ERR", q["label"], q["error"][:120])
            bad += 1
            continue
        r = {v: rng(q["views"][v]) for v in ("elevation", "front", "plan")}
        empty = [v for v in r if r[v] is None]
        if empty:
            print("EMPTY", q["label"], empty)
            bad += 1
            continue
        if q["label"] == A.dp:
            E, F, P = r["elevation"], r["front"], r["plan"]
            rows = [("E.x vs P.x", (E[0], E[1]), (P[0], P[1])), ("E.y vs F.y", (E[2], E[3]), (F[2], F[3])), ("F.x vs P.y", (F[0], F[1]), (P[2], P[3]))]
            for name, a, b in rows:
                d = max(abs(a[0] - b[0]), abs(a[1] - b[1]))
                print("  %s: %.3f..%.3f vs %.3f..%.3f  d=%.3f %s" % (name, a[0], a[1], b[0], b[1], d, "OK" if d <= tol else "FAIL"))
                bad += d > tol
            print("  ext W(E)=%.3f dims.w=%.3f | D(F)=%.3f D(P)=%.3f dims.d=%.3f | H(E)=%.3f H(F)=%.3f dims.h=%.3f tol=%.3f" % (E[1] - E[0], dims["widthM"], F[1] - F[0], P[3] - P[2], dims["depthM"], E[3] - E[2], F[3] - F[2], dims["heightM"], tol))
        elif show_all:
            print("  %-16s E x%.2f..%.2f y%.2f..%.2f | F x%.2f..%.2f y%.2f..%.2f | P x%.2f..%.2f y%.2f..%.2f" % ((q["label"],) + r["elevation"] + r["front"] + r["plan"]))
    print("verify", aid, scale, "problems:", bad)
    return bad


if __name__ == "__main__":
    verify(sys.argv[1], int(sys.argv[2]), len(sys.argv) > 3)
