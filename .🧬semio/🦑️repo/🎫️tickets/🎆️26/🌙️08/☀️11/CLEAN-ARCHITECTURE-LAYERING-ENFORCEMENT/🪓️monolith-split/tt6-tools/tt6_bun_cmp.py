"""📊 Compares two tt6-bun-<phase> result folders file by file: failing-test names only before / only after, pass and fail counts. usage: tt5_bun_cmp.py <before> <after> (phase names)"""
import os, re, sys
G = os.path.join(os.path.dirname(__file__), "..", "🗑️generated")
a, b = (os.path.join(G, f"tt6-bun-{p}") for p in sys.argv[1:3])
worse = better = same = 0
for name in sorted(os.listdir(a)):
    if name == "DONE" or not os.path.exists(os.path.join(b, name)): continue
    ta, tb = (open(os.path.join(d, name), encoding="utf8").read() for d in (a, b))
    fa = {l for l in ta.split("\n") if l.startswith("(fail)")}; fb = {l for l in tb.split("\n") if l.startswith("(fail)")}
    ca = re.findall(r"^ *(\d+) (pass|fail)", ta, re.M); cb = re.findall(r"^ *(\d+) (pass|fail)", tb, re.M)
    if fb - fa or ca != cb:
        label = "WORSE" if fb - fa or (dict((k, int(v)) for v, k in cb).get("pass", 0) < dict((k, int(v)) for v, k in ca).get("pass", 0)) else "changed"
        print(label, name, ta.split("\n")[0][5:100], "| before", ca, "after", cb, "| only-after-fails:", len(fb - fa), "only-before:", len(fa - fb))
        if label == "WORSE": worse += 1
        else: better += 1
    else: same += 1
print(f"identical={same} changed-not-worse={better} worse={worse}")
