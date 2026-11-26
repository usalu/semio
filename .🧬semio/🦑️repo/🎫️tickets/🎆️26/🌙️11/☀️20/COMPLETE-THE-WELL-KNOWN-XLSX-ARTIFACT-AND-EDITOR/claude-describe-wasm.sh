#!/bin/zsh
# [DEBUG] builds the descriptor emitter natively and describes the built stdio, stdio-semio and stdio-bim components into 🗑️generated/claude-describe/out (read-only use of the shared target wasm)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-describe
W=$T/🗑️generated/claude-pane/native/target/wasm32-wasip2/wasm-prototype
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build CARGO_BUILD_JOBS=4 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cd /Users/ueli/Documents/semio || exit 1
cargo build -j 4 -p semio-plugin-describe || exit $?
for id in stdio stdio_bim stdio_semio; do
  src=$W/semio_hub_${id}_component.wasm
  [ -f $src ] || { echo "[DEBUG] missing $src (not built in the shared target)"; continue; }
  $N/target/debug/semio-plugin-describe $src --out $N/out/$id || echo "[DEBUG] describe $id failed $?"
done
