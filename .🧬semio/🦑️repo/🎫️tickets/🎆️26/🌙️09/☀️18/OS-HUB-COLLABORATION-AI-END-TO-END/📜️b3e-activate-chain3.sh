#!/bin/zsh
# 🔗️ Third activation chain (slice B3e), armed BEHIND B3d's chain 2 so this worker never holds two
# cargos at once (preamble rule 7).
#
# What it rebuilds and why:
#   shooting — B3d's chain-1 run was killed at 12:17 under the rule-23 prebuild-lock check, never retried
#   sourcing — B3d's chain-1 run starved in `prebuild_lock_exclusive` for 81 min and was stopped at 13:38
#   raster   — carries this slice's `retire_projection` / `retire_cold` root fix for the undo trap
#
# Usage: 📜️b3e-activate-chain3.sh <pid-to-wait-for> <variant> [<variant> …]
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
CHAIN="$TICKET/🗑️generated/b3e-activate-chain3.txt"
WAIT_PID="$1"; shift
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
echo "chain3 armed $(date -u +%FT%TZ) behind pid $WAIT_PID, variants: $*" >> "$CHAIN"
while kill -0 "$WAIT_PID" 2>/dev/null; do sleep 30; done
echo "chain3 start $(date -u +%FT%TZ)" >> "$CHAIN"
for VARIANT in "$@"; do
  LOG="$TICKET/🗑️generated/b3e-${VARIANT}-activate.txt"
  echo "== $VARIANT start $(date -u +%FT%TZ)" >> "$CHAIN"
  echo "start $(date -u +%FT%TZ)" > "$LOG"
  bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
  CODE=$?
  echo "exit=$CODE $(date -u +%FT%TZ)" >> "$LOG"
  echo "== $VARIANT exit=$CODE $(date -u +%FT%TZ)" >> "$CHAIN"
done
echo "chain3 done $(date -u +%FT%TZ)" >> "$CHAIN"
