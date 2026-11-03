import re, subprocess, sys, os
sys.path.insert(0, os.path.dirname(__file__))
from scan_py import scan

def added(path):
    out = subprocess.run(["git", "-c", "core.quotepath=false", "diff", "-U0", "HEAD", "--", path], capture_output=True, text=True, encoding="utf-8").stdout
    if not out:
        return None
    s = set()
    for m in re.finditer(r"^@@ -\S+ \+(\d+)(?:,(\d+))? @@", out, re.M):
        a = int(m.group(1)); n = int(m.group(2) or 1)
        s.update(range(a, a + n))
    return s

tot = 0
if __name__ == "__main__":
  pass
for p in open(sys.argv[1], encoding="utf-8").read().splitlines():
    if not p.endswith(".py"):
        continue
    r = scan(p)
    ad = added(p)
    for k, v in r["dup"]:
        mine = [x for x in v if ad is None or x[0] in ad]
        if mine and len(v) > 1:
            tot += len(mine)
            print(p.split("architecture/")[-1], k, "dupcount", len(v), "inDiff", len(mine))
print("TOTAL in-diff dup members", tot)
