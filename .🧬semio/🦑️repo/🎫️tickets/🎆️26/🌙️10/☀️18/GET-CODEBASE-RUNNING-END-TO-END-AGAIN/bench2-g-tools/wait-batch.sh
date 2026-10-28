#!/bin/sh
# wait-batch.sh <n> [seconds]: polls the gate table for the verdict GREEN of batch <n> (row "| <n> |"), at most [seconds] (default 520).
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; left=${2:-520}
while [ "$left" -gt 0 ]; do
  row=$(/usr/bin/grep "^| $n |" "$C/result-gate-g.md" | head -1)
  case "$row" in *GREEN*) echo "BATCH-$n-GREEN: $row" | cut -c1-300; exit 0;; esac
  sleep 30; left=$((left-30))
done
echo "batch $n not green yet: $(/usr/bin/grep -c '^| [0-9]' "$C/result-gate-g.md") rows, last: $(/usr/bin/grep '^| [0-9]' "$C/result-gate-g.md" | tail -1 | cut -c1-200)"; exit 1
