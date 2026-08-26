#!/bin/zsh
set -u
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w20-semio-tabular-verify"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" || exit 1
for attempt in 3 4 5 6 7 8 9 10; do
  for c in mutate-semio-model mutate-semio-text; do
    [ -f "$OUT/green-$c.txt" ] && continue
    bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case $c --implementation rust > "$OUT/attempt$attempt-$c.txt" 2>&1
    code=$?
    echo "attempt=$attempt case=$c exit=$code $(grep -E '^\[test\] level=' "$OUT/attempt$attempt-$c.txt" | head -1)"
    grep -m2 -E 'could not compile|No space left' "$OUT/attempt$attempt-$c.txt"
    if [ $code -eq 0 ] && ! grep -q "exited 101" "$OUT/attempt$attempt-$c.txt"; then cp "$OUT/attempt$attempt-$c.txt" "$OUT/green-$c.txt"; fi
  done
  [ -f "$OUT/green-mutate-semio-model.txt" ] && [ -f "$OUT/green-mutate-semio-text.txt" ] && { echo "ALL GREEN"; exit 0; }
  sleep 180
done
echo "GAVE UP"
