#!/bin/sh
# usage: run.sh <name> <repo args...>   (detached, nice 10, log in generated/dash-c/<name>.log, pid in pids.txt)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/dash-c"
n=$1; shift
export SEMIO_SCRIPT_STALL_MS=14400000
pid=$(python3 "$C/chain-tools/detach.py" "$C/🗑️generated/dash-c/$n.log" nice -n 10 bun repo "$@")
echo "$n $pid $(date +%H:%M:%S) $*" >> "$C/🗑️generated/dash-c/pids.txt"
echo "$pid"
