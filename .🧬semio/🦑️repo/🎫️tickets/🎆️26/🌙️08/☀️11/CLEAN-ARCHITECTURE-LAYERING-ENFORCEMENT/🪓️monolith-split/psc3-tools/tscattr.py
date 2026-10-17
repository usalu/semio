"""Compares two tsc outputs (multiset of file, code, message ignoring positions) and attributes the differences to the touched files and facades of a plan. Usage: python3 tscattr.py before.txt after.txt report.json plan.json..."""
import re, sys, json, collections
b, a, rep = sys.argv[1:4]
touched = set(json.load(open(rep))["files"].keys())
fac = set()
for p in sys.argv[4:]:
    fac |= {f["facade"] for f in json.load(open(p))["facades"]}
def load(p):
    c = collections.Counter()
    for l in open(p, encoding="utf8"):
        m = re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$", l)
        if m:
            msg = re.sub(r"(?:'(?:\.\./)+[^']*'|\"(?:\.\./)+[^\"]*\")", "<relpath>", m.group(5))
            msg = re.sub(r"@semio-tech/[\w./-]+", "<pkg>", msg)
            c[(m.group(1).replace("/Users/ueli/Documents/semio/", ""), m.group(4), msg[:140])] += 1
    return c
B, A = load(b), load(a)
removed, added = B - A, A - B
def where(k):
    return "touched" if (k[0] in touched or k[0] in fac) else "other"
for name, d in (("removed", removed), ("added", added)):
    t = collections.Counter(); 
    for k, v in d.items(): t[where(k)] += v
    print(name, dict(t), "total", sum(d.values()))
print("--- added in touched files / facades:")
for k, v in added.items():
    if where(k) == "touched": print(" +", v, k[0][-70:], k[1], k[2][:110])
print("--- removed in touched files / facades:")
n = 0
for k, v in removed.items():
    if where(k) == "touched":
        n += 1
        if n <= 12: print(" -", v, k[0][-70:], k[1], k[2][:110])
print("removed touched kinds", n)
