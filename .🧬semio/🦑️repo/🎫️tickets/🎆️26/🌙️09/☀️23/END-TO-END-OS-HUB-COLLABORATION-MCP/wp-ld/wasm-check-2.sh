#!/bin/zsh
# 🧱️ LD item 2 wasm32 compile-atomic check (envelope observed/target wire change); run through the fleet wasm mutex.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing"
run() { echo "== $*"; cargo check "$@" --message-format short 2>&1 | /usr/bin/grep -E "^[^ ]*: error|^error|Finished|could not compile" ; echo "rc=${pipestatus[1]}"; }
run -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin -p semio-s-artifact-writer-writer -p semio-s-plugin-writer --lib --target wasm32-wasip2
run -p semio-framework-os-kernel --features sync --lib --target wasm32-unknown-unknown
run -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown
