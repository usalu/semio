#!/usr/bin/env bash
# Runs every mathematical_cas lib test individually under a timeout to find the hang.
set -u
TICKET_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NAMES_FILE="$TICKET_DIR/test-names.txt"
RESULTS_FILE="$TICKET_DIR/isolation-results.txt"
TEST_BIN="$(ls target/debug/deps/mathematical_cas-* 2>/dev/null | grep -v '\.d$' | head -1)"
if [ -z "$TEST_BIN" ]; then
  echo "test binary not found, run cargo test -p mathematical_cas --lib -- --list first" >&2
  exit 1
fi
: > "$RESULTS_FILE"

while IFS= read -r name; do
  [ -z "$name" ] && continue
  start=$(date +%s)
  perl -e 'alarm 30; exec @ARGV' "$TEST_BIN" "$name" --exact >/tmp/cas-test-out.log 2>&1
  code=$?
  end=$(date +%s)
  elapsed=$((end - start))
  if [ "$code" -eq 142 ]; then
    echo "HANG   $name (timed out after 30s)" | tee -a "$RESULTS_FILE"
  elif [ "$code" -ne 0 ]; then
    echo "FAIL   $name (exit $code, ${elapsed}s)" | tee -a "$RESULTS_FILE"
  else
    echo "OK     $name (${elapsed}s)" >> "$RESULTS_FILE"
  fi
done < "$NAMES_FILE"

echo "Done. Summary:"
grep -v '^OK' "$RESULTS_FILE" || echo "(all tests OK individually)"
