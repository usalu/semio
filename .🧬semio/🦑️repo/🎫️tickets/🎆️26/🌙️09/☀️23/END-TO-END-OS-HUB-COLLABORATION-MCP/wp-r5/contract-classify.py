import re, sys, collections
def rows(path):
    out = []
    for line in open(path, encoding="utf-8"):
        m = re.match(r"^  (testing/\S+)  (\S+)  (.*)$", line.rstrip("\n"))
        if m: out.append(m.groups())
    return out
def norm(s):
    s = re.sub(r"`[^`]*`", "`X`", s)
    s = re.sub(r"@[\w.-]+", "@X", s)
    s = re.sub(r"\b[a-z0-9]+(?:[.-][a-z0-9]+)+\b", "X", s)
    s = re.sub(r"\d+", "N", s)
    s = re.sub(r"\S*[☀-\U0001FFFF]\S*", "P", s)
    return s[:150]
def owner(scope):
    parts = scope.split("/")
    if parts[0] == "✏️s" and len(parts) > 2: return "/".join(parts[:3])
    return "/".join(parts[:3])
a = rows(sys.argv[1])
c = collections.Counter((k, norm(s)) for k, _, s in a)
if len(sys.argv) > 2:
    b = rows(sys.argv[2]); d = collections.Counter((k, norm(s)) for k, _, s in b)
    print(f"before {len(a)} after {len(b)}")
    for key in sorted(set(c) | set(d), key=lambda k: (d[k] - c[k], k)):
        if c[key] != d[key]: print(f"{d[key]-c[key]:+6d} {c[key]:6d}->{d[key]:6d}  {key[0]}  {key[1]}")
else:
    print(len(a))
    for (k, s), n in c.most_common(int(sys.argv[3]) if len(sys.argv) > 3 else 80): print(f"{n:6d}  {k}  {s}")
