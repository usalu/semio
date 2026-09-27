#!/bin/zsh
# 🧪️ EN2 native-lane proof of the landed test-raw-routing set's Rust half: the standalone, dependency-free test-host crate
# (own `[workspace]`, never a member of the repository workspace) — check with tests, then its tests.
# usage: zsh 📜️fleet-mutex.sh native en2 -- zsh en2-native-test-host.sh
set -u
ROOT="/Users/ueli/Documents/semio"
export CARGO_BUILD_BUILD_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$ROOT/.tmp-ticket/wp-en2/target" CARGO_INCREMENTAL=0
MANIFEST="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml"
cd "$ROOT" || exit 90
echo "[en2] $(date '+%H:%M:%S') check"
nice -n 15 cargo check --manifest-path "$MANIFEST" --lib --tests
check=$?
echo "[en2] $(date '+%H:%M:%S') check rc=$check"
nice -n 15 cargo test --manifest-path "$MANIFEST" --no-fail-fast
tests=$?
echo "[en2] $(date '+%H:%M:%S') test rc=$tests"
exit $(( check + tests ))
