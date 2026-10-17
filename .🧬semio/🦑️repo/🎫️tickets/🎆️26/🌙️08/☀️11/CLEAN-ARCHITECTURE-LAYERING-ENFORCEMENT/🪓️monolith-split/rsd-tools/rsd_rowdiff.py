"""📊 Row-multiset diff of two POS oracle run logs, per cfg section (lib, test).

usage: rsd_rowdiff.py <base.log> <post.log>
Rows are the non-stub report lines `CODE [unit] file xN lines [..]: message`; the key drops the xN count and the line numbers so a moved line is not a new row.
"""
import collections, re, sys

ROW = re.compile(r"^\s+(E\d{4}) \[([^\]]*)\] (\S.*?) x(\d+) lines \[[^\]]*\]: (.*)$")


def rows(path):
    cfg, out = None, {}
    for line in open(path, encoding="utf-8", errors="replace"):
        m = re.match(r"-- cfg (\w+):", line)
        if m:
            cfg = m.group(1)
            out.setdefault(cfg, collections.Counter())
            continue
        m = ROW.match(line.rstrip("\n"))
        if m and cfg:
            out[cfg][(m.group(1), m.group(2), m.group(3), m.group(5))] += int(m.group(4))
    return out


base, post = rows(sys.argv[1]), rows(sys.argv[2])
for cfg in sorted(set(base) | set(post)):
    b, p = base.get(cfg, collections.Counter()), post.get(cfg, collections.Counter())
    only_b, only_p = b - p, p - b
    print(f"== {cfg}: base {sum(b.values())} rows, post {sum(p.values())} rows, gone {sum(only_b.values())}, new {sum(only_p.values())}")
    for tag, rs in (("GONE", only_b), ("NEW ", only_p)):
        for (code, unit, f, msg), n in sorted(rs.items()):
            print(f"  {tag} {code} [{unit}] {f} x{n}: {msg[:140]}")
