#!/bin/bash
# usage: bun-tests.sh <phase> ; bun test per file, per-file log
cd /Users/ueli/Documents/semio
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
P="$1"
out="$G/taf-bun-$P.log"
: > "$out"
while IFS= read -r f; do
  echo "=== $f" >> "$out"
  SEMIO_TEST_ARTIFACT_DIR="$G/taf-artifacts" perl -e 'alarm 900; exec @ARGV' -- bun test "./$f" 2>&1 | sed -E 's/\[[0-9.]+ ?m?s\]//g; s/ \[[0-9.]+ms\]//g' >> "$out"
  echo "--- exit ${PIPESTATUS[0]}" >> "$out"
done < "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/taf-tools/bun-tests.txt"
echo DONE >> "$out"
