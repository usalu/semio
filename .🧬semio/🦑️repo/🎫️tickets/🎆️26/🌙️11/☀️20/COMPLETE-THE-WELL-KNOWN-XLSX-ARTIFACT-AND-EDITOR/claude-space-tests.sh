#!/bin/zsh
# [DEBUG] separate-target native test run of one Space artifact workspace (WS=home|space, ARGS=cargo args)
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; N=$T/🗑️generated/claude-space-tests
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
mkdir -p $N
BUN_DIR=$(dirname $(ls /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)); export PATH=$BUN_DIR:$PATH
cd /Users/ueli/Documents/semio/✏️s/🔌️plugins/🪐️space/🗿️artifacts/${WS:-🏠️home} || exit 1
cargo ${=CMD:-check} ${=SEL:---workspace} --tests -j 4 "$@" > $N/${LOG:-tests}.log 2>&1
echo "exit $?" >> $N/${LOG:-tests}.log
