#!/bin/bash
# usage: seq.sh <tag> <name>|<dir> ...   runs each workspace test sequentially, retries on ENOSPC
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
FREE="$ROOT/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m"
tag="$1"; shift
for spec in "$@"; do
  n="${spec%%|*}"; d="${spec#*|}"
  for try in 1 2 3 4; do
    while [ "$(df -k "$FREE" | awk 'NR==2{print int($4/1048576)}')" -lt 6 ]; do sleep 30; done
    "$C/1029-tools/rta/run.sh" "$n-$tag$try" "$d" --workspace --lib --tests --no-fail-fast
    if grep -q 'No space left on device' "$C/🗑️generated/1029/rta-$n-$tag$try.txt"; then continue; fi
    cp "$C/🗑️generated/1029/rta-$n-$tag$try.rc" "$C/🗑️generated/1029/rta-$n-final.rc"; cp "$C/🗑️generated/1029/rta-$n-$tag$try.txt" "$C/🗑️generated/1029/rta-$n-final.txt"
    break
  done
done
echo done > "$C/🗑️generated/1029/rta-seq-$tag.done"
