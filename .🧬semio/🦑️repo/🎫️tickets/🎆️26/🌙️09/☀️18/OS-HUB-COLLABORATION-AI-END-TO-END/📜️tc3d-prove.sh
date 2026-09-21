#!/usr/bin/env zsh
# 🧾️ Slice TC3d — the whole proof, run the moment 📜️tc3d-hub-boot.sh's hub answers on 7651.
# It is a SEPARATE detached process on purpose: the boot driver's last act is `wait` on the hub hold,
# so it can never run the probes itself, and a worker session that ends before the mutex is granted
# would otherwise leave a live hub with nothing measured against it.
#
# Three things, in order:
#   1. the rebuilt-component laws — the A/B that closes ticket 26/09/18 slice TC3c §5f. They are RED
#      against the component built before the `codec` resolver fix and must be GREEN against the one
#      stage 1 of the hold rebuilds, and `plugin_wasm` picks the newest build across every target*.
#   2. `📜️tc3d-provision.sh` — mint the two humans, then create-and-attach once per kind.
#   3. a readyz re-read, so the capture ends with the hub still up.
# Usage: 📜️tc3d-prove.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
OUT="$GEN/tc3d-prove.txt"
: > "$OUT"
echo "=== tc3d prove watcher started $(date -Iseconds) port=$PORT ===" >> "$OUT"

waited=0
code=000
while [ "$waited" -lt 36000 ]; do
  code=$(curl -s -m 5 -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/readyz" 2>/dev/null || echo 000)
  [ "$code" = "200" ] && break
  sleep 30
  waited=$((waited + 30))
done
echo "=== readyz http=$code after ${waited}s at $(date -Iseconds) ===" >> "$OUT"
[ "$code" = "200" ] || exit 1

echo "=== 1. rebuilt-component codec laws at $(date -Iseconds) ===" >> "$OUT"
( cd "$ROOT" && CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3d" \
  cargo test -p semio-framework-plugin-host --lib codec -- --test-threads=1 ) > "$GEN/tc3d-codec-laws.txt" 2>&1
echo "laws exit=$? $(grep -m1 'test result:' "$GEN/tc3d-codec-laws.txt")" >> "$OUT"

echo "=== 2. provision + create-and-attach at $(date -Iseconds) ===" >> "$OUT"
zsh "$TICKET/📜️tc3d-provision.sh" "$PORT" >> "$OUT" 2>&1
echo "provision exit=$?" >> "$OUT"

echo "=== 3. hub still up at $(date -Iseconds) ===" >> "$OUT"
curl -s -m 5 -o /dev/null -w "readyz=%{http_code}\n" "http://127.0.0.1:$PORT/readyz" >> "$OUT" 2>&1
