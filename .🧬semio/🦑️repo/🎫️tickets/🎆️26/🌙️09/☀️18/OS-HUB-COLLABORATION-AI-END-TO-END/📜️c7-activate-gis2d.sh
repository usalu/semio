#!/usr/bin/env zsh
# ♻️ Slice C5 — re-activate the `gis2d` dev variant so its plugin component carries the CURRENT
# kernel wire. The staged component (`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/dist/component-dev/`,
# 2026-09-20 16:46) predates the `HistoryEntry.label: dsl::LocalizedLabel` change
# (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`, 2026-09-21 04:01), so every history row it emits carries
# a label without the terminology axis and the shell's reader throws on the first render.
# Runs inside the fleet wasm build mutex (preamble rule 27).
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
LOG="$TICKET/🗑️generated/c7-activate-gis2d.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false SEMIO_RENDERER=react
: > "$LOG"
echo "=== activate gis2d react dev at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" c7 -- \
  zsh -c "cd '$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript' && bun ./📜️script.ts activate gis2d react dev" >> "$LOG" 2>&1
echo "=== activate exit $? at $(date -Iseconds) ===" >> "$LOG"
