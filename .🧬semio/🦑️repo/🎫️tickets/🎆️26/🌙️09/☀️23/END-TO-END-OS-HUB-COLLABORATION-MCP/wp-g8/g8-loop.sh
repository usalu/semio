#!/bin/bash
# g8: loop a filtered os-mcp lib test binary; $1 = filter, $2 = runs, $3 = capture name.
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust"
B=/Users/ueli/Documents/semio/.tmp-ticket/wp-g8/target/mcp-lib-bin
O=/Users/ueli/Documents/semio/.tmp-ticket/wp-g8/generated/$3.txt
: > "$O"; f=0
for i in $(seq 1 "$2"); do
  if ! "$B" "$1" > "$O.last" 2>&1; then f=$((f+1)); grep -E "FAILED|panicked" "$O.last" >> "$O"; fi
done
rm -f "$O.last"
echo "filter=$1 runs=$2 failures=$f load=$(uptime | sed 's/.*averages: //')" >> "$O"
