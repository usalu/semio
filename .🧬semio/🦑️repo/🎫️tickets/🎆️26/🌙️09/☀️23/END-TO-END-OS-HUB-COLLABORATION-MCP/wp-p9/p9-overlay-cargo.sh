#!/bin/zsh
# 🧪️ P9 overlay cargo through the fleet overlay lane (session-14 rule 3): one cargo command inside the P9 overlay with PRIVATE build/target dirs (never the shared build-dir), nice 15, incremental off. p9-overlay-cargo.sh <capture-name> <cargo subcommand + args…>
capture="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-logs/$1.txt"; shift
cd "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-overlay" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=268435456
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay p9 -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' p9 "$@"; } > "$capture" 2>&1
