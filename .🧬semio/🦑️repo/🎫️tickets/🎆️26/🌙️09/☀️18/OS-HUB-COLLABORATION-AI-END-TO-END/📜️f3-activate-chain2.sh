#!/bin/zsh
# 🔗️ Slice F3: re-activate batch-A variants one at a time, EACH step through the fleet wasm mutex
# (preamble rule 27). Never `dev <variant>` (that is `nx watch --all`).
# Usage: 📜️f3-activate-chain.sh <variant>...
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
CHAIN="$TICKET/🗑️generated/f3-activate-chain2.txt"
MUTEX="$TICKET/📜️wasm-build-mutex.sh"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
echo "chain start $(date -u +%FT%TZ) for: $*" > "$CHAIN"
for VARIANT in "$@"; do
  LOG="$TICKET/🗑️generated/f3-${VARIANT}-activate.txt"
  echo "→ $VARIANT $(date -u +%FT%TZ)" >> "$CHAIN"
  echo "start $(date -u +%FT%TZ)" > "$LOG"
  zsh "$MUTEX" f3 -- bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
  echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
  echo "← $VARIANT $(tail -1 "$LOG")" >> "$CHAIN"
done
echo "chain done $(date -u +%FT%TZ)" >> "$CHAIN"
