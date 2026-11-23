#!/bin/zsh
# 🧊️ Graph-free detached serve of the procedural generation3d WGPU playground on 6118 (no `repo run`, no project-graph read).
# Start:  screen -dmS g3dwgpu "<ticket>/serve-generation3d-wgpu-native.sh"   then   curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:6118/?plugin=generation3d
# Needs (built by wgpu-native-wasm-build.sh, wgpu-native-generate.ts, wgpu-native-components.sh, wgpu-native-activate.sh): renderer wasm in
# .cache/g3d-native/wasm-dev, generated boot/worker bundles, the procedural plugin module and its activation receipt.
TF="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
mkdir -p "$TF/🗑️generated"; exec > "$TF/🗑️generated/wgpu-native-serve.txt" 2>&1
cd /Users/ueli/Documents/semio || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
export SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio SEMIO_PLUGIN=generation3d SEMIO_RENDERER=wgpu SEMIO_BUILD_MODE=prototype VITE_SEMIO_PLUGIN=generation3d VITE_SEMIO_RENDERER=wgpu S_OS_PORT=6118 SEMIO_VITE_HMR=0 S_LOCAL_ONLY=1 S_G3D_WASM=/Users/ueli/Documents/semio/.cache/g3d-native/wasm-dev
date
"$BUN" "$TF/native-launch.ts" . "$BUN" "$TF/serve-generation3d-wgpu-native.ts"
echo "SERVE_EXIT=$?"; date
