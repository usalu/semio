#!/bin/sh
# usage: repo-graph-3-run.sh NAME [SECONDS]   runs `repo show projects` from the workspace root, logs under 🗑️generated/repo-graph-3, stops it with SIGINT after SECONDS when given
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/repo-graph-3"
N="$1"; S="${2:-0}"
date +%T > "$G/$N.start"
/usr/bin/time -l repo show projects > "$G/$N.out" 2> "$G/$N.log" &
PID=$!
if [ "$S" -gt 0 ]; then ( sleep "$S"; for C in $(pgrep -P $PID); do kill -INT $C 2>/dev/null; done ) & fi
wait $PID; echo "exit $?" >> "$G/$N.log"; date +%T > "$G/$N.end"
