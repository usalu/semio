#!/bin/bash
# usage: proof-retry.sh <N> <base> [manifest package]  reruns proof.sh while the only failure is a Source boundary refusal (another lane's split pending)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1105b"; N="$1"
tries=0
while [ $((tries+=1)) -le 12 ]; do
 bash "$C/1105b-tools/actinfra/proof.sh" "$@"
 /usr/bin/grep -q "Source extent or boundary exceeded\|Source boundary exceeded\|Source refused before compiler\|could not compile " "$G/actinfra-build-$N.txt" 2>/dev/null || break
 cp "$G/actinfra-build-$N.txt" "$G/actinfra-build-$N.refused.$(date +%H%M).txt"; sleep 600
done
