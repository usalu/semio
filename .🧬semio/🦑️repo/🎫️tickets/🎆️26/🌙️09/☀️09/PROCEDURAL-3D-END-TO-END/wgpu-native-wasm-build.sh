#!/bin/zsh
# 🦀️ Native (graph-free) wasm build of the wgpu renderer: cargo -> wasm-bindgen, dev profile, same artifacts
# the `…:wasm` target publishes into the products store. Output: $OUT (see serve-generation3d-wgpu-native.sh).
cd /Users/ueli/Documents/semio || exit 1
TF="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
export PATH="$HOME/.cargo/bin:$PATH" CARGO_HOME=/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home CARGO_PROFILE_DEV_DEBUG=false CARGO_PROFILE_DEV_OPT_LEVEL=1
TGT=/Users/ueli/Documents/semio/.cache/g3d-native/t
OUT=/Users/ueli/Documents/semio/.cache/g3d-native/wasm-dev
mkdir -p "$TGT" "$OUT"
cargo build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown -j "${JOBS:-2}" --offline \
  --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' \
  --config "build.target-dir=\"$TGT\"" --config "build.build-dir=\"$TGT/b\"" || { echo "CARGO_EXIT=$?"; exit 1; }
WASM="$TGT/wasm32-unknown-unknown/debug/semio_renderer_wgpu_component.wasm"
/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home/bin/wasm-bindgen "$WASM" --target web --no-typescript --out-name semio-renderer-wgpu-component --out-dir "$OUT" || exit 1
ls -la "$OUT"; echo BUILD_DONE
