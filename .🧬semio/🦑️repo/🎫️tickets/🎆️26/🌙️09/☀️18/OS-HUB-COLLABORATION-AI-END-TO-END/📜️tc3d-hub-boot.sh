#!/usr/bin/env zsh
# 🚀️ Slice TC3d — rebuild the three plugin components with the `codec` guest-resolver fix compiled
# in, publish a three-package trusted catalog and hold a hub on 7651. Descends from
# 📜️tc3c-hub-boot.sh (same ports, same hold, same mutex discipline) with two differences:
#
#  * the PRE-FLIGHT is the note wasm32-wasip2 gate itself plus the describe emitter's native crate,
#    not a single shared framework crate. TC3c preflighted only `semio-framework-ui-contract` and
#    still entered the hold; TC3d measured the stdio `XmlDeclaration { quote }` refactor landing
#    half-finished at 17:05, which that narrower check would have missed. These two commands are
#    exactly what stages 1 and 2 need, so a green preflight is a hold that can actually finish.
#  * a private CARGO_TARGET_DIR (target-tc3d) and data root (.🧬semio/🌐hub/tc3d-boot).
#
# Usage: 📜️tc3d-hub-boot.sh [port] [plugin-list]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
PLUGINS=${2:-stdio,gis,note}
DATA="$ROOT/.🧬semio/🌐hub/tc3d-boot"
LOG="$GEN/tc3d-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3d"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA" "$GEN"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== tc3d hub boot port=$PORT plugins=$PLUGINS data=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
preflight=1
tries=0
while [ "$tries" -lt 90 ]; do
  ( cd "$ROOT" && cargo check -p semio-s-plugin-note --target wasm32-wasip2 ) > "$GEN/tc3d-preflight-guest.txt" 2>&1 \
    && ( cd "$ROOT" && cargo check -p semio-framework-plugin-describe ) > "$GEN/tc3d-preflight-host.txt" 2>&1 \
    && { preflight=0; break; }
  tries=$((tries + 1))
  echo "preflight $tries RED at $(date '+%H:%M:%S'): $(grep -m1 '^error' "$GEN/tc3d-preflight-guest.txt" "$GEN/tc3d-preflight-host.txt")" >> "$LOG"
  sleep 60
done
echo "=== preflight exit=$preflight after $tries retries at $(date -Iseconds) ===" >> "$LOG"
[ $preflight -eq 0 ] || exit 90

zsh "$TICKET/📜️tc3d-mutex.sh" tc3d -- zsh "$TICKET/📜️tc3d-mutex-work.sh" "$PLUGINS" "$DATA" >> "$LOG" 2>&1
rc=$?
echo "=== mutex work exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
export OS_HUB_CREDENTIAL_SIGN_IN=true
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/tc3d-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/tc3d-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/tc3d-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
