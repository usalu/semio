#!/bin/zsh
# 🧪 LB native check during the rebuild (rule 30): native mutex lane, build-fleet-b, nice 15. check-fleet.sh <capture-name> <cargo check args…>
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb/generated/$1.txt"; shift
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
{ echo "QUEUED $(date '+%H:%M:%S') cargo check $*"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo check "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' lb "$@"; } > "$capture" 2>&1
