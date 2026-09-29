#!/bin/zsh
# 🔱️ P9 trinity agent path over the semio MCP (session 15): G12's hub-lane coverage restricted to the trinity kinds (jack
# `graph.trinity`, rewriting `text.rewriting`) — create, open, one mutation (id-like inputs from the snapshot), undo, redo, export —
# over ONE delegated gateway against a live hub. user1 credentials from G12's private env file (never argv, never printed).
# usage: zsh p9-trinity-mcp.sh <tag> [hubOrigin]   env: SEMIO_OS_MCP_BIN (default G12's s15b gateway, else codecpage)
set -u
TAG="${1:?tag}"; HUB="${2:-http://127.0.0.1:7800}"
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
B="$H/s14-g12-bin"
export SEMIO_OS_MCP_BIN="${SEMIO_OS_MCP_BIN:-$( [ -x "$B/semio-os-mcp-s15b" ] && echo "$B/semio-os-mcp-s15b" || echo "$B/semio-os-mcp-codecpage" )}"
OUT="$H/s14-p9-logs/trinity-$TAG"; mkdir -p "$OUT"
say() { echo "$(date +%H:%M:%S) $*" | tee -a "$OUT/summary.txt"; }
say "trinity $TAG hub=$HUB gateway=$SEMIO_OS_MCP_BIN"
curl -sf -o /dev/null "$HUB/readyz" || { say "hub not ready at $HUB"; exit 1; }
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" || exit 1
( set -a; source "$H/s14-g12-credentials/user1.env"; set +a
  S_OS_MCP_COVERAGE_PLUGINS="trinity,rewriting" S_OS_MCP_COVERAGE_OUT="$OUT/coverage-hub" bun ./📜️script.ts plugin-coverage-check --hub "$HUB" ) > "$OUT/coverage-hub.txt" 2>&1
say "coverage-hub rc=$? $(/usr/bin/grep -E '^\[acceptance\]' "$OUT/coverage-hub.txt" | head -1 | cut -c1-300)"
/usr/bin/grep -h '"graph.trinity"\|"text.rewriting"' "$OUT/coverage-hub"/coverage-hub-rows.jsonl 2>/dev/null | cut -c1-1800 | tee -a "$OUT/summary.txt"
say "trinity $TAG done"
