#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"; N=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-Vo4lE6/8cbd0d6d-32d5-4da7-9277-09843fb54394/scratchpad/run
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= CARGO_TARGET_DIR=$N/t CARGO_BUILD_BUILD_DIR=$N/b
export CARGO_BUILD_JOBS=3 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off
export PATH="$HOME/.bun/bin:$PATH"
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d" || exit 1
L=$1; shift
cargo "$@" > "$T/🗑️generated/run-$L.txt" 2>&1; echo exit $?
