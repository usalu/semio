#!/usr/bin/env python3
"""DB1: query folded stacks. usage: fold-query.py <folded> children <frame-substring> [depth]  |  incl <substr>..."""
import sys, collections
path, mode = sys.argv[1], sys.argv[2]
rows = []
for line in open(path, encoding='utf-8'):
    stack, count = line.rsplit(' ', 1)
    rows.append((stack.split(';'), int(count)))
def short(f): return f[:140]
if mode == 'incl':
    for sub in sys.argv[3:]:
        print(sum(c for s, c in rows if any(sub in f for f in s)), sub)
elif mode == 'children':
    sub = sys.argv[3]; depth = int(sys.argv[4]) if len(sys.argv) > 4 else 1
    agg = collections.Counter()
    for s, c in rows:
        idx = next((i for i, f in enumerate(s) if sub in f), None)
        if idx is None: continue
        key = ' > '.join(short(f) for f in s[idx+1:idx+1+depth]) or '<self>'
        agg[key] += c
    for k, v in agg.most_common(25): print(v, k)
elif mode == 'leaf':
    agg = collections.Counter()
    for s, c in rows: agg[short(s[-1])] += c
    for k, v in agg.most_common(30): print(v, k)
