#!/bin/zsh
# usage: run-e2e-repeat.sh <label> <count>: runs the scenario in STEPS count times through run-e2e-retry.sh, keeping each report as e2e-report-<label>-<n>.json.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
for n in $(seq 1 $2); do
  rm -f $T/🗑️generated/run-e2e-attempts.txt $T/🗑️generated/e2e-report.json
  $T/run-e2e-retry.sh
  cp $T/🗑️generated/e2e-report.json $T/🗑️generated/e2e-report-$1-$n.json 2>/dev/null
  echo "$1-$n $(tail -1 $T/🗑️generated/run-e2e-attempts.txt)" >> $T/🗑️generated/run-e2e-repeat.txt
done
