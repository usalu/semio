#!/bin/zsh
# usage: run1.sh <relative file under C> <tag>
C="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122/🗑️generated/E48"
while :; do l=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1); [ "$l" -le 40 ] && break; sleep 30; done
cd /Users/ueli/Documents/semio
n=$(echo "$1" | md5 | cut -c1-8)
repo test-file --json "$C/$1" > "$OUT/$2-$n.json" 2> "$OUT/$2-$n.err"
echo "exit $?"
python3 - "$OUT/$2-$n.json" <<'PY'
import json,sys
try:
  d=json.load(open(sys.argv[1]))
  for f in d.get('files',[]): print(f.get('status'),f.get('pass'),f.get('fail'),f.get('suite'),f.get('mode'),str(f.get('error'))[:200])
except Exception as e: print("noparse",e)
PY
