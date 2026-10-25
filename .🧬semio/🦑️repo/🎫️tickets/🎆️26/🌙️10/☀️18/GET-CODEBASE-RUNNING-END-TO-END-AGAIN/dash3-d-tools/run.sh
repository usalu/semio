#!/bin/sh
# usage: run.sh <name> <repo args...>   (detached, nice 10, log in generated/dash3-d/<name>.log, pid in pids.txt)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
mkdir -p "$C/🗑️generated/dash3-d"
n=$1; shift
pid=$(env -u SEMIO_SCRIPT_STALL_MS python3 "$C/chain-tools/detach.py" "$C/🗑️generated/dash3-d/$n.log" nice -n 10 bun repo "$@")
echo "$n $pid $(date +%H:%M:%S) $*" >> "$C/🗑️generated/dash3-d/pids.txt"
echo "$pid"
