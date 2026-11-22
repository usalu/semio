#!/bin/zsh
# usage: run1.sh <path relative to the caching library folder> <tag>
C="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E57"
while :; do l=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1); f=$(memory_pressure | tail -1 | sed 's/[^0-9]*\([0-9]*\)%.*/\1/'); [ "$l" -lt 40 ] && [ "$f" -ge 25 ] && break; sleep 60; done
cd /Users/ueli/Documents/semio
n=$(echo "$1" | md5 | cut -c1-8)
repo test-file --json "$C/$1" > "$OUT/$2-$n.json" 2> "$OUT/$2-$n.err"
python3 - "$OUT/$2-$n.json" "$1" <<'PY'
import json,sys
try:
  d=json.load(open(sys.argv[1]))
  for f in d.get('files',[]): print(sys.argv[2].split('/')[-3:-1],f.get('status'),f.get('pass'),f.get('fail'),f.get('suite'),str(f.get('error'))[:160].replace("\n"," "))
except Exception as e: print("noparse",sys.argv[2],e)
PY
