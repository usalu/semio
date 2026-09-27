#!/bin/zsh
# 🧪️ T13: wasm32-wasip2 check of the guest crates T13 changed, through the fleet wasm mutex (rule 26 build-dir).
cd /Users/ueli/Documents/semio || exit 1
export CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t13/target CARGO_INCREMENTAL=0
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm t13 -- zsh -c 'nice -n 10 cargo check --keep-going -p semio-s-artifact-wfc-2d -p semio-s-artifact-wfc-grid2d -p semio-s-artifact-wfc-bitmap -p semio-s-artifact-wfc-3d -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib --target wasm32-wasip2 --message-format short; echo "EXIT-WASM-WFC $? $(date +%T)"; nice -n 10 cargo check --keep-going -p semio-s-artifact-reasoning-wires -p semio-framework-surface --lib --target wasm32-wasip2 --message-format short; echo "EXIT-WASM-WIRES-SURFACE $? $(date +%T)"'
echo ALL_DONE
