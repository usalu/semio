#!/bin/zsh
# 🏗️ One-shot react dev activation for one trinity/wfc/puzzle artifact (slice B3b). No `nx watch`:
# `dev <variant>` re-activates on every peer edit and takes the Vite server down mid-probe.
# Usage: 📜️b3b-activate.sh <variant>
VARIANT="$1"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3b-${VARIANT}-activate.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
