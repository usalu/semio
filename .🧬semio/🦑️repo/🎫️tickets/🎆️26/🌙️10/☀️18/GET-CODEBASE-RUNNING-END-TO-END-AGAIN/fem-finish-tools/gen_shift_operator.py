#!/usr/bin/env python3
"""Generates the shift-operator corpus with NumPy: y = K^-1 B x + s x and K y = B x + s K x for small SPD K and indefinite B."""
import json, sys
import numpy as np
cases = [
    ("dense-three-two-columns", [[4,1,0],[1,3,1],[0,1,2]], [[2,-0.5,0],[-0.5,-1,0.25],[0,0.25,-3]], [[1,0],[0.5,1],[-0.25,2]], 1.7),
    ("tridiagonal-five-three-columns", [[2,-1,0,0,0],[-1,2,-1,0,0],[0,-1,2,-1,0],[0,0,-1,2,-1],[0,0,0,-1,2]], [[1,0.5,0,0,0],[0.5,-2,0.25,0,0],[0,0.25,0.5,-1,0],[0,0,-1,-3,0.5],[0,0,0,0.5,0]], [[1,0,0.5],[0,1,-0.5],[0.25,-1,1],[0.5,0.25,0],[-1,0.5,2]], 3.25),
    ("semidefinite-right-matrix", [[6,-2,0,0],[-2,5,-1,0],[0,-1,4,-1],[0,0,-1,3]], [[1,0,0,0],[0,0,0,0],[0,0,-2,0],[0,0,0,0]], [[1,2],[0,1],[1,-1],[0.5,0.5]], 0.75),
    ("zero-shift-leaves-operator-image", [[4,1,0],[1,3,1],[0,1,2]], [[2,-0.5,0],[-0.5,-1,0.25],[0,0.25,-3]], [[1,0],[0.5,1],[-0.25,2]], 0.0),
]
out = {"schemaVersion": 1, "oracle": f"NumPy {np.__version__} linalg.solve of K y = B x + s K x", "relativeTolerance": 1e-12, "cases": []}
for name, k, b, x, s in cases:
    K, B, X = (np.array(v, dtype=float) for v in (k, b, x))
    rhs = B @ X + s * (K @ X)
    solved = np.linalg.solve(K, B @ X) + s * X
    assert np.allclose(K @ solved, rhs, rtol=1e-13, atol=1e-13)
    out["cases"].append({"name": name, "order": len(k), "columns": len(x[0]), "shift": s, "stiffness": k, "right": b, "x": x, "solved": solved.tolist(), "rhs": rhs.tolist()})
head = {key: value for key, value in out.items() if key != "cases"}
text = "{\n" + "".join(f"  {json.dumps(key)}: {json.dumps(value)},\n" for key, value in head.items()) + '  "cases": [\n' + ",\n".join("    " + json.dumps(case) for case in out["cases"]) + "\n  ]\n}\n"
open(sys.argv[1], "w").write(text)
print(len(out["cases"]), "cases")
