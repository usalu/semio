#!/bin/sh
# usage: run.sh <name> <repo args...> ; logs in C/🗑️generated/demo-verify/<name>.{log,rc}
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
out="$C/🗑️generated/demo-verify"
rm -f "$out/$n.rc"
date +%T > "$out/$n.start"
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo "$@" > "$out/$n.log" 2>&1
echo $? > "$out/$n.rc"
date +%T > "$out/$n.end"
