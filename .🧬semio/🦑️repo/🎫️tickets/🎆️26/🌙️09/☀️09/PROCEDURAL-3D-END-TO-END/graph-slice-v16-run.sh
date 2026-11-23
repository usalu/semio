#!/bin/sh
# Waits until the procedural component is staged or the deadline epoch ($1) passes, then runs ONE read detached into 🗑️generated/react-v16.txt; start and end epochs go to react-v16-start.txt; samples of the slice report every 30 s go to react-v16-samples.txt.
T="$(cd "$(dirname "$0")" && pwd)/🗑️generated"; cd "$(git rev-parse --show-toplevel)" || exit 1
until [ -d .cache/b/p/m/browser/prototype/modules/🌀️procedural ] || [ "$(date +%s)" -ge "$1" ]; do sleep 15; done
{ date +%s; [ -d .cache/b/p/m/browser/prototype/modules/🌀️procedural ] && echo staged || echo deadline; } > "$T/react-v16-start.txt"
( repo run @semio-tech/framework-os-dev:dev-generation3d-react-prototype > "$T/react-v16.txt" 2>&1; echo "exit $?" >> "$T/react-v16-start.txt"; date +%s >> "$T/react-v16-start.txt" ) &
RP=$!; echo $RP > "$T/react-v16-pid.txt"
while kill -0 $RP 2>/dev/null; do
  { date +%T; tail -n 1 "$T/react-v16.txt" | cut -c1-200; python3 -c "
import json;d=json.load(open('.cache/f/slice-memory.json'));m=d.get('memory',{});i=d.get('sources',{}).get('inventory',{});c=[(x['name'][:40],x['entries'],x['estimatedBytes']//1048576) for x in d.get('collections',[]) if 'owner' in x['name'] or 'Memo' in x['name']]
print('pid',d.get('pid'),'up',d.get('uptimeSeconds'),'heap',m.get('heapUsed'),'foot',d.get('footprintMegabytes'),'tree',d.get('treeFootprintMegabytes'),'active',i.get('active'),'owner',i.get('activeOwner'),'coll',c)" 2>&1; } >> "$T/react-v16-samples.txt"
  sleep 30
done
