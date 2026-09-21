#!/usr/bin/env zsh
# 🚀️ Slice JC1 — republish the trusted catalog with jco 1.34.0 (the fixed async task.return binding)
# into a NEW data root, then hold the hub. Same shape as 📜️gm1-hub-boot.sh, with JC1's own port, data
# root, private CARGO_TARGET_DIR (preamble rule 25) and capture names (rule 5). The whole bootstrap
# runs inside the fleet wasm build mutex (rule 27): two wasm-release component builds, two `describe`
# runs and a jco codegen. `OS_HUB_CREDENTIAL_SIGN_IN=true` is set on the hold, without which
# `POST /auth/sessions` answers 403 and no browser can sign in (GM1 §0).
# The bootstrap builds its OWN `os-hub` binary (`TrustedStdioGisBootstrapScript.run`,
# 🌎️hub/📦️packages/🦀️rust/📜️script.ts:12631) — mandatory here, because the Rust twin of the codegen
# policy moved to `semio.os.browser-jco-1.34.0-jspi.v1` and any earlier binary refuses this catalog.
# Usage: 📜️jc1-hub-boot.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7621}
DATA="$ROOT/.🧬semio/🌐hub/jc1-boot"
LOG="$GEN/jc1-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-jc1"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== 1. trusted-stdio-gis-bootstrap OS_HUB_DATA=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" jc1 -- \
  zsh -c "cd '$ROOT/🌎️hub/📦️packages/🦀️rust' && OS_HUB_DATA='$DATA' bun ./📜️script.ts trusted-stdio-gis-bootstrap" >> "$LOG" 2>&1
rc=$?
echo "=== bootstrap exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== 2. hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
export OS_HUB_CREDENTIAL_SIGN_IN=true
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/jc1-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/jc1-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== 3. readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/jc1-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
