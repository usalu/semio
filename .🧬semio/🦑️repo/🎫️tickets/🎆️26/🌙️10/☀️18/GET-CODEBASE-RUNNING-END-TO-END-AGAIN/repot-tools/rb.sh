#!/bin/zsh
# usage: rb.sh <substr>...  brief one-error view per suite
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio
for s in "$@"; do
  echo "=== $s"
  LOCS=1 W=140 python3 $C/repot-tools/r.py "$s" 8 2>&1 | python3 -c "
import sys,re
L=[l.rstrip('\n') for l in sys.stdin]
fails=[l for l in L if l.startswith('(fail)')]
print(len(fails),'fail |',fails[0][:110] if fails else '')
shown=0
for i,l in enumerate(L):
    if re.match(r'\s*(error|TypeError|ReferenceError|SyntaxError|Error|ENOENT)\b',l):
        print('  E:',' | '.join(x.strip()[:170] for x in L[i:i+4] if x.strip())[:420]); shown+=1
        if shown>=1: break
for l in L:
    if l.startswith('@L'): print('  ',l[:330]); break
print(L[-1][:80])
"
done
