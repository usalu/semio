#!/bin/sh
# usage: run-probe-strip.sh  (host on 6260, probe-strip.ts, host stopped; output 🗑️generated/probe-strip.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
G="$here/🗑️generated"
bun=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
sh "$here/serve-pdf-native.sh" > "$G/probe-strip.host.log" 2>&1 &
host=$!
until grep -q "ready on" "$G/probe-strip.host.log" || ! kill -0 "$host" 2>/dev/null; do sleep 2; done
cd "$root"
"$bun" "$here/probe-strip.ts" > "$G/probe-strip.log" 2>&1
kill "$host" 2>/dev/null
wait "$host" 2>/dev/null
echo done >> "$G/probe-strip.log"
