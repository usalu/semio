#!/usr/bin/env zsh
# 🛂️ Slice DS1 session 5 — §9 gap 3. Re-emits real plugin descriptors through each plugin's OWN
# `describe` verb (`bun ./📜️script.ts describe` → `describePluginComponent`: a `wasm32-wasip2`
# `wasm-dev` component build, jco core extraction, then `🛂️.descriptor.semio` + `🔣️.json` at the
# plugin's owner root), then re-runs `🐍️ds1-descriptor-census.mjs` over the whole plugin tree.
# Targets the two plugins the session-4 census singles out: 🧩️puzzle (the one pack ALREADY over the
# 4 MiB contract bound) and 📕️norm (the worst duplication ratio, 21 distinct rows stored 675 times).
# Private artifact dir only (see §10.2) — the shared build-dir is untouched.
# Usage: 📜️ds1-redescribe.sh [plugin-dir-name ...]
set -u
ROOT=/Users/ueli/Documents/semio
GEN="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated"
LOG="$GEN/ds1-redescribe.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-ds1"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

if [ $# -gt 0 ]; then plugins=("$@"); else plugins=("🧩️puzzle" "📕️norm"); fi

: > "$LOG"
for plugin in "${plugins[@]}"; do
  dir="$ROOT/✏️s/🔌️plugins/$plugin/📦️packages/🦀️rust"
  echo "=== describe $plugin at $(date -Iseconds) ===" >> "$LOG"
  if [ ! -d "$dir" ]; then echo "no rust package at $dir" >> "$LOG"; continue; fi
  ( cd "$dir" && bun ./📜️script.ts describe ) >> "$LOG" 2>&1
  echo "=== exit $? ===" >> "$LOG"
  ls -l "$ROOT/✏️s/🔌️plugins/$plugin/🛂️.descriptor.semio" >> "$LOG" 2>&1
done

echo "=== census after at $(date -Iseconds) ===" >> "$LOG"
( cd "$ROOT" && bun "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️ds1-descriptor-census.mjs" ) > "$GEN/ds1-descriptor-census-after.txt" 2>&1
echo "=== census exit $? ===" >> "$LOG"
