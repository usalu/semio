#!/bin/zsh
# 🧱️ S20 faults overlay: one cargo invocation inside the overlay on its ONE private build-dir + target-dir, through the
# overlay lane (fleet mutex), niced; output to a log under `.🧬semio/🌐hub/s14-s20-overlay-build/logs/`.
# usage: zsh overlay-cargo.sh <tag> <cargo args…>   e.g. zsh overlay-cargo.sh sdk check --offline --lib -p semio-framework-plugin
tag="$1"; shift
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults"
B="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-build"
mkdir -p "$B/logs"
log="$B/logs/$(date +%m%d-%H%M%S)-$tag.txt"
echo "QUEUED $(date +%T) cargo $*" > "$log"
cd "$O" || exit 2
CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$B/build" CARGO_TARGET_DIR="$B/target" NX_DAEMON=false \
  zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s20 -- nice -n 15 zsh -c 'echo "START $(date +%T)"; cargo "$@"; echo "EXIT $? $(date +%T)"' cargo-run "$@" >> "$log" 2>&1
echo "$log"
tail -3 "$log"
