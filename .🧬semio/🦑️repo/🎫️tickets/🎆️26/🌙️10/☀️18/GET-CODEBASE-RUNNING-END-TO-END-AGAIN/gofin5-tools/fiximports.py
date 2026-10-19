#!/usr/bin/env python3
"""usage: fiximports.py <real module dir> < vet-output -- removes imports reported as 'imported and not used' from the real sources"""
import os, re, sys
mod = sys.argv[1]
drop = {}
for line in sys.stdin:
    m = re.match(r'(?:vet: )?(.+?):(\d+):\d+: "?([^"\s]+)"? (?:\(.+\) )?imported (?:as \w+ )?and not used', line.strip())
    if not m:
        continue
    path = os.path.normpath(os.path.join(mod, m.group(1)))
    drop.setdefault(path, set()).add(int(m.group(2)))
for path, lines in drop.items():
    src = open(path, encoding='utf8').read().split('\n')
    out = [l for i, l in enumerate(src, 1) if i not in lines]
    open(path, 'w', encoding='utf8').write('\n'.join(out))
    print('fixed', path, sorted(lines))
