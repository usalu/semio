#!/bin/sh
# usage: show.sh <name> <graph-command...>   one graph reader (default `show projects --json`), stderr stamped per line, wall time recorded
cd /Users/ueli/Documents/semio || exit 1
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/graph-c"
N="$1"; shift; [ $# -eq 0 ] && set -- show projects --json
S=$(date +%s)
SEMIO_GRAPH_PHASES=1 nice -n 10 bun repo "$@" 2>&1 >"$G/$N.out" | perl -MPOSIX -ne 'BEGIN{$|=1} print strftime("%H:%M:%S ",localtime).$_' > "$G/$N.err"
E=$(date +%s)
echo "name=$N start=$S end=$E wall=$((E-S))s" > "$G/$N.time"
