#!/bin/sh
# 🏃️ Wave 17 — per-case parity for the cases this wave touched, plus the regression neighbours.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w17-parity-out"
mkdir -p "$OUT"
for c in "$@"; do
  echo "=== $c"
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" > "$OUT/$c.txt" 2>&1
  echo "exit=$?"
  grep -E "^\[test\] level=|^\[test\] " "$OUT/$c.txt" | head -20
done
