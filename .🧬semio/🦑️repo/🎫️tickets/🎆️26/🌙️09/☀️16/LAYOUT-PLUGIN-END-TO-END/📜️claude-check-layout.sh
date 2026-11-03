#!/usr/bin/env bash
# 🦀️ Native type-check of the layout plugin crates through the repository cargo gate (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END).
set -u
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)" || exit 1
T="$(dirname "$0")"; LOG="$T/🗑️generated/claude/check-layout-$(date -u +%H%M%S).txt"
mkdir -p "$(dirname "$LOG")"; echo "start $(date -u +%FT%TZ)" > "$LOG"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-layout -p semio-hub-layout-component --message-format=short "$@" >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"; echo "$LOG"
