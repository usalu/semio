#!/bin/sh
# usage: run.sh <name> <repo args...>   (detached, nice 10, log in generated/dash-b/<name>.log, pid in pids.txt)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
pid=$(python3 "$C/chain-tools/detach.py" "$C/🗑️generated/dash-b/$n.log" nice -n 10 bun repo "$@")
echo "$n $pid $*" >> "$C/🗑️generated/dash-b/pids.txt"
echo "$pid"
