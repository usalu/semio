#!/bin/bash
# usage: fw.sh <out> ; runs each framework 2d/pixels/work bun test file in its own process
cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules" || exit 1
export SEMIO_TEST_ARTIFACT_DIR="${SEMIO_TEST_ARTIFACT_DIR:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1102/artifacts}"; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
OUT="$1"; : > "$OUT"
for root in ◻️2d 🔲️pixels 🌱️value/📋️list/🔗️persistent/🎟️work; do
/usr/bin/grep -rlE 'from "(bun:test|vitest)"' --include='*.ts' "$root" 2>/dev/null | sort | while IFS= read -r f; do
  R=$(bun test "./$f" 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
  echo "${R:-NORESULT} :: $f" >> "$OUT"
done; done
echo DONE >> "$OUT"
