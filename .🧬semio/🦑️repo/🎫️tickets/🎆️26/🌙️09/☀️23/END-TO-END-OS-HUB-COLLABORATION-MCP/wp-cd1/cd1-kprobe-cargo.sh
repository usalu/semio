#!/bin/zsh
# 🧪️ CD1 kernel-probe cargo through the native lane: private build/target dirs inside the CD1 hub probe workspace (never the shared build-dir). cd1-kprobe-cargo.sh <capture-name> <cargo args…>
P="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-cd1-work/kprobe"
capture="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-cd1-work/$1.txt"; shift
cd "$P" || exit 2
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$P/build" CARGO_TARGET_DIR="$P/target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native cd1 -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' cd1 "$@"; } > "$capture" 2>&1
