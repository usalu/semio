"""🗜️ Condenses the cluster lines of `cl.sh` read from stdin: per cluster the side, place and distance, the three nearest drawn shapes, the three nearest solids and the first covering shapes."""
import re
import sys

for line in sys.stdin:
    line = line.rstrip()
    if line.startswith("=="):
        print(line)
        continue
    match = re.match(r"\s+(\S+) \(([^)]*)\) d=(\S+)\s+drawn: (.*?) \| vol: (.*?) \| covering: (.*)", line)
    if not match:
        continue
    side, place, d, drawn, vol, cover = match.groups()
    near = lambda text: " ".join(sorted(text.split(), key=lambda item: float(item.rsplit("]", 1)[1]))[:3])
    print(f"  {side} ({place}) {d} | D {near(drawn)} | V {near(vol)} | C {' '.join(cover.split()[:3])}")
