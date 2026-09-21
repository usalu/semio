#!/usr/bin/env zsh
# 🚀️ Slice TC3c — the N-plugin trusted-catalog bootstrap. Copied from 📜️jc1-hub-boot.sh with TC3c's
# port (7651), data root (.🧬semio/🌐hub/tc3c-boot), private CARGO_TARGET_DIR (preamble rule 25) and
# capture names (rule 5), and with three stages jc1 did not have, all inside ONE fleet wasm mutex
# hold (rule 27) so the queue is paid for once:
#
#   0. `cargo check -p semio-s-plugin-note --target wasm32-wasip2 --features component-app-assembly`
#      — TC3b §6a's one unverified compile gate: the guest half of `world actor`'s new `codec`
#      interface is `#[cfg(all(target_arch = "wasm32", target_env = "p2"))]`, so NO native check ever
#      compiles it. Nothing downstream can work if this is red.
#   1. cold `wasm-release` cdylib builds of the three plugin crates. `produceFreshComponentV1` gives
#      every package its own private CARGO_TARGET_DIR inside the catalog build root, but `build-dir`
#      is shared repo-wide, so building here populates exactly the cache the bootstrap then reuses —
#      the 60–90 min cold pole is paid inside the hold with the lock doing real work.
#   2. the bootstrap verb itself (describe + jco + the hub binary + publication).
#
# The hub hold runs AFTER the mutex is released (the mutex must never be held by a long-lived
# server). `OS_HUB_CREDENTIAL_SIGN_IN=true` is set on the hold, without which `POST /auth/sessions`
# answers 403 and no client can sign in (GM1 §0).
#
# Usage: 📜️tc3c-hub-boot.sh [port] [plugin-list]
#   TC3C_WAIT_FOR_SOURCE=1 makes stage 2 wait for 🗑️generated/tc3c-source-ready.txt (at most 45 min)
#   before starting, so the hold can be queued while the builder rewrite is still being written.
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
PLUGINS=${2:-stdio,gis,note}
DATA="$ROOT/.🧬semio/🌐hub/tc3c-boot"
LOG="$GEN/tc3c-hub-dev.txt"
export NX_DAEMON=false CARGO_PROFILE_WASM_DEV_DEBUG=false
export CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3c"
BIN="$CARGO_TARGET_DIR/debug/os-hub"
mkdir -p "$DATA" "$GEN"
chmod 700 "$DATA"
unset SEMIO_BUILD_BUDGET_MS 2>/dev/null || true

: > "$LOG"
echo "=== tc3c hub boot port=$PORT plugins=$PLUGINS data=$DATA target=$CARGO_TARGET_DIR at $(date -Iseconds) ===" >> "$LOG"
# 🚦️ PRE-FLIGHT, outside the mutex. A peer's in-flight edit anywhere in the shared framework makes
# every plugin build red, and discovering that INSIDE the hold wastes the whole queue slot — which is
# exactly what happened at 13:40:30 (`semio-framework-ui-contract`: `UiNodeRecord` lost its `Clone`
# derive at 13:36 while `UiNodeTable` still derives it). So the tree is proven to compile natively
# first, with a cheap check of the crate every plugin depends on, and only then is the mutex queued.
# Peers land fixes continuously (preamble rule 3: never stop, never revert a peer's file), so this
# retries rather than failing.
preflight=1
tries=0
while [ "$tries" -lt 40 ]; do
  ( cd "$ROOT" && cargo check -p semio-framework-ui-contract ) > "$GEN/tc3c-preflight.txt" 2>&1 && { preflight=0; break; }
  tries=$((tries + 1))
  echo "preflight $tries RED at $(date '+%H:%M:%S'): $(grep -m1 '^error' "$GEN/tc3c-preflight.txt")" >> "$LOG"
  sleep 60
done
echo "=== preflight exit=$preflight after $tries retries at $(date -Iseconds) ===" >> "$LOG"
[ $preflight -eq 0 ] || exit 90

zsh "$TICKET/📜️wasm-build-mutex.sh" tc3c -- zsh "$TICKET/📜️tc3c-mutex-work.sh" "$PLUGINS" "$DATA" >> "$LOG" 2>&1
rc=$?
echo "=== mutex work exit $rc at $(date -Iseconds) ===" >> "$LOG"
echo "--- trusted catalog on disk ---" >> "$LOG"
find "$DATA/trusted-catalog" -maxdepth 2 -print >> "$LOG" 2>&1
[ $rc -eq 0 ] || exit $rc

echo "=== hold hub on port $PORT at $(date -Iseconds) ===" >> "$LOG"
export OS_HUB_CREDENTIAL_SIGN_IN=true
( cd "$ROOT" && bun "$TICKET/🐍️ds1-hub-hold.ts" "$PORT" "$DATA" "$BIN" ) >> "$LOG" 2>&1 &
HUB=$!
echo "$HUB" > "$GEN/tc3c-hub-pid.txt"
echo "hold pid=$HUB" >> "$LOG"
elapsed=0
code=000
while [ "$elapsed" -lt 900 ]; do
  code=$(curl -s -m 5 -o "$GEN/tc3c-hub-readyz.txt" -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  kill -0 "$HUB" 2>/dev/null || { echo "hold exited before readiness after ${elapsed}s" >> "$LOG"; break; }
  sleep 10
  elapsed=$((elapsed + 10))
done
echo "=== readyz http=$code after ${elapsed}s at $(date -Iseconds) ===" >> "$LOG"
cat "$GEN/tc3c-hub-readyz.txt" >> "$LOG" 2>&1
echo "" >> "$LOG"
wait "$HUB"
