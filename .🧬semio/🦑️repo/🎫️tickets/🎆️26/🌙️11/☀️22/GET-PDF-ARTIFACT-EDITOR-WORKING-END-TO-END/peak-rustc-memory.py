"""📈️ Peak physical footprint per rustc crate from a sample-rustc-memory.sh log: `python3 peak-rustc-memory.py <log>`."""
import re, sys
units = {"B": 1 / 1024 / 1024, "K": 1 / 1024, "M": 1, "G": 1024}
peaks = {}
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    match = re.match(r"rustc (\d+) (\S+) (\S+) ([\d.]+)([BKMG])\+?", line)
    if match:
        key = f"{match.group(2)} ({match.group(3)})"
        mb = float(match.group(4)) * units[match.group(5)]
        peaks[key] = max(peaks.get(key, 0), mb)
for key, mb in sorted(peaks.items(), key=lambda row: -row[1]):
    print(f"{mb / 1024:6.2f} GB  {key}")
