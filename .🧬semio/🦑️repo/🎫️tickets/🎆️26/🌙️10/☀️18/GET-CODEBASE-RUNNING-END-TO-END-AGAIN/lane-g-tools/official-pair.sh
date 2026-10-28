#!/bin/sh
# lane-g: official `repo cargo` pair: two concurrent no-edit checks of two dashboard crates in two agent sessions; the pool must hand out two lanes, then the same two warm lanes again.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/lane-g"; T="$C/rust-g-tools"
unset SEMIO_SCRIPT_STALL_MS
sh "$T/healthy.sh" || exit 2
A=semio-framework-repo-dashboard-labels; B=semio-framework-repo-dashboard-preferences
for round in 1 2; do
  python3 "$T/measure.py" "lane-pair$round" "$G/runs.jsonl" -- sh -c "CLAUDE_CODE_SESSION_ID=lane-g-1 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p $A --lib >> $G/off-a.log 2>&1 & CLAUDE_CODE_SESSION_ID=lane-g-2 nice -n 10 bun repo cargo --store=shared --configuration=prototype -- check -p $B --lib >> $G/off-b.log 2>&1 & wait" >> "$G/off.out" 2>&1
done
echo done
