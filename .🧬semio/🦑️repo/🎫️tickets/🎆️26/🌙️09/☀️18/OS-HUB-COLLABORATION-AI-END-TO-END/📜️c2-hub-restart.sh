#!/usr/bin/env zsh
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
LOG="$TICKET/🗑️generated/c2-hub-restart.txt"
export OS_HUB_CREDENTIAL_SIGN_IN=true SEMIO_BUILD_BUDGET_MS=1800000
: > "$LOG"
cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" 7611 "$ROOT/.🧬semio/🌐hub/gm1-boot" "$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-gm1/debug/os-hub" >> "$LOG" 2>&1
