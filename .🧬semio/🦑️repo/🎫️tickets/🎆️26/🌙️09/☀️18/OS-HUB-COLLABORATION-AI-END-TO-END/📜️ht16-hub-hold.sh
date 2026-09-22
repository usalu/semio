#!/usr/bin/env zsh
# 🌎️ Slice HT16 — hold ONE hub on port 7681 on HT16's OWN copy of the jc1 catalog root, executing the
# binary this slice built in its private target dir. Never shares a data root and never restarts a
# peer's hub (7621 = C8/CE3, 7641 = S11, 7651 = TC3e).
# Usage: 📜️ht16-hub-hold.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7681}
DATA="$ROOT/.🧬semio/🌐hub/ht16-boot"
BIN="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-ht16/debug/os-hub"
LOG="$GEN/ht16-hub-7681.txt"
export OS_HUB_CREDENTIAL_SIGN_IN=true
: > "$LOG"
cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" >> "$LOG" 2>&1
