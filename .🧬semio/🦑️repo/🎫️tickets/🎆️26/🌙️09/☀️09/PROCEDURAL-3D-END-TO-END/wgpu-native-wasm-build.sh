#!/bin/zsh
# 🦀️ Wasm build of the wgpu renderer through the repo binary (host memory guard and capacity gate), then wasm-bindgen into the serve's wasm-dev directory.
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.cache/g3d-native/wasm-dev
export CARGO_INCREMENTAL=0
MARK=$(mktemp)
mkdir -p "$OUT"
while pgrep -f "repo run .*:component" >/dev/null; do sleep 20; done
repo cargo --store=shared --configuration=prototype --parallelism=2 -- build -p semio-renderer-wgpu-component --target wasm32-unknown-unknown || { echo "CARGO_EXIT=$?"; exit 1; }
WASM=$(find /Users/ueli/Documents/semio/.cache/b/r/m -name semio_renderer_wgpu_component.wasm -newer "$MARK" -not -path '*/deps/*' 2>/dev/null | head -1)
[ -n "$WASM" ] || WASM=$(find /Users/ueli/Documents/semio/.cache/b/r/m -name semio_renderer_wgpu_component.wasm -not -path '*/deps/*' -print0 2>/dev/null | xargs -0 ls -t | head -1)
echo "WASM=$WASM"
/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home/bin/wasm-bindgen "$WASM" --target web --no-typescript --out-name semio-renderer-wgpu-component --out-dir "$OUT" || exit 1
ls -la "$OUT"; echo BUILD_DONE
