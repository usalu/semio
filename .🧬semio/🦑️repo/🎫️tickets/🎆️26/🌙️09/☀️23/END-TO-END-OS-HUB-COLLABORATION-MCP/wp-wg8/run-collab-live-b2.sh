#!/bin/zsh
# WG8 session 12: hub-live-collaboration-check's law (two native wgpu shells, 12 steps) against hub 7800 on catalog B2.
# The native shells resolve block's component by the serving generation (hub execution-target routes, verified, stored).
# Usage: zsh run-collab-live-b2.sh <hubOrigin>   (no default: the hub origin is passed explicitly, rule 23)   (launch detached: setopt no_bg_nice; nohup zsh … & disown)
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg8/target
export SEMIO_PLUGIN=block2d
export SEMIO_PLUGIN_MODULES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d"
export SEMIO_EXECUTION_TARGET_STORE_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-wg8-execution-targets"
export SEMIO_HUB_LIVE_ORIGIN="$1"
[ -n "$SEMIO_HUB_LIVE_ORIGIN" ] || { echo "usage: run-collab-live-b2.sh <hubOrigin>"; exit 2; }
export SEMIO_HUB_LIVE_EMAIL=user1@semio.dev SEMIO_HUB_LIVE_PASSWORD=gm1-local-dev-pass-1
export SEMIO_HUB_LIVE_PEER_EMAIL=user2@semio.dev SEMIO_HUB_LIVE_PEER_PASSWORD=gm1-local-dev-pass-2
echo "START $(date '+%F %T') hub=$SEMIO_HUB_LIVE_ORIGIN pid=$$"
nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- shell::hub_projection_workspace_tests::two_live_wgpu_shells_collaborate_on_one_hub_document --exact --ignored --nocapture
echo "EXIT=$? $(date '+%F %T')"
