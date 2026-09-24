#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t4/target CARGO_INCREMENTAL=0
run=$1; shift
for o in "$@"; do
  n=${o:t}
  bun "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" parity exhaustive --owner "$o" > ".tmp-ticket/wp-t4/generated/parity-$n-$run.txt" 2>&1
  echo "done $n $(grep -a '^\[test\] level' .tmp-ticket/wp-t4/generated/parity-$n-$run.txt)" >> ".tmp-ticket/wp-t4/generated/parity-owners-$run.log"
done
echo ALLDONE >> ".tmp-ticket/wp-t4/generated/parity-owners-$run.log"
