import json, os, glob, math, sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\🧍️staffage"


def path_of(asset):
    return glob.glob(os.path.join(ROOT, "🗂️catalog", "*", asset, "🔣️.json"))[0]


def load(asset):
    with open(path_of(asset), encoding="utf-8") as f:
        return json.load(f)


def save(asset, doc):
    with open(path_of(asset), "w", encoding="utf-8", newline="\n") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
        f.write("\n")


def rest(asset_doc):
    a = asset_doc["asset"]
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    out = {}

    def solve(bid):
        if bid in out:
            return out[bid]
        b = bones[bid]
        if b["parent"] is None:
            px, py, pa = 0.0, 0.0, 0.0
        else:
            px, py, pa = solve(b["parent"])
        c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
        x = px + c * b["x"] - s * b["y"]
        y = py + s * b["x"] + c * b["y"]
        out[bid] = (x, y, pa + b["angleDeg"])
        return out[bid]

    for k in bones:
        solve(k)
    return out, bones


def tip(r, bones, bid):
    x, y, a = r[bid]
    L = bones[bid]["lengthM"]
    return (x + L * math.cos(math.radians(a)), y + L * math.sin(math.radians(a)))


def path_points(d):
    import re
    toks = re.findall(r"[MmLlHhVvCcQqZzAa]|-?\d*\.?\d+(?:[eE]-?\d+)?", d)
    pts, i, cmd, cx, cy, sx, sy = [], 0, None, 0.0, 0.0, 0.0, 0.0

    def num():
        nonlocal i
        v = float(toks[i])
        i += 1
        return v

    def bez(p0, ps):
        n = 10
        out = []
        for k in range(1, n + 1):
            t = k / n
            if len(ps) == 3:
                a, b, c = ps
                x = (1 - t) ** 3 * p0[0] + 3 * (1 - t) ** 2 * t * a[0] + 3 * (1 - t) * t * t * b[0] + t ** 3 * c[0]
                y = (1 - t) ** 3 * p0[1] + 3 * (1 - t) ** 2 * t * a[1] + 3 * (1 - t) * t * t * b[1] + t ** 3 * c[1]
            else:
                a, b = ps
                x = (1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * a[0] + t * t * b[0]
                y = (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * a[1] + t * t * b[1]
            out.append((x, y))
        return out

    while i < len(toks):
        if toks[i].isalpha():
            cmd = toks[i]
            i += 1
            if cmd in "Zz":
                cx, cy = sx, sy
                continue
        rel = cmd.islower()
        c = cmd.upper()
        ox, oy = (cx, cy) if rel else (0.0, 0.0)
        if c == "M":
            cx, cy = num() + ox, num() + oy
            sx, sy = cx, cy
            pts.append((cx, cy))
            cmd = "l" if rel else "L"
        elif c == "L":
            cx, cy = num() + ox, num() + oy
            pts.append((cx, cy))
        elif c == "H":
            cx = num() + ox
            pts.append((cx, cy))
        elif c == "V":
            cy = num() + oy
            pts.append((cx, cy))
        elif c == "C":
            a = (num() + ox, num() + oy)
            b = (num() + ox, num() + oy)
            e = (num() + ox, num() + oy)
            pts += bez((cx, cy), [a, b, e])
            cx, cy = e
        elif c == "Q":
            a = (num() + ox, num() + oy)
            e = (num() + ox, num() + oy)
            pts += bez((cx, cy), [a, e])
            cx, cy = e
        elif c == "A":
            for _ in range(5):
                num()
            cx, cy = num() + ox, num() + oy
            pts.append((cx, cy))
    return pts


def world_poly(r, shape):
    x, y, a = r[shape["bone"]]
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    if shape["kind"] == "path":
        pts = [list(p) for p in path_points(shape["d"])]
    elif shape["kind"] == "poly":
        pts = shape["points"]
    elif shape["kind"] == "rect":
        pts = [[shape["x"], shape["y"]], [shape["x"] + shape["w"], shape["y"]], [shape["x"] + shape["w"], shape["y"] + shape["h"]], [shape["x"], shape["y"] + shape["h"]]]
    elif shape["kind"] == "ellipse":
        pts = [[shape["cx"] + shape["rx"] * math.cos(t * math.pi / 8), shape["cy"] + shape["ry"] * math.sin(t * math.pi / 8)] for t in range(16)]
    elif shape["kind"] == "capsule":
        pts = [[shape["x0"] - shape["r0"], shape["y0"] - shape["r0"]], [shape["x0"] + shape["r0"], shape["y0"] + shape["r0"]], [shape["x1"] - shape["r1"], shape["y1"] - shape["r1"]], [shape["x1"] + shape["r1"], shape["y1"] + shape["r1"]]]
    else:
        return []
    return [(x + c * px - s * py, y + s * px + c * py) for px, py in pts]


def bbox(pts):
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    return (min(xs), min(ys), max(xs), max(ys))


def r4(v):
    return round(v, 4)


def poly(id, bone, role, z, tone, pts, region=None, regions=None, closed=True, followTip=False, material=None, **kw):
    s = {"id": id, "bone": bone, "role": role, "z": round(z, 4), "filled": closed and role not in ("texture",), "tone": tone, "kind": "poly", "points": [[r4(x), r4(y)] for x, y in pts], "closed": closed}
    if followTip:
        s["followTip"] = True
    return finish(s, region, regions, material, kw)


def rect(id, bone, role, z, tone, x, y, w, h, region=None, regions=None, followTip=False, material=None, r=None, **kw):
    s = {"id": id, "bone": bone, "role": role, "z": round(z, 4), "filled": True, "tone": tone, "kind": "rect", "x": r4(x), "y": r4(y), "w": r4(w), "h": r4(h)}
    if r:
        s["r"] = r4(r)
    if followTip:
        s["followTip"] = True
    return finish(s, region, regions, material, kw)


def ellipse(id, bone, role, z, tone, cx, cy, rx, ry, region=None, regions=None, rot=None, material=None, **kw):
    s = {"id": id, "bone": bone, "role": role, "z": round(z, 4), "filled": True, "tone": tone, "kind": "ellipse", "cx": r4(cx), "cy": r4(cy), "rx": r4(rx), "ry": r4(ry)}
    if rot:
        s["rotationDeg"] = rot
    return finish(s, region, regions, material, kw)


def capsule(id, bone, role, z, tone, x0, y0, x1, y1, r0, r1, region=None, regions=None, followTip=False, material=None, **kw):
    s = {"id": id, "bone": bone, "role": role, "z": round(z, 4), "filled": True, "tone": tone, "kind": "capsule", "x0": r4(x0), "y0": r4(y0), "x1": r4(x1), "y1": r4(y1), "r0": r4(r0), "r1": r4(r1)}
    if followTip:
        s["followTip"] = True
    return finish(s, region, regions, material, kw)


def finish(s, region, regions, material, kw):
    if region:
        s["region"] = region
    if regions:
        s["regions"] = regions
    if material:
        s["material"] = material
    for k, v in kw.items():
        if v is not None:
            s[k] = v
    return s


def lobed(cx, cy, rx, ry, n, depth, phase=0.0, seed=0, steps=None, rot=0.0):
    steps = steps or n * 4
    import random
    rnd = random.Random(seed)
    jit = [1 + (rnd.random() - 0.5) * 0.25 * depth for _ in range(n)]
    pts = []
    for i in range(steps):
        t = 2 * math.pi * i / steps
        lobe = 1 - depth + depth * (0.5 + 0.5 * math.cos(n * t + phase))
        k = jit[int(i * n / steps) % n]
        px, py = rx * lobe * k * math.cos(t), ry * lobe * k * math.sin(t)
        c, s = math.cos(rot), math.sin(rot)
        pts.append((cx + c * px - s * py, cy + s * px + c * py))
    return pts


def ring_poly(cx, cy, rx, ry, n=24, rot=0.0):
    out = []
    for i in range(n):
        t = 2 * math.pi * i / n
        px, py = rx * math.cos(t), ry * math.sin(t)
        c, s = math.cos(rot), math.sin(rot)
        out.append((cx + c * px - s * py, cy + s * px + c * py))
    return out


def blade(x0, y0, x1, y1, w0, wm, w1=0.0, n=7):
    dx, dy = x1 - x0, y1 - y0
    L = math.hypot(dx, dy)
    ux, uy = dx / L, dy / L
    nx, ny = -uy, ux
    left, right = [], []
    for i in range(n + 1):
        t = i / n
        w = (w0 * (1 - t) + w1 * t) * 1.0
        w = max(w, wm * math.sin(math.pi * min(1, t * 1.15)) if t < 0.87 else w)
        w = max(w, 0)
        px, py = x0 + dx * t, y0 + dy * t
        left.append((px + nx * w, py + ny * w))
        right.append((px - nx * w, py - ny * w))
    return left + right[::-1]


def report(asset):
    d = load(asset)
    r, bones = rest(d)
    a = d["asset"]
    print("==", asset, a["dimensions"], "default", a["presentation"]["defaultPose"], a["presentation"]["defaultScale"])
    for b in bones:
        x, y, ang = r[b]
        tx, ty = tip(r, bones, b)
        print(f"  {b:16s} parent={str(bones[b]['parent']):14s} o=({x:6.2f},{y:6.2f}) ang={ang:7.1f} L={bones[b]['lengthM']:5.2f} tip=({tx:6.2f},{ty:6.2f}) joint={bones[b]['joint']['type']}")
    for sc, rep in a["representations"].items():
        if rep.get("status") != "authored":
            continue
        pts = []
        for s in rep["geometry"]["elevation"]:
            if s["role"] != "ground":
                pts += world_poly(r, s)
        print("  scale", sc, "elev bbox", [round(v, 2) for v in bbox(pts)] if pts else None, "n", len(rep["geometry"]["elevation"]), "pens", rep["pens"])
    print("  regions:", sorted({s.get("region") or tuple(s.get("regions", [])) for rep in a["representations"].values() if rep.get("status") == "authored" for s in rep["geometry"]["elevation"]}, key=str))
    print("  symbols:", json.dumps(a.get("symbols"), ensure_ascii=False)[:600])
    print("  materials:", [m["id"] for m in a["materials"]])


if __name__ == "__main__":
    for x in sys.argv[1:]:
        report(x)
