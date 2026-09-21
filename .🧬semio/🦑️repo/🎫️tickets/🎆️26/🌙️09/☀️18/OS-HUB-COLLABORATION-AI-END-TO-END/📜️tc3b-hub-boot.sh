#!/usr/bin/env zsh
# 🚀️ Slice TC3b — bootstrap a FRESH trusted catalog from the catalog-carried-genesis tree into a new
# data root and hold the hub on 7651. Same shape as 📜️jc1-hub-boot.sh with TC3b's port, data root,
# private CARGO_TARGET_DIR (preamble rule 25) and capture names (rule 5). The whole bootstrap runs
# inside the fleet wasm build mutex (rule 27).
#
# ⚠️ PRECONDITION that jc1's run did not have: this tree changes `world actor` (a new `codec`
# interface) AND the owned Semio actor ABI (nine core exports → thirteen). Every component built
# before that change is refused by `OwnedSemioArtifact::from_component`, so the components this
# bootstrap builds must come from THIS tree — which the `trusted-stdio-gis-bootstrap` verb does by
# construction (it builds stdio and gis fresh). Any OTHER plugin named in the N-plugin list must be
# rebuilt cold first. `OS_HUB_CREDENTIAL_SIGN_IN=true` is set on the hold, without which
# `POST /auth/sessions` answers 403 and no browser can sign in (GM1 §0).
#
# Usage: 📜️tc3b-hub-boot.sh [port] [plugin-list]
#   plugin-list defaults to `stdio,gis,note` and is passed to the bootstrap verb as
#   OS_HUB_TRUSTED_PLUGINS; a verb that does not yet read it simply bootstraps stdio+gis.
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
PLUGINS=${2:-stdio,gis,note}
DATA="$ROOT/.🧬semio/🌐hub/tc3b-boot"
LOG="$GEN/tc3b-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3b"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== 1. trusted-stdio-gis-bootstrap plugins=$PLUGINS OS_HUB_DATA=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️wasm-build-mutex.sh" tc3b -- \
  zsh -c "cd '$ROOT/🌎️hub/📦️packages/🦀️rust' && OS_HUB_DATA='$DATA' OS_HUB_TRUSTED_PLUGINS='$PLUGINS' bun ./📜️script.ts trusted-stdio-gis-bootstrap" >> "$LOG" 2>&1
rc=$?
echo "=== bootstrap exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== 2. hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
export OS_HUB_CREDENTIAL_SIGN_IN=true
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/tc3b-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/tc3b-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== 3. readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/tc3b-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
