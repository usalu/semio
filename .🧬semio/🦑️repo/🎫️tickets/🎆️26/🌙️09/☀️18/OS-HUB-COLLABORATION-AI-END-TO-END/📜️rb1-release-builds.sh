#!/bin/zsh
# 📦️ RB1 — the two native release builds nobody has ever run: os-hub (product server) and
# semio-os-mcp (the end-user MCP binary Claude Desktop/Code exec), plus the admin SPA the hub reads
# at startup. Sequential on purpose: one cargo at a time per the ticket preamble. Each step invokes
# its own project's canonical `📜️script.ts` verb directly rather than through the nx wrapper —
# preamble rule 16: the nx daemon stalls in HASH_TASKS under fleet load (measured here: 4m30s at 0 %
# CPU on `nx run os-hub-admin:build` with no further output).
set -u
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false

step() { echo; echo "===== $1 :: start $(date '+%F %T') ====="; }

step "os-hub-admin build (SPA, read by the hub at startup)"
S=$(date +%s)
( cd "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript" && bun ./📜️script.ts build )
echo "exit=$? seconds=$(( $(date +%s) - S ))"

step "os-hub build (cargo --release --bin os-hub -> dist/build)"
S=$(date +%s)
( cd "🌎️hub/📦️packages/🦀️rust" && bun ./📜️script.ts build )
echo "exit=$? seconds=$(( $(date +%s) - S ))"
ls -la "🌎️hub/📦️packages/🦀️rust/dist/build" 2>&1

step "os-mcp build-release (cargo --release --bin semio-os-mcp -> dist/build-release)"
S=$(date +%s)
( cd "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" && bun ./📜️script.ts build-release )
echo "exit=$? seconds=$(( $(date +%s) - S ))"
ls -la "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build-release" 2>&1

echo
echo "===== RB1 RELEASE CHAIN DONE $(date '+%F %T') ====="
