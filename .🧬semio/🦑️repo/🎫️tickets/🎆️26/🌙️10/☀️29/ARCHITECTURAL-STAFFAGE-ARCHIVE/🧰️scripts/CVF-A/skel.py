# [scratch CVF-A] skeleton + anchor sheet of an asset pose in world metres, grid for hand design
import sys, json
from draw import *
def sheet(aid, pose, out, ppm=900, pad=0.1):
    a = load_v1(aid); w = pose_world(a, pose)
    pts = []
    for b in a["rig"]["bones"]:
        pts += [app(w[b["id"]], 0, 0), app(w[b["id"]], b["lengthM"], 0)]
    x0 = min(p[0] for p in pts)-pad; x1 = max(p[0] for p in pts)+pad; y0 = min(0, min(p[1] for p in pts))-0.02; y1 = max(p[1] for p in pts)+pad
    W_ = int((x1-x0)*ppm); H_ = int((y1-y0)*ppm)
    g = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W_}" height="{H_}" viewBox="{x0} {-y1} {x1-x0} {y1-y0}"><rect x="{x0}" y="{-y1}" width="{x1-x0}" height="{y1-y0}" fill="white"/>']
    step = 0.1 if (x1-x0) > 0.8 else 0.05
    gx = math.ceil(x0/step)*step
    while gx < x1:
        g.append(f'<line x1="{gx}" y1="{-y1}" x2="{gx}" y2="{-y0}" stroke="#ccc" stroke-width="{step/80}"/><text x="{gx}" y="{-y0-0.005}" font-size="{step/5}" fill="#888">{gx:.2f}</text>'); gx += step
    gy = math.ceil(y0/step)*step
    while gy < y1:
        g.append(f'<line x1="{x0}" y1="{-gy}" x2="{x1}" y2="{-gy}" stroke="#ccc" stroke-width="{step/80}"/><text x="{x0+0.005}" y="{-gy}" font-size="{step/5}" fill="#888">{gy:.2f}</text>'); gy += step
    g.append(f'<line x1="{x0}" y1="0" x2="{x1}" y2="0" stroke="#000" stroke-width="{step/40}"/>')
    for b in a["rig"]["bones"]:
        p = app(w[b["id"]], 0, 0); q = app(w[b["id"]], b["lengthM"], 0)
        col = "#c00" if b["id"].endswith("f") or "-f" in b["id"] else "#06c"
        g.append(f'<line x1="{p[0]}" y1="{-p[1]}" x2="{q[0]}" y2="{-q[1]}" stroke="{col}" stroke-width="{step/30}"/><circle cx="{p[0]}" cy="{-p[1]}" r="{step/25}" fill="{col}"/><text x="{(p[0]+q[0])/2}" y="{-(p[1]+q[1])/2}" font-size="{step/6}" fill="{col}">{b["id"]}</text>')
    for an in a["anchors"]:
        p = app(w[an["bone"]], an["x"], an["y"])
        g.append(f'<circle cx="{p[0]}" cy="{-p[1]}" r="{step/30}" fill="#080"/><text x="{p[0]}" y="{-p[1]-step/30}" font-size="{step/7}" fill="#080">{an["id"]}</text>')
    g.append("</svg>")
    svg = out[:-4]+".svg"; open(svg, "w").write("\n".join(g)); subprocess.run(["rsvg-convert", svg, "-o", out], check=True); os.remove(svg)
if __name__ == "__main__":
    aid, pose = sys.argv[1], sys.argv[2]
    sheet(aid, pose, f"{OUT}/skel-{aid}-{pose}.png", ppm=float(sys.argv[3]) if len(sys.argv) > 3 else 900)
