#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
echo "=== kernel wasm32 $(date +%T)"
cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2 2>&1 | /usr/bin/grep -E "^error|^warning: unused|Finished|-->" | head -40; echo "KERNEL_EXIT ${pipestatus[1]}"
echo "=== plugin wasm32 $(date +%T)"
cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 2>&1 | /usr/bin/grep -E "^error|Finished" -A6 | head -60; echo "PLUGIN_EXIT ${pipestatus[1]}"
echo "=== note guest wasm32 $(date +%T)"
cargo check -p semio-s-plugin-note --lib --target wasm32-wasip2 2>&1 | /usr/bin/grep -E "^error|Finished" -A6 | head -60; echo "NOTE_EXIT ${pipestatus[1]}"
echo "=== done $(date +%T)"
