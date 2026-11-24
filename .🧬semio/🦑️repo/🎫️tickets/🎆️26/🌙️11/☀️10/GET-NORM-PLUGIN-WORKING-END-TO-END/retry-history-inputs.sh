#!/bin/sh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
for f in "$@"; do
  n=0
  while [ $n -lt 12 ]; do
    rm -f "$T/🗑️generated/labels-$f.done" "$T/🗑️generated/retry-$f.done"
    "$T/run-history-inputs-families.sh" $f
    L="$T/🗑️generated/labels-$f.log"
    if grep -qE 'exceeds its source boundary before|failed to build archive|No space left' "$L" || grep -E '^error: could not compile `' "$L" | grep -vE 'norm-(en|din|vdi|iso)' | grep -q .; then n=$((n+1)); sleep 180; else break; fi
  done
  touch "$T/🗑️generated/retry-$f.done"
done
