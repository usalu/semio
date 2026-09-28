#!/bin/zsh
# 📏️ T14 live-tree baseline (native lane, build-fleet-b + private target) for owner tests that go red in the overlay proof:
# tells whether a red is pre-existing on the live tree or introduced by the T14 pass. usage: baseline-native.sh <crate> [filter…]
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14/target
cd /Users/ueli/Documents/semio || exit 2
crate="$1"; shift
echo "LANE $(date +%T)"
cargo test --no-fail-fast -p "$crate" --lib -- "$@" 2>&1 | /usr/bin/grep -E '^test |^test result|panicked at|SIGABRT|could not compile|^error' ; echo "RC ${pipestatus[1]} $(date +%T)"
