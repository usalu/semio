#!/bin/zsh
# 🧪️ P9 native cargo on the live tree through the fleet native lane (session-14 rule 3): build-fleet-b, CARGO_INCREMENTAL=0, private target, nice 15. p9-native-cargo.sh <capture-name> <cargo subcommand + args…>
cd /Users/ueli/Documents/semio || exit 2
capture="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-logs/$1.txt"; shift
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=268435456 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-p9/target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh .tmp-ticket/📜️fleet-mutex.sh native p9 -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' p9 "$@"; } > "$capture" 2>&1
