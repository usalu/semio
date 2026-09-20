#!/bin/zsh
# 🔗️ Second slice-B3d activation chain, started BEHIND the first one so this worker never holds two
# cargos at once (preamble rule 7): it blocks on chain-1's pid before its first build.
#
# What it rebuilds and why:
#   lowpoly      — chain 1's run was cut (`exit=130` at 09:11) by the session's death, never finished
#   demonstrator — carries this slice's store-owners root fix (report §3.2), unverified until rebuilt
#   fem2d/fem3d  — carry the two `[DEBUG]` eprintln removals (report §3.3); both variants share ONE
#                  `🏗️fem` component, so the second is a component cache hit and only re-receipts
#
# Usage: 📜️b3d-activate-chain2.sh <pid-to-wait-for> <variant> [<variant> …]
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
CHAIN="$TICKET/🗑️generated/b3d-activate-chain2.txt"
WAIT_PID="$1"; shift
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
echo "chain2 armed $(date -u +%FT%TZ) behind pid $WAIT_PID, variants: $*" >> "$CHAIN"
while kill -0 "$WAIT_PID" 2>/dev/null; do sleep 30; done
echo "chain2 start $(date -u +%FT%TZ)" >> "$CHAIN"
for VARIANT in "$@"; do
  LOG="$TICKET/🗑️generated/b3d-${VARIANT}-activate.txt"
  echo "== $VARIANT start $(date -u +%FT%TZ)" >> "$CHAIN"
  echo "start $(date -u +%FT%TZ)" > "$LOG"
  bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
  CODE=$?
  echo "exit=$CODE $(date -u +%FT%TZ)" >> "$LOG"
  echo "== $VARIANT exit=$CODE $(date -u +%FT%TZ)" >> "$CHAIN"
done
echo "chain2 done $(date -u +%FT%TZ)" >> "$CHAIN"
