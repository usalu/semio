#!/bin/bash
# usage: author-12-fails.sh <slug> [scale]
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$(dirname "$0")"
$BUN volumes-check.ts appliances/$1 ${2:+--scale $2} 2>&1 | grep "^FAIL\|PASS\|^  volumes" | python3 -c "
import sys,re
for l in sys.stdin:
    l=l.rstrip()
    if l.startswith('  volumes'): print(l[:260]); continue
    for m in re.findall(r'![A-Za-z0-9:._-]+=[0-9.eE+-]+[<>]=[0-9.eE+-]+(?: \[[^\]]*\])?',l): print('   ',l.split()[1],l.split()[2],m[:200])
    if l.startswith('PASS'): print(l)
"
