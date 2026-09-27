#!/bin/zsh
# 🧪️ S17: the generation3d tool-run snapshot-retirement law, native, through the fleet `native` lane (build-fleet-b, nice 15).
# usage: zsh s17-gen3d-law.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s17/target CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
echo "START $(date '+%T')" > "$1"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native s17 -- nice -n 15 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-fail-fast -- a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias --nocapture >> "$1" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1"
