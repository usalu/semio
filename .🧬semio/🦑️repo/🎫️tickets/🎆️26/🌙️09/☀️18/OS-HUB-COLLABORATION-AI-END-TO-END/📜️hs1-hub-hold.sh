#!/usr/bin/env zsh
# 🧵️ HS1 — holds this slice's OWN hub (never 7611) on a COPIED data root and a COPIED binary, so the
# pool-worker stack overflow can be reproduced and re-measured without touching GM1's live hub.
# Usage: 📜️hs1-hub-hold.sh [port] [binary] [dataRoot]   (RUST_MIN_STACK may be exported by the caller)
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
PORT="${1:-7631}"
BIN="${2:-$ROOT/.🧬semio/🦑️repo/⚡️cache/hs1/os-hub}"
DATA="${3:-$ROOT/.🧬semio/🌐hub/hs1-boot}"
LOG="${HS1_HUB_LOG:-$TICKET/🗑️generated/hs1-hub-hold.txt}"
export OS_HUB_CREDENTIAL_SIGN_IN=true SEMIO_BUILD_BUDGET_MS=1800000
: > "$LOG"
cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" >> "$LOG" 2>&1
