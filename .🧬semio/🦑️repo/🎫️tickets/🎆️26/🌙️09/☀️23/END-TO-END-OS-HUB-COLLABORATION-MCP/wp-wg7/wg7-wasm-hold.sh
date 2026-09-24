#!/bin/zsh
# 🔐️ WG7 one wasm-mutex hold: wasm32 checks of the kernel (sync) and the renderer, revert K8 on failure, then the renderer's release browser build.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
check() {
  cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not" | head -20
  cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not" | head -20
}
date +%H:%M:%S
out=$(check); echo "$out"
if echo "$out" | /usr/bin/grep -q "error"; then
  echo "K8 wasm32 check failed: reverting"; python3 .tmp-ticket/wp-wg7/k8-frame-ceiling.py revert; check
fi
date +%H:%M:%S
bun nx run @semio-tech/framework-renderer-wgpu:wasm-release --outputStyle=stream 2>&1 | tail -40
echo "EXIT=$pipestatus[1]"; date +%H:%M:%S
