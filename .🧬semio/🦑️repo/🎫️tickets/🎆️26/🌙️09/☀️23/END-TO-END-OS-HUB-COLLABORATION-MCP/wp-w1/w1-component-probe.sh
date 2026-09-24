#!/bin/zsh
# 🔬️ W1: does `component-dev` (cargo rustc --crate-type cdylib) rewrite describe's shared wasm-dev bytes?
cd /Users/ueli/Documents/semio || exit 1
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
T=".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_$1.wasm"
D="✏️s/🔌️plugins/$2/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_$1.wasm"
echo "before target $(shasum -a 256 "$T" | cut -c1-64) dist $(shasum -a 256 "$D" 2>/dev/null | cut -c1-64)"
bun nx run "@semio-tech/$1-plugin:component-dev" --outputStyle=stream 2>&1 | tail -25
echo "rc=$pipestatus[1]"
echo "after  target $(shasum -a 256 "$T" | cut -c1-64) dist $(shasum -a 256 "$D" 2>/dev/null | cut -c1-64)"
