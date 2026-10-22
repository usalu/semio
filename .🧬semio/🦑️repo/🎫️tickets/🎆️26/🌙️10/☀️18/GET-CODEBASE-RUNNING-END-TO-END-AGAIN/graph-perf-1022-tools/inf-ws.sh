#!/bin/sh
# usage: inf-ws.sh <workspace> <files.json> <N> <tag> [VAR=value...]
WS=$1; FILES=$2; N=$3; TAG=$4; shift 4
HERE=$(cd "$(dirname "$0")" && pwd); H=$(cd "$HERE/../graph-parallel-tools/harness" && pwd)
OUT="/private/tmp/gp1022/$TAG"; mkdir -p "$OUT"
env BUN_JSC_forceRAMSize=${GP_RAM:-1073741824} SEMIO_HOST_CAPACITY=off GP_TOLERANT=1 SEMIO_GRAPH_WORKERS=0 N="$N" GP_WORKSPACE="$WS" GP_SCRATCH="$OUT/scratch" OUT="$OUT/o" GP_FILES="$FILES" "$@" nice -n 10 bun "$H/nodes-run.mjs"
