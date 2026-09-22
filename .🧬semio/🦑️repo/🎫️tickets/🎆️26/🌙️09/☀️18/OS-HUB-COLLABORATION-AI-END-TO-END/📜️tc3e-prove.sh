#!/usr/bin/env zsh
# 🧾️ Slice TC3e — the whole proof, run the moment 📜️tc3e-hub-boot.sh's hub answers on 7651.
# A SEPARATE detached process on purpose: the boot driver's last act is `wait` on the hub hold, so it
# can never run the probes itself, and a worker session that ends before the mutex is granted would
# otherwise leave a live hub with nothing measured against it.
#   1. the rebuilt-component codec laws (TC3d §6a + TC3e's per-component sweep, §6b)
#   2. 📜️tc3e-provision.sh — mint the two humans, then create-and-attach once per kind
#   3. a readyz re-read, so the capture ends with the hub still up
# Usage: 📜️tc3e-prove.sh [port]
set -u
ROOT=/Users/ueli/Documents/semio
TICKET="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
GEN="$TICKET/🗑️generated"
PORT=${1:-7651}
OUT="$GEN/tc3e-prove.txt"
: > "$OUT"
echo "=== tc3e prove watcher started $(date -Iseconds) port=$PORT ===" >> "$OUT"

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
( cd "$ROOT" && CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-tc3e" \
  cargo test -p semio-framework-plugin-host --lib codec -- --test-threads=1 ) > "$GEN/tc3e-codec-laws.txt" 2>&1
echo "laws exit=$? $(grep -m1 'test result:' "$GEN/tc3e-codec-laws.txt")" >> "$OUT"

echo "=== 2. provision + create-and-attach at $(date -Iseconds) ===" >> "$OUT"
zsh "$TICKET/📜️tc3e-provision.sh" "$PORT" >> "$OUT" 2>&1
echo "provision exit=$?" >> "$OUT"

echo "=== 3. hub still up at $(date -Iseconds) ===" >> "$OUT"
curl -s -m 5 -o /dev/null -w "readyz=%{http_code}\n" "http://127.0.0.1:$PORT/readyz" >> "$OUT" 2>&1
