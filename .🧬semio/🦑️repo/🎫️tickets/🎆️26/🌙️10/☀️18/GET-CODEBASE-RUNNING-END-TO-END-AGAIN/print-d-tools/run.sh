#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
echo "pid=$$ $(date)" > "$C/🗑️generated/print-d/$n.pid"
SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo "$@" > "$C/🗑️generated/print-d/$n.raw" 2>&1
echo "rc=$?" >> "$C/🗑️generated/print-d/$n.raw"
