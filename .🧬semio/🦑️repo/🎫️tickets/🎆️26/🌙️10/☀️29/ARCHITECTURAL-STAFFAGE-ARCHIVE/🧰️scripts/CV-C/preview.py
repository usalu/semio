"""CV-C scratch preview: renders one authored representation (default pose) of a v2 component to SVG and PNG via rsvg-convert.
usage: preview.py <asset> <den> <zoom> [outdir]   (zoom 1 = true print size at 300 dpi; PNGs land in the CV-C generated folder)"""
import sys, json, math, subprocess, re
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/CV-C")
from lib import Frame, CAT
OUT = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CV-C/"


def render(aid, den, zoom=1.0, kind="elevation", symbol=None):
    a = json.load(open(CAT + aid + "/🔣️.json"))["asset"]
    fr = Frame(a["rig"]["bones"])
    if symbol:
        sym = next(s for s in a["symbols"] if s["id"] == symbol)
        rp = {"pens": sym["pens"], "hatches": sym["hatches"], "geometry": sym["geometry"], "status": "authored"}
        shapes = rp["geometry"].get(kind, [])
    else:
        rp = a["representations"][str(den)]
        if rp["status"] != "authored":
            print("unavailable", den); return
        shapes = rp["geometry"][kind]
    k = 1000.0 / den

    def w(bone, x, y):
        ox, oy, ang = fr.world(bone)
        c, s = math.cos(math.radians(ang)), math.sin(math.radians(ang))
        return ox + c * x - s * y, oy + s * x + c * y

    items = []
    allpts = []
    for sh in shapes:
        b = sh["bone"]; kd = sh["kind"]
        if kd == "poly":
            pts = [w(b, *p) for p in sh["points"]]
            d = "M " + " L ".join(f"{p[0]:.5f} {p[1]:.5f}" for p in pts) + (" Z" if sh["closed"] else "")
        elif kd == "rect":
            pts = [w(b, sh["x"] + dx * sh["w"], sh["y"] + dy * sh["h"]) for dx, dy in ((0, 0), (1, 0), (1, 1), (0, 1))]
            d = "M " + " L ".join(f"{p[0]:.5f} {p[1]:.5f}" for p in pts) + " Z"
        elif kd == "ellipse":
            cx, cy = w(b, sh["cx"], sh["cy"]); rx, ry = sh["rx"], sh["ry"]
            pts = [(cx - rx, cy - ry), (cx + rx, cy + ry)]
            d = f"E {cx} {cy} {rx} {ry}"
        elif kd == "capsule":
            (ax, ay), (bx, by) = w(b, sh["x0"], sh["y0"]), w(b, sh["x1"], sh["y1"]); r = sh["r0"]
            th = math.atan2(by - ay, bx - ax); pts = []
            for i in range(0, 13):
                t = th + math.pi / 2 + math.pi * i / 12; pts.append((bx + r * math.cos(t), by + r * math.sin(t)))
            for i in range(0, 13):
                t = th - math.pi / 2 + math.pi * i / 12; pts.append((ax + r * math.cos(t), ay + r * math.sin(t)))
            d = "M " + " L ".join(f"{p[0]:.5f} {p[1]:.5f}" for p in pts) + " Z"
        elif kd == "path":
            toks = sh["d"].split(); out = []; i = 0; pts = []
            while i < len(toks):
                t = toks[i]
                if t == "Z": out.append("Z"); i += 1; continue
                n = {"M": 2, "L": 2, "C": 6, "Q": 4}[t]
                nums = [float(v) for v in toks[i + 1:i + 1 + n]]
                ps = [w(b, nums[j], nums[j + 1]) for j in range(0, n, 2)]
                pts += ps
                out.append(t + " " + " ".join(f"{p[0]:.5f} {p[1]:.5f}" for p in ps)); i += 1 + n
            d = " ".join(out)
        else:
            continue
        allpts += pts
        items.append((sh, d))
    xs = [p[0] for p in allpts]; ys = [p[1] for p in allpts]
    x0, x1, y0, y1 = min(xs), max(xs), min(ys), max(ys)
    mg = 2.0
    W = (x1 - x0) * k + 2 * mg; Hh = (y1 - y0) * k + 2 * mg

    def P(x, y): return ((x - x0) * k + mg, (y1 - y) * k + mg)
    def conv(d):
        if d.startswith("E"):
            _, cx, cy, rx, ry = d.split(); px, py = P(float(cx), float(cy))
            return None, (px, py, float(rx) * k, float(ry) * k)
        toks = d.split(); out = []; i = 0
        while i < len(toks):
            t = toks[i]
            if t == "Z": out.append("Z"); i += 1; continue
            if t.isalpha(): out.append(t); i += 1; continue
            px, py = P(float(toks[i]), float(toks[i + 1])); out.append(f"{px:.4f} {py:.4f}"); i += 2
        return " ".join(out), None
    hat = {h["id"]: h for h in rp["hatches"]}
    defs = []
    for h in hat.values():
        sp = h["spacingMm"]; pen = h["penMm"]
        if h["pattern"] in ("lines", "cross"):
            body = f'<line x1="0" y1="0" x2="0" y2="{sp}" stroke="#000" stroke-width="{pen}"/>' + (f'<line x1="0" y1="0" x2="{sp}" y2="0" stroke="#000" stroke-width="{pen}"/>' if h["pattern"] == "cross" else "")
            defs.append(f'<pattern id="h-{h["id"]}" width="{sp}" height="{sp}" patternUnits="userSpaceOnUse" patternTransform="rotate({-h["angleDeg"]+90})">{body}</pattern>')
        else:
            defs.append(f'<pattern id="h-{h["id"]}" width="{sp}" height="{sp}" patternUnits="userSpaceOnUse"><circle cx="{sp/2}" cy="{sp/2}" r="{max(pen,0.1)*0.6}" fill="#000"/><circle cx="{sp/4}" cy="{sp*0.8}" r="{max(pen,0.1)*0.4}" fill="#000"/></pattern>')
    body = []
    for sh, d in sorted(items, key=lambda t: t[0]["z"]):
        pen = sh.get("penMm", rp["pens"][sh["role"]])
        if pen is None: continue
        cd, el = conv(d)
        g = int(255 * (0.35 + 0.65 * sh["tone"])) if sh["filled"] else None
        fill = f'rgb({g},{g},{g})' if sh["filled"] else "none"
        if sh["role"] == "glazing" and sh["filled"]: fill = "#dfeaf0"
        if sh["role"] in ("texture",) : fill = "none"
        if el:
            body.append(f'<ellipse cx="{el[0]:.4f}" cy="{el[1]:.4f}" rx="{el[2]:.4f}" ry="{el[3]:.4f}" fill="{fill}" stroke="#000" stroke-width="{pen}"/>')
        else:
            body.append(f'<path d="{cd}" fill="{fill}" stroke="#000" stroke-width="{pen}" stroke-linejoin="round"/>')
            if "hatch" in sh:
                body.append(f'<path d="{cd}" fill="url(#h-{sh["hatch"]})" stroke="none"/>')
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" width="{W:.3f}mm" height="{Hh:.3f}mm" viewBox="0 0 {W:.3f} {Hh:.3f}"><rect width="100%" height="100%" fill="#fff"/><defs>{"".join(defs)}</defs>{"".join(body)}</svg>'
    tag = f"{aid}-{symbol or den}-{kind}-z{zoom:g}"
    open(OUT + tag + ".svg", "w").write(svg)
    dpi = 300 * zoom
    subprocess.run(["rsvg-convert", "-d", str(dpi), "-p", str(dpi), "-o", OUT + tag + ".png", OUT + tag + ".svg"], check=True)
    print(OUT + tag + ".png", f"{W:.1f} x {Hh:.1f} mm")


if __name__ == "__main__":
    aid, den, zoom = sys.argv[1], sys.argv[2], float(sys.argv[3])
    if den.startswith("sym:"):
        render(aid, 0, zoom, sys.argv[4] if len(sys.argv) > 4 else "elevation", symbol=den[4:])
    else:
        render(aid, int(den), zoom, sys.argv[4] if len(sys.argv) > 4 else "elevation")
