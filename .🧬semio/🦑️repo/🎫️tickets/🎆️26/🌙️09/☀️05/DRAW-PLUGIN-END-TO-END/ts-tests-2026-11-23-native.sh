#!/bin/sh
# Native fallback: runs every Draw subset bun test file separately when the repo graph service is unavailable.
B="$1"; LIST="$2"; OUT="$3"
: > "$OUT"
while IFS= read -r f; do
  if timeout 240 "$B" test --timeout 120000 "./$f" > /tmp/ts-one.$$ 2>&1; then echo "PASS $f" >> "$OUT"; else echo "FAIL $f" >> "$OUT"; tail -25 /tmp/ts-one.$$ >> "$OUT"; fi
done < "$LIST"
rm -f /tmp/ts-one.$$
