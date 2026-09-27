#!/bin/zsh
# 🔬️ H12: release build of the std-only owned-interpreter probe (`wp-h12/owned-probe`) through the native mutex; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-h12/owned-probe
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h12 -- nice -n 15 cargo build --release >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
