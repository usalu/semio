#!/usr/bin/env zsh
# 🚀️ TC5 — rebuild ALL selectable s plugin packages into a trusted catalog and hold hub 7700.
set -u
ROOT=/Users/ueli/Documents/semio
WP="$ROOT/.tmp-ticket/wp-tc5"
GEN="$WP/generated"
MUTEX="$ROOT/.tmp-ticket/📜️fleet-mutex.sh"
# resolve hub script without typing emoji in callers that already have it wrong
HUB_SCRIPT=$(ls -d "$ROOT"/*hub/📦️packages/🦀️rust/📜️script.ts | head -1)
PORT=${1:-7700}
DATA="$ROOT/.🧬semio/🌐hub/tc5-boot"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$WP/target"
export SEMIO_BUILD_BUDGET_MS=172800000
export OS_HUB_DATA="$DATA"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA" "$GEN"
chmod 700 "$DATA"
LOG="$GEN/tc5-hub-boot.txt"
: > "$LOG"
stamp() { echo "=== $* at $(date -Iseconds) ===" | tee -a "$LOG"; }

stamp "prebuild os-hub via hub mutex"
zsh "$MUTEX" hub tc5 -- env CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$CARGO_TARGET_DIR" \
  cargo build -p semio-hub --bin os-hub > "$GEN/tc5-hub-build.txt" 2>&1
echo "hub-build exit=$?" | tee -a "$LOG"
ls -la "$BIN" | tee -a "$LOG"
[ -x "$BIN" ] || exit 91

stamp "trusted-catalog-bootstrap --packages all via wasm mutex (script=$HUB_SCRIPT)"
zsh "$MUTEX" wasm tc5 -- env \
  NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_INCREMENTAL=0 \
  CARGO_TARGET_DIR="$CARGO_TARGET_DIR" SEMIO_BUILD_BUDGET_MS="$SEMIO_BUILD_BUDGET_MS" \
  OS_HUB_DATA="$DATA" \
  bun "$HUB_SCRIPT" trusted-catalog-bootstrap --packages all \
  > "$GEN/tc5-bootstrap.txt" 2>&1
BOOT_RC=$?
echo "bootstrap exit=$BOOT_RC" | tee -a "$LOG"
tail -60 "$GEN/tc5-bootstrap.txt" | tee -a "$LOG"
[ "$BOOT_RC" -eq 0 ] || exit "$BOOT_RC"

stamp "hold hub on $PORT (nohup)"
nohup bun "$WP/tc5-hub-hold.ts" "$PORT" "$DATA" "$BIN" > "$GEN/tc5-hub-hold.txt" 2>&1 &
HOLD_PID=$!
disown "$HOLD_PID" 2>/dev/null || true
echo "$HOLD_PID" > "$GEN/tc5-hub-hold-pid.txt"
echo "hold_pid=$HOLD_PID" | tee -a "$LOG"

waited=0
code=000
while [ "$waited" -lt 3600 ]; do
  code=$(curl -s -m 5 -o "$GEN/tc5-readyz.json" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  sleep 15
  waited=$((waited + 15))
done
echo "readyz http=$code after ${waited}s" | tee -a "$LOG"
if [ "$code" = "200" ]; then
  python3 -c "import json; d=json.load(open('$GEN/tc5-readyz.json')); print('status', d.get('status')); print('artifactAuthority', d.get('artifactAuthority')); print('features', d.get('features'))" | tee -a "$LOG"
fi
# record catalog generation
python3 - <<PY | tee -a "$LOG"
import json, os
from pathlib import Path
root = Path("$DATA") / "trusted-catalog"
cur = root / "current.json"
print("current_exists", cur.exists())
if cur.exists():
  print(cur.read_text()[:800])
PY
stamp "boot script done"
exit 0
