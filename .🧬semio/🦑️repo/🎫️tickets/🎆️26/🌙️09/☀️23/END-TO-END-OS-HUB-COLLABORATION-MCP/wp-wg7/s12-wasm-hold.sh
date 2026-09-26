#!/bin/zsh
# 🔐️ WG7 session-12 wasm-mutex hold: wasm32 `--lib` checks of the kernel (`sync`: link primitive, browser actor) and the renderer (browser
# component codec, link status), then — only when green — the
# renderer's release browser build the ticket serve mounts. Progress lines in `.🧬semio/🌐hub/s12-wg7-logs/wasm-hold.txt`.
# usage (detached): nohup zsh s12-wasm-hold.sh > …/wasm-hold.txt 2>&1 & disown
set -u
R=/Users/ueli/Documents/semio
cd $R || exit 1
export CARGO_INCREMENTAL=0 NX_DAEMON=false
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
echo "[wg7-hold] START $(date '+%F %T')"
nice -n 15 cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not" | head -40
rc=$pipestatus[1]
echo "[wg7-hold] KERNEL CHECK rc=$rc $(date '+%F %T')"
[ $rc -eq 0 ] || exit $rc
nice -n 15 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|warning: unused|Finished|could not" | head -60
rc=$pipestatus[1]
echo "[wg7-hold] CHECK rc=$rc $(date '+%F %T')"
[ $rc -eq 0 ] || exit $rc
nice -n 15 bun nx run @semio-tech/framework-renderer-wgpu:wasm-release --outputStyle=stream 2>&1 | tail -40
echo "[wg7-hold] RELEASE rc=$pipestatus[1] $(date '+%F %T')"
