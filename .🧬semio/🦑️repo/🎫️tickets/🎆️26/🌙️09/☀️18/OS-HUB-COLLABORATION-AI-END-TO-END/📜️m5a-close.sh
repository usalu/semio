#!/bin/sh
# 🏁️ M5a §8.7 closing sequence — the four steps, in order, one cargo at a time, each captured.
# Every step stops the script on failure so nothing later is reported against a stale binary.
# Run from the repo root:  sh .🧬semio/…/📜️m5a-close.sh
set -e
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🗑️generated"
cd "$ROOT"
export CARGO_PROFILE_WASM_DEV_DEBUG=false

echo "== 1/4 laws =="
cargo test -p semio-framework-os-mcp --lib m5a 2>&1 | tee "$OUT/m5a-s5-cargo-test.txt" | tail -40

echo "== 2/4 note descriptor (also compile-proves the 191-declaration sweep for note) =="
( cd "$ROOT/✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust" && bun ./📜️script.ts describe ) 2>&1 | tee "$OUT/m5a-s5-note-describe.txt" | tail -40
git --no-pager diff --stat -- "✏️s/🔌️plugins/🗒️note/🔣️.json"

echo "== 3/4 capability audit gate (expected RED until A3 regenerates the rest) =="
bun nx run @semio-tech/framework-os-mcp-rs:capability-audit-check 2>&1 | tee "$OUT/m5a-s5-capability-audit.txt" | tail -60 || true

echo "== 4/4 live agent loop — the 3 approval steps must stop skipping =="
curl -sf -m 8 -o /dev/null "http://127.0.0.1:6080/__semio/agent-bridge" || { echo "no live shell on :6080 — start one and rerun step 4"; exit 1; }
S_OS_MCP_LIVE_SHELL_URL=http://127.0.0.1:6080 S_OS_MCP_LIVE_PLUGIN=note \
  bun nx run @semio-tech/framework-os-mcp-rs:live-agent-loop-check 2>&1 | tee "$OUT/m5a-s5-live-agent-loop.txt" | tail -40
