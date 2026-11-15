"""Reviewer 2 helper: one line per region from the dump files in 🗑️generated/areareview2."""
import os, re, sys
T = os.path.dirname(os.path.abspath(__file__))
for slug in sys.argv[1:]:
    t = open(os.path.join(T, "🗑️generated", "areareview2", slug + ".txt")).read().split("\n== ")
    print("#", slug, t[0].split("\n")[0][len(slug):][:110])
    for b in t[1:]:
        L = b.split("\n")
        m = re.match(r"(\S+) \[(\S+)\] areaM2 (\S+) \(HEAD (\S+)\) basis (\S+) geom (\S+) heatW (\S+)", L[0])
        s = [l for l in L if "SUM all" in l]
        pose = [l for l in L if l.strip().startswith("pose:")]
        print(" ", m.group(1), m.group(2), "A", m.group(3), "H", m.group(4), "" if m.group(5) == "None" else m.group(5), "heat", m.group(7), "|", s[0].strip().replace("SUM all layers", "all").replace("SUM of-refs/no-layer", "outer") if s else "", "|", pose[0].strip()[6:100] if pose else "")
