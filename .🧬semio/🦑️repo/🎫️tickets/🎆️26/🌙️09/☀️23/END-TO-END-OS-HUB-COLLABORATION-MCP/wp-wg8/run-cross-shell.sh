#!/bin/zsh
# WG8 session 12: the cross-shell journey — native wgpu user A (Rust law) + React `s` user B (browser driver) on one hub document.
# Usage: [CROSS_MODE=edits|cursors] zsh run-cross-shell.sh <tag> [hubOrigin] [reactUrl]   (launch detached: setopt no_bg_nice; nohup zsh run-cross-shell.sh … & disown)
# edits = block2d co-editing law, cursors = puzzle2d board-cursor law.
TAG="$1"; HUB="${2:-http://127.0.0.1:7800}"; REACT="${3:-http://127.0.0.1:6590/}"
export CROSS_MODE="${CROSS_MODE:-edits}"
LAW=a_native_and_a_react_user_collaborate_on_one_hub_document
[ "$CROSS_MODE" = cursors ] && LAW=a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board
ROOT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-wg8-captures/cross-shell-$TAG"
rm -rf "$ROOT"; mkdir -p "$ROOT/handshake" "$ROOT/react"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-wg8/target
export SEMIO_PLUGIN=block2d
export SEMIO_PLUGIN_MODULES="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d"
export SEMIO_HUB_LIVE_ORIGIN="$HUB" SEMIO_HUB_LIVE_EMAIL=user1@semio.dev SEMIO_HUB_LIVE_PASSWORD=gm1-local-dev-pass-1 SEMIO_HUB_LIVE_PEER_EMAIL=user2@semio.dev
export SEMIO_CROSS_SHELL_DIR="$ROOT/handshake" CROSS_B_EMAIL=user2@semio.dev CROSS_B_PASSWORD=gm1-local-dev-pass-2
echo "START $(date '+%F %T') tag=$TAG mode=$CROSS_MODE law=$LAW hub=$HUB react=$REACT pid=$$"
nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-run > "$ROOT/build.txt" 2>&1 || { echo "BUILD rc=$?"; exit 1; }
bun .tmp-ticket/wp-wg8/cross-shell.mjs "$REACT" "$ROOT/handshake" "$ROOT/react" > "$ROOT/react.txt" 2>&1 &
DRIVER=$!
echo "DRIVER pid=$DRIVER"
nice -n 15 cargo test -p semio-framework-os-renderer-wgpu --lib --no-fail-fast -- "shell::hub_projection_workspace_tests::$LAW" --exact --ignored --nocapture > "$ROOT/native.txt" 2>&1
echo "NATIVE rc=$? $(date '+%F %T')"
wait $DRIVER
echo "DRIVER rc=$? $(date '+%F %T')"
