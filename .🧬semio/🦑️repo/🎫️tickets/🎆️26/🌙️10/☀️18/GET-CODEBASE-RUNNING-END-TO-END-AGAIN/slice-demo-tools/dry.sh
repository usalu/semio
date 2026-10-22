#!/bin/sh
# usage: dry.sh <name> <repo args...>  -> filtered log in 🗑️generated/slice-demo/<name>.log
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo "$@" > "$C/🗑️generated/slice-demo/$n.raw" 2>&1
rc=$?
/usr/bin/grep -v -E "source-admission|parser-source|javascript-source-provider|^\[script\] repository:command|generation-verify|implementation-source|^\[capacity\] waiting|running elapsedMs" "$C/🗑️generated/slice-demo/$n.raw" > "$C/🗑️generated/slice-demo/$n.log"
echo "rc=$rc" >> "$C/🗑️generated/slice-demo/$n.log"
tail -25 "$C/🗑️generated/slice-demo/$n.log" | cut -c1-300
