"""errdiff.py <base.err> <cur.err>: multiset diff of (message, quoted source line without line number)."""
import re, sys, collections
def rows(fn):
    lines = open(fn).read().splitlines(); out = collections.Counter()
    for i, l in enumerate(lines):
        if l.startswith("error") and "aborting" not in l and "temp dir" not in l:
            src = ""
            for j in range(i + 1, min(i + 6, len(lines))):
                m = re.match(r"\s*\d+ \| (.*)", lines[j])
                if m: src = m.group(1).strip(); break
            out[(l, src)] += 1
    return out
b, c = rows(sys.argv[1]), rows(sys.argv[2])
print("base", sum(b.values()), "cur", sum(c.values()))
for k in sorted(set(b) | set(c)):
    if b[k] != c[k]: print(("NEW " if c[k] > b[k] else "GONE"), b[k], "->", c[k], k[0][:110], "|", k[1][:150])
