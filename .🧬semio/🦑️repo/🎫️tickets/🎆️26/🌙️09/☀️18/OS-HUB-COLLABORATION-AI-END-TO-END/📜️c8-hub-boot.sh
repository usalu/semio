#!/usr/bin/env zsh
# 🌍️ Slice C8 — publish a trusted catalog that carries the CURRENT guest, then hold a hub on it.
#
# Why a new catalog at all: the browser's document actor is NOT the dev-serve component. It is the
# `packages/gis/browser/closed-actor.mjs` of the hub's published generation (measured: the probe
# decodes a 47 416 521-byte core, which is jc1-boot's 2026-09-21 05:33 generation). C7's guest fix
# (`ColdDocumentPairFrontier::validate` accepting a genesis frontier) therefore cannot reach a
# browser through an `activate <variant> react dev` run — only through a catalog republish.
#
# Why a new data root and a new port: hub 7621 runs `target-jc1/debug/os-hub` in place and is shared
# with a peer slice, so neither its binary (an in-place overwrite is a silent SIGKILL on macOS) nor
# its catalog may be touched. This publishes into `.🧬semio/🌐hub/c8-boot` and holds port 7671 with
# a binary in C8's own target dir, which also carries PR1's hub-side presence fixes.
#
# The publish runs inside the fleet wasm build mutex at C8's fixed queue position (preamble rule 27);
# the hold runs AFTER the mutex is released, so the queue is not blocked by a running hub.
# Usage: 📜️c8-hub-boot.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7671}
DATA="$ROOT/.🧬semio/🌐hub/c8-boot"
LOG="$GEN/c8-hub-boot.txt"
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-c8-hub"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== 1. trusted-catalog-bootstrap --packages stdio,gis OS_HUB_DATA=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
zsh "$TICKET/📜️mutex-ordered.sh" 20260922110000 c8 -- \
  zsh -c "cd '$ROOT/🌎️hub/📦️packages/🦀️rust' && OS_HUB_DATA='$DATA' bun ./📜️script.ts trusted-catalog-bootstrap --packages stdio,gis" >> "$LOG" 2>&1
rc=$?
echo "=== bootstrap exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== 2. hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
export OS_HUB_CREDENTIAL_SIGN_IN=true
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/c8-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/c8-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== 3. readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/c8-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
