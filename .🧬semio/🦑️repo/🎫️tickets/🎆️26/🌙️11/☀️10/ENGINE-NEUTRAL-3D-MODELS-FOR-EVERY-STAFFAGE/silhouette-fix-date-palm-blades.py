import sys, re; sys.path.insert(0, ".")
from silhouette_edit import *
def sample(d):
    toks = re.findall(r"[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?", d)
    pts = []; i = 0; cur = None; cmd = None
    while i < len(toks):
        t = toks[i]
        if t in "MLCQZ": cmd = t; i += 1; continue
        if cmd == "M" or cmd == "L":
            cur = (float(toks[i]), float(toks[i + 1])); i += 2; pts.append(cur)
        elif cmd == "C":
            p = [(float(toks[i + k]), float(toks[i + k + 1])) for k in (0, 2, 4)]; i += 6
            for s in range(1, 21):
                u = s / 20; a, b, c, e = cur, p[0], p[1], p[2]
                pts.append(tuple((1 - u) ** 3 * a[k] + 3 * (1 - u) ** 2 * u * b[k] + 3 * (1 - u) * u * u * c[k] + u ** 3 * e[k] for k in (0, 1)))
            cur = p[2]
        elif cmd == "Q":
            p = [(float(toks[i + k]), float(toks[i + k + 1])) for k in (0, 2)]; i += 4
            for s in range(1, 11):
                u = s / 10; a, b, c = cur, p[0], p[1]
                pts.append(tuple((1 - u) ** 2 * a[k] + 2 * (1 - u) * u * b[k] + u * u * c[k] for k in (0, 1)))
            cur = p[1]
        else: i += 1
    return pts
raw0, doc0, idx0 = load(find("date-palm"))
g = doc0["asset"]["representations"]["50"]["geometry"]["elevation"]
W = {}
for s in g:
    if s["id"].startswith("fr-") and s["id"].endswith("-blade") and s["kind"] == "path": W[s["id"]] = sample(s["d"])
def half(pts, u):
    near = [y for x, y in pts if abs(x - u) <= 0.03]
    if not near: return None
    return (max(near) - min(near)) / 2
def mutate(solids):
    for s in solids:
        if s["id"] in W and s["kind"] == "loft":
            pts = W[s["id"]]
            for sec in s["sections"]:
                h = half(pts, sec["at"][0])
                if h is not None: sec["rx"] = round(max(h, 0.004), 5)
print(patch_volume("date-palm", mutate))
