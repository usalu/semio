#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/bin:$PATH"  # cargo-gate
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/build-cancel/target CARGO_BUILD_BUILD_DIR=$N/build-cancel/build
export CARGO_BUILD_JOBS=3 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🍉️tests-06/📦️packages/🦀️rust" || exit 1
L=$1; shift
cargo "$@" > $T/🗑️generated/cancel-$L.txt 2>&1; echo exit $?
