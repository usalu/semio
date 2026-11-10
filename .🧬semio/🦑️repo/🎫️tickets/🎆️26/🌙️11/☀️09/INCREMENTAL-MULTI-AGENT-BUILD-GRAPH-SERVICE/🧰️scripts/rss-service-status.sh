#!/usr/bin/env bash
# usage: rss-service-status.sh <out> <seconds> [interval]   polls `repo service host status` (memory report) and appends one JSON line per sample
out="$1"; end=$((SECONDS+$2)); : > "$out"
while [ $SECONDS -lt $end ]; do
  timeout 20 repo service host status --output=json 2>/dev/null | python3 -c "
import sys,json,re
t=sys.stdin.read();i=t.find('{');j=t.rfind('}')
try:
  d=json.loads(t[i:j+1]);m=d.get('memory',{})
  print(json.dumps({'t':$SECONDS,'rss':m.get('rssMb'),'hwm':m.get('hwmMb'),'heap':m.get('heapMb'),'extra':m.get('extraMb'),'objs':m.get('objectCount'),'types':[(x['type'],x['count']) for x in m.get('objectTypes',[])[:8]],'caches':[(c['name'],c['entries'],round(c['bytes']/1048576,1)) for c in m.get('caches',[]) if c['entries']],'indexBytesMb':[round(w.get('indexBytes',0)/1048576) for w in d.get('workspaces',[])],'inFlight':d.get('inFlight')}))
except Exception as e: print(json.dumps({'t':$SECONDS,'error':str(e)[:80]}))
" >> "$out"
  sleep "${3:-8}"
done
