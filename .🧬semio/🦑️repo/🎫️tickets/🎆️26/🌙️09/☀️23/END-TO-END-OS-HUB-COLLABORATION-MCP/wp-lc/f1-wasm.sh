#!/bin/zsh
# ⌨️ LC — F1 wasm32 fast gate (window 2): the two guests whose code changed (jack text-edit, vcs retained edit work) + the SDK.
cd /Users/ueli/Documents/semio || exit 2
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-lc/target
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
echo "START f1-wasm $(date '+%H:%M:%S')"
zsh .tmp-ticket/📜️fleet-mutex.sh wasm lc -- nice -n 15 cargo check -p semio-s-plugin-trinity -p semio-s-plugin-vcs --lib --target wasm32-wasip2 --message-format short > ".🧬semio/🌐hub/s13-lc-laws/f1-wasm-1.txt" 2>&1
echo "END f1-wasm rc=$? $(date '+%H:%M:%S')"
