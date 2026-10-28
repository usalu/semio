#!/bin/sh
# Second window chain (11:15): course and print first (dashboard was withheld by a parse error of a peer's file in generation 544), dashboard last.
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/bench-g/window"
M="env -u SEMIO_SCRIPT_STALL_MS python3 $C/bench-g-tools/measure.py"
R=/Users/ueli/Documents/semio
export BENCH_WINDOW=1
export BENCH_WINDOW_UNTIL=1130
cd $R || exit 1
for p in course print; do $M noop --product $p --root $R --tool bazel --out "$O" --prime --samples 3 --retries 3; done
$M edit --product course --root $R --tool bazel --out "$O" --samples 2 --retries 3
$M peer --product course --peer scheduling --root $R --tool bazel --out "$O" --samples 2 --retries 3
$M noop --product dashboard --root $R --tool bazel --out "$O" --prime --samples 3 --retries 3
$M edit --product print --root $R --tool bazel --out "$O" --samples 2 --retries 3
$M concurrent --root $R --tool bazel --out "$O"
echo WINDOW2-DONE
