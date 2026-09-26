#!/bin/zsh
# ⚖️ H10: runs named semio-hub lib laws from the H10 private target at nice 15.
# usage: laws.sh <capture> <filter…>
OUT=$1; shift
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h10/target
echo "=== start $(date +%T) filters: $*" > "$OUT"
for filter in "$@"; do
  nice -n 15 cargo test -p semio-hub --lib --no-fail-fast -- "$filter" --test-threads 4 >> "$OUT" 2>&1
  echo "=== filter $filter exit $?" >> "$OUT"
done
echo "=== done $(date +%T)" >> "$OUT"
