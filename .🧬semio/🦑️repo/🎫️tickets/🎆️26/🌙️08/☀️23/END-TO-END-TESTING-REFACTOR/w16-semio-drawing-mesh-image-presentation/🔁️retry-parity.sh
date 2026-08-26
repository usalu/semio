#!/bin/bash
# 🔁️ Retries the two remaining wave-16 parity runs until `semio-s-plugin-stdio` compiles again.
# The Rust subject phase is currently blocked by another session's in-progress 📄️pdf/🖼️tiff refactor
# in the working tree, which is not this wave's work. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR.
OUT="/private/tmp/claude-501/-Users-ueli-Documents-semio/34f3999f-e145-4d4e-ab13-c3c2aef22ddf/scratchpad"
TEST="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
cd "$TEST" || exit 1
for attempt in $(seq 1 60); do
  echo "attempt $attempt $(date +%H:%M:%S)" >> "$OUT/w16-retry.log"
  for case in mutate-semio-mesh mutate-semio-drawing; do
    name="${case#mutate-semio-}"
    if grep -q '^\[test\] level' "$OUT/w16-$name-parity.txt" 2>/dev/null; then continue; fi
    bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case "$case" --implementation rust > "$OUT/w16-$name-parity.txt" 2>&1
    echo "EXIT=$?" >> "$OUT/w16-$name-parity.txt"
  done
  if grep -q '^\[test\] level' "$OUT/w16-mesh-parity.txt" 2>/dev/null && grep -q '^\[test\] level' "$OUT/w16-drawing-parity.txt" 2>/dev/null; then
    bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio > "$OUT/w16-stdio-oracle.txt" 2>&1
    echo "EXIT=$?" >> "$OUT/w16-stdio-oracle.txt"
    echo "done $(date +%H:%M:%S)" >> "$OUT/w16-retry.log"
    exit 0
  fi
  sleep 120
done
