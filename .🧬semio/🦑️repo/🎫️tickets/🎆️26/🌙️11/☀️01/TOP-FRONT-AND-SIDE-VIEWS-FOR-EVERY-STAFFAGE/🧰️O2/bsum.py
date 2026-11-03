import json, sys
p = sys.argv[1]
d = json.load(open(p, encoding="utf-8"))
only = sys.argv[2:]
for a, v in d.items():
    if only and a not in only:
        continue
    dm = v["dimensions"]
    print("==", a, "dims w/d/h", dm["widthM"], dm["depthM"], dm["heightM"], "pose", v["defaultPose"])
    for s, sc in v["scales"].items():
        e, f, pl = sc["elevation"], sc["front"], sc["plan"]
        def rng(b, i):
            return None if b is None else round(b[i + 1] - b[i], 4)
        print(" ", s, "E x[%s] y[%s] | F x[%s] y[%s] | P x[%s] y[%s]" % tuple(
            ("-" if b is None else "%.3f..%.3f" % (b[i], b[i + 1])) for b in (e, f, pl) for i in (0, 2)))
        if f and pl and e:
            tol = max(0.02, 0.03 * max(dm["widthM"], dm["depthM"], dm["heightM"]))
            bad = []
            if abs(e[0] - pl[0]) > tol or abs(e[1] - pl[1]) > tol: bad.append("Ex!=Px")
            if abs(e[2] - f[2]) > tol or abs(e[3] - f[3]) > tol: bad.append("Ey!=Fy")
            if abs(f[0] - pl[2]) > tol or abs(f[1] - pl[3]) > tol: bad.append("Fx!=Py")
            if abs((e[1] - e[0]) - dm["widthM"]) > tol: bad.append("w")
            if abs((f[1] - f[0]) - dm["depthM"]) > tol: bad.append("d")
            if abs((e[3] - e[2]) - dm["heightM"]) > tol: bad.append("h")
            print("    tol %.3f" % tol, bad or "OK")
        empties = [k for k, vv in sc["all"].items() if any(vv[w] is None for w in ("elevation", "front", "plan"))]
        if empties and f:
            print("    EMPTY at", empties[:6])
