#!/bin/zsh
# 🔗️ Slice F2: re-activate the batch-A variants this slice changed, strictly one cargo at a time.
# Never `dev <variant>` (that is `nx watch --all` and re-activates on every peer edit, taking the
# Vite server down mid-probe). Each variant writes its own capture; the chain file is the cursor.
# Usage: 📜️f2-activate-chain.sh <variant>...
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
CHAIN="$TICKET/🗑️generated/f2-activate-chain.txt"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
echo "chain start $(date -u +%FT%TZ) for: $*" > "$CHAIN"
for VARIANT in "$@"; do
  LOG="$TICKET/🗑️generated/f2-${VARIANT}-activate.txt"
  echo "→ $VARIANT $(date -u +%FT%TZ)" >> "$CHAIN"
  echo "start $(date -u +%FT%TZ)" > "$LOG"
  bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
  echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"
  echo "← $VARIANT $(tail -1 "$LOG")" >> "$CHAIN"
done
echo "chain done $(date -u +%FT%TZ)" >> "$CHAIN"
