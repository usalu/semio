"""🧾️ Compares the verbose test lines of two runs of suites.sh: `python3 results.py <tag-a> <tag-b>` prints per suite the counts of passed and failed lines of each run and every test whose result differs or that only one run has."""
import glob, os, re, sys
gen = os.environ["GEN"] + "/vt"


def lines(tag, suite):
    out = {}
    for line in open(f"{gen}/{tag}-{suite}.log", errors="replace"):
        m = re.match(r"\s*(✓|×|✗|↓|❯)\s+(.*?)(\s+\d+(\.\d+)?\s*m?s)?\s*$", re.sub(r"\x1b\[[0-9;]*m", "", line))
        if m and m.group(1) != "❯":
            out[m.group(2)] = m.group(1)
    return out


for path in sorted(glob.glob(f"{gen}/{sys.argv[1]}-*.log")):
    suite = os.path.basename(path)[len(sys.argv[1]) + 1:-4]
    a, b = lines(sys.argv[1], suite), lines(sys.argv[2], suite)
    count = lambda r: f"{sum(1 for v in r.values() if v == '✓')} passed {sum(1 for v in r.values() if v in '×✗')} failed {sum(1 for v in r.values() if v == '↓')} skipped"
    print(f"{suite}: {sys.argv[1]} {count(a)} | {sys.argv[2]} {count(b)}")
    for name in sorted(set(a) | set(b)):
        if a.get(name) != b.get(name) and not (a.get(name) in (None, "↓") and b.get(name) in (None, "↓")):
            print(f"   {a.get(name, '-')} → {b.get(name, '-')} {name[:200]}")
