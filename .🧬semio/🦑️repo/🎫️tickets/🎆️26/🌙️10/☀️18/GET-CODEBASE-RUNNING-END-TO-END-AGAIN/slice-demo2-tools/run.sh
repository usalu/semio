#!/bin/sh
# usage: run.sh <name> <repo args...> -> raw log in 🗑️generated/slice-demo2/<name>.raw, rc line appended
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo "$@" > "$C/🗑️generated/slice-demo2/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/slice-demo2/$n.raw"
