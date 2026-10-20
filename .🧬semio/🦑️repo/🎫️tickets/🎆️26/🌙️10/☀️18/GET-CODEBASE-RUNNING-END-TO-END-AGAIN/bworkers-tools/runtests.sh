#!/bin/bash
# usage: runtests.sh <root> <relative test file>...   runs each file alone and prints its pass/fail line and failing test names
root=$1; shift
cd "$root" || exit 2
for f in "$@"; do
  echo "=== ${f##*/library/}"
  BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 NODE_PATH=/Users/ueli/Documents/semio/node_modules perl -e 'alarm 280; exec @ARGV' -- bun test "./$f" 2>&1 | command grep -E '^\(fail\)|^ *[0-9]+ (pass|fail)$|^error: (Cannot|Unable)' | head -12 | cut -c1-220
done
