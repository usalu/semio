#!/bin/sh
# usage: retry.sh <name> <max> run <target>  -> retries while the failure is a repo CLI loader/graph/lease fault
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; max=$2; shift 2
out="$C/🗑️generated/demo-libs"
i=1
while [ $i -le $max ]; do
  "$C/demo-libs-tools/run.sh" "$n-$i" "$@"
  rc=$(cat "$out/$n-$i.rc")
  echo "attempt $i rc=$rc $(date +%T)" >> "$out/$n.attempts"
  [ "$rc" = 0 ] && break
  if ! /usr/bin/grep -q -E "Test Files|RUN  v|vitest|Tests  |passed|assert|AssertionError" "$out/$n-$i.log"; then
    /usr/bin/grep -m1 -E "^error" "$out/$n-$i.log" | cut -c1-300 >> "$out/$n.attempts"
    sleep 180; i=$((i+1)); continue
  fi
  break
done
echo "final rc=$rc attempt=$i" >> "$out/$n.attempts"
