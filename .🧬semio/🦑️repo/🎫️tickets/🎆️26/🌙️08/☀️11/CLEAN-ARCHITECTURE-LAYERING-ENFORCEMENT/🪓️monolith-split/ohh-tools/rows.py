"""📋 OHH row digest (read only): groups non-stub oracle rows of a nonstub dump by code and crate, own crates only (from families/<fam>.json `own` or `crates`).
usage: rows.py <nonstub.txt> <family-json> [--ctx] [--codes E0425,E0432,...]"""
import json, re, sys, collections
txt = open(sys.argv[1], encoding="utf-8").read()
spec = json.load(open(sys.argv[2], encoding="utf-8"))
own = set(c.replace("-", "_") for c in spec.get("own", spec["crates"]))
ctx = "--ctx" in sys.argv
codes = set(sys.argv[sys.argv.index("--codes") + 1].split(",")) if "--codes" in sys.argv else None
blocks = re.split(r"\n(?=#\d+ E)", txt)
cnt = collections.Counter()
out = []
for b in blocks:
    m = re.match(r"#(\d+) (E\d+|-) (\S+?):(\d+)", b)
    if not m:
        continue
    code, f, ln = m.group(2), m.group(3), m.group(4)
    mm = re.match(r"__m_([A-Za-z0-9_]+)/(.*)", f)
    crate = mm.group(1) if mm else ""
    if (crate in own) == ctx:
        continue
    if codes and code not in codes:
        continue
    first = b.split("\n")[1] if "\n" in b else ""
    cnt[(code, crate)] += 1
    out.append((code, crate, mm.group(2) if mm else f, ln, first[:220]))
print(sum(cnt.values()), "rows")
for k, v in sorted(cnt.items()):
    print(v, *k)
print()
for r in sorted(out):
    print(r[0], r[1], r[2][-80:] + ":" + r[3], "|", r[4])
