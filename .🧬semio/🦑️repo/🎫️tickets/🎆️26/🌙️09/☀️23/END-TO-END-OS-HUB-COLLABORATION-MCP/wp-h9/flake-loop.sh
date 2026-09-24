#!/bin/zsh
# Loops one hub lib law on a copied test binary until it fails or N runs pass.
# usage: flake-loop.sh <binary> <runs> <exact-law> <capture-dir>
BIN=$1 RUNS=$2 LAW=$3 OUT=$4
cd "/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust"
export RUST_MIN_STACK=268435456
echo "=== start $(date +%T) runs=$RUNS law=$LAW"
for i in $(seq 1 $RUNS); do
  "$BIN" --exact "$LAW" --test-threads=1 > "$OUT/flake-run.txt" 2>&1
  rc=$?
  if [ $rc -ne 0 ]; then echo "FAIL run=$i rc=$rc"; cp "$OUT/flake-run.txt" "$OUT/flake-fail-$i.txt"; fi
  [ $((i % 25)) -eq 0 ] && echo "progress $i $(date +%T)"
done
echo "=== done $(date +%T)"
