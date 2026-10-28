#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"
for spec in "$@"; do
  tag=${spec%%|*}; rest=${spec#*|}; id=${rest%%|*}; list=${rest#*|}
  date +%H:%M:%S >> "$G/reports/queue-d.log"
  bash "$T/run-a.sh" "$tag" "$id" "$list" --rounds 300 >> "$G/reports/queue-d.log" 2>> "$G/reports/$tag.err"
done
echo done >> "$G/reports/queue-d.log"
