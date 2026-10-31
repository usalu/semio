# [scratch CV-A] FK, world<->local path conversion and SVG preview for animal assets (mirrors 🦴️rig/🟦️.ts)
import json, math, re, subprocess, os
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE"
S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage"
OUT = f"{T}/🗑️generated/CV-A"

def mat(x, y, deg):
    r = math.radians(deg); c, s = math.cos(r), math.sin(r)
    return (c, s, -s, c, x, y)
def mul(p, l):
    return (p[0]*l[0]+p[2]*l[1], p[1]*l[0]+p[3]*l[1], p[0]*l[2]+p[2]*l[3], p[1]*l[2]+p[3]*l[3],
            p[0]*l[4]+p[2]*l[5]+p[4], p[1]*l[4]+p[3]*l[5]+p[5])
def app(m, x, y):
    return (m[0]*x+m[2]*y+m[4], m[1]*x+m[3]*y+m[5])
def inv(m):
    d = m[0]*m[3]-m[1]*m[2]
    return (m[3]/d, -m[1]/d, -m[2]/d, m[0]/d, (m[2]*m[5]-m[3]*m[4])/d, (m[1]*m[4]-m[0]*m[5])/d)

def load_v1(aid):
    return json.load(open(f"{T}/🧰️scripts/CV-A/v1/{aid}.json"))["asset"]

def fk(asset, offsets, root=(0, 0, 0)):
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    w = {}
    def solve(bid):
        if bid in w: return w[bid]
        b = bones[bid]; j = b["joint"]
        off = offsets.get(bid, {}).get("angleDeg", 0)
        off = min(max(off, j.get("minDeg", -1e9)), j.get("maxDeg", 1e9))
        loc = mat(b["x"], b["y"], b["angleDeg"] + off)
        par = mat(*root) if b["parent"] is None else solve(b["parent"])
        w[bid] = mul(par, loc); return w[bid]
    for b in bones: solve(b)
    return w

def pose_world(asset, pose_id):
    p = next(x for x in asset["poses"] if x["id"] == pose_id)
    w = fk(asset, p["bones"], (0, 0, p.get("root", {}).get("angleDeg", 0)))
    anch = {a["id"]: a for a in asset["anchors"]}
    ys = [app(w[anch[c]["bone"]], anch[c]["x"], anch[c]["y"])[1] for c in p["contacts"]]
    dy = -min(ys)
    w = {k: (m[0], m[1], m[2], m[3], m[4], m[5] + dy) for k, m in w.items()}
    return w

TOK = re.compile(r"[MLCQZHVmlcqzhv]|-?\d*\.?\d+(?:e-?\d+)?")
def path_map(d, f):
    """apply f(x,y)->(x,y) to an absolute M/L/C/Q/Z path"""
    toks = TOK.findall(d); out = []; i = 0; cmd = None
    n = {"M": 2, "L": 2, "C": 6, "Q": 4, "Z": 0}
    while i < len(toks):
        t = toks[i]
        if t.isalpha():
            cmd = t; i += 1
            if cmd not in n: raise ValueError("absolute M L C Q Z only: " + cmd)
            if cmd == "Z": out.append("Z"); continue
        k = n[cmd]; vals = [float(v) for v in toks[i:i+k]]; i += k
        pts = []
        for j in range(0, k, 2):
            x, y = f(vals[j], vals[j+1]); pts += [round(x, 4), round(y, 4)]
        out.append(cmd + " " + " ".join(f"{v:.4f}".rstrip("0").rstrip(".") if abs(v) >= 5e-5 else "0" for v in pts))
        if cmd == "M": cmd = "L"
    return " ".join(out)

def world_to_local(world, bone, d):
    m = inv(world[bone]); return path_map(d, lambda x, y: app(m, x, y))

def shape_svg(s, m, style):
    k = s["kind"]
    if k == "path":
        d = s["d"]
    elif k == "capsule":
        x0, y0, x1, y1, r0, r1 = (s[q] for q in ("x0", "y0", "x1", "y1", "r0", "r1"))
        a = math.atan2(y1-y0, x1-x0); nx, ny = -math.sin(a), math.cos(a)
        d = (f"M {x0+nx*r0} {y0+ny*r0} L {x1+nx*r1} {y1+ny*r1} A {r1} {r1} 0 0 0 {x1-nx*r1} {y1-ny*r1} "
             f"L {x0-nx*r0} {y0-ny*r0} A {r0} {r0} 0 0 0 {x0+nx*r0} {y0+ny*r0} Z")
    elif k == "ellipse":
        d = f"M {s['cx']-s['rx']} {s['cy']} A {s['rx']} {s['ry']} {-s.get('rotationDeg',0)} 1 0 {s['cx']+s['rx']} {s['cy']} A {s['rx']} {s['ry']} {-s.get('rotationDeg',0)} 1 0 {s['cx']-s['rx']} {s['cy']} Z"
    elif k == "poly":
        d = "M " + " L ".join(f"{x} {y}" for x, y in s["points"]) + (" Z" if s["closed"] else "")
    elif k == "rect":
        x, y, w, h = s["x"], s["y"], s["w"], s["h"]; d = f"M {x} {y} L {x+w} {y} L {x+w} {y+h} L {x} {y+h} Z"
    tr = f"matrix({m[0]} {-m[1]} {m[2]} {-m[3]} {m[4]} {-m[5]})"
    return f'<path transform="{tr}" d="{d}" {style}/>'

