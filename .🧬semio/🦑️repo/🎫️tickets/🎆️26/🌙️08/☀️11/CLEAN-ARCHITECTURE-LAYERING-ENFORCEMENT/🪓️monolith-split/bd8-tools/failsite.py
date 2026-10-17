#!/usr/bin/env python3
"""Usage: failsite.py <bun-test.log> : prints, for every failing expect, the source text of the failing call (the column bun reports is the end of the expect(...) call)."""
import re, sys
log = open(sys.argv[1]).read()
seen = set()
for m in re.finditer(r'at <anonymous> \((/[^)]*?\.ts):(\d+):(\d+)\)', log):
    f, l, c = m.group(1), int(m.group(2)), int(m.group(3))
    if (f, l, c) in seen: continue
    seen.add((f, l, c))
    line = open(f).read().split('\n')[l - 1]
    start = max(0, c - 230)
    print(f'{f.split("/")[-2]} L{l}:C{c}: ...{line[start:c+10]}')
