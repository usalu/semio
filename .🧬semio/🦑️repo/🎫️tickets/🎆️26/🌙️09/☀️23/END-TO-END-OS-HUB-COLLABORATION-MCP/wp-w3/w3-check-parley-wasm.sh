#!/bin/zsh
# 🧪 wasm32 checks for the parley no_std change: layout guest (wasip2), ui_render + ui wgpu-engine (browser target).
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
out=.tmp-ticket/wp-w3/generated
cargo check -p semio-s-plugin-layout --lib --target wasm32-wasip2 > $out/check-layout-wasip2.txt 2>&1; echo "layout-wasip2 rc=$?"
cargo check -p semio-framework-ui-render --lib --target wasm32-unknown-unknown > $out/check-ui-render-wasm.txt 2>&1; echo "ui-render-wasm32uu rc=$?"
cargo check -p semio-framework-ui --lib --features wgpu-engine --target wasm32-unknown-unknown > $out/check-ui-wasm.txt 2>&1; echo "ui-wasm32uu rc=$?"
