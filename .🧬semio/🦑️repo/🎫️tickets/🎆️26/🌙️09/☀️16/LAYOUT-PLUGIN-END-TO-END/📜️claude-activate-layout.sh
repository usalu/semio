#!/usr/bin/env bash
# 📏️ Prepare + activate the layout react prototype runtime through the tool-neutral build entry (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END).
set -u
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)" || exit 1
T="$(dirname "$0")"; LOG="$T/🗑️generated/claude/activate-layout-$(date -u +%H%M%S).txt"
mkdir -p "$(dirname "$LOG")"; echo "start $(date -u +%FT%TZ)" > "$LOG"
SEMIO_LANGUAGE=en repo run @semio-tech/framework-os-dev:activate-layout-react-prototype --output=stream "$@" >> "$LOG" 2>&1
echo "exit=$? $(date -u +%FT%TZ)" >> "$LOG"; echo "$LOG"
