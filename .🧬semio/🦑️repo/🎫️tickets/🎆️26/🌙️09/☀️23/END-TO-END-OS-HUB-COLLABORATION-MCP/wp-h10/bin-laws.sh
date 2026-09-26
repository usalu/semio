#!/bin/zsh
# ⚖️ H10: runs the semio-hub `os-hub` bin laws (integration-fixtures: real GIS release guest) from the H10 target at nice 15.
# usage: bin-laws.sh <capture> [filter]
OUT=$1; FILTER=${2:-}
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h10/target
echo "=== start $(date +%T) filter=$FILTER" > "$OUT"
nice -n 15 cargo test -p semio-hub --bin os-hub --features integration-fixtures --no-fail-fast -- $FILTER --test-threads 4 >> "$OUT" 2>&1
echo "=== exit $? $(date +%T)" >> "$OUT"
