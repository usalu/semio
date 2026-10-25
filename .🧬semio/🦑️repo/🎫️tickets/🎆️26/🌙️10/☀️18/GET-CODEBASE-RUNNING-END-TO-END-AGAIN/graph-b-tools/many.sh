#!/bin/sh
# usage: many.sh <name> <projects,comma> [extra args...]   one bazel run-many typecheck, stderr stamped, wall recorded
cd /Users/ueli/Documents/semio || exit 1
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/graph-b"
N="$1"; P="$2"; shift 2; S=$(date +%s)
SEMIO_GRAPH_PHASES=1 nice -n 10 bun repo --tool=bazel run-many --targets=typecheck --projects="$P" "$@" 2>&1 | perl -MPOSIX -ne 'BEGIN{$|=1} print strftime("%H:%M:%S ",localtime).$_' > "$G/$N.log"
echo "name=$N rc=$? wall=$(( $(date +%s)-S ))s" > "$G/$N.time"
