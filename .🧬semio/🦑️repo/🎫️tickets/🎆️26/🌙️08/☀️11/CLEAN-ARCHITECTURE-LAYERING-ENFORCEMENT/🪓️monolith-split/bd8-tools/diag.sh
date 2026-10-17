#!/bin/zsh
# usage: diag.sh <test.ts> -> runs a soft-expect copy (bd8-diag.ts next to it), prints every failing assertion site, deletes the copy
T="$1"; D="$(dirname "$T")"; C="$D/bd8-diag.ts"; F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
export BD8_SOFT_LOG="$F/🗑️generated/bd8-soft.log"; : > "$BD8_SOFT_LOG"
python3 - "$T" "$C" <<'PY'
import re,sys
s=open(sys.argv[1]).read()
m=re.search(r'import\s*\{([^}]*)\}\s*from\s*"bun:test";?',s)
names=[n.strip() for n in m.group(1).split(",") if n.strip() and n.strip()!="expect"]
shim="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/bd8-tools/softexpect.ts"
s2=s[:m.start()]+'import { '+', '.join(names)+' } from "bun:test"; import { expect } from "'+shim+'";'+s[m.end():]
assert s2!=s
open(sys.argv[2],"w").write(s2)
PY
( cd /Users/ueli/Documents/semio && ulimit -n 65536 && nice -n 10 bun test "$C" > "$F/🗑️generated/bd8-diag-run.log" 2>&1 )
rm -f "$C"
python3 - "$BD8_SOFT_LOG" "$T" <<'PY'
import re,sys
log=open(sys.argv[1]).read().strip().split("\n")
src=open(sys.argv[2]).read().split("\n")
seen=set()
for l in log:
    m=re.search(r'\(([^)]*bd8-diag\.ts):(\d+):(\d+)\)',l)
    if not m: print("?",l[:200]); continue
    ln,col=int(m.group(2)),int(m.group(3))
    if (ln,col) in seen: continue
    seen.add((ln,col))
    line=src[ln-1]
    print(f"L{ln}:C{col}: ...{line[max(0,col-190):col+5]}")
print("failing assertions:",len(seen))
PY
