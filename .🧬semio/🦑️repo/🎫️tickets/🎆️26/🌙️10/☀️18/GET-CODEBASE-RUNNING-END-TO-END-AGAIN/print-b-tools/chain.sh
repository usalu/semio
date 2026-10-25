#!/bin/sh
# usage: chain.sh <name:target>... ; runs each sequentially, summary in C/🗑️generated/print-b/chain-summary.txt
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for spec in "$@"; do
  n=${spec%%=*}; t=${spec#*=}
  sh "$C/print-b-tools/run.sh" "$n" --tool=bazel run "$t"
  echo "$n $t $(tail -1 "$C/🗑️generated/print-b/$n.raw") $(date +%H:%M)" >> "$C/🗑️generated/print-b/chain-summary.txt"
done
echo done >> "$C/🗑️generated/print-b/chain-summary.txt"
