#!/bin/sh
# usage: exec-probe.sh <name> <project> [seconds]   a real `repo exec` that loads the (scoped) graph in a child (SEMIO_BUILD_GRAPH_ISOLATE=on), then idles: lets the loader footprint be sampled through the idle phase
cd /Users/ueli/Documents/semio || exit 1
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/graph-c"
mkdir -p "$G"; N="$1"; P="$2"; T="${3:-240}"; S=$(date +%s)
SEMIO_BUILD_GRAPH_ISOLATE="${ISOLATE:-on}" SEMIO_GRAPH_PHASES=1 nice -n 10 bun repo exec --projects="$P" -- sh -c "echo IDLE-START; sleep $T; echo IDLE-END" 2>&1 | perl -MPOSIX -ne 'BEGIN{$|=1} print strftime("%H:%M:%S ",localtime).$_' > "$G/$N.err"
echo "name=$N wall=$(( $(date +%s)-S ))s" > "$G/$N.time"
