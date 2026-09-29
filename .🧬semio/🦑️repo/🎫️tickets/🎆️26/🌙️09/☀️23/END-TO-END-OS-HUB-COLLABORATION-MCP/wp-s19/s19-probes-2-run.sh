#!/bin/zsh
# [DEBUG] S19 one-off: while HOLDING the native lane, writes the temporary `s19_probe2` probes (test-only), runs them
# (build-fleet-b), then reverts them — so no other lane job ever compiles them. usage: zsh s19-probes-2-run.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-target" CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
echo "START $(date '+%T')" > "$1"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native s19 -- zsh -c '
H=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19
python3 $H/s19-probes-2.py --write || exit 1
run() { local label=$1; shift; perl -e "alarm shift; exec @ARGV" 1680 nice -n 15 cargo test "$@" --no-fail-fast -- s19_probe2 --nocapture --test-threads 1; echo "STEP $label rc=$? $(date +%T)"; }
run sourcing -p semio-s-artifact-sourcing-curation --lib
run norm -p semio-s-plugin-norm --lib
run generation3d -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
run generation2d -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib
run flow -p semio-s-artifact-flow-flow --lib
python3 $H/s19-probes-2.py --revert
' >> "$1" 2>&1
echo "END rc=$? $(date '+%T')" >> "$1"
