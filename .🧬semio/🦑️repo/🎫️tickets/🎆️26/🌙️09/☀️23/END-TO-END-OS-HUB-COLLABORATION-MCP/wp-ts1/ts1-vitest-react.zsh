#!/bin/zsh
# 🧪️ TS1: runs the renderer-react vitest suites touched by the os tsc 0-error sweep (one file at a time, ≤ 1 browser).
# usage: zsh ts1-vitest-react.zsh <capture> <filter…> [-- vitest args…]
capture="$1"; shift
cd /Users/ueli/Documents/semio || exit 2
config='🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts'
NX_DAEMON=false SEMIO_TEST_LEVEL=long zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native ts1 -- nice -n 15 bunx vitest run --config "$config" --fileParallelism=false --reporter=verbose "$@" > "$capture" 2>&1
echo "rc=$?" >> "$capture"
