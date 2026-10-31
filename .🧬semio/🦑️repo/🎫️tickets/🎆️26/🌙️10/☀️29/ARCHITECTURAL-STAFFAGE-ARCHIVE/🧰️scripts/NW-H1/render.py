"""NW-H1 quick look renderer (own scratch tool, not authoritative): draws an asset document at a scale for a pose or clip time to PNG via rsvg-convert."""
import json, math, re, subprocess, sys, os
from nwlib import *


def load(path):
    return json.load(open(path))["asset"]


def rig_of(a):
    return Rig(a["rig"]["bones"], a["rig"]["root"])


def pose_offsets(a, pid):
    p = next(x for x in a["poses"] if x["id"] == pid)
    return p["bones"], p


def clip_offsets(a, cid, t):
    c = next(x for x in a["clips"] if x["id"] == cid)
    keys = c["keys"]
    t = t % c["durationS"] if c["loop"] else min(t, c["durationS"])
    i = max(j for j in range(len(keys)) if keys[j]["t"] <= t + 1e-9)
    k0 = keys[i]
    k1 = keys[min(i + 1, len(keys) - 1)]
    w = 0 if k1["t"] == k0["t"] else (t - k0["t"]) / (k1["t"] - k0["t"])
    pose = next(x for x in a["poses"] if x["id"] == k0["pose"])
    offs = {}
    ids = set(k0.get("bones", {})) | set(k1.get("bones", {}))
    for b in ids:
        o0 = k0["bones"].get(b, {})
        o1 = k1["bones"].get(b, {})
        offs[b] = {kk: o0.get(kk, 0) * (1 - w) + o1.get(kk, 0) * w for kk in ("angleDeg", "dx", "dy") if kk in o0 or kk in o1}
    return offs, pose, c


def tokens(d):
    return re.findall(r"[MLCZ]|-?\d*\.?\d+(?:e-?\d+)?", d)


def shape_rings(s, W):
    wb = W[s["bone"]]
    k = s["kind"]
    if k == "poly":
        return [[wb.ap(*p) for p in s["points"]]], s["closed"]
    if k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]
        return [[wb.ap(*p) for p in ((x, y), (x + w, y), (x + w, y + h), (x, y + h))]], True
    if k == "ellipse":
        pts = []
        rot = math.radians(s.get("rotationDeg", 0))
        for i in range(32):
            a = 2 * math.pi * i / 32
            ex, ey = s["rx"] * math.cos(a), s["ry"] * math.sin(a)
            pts.append(wb.ap(s["cx"] + ex * math.cos(rot) - ey * math.sin(rot), s["cy"] + ex * math.sin(rot) + ey * math.cos(rot)))
        return [pts], True
    if k == "capsule":
        x0, y0, x1, y1, r0, r1 = s["x0"], s["y0"], s["x1"], s["y1"], s["r0"], s["r1"]
        a = math.atan2(y1 - y0, x1 - x0)
        pts = []
        for i in range(0, 13):
            t = a + math.pi / 2 + math.pi * i / 12
            pts.append(wb.ap(x1 + r1 * math.cos(t), y1 + r1 * math.sin(t)))
        for i in range(0, 13):
            t = a - math.pi / 2 + math.pi * i / 12
            pts.append(wb.ap(x0 + r0 * math.cos(t), y0 + r0 * math.sin(t)))
        return [pts], True
    if k == "path":
        tk = tokens(s["d"])
        pts = []
        i = 0
        cur = None
        closed = False
        while i < len(tk):
            c = tk[i]
            if c == "M":
                cur = (float(tk[i + 1]), float(tk[i + 2]))
                pts.append(wb.ap(*cur))
                i += 3
            elif c == "L":
                cur = (float(tk[i + 1]), float(tk[i + 2]))
                pts.append(wb.ap(*cur))
                i += 3
            elif c == "C":
                v = [float(x) for x in tk[i + 1:i + 7]]
                p0 = cur
                for j in range(1, 9):
                    t = j / 8
                    x = (1 - t) ** 3 * p0[0] + 3 * (1 - t) ** 2 * t * v[0] + 3 * (1 - t) * t * t * v[2] + t ** 3 * v[4]
                    y = (1 - t) ** 3 * p0[1] + 3 * (1 - t) ** 2 * t * v[1] + 3 * (1 - t) * t * t * v[3] + t ** 3 * v[5]
                    pts.append(wb.ap(x, y))
                cur = (v[4], v[5])
                i += 7
            elif c == "Z":
                closed = True
                i += 1
            else:
                i += 1
        return [pts], closed
    raise ValueError(k)


