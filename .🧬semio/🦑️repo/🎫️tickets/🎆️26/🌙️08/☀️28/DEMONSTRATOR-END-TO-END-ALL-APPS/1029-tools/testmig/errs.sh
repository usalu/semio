#!/bin/bash
# usage: errs.sh <name>  -> errors with location, grouped
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1029"
python3 - "$C/testmig-$1.txt" <<'PY'
import re,sys
t=open(sys.argv[1]).read()
blocks=re.split(r'\n(?=(?:error|warning)(?:\[E\d+\])?: )',t)
out=[]
for b in blocks:
    if not b.startswith("error"): continue
    head=b.split("\n")[0]
    m=re.search(r'--> (\S+):(\d+):(\d+)',b)
    loc=m.group(1).split("subsets/")[-1]+":"+m.group(2) if m else "?"
    out.append((loc,head[:200]))
for l,h in out: print(l,"|",h)
print(len(out),"errors")
PY
