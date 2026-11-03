import ast, subprocess, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from scan_py import scan
import re

def added(path):
    out = subprocess.run(["git", "-c", "core.quotepath=false", "diff", "-U0", "HEAD", "--", path], capture_output=True, text=True, encoding="utf-8").stdout
    if not out:
        return None
    s = set()
    for m in re.finditer(r"^@@ -\S+ \+(\d+)(?:,(\d+))? @@", out, re.M):
        a = int(m.group(1)); n = int(m.group(2) or 1)
        s.update(range(a, a + n))
    return s

T = sys.argv[1]
seen = set()
for ln in open(T, encoding="utf-8").read().splitlines():
    if not ln.strip() or ln.startswith("TOTAL"):
        continue
    rel = ln.split(" dupcount")[0].rsplit(" ", 1)
    seen.add(rel[0])
for p in open(sys.argv[2], encoding="utf-8").read().splitlines():
    if not p.endswith(".py"):
        continue
    short = p.split("architecture/")[-1]
    if short not in seen:
        continue
    r = scan(p)
    ad = added(p)
    lines = open(p, encoding="utf-8").read().splitlines()
    for k, v in r["dup"]:
        if not any(ad is None or x[0] in ad for x in v):
            continue
        print("==", short, k)
        for line, name in v:
            print("  ", line, name, "DIFF" if (ad is None or line in ad) else "old", "|", lines[line - 1].strip()[:110])
