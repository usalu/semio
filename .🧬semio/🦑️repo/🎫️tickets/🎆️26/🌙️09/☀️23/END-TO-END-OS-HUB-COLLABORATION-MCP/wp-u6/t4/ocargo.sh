#!/bin/zsh
# 🧪️ U6 T4 overlay lane: runs one shell command inside the private full-repo overlay with PRIVATE build/target dirs (never the
# shared build-fleet-b), nice 15, incremental off, through the fleet `overlay` lane. ocargo.sh <capture-name> '<command>'
capture="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs/$1.txt"; shift
cd "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-obuild"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-otarget"
echo "QUEUED $(date '+%T') $*" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay u6 -- nice -n 15 zsh -c "echo START \$(date '+%T'); $*" >> "$capture" 2>&1
echo "LANE-EXIT rc=$? $(date '+%T')" >> "$capture"
