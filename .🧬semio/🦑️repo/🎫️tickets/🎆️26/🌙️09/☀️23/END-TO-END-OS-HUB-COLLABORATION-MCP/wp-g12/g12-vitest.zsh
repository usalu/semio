#!/bin/zsh
# 🧪️ G12: runs renderer vitest files in the native lane (one file at a time, no browser). usage: zsh g12-vitest.zsh <capture> <filter…>
capture="$1"; shift
cd /Users/ueli/Documents/semio || exit 2
config='🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts'
NX_DAEMON=false SEMIO_TEST_LEVEL=long zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native g12 -- nice -n 15 bunx vitest run --config "$config" --fileParallelism=false --reporter=verbose "$@" > "$capture" 2>&1
echo "rc=$?" >> "$capture"
