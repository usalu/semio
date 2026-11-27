#!/bin/bash
# [DEBUG] direct per-file runner used when the repo capacity gate starves the runner
cd /Users/ueli/Documents/semio
BUN=$(cat /tmp/bunpath); OUT="$1"; : > "$OUT"
while IFS= read -r f; do
  r=$(timeout 300 "$BUN" test --timeout 120000 "./$f" 2>&1); c=$?
  echo "=== $c $f" >> "$OUT"; [ $c -ne 0 ] && echo "$r" | tail -40 >> "$OUT"
  echo "$r" | grep -E '^\s*[0-9]+ (pass|fail)' | tr '\n' ' ' >> "$OUT"; echo >> "$OUT"
done < /tmp/x
echo DONE >> "$OUT"
