#!/bin/zsh
# 🧊️ R9 session 13: wasm32 checks of the guest/browser crates R9 touched (plugin SDK shim removal, kernel directory client
# sign-out command, ui-contract test move, demonstrator/procedural extension exports, wgpu renderer door + sign-in path).
# Run through the fleet wasm mutex: `zsh .tmp-ticket/📜️fleet-mutex.sh wasm r9 -- zsh .tmp-ticket/wp-r9/r9-wasm-check.sh <n>`.
cd /Users/ueli/Documents/semio || exit 2
out="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-r9-captures/wasm-check-${1:-1}.txt"
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-r9/target
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
echo "start=$(date '+%F %T')" > "$out"
s=$(date +%s)
nice -n 10 cargo check --keep-going --lib --message-format=short --target wasm32-wasip2 -p semio-framework-plugin --features semio-framework-plugin/component-guest,semio-framework-plugin/component-extension-guest -p semio-framework-os-kernel -p semio-framework-ui-contract -p semio-s-plugin-demonstrator -p semio-s-plugin-playbook-procedural >> "$out" 2>&1
echo "WASIP2 EXIT=$? secs=$(( $(date +%s) - s ))" >> "$out"
s=$(date +%s)
nice -n 10 cargo check --keep-going --lib --message-format=short --target wasm32-unknown-unknown -p semio-framework-os-kernel -p semio-framework-os-renderer-wgpu >> "$out" 2>&1
echo "UNKNOWN EXIT=$? secs=$(( $(date +%s) - s )) end=$(date '+%F %T')" >> "$out"
