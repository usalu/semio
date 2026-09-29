#!/bin/zsh
# [DEBUG] S19 one-off: runs the temporary census probes (native lane, build-fleet-b), one capture. usage: zsh s19-census-run.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-target" CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
echo "START $(date '+%T')" > "$1"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native s19 -- zsh -c '
run() { local label=$1; shift; perl -e "alarm shift; exec @ARGV" 1680 nice -n 15 cargo test "$@" --no-fail-fast -- s19_census --nocapture --test-threads 1; echo "STEP $label rc=$? $(date +%T)"; }
run forms -p semio-s-artifact-forms-forms --lib
run sequence -p semio-s-artifact-sequence-sequence --lib
run generation3d -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
run generation2d -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib
run norm -p semio-s-plugin-norm --lib
' >> "$1" 2>&1
echo "END rc=$? $(date '+%T')" >> "$1"
