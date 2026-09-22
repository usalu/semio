#!/usr/bin/env zsh
# 🔨️ Slice C8 — rebuild the hub binary into C8's own uplift dir after the creation-observability fix.
# Private CARGO_TARGET_DIR (preamble rule 25), shared build-dir, no incremental (rule 30).
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
LOG="$TICKET/🗑️generated/c8-hub-rebuild.txt"
export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-c8-hub"
: > "$LOG"
echo "=== cargo build --bin os-hub at $(date -Iseconds) ===" >> "$LOG"
cd "$ROOT/🌎️hub/📦️packages/🦀️rust" && cargo build --manifest-path Cargo.toml --bin os-hub >> "$LOG" 2>&1
echo "=== exit $? at $(date -Iseconds) ===" >> "$LOG"
