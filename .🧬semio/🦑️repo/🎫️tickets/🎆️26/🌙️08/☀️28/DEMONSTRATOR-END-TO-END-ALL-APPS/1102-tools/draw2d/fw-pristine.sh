#!/bin/bash
# usage: fw.sh <out> ; runs each framework 2d/pixels/work bun test file in its own process
cd "/private/tmp/claude-501/s/pristine/🧰️framework/🔨️modules" || exit 1
OUT="$1"; : > "$OUT"
for root in ◻️2d 🔲️pixels 🌱️value/📋️list/🔗️persistent/🎟️work; do
/usr/bin/grep -rl 'from "bun:test"' --include='*.ts' "$root" 2>/dev/null | sort | while IFS= read -r f; do
  R=$(bun test "./$f" 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
  echo "${R:-NORESULT} :: $f" >> "$OUT"
done; done
echo DONE >> "$OUT"
