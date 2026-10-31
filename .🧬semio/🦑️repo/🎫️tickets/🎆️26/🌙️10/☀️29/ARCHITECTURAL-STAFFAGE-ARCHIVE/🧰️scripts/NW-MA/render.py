"""Scratch SVG/PNG renderer of one authored representation (pose offsets applied through the rig) for quick looks; the official preview CLI is the reference."""
import re, subprocess, sys, math
from lib import *


def flat(s):
    k = s["kind"]
    if k == "poly":
        return [(p[0], p[1]) for p in s["points"]], s["closed"]
    if k == "ellipse":
        return ellipse_pts(s["cx"], s["cy"], s["rx"], s["ry"], n=40, rot=s.get("rotationDeg", 0)), True
    if k == "capsule":
        return [(s["x0"], s["y0"]), (s["x1"], s["y1"])], False
    if k == "path":
        toks = re.findall(r"[MLCQZ]|-?\d*\.?\d+(?:e-?\d+)?", s["d"])
        pts, i, cur = [], 0, (0, 0)
        while i < len(toks):
            t = toks[i]; i += 1
            if t == "Z":
                continue
            n = {"M": 2, "L": 2, "C": 6, "Q": 4}[t]
            v = [float(x) for x in toks[i:i + n]]; i += n
            if t in "ML":
                cur = (v[0], v[1]); pts.append(cur)
            elif t == "C":
                for u in [j / 10 for j in range(1, 11)]:
                    a, b, c, d = (1 - u) ** 3, 3 * (1 - u) ** 2 * u, 3 * (1 - u) * u * u, u ** 3
                    pts.append((a * cur[0] + b * v[0] + c * v[2] + d * v[4], a * cur[1] + b * v[1] + c * v[3] + d * v[5]))
                cur = (v[4], v[5])
            else:
                for u in [j / 10 for j in range(1, 11)]:
                    pts.append(((1 - u) ** 2 * cur[0] + 2 * (1 - u) * u * v[0] + u * u * v[2], (1 - u) ** 2 * cur[1] + 2 * (1 - u) * u * v[1] + u * u * v[3]))
                cur = (v[2], v[3])
        return pts, s["d"].strip().endswith("Z")
    raise ValueError(k)


def render(asset, rig, den, out, pose_id=None, pxmm=12, bonesover=None, plan=False):
    rp = next((p for p in asset["poses"] if p["id"] == (pose_id or asset["presentation"]["defaultPose"])), asset["poses"][0])
    bo = dict(rp["bones"])
    if bonesover:
        bo.update(bonesover)
    W = rig.world(bo)
    r = asset["representations"][str(den)]
    shapes = sorted(r["geometry"]["plan" if plan else "elevation"], key=lambda s: s["z"])
    pn = r["pens"]
    items, xs, ys = [], [], []
    for s in shapes:
        pts, closed = flat(s)
        wp = [ap(W[s["bone"]], p) for p in pts]
        items.append((s, wp, closed))
        xs += [p[0] for p in wp]; ys += [p[1] for p in wp]
    sc = 1000.0 / den * pxmm
    x0, x1, y0, y1 = min(xs) - 0.3, max(xs) + 0.3, min(ys + [0]) - 0.1, max(ys) + 0.3
    w, h = (x1 - x0) * sc, (y1 - y0) * sc
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{w:.0f}" height="{h:.0f}" viewBox="0 0 {w:.1f} {h:.1f}"><rect width="100%" height="100%" fill="white"/>']
    for s, wp, closed in items:
        pen = s.get("penMm") or pn.get(s["role"]) or pn["body"]
        pts = " ".join(f"{(x - x0) * sc:.1f},{(y1 - y) * sc:.1f}" for x, y in wp)
        grey = int(255 * min(1, max(0, s["tone"])))
        fill = f"rgb({grey},{grey},{grey})" if s["filled"] else "none"
        tag = "polygon" if closed else "polyline"
        svg.append(f'<{tag} points="{pts}" fill="{fill}" fill-opacity="0.55" stroke="black" stroke-width="{pen * pxmm:.2f}" stroke-linejoin="round"/>')
    svg.append(f'<line x1="0" x2="{w}" y1="{(y1 - 0) * sc:.1f}" y2="{(y1 - 0) * sc:.1f}" stroke="black" stroke-width="{pn["ground"] * pxmm:.2f}"/></svg>')
    open(out + ".svg", "w").write("\n".join(svg))
    subprocess.run(["rsvg-convert", out + ".svg", "-o", out + ".png"], check=True)
