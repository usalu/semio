#!/bin/zsh
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
for spec in "semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown" "semio-framework-os-kernel --lib --target wasm32-wasip2" "semio-framework-plugin --lib --target wasm32-wasip2" "semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown"; do
  echo "=== $spec $(date +%T)"
  eval cargo check -p $spec 2>&1 | /usr/bin/grep -E "^error|Finished" -A7 | head -40
  echo "EXIT ${pipestatus[1]}"
done
echo "=== done $(date +%T)"
