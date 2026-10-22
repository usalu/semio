#!/bin/sh
# usage: inf.sh <N> <tag> [VAR=value...]   real emojiProjectJsonNodes over N live projects in a scratch cache (graph-parallel2 copy); HARNESS overrides the driver
N=$1; TAG=$2; shift 2
HERE=$(cd "$(dirname "$0")" && pwd)
OUT="/private/tmp/gpp2/$TAG"; mkdir -p "$OUT"
cd /Users/ueli/Documents/semio || exit 1
env BUN_JSC_forceRAMSize=${GP_RAM:-1073741824} SEMIO_HOST_CAPACITY=off GP_TOLERANT=1 SEMIO_GRAPH_WORKERS=${SEMIO_GRAPH_WORKERS:-0} N="$N" GP_WORKSPACE=/Users/ueli/Documents/semio GP_SCRATCH="${GP_SCRATCH:-$OUT/scratch}" OUT="$OUT/o" GP_FILES=/private/tmp/gr1022/files.json "$@" nice -n 10 bun ${BUN_ARGS:-} "${HARNESS:-$HERE/nodes.mjs}"
