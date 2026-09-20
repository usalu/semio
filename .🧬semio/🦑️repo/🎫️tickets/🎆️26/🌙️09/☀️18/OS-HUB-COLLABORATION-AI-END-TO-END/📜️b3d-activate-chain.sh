#!/bin/zsh
# 🔗️ Serial react-dev activation chain for slice B3d.
#
# Eight of the fourteen B3d variants have real (non-test) Rust source newer than their staged
# component — M5a's 191-declaration `action_audience`/`action_destructive` sweep landed in every
# `✏️editor/🦀️.rs` between 00:10 and 02:50 on 2026-09-20 — so their served guest predates the
# sources and a bar measurement on it would not be a measurement of the tree. This chain rebuilds
# them ONE AT A TIME (preamble rule 7: at most one cargo from this worker) and writes one capture
# per variant, so a cut leaves every finished variant's evidence on disk.
#
# Usage: 📜️b3d-activate-chain.sh <variant> [<variant> …]
cd /Users/ueli/Documents/semio || exit 1
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
CHAIN="$TICKET/🗑️generated/b3d-activate-chain.txt"
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false SEMIO_RENDERER=react
echo "chain start $(date -u +%FT%TZ) variants: $*" >> "$CHAIN"
for VARIANT in "$@"; do
  LOG="$TICKET/🗑️generated/b3d-${VARIANT}-activate.txt"
  echo "== $VARIANT start $(date -u +%FT%TZ)" >> "$CHAIN"
  echo "start $(date -u +%FT%TZ)" > "$LOG"
  bun nx run "@semio-tech/framework-os-dev:activate-${VARIANT}-react-dev" >> "$LOG" 2>&1
  CODE=$?
  echo "exit=$CODE $(date -u +%FT%TZ)" >> "$LOG"
  echo "== $VARIANT exit=$CODE $(date -u +%FT%TZ)" >> "$CHAIN"
done
echo "chain done $(date -u +%FT%TZ)" >> "$CHAIN"
