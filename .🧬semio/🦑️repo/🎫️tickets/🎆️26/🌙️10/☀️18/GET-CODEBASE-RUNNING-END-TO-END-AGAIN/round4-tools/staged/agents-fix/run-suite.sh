#!/bin/bash
# usage: run-suite.sh <overlay|live> <repo-relative test dir> <test file basename ...>   one bun process, niced, from inside the test directory (never scans the repo)
set -u
A="$(cd "$(dirname "$0")" && pwd)"
MODE="$1"; DIR="$2"; shift 2
cd "/Users/ueli/Documents/semio/$DIR" || exit 3
PRE=()
[ "$MODE" = overlay ] && PRE=(--preload "$A/overlay-preload.ts")
for f in "$@"; do
  echo "## $MODE $DIR/$f"
  nice -n 10 bun test -c /Users/ueli/Documents/semio/bunfig.toml ${PRE[@]+"${PRE[@]}"} "./$f" 2>&1 | command grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)|^error:|SyntaxError|Cannot find|Ran [0-9]+' | head -12
done
