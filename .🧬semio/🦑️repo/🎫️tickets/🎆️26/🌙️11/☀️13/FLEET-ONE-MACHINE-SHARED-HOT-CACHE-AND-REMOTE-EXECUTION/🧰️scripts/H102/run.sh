#!/bin/sh
# H102: oracle -> check -> simulation. Usage: run.sh <graph.log> [scratch-dir]
set -e
HERE=$(cd "$(dirname "$0")" && pwd); OUT=${2:-${TMPDIR:-/tmp}/H102}; mkdir -p "$OUT"
nice -n 10 python3 "$HERE/oracle.py" "$OUT/worlds.json" 102 300
nice -n 10 node "$HERE/check.mjs" "$OUT/worlds.json"
nice -n 10 python3 "$HERE/simulate.py" "$1" 100 24
