#!/bin/sh
# usage: run.sh <name> <repo args...> ; log in C/🗑️generated/demo-c/<name>.raw, rc line appended
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
SEMIO_SCRIPT_STALL_MS=14400000 SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo "$@" > "$C/🗑️generated/demo-c/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/demo-c/$n.raw"
