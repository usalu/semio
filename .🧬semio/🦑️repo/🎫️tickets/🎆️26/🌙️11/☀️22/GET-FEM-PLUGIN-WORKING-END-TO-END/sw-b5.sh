#!/usr/bin/env bash
# 🧪️ bin-05 laws in private dirs C:\t\sw. Usage: sw-b5.sh <tag> [filter...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/sw" /c/t/swtmp
export CARGO_BUILD_BUILD_DIR='C:\t\sw' CARGO_TARGET_DIR='C:\t\sw' TMP='C:\t\swtmp' TEMP='C:\t\swtmp' TMPDIR='C:\t\swtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$T/🗑️generated/sw/$1.log"; shift
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/📦️packages/🦀️rust" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-os-store-tests-05 --no-fail-fast -- --nocapture "$@" > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
