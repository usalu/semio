#!/bin/zsh
# 🔐️ WG9 wasm-mutex hold 1 (≤ 25 min): kernel wasm32 `--lib` checks for the landed link-expiry + echo-suppression sets — browser
# (`wasm32-unknown-unknown --features sync`: the browser document actor) and guest (`wasm32-wasip2`). Lines prefixed [wg9-wasm].
# usage: setopt no_bg_nice; nohup zsh wg9-wasm-kernel.sh > <log> 2>&1 & disown
set -u
R=/Users/ueli/Documents/semio
cd $R || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR
M=($R/.tmp-ticket/*fleet-mutex.sh)
echo "[wg9-wasm] queued $(date '+%F %T')"
zsh $M[1] wasm wg9 -- zsh -c '
echo "[wg9-wasm] HELD $(date "+%F %T")"
nice -n 10 cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not|🔄️sync/🦀️.rs"
echo "[wg9-wasm] KERNEL-BROWSER rc=${pipestatus[1]} $(date "+%F %T")"
nice -n 10 cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2 --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|could not|🔄️sync/🦀️.rs"
echo "[wg9-wasm] KERNEL-WASIP2 rc=${pipestatus[1]} $(date "+%F %T")"
'
echo "[wg9-wasm] END rc=$? $(date '+%F %T')"
