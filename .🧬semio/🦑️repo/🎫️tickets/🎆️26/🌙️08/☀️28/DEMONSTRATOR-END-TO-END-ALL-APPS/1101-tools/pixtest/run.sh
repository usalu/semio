#!/bin/bash
# usage: run.sh <tag> <seconds> <binary> [test args...]  -- bounded: kills only its own recorded pid
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; S=$2; B=$3; shift 3
O="$C/🗑️generated/1101/pixtest-run-$N"; rm -f "$O.rc"
cd 🧰️framework/🔨️modules/🔲️pixels/🧪️tests/🥣️companion/📦️packages/🦀️rust
"$B" "$@" > "/Users/ueli/Documents/semio/$O.txt" 2>&1 &
P=$!; echo $P > "/Users/ueli/Documents/semio/$O.pid"
for i in $(seq 1 $S); do kill -0 $P 2>/dev/null || break; sleep 1; done
if kill -0 $P 2>/dev/null; then kill -9 $P; wait $P 2>/dev/null; echo "TIMEOUT" > "/Users/ueli/Documents/semio/$O.rc"; else wait $P; echo $? > "/Users/ueli/Documents/semio/$O.rc"; fi
