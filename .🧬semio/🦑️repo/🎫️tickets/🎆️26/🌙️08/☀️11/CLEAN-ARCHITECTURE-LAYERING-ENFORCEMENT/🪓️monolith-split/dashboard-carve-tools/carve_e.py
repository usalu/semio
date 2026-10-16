"""🧹 Stage E of the dashboard carve: the `use dashboard_*` lines of every carved file sit together, sorted, after the `super`/`crate` imports. Usage: python3 carve_e.py <dashboard-dir> <original-dir>"""
import filecmp, os, re, sys
D, O = sys.argv[1], sys.argv[2]
changed = 0
for dp, dn, fn in os.walk(D):
    if "node_modules" in dp or "🧭️journeys" in dp:
        continue
    for f in fn:
        if not f.endswith(".rs"):
            continue
        path = os.path.join(dp, f)
        old = os.path.join(O, os.path.relpath(path, D))
        if os.path.exists(old) and filecmp.cmp(old, path, shallow=False):
            continue
        lines = open(path, encoding="utf-8").read().split("\n")
        i = 0
        while i < len(lines):
            if lines[i].startswith("use ") and lines[i].rstrip().endswith(";"):
                j = i
                while j < len(lines) and lines[j].startswith("use ") and lines[j].rstrip().endswith(";"):
                    j += 1
                run = lines[i:j]
                head = [l for l in run if l.startswith(("use super::", "use crate::", "use self::"))]
                dash = sorted((l for l in run if l.startswith("use dashboard_")), key=lambda l: l.encode())
                rest = [l for l in run if l not in head and not l.startswith("use dashboard_")]
                new = head + dash + rest
                if new != run and dash:
                    lines[i:j] = new
                    changed += 1
                i = j
            else:
                i += 1
        open(path, "w", encoding="utf-8").write("\n".join(lines))
print("stage E reordered", changed, "import runs")
