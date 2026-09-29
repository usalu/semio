#!/bin/zsh
# 🧪️ C12: the os root in-source laws (`💻️os/🟦️.ts`: backbone envelope io, mailbox refusal vocabulary, …) in the native lane.
# usage: zsh vitest-os-root.sh <capture>
out="$1"; mkdir -p "$(dirname "$out")"
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" || exit 1
echo "=== queued $(date +%T)" > "$out"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c12 -- zsh -c 'echo "=== start $(date +%T)"; NX_DAEMON=false nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts"; echo "=== exit $? $(date +%T)"' >> "$out" 2>&1
