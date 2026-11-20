#!/bin/sh
# 📐 Refits the length factors FIT of the outer fronds with r3-date-palm-reach.ts (three passes).
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
mkdir -p "🗑️generated/r3-date-palm"
for it in 1 2 3; do
$BUN r3-date-palm-reach.ts > "🗑️generated/r3-date-palm/reach.txt"
python3 - <<'PY'
import re
rows=[l.split() for l in open('🗑️generated/r3-date-palm/reach.txt') if l[0].isdigit()]
p='r3-date-palm-rig-fix.ts'
s=open(p).read()
m=re.search(r'const FIT: Record<number, number> = \{(.*?)\};',s)
cur={}
for kv in m.group(1).split(','):
  if ':' in kv: k,v=kv.split(':'); cur[int(k)]=float(v)
for r in rows:
  k=int(r[0]); f=float(r[-1]); cur[k]=round(cur.get(k,1)*f,4)
new='const FIT: Record<number, number> = {'+', '.join(f'{k}: {v}' for k,v in sorted(cur.items()) if v!=1)+'};'
open(p,'w').write(s[:m.start()]+new+s[m.end():])
print(new)
PY
done
tail -1 "🗑️generated/r3-date-palm/reach.txt"
