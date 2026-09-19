#!/bin/zsh
# 🏗️ One-shot react dev activation for a slice-B3d plugin (no `nx watch`, which re-activates on peer edits).
# Usage: 📜️b3d-activate.sh <variant> <port>
VARIANT="$1"; PORT="$2"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT="$PORT"
LOG=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated/b3d-${VARIANT}-activate.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
