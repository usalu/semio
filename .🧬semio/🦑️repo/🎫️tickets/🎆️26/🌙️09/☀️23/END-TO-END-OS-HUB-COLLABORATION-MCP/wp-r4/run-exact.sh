#!/bin/zsh
# usage: run-exact.sh <seconds> <test> [more tests...]
secs=$1; shift
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust"
for t in "$@"; do
  RUST_MIN_STACK=67108864 /Users/ueli/Documents/semio/.tmp-ticket/wp-r4/dbtest --exact "$t" --nocapture > /tmp/r4-exact.$$ 2>&1 &
  p=$!; i=0
  while kill -0 $p 2>/dev/null && [ $i -lt $secs ]; do sleep 1; i=$((i+1)); done
  if kill -0 $p 2>/dev/null; then kill $p; echo "TIMEOUT $t"; else
    if grep -q "1 passed" /tmp/r4-exact.$$; then echo "PASS $t"; else echo "FAIL $t"; grep -A6 "panicked at" /tmp/r4-exact.$$ | head -12; fi
  fi
done
rm -f /tmp/r4-exact.$$
