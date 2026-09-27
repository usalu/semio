#!/bin/zsh
# 🧪️ S19 item 1: re-verify S17's tool-run snapshot retirement (kernel + SDK + gen3d `--lib --tests`, gen3d law) and N1's norm
# test fixes on the current tree, native lane, build-fleet-b. usage: zsh s19-item1.sh <capture-prefix>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19/target CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
M=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
echo "CHECK START $(date '+%T')" > "$1-check.txt"
zsh $M native s19 -- nice -n 15 cargo check -p semio-framework-os-kernel -p semio-framework-plugin -p semio-s-artifact-procedural-generation3d --features semio-s-artifact-procedural-generation3d/component-app-assembly --lib --tests --keep-going --message-format short >> "$1-check.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1-check.txt"
echo "LAW START $(date '+%T')" > "$1-law.txt"
zsh $M native s19 -- nice -n 15 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-fail-fast -- a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias --nocapture >> "$1-law.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1-law.txt"
echo "NORM START $(date '+%T')" > "$1-norm.txt"
zsh $M native s19 -- nice -n 15 cargo test -p semio-s-plugin-norm --lib --test compliance_gate --no-fail-fast >> "$1-norm.txt" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1-norm.txt"
echo "ALL_DONE $(date '+%T')" >> "$1-norm.txt"
