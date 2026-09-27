#!/bin/zsh
# 🎒️ R10 item 2: standalone test run of the first-party zip container (unchanged deflate sources + staged zip module),
# private target/build dirs, run only through the fleet overlay lane.
cd "/Users/ueli/Documents/semio/.tmp-ticket/wp-r10/zipcheck" || exit 2
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-r10/zipcheck/target" CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-r10/zipcheck/build"
echo "START $(date '+%H:%M:%S')"
nice -n 15 cargo test --offline --no-fail-fast 2>&1
echo "END rc=$? $(date '+%H:%M:%S')"
