#!/bin/zsh
# WG8: wasm32 compile gates for the target-neutral kernel store + framework transport edits, one wasm hold.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
echo "QUEUED $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm wg8 -- zsh -c '
echo "HELD $(date "+%F %T")"
cargo check -p semio-framework -p semio-framework-os-kernel --lib --target wasm32-wasip2 --message-format=short 2>&1 | grep -E "^error|error\[|: error|Finished|warning: .semio-framework(-os-kernel)?. \(lib\)"
echo "WASIP2 rc=$pipestatus[1]"
cargo check -p semio-framework-os-kernel --lib --features sync --target wasm32-unknown-unknown --message-format=short 2>&1 | grep -E "^error|error\[|: error|Finished|warning: .semio-framework-os-kernel. \(lib\)"
echo "UNKNOWN rc=$pipestatus[1]"
'
echo "EXIT=$? $(date '+%F %T')"
