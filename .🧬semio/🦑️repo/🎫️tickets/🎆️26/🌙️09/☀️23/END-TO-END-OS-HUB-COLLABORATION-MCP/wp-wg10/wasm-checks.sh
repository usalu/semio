#!/bin/zsh
# WG10 s13: wasm32 checks of every guest-linked / browser crate WG10's landing touches, inside the fleet wasm mutex.
# usage: setopt no_bg_nice; nohup zsh .tmp-ticket/📜️fleet-mutex.sh wasm wg10 -- zsh .tmp-ticket/wp-wg10/wasm-checks.sh > <capture> 2>&1 & disown
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0
echo "HELD $(date '+%F %T')"
nice -n 10 cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2 --keep-going --message-format=short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|Finished|warning: .semio-framework-os-kernel. \(lib\)"
echo "KERNEL-WASIP2 rc=$pipestatus[1] $(date '+%T')"
nice -n 10 cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --keep-going --message-format=short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|Finished|warning: .semio-framework-os-kernel. \(lib\)"
echo "KERNEL-SYNC-UNKNOWN rc=$pipestatus[1] $(date '+%T')"
nice -n 10 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --keep-going --message-format=short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|Finished|warning: .semio-framework-os-renderer-wgpu. \(lib\)"
echo "RENDERER-UNKNOWN rc=$pipestatus[1] $(date '+%T')"
