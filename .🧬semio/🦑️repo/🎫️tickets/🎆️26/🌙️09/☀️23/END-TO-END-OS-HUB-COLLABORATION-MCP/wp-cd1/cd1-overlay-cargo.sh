#!/bin/zsh
# 🧪️ CD1 overlay cargo (native lane, as the coordinator directed): runs one cargo command inside the CD1 APFS-clone overlay with PRIVATE build/target dirs under the CD1 hub dir (never the shared build-dir). cd1-overlay-cargo.sh <capture-name> <cargo args…>
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"
capture="$H/s14-cd1-work/$1.txt"; shift
cd "$H/s14-cd1-overlay" || exit 2
export CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$H/s14-cd1-build" CARGO_TARGET_DIR="$H/s14-cd1-target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native cd1 -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' cd1 "$@"; } > "$capture" 2>&1
