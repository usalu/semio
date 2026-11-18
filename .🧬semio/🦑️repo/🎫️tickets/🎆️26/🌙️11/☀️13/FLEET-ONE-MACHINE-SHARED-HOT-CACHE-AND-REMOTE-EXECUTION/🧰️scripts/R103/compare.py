import re, sys
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R103/"
def load(n):
    rows = {}
    for l in open(SC + n + ".txt", encoding="utf8"):
        m = re.match(r"(.*?) :\s*(?:(\d+) pass)?\s*(?:(\d+) fail)?", l)
        if l.startswith("MISSING"): rows[l.strip()] = (0, 99); continue
        if m and " :" in l: rows[m.group(1)] = (int(m.group(2) or 0), int(m.group(3) or 0))
    return rows
b, a = load("before"), load("after")
bad = [(k, b.get(k), v) for k, v in a.items() if v[1] > b.get(k, (0, 0))[1] and "set-aside" not in k or (v[1] > 0 and "set-aside" in k)]
for k in b:
    if k not in a: bad.append((k, b[k], None))
print("RED" if bad else "GREEN", bad)
sys.exit(1 if bad else 0)
