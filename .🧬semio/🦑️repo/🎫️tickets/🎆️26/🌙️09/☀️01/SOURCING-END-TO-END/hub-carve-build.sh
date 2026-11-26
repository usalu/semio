#!/bin/sh
# 🪚️ Serialized hub-component carving build: takes the shared build lock, builds the sourcing closure through the admitted entry, releases the lock.
# Usage: ONLY="pkg ..." [ONLY_HUB=1] hub-carve-build.sh <label>
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
lock="$here/🗑️generated/build.lock"
until mkdir "$lock" 2>/dev/null; do sleep 30; done
trap 'rmdir "$lock" 2>/dev/null' EXIT INT TERM
mkdir -p "$here/🗑️generated/hub-carve"
sh "$here/native-wasm-build.sh" > "$here/🗑️generated/hub-carve/build-$1.txt" 2>&1
echo "exit $?" >> "$here/🗑️generated/hub-carve/build-$1.txt"
