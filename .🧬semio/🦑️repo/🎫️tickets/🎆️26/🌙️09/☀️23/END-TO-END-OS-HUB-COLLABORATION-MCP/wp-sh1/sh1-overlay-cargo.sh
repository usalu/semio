#!/bin/zsh
# 🧪️ SH1 overlay cargo runner: runs one cargo command inside the scratch overlay with private build/target dirs (never the shared build-dir), nice 10, incremental off.
cd "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-sh1-overlay" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-sh1-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-sh1-target"
echo "START $(date '+%H:%M:%S') cargo $*"
nice -n 10 cargo "$@"
rc=$?
echo "END rc=$rc $(date '+%H:%M:%S')"
exit $rc
