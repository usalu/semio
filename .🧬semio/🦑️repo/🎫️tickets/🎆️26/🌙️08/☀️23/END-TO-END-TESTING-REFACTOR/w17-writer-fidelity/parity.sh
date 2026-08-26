#!/bin/zsh
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
OUT="/private/tmp/claude-501/-Users-ueli-Documents-semio/34f3999f-e145-4d4e-ab13-c3c2aef22ddf/scratchpad/w17"
for c in "$@"; do
  echo "=== CASE $c"
  bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$c" > "$OUT/parity-$c.txt" 2>&1
  echo "=== EXIT $c $?"
  grep -E "^\[test\] level=" "$OUT/parity-$c.txt" || tail -3 "$OUT/parity-$c.txt"
done
echo ALLPARITY
