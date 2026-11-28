#!/usr/bin/env bash
# 🧪️ bin-05 initializer replay laws in private dirs C:\t\si. Usage: si-b5.sh <tag> [filter...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/si" /c/t/sitmp
export CARGO_BUILD_BUILD_DIR='C:\t\si' CARGO_TARGET_DIR='C:\t\si' TMP='C:\t\sitmp' TEMP='C:\t\sitmp' TMPDIR='C:\t\sitmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$T/🗑️generated/si/$1.log"; shift
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/📦️packages/🦀️rust" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-os-store-tests-05 --no-fail-fast -- --nocapture "$@" > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
