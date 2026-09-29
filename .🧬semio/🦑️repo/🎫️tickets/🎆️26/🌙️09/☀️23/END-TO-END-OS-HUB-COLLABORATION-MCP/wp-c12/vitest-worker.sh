#!/bin/zsh
# 🧪️ C12: the os worker in-source laws (+ the os root laws) in the native lane. Usage: zsh vitest-worker.sh <capture>
out="$1"; mkdir -p "$(dirname "$out")"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" || exit 1
echo "=== queued $(date +%T)" > "$out"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'echo "=== start $(date +%T)"; NX_DAEMON=false nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "👷️worker" "💻️os/🟦️.ts" "backbone-envelope-io"; echo "=== exit $? $(date +%T)"' >> "$out" 2>&1
