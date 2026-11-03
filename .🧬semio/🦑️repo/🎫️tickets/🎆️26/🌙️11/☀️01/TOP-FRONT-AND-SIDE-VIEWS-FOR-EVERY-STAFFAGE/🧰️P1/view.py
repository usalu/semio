import sys; sys.path.insert(0, ".")
import json, math, os
from lib import CATALOG, TICKET

def tone(t):
    v = int(255 * (1 - t) * 0.0 + 255 * t) if t is not None else 200
    return f"rgb({v},{v},{v})"

def shape_svg(s, k):
    fill = tone(s.get("tone", 0.5)) if s.get("filled") else "none"
    stroke = "#c33" if s["role"] == "detail" else "#222"
    sw = 0.8 if s["role"] != "detail" else 0.5
    k_ = s["kind"]
    a = f'fill="{fill}" stroke="{stroke}" stroke-width="{sw}"'
    def X(x): return x * k
    def Y(y): return -y * k
    if k_ == "rect":
        r = s.get("r", 0) * k
        return f'<rect x="{X(s["x"])}" y="{Y(s["y"] + s["h"])}" width="{s["w"]*k}" height="{s["h"]*k}" rx="{r}" {a}/>'
    if k_ == "ellipse":
        return f'<ellipse cx="{X(s["cx"])}" cy="{Y(s["cy"])}" rx="{s["rx"]*k}" ry="{s["ry"]*k}" {a}/>'
    if k_ == "poly":
        pts = " ".join(f"{X(x)},{Y(y)}" for x, y in s["points"])
        tag = "polygon" if s["closed"] else "polyline"
        return f'<{tag} points="{pts}" {a}/>'
    if k_ == "capsule":
        return f'<line x1="{X(s["x0"])}" y1="{Y(s["y0"])}" x2="{X(s["x1"])}" y2="{Y(s["y1"])}" stroke="{stroke}" stroke-width="{s["r0"]*2*k}" stroke-linecap="round"/>'
    if k_ == "path":
        t = s["d"].split(); out = []; i = 0; cmd = ""
        for tok in t:
            if tok.isalpha():
                cmd = tok; out.append(tok)
            else:
                out.append(str(float(tok) * k * (1 if (len([o for o in out if not o.isalpha()]) % 2 == 0) else -1)))
        return f'<path d="{" ".join(out)}" {a}/>'
    return ""

def panel(shapes, k, title):
    shapes = sorted(shapes, key=lambda s: s["z"])
    xs, ys = [], []
    for s in shapes:
        if s["kind"] == "rect":
            xs += [s["x"], s["x"] + s["w"]]; ys += [s["y"], s["y"] + s["h"]]
        elif s["kind"] == "ellipse":
            xs += [s["cx"] - s["rx"], s["cx"] + s["rx"]]; ys += [s["cy"] - s["ry"], s["cy"] + s["ry"]]
        elif s["kind"] == "poly":
            xs += [p[0] for p in s["points"]]; ys += [p[1] for p in s["points"]]
        elif s["kind"] == "path":
            n = [float(t) for t in s["d"].split() if not t.isalpha()]
            xs += n[0::2]; ys += n[1::2]
    x0, x1, y0, y1 = min(xs), max(xs), min(ys), max(ys)
    m = 10
    body = "".join(shape_svg(s, k) for s in shapes)
    w, h = (x1 - x0) * k + 2 * m, (y1 - y0) * k + 2 * m
    return f'<div style="display:inline-block;margin:6px;vertical-align:top"><div style="font:11px sans-serif">{title} [{x0:.3f},{x1:.3f}] x [{y0:.3f},{y1:.3f}]</div><svg width="{w}" height="{h}" viewBox="{x0*k-m} {-y1*k-m} {w} {h}" style="background:#fff">{body}</svg></div>'

def main():
    asset = sys.argv[1]
    px = float(sys.argv[2]) if len(sys.argv) > 2 else 360
    scales = sys.argv[3].split(",") if len(sys.argv) > 3 else ["50", "100", "200"]
    d = json.load(open(os.path.join(CATALOG, asset, "🔣️.json"), encoding="utf-8"))["asset"]
    html = ["<html><body style='margin:4px;background:#ddd'><h3>" + asset + "</h3>"]
    for sc in scales:
        g = d["representations"][sc]["geometry"]
        # scale factor: longest view dimension -> px
        allx = []
        for v in ("front", "plan"):
            for s in g[v]:
                pass
        k = px / max(d["dimensions"]["widthM"], d["dimensions"]["depthM"], d["dimensions"]["heightM"])
        html.append(f"<div>1:{sc}")
        html.append(panel(g["front"], k, "front"))
        html.append(panel(g["plan"], k, "plan"))
        html.append("</div>")
    html.append("</body></html>")
    out = os.path.join(TICKET, "🗑️generated", "P1", "view-" + asset + ".html")
    open(out, "w", encoding="utf-8").write("\n".join(html))

main()
