#!/bin/bash
# W4 consolidated plugin sweep: cargo check -p <crate> --lib, max 3 concurrent.
TICKET="$1"
SWEEP="$TICKET/w4-sweep"
LIST="$2"
run_one() {
  c="$1"; t="$3"
  start=$(date +%s)
  CARGO_TARGET_DIR="$t" cargo check -p "$c" --lib > "$SWEEP/$c.txt" 2>&1
  rc=$?
  end=$(date +%s)
  echo "$c|$rc|$((end-start))" >> "$SWEEP/results.psv"
  echo "done $c rc=$rc $((end-start))s"
}
export -f run_one
export SWEEP
: > "$SWEEP/results.psv"
i=0
while read -r c; do
  [ -z "$c" ] && continue
  slot=$((i % 3))
  t="$TICKET/🎯️target-sweep$slot"
  run_one "$c" "" "$t" &
  i=$((i+1))
  while [ "$(jobs -r | wc -l)" -ge 3 ]; do wait -n; done
done < "$LIST"
wait
echo "SWEEP COMPLETE"
