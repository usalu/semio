#!/bin/zsh
# 🔁️ Retries parity for the two cases whose Rust subject phase is blocked by a peer session's
# in-flight os-kernel refactor, until the workspace compiles again. Ticket 26/08/23.
set -u
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w20-semio-tabular-verify"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12; do
  for c in mutate-semio-model mutate-semio-text; do
    if [ -f "$OUT/green-$c.txt" ]; then continue; fi
    bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case $c --implementation rust > "$OUT/attempt$attempt-$c.txt" 2>&1
    code=$?
    line=$(grep -E "^\[test\] level=" "$OUT/attempt$attempt-$c.txt" | head -1)
    echo "attempt=$attempt case=$c exit=$code $line"
    if [ $code -eq 0 ] && ! grep -q "exited 101" "$OUT/attempt$attempt-$c.txt"; then
      cp "$OUT/attempt$attempt-$c.txt" "$OUT/green-$c.txt"
    fi
  done
  if [ -f "$OUT/green-mutate-semio-model.txt" ] && [ -f "$OUT/green-mutate-semio-text.txt" ]; then echo "ALL GREEN"; exit 0; fi
  sleep 120
done
echo "GAVE UP"
