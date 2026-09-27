#!/bin/zsh
# 🧪️ S17 final landing check of the VersionPin set's demonstrator hunk (`tree_pin!()` pins), build-landing (coordinator
# 06:1x: final landing checks until 06:55 may use it), nice 10. usage: zsh s17-check-demonstrator.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing
echo "START $(date '+%T')" > "$1"
nice -n 10 cargo check -p semio-s-plugin-demonstrator --lib --message-format short >> "$1" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1"
