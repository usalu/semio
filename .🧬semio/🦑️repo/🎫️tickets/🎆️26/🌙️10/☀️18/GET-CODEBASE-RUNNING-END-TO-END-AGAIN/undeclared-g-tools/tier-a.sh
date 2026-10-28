#!/bin/bash
# tier-a.sh: the tier A targets, one at a time (never two oracle runs in parallel), summary lines appended to reports/tier-a.log
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; T="$C/undeclared-g-tools"; G="$C/🗑️generated/undeclared-g"
for spec in "$@"; do
  tag=${spec%%|*}; rest=${spec#*|}; id=${rest%%|*}; list=${rest#*|}
  date +%H:%M:%S >> "$G/reports/tier-a.log"
  bash "$T/run-a.sh" "$tag" "$id" "$list" >> "$G/reports/tier-a.log" 2>> "$G/reports/$tag.err"
done
echo done >> "$G/reports/tier-a.log"
