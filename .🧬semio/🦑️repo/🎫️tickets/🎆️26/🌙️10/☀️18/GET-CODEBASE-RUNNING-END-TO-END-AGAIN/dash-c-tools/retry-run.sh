#!/bin/sh
# usage: retry-run.sh <name> <tries> <repo args...>  Reruns `bun repo <args>` while it dies of a stale-source/identity race (live peer edits). Writes <name>.done with the verdict.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/dash-c"; export SEMIO_SCRIPT_STALL_MS=14400000
n=$1; t=$2; shift 2; i=0
while [ $i -lt $t ]; do
  i=$((i+1)); log="$G/$n-$i.log"
  nice -n 10 bun repo "$@" > "$log" 2>&1; rc=$?
  [ $rc -eq 0 ] && { echo "ok on try $i" > "$G/$n.done"; exit 0; }
  /usr/bin/grep -q "Stale loaded Source revision\|roster changed\|Source descriptor or path changed\|restart required" "$log" || { echo "failed on try $i rc=$rc (not a race)" > "$G/$n.done"; exit $rc; }
  sleep 20
done
echo "gave up after $t tries" > "$G/$n.done"; exit 1
