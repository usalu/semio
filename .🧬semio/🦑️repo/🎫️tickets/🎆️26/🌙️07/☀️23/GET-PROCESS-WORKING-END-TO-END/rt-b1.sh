#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; B=$T/🗑️generated/rt-build; G=$T/🗑️generated
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$B/target CARGO_BUILD_BUILD_DIR=$B/build
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
R=/Users/ueli/Documents/semio
cd "$R/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" && cargo test -j 2 --no-fail-fast -p semio-s-artifact-process-process3d-examples > $G/rt-b1-examples.txt 2>&1; echo "exit $?" > $G/rt-b1-examples.done
