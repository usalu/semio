#!/usr/bin/env zsh
# 🚀️ Slice TC3e — rebuild the three plugin components with the note VIEWER's bounded disposers
# compiled in, publish a three-package trusted catalog and hold a hub on 7651. Descends from
# 📜️tc3d-hub-boot.sh; the differences are the ORDERED mutex stamp (session 8's fixed queue order,
# 📜️mutex-ordered.sh), the note-first fail-fast work script, the fresh data root
# .🧬semio/🌐hub/tc3e-boot, and a preflight that also builds the native `describe` binary the
# in-hold codec gate needs.
# Usage: 📜️tc3e-hub-boot.sh [port] [plugin-list]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
PLUGINS=${2:-note,stdio,gis}
DATA="$ROOT/.🧬semio/🌐hub/tc3e-boot"
LOG="$GEN/tc3e-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3d"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA" "$GEN"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== tc3e hub boot port=$PORT plugins=$PLUGINS data=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
preflight=1
tries=0
while [ "$tries" -lt 30 ]; do
  ( cd "$ROOT" && cargo check -p semio-s-plugin-note --target wasm32-wasip2 ) > "$GEN/tc3e-preflight-guest.txt" 2>&1 \
    && ( cd "$ROOT" && cargo build -p semio-framework-plugin-describe ) > "$GEN/tc3e-preflight-host.txt" 2>&1 \
    && { preflight=0; break; }
  tries=$((tries + 1))
  echo "preflight $tries RED at $(date '+%H:%M:%S'): $(grep -m1 '^error' "$GEN/tc3e-preflight-guest.txt" "$GEN/tc3e-preflight-host.txt")" >> "$LOG"
  sleep 60
done
echo "=== preflight exit=$preflight after $tries retries at $(date -Iseconds) ===" >> "$LOG"
[ $preflight -eq 0 ] || exit 90

echo "=== queued on the ordered mutex as 20260922110200-tc3e at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️mutex-ordered.sh" 20260922110200 tc3e -- zsh "$TICKET/📜️tc3e-mutex-work.sh" "$PLUGINS" "$DATA" >> "$LOG" 2>&1
rc=$?
echo "=== mutex work exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
export OS_HUB_CREDENTIAL_SIGN_IN=true
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/tc3e-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/tc3e-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/tc3e-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
