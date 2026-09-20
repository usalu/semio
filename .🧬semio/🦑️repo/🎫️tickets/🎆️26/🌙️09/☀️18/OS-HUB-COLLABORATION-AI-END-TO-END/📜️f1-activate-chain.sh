#!/bin/zsh
# 🔗️ Slice F1: activate the remaining batch-A variants back-to-back, strictly one cargo at a time.
# The shared cargo build-dir lock is fleet-contended (32 queued cargos, all in `prebuild_lock_exclusive`),
# so an activation can idle 30+ min before it gets the lock; chaining means the wait is paid once per
# variant instead of once per polling round. Usage: 📜️f1-activate-chain.sh <wait-for-pid> <variant>...
WAIT_PID="$1"; shift
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
CHAIN="$TICKET/🗑️generated/f1-activate-chain.txt"
echo "chain start $(date -u +%FT%TZ) waiting on pid=$WAIT_PID for: $*" > "$CHAIN"
if [[ -n "$WAIT_PID" && "$WAIT_PID" != "0" ]]; then
  while kill -0 "$WAIT_PID" 2>/dev/null; do /usr/bin/python3 -c "import time; time.sleep(20)"; done
fi
for VARIANT in "$@"; do
  echo "→ $VARIANT $(date -u +%FT%TZ)" >> "$CHAIN"
  zsh "$TICKET/📜️b1a-activate.sh" "$VARIANT"
  echo "← $VARIANT $(tail -1 "$TICKET/🗑️generated/b1a-${VARIANT}-activate.txt")" >> "$CHAIN"
done
echo "chain done $(date -u +%FT%TZ)" >> "$CHAIN"
