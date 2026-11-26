#!/bin/sh
# 🚚️ Waits for a running hub build (pid $1), then stages the four sourcing modules, restarts the 6081 host and runs every journey in English and German.
# @see ./run-all-journeys.sh
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
while kill -0 "$1" 2>/dev/null; do sleep 30; done
tail -3 "$here/🗑️generated/boot-fault/hub-build.txt"
BUN=$(ls -d ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SEMIO_NATIVE_TRANSPILE_BIN=${SEMIO_NATIVE_TRANSPILE_BIN:-$PWD/.cache/b/p/m/browser/prototype/tools/native-transpile/darwin/semio-native-transpile} "$BUN" "$here/native-stage-modules.ts" sourcing sourcing-module-beams sourcing-module-slabs sourcing-module-windows || exit 1
pkill -f serve-sourcing-native.ts; sleep 2
nohup "$BUN" "$here/serve-sourcing-native.ts" > "$here/🗑️generated/serve-native.txt" 2>&1 &
until grep -q ready "$here/🗑️generated/serve-native.txt" 2>/dev/null; do sleep 2; done
rm -rf "$here/🗑️generated/all-journeys"; sh "$here/run-all-journeys.sh"; cat "$here/🗑️generated/all-journeys/summary.txt"
