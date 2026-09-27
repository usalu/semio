#!/bin/zsh
# 🧪️ H14 overlay cargo runner: one cargo command inside the scratch overlay `.🧬semio/🌐hub/s14-h14-overlay` through the
# fleet's overlay lane, with a PRIVATE build-dir and target (never the shared build-dir), nice 15, incremental off.
# usage: h14-overlay-cargo.sh <capture> <cargo args…>
OUT=$1; shift
cd "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-overlay" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target"
echo "=== queued $(date +%T) cargo $*" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay h14 -- zsh -c 'echo "=== start $(date +%T)"; nice -n 15 cargo "$@"; echo "=== EXIT $? $(date +%T)"' h14 "$@" >> "$OUT" 2>&1
