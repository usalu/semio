#!/bin/zsh
# 🧪️ C12: the React TextEditor delivery laws (engine-contract "editor delivery" + "splice typing") with the react package's own vitest
# config, in the native lane. usage: zsh vitest-delivery.sh <capture>
OUT="$1"
REACT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'cd "$0" && echo "=== start $(date +%T)" && NX_DAEMON=false nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "🔬️engine-contract" -t "editor delivery|splice typing|neutral editor delivery"; echo "=== exit $? $(date +%T)"' "$REACT" >> "$OUT" 2>&1
