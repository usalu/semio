#!/usr/bin/env zsh
# 🚀️ Slice TC1 — the trusted stdio+GIS catalog, published, and a hub whose artifactAuthority is ready.
# Same two steps as DS1's 📜️ds1-hub-boot.sh, with three differences:
#   - the WHOLE bootstrap runs inside the fleet wasm build mutex (preamble rule 27): it drives two
#     `wasm-release` component builds, a `describe` and a jco codegen;
#   - its own private CARGO_TARGET_DIR (rule 25) so DS1's uplift dir is untouched;
#   - the hub binary is the one `trusted-stdio-gis-bootstrap` builds itself, because TC1 changed a Rust
#     constant the hub links (DOCUMENT_BROWSER_ACTOR_INTERFACES) — a stale binary would refuse the
#     candidate's 17-interface actor record and never reach artifactAuthority.ready.
# Usage: 📜️tc1-hub-boot.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7611}
DATA="$ROOT/.🧬semio/🌐hub/tc1-boot"
LOG="$GEN/tc1-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc1"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== 1. trusted-stdio-gis-bootstrap OS_HUB_DATA=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" tc1 -- \
  zsh -c "cd '$ROOT/🌎️hub/📦️packages/🦀️rust' && OS_HUB_DATA='$DATA' bun ./📜️script.ts trusted-stdio-gis-bootstrap" >> "$LOG" 2>&1
rc=$?
echo "=== bootstrap exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== 2. hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/tc1-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/tc1-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== 3. readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/tc1-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
