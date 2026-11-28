#!/bin/zsh
# [DEBUG] cargo wrapper for shared-formula/boolean work: cargo <sub> in package dir $PKG (model|mutations|base) with a private target dir
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; O=$T/🗑️generated/claude-sf
mkdir -p $O
export PATH=$(echo ~/.rustup/toolchains/nightly-*/bin):$PATH
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$O/target CARGO_BUILD_BUILD_DIR=$O/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
B="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"
case $PKG in model) cd "$B/🧬️schema/📦️packages/🦀️rust";; mutations) cd "$B/🧬️schema/🧬️mutations/📦️packages/🦀️rust";; xlsx) cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📦️packages/🦀️rust";; base) cd "$B/📦️packages/🦀️rust";; esac || exit 1
cargo "$1" -j 3 "${@:2}"
