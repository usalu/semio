"""Streams a gate log and prints per-spec durations: project, file, seconds, title."""
import re, sys, collections
pat = re.compile(r"^\[(dev|rehearsal)\]\s+(✓|✘)\s+(\d+) \[(\w+)\] › .*?tests/([^/]+)/(?:(?:[^/]+)/)?[^/:]*:(\d+):\d+ › (.*) \(([\d.]+)(ms|s|m)\)\s*$")
rows = []
for line in open(sys.argv[1], errors="replace"):
    line = re.sub(r"\x1b\[[0-9;]*m", "", line.rstrip("\n"))
    m = pat.match(line)
    if not m: continue
    v = float(m[8]); v = v/1000 if m[9] == "ms" else v*60 if m[9] == "m" else v
    rows.append((m[1], m[4], m[5], int(m[6]), v, m[7][:80]))
by = collections.defaultdict(lambda: [0, 0.0])
for r in rows: by[(r[0], r[1])][0] += 1; by[(r[0], r[1])][1] += r[4]
for k, (n, s) in by.items(): print(f"{k[0]:10} {k[1]:10} n={n:2} sum={s:7.0f}s")
print("TOTAL", f"{sum(r[4] for r in rows):.0f}s", len(rows))
if len(sys.argv) > 2:
    for r in sorted(rows, key=lambda r: -r[4])[:int(sys.argv[2])]: print(f"{r[4]:7.1f} {r[1]:9} {r[2]} :{r[3]} {r[5]}")
