#!/bin/sh
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-check.ts furniture/$1 2>&1 | grep "^FAIL\|^ok" | grep "^FAIL" | python3 -c "
import sys,re
for l in sys.stdin:
    head=l[:45].split()
    toks=re.findall(r'!\S+=[^ ]+(?: \[[^\]]*\])?',l)
    print(' '.join(head[1:3]),' | '.join(t[:170] for t in toks))
"
