#!/usr/bin/env python3
"""[mem2] Prints the phase marks of a harness log as a table (streams the log). usage: marks.py <log>. Not a library file."""
import json, sys
for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
    if "[DEBUG] M2 " not in line:
        continue
    r = json.loads(line.split("[DEBUG] M2 ", 1)[1])
    print(f"{r['label']:24} t={r['t']/1000:7.1f}s rss={r['rss']:5} max={r['maxRss']:5} heap={r['heap']:5} extra={r['extra']:5} fp={r.get('fp')} fpPeak={r.get('fpPeak')} | {r.get('top','')[:100]}")
