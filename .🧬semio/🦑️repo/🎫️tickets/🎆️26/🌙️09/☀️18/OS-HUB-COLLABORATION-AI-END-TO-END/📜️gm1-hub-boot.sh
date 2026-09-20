#!/usr/bin/env zsh
# 🚀️ Slice GM1 — TC1 §0's resume, re-run now that the mandatory gis cold-map proof is green.
# Identical to 📜️tc1-hub-boot.sh except for GM1's own capture names and private CARGO_TARGET_DIR
# (preamble rules 5 and 25). The whole bootstrap runs inside the fleet wasm build mutex (rule 27):
# it drives two `wasm-release` component builds, two `describe` runs and a jco codegen.
# Usage: 📜️gm1-hub-boot.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7611}
DATA="$ROOT/.🧬semio/🌐hub/gm1-boot"
LOG="$GEN/gm1-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-gm1"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== 1. trusted-stdio-gis-bootstrap OS_HUB_DATA=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" gm1 -- \
  zsh -c "cd '$ROOT/🌎️hub/📦️packages/🦀️rust' && OS_HUB_DATA='$DATA' bun ./📜️script.ts trusted-stdio-gis-bootstrap" >> "$LOG" 2>&1
rc=$?
echo "=== bootstrap exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== 2. hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/gm1-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/gm1-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== 3. readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/gm1-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
