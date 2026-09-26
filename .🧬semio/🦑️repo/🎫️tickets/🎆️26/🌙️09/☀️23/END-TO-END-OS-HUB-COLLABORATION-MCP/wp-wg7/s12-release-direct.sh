#!/bin/zsh
# 🔐️ WG7 renderer wasm-release through the `wasmshort` lane, calling the package's own build script directly (the nx target's
# command) so the FULL output is kept. Log lines prefixed [wg7-release].
set -u
R=/Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 NX_DAEMON=false
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
M=($R/.tmp-ticket/*fleet-mutex.sh)
P="$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"
echo "[wg7-release] queued $(date '+%F %T')"
cd "$P" || exit 1
zsh $M[1] wasmshort wg7 -- nice -n 15 bun ../../🏗️compiler/🌐️wasm/📜️script.ts build release 2>&1
echo "[wg7-release] rc=$? $(date '+%F %T')"
