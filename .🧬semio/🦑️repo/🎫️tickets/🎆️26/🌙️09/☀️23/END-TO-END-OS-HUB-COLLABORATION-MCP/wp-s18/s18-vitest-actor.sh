#!/bin/zsh
# 🧪️ S18: 🎭️actor in-source laws (vitest) for the given files, run inside the native lane.
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript"
cd "$P" && echo "[s18] HELD $(date '+%F %T')" && SEMIO_TEST_LEVEL=long NX_DAEMON=false nice -n 15 bun "$P/📜️script.ts" test "$@"; echo "[s18] rc=$? $(date '+%F %T')"
