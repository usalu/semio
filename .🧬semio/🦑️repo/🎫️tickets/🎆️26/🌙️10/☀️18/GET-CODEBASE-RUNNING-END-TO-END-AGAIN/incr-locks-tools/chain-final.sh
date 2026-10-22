#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/incr-locks-run"
for step in "prepare" "plan --scenario=all"; do
  name=$(echo "$step" | cut -d' ' -f1)
  nice -n 10 bun repo incremental $step > "$G/final-incremental-$name.log" 2>&1
  echo "incremental $step exit $?" >> "$G/final-chain.status"
done
nice -n 10 bun repo concurrency plan > "$G/final-concurrency-plan.log" 2>&1
echo "concurrency plan exit $?" >> "$G/final-chain.status"
echo done >> "$G/final-chain.status"
