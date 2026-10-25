#!/bin/sh
# usage: watch-image.sh <run-id> : while the exact-cargo-storage image test runs, snapshot its process tree and the private lease directory every 20 s
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
L="$C/🗑️generated/exact-d/$1-exact-cargo-storage.log"
O="$C/🗑️generated/exact-d/$1-image-watch.txt"
while ! /usr/bin/grep -q "^exact-cargo-storage " "$C/🗑️generated/exact-d/$1-summary.txt" 2>/dev/null; do
  if /usr/bin/grep -q 'image/🟦️.ts:' "$L" 2>/dev/null; then
    { date +%T; ps -eo pid,ppid,etime,%cpu,command | /usr/bin/grep -E "hdiutil|diskutil|exact-cargo-storage|cargo (test|build|metadata)|rustc " | /usr/bin/grep -v grep | cut -c1-200; ls -la "🧰️framework/🔨️modules/🏃️process/.cache/b/l/m/data" 2>/dev/null | wc -l; echo ---; } >> "$O"
  fi
  sleep 20
done
