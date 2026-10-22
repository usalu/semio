#!/bin/sh
# usage: inf-mem.sh <N> <tag> [VAR=value...]   same as graph-parallel2-tools/inf.sh with the heap census driver (nodes-mem.mjs)
HERE=$(cd "$(dirname "$0")" && pwd)
N=$1; TAG=$2; shift 2
HARNESS="$HERE/nodes-mem.mjs" sh "$HERE/../graph-parallel2-tools/inf.sh" "$N" "$TAG" "$@"
