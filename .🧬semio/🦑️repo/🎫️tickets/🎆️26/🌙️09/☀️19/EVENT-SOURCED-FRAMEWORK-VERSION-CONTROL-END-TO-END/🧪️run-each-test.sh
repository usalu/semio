#!/bin/zsh
# Runs every test matching $2 of test binary $1 in its own process; prints "ok|FAIL name".
BIN="$1"; FILTER="$2"
"$BIN" --list 2>/dev/null | grep ': test$' | sed 's/: test$//' | grep -- "$FILTER" | while read -r name; do
  if "$BIN" --exact "$name" --test-threads=1 >/dev/null 2>&1; then echo "ok $name"; else echo "FAIL $name"; fi
done
