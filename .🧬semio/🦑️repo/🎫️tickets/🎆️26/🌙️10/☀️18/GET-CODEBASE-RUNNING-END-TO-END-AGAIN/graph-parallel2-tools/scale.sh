#!/bin/sh
# usage: scale.sh <N> <tag> <workers...>   runs the harness once per worker count, one after the other (cold scratch cache each), prints one SUMMARY line per run
HERE=$(cd "$(dirname "$0")" && pwd)
N=$1; TAG=$2; shift 2
for W in "$@"; do
  echo "RUN workers=$W load=$(uptime | sed 's/.*averages: //')"
  sh "$HERE/inf.sh" "$N" "$TAG-$W" SEMIO_GRAPH_WORKERS="$W" SEMIO_GRAPH_PHASES=1 2>&1 | grep -E "^\[graph-farm\]|SUMMARY"
done
