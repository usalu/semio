#!/bin/bash
# usage: author-69-chk.sh humans/slug [extra args]  -> only failing tokens
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-check.ts "$1" --product "${@:2}" 2>&1 | python3 -c '
import sys,re
for l in sys.stdin:
    l=l.rstrip("\n")
    if l.startswith("FAIL") and "FAIL:" not in l[:6]:
        h=l.split("[p]")[0].strip()
        toks=[t for t in l.split() if t.startswith("!")]
        print(h, toks)
    elif l.startswith("ok") : pass
    else: print(l[:300])
'
