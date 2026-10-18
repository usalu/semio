"""🔎 OHF row source viewer: for rows of a harvest TSV whose message matches a regex, prints the real repo file (normalised), line and the real source line(s). usage: rowsrc.py <family> <label> <regex> [context]"""
import os, re, sys
GEN = os.path.dirname(os.path.dirname(os.path.abspath(__file__))) + "/🗑️generated"
ROOT = "/Users/ueli/Documents/semio/"
fam, label, rx = sys.argv[1:4]
ctx = int(sys.argv[4]) if len(sys.argv) > 4 else 0
seen = set()
for l in open(f"{GEN}/ohf-rows-{fam}-{label}.tsv", encoding="utf-8"):
    x = l.rstrip("\n").split("\t")
    if not re.search(rx, x[5]):
        continue
    f = os.path.normpath(ROOT + x[3])
    key = (f, x[4])
    if key in seen:
        continue
    seen.add(key)
    try:
        lines = open(f, encoding="utf-8").read().split("\n")
    except Exception as e:
        print("??", f, e)
        continue
    n = int(x[4])
    print(f"{x[0]} {f[len(ROOT):]}:{n}  [{x[5][:100]}]")
    for i in range(max(0, n - 1 - ctx), min(len(lines), n + ctx)):
        print("   ", i + 1, lines[i][:240])
