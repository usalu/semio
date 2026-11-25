#!/bin/sh
# usage: run-probe-open.sh  (starts the native host on 6260, runs probe-open.ts, stops the host; log 🗑️generated/probe-open.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
G="$here/🗑️generated"
bun=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
sh "$here/mount-products.sh"
sh "$here/serve-pdf-native.sh" > "$G/probe-open.host.log" 2>&1 &
host=$!
until grep -q "ready on" "$G/probe-open.host.log" || ! kill -0 "$host" 2>/dev/null; do sleep 2; done
cd "$root"
DEBUG=pw:browser "$bun" "$here/probe-open.ts" > "$G/probe-open.log" 2>&1
kill "$host" 2>/dev/null
wait "$host" 2>/dev/null
