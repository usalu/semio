#!/bin/zsh
# usage: r.sh <path relative to caching dir> ...
C="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
O="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1125/🗑️generated/F18"
cd /Users/ueli/Documents/semio
for f in "$@"; do
  while :; do l=$(uptime | sed 's/.*averages: //' | cut -d' ' -f1 | cut -d. -f1); [ "$l" -le 38 ] && break; sleep 20; done
  n=$(echo "$f" | md5 | cut -c1-6)
  s=$(date +%s)
  repo test-file "$C/$f" > "$O/r-$n.txt" 2>&1; rc=$?
  echo "=== $f rc=$rc $(( $(date +%s)-s ))s"
  grep -E "^ *[0-9]+ (pass|fail)$" "$O/r-$n.txt" | tr '\n' ' '; echo
  grep -E "^error:" "$O/r-$n.txt" | cut -c1-220 | sort | uniq -c | sort -rn | head -4
done
