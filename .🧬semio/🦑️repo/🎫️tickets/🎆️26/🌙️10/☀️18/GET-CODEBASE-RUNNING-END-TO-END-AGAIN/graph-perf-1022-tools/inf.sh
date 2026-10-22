#!/bin/sh
# usage: inf.sh <N> <tag> [VAR=value...]   real emojiProjectJsonNodes over N live projects (50% cargo-only), scratch storage; prints SUMMARY
N=$1; TAG=$2; shift 2
HERE=$(cd "$(dirname "$0")" && pwd); H=$(cd "$HERE/../graph-parallel-tools/harness" && pwd)
OUT="/private/tmp/gp1022/$TAG"; mkdir -p "$OUT"
env BUN_JSC_forceRAMSize=${GP_RAM:-1073741824} SEMIO_HOST_CAPACITY=off GP_TOLERANT=1 SEMIO_GRAPH_WORKERS=0 N="$N" GP_WORKSPACE=/Users/ueli/Documents/semio GP_SCRATCH="$OUT/scratch" OUT="$OUT/o" GP_FILES=/private/tmp/graph-perf-1022-files.json "$@" nice -n 10 bun $BUN_ARGS "${HARNESS:-$H/nodes-run.mjs}"
