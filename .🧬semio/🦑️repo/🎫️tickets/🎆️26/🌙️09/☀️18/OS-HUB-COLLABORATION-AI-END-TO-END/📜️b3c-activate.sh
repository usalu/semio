#!/bin/zsh
# 🏗️ One-shot react dev activation for a procedural/flow/process variant (slice B3c).
# Preamble rule 27: the whole activation chain (component-dev → materialize-dev, all wasm32) runs
# inside the fleet wasm build mutex, so it never joins a `prebuild_lock_exclusive` cycle.
# Usage: 📜️b3c-activate.sh <variant> <port>
VARIANT="$1"; PORT="$2"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react S_OS_PORT="$PORT"
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
LOG="$TICKET/🗑️generated/b3c-${VARIANT}-activate.txt"
echo "start $(date -u +%FT%TZ)" > "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" b3c -- bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
