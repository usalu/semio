#!/bin/zsh
# 🧪️ LB2 overlay cargo through the fleet overlay lane (session-14 rule 3): runs one cargo command inside the LB2 scratch overlay with PRIVATE build/target dirs (never the shared build-dir), nice 15, incremental off. overlay-cargo.sh <capture-name> <cargo subcommand + args…>
capture="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/generated/$1.txt"; shift
cd "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-overlay" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay lb2 -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' lb2 "$@"; } > "$capture" 2>&1
