#!/bin/sh
# R32 (c)/(d): the relative helper on a capsule whose image is attached below it, as the cache opener does (anchor .cache/b, child g, entry .reset.json). usage: probe-mount.sh <scratch dir>; needs hdiutil, no sudo; detaches again.
set -e
S=$(cd "$1" && pwd -P); BUN=${BUN:-$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun}
HERE=$(cd "$(dirname "$0")" && pwd)
rm -rf "$S/cap" && mkdir -p "$S/cap/g/m"
hdiutil create -size 10m -fs APFS -volname r32 "$S/cap/g/fixed.dmg" >/dev/null
hdiutil attach -nobrowse -mountpoint "$S/cap/g/m" "$S/cap/g/fixed.dmg" >/dev/null
trap 'hdiutil detach "$S/cap/g/m" >/dev/null 2>&1 || true' EXIT
BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 "$BUN" "$HERE/probe-mount.ts" "$S/cap"
