#!/bin/zsh
# WG10 s13: runs ONE native live law of the renderer test binary (durable copy, no cargo) against an explicit hub origin.
# usage: zsh run-live-law.sh <hubOrigin> <law path> [extra env assignments…]   (launch detached; capture = stdout)
#   laws: shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document (hub-live-collaboration-check)
HUB="$1"; LAW="$2"; shift 2
[ -n "$HUB" ] && [ -n "$LAW" ] || { echo "usage: run-live-law.sh <hubOrigin> <law>"; exit 2; }
cd /Users/ueli/Documents/semio || exit 1
export SEMIO_PLUGIN=block2d
export SEMIO_PLUGIN_MODULES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d"
export SEMIO_EXECUTION_TARGET_STORE_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-wg10-execution-targets"
export SEMIO_HUB_LIVE_ORIGIN="$HUB"
export SEMIO_HUB_LIVE_EMAIL=user1@semio.dev SEMIO_HUB_LIVE_PASSWORD=gm1-local-dev-pass-1
export SEMIO_HUB_LIVE_PEER_EMAIL=user2@semio.dev SEMIO_HUB_LIVE_PEER_PASSWORD=gm1-local-dev-pass-2
for assignment in "$@"; do export "$assignment"; done
echo "START $(date '+%F %T') hub=$HUB law=$LAW pid=$$"
nice -n 10 "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-wg10-bin/renderer-tests" "$LAW" --exact --ignored --nocapture --test-threads=1
echo "EXIT=$? $(date '+%F %T')"
