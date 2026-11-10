#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
n=$1; shift
for e in "$@"; do
 m="${e%%|*}"; p="${e##*|}"
 "$C/1105b-tools/chk.sh" W-a11y2-$n "$m" wasm -p "$p"
 echo "W-a11y2-$n $p rc=$(cat "$C/🗑️generated/1105b/W-a11y2-$n.rc")" >> "$C/🗑️generated/1105b/a11y2-chain.log"
 n=$((n+1))
done
