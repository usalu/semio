#!/bin/bash
# usage: job.sh <cmd> <ids,comma> [json-extra]   e.g. job.sh probe park-bench '{"scales":[200]}'
J="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-FO/jobs"
N=$(date +%s%N)
python3 - "$1" "$2" "${3:-{\}}" "$J/$N.job.json.tmp" <<'PY'
import json,sys
cmd,ids,extra,out=sys.argv[1:5]
d={'cmd':cmd,'ids':ids.split(',')}
d.update(json.loads(extra))
json.dump(d,open(out,'w'))
PY
mv "$J/$N.job.json.tmp" "$J/$N.job.json"
for i in $(seq 1 2400); do [ -f "$J/$N.out" ] && break; sleep 0.5; done
cat "$J/$N.out"; rm -f "$J/$N.out"
