#!/bin/bash
# ⏳️ Re-checks the upstream framework closure (workspace resolution + `semio-framework-plugin` lib) of one stdio
# workspace every 150 s until it is green or the budget (seconds, default 540) runs out; exit 0 when green.
# usage: ⏳️await-upstream.sh <workspace Cargo.toml> [budget seconds]
WS=$1; BUDGET=${2:-540}; HERE="$(cd "$(dirname "$0")" && pwd)"; start=$(date +%s)
while :; do
  if "$HERE/🏗️cargo.sh" upstream check --manifest-path "$WS" -p semio-framework-plugin -p semio-framework-os-kernel --lib --message-format short >/dev/null; then echo "upstream green"; exit 0; fi
  grep -E "^error" "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/p8/logs/upstream.log" | head -2
  [ $(( $(date +%s) - start + 150 )) -gt "$BUDGET" ] && { echo "upstream still red"; exit 1; }
  sleep 150
done
