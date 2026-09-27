#!/bin/zsh
# 🔎️ H12: wasm32-wasip2 check of semio-framework-plugin + puzzle 2d/3d (component-app-assembly) in the landing build dir,
# through the fleet wasm mutex; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm h12 -- env CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target zsh -c 'echo "=== start $(date +%T)"; nice -n 10 cargo check --keep-going -p semio-framework-plugin -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-3d --lib --target wasm32-wasip2 --features semio-s-artifact-puzzle-2d/component-app-assembly,semio-s-artifact-puzzle-3d/component-app-assembly; echo "=== EXIT $? $(date +%T)"' >> "$OUT" 2>&1
