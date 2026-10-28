#!/bin/sh
# usage: census-loop.sh <list> <outdir> <wall> — resumable census; restarts after peer churn (a half-edited pin or toolchain file) until the summary says DONE
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=0
while [ $n -lt 40 ]; do
  nice -n 10 bun "$C/route-g-tools/census-run.ts" "$1" "$2" "$3"
  /usr/bin/grep -q '^DONE' "$2/summary.txt" 2>/dev/null && break
  n=$((n+1)); sleep 30
done
