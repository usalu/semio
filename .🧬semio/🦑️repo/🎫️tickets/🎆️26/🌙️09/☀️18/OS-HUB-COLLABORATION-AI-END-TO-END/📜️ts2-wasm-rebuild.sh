#!/bin/zsh
# 🧊 TS2: rebuilds the two stale wasm-pack `pkg/` artifacts (puzzle board session, framework editor)
# whose `.d.ts` predate `pointerCancelScreen`. Runs as ONE fleet-mutex hold, both builds in order.
set -u
root="/Users/ueli/Documents/semio"
out="$root/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated"
export CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_INCREMENTAL=0
echo "=== ts2 wasm rebuild start $(date '+%H:%M:%S') ==="
cd "$root/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust" || exit 1
echo "--- puzzle wasm $(date '+%H:%M:%S')"
bun ./📜️script.ts wasm
echo "--- puzzle rc=$? $(date '+%H:%M:%S')"
cd "$root/🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust" || exit 1
echo "--- editor wasm $(date '+%H:%M:%S')"
bun ./📜️script.ts wasm
echo "--- editor rc=$? $(date '+%H:%M:%S')"
ls -la "$root/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg" "$root/🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/pkg"
echo "=== ts2 wasm rebuild done $(date '+%H:%M:%S') ==="
