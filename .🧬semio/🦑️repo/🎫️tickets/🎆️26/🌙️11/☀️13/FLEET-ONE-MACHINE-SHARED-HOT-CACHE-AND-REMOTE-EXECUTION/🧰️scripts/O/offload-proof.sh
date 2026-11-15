#!/bin/zsh
# usage: offload-proof.sh <crate> [always|auto] [outfile]   one real offloaded cargo build of a small crate through the fleet, then the unit events of its lane (which origin compiled which unit, declines)
CRATE="$1"; MODE="${2:-always}"; OUT="${3:-/dev/stdout}"
unset SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_TOKEN SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_INSTANCE SEMIO_REMOTE_EXECUTION_ENDPOINT
cd /Users/Shared/semio || exit 2
echo "start $(date +%T) load $(uptime | sed 's/.*averages: //')" > "$OUT"
S=$(date +%s)
repo cargo --offload="$MODE" --store=shared --configuration=prototype -- build -p "$CRATE" > "$OUT.build" 2>&1; CODE=$?
echo "exit $CODE wall $(( $(date +%s) - S )) s, Compiling lines $(grep -c Compiling "$OUT.build")" >> "$OUT"
cd .cache/b/r/m/u/x || exit 0
L=$(ls -t p*.jsonl | head -1)
python3 - "$L" >> "$OUT" <<'PY'
import json,collections,sys
lane=sys.argv[1][:-6]
c=collections.Counter(); origins=collections.Counter(); ms=[]; det=collections.Counter()
for l in open('offload.jsonl'):
    try: r=json.loads(l)
    except: continue
    if r.get('lane')!=lane: continue
    k=r.get('kind'); c[(k,r.get('reason'))]+=1
    if k=='offload-run': origins[r.get('origin')]+=1; ms.append(r.get('milliseconds',0))
    elif r.get('detail'): det[(r.get('reason'),r['detail'][:90])]+=1
print('lane',lane)
for k,v in c.most_common(): print(' ',v,k)
print(' offload-run origins',dict(origins),'sum ms',sum(ms),'max',max(ms) if ms else 0,'avg',sum(ms)//max(1,len(ms)))
for k,v in det.most_common(10): print('  detail',v,k)
d=collections.Counter()
for l in open(sys.argv[1]):
    try: r=json.loads(l)
    except: continue
    d[r.get('kind')]+=1
print('unit events',dict(d))
PY
