"""Author 28: exposed vs declared area per region (owned engine numbers, product deviation alongside). python3 author-28-areas.py <slug> <scale>"""
import sys, json, re, subprocess
import author_07_lib as L
slug, sc = sys.argv[1], sys.argv[2]
def run(extra):
    r = subprocess.run([L.bun(), f"{L.T}/volumes-check.ts", f"humans/{slug}", "--scale", sc, "--rules", "volumes-thermal-coverage", "--json", *extra], cwd=L.T, capture_output=True, text=True)
    return json.loads(r.stdout[r.stdout.index("{"):])["measurements"]
prod = {m["name"]: m["value"] for m in run([]) if m["name"].startswith("areaDeviation")}
for m in run(["--owned"]):
    n = m["name"]
    e = re.search(r"exposed ([\d.]+) m2.*areaM2 ([\d.]+)", m.get("note") or "")
    if n.startswith("areaDeviation:") and e:
        a, b = float(e.group(1)), float(e.group(2))
        print(f"{n[14:]:14s} exposed {a:.4f} declared {b:.4f} ratio {a/b:.2f}  product dev {prod.get(n, float('nan')):.2f}")
