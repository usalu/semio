#!/bin/bash
# usage: suite.sh <out> ; runs each draw TS test file in its own bun process
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw" || exit 1
OUT="$1"; : > "$OUT"
/usr/bin/grep -rl 'from "bun:test"' --include='*.ts' . 2>/dev/null | sort | while IFS= read -r f; do
  R=$(bun test "$f" 2>&1 | /usr/bin/grep -E "^ *[0-9]+ (pass|fail)$" | tr '\n' ' ')
  echo "${R:-NORESULT} :: $f" >> "$OUT"
done
echo DONE >> "$OUT"
