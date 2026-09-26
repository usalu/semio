#!/bin/zsh
# 🧪️ T12 session 12: test-parity (Rust subject + Python oracle) of the semio cases whose adapters moved to the hosts'
# `step_fixture_uris`. Captures `.🧬semio/🌐hub/s12-t12-captures/parity-<case>-2.txt`; progress `parity-progress.txt`.
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-t12-captures
export SEMIO_TEST_LEVEL=exhaustive NX_DAEMON=false PYTHONDONTWRITEBYTECODE=1 CARGO_INCREMENTAL=0
for c in "$@"; do
  start=$(date +%s)
  nice -n 15 bun nx run @semio-tech/repo-test-domain:test-parity --outputStyle=stream -- --case "$c" > "$OUT/parity-$c-2.txt" 2>&1
  code=$?
  echo "$c EXIT=$code $(( $(date +%s) - start ))s $(sed 's/\x1b\[[0-9;]*m//g' "$OUT/parity-$c-2.txt" | /usr/bin/grep -o 'executed=[0-9]* passed=[0-9]* failed=[0-9]* errored=[0-9]* parity=[0-9/]*')" >> "$OUT/parity-progress.txt"
done
echo "ALL_DONE $(date '+%T')" >> "$OUT/parity-progress.txt"
