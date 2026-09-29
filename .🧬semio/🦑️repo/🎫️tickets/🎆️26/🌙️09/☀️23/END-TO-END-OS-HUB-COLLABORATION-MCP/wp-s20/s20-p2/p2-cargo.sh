#!/bin/zsh
# 🧱️ S20 pass-2 overlay: one cargo invocation inside `s14-s20-overlay-faults-p2` on ITS private build-dir + target-dir
# (third-party units seeded by an APFS clone of the pass-1 overlay's build-dir), through the overlay lane, niced; output to
# a log under `.🧬semio/🌐hub/s14-s20-overlay-build-p2/logs/`.
# usage: zsh p2-cargo.sh <tag> <cargo args…>
tag="$1"; shift
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2"
B="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-build-p2"
mkdir -p "$B/logs"
log="$B/logs/$(date +%m%d-%H%M%S)-$tag.txt"
echo "QUEUED $(date +%T) cargo $*" > "$log"
cd "$O" || exit 2
CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$B/build" CARGO_TARGET_DIR="$B/target" NX_DAEMON=false \
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s20 -- nice -n 15 zsh -c 'echo "START $(date +%T)"; cargo "$@"; echo "EXIT $? $(date +%T)"' cargo-run "$@" >> "$log" 2>&1
echo "$log"
tail -3 "$log"
