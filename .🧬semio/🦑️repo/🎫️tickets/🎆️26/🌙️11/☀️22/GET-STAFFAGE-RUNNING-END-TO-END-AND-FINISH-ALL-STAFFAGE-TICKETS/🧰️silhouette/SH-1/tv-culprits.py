"""🕵️ Reads the output of `🧰️three-views/tv1-sweep.ts <id> --detail 1` and counts, per scale and pair of views, which two shapes set the edge that differs most at the failing instants (the first is the view named first in the finding): python3 tv-culprits.py <file>…"""
import collections, re, sys

for path in sys.argv[1:]:
    count, worst = collections.Counter(), {}
    lines = open(path, encoding="utf-8").read().splitlines()
    for i, line in enumerate(lines):
        m = re.match(r"\[(spread-[a-z-]+)\] 1:(\d+) (\S+) (.*?) \(.*tolerance ([\d.]+) m\).*: \[(-?[\d.]+), (-?[\d.]+)\] vs \[(-?[\d.]+), (-?[\d.]+)\]", line)
        if not m:
            continue
        a0, a1, b0, b1 = map(float, m.group(6, 7, 8, 9))
        edge = 0 if abs(a0 - b0) >= abs(a1 - b1) else 1
        detail = [l.strip() for l in lines[i + 1:i + 5]]
        if len(detail) < 4 or ":" not in detail[0]:
            continue
        first = lambda l: l.split(": ", 1)[1].split(" | ")[0].rsplit(" ", 1)[0]
        key = (m.group(2), m.group(1), "min" if edge == 0 else "max", first(detail[edge]), first(detail[2 + edge]))
        count[key] += 1
        d = abs(a0 - b0) if edge == 0 else abs(a1 - b1)
        worst[key] = max(worst.get(key, 0), d)
    print(path.rsplit("/", 1)[-1], lines[-1][:100] if lines else "")
    for key, n in count.most_common(12):
        print(f"  {n:3d}× 1:{key[0]} {key[1]} {key[2]}: {key[3]} vs {key[4]} (up to {worst[key] * 1000:.0f} mm)")
