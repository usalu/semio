import math, re
from kit import *


def fix_path(d, ox, oy, ang):
    sa, ca = math.sin(ang * D2R), math.cos(ang * D2R)
    toks = re.findall(r"[MmLlCcQqZzHhVvAa]|-?\d*\.?\d+(?:[eE]-?\d+)?", d)
    out, nums, cmd = [], [], None
    i = 0
    res = []
    while i < len(toks):
        t = toks[i]
        if re.match(r"[A-Za-z]", t):
            cmd = t
            res.append(t)
            i += 1
            continue
        px, py = float(toks[i]), float(toks[i + 1])
        yw = oy + sa * px + ca * py
        if yw < 0:
            if abs(sa) >= abs(ca):
                px += -yw / sa
            else:
                py += -yw / ca
        res.append(f"{round(px, 4)} {round(py, 4)}")
        i += 2
    return "".join(r if re.match(r"[A-Za-z]$", r) else (" " + r + " ") for r in res).replace("  ", " ").strip()


def run(asset, ids):
    d = load(asset)
    r, b = rest(d)
    n = 0
    for sc, rep in d["asset"]["representations"].items():
        if rep.get("status") != "authored":
            continue
        for s in rep["geometry"]["elevation"]:
            if s["role"] == "ground" or s["bone"] not in ids:
                continue
            ox, oy, ang = r[s["bone"]]
            if s["kind"] == "path":
                if min([p[1] for p in world_poly(r, s)] or [0]) < -0.0005:
                    s["d"] = fix_path(s["d"], ox, oy, ang); n += 1
            elif s["kind"] == "poly":
                sa, ca = math.sin(ang * D2R), math.cos(ang * D2R)
                for p in s["points"]:
                    yw = oy + sa * p[0] + ca * p[1]
                    if yw < 0:
                        if abs(sa) >= abs(ca): p[0] = round(p[0] - yw / sa, 4)
                        else: p[1] = round(p[1] - yw / ca, 4)
                        n += 1
        for s in rep["geometry"]["front"]:
            if s["role"] == "ground" or s["bone"] not in ids:
                continue
            if s["kind"] == "poly":
                for p in s["points"]:
                    if p[1] < 0:
                        p[1] = 0.0; n += 1
    save(asset, d)
    print(asset, "fixed", n)


run("apple-tree", {"trunk"} | {"stem"})
run("hornbeam-hedge", {f"stem-{i}" for i in range(1, 7)})
run("bamboo-clump", {f"culm-{i}-a" for i in range(1, 7)})
