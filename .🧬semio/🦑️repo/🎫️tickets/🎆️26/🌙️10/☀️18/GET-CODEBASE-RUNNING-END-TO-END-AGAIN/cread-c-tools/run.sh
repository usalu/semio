#!/bin/sh
# usage: run.sh <name> <repo args...>   (detached, nice 10, log in generated/cread-c/<name>.log, pid in pids.txt)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/cread-c"
n=$1; shift
export SEMIO_SCRIPT_STALL_MS=14400000
pid=$(python3 "$C/chain-tools/detach.py" "$C/🗑️generated/cread-c/$n.log" nice -n 10 bun repo "$@")
echo "$n $pid $(ps -o lstart= -p $pid) $*" >> "$C/🗑️generated/cread-c/pids.txt"
echo "$pid"
