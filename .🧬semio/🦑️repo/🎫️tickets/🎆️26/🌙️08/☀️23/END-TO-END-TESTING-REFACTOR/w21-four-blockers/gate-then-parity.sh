#!/usr/bin/env bash
# ⏳️ Waits until a stdio SUBJECT host actually links (os-kernel is being repaired by a peer session
# right now), then runs the per-case parity sweep for the four w21 blockers.
GATE="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tests/hosts/test-s-plugins-stdio-artifacts-jpg-c9977f-create-and-read-jpeg-subject-rust"
TD="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/agents/local/cargo-test-hosts"
OUT="$1"; shift
cd "$GATE" || exit 1
for i in $(seq 1 60); do
  CARGO_TARGET_DIR="$TD" cargo check --quiet --features sut --bin host > /tmp/w21-gate.txt 2>&1
  rc=$?
  echo "[gate] attempt $i rc=$rc errors=$(grep -c '^error' /tmp/w21-gate.txt) $(date +%T)" >> "$OUT"
  if [ $rc -eq 0 ]; then echo "GATE-GREEN $(date +%T)" >> "$OUT"; break; fi
  sleep 180
done
if [ "$rc" -ne 0 ]; then echo "GATE-STILL-RED $(date +%T)" >> "$OUT"; exit 1; fi
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
for c in "$@"; do
  echo "===== $c  start $(date +%T)" >> "$OUT"
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" >> "$OUT" 2>&1
  echo "===== $c  exit=$? end $(date +%T)" >> "$OUT"
done
echo "ALLDONE $(date +%T)" >> "$OUT"
