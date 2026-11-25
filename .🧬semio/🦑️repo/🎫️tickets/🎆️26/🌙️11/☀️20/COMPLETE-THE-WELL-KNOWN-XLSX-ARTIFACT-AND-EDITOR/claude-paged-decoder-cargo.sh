#!/bin/zsh
# [DEBUG] cargo wrapper for the xlsx paged decoder work (small private target dir)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-paged-decoder
mkdir -p $O
export PATH=$(echo ~/.rustup/toolchains/nightly-*/bin):$PATH
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$O/target CARGO_BUILD_BUILD_DIR=$O/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx" || exit 1
cargo "$1" -j 4 "${@:2}"
