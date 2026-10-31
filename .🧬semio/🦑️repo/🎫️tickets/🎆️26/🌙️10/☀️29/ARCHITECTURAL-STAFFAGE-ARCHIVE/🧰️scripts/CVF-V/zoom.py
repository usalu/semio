import sys, json, subprocess, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from qc2 import *
# usage: zoom.py id scale pose cx cy halfmm out.png   (cx,cy paper mm as printed by qc2)
id, k, pose, cx, cy, h, out = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4]), float(sys.argv[5]), float(sys.argv[6]), sys.argv[7]
a = json.load(open(f"{G}/work/{id}.json"))["asset"]
p = [q for q in a["poses"] if q["id"] == pose][0]
R, items = prep(posed(a, p), k); parents = {b["id"]: b["parent"] for b in a["rig"]["bones"]}
px = 900/(2*h); body = []
def T(q): return f"{(q[0]-cx+h)*px:.1f},{(q[1]-cy+h)*px:.1f}"
cols = ["#d33", "#36c", "#292", "#c80", "#808", "#088", "#555"]
for i, it in enumerate(items):
    P = it["P"]
    if it["box"][1] < cx-h or it["box"][0] > cx+h or it["box"][3] < cy-h or it["box"][2] > cy+h: continue
    body.append(f'<path d="M{"L".join(T(q) for q in P)}{"Z" if it["closed"] else ""}" fill="{cols[i%7]}" fill-opacity="0.08" stroke="#bbb" stroke-width="1"/>')
for i, it in enumerate(items):
    if it["box"][1] < cx-h or it["box"][0] > cx+h or it["box"][3] < cy-h or it["box"][2] > cy+h: continue
    for r in runs(it, items, parents):
        body.append(f'<path d="M{"L".join(T(q) for q in r)}" fill="none" stroke="{cols[i%7]}" stroke-width="{max(1.5, it["pen"]*px)}" stroke-opacity="0.8"/>')
    body.append(f'<text x="{T(it["P"][0]).split(",")[0]}" y="{T(it["P"][0]).split(",")[1]}" font-size="11" fill="{cols[i%7]}">{it["s"]["id"]}</text>')
open(out + ".svg", "w").write(f'<svg xmlns="http://www.w3.org/2000/svg" width="900" height="900"><rect width="900" height="900" fill="white"/>{"".join(body)}<circle cx="450" cy="450" r="6" fill="none" stroke="black"/></svg>')
subprocess.run(["rsvg-convert", out + ".svg", "-o", out], check=True)
