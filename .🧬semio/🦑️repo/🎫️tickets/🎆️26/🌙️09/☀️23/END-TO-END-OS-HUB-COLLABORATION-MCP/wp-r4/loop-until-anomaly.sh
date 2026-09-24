#!/bin/zsh
# usage: loop-until-anomaly.sh <test> <runs> <per-run-seconds> <outfile>
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust"
for i in $(seq 1 $2); do
  RUST_MIN_STACK=67108864 /Users/ueli/Documents/semio/.tmp-ticket/wp-r4/dbtest --exact "$1" --nocapture > "$4" 2>&1 &
  p=$!; n=0
  while kill -0 $p 2>/dev/null && [ $n -lt $3 ]; do sleep 1; n=$((n+1)); done
  if kill -0 $p 2>/dev/null; then kill $p; echo "HANG at run $i"; exit 0; fi
  if ! grep -q "1 passed" "$4"; then echo "FAIL at run $i"; exit 0; fi
done
echo "no anomaly in $2 runs"
