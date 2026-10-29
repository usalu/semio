#!/bin/bash
# usage: retry.sh <n> [--target T] crates...   (re-runs chk.sh while the compiler image reports ENOSPC, max 5 attempts)
D="$(dirname "$0")"
N=$1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
cd /Users/ueli/Documents/semio
wait_space(){ while [ "$(df -k '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m' | tail -1 | awk '{print $4}')" -lt 6000000 ]; do sleep 60; done; }
for attempt in 1 2 3 4 5; do
  "$D/chk.sh" "$@"
  if grep -qE 'No space left on device|ENOSPC|compiler image (ran out|is still full)' "$C/🗑️generated/1029/semiodsl-chk$N.txt"; then
    cp "$C/🗑️generated/1029/semiodsl-chk$N.txt" "$C/🗑️generated/1029/semiodsl-chk$N.enospc$attempt.txt"
    wait_space
  rm -f "$C/🗑️generated/1029/semiodsl-chk$N.rc"
    sleep 300
  else
    break
  fi
done
[ -f "$C/🗑️generated/1029/semiodsl-chk$N.rc" ] || echo 99 > "$C/🗑️generated/1029/semiodsl-chk$N.rc"
