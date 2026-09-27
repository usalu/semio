#!/bin/zsh
# 🧪️ S17 landing gates of the tool-run snapshot retirement set, sequential: (1) the generation3d law (native, `native` lane),
# (2) `cargo check --lib --tests` of the kernel (store) + SDK (native lane), (3) the wasm32 fast gate of kernel + SDK +
# procedural (wasm mutex). build-fleet-b, nice 15. usage: zsh s17-land-tool-run.sh <capture-prefix>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s17/target CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
M=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
echo "LAW START $(date '+%T')" > "$1-law.txt"
zsh $M native s17 -- nice -n 15 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-fail-fast -- a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias --nocapture >> "$1-law.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1-law.txt"
echo "CHECK START $(date '+%T')" > "$1-check.txt"
zsh $M native s17 -- nice -n 15 cargo check -p semio-framework-os-kernel -p semio-framework-plugin --lib --tests --keep-going --message-format short >> "$1-check.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1-check.txt"
echo "WASM START $(date '+%T')" > "$1-wasm.txt"
zsh $M wasm s17 -- nice -n 15 cargo check -p semio-framework-os-kernel -p semio-framework-plugin -p semio-s-plugin-procedural --lib --target wasm32-wasip2 --keep-going --message-format short >> "$1-wasm.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1-wasm.txt"
echo "ALL_DONE $(date '+%T')" >> "$1-wasm.txt"
