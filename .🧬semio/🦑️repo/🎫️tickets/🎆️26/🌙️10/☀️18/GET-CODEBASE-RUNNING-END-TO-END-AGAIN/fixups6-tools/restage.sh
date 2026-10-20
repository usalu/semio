#!/bin/sh
set -e
T=$(cd "$(dirname "$0")" && pwd)
if [ -n "${1:-}" ]; then SPLIT_TREE=$(cd "$1" && pwd); export SPLIT_TREE; fi
bun "$T/split.ts"
bun "$T/oracle.ts"
