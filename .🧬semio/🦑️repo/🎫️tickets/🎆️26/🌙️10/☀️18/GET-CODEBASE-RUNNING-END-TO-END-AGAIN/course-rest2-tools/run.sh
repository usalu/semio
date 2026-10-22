#!/bin/sh
# usage: run.sh <log-name> <repo args...>   (runs detached-friendly, nice, logs under 🗑️generated/course-rest2)
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
export SEMIO_BUILD_GRAPH_POLICY=${SEMIO_BUILD_GRAPH_POLICY:-current}
nice -n 10 bun repo "$@" > "$C/🗑️generated/course-rest2/$n.log" 2>&1
echo "rc=$?" >> "$C/🗑️generated/course-rest2/$n.log"
