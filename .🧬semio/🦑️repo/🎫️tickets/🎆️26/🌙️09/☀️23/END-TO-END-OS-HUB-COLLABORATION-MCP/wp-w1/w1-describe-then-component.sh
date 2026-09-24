#!/bin/zsh
# 🔬️ W1: describe (now the component-dev unit) then component-dev; the described bytes must equal the staged dist bytes.
cd /Users/ueli/Documents/semio || exit 1
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
export CARGO_INCREMENTAL=0
T=".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/semio_s_plugin_$1.wasm"
D="✏️s/🔌️plugins/$2/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_$1.wasm"
s=$(date +%s); bun nx run "@semio-tech/$1-plugin:describe" --outputStyle=stream 2>&1 | grep -E 'described|Finished|Compiling semio-s-plugin|error' ; echo "describe rc=$pipestatus[1] wall=$(( $(date +%s)-s ))s"
echo "json wasmSha256 $(python3 -c "import json;print(json.load(open('✏️s/🔌️plugins/$2/🔣️.json'))['hashes']['wasmSha256'])")"
echo "target $(shasum -a 256 "$T" | cut -c1-64)"
s=$(date +%s); bun nx run "@semio-tech/$1-plugin:component-dev" --outputStyle=stream 2>&1 | grep -E 'Finished|Compiling semio-s-plugin|staged|cache' ; echo "component rc=$pipestatus[1] wall=$(( $(date +%s)-s ))s"
echo "dist   $(shasum -a 256 "$D" | cut -c1-64)"