def render(shapes, world, pens, scale_px_per_m, bbox, out_png, hatches=None, ground=True, note=None, grid=None):
    """bbox=(x0,y0,x1,y1) metres; pens in model metres (stroke) via pens dict role->metres"""
    x0, y0, x1, y1 = bbox
    W = int((x1-x0)*scale_px_per_m); H = int((y1-y0)*scale_px_per_m)
    g = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="{x0} {-y1} {x1-x0} {y1-y0}"><rect x="{x0}" y="{-y1}" width="{x1-x0}" height="{y1-y0}" fill="white"/>']
    if grid:
        gx = math.ceil(x0/grid)*grid
        while gx < x1:
            g.append(f'<line x1="{gx}" y1="{-y1}" x2="{gx}" y2="{-y0}" stroke="#ddd" stroke-width="{grid/40}"/>'); gx += grid
        gy = math.ceil(y0/grid)*grid
        while gy < y1:
            g.append(f'<line x1="{x0}" y1="{-gy}" x2="{x1}" y2="{-gy}" stroke="#ddd" stroke-width="{grid/40}"/>'); gy += grid
    if ground: g.append(f'<line x1="{x0}" y1="0" x2="{x1}" y2="0" stroke="#999" stroke-width="{pens.get("ground",0.001)}"/>')
    for s in sorted(shapes, key=lambda s: s["z"]):
        pen = s.get("penMm"); role = s["role"]
        sw = (pen/1000*pens["mmToM"]) if pen else pens.get(role)
        if sw is None: continue
        tone = s.get("tone", 0.5)
        fill = f"rgb({int(255*(1-tone))},{int(255*(1-tone))},{int(255*(1-tone))})" if s["filled"] and role in ("body", "detail") else ("none" if not s["filled"] else "#cde")
        if s["filled"] and role == "body": fill = f"rgb({int(255-tone*200)},{int(255-tone*200)},{int(255-tone*200)})"
        g.append(shape_svg(s, world[s["bone"]], f'fill="{fill}" stroke="black" stroke-width="{sw}" stroke-linejoin="round"'))
    if note: g.append(f'<text x="{x0+0.01}" y="{-y1+0.04}" font-size="0.03" fill="red">{note}</text>')
    g.append("</svg>")
    svg = out_png[:-4] + ".svg"; open(svg, "w").write("\n".join(g))
    subprocess.run(["rsvg-convert", svg, "-o", out_png], check=True); os.remove(svg)

def bbox_of(shapes, world):
    import itertools
    xs = []; ys = []
    for s in shapes:
        if s["role"] == "ground": continue
        k = s["kind"]; pts = []
        if k == "path":
            toks = TOK.findall(s["d"]); i = 0; cur = None; cmd = None
            while i < len(toks):
                if toks[i].isalpha():
                    cmd = toks[i]; i += 1
                    if cmd == "Z": continue
                if cmd in ("M", "L"):
                    cur = (float(toks[i]), float(toks[i+1])); i += 2; pts.append(cur)
                elif cmd == "C":
                    v = [float(t) for t in toks[i:i+6]]; i += 6
                    for j in range(1, 21):
                        u = j/20; a = (1-u)**3; b = 3*u*(1-u)**2; c = 3*u*u*(1-u); d = u**3
                        pts.append((a*cur[0]+b*v[0]+c*v[2]+d*v[4], a*cur[1]+b*v[1]+c*v[3]+d*v[5]))
                    cur = (v[4], v[5])
        elif k == "capsule":
            pts = [(s["x0"]+dx*s["r0"], s["y0"]+dy*s["r0"]) for dx in (-1,1) for dy in (-1,1)] + [(s["x1"]+dx*s["r1"], s["y1"]+dy*s["r1"]) for dx in (-1,1) for dy in (-1,1)]
        elif k == "ellipse":
            r = max(s["rx"], s["ry"]); pts = [(s["cx"]+dx*r, s["cy"]+dy*r) for dx in (-1,1) for dy in (-1,1)]
        elif k == "poly": pts = [tuple(p) for p in s["points"]]
        elif k == "rect": pts = [(s["x"], s["y"]), (s["x"]+s["w"], s["y"]+s["h"])]
        for p in pts:
            a = app(world[s["bone"]], *p); xs.append(a[0]); ys.append(a[1])
    return min(xs), min(ys), max(xs), max(ys)

def save(asset, aid):
    p = f"{S}/🗂️catalog/🐕️animals/{aid}/🔣️.json"
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    json.dump(doc, open(p, "w"), ensure_ascii=False, indent=2); open(p, "a").write("\n")

def skeleton(asset, world, with_labels=True):
    """returns svg fragments and table of bone origin/end in world coords"""
    rows = {}
    for b in asset["rig"]["bones"]:
        m = world[b["id"]]; rows[b["id"]] = (app(m, 0, 0), app(m, b["lengthM"], 0))
    return rows

def render_with_skeleton(asset, world, shapes, bbox, ppm, out_png, pens, grid=0.1, anchors=True):
    x0, y0, x1, y1 = bbox
    rows = skeleton(asset, world)
    # draw shapes first to svg then overlay: reuse render by monkeypatch via extra shapes
    extra = []
    for bid, (a, b) in rows.items():
        extra.append({"id": "sk-" + bid, "bone": asset["rig"]["root"], "role": "detail", "z": 99, "filled": False, "tone": 0, "kind": "poly", "closed": False,
                      "points": [list(a), list(b)], "penMm": 0.0, "_world": True})
    # simple approach: transform with identity world for these
    w2 = dict(world); w2[asset["rig"]["root"]] = world[asset["rig"]["root"]]
    # render shapes, then re-open svg is overkill; draw skeleton as separate world-space shapes with identity matrix
    ident = (1, 0, 0, 1, 0, 0)
    sh = list(shapes)
    for e in extra:
        e["bone"] = "__id"; sh.append(e)
    w2["__id"] = ident
    pens2 = dict(pens); pens2["detail"] = pens.get("skel", 0.0015)
    render(sh, w2, pens2, ppm, bbox, out_png, grid=grid)
