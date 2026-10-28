#!/bin/sh
# lane2-g: interleaved before/after series, two series each, 3 rounds of 2500-fn crates; pid in $A/🗑️generated/lane2-g/measure.pid
A="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd "$A/lane2-g-tools" || exit 1
for series in 1 2; do
  for mode in custody lane; do
    bun measure-consumers.ts $mode 3 "$A/🗑️generated/lane2-g/consumers.jsonl" 2500 >> "$A/🗑️generated/lane2-g/measure.log" 2>&1
  done
done
echo done >> "$A/🗑️generated/lane2-g/measure.log"
