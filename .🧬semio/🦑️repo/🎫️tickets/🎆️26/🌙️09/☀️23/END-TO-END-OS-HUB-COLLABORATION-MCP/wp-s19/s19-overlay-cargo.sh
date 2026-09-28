#!/bin/zsh
# 🧪️ S19 overlay cargo runner: ONE cargo command inside the scratch overlay through the fleet `overlay` lane, with a private
# build-dir + target inside the overlay (never the shared build-dir), nice 15, incremental off, the lane hold capped at 28 min
# (SIGALRM; finished units stay cached, so a re-run continues).
# usage: zsh s19-overlay-cargo.sh <capture> <cargo args…>
capture="$1"; shift
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"
cd "$O" || exit 2
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
echo "START $(date '+%H:%M:%S') cargo $*" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo "$@" >> "$capture" 2>&1
rc=$?
echo "END rc=$rc $(date '+%H:%M:%S')" >> "$capture"
exit $rc
