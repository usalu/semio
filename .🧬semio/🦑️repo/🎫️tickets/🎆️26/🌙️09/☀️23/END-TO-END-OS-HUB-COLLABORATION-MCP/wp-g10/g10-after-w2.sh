#!/bin/zsh
# 🧭️ G10: every hub-site proof on W2's catalog B. usage: g10-after-w2.sh [hubOrigin]
# Runs against my own catalog-B hub (default :8030, `.🧬semio/🌐hub/s11-g10-hub-8030-b`); the durability gate owns a hub
# (restart law) on :8031 from the credentialed clone `.🧬semio/🌐hub/s11-g10-hub-src-b`. Captures go to `.🧬semio/🌐hub/s11-g10-logs`.
set -u
ROOT=/Users/ueli/Documents/semio; G=$ROOT/.tmp-ticket/wp-g10; HUBS="$ROOT/.🧬semio/🌐hub"; OUT="$HUBS/s11-g10-logs"; HUB="${1:-http://127.0.0.1:8030}"
mkdir -p "$OUT"; cd "$ROOT"
OS_MCP_HUB_ORIGIN=$HUB bun nx run @semio-tech/framework-os-mcp-rs:hub-agent-participant-check > "$OUT/hub-agent-participant.txt" 2>&1; echo "hub-agent-participant rc=$?"
bun $G/g10-quartet-live.ts $HUB > "$OUT/quartet-live.txt" 2>&1; echo "quartet rc=$?"
OS_MCP_HUB_BINARY="$HUBS/s11-g10-bin/os-hub" OS_MCP_HUB_DATA_DIR="$HUBS/s11-g10-hub-src-b" OS_MCP_HUB_PORT=8031 SEMIO_TEST_ARTIFACT_DIR="$OUT/durability" bun nx run @semio-tech/framework-os-mcp-rs:hub-edit-durability-check > "$OUT/hub-edit-durability.txt" 2>&1; echo "durability rc=$?"
