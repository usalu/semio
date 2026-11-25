"""📋️ The hard findings of gate logs by code: `python3 codes.py <dir> [<id>…]` prints per asset one line `code ×count (worst mm)` summed over the scales, from the logs `<dir>/<id>.log` or `<dir>/<id>-gate.log` of `../MR/gate.ts`."""
import re, sys, os, collections
d = sys.argv[1]
ids = sys.argv[2:] or sorted({re.sub(r"(-gate)?\.log$", "", f) for f in os.listdir(d) if f.endswith(".log") and ("-gate" in f or not re.search(r"-(scan|measure|check|matrix)\.log$", f))})
for a in ids:
    path = next((p for p in (os.path.join(d, a + ".log"), os.path.join(d, a + "-gate.log")) if os.path.exists(p)), None)
    if path is None: continue
    codes = collections.defaultdict(lambda: [0, 0])
    total = "?"
    for line in open(path):
        m = re.match(r"\S+ 1:\d+ HARD (\S+) ×(\d+) worst (\d+) mm", line)
        if m: codes[m.group(1)][0] += int(m.group(2)); codes[m.group(1)][1] = max(codes[m.group(1)][1], int(m.group(3)))
        m = re.match(r"hard findings: (\d+)", line)
        if m: total = m.group(1)
    print("%s | %s | %s" % (a, total, ", ".join("`%s` ×%d (%d mm)" % (c, n, mm) for c, (n, mm) in sorted(codes.items())) or "none"))
