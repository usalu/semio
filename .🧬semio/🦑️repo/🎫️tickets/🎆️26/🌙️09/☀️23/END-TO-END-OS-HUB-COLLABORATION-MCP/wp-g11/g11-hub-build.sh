#!/bin/zsh
# 🏗️ G11: current-tree os-hub into G11's private target, niced; then a signed copy under `.🧬semio/🌐hub/s13-g11-bin/<name>`.
# usage: zsh g11-hub-build.sh <copyName>
set -u
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-g11/target
B="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-g11-bin"; mkdir -p "$B"
date; S=$(date +%s)
nice -n 10 cargo build -p semio-hub --bin os-hub
RC=$?; echo "BUILD_EXIT=$RC secs=$(( $(date +%s)-S ))"
if [ $RC -eq 0 ]; then rm -f "$B/$1"; cp "$CARGO_TARGET_DIR/debug/os-hub" "$B/$1" && codesign -s - -f "$B/$1" && shasum -a 256 "$CARGO_TARGET_DIR/debug/os-hub" "$B/$1"; fi
date