def render(a, den, offs, pose, out, px_per_mm=40, ground_contacts=None, pad=0.15, regions=False, hatch=False):
    rig = rig_of(a)
    rep = a["representations"][str(den)]
    root = pose.get("root", {"x": 0, "y": 0})
    W = rig.fk(offs, (root.get("x", 0), root.get("y", 0), 0))
    # auto ground for listed contacts
    cons = ground_contacts if ground_contacts is not None else pose.get("contacts", [])
    if pose.get("ground", "auto") == "auto" and cons:
        ys = []
        for c in cons:
            an = next(x for x in a["anchors"] if x["id"] == c)
            ys.append(W[an["bone"]].ap(an["x"], an["y"])[1])
        dy = -min(ys)
        W = rig.fk(offs, (root.get("x", 0), root.get("y", 0) + dy, 0))
    shapes = sorted(rep["geometry"]["elevation"], key=lambda s: s["z"])
    items = []
    xs, ys = [], []
    for s in shapes:
        rings, closed = shape_rings(s, W)
        for r in rings:
            for p in r:
                xs.append(p[0]); ys.append(p[1])
        items.append((s, rings, closed))
    x0, x1, y0, y1 = min(xs) - pad, max(xs) + pad, 0 - 0.05, max(ys) + pad
    k = 1000.0 / den * px_per_mm
    Wp, Hp = (x1 - x0) * k, (y1 - y0) * k
    pens = rep["pens"]
    out_s = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d"><rect width="100%%" height="100%%" fill="white"/>' % (Wp, Hp, Wp, Hp)]
    out_s.append('<line x1="0" x2="%d" y1="%d" y2="%d" stroke="#888" stroke-width="%.2f"/>' % (Wp, Hp - 0.05 * k, Hp - 0.05 * k, pens["ground"] * px_per_mm))
    for s, rings, closed in items:
        pen = s.get("penMm", pens.get(s["role"]) or 0.1) * px_per_mm
        for r in rings:
            pts = " ".join("%.2f,%.2f" % ((p[0] - x0) * k, (y1 - p[1]) * k) for p in r)
            tone = int(255 * s["tone"])
            if s["filled"] and closed:
                out_s.append('<polygon points="%s" fill="rgb(%d,%d,%d)" stroke="black" stroke-width="%.2f" stroke-linejoin="round"/>' % (pts, tone, tone, tone, pen))
            else:
                tag = "polygon" if closed else "polyline"
                out_s.append('<%s points="%s" fill="none" stroke="black" stroke-width="%.2f" stroke-linecap="round" stroke-linejoin="round"/>' % (tag, pts, pen))
    out_s.append("</svg>")
    svg = out + ".svg"
    open(svg, "w").write("\n".join(out_s))
    subprocess.run(["rsvg-convert", "-o", out + ".png", svg], check=True)
    return out + ".png"


if __name__ == "__main__":
    path, den, mode, ident = sys.argv[1:5]
    out = sys.argv[5]
    ppm = float(sys.argv[6]) if len(sys.argv) > 6 else 40
    a = load(path)
    if mode == "pose":
        offs, pose = pose_offsets(a, ident)
    else:
        t = float(sys.argv[7])
        offs, pose, c = clip_offsets(a, ident, t)
    print(render(a, int(den), offs, pose, out, ppm))
