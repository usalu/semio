#!/bin/sh
# usage: chain.sh name=project:target ...   (Bazel verdicts, sequential; summary in 🗑️generated/print-d/chain-summary.txt)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for a in "$@"; do
  n=${a%%=*}; t=${a#*=}
  sh "$C/print-d-tools/run.sh" "$n" --tool=bazel run "$t"
  echo "$n $t $(tail -n 1 "$C/🗑️generated/print-d/$n.raw") $(date +%H:%M)" >> "$C/🗑️generated/print-d/chain-summary.txt"
done
