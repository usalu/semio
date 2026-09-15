#!/bin/zsh
# 🔁 Lane `react-remaining-reds`: restage the generation3d React guest with the nx cache SKIPPED.
# The peer wave that batched `actor-ui-patch` receipts changed `🎭️actor/🚪️lifetime/🩹️patch/🦀️.rs` on the
# GUEST side (2026-09-15 00:37) while the served wasm was from 2026-09-14 23:07, so every boot on 6018
# died on `actor-ui-patch.pairing` — the host's new one-receipt-per-batch validator against a guest that
# still emits one receipt per patch. Retried through peer churn, exactly like `📜️restage-retry-s5.sh`.
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/react-reds/restage.txt"
mkdir -p "$(dirname "$LOG")"
cd /Users/ueli/Documents/semio
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false NX_SKIP_NX_CACHE=true
for attempt in $(seq 1 6); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 120
done
date >> "$LOG"
echo "RESTAGE-REACT-REDS-DONE" >> "$LOG"
