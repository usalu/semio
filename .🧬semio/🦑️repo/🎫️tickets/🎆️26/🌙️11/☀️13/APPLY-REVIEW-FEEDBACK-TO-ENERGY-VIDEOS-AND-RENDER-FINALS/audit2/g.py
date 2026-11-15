import re, sys, os
HERE = os.path.dirname(os.path.abspath(__file__))
flist = os.path.join(HERE, "files.txt")
arg = sys.argv[1]
if arg.startswith("@"):
    with open(arg[1:], encoding="utf-8") as fh:
        arg = fh.read().strip("\n")
pat = re.compile(arg)
root = "C:/git/semio"
files = [l.rstrip("\n") for l in open(flist, encoding="utf-8") if l.strip()]
total = 0
for f in files:
    p = os.path.join(root, f)
    try:
        with open(p, encoding="utf-8") as fh:
            lines = fh.readlines()
    except Exception:
        continue
    hits = [(i + 1, l.rstrip("\n")) for i, l in enumerate(lines) if pat.search(l)]
    if not hits:
        continue
    for n, l in hits:
        print(f"{f}:{n}: {l.strip()[:300]}")
    total += len(hits)
print("TOTAL", total)
