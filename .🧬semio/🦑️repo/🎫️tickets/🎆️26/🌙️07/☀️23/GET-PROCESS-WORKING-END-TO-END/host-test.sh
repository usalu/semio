#!/bin/zsh
export PATH="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/bin:$PATH"
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=$T/🗑️generated/native
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/target CARGO_BUILD_BUILD_DIR=$N/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🔨️modules/🏠️host/🧰️owned/📦️packages/🦀️rust" || exit 1
cargo test -j 3 -p semio-s-artifact-process-process3d-host  > $T/🗑️generated/host-test.txt 2>&1
echo "exit $?" > $T/🗑️generated/host-test.done
