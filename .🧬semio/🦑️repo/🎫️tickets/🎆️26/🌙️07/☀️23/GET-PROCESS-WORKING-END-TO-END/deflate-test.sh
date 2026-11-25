#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/bin:$PATH"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native-test
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
mkdir -p $N
cd "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/📦️packages/🦀️rust" || exit 1
cargo test -j 3 --lib ${FILTER:-borrowed_pack_document} > $T/🗑️generated/deflate-test.txt 2>&1
echo "exit $?" > $T/🗑️generated/deflate-test.done
