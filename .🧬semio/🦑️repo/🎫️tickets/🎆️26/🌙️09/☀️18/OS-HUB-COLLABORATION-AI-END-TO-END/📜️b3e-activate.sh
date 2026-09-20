#!/bin/zsh
# 🔒 One react-dev activation for slice B3e, serialized through the fleet wasm mutex (preamble
# rule 27) so this worker never contributes to a `prebuild_lock_exclusive` cycle. ONE variant per
# call, foreground, with its own capture so a cut leaves evidence.
#
# Usage: 📜️b3e-activate.sh <variant>
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
VARIANT="$1"
LOG="$TICKET/🗑️generated/b3e-${VARIANT}-activate.txt"
CHAIN="$TICKET/🗑️generated/b3e-activate-chain3.txt"
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
echo "== $VARIANT queued $(date -u +%FT%TZ)" >> "$CHAIN"
echo "queued $(date -u +%FT%TZ)" > "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" b3e -- bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
CODE=$?
echo "exit=$CODE $(date -u +%FT%TZ)" >> "$LOG"
echo "== $VARIANT exit=$CODE $(date -u +%FT%TZ)" >> "$CHAIN"
exit $CODE
