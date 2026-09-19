#!/bin/zsh
# 🏗️ One-shot react dev activation for a batch-A dormant plugin. Deliberately NOT `dev <variant>`:
# that runs `nx watch --all` and re-activates on every peer edit, taking the Vite server down
# mid-probe. Usage: 📜️b1a-activate.sh <variant>
VARIANT="$1"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b1a-${VARIANT}-activate.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
