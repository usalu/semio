#!/bin/zsh
# 🧪 LB cargo test during the rebuild (rule 30): native lane, build-fleet-b, private target, nice 15. cargo-fleet.sh <capture-name> <cargo subcommand + args…>
cd /Users/ueli/Documents/semio || exit 2
capture=".tmp-ticket/wp-lb/generated/$1.txt"; shift
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.tmp-ticket/wp-lb/target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' lb "$@"; } > "$capture" 2>&1
