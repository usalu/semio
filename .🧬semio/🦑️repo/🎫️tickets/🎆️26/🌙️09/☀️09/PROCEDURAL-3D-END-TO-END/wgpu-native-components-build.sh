#!/bin/zsh
# 🧩️ Native (graph-free, lease-free) wasm32-wasip2 build of the generation3d component closure: flow + procedural (hub workspace) and the nine
# isolated flow-extension workspaces, all into one shared target dir. Output: .cache/g3d-native/tp2/wasm32-wasip2/debug/<wasmOut>.
cd /Users/ueli/Documents/semio || exit 1
export PATH="$HOME/.cargo/bin:$PATH" CARGO_HOME=/Users/ueli/Documents/semio/.cache/b/r/m/cargo/home CARGO_PROFILE_DEV_DEBUG=false CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_DEV_STRIP=symbols
TGT=/Users/ueli/Documents/semio/.cache/g3d-native/tp2
mkdir -p "$TGT"
CFG=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' --config "build.target-dir=\"$TGT\"" --config "build.build-dir=\"$TGT/b\"")
build() { local dir="$1"; shift; (cd "$dir" && cargo build "$@" --target wasm32-wasip2 -j "${JOBS:-2}" --offline "${CFG[@]}") || { echo "CARGO_EXIT=$? in $dir"; return 1; }; }
build 🌎️hub -p semio-hub-procedural-component -p semio-hub-flow-component || exit 1
for e in 🏗️bim 📐️brep 📖️dictionary 🖍️draw 📃️list 🧠️logic 🧮️math 🔤️primitive 📝️text; do
  build "✏️s/🔌️plugins/🌊️flow/🧩️extensions/$e/🧩️component/📦️packages/🦀️rust" || exit 1
done
ls -la "$TGT"/wasm32-wasip2/debug/*.wasm; echo BUILD_DONE
