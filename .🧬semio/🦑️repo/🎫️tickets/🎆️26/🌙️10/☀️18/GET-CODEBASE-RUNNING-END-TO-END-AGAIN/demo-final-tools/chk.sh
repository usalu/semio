#!/bin/sh
# usage: chk.sh <name> <cargo args...> -> log C/🗑️generated/demo-final/<name>.raw, rc line appended
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$C/🗑️generated/demo-final/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/demo-final/$n.raw"
