#!/usr/bin/env zsh
# 🌎️ Slice GM1 — step 2 only: hold the hub on an ALREADY PUBLISHED trusted catalog.
# `OS_HUB_CREDENTIAL_SIGN_IN=true` is the difference from 📜️gm1-hub-boot.sh's inline hold: without it
# `/readyz` reports `publicSessionIssuance: false` and `POST /auth/sessions` answers 403, so no browser
# can sign in and outcome 3's two humans cannot reach this hub at all.
# Usage: 📜️gm1-hub-hold.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7611}
DATA="$ROOT/.🧬semio/🌐hub/gm1-boot"
BIN="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-gm1/debug/os-hub"
LOG="$GEN/gm1-hub-hold.txt"
export OS_HUB_CREDENTIAL_SIGN_IN=true
: > "$LOG"
cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" >> "$LOG" 2>&1
