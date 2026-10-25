#!/bin/sh
# usage: queue.sh <batch>...  waits until the running chain wrote its done line, then runs each batch chain after the other
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/fin-d"
while ! grep -q '^done .*b-dash' "$O/chain.tsv"; do sleep 20; done
for b in "$@"; do sh "$C/fin-d-tools/chain.sh" "$C/fin-d-tools/$b"; done
echo "queue-finished $(date +%H:%M:%S)" >> "$O/chain.tsv"
