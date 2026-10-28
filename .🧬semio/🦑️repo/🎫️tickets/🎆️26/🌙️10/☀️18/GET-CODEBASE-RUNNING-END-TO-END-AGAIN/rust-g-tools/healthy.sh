#!/bin/sh
# healthy.sh : waits (max 15 min) until the build library that every `bun repo` run imports parses again (a peer's half-finished edit breaks every run with a syntax error)
cd /Users/ueli/Documents/semio || exit 1
for i in $(seq 1 90); do
  bun -e 'await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts");await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts")' >/dev/null 2>&1 && exit 0
  sleep 10
done
exit 1
