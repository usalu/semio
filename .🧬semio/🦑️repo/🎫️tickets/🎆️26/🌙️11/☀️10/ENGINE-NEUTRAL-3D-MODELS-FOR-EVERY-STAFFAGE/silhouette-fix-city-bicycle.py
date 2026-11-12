import sys, copy; sys.path.insert(0, ".")
from silhouette_edit import *
p = find("city-bicycle")
raw, doc, idx = load(p)
a = doc["asset"]
R = 0.014
def cap(old, id_, x0, y0, x1, y1):
    n = {k: v for k, v in old.items() if k not in ("solid", "penMm")}
    n.update({"id": id_, "kind": "capsule", "x0": x0, "y0": y0, "x1": x1, "y1": y1, "r0": R, "r1": R})
    for k in ("cx", "cy", "rx", "ry"): n.pop(k, None)
    return n
ops = []
for sc, r in a["representations"].items():
    g = r.get("geometry")
    if g is None: continue
    for v in ("front", "plan"):
        for k, s in enumerate(g[v]):
            base = ("asset", "representations", sc, "geometry", v, k)
            if v == "front" and s["id"] == "handlebar":
                ops.append(("replace", base, cap(s, "handlebar-l", 0.0, 1.066, 0.26, 1.045)))
                ops.append(("insert_after", base, [cap(s, "handlebar-r", 0.0, 1.066, -0.26, 1.045)]))
            elif v == "front" and s["id"] in ("grip-l", "grip-r"):
                n = copy.deepcopy(s); z0, z1 = n["solid"]["z0"], n["solid"]["z1"]
                if s["id"] == "grip-l": n["solid"]["z0"] = 0.26
                else: n["solid"]["z1"] = -0.26
                ops.append(("replace", base, n))
            elif v == "plan" and s["id"] in ("handlebar-l", "handlebar-r"):
                sgn = 1 if s["id"].endswith("l") else -1
                ops.append(("replace", base, cap(s, s["id"], 0.04, 0.0, -0.022, sgn * 0.26)))
            elif v == "plan" and s["id"] == "handlebar":
                ops.append(("replace", base, cap(s, "handlebar-l", 0.04, 0.0, -0.022, 0.26)))
                ops.append(("insert_after", base, [cap(s, "handlebar-r", 0.04, 0.0, -0.022, -0.26)]))
print(len(ops), apply(p, ops, expect=raw))
