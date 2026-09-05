#!/usr/bin/env bash
# 🔁️ Peers rename artifact directories every few minutes while long builds run, so a build valid at
# launch fails minutes later on a path that moved under it. Re-heals broken `#[path]` attributes every
# 45s for the crates the demonstrator needs. Safe by construction: the healer rewrites a segment only
# when EXACTLY ONE sibling on disk shares its ASCII tail; anything ambiguous is reported, never guessed.
set -uo pipefail
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
LOG="${HEAL_LOG:?HEAL_LOG required}"
while true; do
  out=$(python3 "$T/🩺️heal-paths.py" --scan "✏️s/🔌️plugins" 2>&1 | grep -E '^  healed|^  UNRESOLVED')
  [ -n "$out" ] && echo "[$(date '+%H:%M:%S')] $out" >> "$LOG"
  sleep 45
done
