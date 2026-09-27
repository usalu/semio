#!/bin/zsh
# 🎟️ LB2 overlay proof of p1 in ONE overlay-lane hold: apply the prepared patch to the overlay, run the new law, then the details unit tests.
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-overlay"
cd "$O" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-target"
echo "APPLY $(date '+%H:%M:%S')"; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/lb2-p1-arena-budget.py --write --root "$O"; echo "APPLY rc=$?"
echo "LAW $(date '+%H:%M:%S')"; nice -n 15 cargo test -p semio-s-artifact-stdio-contract --test details_arena_headroom --no-fail-fast -- --nocapture; echo "LAW rc=$? $(date '+%H:%M:%S')"
echo "UNIT $(date '+%H:%M:%S')"; nice -n 15 cargo test -p semio-s-artifact-stdio-contract --lib --no-fail-fast -- details; echo "UNIT rc=$? $(date '+%H:%M:%S')"
