#!/bin/zsh
# 🔐️ WG7 short-lane renderer rebuild (preamble rule 19): hold 1 = renderer wasm32 `--lib` check; only when green, hold 2 = renderer
# `wasm-release`. Each is its own `wasmshort` hold, so a failing check releases the lane at once. Log lines prefixed [wg7-short].
set -u
R=/Users/ueli/Documents/semio
cd $R || exit 1
export CARGO_INCREMENTAL=0 NX_DAEMON=false
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg7-short] CHECK queued $(date '+%F %T')"
zsh $M[1] wasmshort wg7 -- nice -n 15 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not" | head -40
rc=$pipestatus[1]
echo "[wg7-short] CHECK rc=$rc $(date '+%F %T')"
[ $rc -eq 0 ] || exit $rc
echo "[wg7-short] RELEASE queued $(date '+%F %T')"
zsh $M[1] wasmshort wg7 -- nice -n 15 bun nx run @semio-tech/framework-renderer-wgpu:wasm-release --outputStyle=stream 2>&1 | tail -20
echo "[wg7-short] RELEASE rc=$pipestatus[1] $(date '+%F %T')"
