#!/bin/zsh
# 🧪️ WG11 s14 (from WG10's run-live-law.sh): runs ONE ignored native live law of a durable renderer test binary (no cargo)
# against an explicit hub. Credentials are the local test users of the hub recipe (never printed).
# usage: zsh wg11-live-law.sh <binary> <hubOrigin> <law path> [VAR=value…]   (launch detached; capture = stdout)
BIN="$1"; HUB="$2"; LAW="$3"; shift 3
[ -x "$BIN" ] && [ -n "$HUB" ] && [ -n "$LAW" ] || { echo "usage: wg11-live-law.sh <binary> <hubOrigin> <law>"; exit 2; }
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_PLUGIN=block2d
export SEMIO_PLUGIN_MODULES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d"
export SEMIO_EXECUTION_TARGET_STORE_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-wg11-execution-targets"
export SEMIO_HUB_LIVE_ORIGIN="$HUB"
export SEMIO_HUB_LIVE_EMAIL=user1@semio.dev SEMIO_HUB_LIVE_PASSWORD=gm1-local-dev-pass-1
export SEMIO_HUB_LIVE_PEER_EMAIL=user2@semio.dev SEMIO_HUB_LIVE_PEER_PASSWORD=gm1-local-dev-pass-2
for assignment in "$@"; do export "$assignment"; done
echo "START $(date '+%F %T') hub=$HUB law=$LAW RUST_MIN_STACK=${RUST_MIN_STACK:-default} pid=$$"
nice -n 10 "$BIN" "$LAW" --exact --ignored --nocapture --test-threads=1
echo "EXIT=$? $(date '+%F %T')"
