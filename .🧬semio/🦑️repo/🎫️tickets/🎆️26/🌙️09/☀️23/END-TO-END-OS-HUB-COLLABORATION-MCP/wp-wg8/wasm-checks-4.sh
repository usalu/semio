#!/bin/zsh
# WG8 session 12: kernel `sync` wasm32-unknown-unknown gate (the 02:17 run lost a proc-macro dylib to a concurrent build-dir rewrite) + renderer again.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
echo "QUEUED $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm wg8 -- zsh -c '
echo "HELD $(date "+%F %T")"
nice -n 15 cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --message-format=short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|Finished|warning: .semio-framework-os-kernel. \(lib\)"
echo "KERNEL-UNKNOWN rc=$pipestatus[1]"
nice -n 15 cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown --message-format=short 2>&1 | /usr/bin/grep -E "^error|error\[|: error|Finished|warning: .semio-framework-os-renderer-wgpu. \(lib\)"
echo "RENDERER-UNKNOWN rc=$pipestatus[1]"
'
echo "EXIT=$? $(date '+%F %T')"
