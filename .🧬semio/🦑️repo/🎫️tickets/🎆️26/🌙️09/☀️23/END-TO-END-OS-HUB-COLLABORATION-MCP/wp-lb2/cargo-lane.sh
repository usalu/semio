#!/bin/zsh
# 🧪 LB2 native cargo through the fleet native lane (session-14 rule 3): build-fleet-b, CARGO_INCREMENTAL=0, private target, nice 15. cargo-lane.sh <capture-name> <cargo subcommand + args…>
cd /Users/ueli/Documents/semio || exit 2
capture=".🧬semio/🌐hub/s14-lb2-captures/$1.txt"; shift
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-target"
{ echo "QUEUED $(date '+%H:%M:%S') cargo $*"; zsh .tmp-ticket/📜️fleet-mutex.sh native lb2 -- zsh -c 'echo "START $(date "+%H:%M:%S")"; nice -n 15 cargo "$@"; echo "EXIT $? $(date "+%H:%M:%S")"' lb2 "$@"; } > "$capture" 2>&1
