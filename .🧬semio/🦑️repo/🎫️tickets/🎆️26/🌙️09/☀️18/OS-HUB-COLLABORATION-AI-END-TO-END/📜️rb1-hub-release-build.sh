#!/bin/zsh
# 📦️ RB1 §1 — the os-hub release build alone. Split out of 📜️rb1-release-builds.sh after its first
# run died at 95 min on a peer's mid-refactor `LocalizedLabel` (E0308/E0599 in semio-framework-plugin,
# resolved in the tree by 10:30) while the MCP release build in the same chain succeeded.
set -u
cd /Users/ueli/Documents/semio || exit 1
export CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false
echo "===== os-hub build (cargo --release --bin os-hub -> dist/build) :: start $(date '+%F %T') ====="
S=$(date +%s)
( cd "🌎️hub/📦️packages/🦀️rust" && bun ./📜️script.ts build )
echo "exit=$? seconds=$(( $(date +%s) - S )) at $(date '+%F %T')"
ls -la "🌎️hub/📦️packages/🦀️rust/dist/build" 2>&1
echo "===== RB1 HUB RELEASE DONE $(date '+%F %T') ====="
