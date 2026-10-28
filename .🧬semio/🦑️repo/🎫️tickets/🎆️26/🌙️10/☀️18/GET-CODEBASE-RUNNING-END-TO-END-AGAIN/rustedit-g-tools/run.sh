#!/bin/sh
# rustedit-g: ONE measured official print-rs:build run. usage: run.sh <label>
# record: $O/<label>.json (phases, tree RSS, load, swap), log $O/<label>.log, pre/post state in $O/<label>.state
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/rustedit-g"
L=$1
R=/Users/ueli/Documents/semio
mkdir -p "$O"
cd "$R" || exit 1
{ date +%s000; uptime; sysctl -n vm.swapusage; bun repo leases status 2>&1 | /usr/bin/grep -c WAITING; } > "$O/$L.state.pre" 2>&1
echo "$(date +%s)000" > "$O/$L.start"
RUSTEDIT_TIMING=1 env -u SEMIO_SCRIPT_STALL_MS python3 -I "$C/rustedit-g-tools/measure.py" run --root "$R" --id "$L" --out "$O" -- nice -n 10 bun repo --tool=bazel run @semio-tech/print-rs:build > "$O/$L.measure.out" 2>&1
echo "$(date +%s)000" > "$O/$L.end"
{ date +%s000; uptime; sysctl -n vm.swapusage; } > "$O/$L.state.post" 2>&1
