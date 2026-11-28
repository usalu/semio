#!/bin/sh
# 🔁️ Rebuilds each given sourcing component under the build lock until its wasm is fresh (peers' half-edited crates fail single attempts), then stages, restarts 6081 and runs every journey.
# Usage: [NATIVE=1] sh rebuild-sourcing-retry.sh "<package> ..." [attempts]   (from the workspace root)
# @see ./after-hub.sh
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
for package in $1; do
  base=$(echo "$package" | tr - _); i=0; start=$(date +%s)
  until [ -f "$here/🗑️generated/native-wasm/$base.wasm" ] && [ "$(stat -f %m "$here/🗑️generated/native-wasm/$base.wasm")" -gt "$start" ]; do
    [ $i -ge "${2:-12}" ] && { echo "gave up $package"; exit 1; }
    until mkdir "$here/🗑️generated/build.lock" 2>/dev/null; do sleep 30; done
    hub=; case $package in semio-hub-*) hub=1;; esac
    NATIVE="${NATIVE:-}" ONLY="$package" ONLY_HUB=$hub sh "$here/native-wasm-build.sh" > "$here/🗑️generated/retry-$base-$i.txt" 2>&1
    rmdir "$here/🗑️generated/build.lock"; echo "$package attempt $i done"; i=$((i+1))
    [ -f "$here/🗑️generated/native-wasm/$base.wasm" ] && [ "$(stat -f %m "$here/🗑️generated/native-wasm/$base.wasm")" -gt "$start" ] || sleep 120
  done
done
BUN=$(ls -d ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SEMIO_NATIVE_TRANSPILE_BIN=${SEMIO_NATIVE_TRANSPILE_BIN:-$PWD/.cache/b/p/m/browser/prototype/tools/native-transpile/darwin/semio-native-transpile} "$BUN" "$here/native-stage-modules.ts" sourcing sourcing-module-beams sourcing-module-slabs sourcing-module-windows || exit 1
pkill -f serve-sourcing-native.ts; sleep 2; rm -f "$here/🗑️generated/serve-native.txt"
nohup "$BUN" "$here/serve-sourcing-native.ts" > "$here/🗑️generated/serve-native.txt" 2>&1 &
until grep -q ready "$here/🗑️generated/serve-native.txt" 2>/dev/null; do sleep 2; done
rm -rf "$here/🗑️generated/all-journeys"; sh "$here/run-all-journeys.sh"; cat "$here/🗑️generated/all-journeys/summary.txt"
