#!/bin/sh
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w17-parity-out2"
mkdir -p "$OUT"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
for c in "$@"; do
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" > "$OUT/$c.txt" 2>&1
  code=$?
  line=$(grep -m1 '^\[test\] level=' "$OUT/$c.txt")
  echo "RESULT $c exit=$code ${line:-NO-SUMMARY}"
done
echo "ROUND2-DONE"
