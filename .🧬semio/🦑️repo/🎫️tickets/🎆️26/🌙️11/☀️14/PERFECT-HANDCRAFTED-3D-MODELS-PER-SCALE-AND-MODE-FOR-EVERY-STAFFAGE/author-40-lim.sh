#!/bin/bash
# usage: author-40-lim.sh slug scale [list]  -> solids below their C13 limit
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$(dirname "$0")"
$BUN author-24-limits.ts humans/$1 $2 ${3:-body} 2>&1 | python3 -c "
import sys,re
for l in sys.stdin:
    m=re.search(r'min=([\d.]+)mm limit=([\d.]+)mm',l)
    if m and float(m.group(1))<float(m.group(2))-0.05: print(l.rstrip())
"
