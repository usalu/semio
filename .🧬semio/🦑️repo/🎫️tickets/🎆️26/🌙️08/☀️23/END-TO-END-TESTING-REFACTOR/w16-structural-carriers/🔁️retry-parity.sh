#!/bin/sh
# 🔁️ Retries `parity exhaustive` for one case until the SHARED `semio-s-plugin-stdio-test-oracle`
# crate compiles again. A concurrent session is mid-refactor in `🧪️oracle/📄️document/🦀️component.rs`
# (`font_program` changed from a 3-tuple to a 4-tuple, call sites not yet all updated), which blocks
# every Rust subject host in the plugin — nothing to do with this case. Poll, do not chase.
# Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.
CASE="$1"
OUT="$2"
ATTEMPTS="${3:-12}"
cd "$(dirname "$0")/../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 2
i=1
while [ "$i" -le "$ATTEMPTS" ]; do
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$CASE" --implementation rust >"$OUT" 2>&1
  code=$?
  if [ "$code" -eq 0 ]; then
    echo "attempt $i EXIT=0" >>"$OUT"
    exit 0
  fi
  if ! grep -q "could not compile\|no result stream\|without emitting results\|ETIMEDOUT\|lock contention" "$OUT"; then
    echo "attempt $i EXIT=$code (not a peer build failure — stopping)" >>"$OUT"
    exit "$code"
  fi
  echo "[retry] attempt $i blocked by a peer build failure, waiting" >&2
  sleep 120
  i=$((i + 1))
done
echo "attempt $i EXIT=$code (gave up after $ATTEMPTS attempts)" >>"$OUT"
exit 1
