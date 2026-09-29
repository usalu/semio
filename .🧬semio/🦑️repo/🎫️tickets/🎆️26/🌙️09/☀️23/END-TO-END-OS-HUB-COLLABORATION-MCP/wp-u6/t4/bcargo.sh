#!/bin/zsh
# 🧪️ U6 T4 baseline lane: the same as ocargo.sh inside the PRISTINE base clone (its own private build/target dirs).
capture="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs/$1.txt"; shift
cd "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-base" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-bbuild"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-btarget"
echo "QUEUED $(date '+%T') $*" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay u6 -- nice -n 15 zsh -c "echo START \$(date '+%T'); $*" >> "$capture" 2>&1
echo "LANE-EXIT rc=$? $(date '+%T')" >> "$capture"
