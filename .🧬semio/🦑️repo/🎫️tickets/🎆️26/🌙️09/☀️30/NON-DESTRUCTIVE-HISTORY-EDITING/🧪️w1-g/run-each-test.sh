#!/bin/zsh
# Runs every test of binary $1 whose name matches one of the prefixes $3.. in its own process (6 in parallel); writes "ok|FAIL name" lines to $2.
BIN="$1"; OUT="$2"; shift 2
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
"$BIN" --list 2>/dev/null | /usr/bin/grep ': test$' | sed 's/: test$//' | /usr/bin/grep -E "^($(IFS='|'; echo "$*"))" > "$OUT.names"
: > "$OUT"
cat "$OUT.names" | xargs -P 6 -I{} zsh -c 'if "$0" --exact "$1" --test-threads=1 >/dev/null 2>&1; then echo "ok $1"; else echo "FAIL $1"; fi' "$BIN" {} >> "$OUT"
sort -k2 "$OUT" -o "$OUT"
echo "done $(/usr/bin/grep -c '^ok' "$OUT") ok $(/usr/bin/grep -c '^FAIL' "$OUT") fail" >> "$OUT.summary"
