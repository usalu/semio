#!/bin/sh
# usage: run-round4-journey.sh <out name> [steps...]  (starts the native host on 6260, runs pdf-pane-journey.ts with PDF_JOURNEY_OUT=<out name>, stops the host; logs in 🗑️generated/<out name>.{host,out}.log)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
G="$here/🗑️generated"
name=${1:?out name}; shift
bun=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
sh "$here/mount-products.sh"
sh "$here/serve-pdf-native.sh" > "$G/$name.host.log" 2>&1 &
host=$!
until grep -q "ready on" "$G/$name.host.log" || ! kill -0 "$host" 2>/dev/null; do sleep 2; done
cd "$root"
status=0
PDF_JOURNEY_OUT="$name" "$bun" "$here/pdf-pane-journey.ts" "$@" > "$G/$name.out.log" 2>&1 || status=$?
kill "$host" 2>/dev/null
wait "$host" 2>/dev/null
echo "exit $status" > "$G/$name.done"
