#!/usr/bin/env zsh
# 🚀️ Slice DS1 — §9 gap 4. The trusted stdio+GIS catalog, published, and a hub that answers /readyz.
# Prerequisites already satisfied before this script runs (see §10.5/§10.9 of 📓️ds1-stdio-descriptor-bound.md):
#   - both `wasm-release` components are compiled into the SHARED build-dir (stdio 61 min, gis 62 min),
#     so the bootstrap's fresh per-run target dir only re-links/uplifts them;
#   - the `os-hub` binary is a rm+cp+codesign COPY of the coordinator's
#     (`⚡️cache/cargo/target-coordinator-hub/debug/os-hub`) — rule 26 reserves BUILDING it, not running
#     a copy, and an in-place overwrite would SIGKILL the peers executing the original.
# Steps: 1 materialize+publish the bundle, 2 hold a hub on that data root, 3 read /readyz and the
# document open-plan route. Usage: 📜️ds1-hub-boot.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7611}
DATA="$ROOT/.🧬semio/🌐hub/ds1-boot"
LOG="$GEN/ds1-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-ds1"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== binary $(ls -l "$BIN" | awk '{print $5, $9}') ===" >> "$LOG"
echo "=== 1. trusted-stdio-gis-bootstrap OS_HUB_DATA=$DATA at $(date -Iseconds) ===" >> "$LOG"
( cd "$ROOT/🌎️hub/📦️packages/🦀️rust" && OS_HUB_DATA="$DATA" bun ./📜️script.ts trusted-stdio-gis-bootstrap ) >> "$LOG" 2>&1
rc=$?
echo "=== bootstrap exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
find "$DATA/trusted-catalog" -name 'descriptor.semio' -exec ls -l {} \; >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== 2. hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/ds1-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/ds1-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== 3. readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/ds1-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
