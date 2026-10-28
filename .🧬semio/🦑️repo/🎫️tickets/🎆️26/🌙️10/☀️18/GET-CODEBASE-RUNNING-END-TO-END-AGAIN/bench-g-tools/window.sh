#!/bin/sh
# Measurement window 10:30-11:30 (coordinator addendum 10:10): live tree, one run at a time, edits restored byte for byte.
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/bench-g/window"
M="env -u SEMIO_SCRIPT_STALL_MS python3 $C/bench-g-tools/measure.py"
R=/Users/ueli/Documents/semio
export BENCH_WINDOW=1
mkdir -p "$O"
cd $R || exit 1
while [ "$(date +%H%M)" -lt 1030 ]; do sleep 15; done
n=0
until bun repo --help >/dev/null 2>&1; do n=$((n+1)); [ $n -gt 30 ] && echo "CLI unhealthy" && exit 3; sleep 60; done
for p in dashboard course print; do $M noop --product $p --root $R --tool bazel --out "$O" --prime --samples 3; done
for p in course print dashboard; do $M edit --product $p --root $R --tool bazel --out "$O" --samples 2; done
$M peer --product course --peer scheduling --root $R --tool bazel --out "$O" --samples 2
$M peer --product course --peer unrelated-ts --root $R --tool bazel --out "$O" --samples 2
$M concurrent --root $R --tool bazel --out "$O"
echo WINDOW-DONE
