#!/bin/sh
# Live proof: holders before and after one opener sweep must agree (modulo finished processes), plus df and file counts.
cd /Users/ueli/Documents/semio || exit 1
OUT="$1"; mkdir -p "$OUT"
snap() { bun repo leases status --json 2>/dev/null | python3 -c "
import json,sys
t=sys.stdin.read(); t=t[t.index('{'):]
d=json.loads(t)
rows=sorted({(h['resource'],h['pid'],h['mode']) for h in d['holders']})
json.dump({'holders':rows,'waiters':len(d['waiters']),'cycles':len(d['cycles'])},open('$OUT/$1.json','w'))
print('$1',len(rows),'holders',len(d['waiters']),'waiters',len(d['cycles']),'cycles')
"; df -h .cache/b/l/m | tail -1; ls .cache/b/l/m/data | awk '/\.sqlite$/{a++} /\.queue$/{b++} /^(t|i|proof)-/{n++} END{print "sqlite",a,"queue",b,"nested",n+0}'; ls .cache/b/l/m/data/claims | wc -l | awk '{print "claims",$1}'; }
snap before
bun repo cache status >/dev/null 2>"$OUT/opener.stderr"
grep -a 'repo leases' "$OUT/opener.stderr" | head
snap after
python3 - <<P
import json,os
b=json.load(open('$OUT/before.json')); a=json.load(open('$OUT/after.json'))
alive=lambda pid: os.path.exists('/proc/%d'%pid) or os.system('kill -0 %d 2>/dev/null'%pid)==0
lost=[h for h in b['holders'] if h not in a['holders'] and alive(h[1])]
print('holders lost while their process is alive:',len(lost)); [print(' ',h) for h in lost[:10]]
P
