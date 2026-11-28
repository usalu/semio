#!/bin/sh
# 🔨️ Builds only the sourcing hub wasm under the shared build lock; never stages, never restarts 6081.
# Usage: sh discard-wasm-build.sh   (from the workspace root)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
until mkdir "$here/🗑️generated/build.lock" 2>/dev/null; do sleep 20; done
NATIVE=1 ONLY=semio-hub-sourcing-component ONLY_HUB=1 sh "$here/native-wasm-build.sh" > "$here/🗑️generated/discard-wasm.out" 2>&1
echo "exit $?" >> "$here/🗑️generated/discard-wasm.out"
rmdir "$here/🗑️generated/build.lock"
